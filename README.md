## Rust CRUD API

> Rest api build with the warp framework to manage a todo list

This API uses a fully async version of rust thanks to warp and tokio

## Build 

```
cargo build
```

## Run

First, run a postgres db with docker

```
docker run -p 7878:5432 -d postgres:9.6.12
```

Then run the app

```
cargo run
```