use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use std::sync::Arc;

use anyhow::Result;
use deno_core::JsRuntime;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::try_join;

use crate::http::fetch_object;

#[derive(Debug)]
pub enum Status {
    Ready,
    Error(String),
}

impl Add<Status> for Status {
    type Output = Status;
    fn add(self, other: Status) -> Status {
        // combine two errors
        match (&self, &other) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) | // self
            // ready + error = error + ready = error
            (Self::Error(_), Self::Ready) => self,
            (Self::Ready, Self::Error(_)) => other,
            // error + error = combined errors
            (Self::Error(err_lhs), Self::Error(err_rhs)) => Self::Error(format!("{}\n{}", err_lhs, err_rhs)),
        }
    }
}

impl AddAssign for Status {
    fn add_assign(&mut self, rhs: Self) {
        match (&self, &rhs) {
            // ready + ready = ready
            (Self::Ready, Self::Ready) | // self
            // ready + error = error + ready = error
            (Self::Error(_), Self::Ready) => { /* *self = self is redundant and also doesn't work */ }
            (Self::Ready, Self::Error(_)) => { *self = rhs; }
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

fn unwrap_json_string(val: &Value, name: impl Display) -> Result<&String> {
    // pulls String out of Value
    // name is just for debugging/tracing, not strictly necessary for functionality
    if let Value::String(s) = val {
        Ok(s)
    } else {
        Err(anyhow::anyhow!(
            "{name} is not a string, but instead: {val:?}"
        ))
    }
}

async fn check_staples(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(
        host,
        "js/jssrc/model/startwlm/Hme_StplPnch.model.htm",
        runtime,
    )
    .await?;
    let mut status = Status::Ready;
    for (key, val) in obj.iter() {
        let s = unwrap_json_string(val, format!("Stapler status key {key}"))?;
        match s.as_str() {
            "Enable" | "Nothing" => { /*status += Status::Ready // redundant add */ } // no stapler is still ready i think
            _ => status += Status::Error(format!("Stapler error for {key}: {s}")),
        }
    }
    Ok(status)
}

async fn check_toner(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Toner.model.htm", runtime).await?;
    let mut status = Status::Ready;
    static TONER_KEYS: [&str; 4] = ["Black", "Cyan", "Magenta", "Yellow"];
    let Value::Array(toner_arr) = &obj["Renaming"] else {
        return Err(anyhow::anyhow!(
            "Toner object does not have a Renaming key."
        ));
    };
    // the printer has a "ColorOrMono" key but i'd rather just enumerate the array directly
    for (i, color) in toner_arr.iter().enumerate() {
        let Value::Number(toner_level) = color else {
            return Err(anyhow::anyhow!("Toner 'Renaming' key {i} is not a number."));
        };
        const THRESHOLD: f64 = 15f64;
        let Some(level) = toner_level.as_f64() else {
            return Err(anyhow::anyhow!(
                "Toner level {toner_level} is not a valid f64."
            ));
        };
        if level < THRESHOLD {
            // the actual core logic wrapped by all this error handling
            let color_name = match TONER_KEYS.get(i) {
                Some(color) => color.to_string(),
                None => format!("Unknown color {i}"),
            };
            status += Status::Error(format!("{color_name} Toner is at {toner_level}%"));
        }
    }
    let s = unwrap_json_string(&obj["wasteToner"], "wasteToner key")?;
    static WASTE_TONER_STATUSES: [&str; 4] = [
        "Warning", "Full", "OK", // seems to be only "Ready" value?
        "Removed",
    ];
    match s.parse::<usize>()? {
        2 => { /*status += Status::Ready // redundant add */ }
        i @ (0 | 1 | 3) => {
            status += Status::Error(format!("Waste Toner status is {}", WASTE_TONER_STATUSES[i]))
        }
        _ => status += Status::Error(format!("Waste Toner status is: {s}")),
    }
    Ok(status)
}
// ripped this from the cursed printer js, hopefully wont change...
static STATUSES: [&str; 16] = [
    "Printing...",
    "Scanning...",
    "Ready.",
    "Toner Low...", // error
    "OK",
    "Connected phone in use.",
    "Dialing...",
    "Receiving...",
    "Sending...",
    "Error has occurred.", // error
    "Preparing...",
    "Sleeping...",
    "Cannot recognize.", // error
    "Adjusting...",
    "Phone is off the hook.",
    "Suspending...",
];
static ERRORS: [usize; 3] = [
    // map to the 3 errors above ^, also ripped from printer js
    3, 9, 12,
];
async fn check_status(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(
        host,
        "js/jssrc/model/startwlm/Hme_DvcSts.model.htm",
        runtime,
    )
    .await?;
    let mut status = Status::Ready;

    let pds = unwrap_json_string(&obj["PrinterDeviceStatus"], "PrinterDeviceStatus key")?;
    let pdsint = pds.parse::<usize>()?;
    if ERRORS.contains(&pdsint) {
        status += Status::Error(format!("Printer status is {}", STATUSES[pdsint]));
    }

    let sds = unwrap_json_string(&obj["ScannerDeviceStatus"], "ScannerDeviceStatus key")?;
    let sdsint = sds.parse::<usize>()?;
    if ERRORS.contains(&sdsint) {
        status += Status::Error(format!("Scanner status is {}", STATUSES[sdsint]));
    }

    let pm = unwrap_json_string(&obj["PanelMessage"], "PanelMessage key")?;
    if pm != "Ready." {
        status += Status::Error(format!("Panel message is: {pm}"));
    }

    Ok(status)
}

async fn check_paper(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Paper.model.htm", runtime).await?;
    // TODO
    // dbg!(&obj);
    Ok(Status::Ready)
}

async fn device_info(host: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<String> {
    // i can't get the full unique location-based names, so this will have to do
    let obj = fetch_object(
        host,
        "js/jssrc/model/dvcinfo/dvcconfig/DvcConfig_Config.model.htm",
        runtime,
    )
    .await?;
    let model = unwrap_json_string(&obj["model"], "Model key")?;
    let location = unwrap_json_string(&obj["location"], "location key")?;
    let host_name = unwrap_json_string(&obj["hostName"], "hostName key")?;
    let serial_number = unwrap_json_string(&obj["serialNumber"], "serialNumber key")?;
    Ok(format!(
        "{host_name} at {location} (model {model} serial {serial_number})"
    ))
}

pub async fn check_printer(ip: &str, runtime: Arc<Mutex<JsRuntime>>) -> Result<()> {
    let host = &format!("https://{ip}");
    /*
    staples: https://{ip}/js/jssrc/model/startwlm/Hme_StplPnch.model.htm
    toner: https://{ip}/js/jssrc/model/startwlm/Hme_Toner.model.htm
    device status: https://{ip}/js/jssrc/model/startwlm/Hme_DvcSts.model.htm
    paper: https://{ip}/js/jssrc/model/startwlm/Hme_Paper.model.htm
    device info https://{ip}/js/jssrc/model/dvcinfo/dvcconfig/DvcConfig_Config.model.htm
    */

    let (staples_obj, toner_obj, status_obj, paper_obj, device_info) = try_join!(
        check_staples(host, runtime.clone()),
        check_toner(host, runtime.clone()),
        check_status(host, runtime.clone()),
        check_paper(host, runtime.clone()),
        device_info(host, runtime.clone())
    )?;
    println!(
        "Status for {device_info}: {}",
        staples_obj + toner_obj + status_obj + paper_obj
    );
    Ok(())
}
