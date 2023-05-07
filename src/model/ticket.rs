use crate::ctx::Ctx;
use crate::model::base::{db_create, db_delete, db_get, db_list, db_update, DbBmc};
use crate::model::{ModelManager, Result};
use modql::filter::FilterGroups;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlb::Fields;
use sqlx::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

// region:    --- Ticket Types
#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Deserialize, Serialize)]
pub struct Ticket {
	pub id: i64,

	// -- Timestamps
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,

	pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TicketForCreate {
	pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TicketForUpdate {
	pub title: Option<String>,
}
// endregion: --- Ticket Types

pub struct TicketBmc;

impl DbBmc for TicketBmc {
	const TABLE: &'static str = "ticket";
}

impl TicketBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		ticket_fc: TicketForCreate,
	) -> Result<i64> {
		db_create::<Self, _>(ctx, mm, Some(ctx.user_id()), ticket_fc).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Ticket> {
		db_get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		_filter: impl Into<Option<FilterGroups>>,
	) -> Result<Vec<Ticket>> {
		db_list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		ticket_fu: TicketForUpdate,
	) -> Result<()> {
		db_update::<Self, _>(ctx, mm, Some(ctx.user_id()), id, ticket_fu).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		db_delete::<Self>(ctx, mm, id).await
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use crate::test_utils::init_test_tracing;
	use crate::{model, test_utils};
	use anyhow::Result;
	use std::env;
	use tracing::{debug, info};

	#[tokio::test]
	async fn test_model_ticket_create() -> Result<()> {
		// -- Setup & Fixtures
		init_test_tracing();
		let mm = test_utils::init_dev_all().await;
		let root_ctx = Ctx::root_ctx();
		let title = "TEST TITLE - test_model_ticket_create";

		info!("hello");

		// -- Exec - Create
		let id = TicketBmc::create(
			&root_ctx,
			&mm,
			TicketForCreate { title: title.to_string() },
		)
		.await?;

		// -- Check - Create
		let ticket = TicketBmc::get(&root_ctx, &mm, id).await?;
		assert_eq!(title, ticket.title);

		// -- Clean - Delete
		TicketBmc::delete(&root_ctx, &mm, id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_model_ticket_delete_fail() -> Result<()> {
		// -- Setup & Fixtures
		let mm = test_utils::init_dev_all().await;
		let root_ctx = Ctx::root_ctx();
		let id = 10; // below 1000 so should have no row.

		// -- Exec
		let res = TicketBmc::delete(&root_ctx, &mm, id).await;

		// -- Check
		assert!(
			matches!(res, Err(model::Error::EntityNotFound { typ: "ticket", id })),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
