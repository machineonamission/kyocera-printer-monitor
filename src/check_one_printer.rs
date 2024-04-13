use deno_core::JsRuntime;
use crate::http::fetch_object;
use anyhow::Result;

pub async fn check_printer(ip: &str, runtime: &mut JsRuntime) -> Result<()>{
    let host = &format!("https://{ip}");
    let printer_status_obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_DvcSts.model.htm", runtime).await?;
    dbg!(printer_status_obj);
    Ok(())
}