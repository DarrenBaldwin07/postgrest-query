<h1 align="center">postgrest-query</h1>
<p align="center">A type-safe query builder-esque experience for <a href='https://github.com/PostgREST/postgrest' target='_blank'>PostgREST</a></p>

## Install

```bash
cargo add postgrest-query
```

## Usage

```rust
use postgrest_query::PostgrestClient;

const URL: &str = "https://org-postgrest-query-demo-inst.data-1.use1.tembo.io/restapi/v1";

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    id: i32,
    name: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), std::error::Error> {
    let db = PostgrestClient::new(URL);

    let user_query = db.from("users").find_many::<User>().exec().await;
}
```

## TODO

- [x] Simple `Rust -> PostgREST -> Postgres` leveraging `serde` and resembling the [Prisma](https://github.com/prisma/prisma) query builder
- [ ] Support the entire PostgREST API
- [ ] Potentially let consumers provide their own handler (defaults to [reqwest](https://crates.io/crates/reqwest) for now)
- [ ] Compile time query checks? (similar to `sqlx`)
- [ ] Explore potential ports of this to other languages (TypeScript, Python, Go, etc.)

> Issues and PRs are welcome!
