use std::fmt::{Display, Formatter};
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
    Error(String)
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
    let status = match &obj["staple_a"] {
        Value::String(s) => {
            match s.as_str() {
                "Enable" => {Status::Ready},
                "Nothing" => {Status::Ready}, // no stapler is still ready i think
                _ => {Status::Error(s.clone())}
            }
        },
        // invalid statuses
        Value::Null => {return Err(anyhow::anyhow!("Stapler status not found."))}, //
        _ => return Err(anyhow::anyhow!("Stapler status is not a string"))
    };
    Ok(status)
}
async fn check_toner(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Toner.model.htm", runtime).await?;
    // TODO
    Ok(Status::Ready)
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

pub async fn check_printer(ip: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<()>{
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
    dbg!(staples_obj, toner_obj, status_obj, paper_obj);
    Ok(())
}