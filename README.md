# errepi-rs

Rust bindings for Errepi Net microservices (gRPC, tonic). Pure client library, no server code. Mirror of [`errepi-py`](https://github.com/errepinet/errepi-py).

## Requirements

- Rust edition 2024, rustc >= 1.85
- `protoc` on PATH (libprotoc 3.21.12 in use) — required at build time to compile the gRPC protos
- `protos/` git submodule checked out (`git submodule update --init`)

## Add as dependency

```toml
[dependencies]
errepi-rs = { git = "ssh://git@github.com/errepinet/errepi-rs.git" }
```

For a specific version/tag:

```toml
errepi-rs = { git = "ssh://git@github.com/errepinet/errepi-rs.git", tag = "v0.1.0" }
```

## Crate layout

| Module | Contents |
|---|---|
| `errepi::cron` | `CronConfigurator` client, cron domain models, conversions |
| `errepi::regs` | `GenericRegsClient` client, regs domain models, conversions |
| `errepi::models` | shared `AppInfo` model |
| `errepi::conf` | `CronClientConfiguration`, `RegsClientConfiguration` |
| `errepi::retry` | transient gRPC error retry with exponential backoff |

Domain models mirror the pydantic v2 models of `errepi-py` (same fields, snake_case), with serde `Serialize`/`Deserialize` derives. Conversions to/from the prost-generated messages live in `conversions.rs` per submodule.

## Client configuration

Both clients take a configuration struct implementing `Default`:

| Field | Default | Meaning |
|---|---|---|
| `host` | `"localhost"` | gRPC server host |
| `port` | `50051` | gRPC server port (`50052` for regs in examples) |
| `max_retries` | `3` | retries for transient failures |
| `retry_delay_secs` | `1` | base backoff delay (doubles each attempt) |

Transient failures (`UNAVAILABLE`, `DEADLINE_EXCEEDED`) are retried with exponential backoff `delay * 2^attempt`; non-transient errors return immediately.

## Examples

```rust
use errepi_rs::cron::CronConfigurator;
use errepi_rs::conf::CronClientConfiguration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cron = CronConfigurator::new(CronClientConfiguration::default()).await?;
    let info = cron.app_info().await?;
    println!("App info: {info:?}");
    Ok(())
}
```

More detailed examples in the `examples/` directory:

```bash
cargo run --example cron_example
cargo run --example regs_example
```

Both need a live gRPC service (cron on port 50051, regs on port 50052).

## Cron client

`CronConfigurator::new(config)` — 13 methods (all async, `&mut self`):

- `app_info()`
- `get_configuration(tenant_id, namespace, name)`
- `set_configuration(tenant_id, namespace, name, config)`
- `unset_configuration(tenant_id, namespace, name)`
- `list_jobs(tenant_id, namespace)`
- `create_job(tenant_id, namespace, job)`
- `update_job(tenant_id, namespace, job_id, job)`
- `delete_job(tenant_id, namespace, job_id)`
- `get_job(tenant_id, namespace, job_id)`
- `job_results(tenant_id, namespace, job_id)`
- `get_ref(tenant_id, namespace, key)`
- `set_ref(tenant_id, namespace, key, reference)`
- `unset_ref(tenant_id, namespace, key)`

Plus the free helper `http_job_type(http_job: HTTPJob) -> JobType`.

## Regs client

`GenericRegsClient::new(config)` — 6 methods (all async, `&mut self`):

- `app_info()`
- `states_list(search: Option<&str>)`
- `cities_list(search: Option<&str>)`
- `caps_list(search: Option<&str>)`
- `provinces_list(search: Option<&str>)`
- `regions_list(search: Option<&str>)`

The optional `search` filters by prefix, e.g. `Some("Roma")` for cities.

## Development

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

MIT — see the [LICENSE](LICENSE) file. Copyright © 2023-2026 Errepi Net S.R.L.
