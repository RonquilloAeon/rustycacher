https://doc.rust-lang.org/cargo/guide/project-layout.html
https://stackoverflow.com/a/57767413/5340241
https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424

- Create a library that hashes to redis
- Create a binary that caches from stdin
- Create an API binary that uses the library

## Getting started
```
docker-compose up
export DB_URL=redis://localhost:6379
echo 'Hello everyone!' | cargo run
cargo run --bin api
```
