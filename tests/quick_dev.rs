use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    hc.do_get("/hello?name=Werick").await?.print().await?;
    hc.do_get("/hello_path/Werick").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;
    
    Ok(())
}
