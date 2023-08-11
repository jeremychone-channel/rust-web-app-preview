**IMPORTANT - THIS IS JUST A PREVIEW REPO, WILL GET REMOVED WHEN The `awesomeapp-dev/rust-web-app` repo will be available.**

Rust Axum Level Up Course

YouTube Full Course: _coming soon_

MIT OR Apache, all free to use. 

## Starting the DB

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

## Dev

```sh
# Terminal 1 - For web-server run.
cargo run
# or `cargo run -p web-server`

# Terminal 2 - For quick dev (part of web-server/examples for now).
cargo run -p web-server --example quick_dev
```

or unit tests

```sh
# threads=1 for now, since recreate DB and could cause issue when concurrent.
cargo test -- --test-threads=1
```


## Dev (HOT-RELOAD)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w crates/services/web-server/src -w .cargo/ -x "run"

# Terminal 2 - For quick dev.
cargo watch -q -c -w crates/services/web-server/examples -x "run -p web-server --example quick_dev"
```

Unit test HOT-RELOAD

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```



## Generate new key

```sh
cargo run -p gen_key
```

<br /><br />
_[This repo](https://github.com/jeremychone-channel/rust-web-app-preview)_