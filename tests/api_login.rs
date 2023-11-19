use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn api_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    let login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    login.await?.print().await?;
    Ok(())
}
