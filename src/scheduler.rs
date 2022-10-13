use std::env;

use log::info;

pub struct Scheduler;

impl Scheduler {
    pub async fn task(&self) {
        info!("Task started");
        if let Err(e) = reqwest::Client::new()
            .delete(format!("{}/texts/old", env::var("SERVICE_URL").unwrap()))
            .basic_auth(
                env::var("BASIC_AUTH_USERNAME").unwrap(),
                Some(env::var("BASIC_AUTH_PASSWORD").unwrap()),
            )
            .send()
            .await
        {
            println!("{}", e);
        }
        info!("Task completed");
    }
}
