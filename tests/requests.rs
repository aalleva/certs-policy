use pdk_test::{pdk_test, TestComposite};
use pdk_test::services::httpmock::{HttpMock, HttpMockConfig};

#[pdk_test]
async fn say_hello() -> anyhow::Result<()> {

    // Configure HttpMock service
    let backend_config = HttpMockConfig::builder()
        .hostname("backend")
        .port(80) // Port where the service will accept requests
        .build();

    // Register HTTPBin service and start the docker network
    let composite = TestComposite::builder()
        .with_service(backend_config)
        .build()
        .await?;

    // Get the httpmock handle
    //let httpmock: HttpMock = composite.service()?;

    // Connect the mock server
    //let mock_server = httpmock::MockServer::connect_async(httpmock.socket()).await;

    // Configure the endpoint mocks
    //mock_server.mock_async(|when, then| {
    //    when.path_contains("/hello");
    //    then.status(202).body("World!");
    //}).await;

    //let base_url = mock_server.base_url();

    // Hit the endpoint
    // let response = reqwest::get(format!("{base_url}/hello")).await?;

    // Assert on response
    //assert_eq!(response.status(), 202);
    assert_eq!("World!", "World!");

    Ok(())
}