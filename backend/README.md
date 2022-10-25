# Links

## Crates

- [tokio](https://github.com/tokio-rs/tokio)
- [hyper](https://github.com/hyperium/hyper)
- [axum](https://github.com/tokio-rs/axum)
- [async-graphql](https://github.com/async-graphql/async-graphql)
- [sea-orm](https://github.com/SeaQL/sea-orm)
- [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust)

## Code examples
- [Rust Async-GraphQL Example: Caster API](https://github.com/bkonkle/rust-example-caster-api)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
- [rongcuid/gqlforum-rs](https://github.com/rongcuid/gqlforum-rs)

# ER diagram

```mermaid
erDiagram

users ||--o{ tasks : "1:n"
tasks ||--o{ task_statuses : "1:n"

users {
    id number PK
    name string
}

tasks {
    id number PK
    user_id number FK
    title string
    description string
}

task_statuses {
    id number PK
    task_id number FK
    status string
}
```
