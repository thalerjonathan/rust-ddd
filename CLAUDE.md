# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Domain Driven Design (DDD) project exploring volleyball referee management. The project demonstrates both monolith and microservices architectures, transitioning from a simple monolith to a distributed microservices system with event-driven communication.

**Domain**: Volleyball referee management tool with entities like Referees, Teams, Venues, Fixtures, Availabilities, and Assignments.

**Frontend**: Rust WASM using Leptos framework (React-like experience)
**Backend**: Rust with DDD principles, PostgreSQL databases, REST APIs

## Architecture Patterns

### Monolith Architecture
- Single application with shared database
- Located in `/monolith` directory
- Domain aggregates: Referee, Team, Venue, Fixture, Availability, Assignment
- Repository pattern with PostgreSQL implementations
- Transactional boundaries handled in REST layer

### Microservices Architecture
- 6 separate services: referees, teams, venues, fixtures, availabilities, assignments
- Each service has its own PostgreSQL database
- Event-driven communication via Kafka
- Redis caching for cross-service data resolution
- Nginx as API Gateway and load balancer
- Distributed tracing with Jaeger
- Saga pattern for cross-service transactions
- Outbox/Inbox pattern for reliable event handling

## Development Commands

### Monolith Development
```bash
# From /monolith directory:
sh start_db.sh          # Start PostgreSQL database
sh run.sh               # Build and run backend (includes config loading)
sh tests.sh             # Run test suite (single-threaded)

# From /frontend directory:
sh run.sh               # Build and run Leptos frontend (opens browser)
```

### Microservices Development

#### Infrastructure Setup (run in order from /microservice):
```bash
sh start_all_db.sh                # Start all PostgreSQL databases
sh start_redis.sh                 # Start Redis cache
sh start_kafka.sh                 # Start Kafka message broker
sh start_debezium.sh              # Start Debezium for CDC
sh create_debezium_connectors.sh  # Configure Debezium connectors
sh start_nginx.sh                 # Start Nginx API Gateway
sh start_jaeger.sh                # Start Jaeger tracing
sh start_keycloak.sh              # Start Keycloak IdP (for auth)
```

#### Service Deployment Options:

**Option 1: Direct Rust Build**
```bash
sh build_and_run_all.sh    # Compile and run all services (2 instances each)
```

**Option 2: Docker Compose**
```bash
sh run_services.sh         # Run all services via Docker
sh stop_services.sh        # Stop all services
```

#### Frontend
```bash
# From /frontend:
sh build_and_run_frontend.sh  # Docker build and run
sh run_frontend.sh            # Direct run via Docker compose
```

#### Testing
```bash
# From individual service directories:
sh tests.sh               # Run E2E tests (requires infrastructure running)
```

#### Cleanup
```bash
sh kill_all.sh            # Stop all running services
sh clean_all_db.sh        # Clean all databases
```

## Key Technologies

- **Database**: PostgreSQL with SQLx (no ORM by design choice)
- **Frontend**: Leptos (Rust WASM framework)
- **Message Broker**: Kafka with Debezium for Change Data Capture
- **Caching**: Redis
- **API Gateway**: Nginx
- **Tracing**: Jaeger with OpenTelemetry
- **Authentication**: Keycloak IdP
- **Containerization**: Docker and Docker Compose

## Important Implementation Details

### Database Layer
- Uses SQLx for direct SQL queries (deliberately avoiding ORMs like Diesel/SeaORM)
- Repository pattern with PostgreSQL-specific implementations
- Transactional boundaries managed at REST layer level
- Each microservice has separate database schemas

### Domain Layer
- Aggregates are technology-agnostic with immutable getters and mutable domain logic
- Extensive use of `From` trait implementations for type conversions
- `save()` method implemented as upsert for both insert/update operations

### Event Handling
- Outbox pattern: Domain events stored in DB table, async processor publishes to Kafka
- Inbox pattern: Incoming events deduplicated via database table
- At-least-once delivery semantics
- Each service runs 2 instances with Kafka consumer groups for load distribution

### Frontend-Backend Integration
- Shared types between frontend and backend via workspace dependencies
- No OpenAPI needed due to shared Rust types
- REST API with JSON serialization

## Project Structure

```
rust-ddd/
├── monolith/           # Monolithic implementation
├── microservice/       # Microservices implementation
│   ├── services/       # Individual service implementations
│   ├── infra/         # Infrastructure (databases, Kafka, etc.)
│   └── ADR/           # Architecture Decision Records
├── frontend/          # Leptos WASM frontend
├── shared/            # Shared types and utilities
└── reports/           # Weekly development reports
```

## Build Requirements

- Rust nightly toolchain
- `trunk` for Leptos frontend: `cargo install trunk`
- WASM target: `rustup target add wasm32-unknown-unknown`
- Docker and Docker Compose
- SQLx CLI for database migrations (if needed)

## Testing Strategy

- E2E tests require full infrastructure stack running
- Tests run single-threaded to avoid database race conditions
- Each microservice has independent test suite
- Integration tests validate cross-service communication

## Performance Considerations

- Redis caching essential for microservices to avoid N+1 queries
- Kafka partitioning aligns with service instance count
- Database JOINs optimized in monolith, replaced with caching in microservices
- Nginx load balancing across service instances