**IMPORTANT - THIS IS JUST A PREVIEW REPO, WILL GET REMOVED WHEN The `rust-axum-base` repo will be available.**

Rust Axum Level Up Course

YouTube Full Course: _coming soon_

MIT OR Apache, all free to use. 

# Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

# Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - For quick dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

or cargo watch the unit tests.

```sh
# Important - set threads to 1 since many test might use DB.
cargo watch -q -c -x "test -- --test-threads=1"
```


# Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For quick dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

or unit tests

```sh
# threads=1 for now, since recreate DB and could cause issue when concurrent.
cargo test -- --test-threads=1
```


<br /><br />
_[This repo](https://github.com/jeremychone-channel/rust-axum-base-preview)_