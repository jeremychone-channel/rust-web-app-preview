#![allow(unused)] // For initial development only.

use anyhow::Result;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// hc.do_get("/main.rs").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	// sleep(Duration::from_secs(3)).await;

	let req_create_ticket = hc.do_post(
		"/api/rpc",
		json!({
			"id": null,
			"method": "create_ticket",
			"params": {
				"title": "ticket AAA"
			}
		}),
	);
	req_create_ticket.await?.print().await?;

	// let req_delete_tickets = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"id": 11,
	// 		"method": "delete_ticket",
	// 		"params": {
	// 			"id": 1001
	// 		}
	// 	}),
	// );
	// req_delete_tickets.await?.print().await?;

	let req_list_tickets = hc.do_post(
		"/api/rpc",
		json!({
			"id": 11,
			"method": "list_tickets"
		}),
	);
	req_list_tickets.await?.print().await?;

	Ok(())
}
