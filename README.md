**IMPORTANT - THIS IS JUST A PREVIEW REPO, WILL GET REMOVED WHEN The `rust-axum-base` repo will be available.**

Rust Axum Level Up Course

YouTube Full Course: _coming soon_

MIT OR Apache, all free to use. 

# Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# (optional, to see all sql commands on the postgres side)
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# Then, in the psql console, type 
ALTER DATABASE postgres SET log_statement = 'all';
```

# Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - For quick dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

# Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For test.
cargo test quick_dev -- --nocapture
```


<br /><br />
_[This repo](https://github.com/jeremychone-channel/rust-axum-base-preview)_