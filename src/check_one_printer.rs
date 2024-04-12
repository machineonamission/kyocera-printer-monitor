use deno_core::JsRuntime;
use crate::http::fetch_object;
use anyhow::Result;

pub fn check_printer(ip: &str, runtime: &mut JsRuntime) -> Result<()>{
    let printer_status_obj = fetch_object(&format!("http://{ip}/js/jssrc/model/startwlm/Hme_DvcSts.model.htm"), runtime)?;
    dbg!(printer_status_obj);
    Ok(())
}