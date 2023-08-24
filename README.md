**IMPORTANT - THIS IS JUST A PREVIEW REPO, WILL GET REMOVED WHEN The `awesomeapp-dev/rust-web-app` repo will be available.**

Rust Axum Level Up Course

YouTube Full Course: _coming soon_

MIT OR Apache, all free to use. 

This is a multi-crate Rust Web application with the following structure. 

- `crates/libs` - These are the library crates utilized by the service crates.
	- `base` - Provides primitive utilities for other libraries and services. Should remain as minimalist as possible
	- `core` - provides core functionalities for application services (e.g., Web Service and Job Service). Key sub-modules within this library include: 
		- **model** layer, accountable for all data typing and access.
		- **ctx** layer, responsible of holding the authenticated request context for authorization and tracability. 
		- **event** layer will be responsible to provide the event infrastructure of the applicaction services.
- `crates/services` - Represents the application services. For **rust-web-app**, being a singular web application, there's just one service: 
	- `web-server` - The Web server servicing the Web APIs and static files.
- `crates/tools` - Comprises development tool crates, typically used as binary executables (e.g., executing `cargo run -b gen_key` to generate a new key that can be used for the application).

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
cargo run -p web-server

# Terminal 2 - For quick dev (part of web-server/examples for now).
cargo run -p web-server --example quick_dev
```

or unit tests

```sh
# threads=1 for now
cargo test 
```


## Dev (LIVE-RELOAD)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w crates/services/web-server/src -w .cargo/ -x "run -p web-server"

# Terminal 2 - For quick dev.
cargo watch -q -c -w crates/services/web-server/examples -x "run -p web-server --example quick_dev"
```

Unit test LIVE-RELOAD

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