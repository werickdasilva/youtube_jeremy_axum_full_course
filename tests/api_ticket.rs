use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn api_tickets() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;
    let api_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    api_login.await?.print().await?;
    let login = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );

    login.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
