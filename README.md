# FitTrack

FitTrack is a some small experimental app for tracking exercises, workouts, sets, and progress

## Stack

- Rust
- eframe / egui
- axum
- PostgreSQL
- SQLx
- JWT auth

## Requirements

- Rust toolchain
- Docker and Docker Compose
- `just`

## Setup

```sh
just env
```

Fill `.env` with your own secrets:

- `JWT_SECRET`
- `POSTGRES_PASSWORD`

## Run

Start the database and server:

```sh
just up
```

Run the desktop app:

```sh
just run
```

## Useful commands

```sh
just check
just clippy
just test
just clean
just down
just restart
just rebuild
```

## Notes

- Desktop app talks to the local `axum` server over HTTP
- Default API URL: `http://127.0.0.1:7272/api`
