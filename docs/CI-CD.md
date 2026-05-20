# Непрерывная Интеграция и Качество Кода (CI Pipeline)

>[!IMPORTANT]
> Данный документ детализирует процессы, происходящие с кодом до его попадания в среду развертывания (которая описана в `Deploy.md`). Строгий CI-пайплайн гарантирует, что в Production попадет только проверенный, безопасный и оптимизированный код.

---

## **1. Архитектура CI Пайплайна (GitLab CI / GitHub Actions)**

Пайплайн запускается автоматически при создании Pull Request (Merge Request) или коммите в ветку `main`. Для C++ микросервисов процесс сборки и тестирования разделен на жесткие этапы (Stages), каждый из которых блокирует дальнейшее продвижение при возникновении ошибки.

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 20}}}%%
flowchart TD
    classDef dashedBlack fill:#fff,stroke:#000,stroke-width:2px,stroke-dasharray: 6 6,color:#000,rx:15,ry:15
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    Trigger("Commit to 'main'\nor PR Created"):::dashedBlack
    
    subgraph Stage 1: Build & Lint
        direction LR
        ClangTidy("Clang-Tidy\n(Static Analysis)"):::solidBlack
        CMake("CMake Build\n(GCC / Clang)"):::solidBlack
    end
    
    subgraph Stage 2: Testing
        direction LR
        GTest("Unit Tests\n(GoogleTest)"):::solidBlack
        Valgrind("Valgrind\n(Memory Leaks Check)"):::storage
    end

    subgraph Stage 3: Security & SAST
        direction LR
        Sonar("SonarQube\n(Code Quality)"):::solidBlack
        Trivy("Trivy\n(Vulnerability Scan)"):::storage
    end

    subgraph Stage 4: Package & Release
        direction LR
        Docker("Docker Build\n(Multi-stage)"):::solidBlack
        Push("Push to Registry\n(ECR / Harbor)"):::storage
    end

    Trigger --> Stage1
    Stage1 -->|"If Passed"| Stage2
    Stage2 -->|"If Passed"| Stage3
    Stage3 -->|"If Passed"| Stage4
```

---

## **2. Детализация Этапов (Stages)**

### **2.1. Сборка и Линтинг (Build & Lint)**

Кодовая база C++20 требует строгих стандартов форматирования и проверки на этапе компиляции.

*   **Clang-Format:** Проверяет код на соответствие `.clang-format` файлу проекта (стандарт Google C++ Style Guide с локальными модификациями).
*   **Clang-Tidy:** Ловит потенциальные баги до компиляции (например, неинициализированные переменные, опасные приведения типов, нарушения `const-correctness`).
*   **Компиляция:** Сборка происходит через CMake. В пайплайне всегда включен флаг `-Werror` (считать Warnings ошибками), чтобы код с предупреждениями не мог быть слит в `main`.

### **2.2. Тестирование и Анализ Памяти (Testing)**

Отказоустойчивость сервисов реального времени (In-Memory буферы, WebSockets) критически зависит от работы с памятью.

*   **GoogleTest (GTest):** Запускает набор Unit-тестов для бизнес-логики.
*   **Valgrind / AddressSanitizer (ASan):** Бинарный файл запускается в специальном окружении для поиска утечек памяти (Memory Leaks), обращений к освобожденной памяти (Use-After-Free) и состояний гонки (Race Conditions).

>[!CAUTION]
> **Блокировка пайплайна (Quality Gate)**
> Покрытие Unit-тестами (Code Coverage) должно быть не ниже 80%. Если покрытие падает ниже этого порога, пайплайн завершается с ошибкой (`exit 1`), и слияние ветки (Merge Request) автоматически блокируется.

---

## **3. Безопасность и SAST (Security Scanning)**

STREMO обрабатывает платежные данные и трансляции, поэтому безопасность интегрирована прямо в процесс сборки (Shift-Left Security).

```mermaid
%%{init: {"theme": "base", "look": "handDrawn", "flowchart": {"rankSpacing": 80, "nodeSpacing": 20}}}%%
flowchart LR
    classDef dashedBlack fill:#fff,stroke:#000,stroke-width:2px,stroke-dasharray: 6 6,color:#000,rx:15,ry:15
    classDef solidBlack fill:#fff,stroke:#000,stroke-width:2px,color:#000,rx:15,ry:15
    classDef storage fill:#fdfdfd,stroke:#555,stroke-width:2px,color:#000,rx:5,ry:5

    SourceCode("Source Code"):::dashedBlack --> SAST("SonarQube\n(Code Vulnerabilities)"):::solidBlack
    DockerImage("Compiled Docker Image"):::dashedBlack --> Trivy("Trivy Scanner\n(OS Vulnerabilities)"):::storage
    
    SAST -->|"Quality Gate"| Result("Merge Request\nStatus"):::solidBlack
    Trivy -->|"Block High/Critical"| Result
```

*   **SonarQube (SAST):** Анализирует исходный код на уязвимости (например, SQL Injection, Buffer Overflow) и технический долг (Code Smell).
*   **Trivy (Container Scanning):** После того как Docker-образ собран, Trivy сканирует его слои на предмет уязвимостей в системных библиотеках (CVE). Если найдены уязвимости уровня `CRITICAL` или `HIGH` в базовом образе, пуш в Registry блокируется.

---

## **4. Оптимизация Docker-образов (Multi-Stage Build)**

Поскольку C++ требует тяжелых компиляторов и библиотек (Boost, FFmpeg dev-headers), мы используем **Multi-Stage Builds** для создания финального образа, который пойдет в Production.

1.  **Stage 1 (Builder):** Использует тяжелый образ (`ubuntu` или `debian` с GCC/Clang, CMake, Conan/vcpkg). Здесь компилируется бинарный файл.
2.  **Stage 2 (Runtime):** Использует минималистичный образ `distroless/cc` (образ без shell, пакетного менеджера и лишних библиотек). В него копируется только скомпилированный бинарный файл и нужные `.so` файлы.

>[!TIP]
> **Преимущества Distroless:**
> Финальный образ микросервиса весит около 20-30 МБ вместо 1 ГБ+. Это кардинально ускоряет время запуска пода (Cold Start) в Kubernetes во время автомасштабирования (HPA) и сводит площадь атаки (Attack Surface) практически к нулю (внутри контейнера нет даже `bash`).