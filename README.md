# Athena Chess

Athena Chess is a configurable chess engine written in Rust, designed to run either as a backend service or locally as a CLI. The project aims to provide a fast, modular, and extensible chess bot leveraging modern Rust features and parallelism.

> **Project Status:**
> Athena Chess is currently undergoing a major rewrite with a new workspace-based architecture for better modularity and maintainability. Core move generation and board logic are implemented in the `athena-core` crate.

---

## Project Structure

The project is organized as a Rust workspace with three main crates:

- **athena-core**: Core chess engine functionality including board representation, move generation, and evaluation
- **athena-cli**: Command-line interface for local usage and testing
- **athena-service**: Backend service implementation for remote integration

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

### athena-core
- **src/game/**: Core chess logic including board representation and move generation
  - **board/**: Bitboard-based board representation
  - **attack_tables/**: Magic bitboard attack table generation
- **src/evaluation/**: Position evaluation and search algorithms
- **benches/**: Criterion benchmarks for engine performance

### athena-cli
- Command-line interface for local usage
- Interactive mode for testing and development

### athena-service
- Backend TCP service for remote integration
- JSON-based protocol for game state and move commands

---

## Getting Started

### Prerequisites
- Rust (edition 2024, recommended latest stable)

### Build & Run (CLI)
```sh
cargo run -p athena-cli
```
This will start the interactive CLI mode.

### Build & Run (Service)
To run as a TCP backend service:
```sh
cargo run -p athena-service -- --port 6969
```
This will start the service on port 6969.

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
