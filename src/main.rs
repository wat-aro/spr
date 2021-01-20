use colored::Colorize;
use spr::access_token::{Params as AccessTokenParams, Response as AccessTokenResponseType};
use spr::device_code::{Params as DeviceCodeParams, Response as DeviceCodeResponse};
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client_id = "2db88301ea022dd5bc00";
    let device_code_params = DeviceCodeParams {
        client_id,
        scope: "repo",
    };

    let request = reqwest::Client::new()
        .post("https://github.com/login/device/code")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .json(&device_code_params);

    let device_code_response: DeviceCodeResponse = request.send().await?.json().await?;

    println!(
        "{} First copy your one-time code: {}",
        "!".yellow(),
        device_code_response.user_code
    );
    print!(
        "{} to open github.com in your browser... ",
        "Press Enter".bold()
    );
    stdout().flush().unwrap();
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();

    let launcher = env::var("BROWSER").unwrap_or(String::from("open"));
    Command::new(launcher)
        .args(&[device_code_response.verification_uri])
        .output()
        .expect("Failed to open browser");

    let grant_type = "urn:ietf:params:oauth:grant-type:device_code";
    let access_token_params = AccessTokenParams {
        client_id,
        device_code: &device_code_response.device_code,
        grant_type,
    };

    poll(&access_token_params).await?;

    Ok(())
}

async fn poll(params: &AccessTokenParams<'_>) -> Result<AccessTokenResponseType, reqwest::Error> {
    let request = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .json(&params);

    let response = request
        .send()
        .await?
        .json::<AccessTokenResponseType>()
        .await?;

    Ok(response)
}
