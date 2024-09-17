// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
use pdk::logger;
use crate::generated::config::Config;

pub struct Subject {
    name: String,
    email: String,
}

fn parse_subject() -> Result<Subject>{
    logger::info!("Into parse_subject function");
    Ok(Subject {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    })
}

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, stream: StreamProperties) -> Flow<()> {
    let headers_state = request_state.into_headers_state().await;
    
    match parse_subject() {
        Ok(subject) => {
            logger::info!("Subject: {}", subject.name);
            logger::info!("Email: {}", subject.email);
            logger::info!("Request URL: {:?}", headers_state.handler().headers());
            Flow::Continue(())
        }
        Err(e) => {
            Flow::Break(Response::new(401))
        }
    }

}

#[entrypoint]
async fn configure(launcher: Launcher) -> Result<()> {
    let filter = on_request(request_filter);
    launcher.launch(filter).await?;
    Ok(())
}
