# Forex Starter (Rust)

A beginner-friendly Rust web app that serves a tiny forex dashboard with sample data and a chart.

## One-button setup

```bash
./run.sh
```

The script installs Rust (via rustup) if needed, then runs the server on `http://localhost:3000`.

## What you will learn

- How Rust serves HTTP requests using Axum.
- How to shape JSON data for a frontend chart.
- How to run a lightweight server locally or on a Linux VM.

## API endpoints

- `GET /` - dashboard UI
- `GET /api/summary` - quick market summary
- `GET /api/ohlc` - sample OHLC data

## Deploy tips

1. Clone your repo on a server (Ubuntu, Debian, etc.).
2. Run `./run.sh`.
3. Open port `3000` or reverse proxy with Nginx.

> Data is synthetic for learning only.
