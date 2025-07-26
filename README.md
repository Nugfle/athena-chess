# Athena Chess

Athena Chess is a configurable chess engine written in Rust, designed to run either as a backend service or locally as a CLI. The project aims to provide a fast, modular, and extensible chess bot leveraging modern Rust features and parallelism.

> **Project Status:**
> Athena Chess is in an early stage of development. Core move generation and board logic are implemented, but there is not yet a fully working bot or evaluation logic.

---

## Features

- **Bitboard-based Engine:** Efficient board representation using bitboards for fast move generation.
- **Magic Bitboards:** Precomputed attack tables for sliding pieces (rook, bishop, queen) using magic bitboard techniques.
- **Parallelized Table Generation:** Attack tables are generated in parallel for fast startup.
- **CLI and Service Modes:**
  - Run as a local CLI for experimentation.
  - Run as a backend service (TCP server) for integration with other systems.
- **Benchmarks:** Criterion-based benchmarks for performance profiling.

---

## Architecture

- **src/game/**: Core chess logic (board, pieces, moves, attack tables).
- **src/service/**: Backend TCP service for engine integration (WIP).
- **src/main.rs**: Entry point for CLI and service modes.
- **benches/**: Criterion benchmarks for engine performance.

---

## Getting Started

### Prerequisites
- Rust (edition 2024, recommended latest stable)

### Build & Run (CLI)
```sh
cargo run
```
This will initialize the engine and execute a sample move. (See `src/main.rs` for details.)

### Build & Run (Service)
To run as a TCP backend service (requires `tokio`, `serde`, and `serde_json`):
```sh
cargo run --features service -- 6969
```
This will start the service on port 6969. (Protocol WIP)

### Docker
A `Dockerfile` is provided:
```sh
docker build -t athena-chess .
docker run -p 6969:6969 athena-chess
```

---

## Development
- Benchmarks can be run with
```sh
cargo bench --features benchmark
```
- Contributions and issues are welcome!

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
