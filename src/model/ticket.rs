use crate::ctx::Ctx;
use crate::model::store::db::{
	db_create, db_delete, db_get, db_list, db_update, DbBmc,
};
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
		mm: &ModelManager,
		ctx: &Ctx,
		ticket_fc: TicketForCreate,
	) -> Result<i64> {
		db_create::<Self, _>(mm.db(), Some(ctx.user_id()), ticket_fc).await
	}

	pub async fn get(mm: &ModelManager, _ctx: &Ctx, id: i64) -> Result<Ticket> {
		db_get::<Self, _>(mm.db(), id).await
	}

	pub async fn list(
		mm: &ModelManager,
		_ctx: &Ctx,
		_filter: impl Into<Option<FilterGroups>>,
	) -> Result<Vec<Ticket>> {
		db_list::<Self, _>(mm.db()).await
	}

	pub async fn update(
		mm: &ModelManager,
		ctx: &Ctx,
		id: i64,
		ticket_fu: TicketForUpdate,
	) -> Result<()> {
		db_update::<Self, _>(mm.db(), Some(ctx.user_id()), id, ticket_fu).await
	}

	pub async fn delete(mm: &ModelManager, _ctx: &Ctx, id: i64) -> Result<()> {
		db_delete::<Self>(mm.db(), id).await
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
			&mm,
			&root_ctx,
			TicketForCreate { title: title.to_string() },
		)
		.await?;

		// -- Check - Create
		let ticket = TicketBmc::get(&mm, &root_ctx, id).await?;
		assert_eq!(title, ticket.title);

		// -- Clean - Delete
		TicketBmc::delete(&mm, &root_ctx, id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_model_ticket_delete_fail() -> Result<()> {
		// -- Setup & Fixtures
		let mm = test_utils::init_dev_all().await;
		let root_ctx = Ctx::root_ctx();
		let id = 10; // below 1000 so should have no row.

		// -- Exec
		let res = TicketBmc::delete(&mm, &root_ctx, id).await;

		// -- Check
		assert!(
			matches!(res, Err(model::Error::EntityNotFound { typ: "ticket", id })),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}
// endregion: --- Tests
