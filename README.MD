<h1 align="center">postgrest-query</h1>
<p align="center">A type-safe query builder esque experience via PostgREST</p>

## Install

```bash
cargo add postgrest-query
```

## Usage

```rust
use postgrest_query::PostgrestClient;
const URL: &str = "http://localhost:3000";
let client = PostgrestClient::new("http://localhost:3000");

```

## TODO

- [ ] Simple `Rust -> PostgREST -> Postgres` and make `serde` compatible
- [ ] Allow providing custom schemas
- [ ] Potentially let consumers provide their own fetcher (defaults to [reqwest](https://crates.io/crates/reqwest))
- [ ] Compile time query checks (similar to `sqlx`)
