# AGENTS.md

## Project

Rust client bindings for Errepi Net microservices (gRPC, tonic). No server code. Edition 2024, rustc >= 1.85.

- Crate `errepi-rs` (workspace root, lib target); modules: `errepi::cron` (client `CronConfigurator` + domain models + conversions) and `errepi::regs` (client `GenericRegsClient` + domain models + conversions). Shared `AppInfo` model in `errepi::models` (defined in both service protos).
- `protos/` is a git submodule (`errepinet-sys-services-protos`) with the gRPC `.proto` defs, used only for development. `build.rs` dual-mode: with submodule + `protoc` on PATH (libprotoc 3.21.12 in use) it compiles via `tonic-prost-build` (`compile_protos`) into `OUT_DIR` and syncs the generated modules to `src/generated/` (commit the diff when bumping); without them (cargo git dependencies do NOT initialize submodules) it copies the committed pre-generated modules from `src/generated/` into `OUT_DIR` — no protoc needed by consumers. Code reaches `tonic::include_proto!` (`src/pb.rs`) via `OUT_DIR` in both modes. Bump pointer with `git -C protos pull && cargo build && git add protos src/generated && git commit`.
- Domain models mirror the pydantic v2 models of `errepi-py` (same fields, snake_case). Conversions between domain models and prost-generated messages live in `conversions.rs` per submodule (same split as `errepi-py` `conversions.py`). Enums map by name from pb i32; oneofs (`JobFrequency`, `JobType`) map to Rust enums.
- Client interfaces mirror the RPCs of the proto services (same method names, snake_case; `tenant_id`/`namespace` params where the proto requests have them). All methods async, take `&mut self` (tonic generated clients).
- Transient gRPC failures (`UNAVAILABLE`, `DEADLINE_EXCEEDED`) retried with exponential backoff; `max_retries` and `retry_delay_secs` configurable on the client configuration (defaults 3 retries, 1s base delay, doubling each attempt). Non-transient errors return immediately.

## Commands / environment

- `cargo build` / `cargo test` / `cargo clippy --all-targets -- -D warnings` from the crate root.
- Unit tests only (conversions roundtrips, enum/oneof mapping, retry predicate, config defaults). Integration tests against live services are NOT part of the suite; sanity-check with `cargo run --example cron_example` / `cargo run --example regs_example` (both need a live gRPC service).
- `Cargo.lock` committed (library consumers use it as a source of truth per crate convention? keep committed for reproducibility of the workspace).
- Environment managed with cargo; no rust-toolchain file (system rustc 1.97.1).

## Conventions

- **Always use the caveman skill** (`.agents/skills/caveman`) for responses — repo-local skill; adjust with `/caveman lite|full|ultra`, stop with "stop caveman". Keep code, errors, symbols exact.
- Serde derives on domain models (`Serialize`/`Deserialize`) mirror the pydantic serialization surface of `errepi-py`.
- Timestamps: prost `Timestamp` ↔ `chrono::DateTime<Utc>` via `TryFrom` (UTC).
- `CronConfigurator::new(config)` and `GenericRegsClient::new(config)` take a client configuration (defaults `localhost:50051`). No env vars read by the library.
- Client configurations: `host`, `port` (u16), `max_retries` (u32), `retry_delay_secs` (u64), `Default` impl.
- Comments in English; commit messages in Italian (per errepinet convention).
- Every source file starts with the Errepi Net copyright license header — keep it when touching a file.
- `conint(ge=0)` in the python models maps to unsigned integer types (`u32`/`u64`), enforced by type.
