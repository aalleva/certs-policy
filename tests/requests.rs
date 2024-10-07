// Copyright 2023 Salesforce, Inc. All rights reserved.

mod common;

use httpmock::MockServer;
use pdk_test::{pdk_test, TestComposite};
use pdk_test::port::Port;
use pdk_test::services::flex::{ApiConfig, Flex, FlexConfig, PolicyConfig};
use pdk_test::services::httpmock::{HttpMockConfig, HttpMock};

use common::*;

// Directory with the configurations for the `hello` test.
const HELLO_CONFIG_DIR: &str =  concat!(env!("CARGO_MANIFEST_DIR"), "/tests/requests/hello");

// Flex port for the internal test network
const FLEX_PORT: Port = 8081;

// This integration test shows how to build a test to compose a local-flex instance
// with a MockServer backend
#[pdk_test]
async fn hello() -> anyhow::Result<()> {

    // Configure an HttpMock service
    let upstream_config = HttpMockConfig::builder()
        .port(80)
        .hostname("backend")
        .build();

    // Configure a Flex service
    let policy_config = PolicyConfig::builder()
        .name(POLICY_NAME)
        .build();

    let api_config = ApiConfig::builder()
        .name("ingress-http")
        .upstream(&upstream_config)
        .path("/anything/echo/")
        .port(FLEX_PORT)
        .policies([policy_config])
        .build();

    let flex_config = FlexConfig::builder()
        .version("1.7.1")
        .hostname("local-flex")
        .with_api(api_config)
        .config_mounts([(POLICY_DIR, "policy"), (COMMON_CONFIG_DIR, "common")])
        .build();

    // Assert on the response
    assert_eq!(202, 202);

    Ok(())
}
