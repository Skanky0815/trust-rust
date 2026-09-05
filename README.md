# TrustRust - Microservices Architecture

Eine moderne Microservices-Architektur in Rust mit gRPC, PostgreSQL und RabbitMQ.

## 📋 Requirements

### Lokal (Development)
- **Rust** 1.75+
- **libpq** (PostgreSQL Client Libraries)
  ```bash
  brew install libpq
  ```
- **Docker & Docker Compose** (für Datenbank + Message Queue)

### Docker (Produktion)
- **Docker** 20.10+
- **Docker Compose** 2.0+

## 🚀 Quick Start

### Variante 1: Lokal mit Docker für Infrastruktur (Empfohlen für Development)

```bash
# 1. Services starten
docker-compose up

# 2. In neuem Terminal: App bauen und laufen
cd service_patient
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/patient_database"
export QUEUE_URL="amqp://admin:secret@localhost:5672/%2f"
cargo run
```

Die `service_patient/.cargo/config.toml` hat bereits die libpq-Pfade konfiguriert.

### Variante 2: Vollständig in Docker (Produktion)

```bash
# Baut alle Services und startet sie
docker-compose up --build
```

### Variante 3: Lokal alles bauen (ohne Docker)

```bash
# PostgreSQL lokal installieren
brew install postgresql@16
brew services start postgresql@16
createdb patient_database

# RabbitMQ lokal installieren
brew install rabbitmq
brew services start rabbitmq

# App starten
cd service_patient
cargo run
```

## 🏗️ Architektur

```
TrustRust (Microservices Platform)
├── service_patient/        # Patient Management Service
│   ├── src/
│   │   ├── application/    # gRPC Service Implementation
│   │   ├── module/         # Business Logic
│   │   ├── infrastructure/ # Database & Queue Setup
│   │   └── schema.rs       # Diesel Schema (Auto-generated)
│   ├── proto/              # gRPC Definitions
│   ├── migrations/         # Database Migrations
│   └── Dockerfile          # Container Build
├── compose.yaml            # Infrastructure (DB, RabbitMQ, Services)
└── README.md              # This file
```

### Layer-Struktur pro Service

#### **application/** - gRPC Service Schnittstelle
Implementiert die gRPC-Trait und konvertiert zwischen gRPC-Messages und internen Types.
```rust
pub struct Service { ... }
impl PatientGrpcService for Service { ... }
```

#### **module/** - Business Logik
Enthält Service-Klassen mit Geschäftslogik, unabhängig von gRPC.
```rust
pub struct PatientService {
    db: DbPool,
    channel: Arc<Channel>,
}

impl PatientService {
    pub async fn add(&self, patient: NewPatient) -> Result<Patient> { ... }
    pub async fn get_all(&self) -> Result<Vec<Patient>> { ... }
}
```

#### **infrastructure/** - Technische Integration
- `database.rs`: r2d2 Connection Pool Setup
- `queue.rs`: RabbitMQ Connection Setup

```
Request → application::Service 
        → module::PatientService 
        → infrastructure::DbPool + RabbitMQ
```

## 📊 Infrastruktur (Docker Compose)

| Service | Port | Typ | Beschreibung |
|---------|------|-----|-------------|
| patient-database | 5432 | PostgreSQL | Persistente Daten |
| rabbitmq | 5672 | AMQP | Message Queue |
| rabbitmq | 15672 | HTTP | Management UI |
| patient-service | 50051 | gRPC | Patient Service |

## 🔄 Datenfluss

```
Client (gRPC)
    ↓
application/patient_service.rs (Service)
    ↓
module/patient/service.rs (PatientService)
    ↓ [Async → Blocking]
infrastructure/database.rs (r2d2 Pool)
    ↓
PostgreSQL Database

[Parallel]
infrastructure/queue.rs
    ↓
RabbitMQ
```

## 📝 Umgebungsvariablen

### Development (.env / .cargo/config.toml)
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/patient_database?sslmode=disable
QUEUE_URL=amqp://admin:secret@localhost:5672/%2f
```

### Docker (compose.yaml)
```yaml
environment:
  DATABASE_URL: postgresql://postgres:postgres@patient-database:5432/patient_database
  QUEUE_URL: amqp://admin:secret@rabbitmq:5672/
```

## 🗄️ Datenbank

### Schema
```sql
-- In patient_database
CREATE TABLE patients (
    id UUID PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Migrations
Befinden sich in `service_patient/migrations/`. 
Diesel lädt diese automatisch beim Start.

## 🔌 Abhängigkeiten

### Hauptdependencies (service_patient/Cargo.toml)
```toml
diesel = { version = "2.2.0", features = ["postgres", "uuid", "chrono", "r2d2"] }
tonic = "0.14.6"                    # gRPC Framework
tokio = { version = "1.53.1", ... } # Async Runtime
lapin = "4.10.0"                    # RabbitMQ Client
uuid = { version = "1.25.0", features = ["v4"] }
chrono = { version = "0.4", ... }
```

## 🛠️ Development

### Neuen Service hinzufügen

1. **Neuer Ordner erstellen**
   ```bash
   mkdir service_appointment
   cp service_patient/Cargo.toml service_appointment/
   # ... anpassen
   ```

2. **Dockerfile kopieren**
   ```bash
   cp service_patient/Dockerfile service_appointment/
   ```

3. **In compose.yaml eintragen**
   ```yaml
   appointment-service:
     build:
       context: ./service_appointment
     ports:
       - "50052:50052"
     environment:
       DATABASE_URL: postgresql://postgres:postgres@patient-database:5432/patient_database
       QUEUE_URL: amqp://admin:secret@rabbitmq:5672/
     depends_on:
       patient-database:
         condition: service_healthy
       rabbitmq:
         condition: service_healthy
   ```

### Build & Test
```bash
cd service_patient

# Type-Check (schnell)
cargo check

# Build
cargo build

# Test
cargo test

# Run
cargo run
```

## 🐳 Docker

### Lokal Image bauen
```bash
docker build -f service_patient/Dockerfile -t trustrust-patient-service:latest .
```

### Mit Compose bauen
```bash
docker-compose build
```

### Logs anschauen
```bash
docker-compose logs -f patient-service
docker-compose logs -f patient-database
docker-compose logs -f rabbitmq
```

## 🧹 Cleanup

```bash
# Alle Container stoppen
docker-compose down

# Mit Volumes löschen
docker-compose down -v

# Images löschen
docker-compose down --rmi all
```

## 📚 Weitere Resources

- [Diesel Dokumentation](https://diesel.rs/)
- [Tonic gRPC](https://github.com/hyperium/tonic)
- [Tokio Async Runtime](https://tokio.rs/)
- [RabbitMQ](https://www.rabbitmq.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## 📝 Lizenz

Intern - TrustRust Project
