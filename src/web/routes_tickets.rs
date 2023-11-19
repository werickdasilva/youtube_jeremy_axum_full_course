use crate::{
    context::{self, Context},
    error::Result,
    model::{ModelController, Ticket, TicketForCreate},
};
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

async fn create_ticket(
    context: Context,
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(context, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    context: Context,
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets(context).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    context: Context,
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_tickets(context, id).await?;

    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
