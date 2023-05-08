#![allow(unused)] // For initial development only.

use anyhow::Result;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	// sleep(Duration::from_secs(3)).await;

	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": null,
			"method": "create_task",
			"params": {
				"title": "task AAA"
			}
		}),
	);
	req_create_task.await?.print().await?;

	// region:    --- Opional Delete
	// let req_delete_tasks = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"id": 11,
	// 		"method": "delete_task",
	// 		"params": {
	// 			"id": 1001
	// 		}
	// 	}),
	// );
	// req_delete_tasks.await?.print().await?;
	// endregion: --- Opional Delete

	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 11,
			"method": "list_tasks"
		}),
	);
	req_list_tasks.await?.print().await?;

	Ok(())
}
