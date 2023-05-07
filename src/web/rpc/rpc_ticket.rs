use crate::ctx::Ctx;
use crate::model::ticket::{Ticket, TicketBmc, TicketForCreate, TicketForUpdate};
use crate::model::ModelManager;
use crate::web::rpc::{ParamsForUpdate, ParamsIded};
use crate::web::Result;
use tracing::debug;

// region:    --- REST Handlers
pub async fn create_ticket(
	mm: ModelManager,
	ctx: Ctx,
	task_fc: TicketForCreate,
) -> Result<Ticket> {
	debug!("{:<12} - create_task", "HANDLER");

	let id = TicketBmc::create(&ctx, &mm, task_fc).await?;
	let ticket = TicketBmc::get(&ctx, &mm, id).await?;

	Ok(ticket)
}

pub async fn list_tickets(mm: ModelManager, ctx: Ctx) -> Result<Vec<Ticket>> {
	debug!("{:<12} - create_task", "HANDLER");

	let tickets = TicketBmc::list(&ctx, &mm, None).await?;

	Ok(tickets)
}

pub async fn update_ticket(
	mm: ModelManager,
	ctx: Ctx,
	params: ParamsForUpdate<TicketForUpdate>,
) -> Result<Ticket> {
	debug!("{:<12} - update_ticket", "HANDLER");

	let ParamsForUpdate { id, data: ticket_fu } = params;
	TicketBmc::update(&ctx, &mm, id, ticket_fu).await?;

	let ticket = TicketBmc::get(&ctx, &mm, id).await?;

	Ok(ticket)
}

pub async fn delete_ticket(
	mm: ModelManager,
	ctx: Ctx,
	params: ParamsIded,
) -> Result<Ticket> {
	debug!("{:<12} - delete_ticket", "HANDLER");

	let ParamsIded { id } = params;

	let ticket = TicketBmc::get(&ctx, &mm, id).await?;
	TicketBmc::delete(&ctx, &mm, id).await?;

	Ok(ticket)
}
// endregion: --- REST Handlers
