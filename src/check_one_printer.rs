use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use deno_core::JsRuntime;
use crate::http::fetch_object;
use anyhow::Result;
use tokio::try_join;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Status {
    Ready,
    Error(String),
}

impl std::ops::Add<Status> for Status {
    type Output = Status;
    fn add(self, other: Status) -> Status {
        // combine two errors
        match (&self, &other) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) => self,
            // ready + error = error + ready = error
            (Self::Ready, Self::Error(_)) => other,
            (Self::Error(_), Self::Ready) => self,
            // error + error = combined errors
            (Self::Error(err_lhs), Self::Error(err_rhs)) => Self::Error(format!("{}\n{}", err_lhs, err_rhs)),
        }
    }
}

impl AddAssign for Status {
    fn add_assign(&mut self, rhs: Self) {
        match (&self, &rhs) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) => { /* *self = self is redundant */ }
            // ready + error = error + ready = error
            (Self::Ready, Self::Error(_)) => { *self = rhs; }
            (Self::Error(_), Self::Ready) => { /* *self = self is redundant */ }
            // error + error = combined errors
            (Self::Error(err_lhs), Self::Error(err_rhs)) => { *self = Self::Error(format!("{}\n{}", err_lhs, err_rhs)) }
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ready => {
                write!(f, "Ready")
            }
            Status::Error(e) => {
                write!(f, "Error: {}", e)
            }
        }
    }
}


async fn check_staples(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_StplPnch.model.htm", runtime).await?;
    let mut status = Status::Ready;
    for (key, val) in obj.iter() {
        match val {
            Value::String(s) => {
                match s.as_str() {
                    "Enable" | "Nothing" => { /*status += Status::Ready // redundant add */ } // no stapler is still ready i think
                    _ => { status += Status::Error(format!("Stapler error for {key}: {s}")) }
                }
            }
            // invalid statuses
            Value::Null => { return Err(anyhow::anyhow!("Stapler status key {key} is null.")); } //
            e => return Err(anyhow::anyhow!("Stapler status key {key} is not a string, but instead: {e:?}"))
        };
    }
    Ok(status)
}

async fn check_toner(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Toner.model.htm", runtime).await?;
    let mut status = Status::Ready;
    dbg!(&obj);
    static TONER_KEYS: [&str; 4] = ["Black", "Cyan", "Magenta", "Yellow"];
    let Value::Array(toner_arr) = &obj["Renaming"] else { return Err(anyhow::anyhow!("Toner object does not have a Renaming key.")); };
    for (i, color) in toner_arr.iter().enumerate() {
        let Value::Number(toner_level) = color else { return Err(anyhow::anyhow!("Toner 'Renaming' key {i} is not a number.")); };
        const THRESHOLD:f64 = 15f64;
        let Some(level) = toner_level.as_f64() else { return Err(anyhow::anyhow!("Toner level {toner_level} is not a valid f64.")); };
        if level < THRESHOLD {
            let color_name = match TONER_KEYS.get(i) {
                Some(color) => color.to_string(),
                None => format!("Unknown color {i}")
            };
            status += Status::Error(format!("{color_name} Toner is at {toner_level}%"));
        }
    }
    Ok(status)
}

async fn check_status(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_DvcSts.model.htm", runtime).await?;
    // TODO
    Ok(Status::Ready)
}

async fn check_paper(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Paper.model.htm", runtime).await?;
    // TODO
    Ok(Status::Ready)
}

pub async fn check_printer(ip: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<()> {
    let host = &format!("https://{ip}");
    /*
    staples: https://{ip}/js/jssrc/model/startwlm/Hme_StplPnch.model.htm
    toner: https://{ip}/js/jssrc/model/startwlm/Hme_Toner.model.htm
    device status: https://{ip}/js/jssrc/model/startwlm/Hme_DvcSts.model.htm
    paper: https://{ip}/js/jssrc/model/startwlm/Hme_Paper.model.htm
    */

    let (staples_obj, toner_obj, status_obj, paper_obj) = try_join!(
        check_staples(host, runtime.clone()),
        check_toner(host, runtime.clone()),
        check_status(host, runtime.clone()),
        check_paper(host, runtime.clone())
    )?;
    println!("{}", staples_obj + toner_obj + status_obj + paper_obj);
    Ok(())
}