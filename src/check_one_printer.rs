use deno_core::JsRuntime;
use crate::http::fetch_object;
use anyhow::Result;
use tokio::try_join;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn check_printer(ip: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<()>{
    let host = &format!("https://{ip}");
    /*
    staples: https://165.134.155.129/js/jssrc/model/startwlm/Hme_StplPnch.model.htm
    toner: https://165.134.155.129/js/jssrc/model/startwlm/Hme_Toner.model.htm
    device status: https://165.134.155.129/js/jssrc/model/startwlm/Hme_DvcSts.model.htm
    paper: https://165.134.155.129/js/jssrc/model/startwlm/Hme_Paper.model.htm
    */
    
    let (staples_obj, toner_obj, status_obj, paper_obj) = try_join!(
        fetch_object(host, "js/jssrc/model/startwlm/Hme_StplPnch.model.htm", runtime.clone()),
        fetch_object(host, "js/jssrc/model/startwlm/Hme_Toner.model.htm", runtime.clone()),
        fetch_object(host, "js/jssrc/model/startwlm/Hme_DvcSts.model.htm", runtime.clone()),
        fetch_object(host, "js/jssrc/model/startwlm/Hme_Paper.model.htm", runtime.clone())
    )?;
    dbg!(staples_obj, toner_obj, status_obj, paper_obj);
    Ok(())
}