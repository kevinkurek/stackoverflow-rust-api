### Stack Overflow Clone API in Rust
This is a simple RESTful API for a Stack Overflow clone built using Rust, Axum, and PostgreSQL.

### Start Rust Server
```bash
# cargo watch // automatically restart server when source files change
$ cargo watch -q -c -w src/ -x run
```

### sample POST
```bash
curl -X POST http://localhost:8000/question \
  -H 'Content-Type: application/json' \
  -d '{"title":"Title","description":"Description"}'
```


### Start PostgreSQL Database
```bash
$ docker run --name stack-overflow-db -e POSTGRES_PASSWORD=xxxxxx -p 55008:5432 -d postgres
```

### Run Database Migrations
```bash
$ sqlx migrate run
```