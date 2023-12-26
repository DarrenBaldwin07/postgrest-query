<h1 align="center">postgrest-query</h1>
<p align="center">A type-safe query builder-esque experience via PostgREST</p>

## Install

```bash
cargo add postgrest-query
```

## Usage

```rust
use postgrest_query::PostgrestClient;

const URL: &str = "https://org-postgrest-query-demo-inst-oltp-demo.data-1.use1.tembo.io/restapi/v1";

#[derive(serde::Serialize, serde::Deserialize)]
struct UserModel {
    id: i32,
    name: String,
    password: String,
}

let db = PostgrestClient::new(URL);

let user_query = db.from("users").find_many::<UserModel>().await;
```

## TODO

- [ ] Simple `Rust -> PostgREST -> Postgres` leveraging `serde` and resembling the [Prisma](https://github.com/prisma/prisma) query builder
- [ ] Allow providing custom schemas to enable even better type safety
- [ ] Potentially let consumers provide their own fetcher (defaults to [reqwest](https://crates.io/crates/reqwest)) for now
- [ ] Compile time query checks (similar to `sqlx`)
