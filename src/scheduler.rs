use std::{env, time::Duration};

use actix::prelude::*;

const DURE: Duration = Duration::from_secs(10);
pub struct Scheduler;

impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");

        ctx.run_later(DURE, move |this, ctx| this.schedule_task(ctx));
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

impl Scheduler {
    fn schedule_task(&self, ctx: &mut Context<Self>) {
        if let Err(e) = reqwest::blocking::Client::new()
            .delete(format!("{}/texts/old", env::var("SERVICE_URL").unwrap()))
            .basic_auth(
                env::var("BASIC_AUTH_USERNAME").unwrap(),
                Some(env::var("BASIC_AUTH_PASSWORD").unwrap()),
            )
            .send()
        {
            println!("{}", e);
        }
        ctx.run_later(DURE, move |this, ctx| this.schedule_task(ctx));
    }
}
