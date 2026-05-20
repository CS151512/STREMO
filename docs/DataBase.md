# Архитектура Баз Данных STREMO

![](../img/db-md.png)

>[!IMPORTANT]
> База данных — самый критичный компонент платформы. STREMO использует подход гибридного хранения данных (Polyglot Persistence). Документ описывает разделение на Глобальный кластер (для транзакций) и Шардированный кластер (для тяжелых данных), а также механизмы репликации и отказоустойчивости.

---

## **1. Глобальная Топология Данных**

Чтобы избежать бутылочного горлышка (Bottleneck) при росте платформы, данные строго разделены на три независимых кластера в зависимости от паттерна их использования (Read-Heavy, Write-Heavy или Analytical).

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 100, "nodeSpacing": 20}}}%%
flowchart TD
    classDef cluster fill:#f8fafc,stroke:#94a3b8,stroke-width:2px,color:#334155,rx:10,ry:10,stroke-dasharray: 5 5
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    BFF("API Gateway / Microservices"):::solidBlack

    subgraph Cluster_Global ["1. Global Transactional DB (PostgreSQL)"]
        direction TB
        DB_Users("Users, Auth, Billing"):::storage
    end

    subgraph Cluster_Sharded ["2. Sharded Distributed DB (Citus / Postgres)"]
        direction TB
        DB_Streams("Chat History, VODs, Stream Meta"):::storage
    end

    subgraph Cluster_Analytics ["3. OLAP Analytics (ClickHouse)"]
        direction TB
        DB_Metrics("CCV, Ad Impressions, Traffic"):::storage
    end

    BFF -->|"ACID Transactions"| Cluster_Global
    BFF -->|"High Write Throughput"| Cluster_Sharded
    BFF -->|"Analytical Queries"| Cluster_Analytics
```

---

## **2. Глобальный Кластер (PostgreSQL)**

Этот кластер хранит "золотые данные" платформы. Здесь происходят финансовые операции, хранятся профили, пароли и настройки. 
*   **Паттерн:** Read-Heavy (часто читают профили, редко меняют).
*   **Требования:** Строгий ACID, гарантия отсутствия потерь при сбоях.

### **2.1 Схема Репликации (High Availability)**

Для обеспечения доступности (HA) используется асинхронная потоковая репликация (Streaming Replication) с автоматическим переключением (Failover) через Patroni или PgBouncer.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 40}}}%%
flowchart TD
    classDef dashedBlack fill:#fff,stroke:#000,stroke-width:2px,stroke-dasharray: 6 6,color:#000,rx:15,ry:15
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    Router("PgBouncer / HAProxy\n(Connection Pooling)"):::solidBlack
    
    Master("PostgreSQL PRIMARY\n(Read / Write)"):::storage
    Replica1("PostgreSQL REPLICA 1\n(Read-Only)"):::storage
    Replica2("PostgreSQL REPLICA 2\n(Read-Only)"):::storage

    Router -->|"Writes (UPDATE/INSERT)"| Master
    Router -.->|"Reads (GET /users)"| Replica1
    Router -.->|"Reads (GET /users)"| Replica2
    
    Master ==>|"WAL Streaming (Async)"| Replica1
    Master ==>|"WAL Streaming (Async)"| Replica2
```

### **2.2 Основные Таблицы Глобального Кластера**

*   `users`: ID, email, хэш пароля, дата регистрации.
*   `profiles`: bio, avatar_url, настройки уведомлений.
*   `wallets`: баланс fiat, баланс Bits (строгие транзакционные блокировки `SELECT FOR UPDATE`).
*   `outbox_events`: системная таблица для паттерна Transactional Outbox (события для отправки в Kafka).
*   `followers`: связь Many-to-Many между зрителями и каналами.

---

## **3. Шардированный Кластер (Citus Data)**

Самая большая проблема стриминговой платформы — чат. Если 100,000 человек одновременно пишут в разные чаты, один мастер-сервер PostgreSQL не выдержит нагрузки на запись (Write-Heavy). Мы решаем это с помощью расширения **Citus**, которое превращает PostgreSQL в распределенную СУБД.

### **3.1 Принцип Шардирования (Horizontal Scaling)**

Данные разбиваются на части (шарды) и распределяются по разным физическим серверам (Worker Nodes). 
*   **Ключ распределения (Distribution Key):** `channel_id`.
*   **Почему `channel_id`?** Это гарантирует, что вся история чата конкретного стримера, все его клипы и метаданные его стрима будут лежать на **одном физическом диске**. Это позволяет делать быстрые локальные `JOIN` запросы без пересылки данных по сети между нодами.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 100, "nodeSpacing": 15}}}%%
flowchart TD
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    App("Chat Service / Stream Meta Svc"):::solidBlack

    Coordinator("Citus Coordinator Node\n(Query Router)"):::storage
    
    App -->|"INSERT INTO chat_messages\n(channel_id=1)"| Coordinator
    App -->|"INSERT INTO chat_messages\n(channel_id=9)"| Coordinator
    App -->|"INSERT INTO chat_messages\n(channel_id=4)"| Coordinator

    subgraph Citus Worker Nodes
        direction LR
        Worker1("Worker Node A\n(Shards for ch: 1, 2, 3)"):::storage
        Worker2("Worker Node B\n(Shards for ch: 4, 5, 6)"):::storage
        Worker3("Worker Node C\n(Shards for ch: 7, 8, 9)"):::storage
    end

    Coordinator ==>|"Routes ch=1"| Worker1
    Coordinator ==>|"Routes ch=4"| Worker2
    Coordinator ==>|"Routes ch=9"| Worker3
```

### **3.2 Основные Таблицы Шардированного Кластера**

*   `chat_messages`: ID сообщения, `channel_id` (Sharding Key), `user_id`, текст, timestamp.
*   `streams`: ID трансляции, `channel_id` (Sharding Key), title, category, status.
*   `vods`: сохраненные видео, привязаны к `channel_id`.
*   `moderation_actions`: баны и таймауты (распределены по `channel_id` для быстрого доступа во время стрима).

>[!TIP]
> **Автоматическая балансировка**
> При добавлении новых серверов в кластер, Citus автоматически в фоновом режиме (не прерывая работу платформы) перенесет часть шардов на новые сервера, размазывая нагрузку.

---

## **4. ClickHouse (Слой Аналитики)**

Аналитика требует совершенно другого подхода. Стример хочет видеть график "Сколько зрителей было на каждой минуте моего 10-часового стрима". PostgreSQL будет выполнять такой запрос очень долго, сканируя миллионы строк.
**ClickHouse** решает это благодаря столбцовому хранению и векторизованным вычислениям.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 20}}}%%
flowchart LR
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    Ingest("Ingest Svc"):::solidBlack -->|"Publishes"| Kafka("Kafka Bus\n(metrics_topic)"):::storage
    
    Kafka -->|"Consume Batch\n(100k events/sec)"| CH_Buffer("ClickHouse\nKafka Engine Table"):::storage
    
    CH_Buffer -->|"Materialized View\n(Aggregates by Minute)"| CH_Final("ClickHouse\nMergeTree (Fast Reads)"):::storage
    
    BFF("API Gateway"):::solidBlack -->|"GET /dashboard"| CH_Final
```

### **4.1 Почему ClickHouse?**
1.  **Огромная скорость вставки:** Способен "заглатывать" сотни тысяч метрик в секунду напрямую из Kafka (через встроенный Kafka Table Engine).
2.  **Агрегация на лету:** Данные о зрителях сбрасываются каждую секунду. ClickHouse (через Materialized Views) автоматически сжимает их, вычисляя средний и пиковый онлайн (CCV) за каждую минуту.
3.  **Экономия диска:** Столбцовые данные отлично сжимаются (в 3-5 раз лучше, чем в PostgreSQL).

---

## **5. Механизмы Кэширования (Redis Cluster)**

Базы данных не должны обрабатывать 100% трафика. Все частые запросы экранируются кластером Redis 7.

*   **Token Bucket:** Хранит лимиты Rate Limit (защита от DDoS).
*   **Pub/Sub:** Мгновенная рассылка сообщений чата между подами Kubernetes (данные не сохраняются в Redis, только маршрутизируются).
*   **Сессии и JWT:** Список отозванных токенов (Blacklist).
*   **Live Directory:** Список активных стримов для Главной страницы обновляется в Redis каждую секунду. БД не участвует в рендеринге каталога.
