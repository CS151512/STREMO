# Стратегия Шардирования (Sharding Strategy)

![](../img/sharding-md.png)

>[!IMPORTANT]
> В высоконагруженных стриминговых системах объем генерируемых данных (особенно в чатах трансляций) мгновенно превышает возможности вертикального масштабирования (Scale-Up). Данный документ описывает механизмы горизонтального масштабирования (Scale-Out) с использованием кластера **PostgreSQL + Citus Data**.

---

## **1. Архитектура Распределенной Базы Данных Citus**

Citus превращает PostgreSQL в распределенную СУБД, состоящую из одного Coordinator-узла и множества Worker-узлов. Приложение (наши микросервисы) общается только с Coordinator-ом, думая, что работает с обычной локальной базой Postgres.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 100, "nodeSpacing": 30}}}%%
flowchart TD
    classDef dashedBlack fill:#fff,stroke:#000,stroke-width:2px,stroke-dasharray: 6 6,color:#000,rx:15,ry:15
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    API("Microservices\n(Chat, VOD, Stream Meta)"):::dashedBlack
    
    subgraph Citus Cluster
        Coordinator("Coordinator Node\n(Routing & Planning)"):::solidBlack
        
        Worker1("Worker Node 1\n(Shards: 1001-1050)"):::storage
        Worker2("Worker Node 2\n(Shards: 1051-1100)"):::storage
        Worker3("Worker Node N\n(Shards: 1101-1150)"):::storage
    end
    
    API -->|"Standard SQL Queries"| Coordinator
    
    Coordinator ==>|"Distributed Query Execution"| Worker1
    Coordinator ==>|"Distributed Query Execution"| Worker2
    Coordinator ==>|"Distributed Query Execution"| Worker3
```

### **1.1. Маршрутизация запросов**
Когда `chat-service` отправляет запрос `SELECT * FROM chat_messages WHERE channel_id = 'user-123'`, Coordinator вычисляет хэш от `user-123`, определяет, на каком Worker-узле находится нужный шард, и прозрачно проксирует запрос именно туда, минуя остальные сервера.

---

## **2. Выбор Ключа Распределения (Sharding Key)**

Фундаментальное правило шардирования: **данные, которые запрашиваются вместе, должны лежать вместе (на одном физическом диске)**.

Мы используем `channel_id` (ID стримера) в качестве единого ключа распределения (Distribution Column) для всех высоконагруженных таблиц (чаты, статистика, клипы). Это позволяет Citus выполнять операции `JOIN` локально на Worker-узлах без необходимости пересылать терабайты данных по сети (Network Shuffle).

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 15}}}%%
flowchart LR
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    subgraph Logical Concept: Tenant Isolation
        direction TB
        TenantA("Channel A (channel_id=123)"):::solidBlack
        TenantB("Channel B (channel_id=456)"):::solidBlack
    end

    subgraph Physical Disk: Worker 1
        direction TB
        DataA("chat_messages (ch_id=123)\nstreams (ch_id=123)\nvods (ch_id=123)"):::storage
    end

    subgraph Physical Disk: Worker 2
        direction TB
        DataB("chat_messages (ch_id=456)\nstreams (ch_id=456)\nvods (ch_id=456)"):::storage
    end

    TenantA -->|"Hashes to Shard X"| DataA
    TenantB -->|"Hashes to Shard Y"| DataB
```

>[!TIP]
> **Co-location (Совместное размещение)**
> Запрос вида `SELECT c.text, s.title FROM chat_messages c JOIN streams s ON c.channel_id = s.channel_id WHERE c.channel_id = '123'` выполнится за миллисекунды, так как обе таблицы для этого канала физически лежат на `Worker 1`.

---

## **3. Reference Tables (Справочные таблицы)**

Некоторые таблицы (например, `categories` со списком игр или `badges` со значками модераторов) нужны часто, но они слишком малы для шардирования по ключу.

В Citus они помечаются как **Reference Tables**. Координатор автоматически создает полную копию (реплику) таких таблиц на **каждом** Worker-узле. Это позволяет делать JOIN шардированных таблиц (чат) со справочными (значки) без сетевых задержек.

---

## **4. Решение проблемы Hotspots (Горячие точки)**

Самая опасная ситуация при шардировании по `channel_id` — запуск стрима топовым киберспортсменом с онлайном 300,000 зрителей. В этот момент один конкретный `channel_id` сгенерирует 90% нагрузки всей платформы, и Worker, на котором лежит его шард, может упасть (Hotspot).

### **4.1. Смягчение нагрузки (Mitigation Strategies)**

1.  **Batching на уровне приложения:** `chat-service` не пишет сообщения в БД поштучно. Он аккумулирует их в RAM и делает один Bulk Insert раз в 500мс. Даже 10,000 сообщений превратятся всего в 2 запроса к Worker-узлу.
2.  **Tenant Isolation (Изоляция тенанта):** С помощью функции `citus_isolate_tenant()` мы можем на лету "вырезать" шард крупного стримера из общего Worker-узла и перенести его на отдельный, выделенный сверхмощный сервер без остановки записи.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 20}}}%%
flowchart LR
    classDef dashedBlack fill:#fff,stroke:#000,stroke-width:2px,stroke-dasharray: 6 6,color:#000,rx:15,ry:15
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    subgraph Before Isolation
        WorkerA("Worker 1\n(Channels: A, B, C)"):::storage
    end
    
    Admin("DBA / Automation"):::solidBlack -->|"citus_isolate_tenant('channel_A')"| Citus
    
    subgraph After Isolation
        WorkerA_New("Worker 1\n(Channels: B, C)"):::storage
        Worker_Dedicated("Worker 2 (Dedicated)\n(Channel A)"):::dashedBlack
    end
```

>[!CAUTION]
> **Мониторинг шардов**
> SRE команда обязана настроить алерты в Grafana на CPU/IOPS каждого отдельного Citus Worker. Если утилизация диска (IO Wait) на одном воркере превышает 80% в течение 2 минут, запускается скрипт автоматической изоляции горячего шарда.
