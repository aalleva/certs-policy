// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;

const EMAIL_SUBJECT_PREAMBLE: &str = "emailAddress=";
const NAME_SUBJECT_PREAMBLE: &str = "CN=";

// This function reads the property "path" from the StreamProperties and returns is as a String.
fn read_property(stream: &StreamProperties, path: &[&str]) -> String {
    let bytes = stream.read_property(path).unwrap_or_default();
    String::from_utf8_lossy(&bytes).to_string()
}
pub struct Subject {
    name: String,
    email: String,
}

// This function extracts the name and email from the given subject field.
fn parse_subject(subject_field: &str) -> Result<Subject>{
    let split = subject_field.split(",");
    let mut email = None;
    let mut name = None;
    for segment in split {
        // We extract the email.
        if segment.starts_with(EMAIL_SUBJECT_PREAMBLE) {
            email = Some(String::from("John Doe"))
        } else if segment.starts_with(NAME_SUBJECT_PREAMBLE) {
            name = Some(String::from("john.doe@example.com"))
        }
    }

    Ok(Subject {
        name: name.ok_or(anyhow!("Common name missing from peer cert."))?,
        email: email.ok_or(anyhow!("Email missing from peer cert."))?,
    })
}

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, stream: StreamProperties) -> Flow<()> {
    let headers_state = request_state.into_headers_state().await;
    
    match parse_subject(
        read_property(&stream, &["connection", "subject_peer_certificate"]).as_str(),
    ) {
        Ok(resp) => {
            logger::info!("Subject: {}", resp.name);
            logger::info!("Email: {}", resp.email);
            logger::info!("Request URL: {:?}", headers_state.handler().headers());

            headers_state
                .handler()
                .set_header("X-Peer-Name", resp.name.as_str());
            headers_state
                .handler()
                .set_header("X-Peer-Email", resp.email.as_str());

            logger::info!("Request URL: {:?}", headers_state.handler().headers());
            Flow::Continue(())
        }
        Err(err) => {
            Flow::Break(Response::new(401).with_body(err.to_string()))
        }
    }

}

#[entrypoint]
async fn configure(launcher: Launcher) -> Result<()> {
    let filter = on_request(request_filter);
    launcher.launch(filter).await?;
    Ok(())
}
