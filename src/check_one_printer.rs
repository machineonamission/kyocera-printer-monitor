use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use kg_js::JsEngine;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::try_join;

use crate::http::fetch_object;
use crate::js::Object;
use crate::r#static::*;
use crate::status::Status;
use crate::{http, json_utils::*};

async fn check_staples(host: &str, runtime: Rc<Mutex<JsEngine>>) -> Result<Status> {
    let obj = fetch_object(
        host,
        "js/jssrc/model/startwlm/Hme_StplPnch.model.htm",
        runtime,
    )
    .await?;
    let mut status = Status::Ready;
    for (key, val) in obj.iter() {
        let s = unwrap_json_string(val, format!("Stapler status key {key}"))?;
        // later review of the code seems to confirm that Enable and Nothing are the 2 "ok" values and anything else is an error
        match s.as_str() {
            "Enable" | "Nothing" => { /*status += Status::Ready // redundant add */ } // no stapler is still ready i think
            _ => status += Status::Error(format!("Stapler error for {key}: {s}"), 1),
        }
    }
    Ok(status)
}

async fn check_toner(host: &str, runtime: Rc<Mutex<JsEngine>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Toner.model.htm", runtime).await?;
    let mut status = Status::Ready;

    let Value::Array(toner_arr) = &obj["Renaming"] else {
        return Err(anyhow::anyhow!(
            "Toner object does not have a Renaming key."
        ));
    };
    // the printer has a "ColorOrMono" key but i'd rather just enumerate the array directly
    for (i, color) in toner_arr.iter().enumerate() {
        let level = unwrap_json_number(color, format!("Toner 'Renaming' key {i}"))?;
        if level == 0f64 || level == -1f64 {
            // the actual core logic wrapped by all this error handling
            let color_name = match TONER_KEYS.get(i) {
                Some(color) => color.to_string(),
                None => format!("Unknown color {i}"),
            };
            // -1 seems to be an error state
            if level == -1f64 {
                status += Status::Error(
                    format!("{color_name} Toner is at an unknown level. It may be empty or not installed."),
                    1,
                );
            } else {
                status += Status::Error(format!("{color_name} Toner is at {level}%"), 1);
            }
        }
    }
    let s = unwrap_json_string(&obj["wasteToner"], "wasteToner key")?;

    match s.parse::<usize>()? {
        2 => { /*status += Status::Ready // redundant add */ }
        i @ (0 | 1 | 3) => {
            status += Status::Error(
                format!("Waste Toner status is {}", WASTE_TONER_STATUSES[i]),
                1,
            )
        }
        _ => status += Status::Error(format!("Waste Toner status is: unknown error {s}"), 1),
    }
    Ok(status)
}

async fn check_status(host: &str, runtime: Rc<Mutex<JsEngine>>) -> Result<Status> {
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
        status += Status::Error(format!("Printer status is: {}", STATUSES[pdsint]), 1);
    }

    let sds = unwrap_json_string(&obj["ScannerDeviceStatus"], "ScannerDeviceStatus key")?;
    let sdsint = sds.parse::<usize>()?;
    if ERRORS.contains(&sdsint) {
        status += Status::Error(format!("Scanner status is: {}", STATUSES[sdsint]), 1);
    }

    let pm = unwrap_json_string(&obj["PanelMessage"], "PanelMessage key")?;
    if let Status::Error(_, _) = status {
        status += Status::Error(format!("Panel message is: {pm}"), 0); // 0 cause this only activates if either 2 are true errors
    }

    Ok(status)
}

fn group_cassettes_by_type(obj: &Object, cassette_count: usize) -> Result<Vec<Vec<usize>>> {
    // groups cassettes into arrays based on their paper size and type
    let sizes = unwrap_json_array(&obj["PaperSize"], "PaperSize key")?;
    let types = unwrap_json_array(&obj["PaperType"], "PaperType key")?;

    let mut cassettes = HashMap::<[String; 2], Vec<usize>>::new();

    for cassette in 0..cassette_count {
        let size = unwrap_json_string(&sizes[cassette], format!("Cassette {cassette} PaperSize"))?;
        let ctype = unwrap_json_string(&types[cassette], format!("Cassette {cassette} PaperSize"))?;
        let key = [size.clone(), ctype.clone()];
        cassettes.entry(key).or_default().push(cassette);
    }
    let out: Vec<Vec<usize>> = cassettes.values().cloned().collect();

    Ok(out)
}

async fn check_paper(host: &str, runtime: Rc<Mutex<JsEngine>>) -> Result<Status> {
    let obj = fetch_object(host, "js/jssrc/model/startwlm/Hme_Paper.model.htm", runtime).await?;
    let mut status = Status::Ready;

    // im not 100% sure if this is the right way to count cassettes, but it seems to work?

    // i think we can always assume a printer has a MP tray,
    // and we have to since not all printers have a value saying if they have one
    let mut cassette_count =
        unwrap_json_number(&obj["getCassetCount"], "getCassetCount key")? as usize;
    // if the key does exist, regardless of if its 0 or 1, mptray is not included in the count and we bing chilling
    if obj.get("mpTrayStatus").is_none() {
        cassette_count -= 1usize // if this key doesn't exist, mptray is included in the count, subtract
    }

    // getPaperStatus returns if the printer can detect the exactish paper count vs just empty/full,
    // i dont think we need it cause getCassetLevel is just fixed at 100 or level_empty which is fine

    let type_groups = group_cassettes_by_type(&obj, cassette_count)?;

    let levels = unwrap_json_array(&obj["getCassetLevel"], "getCassetLevel key")?;
    let sizes = unwrap_json_array(&obj["PaperSize"], "PaperSize key")?;
    let types = unwrap_json_array(&obj["PaperType"], "PaperType key")?;
    let capacities = unwrap_json_array(&obj["PaperCapacity"], "PaperCapacity key")?;

    for cassettes in type_groups {
        // check if the entire group is empty
        let mut group_is_empty = true;
        for cassette in &cassettes {
            let level =
                unwrap_json_string(&levels[*cassette], format!("Cassette {cassette} level"))?;
            // let level = "level_unknown"; // for debugging
            let mut unknown = false;
            let levelint = if "level_empty" == level {
                0usize
            } else {
                // parse or else mark error
                level.parse::<usize>().unwrap_or_else(|_| {
                    unknown = true;
                    0usize
                })
            };
            if levelint != 0 {
                group_is_empty = false;
                break;
            }
        }
        if group_is_empty {
            // because we grouped them, we only need to query one cassette for the paper size and type
            let paper_size_raw = unwrap_json_string(
                &sizes[cassettes[0]],
                format!("Cassette {} PaperSize", cassettes[0]),
            )?;
            let paper_size = match PAPER_SIZES.get(paper_size_raw) {
                None => {
                    format!("Unknown size {paper_size_raw}")
                }
                Some(s) => s.to_string(),
            };
            let paper_type_raw = unwrap_json_string(
                &types[cassettes[0]],
                format!("Cassette {} PaperType", cassettes[0]),
            )?;
            let media_type = match MEDIA_TYPES.get(paper_type_raw) {
                None => {
                    format!("Unknown media type {paper_size_raw}")
                }
                Some(s) => s.to_string(),
            };

            // theoretically 1 machine can have unknown cassettes and known empty cassettes
            let mut known_cassettes = Vec::<usize>::new();
            let mut known_capacity = 0usize;
            let mut unknown_cassettes = Vec::<usize>::new();
            let mut unknown_capacity = 0usize;
            for cassette in &cassettes {
                let level =
                    unwrap_json_string(&levels[*cassette], format!("Cassette {cassette} level"))?;
                // let level = "level_unknown"; // for debugging
                let unknown = if "level_empty" == level {
                    false
                } else {
                    // if valid, it's not unknown
                    level.parse::<usize>().is_err()
                };

                let paper_capacity = unwrap_json_string(
                    &capacities[*cassette],
                    format!("Cassette {cassette} PaperCapacity"),
                )?
                .parse::<usize>()?;
                if unknown {
                    unknown_cassettes.push(*cassette);
                    unknown_capacity += paper_capacity;
                } else {
                    known_cassettes.push(*cassette);
                    known_capacity += paper_capacity;
                }
            }
            // evil formatting fuckery
            let mut status_message = Vec::<String>::new();
            for (type_cassettes, capacity, level_type_singular, level_type_plural) in [
                (known_cassettes, known_capacity, "is empty", "are empty"),
                (
                    unknown_cassettes,
                    unknown_capacity,
                    "is at an unknown level. It may be empty or not installed",
                    "are at an unknown level. They may be empty or not installed",
                ),
            ] {
                let cassettes_str = type_cassettes
                    .iter()
                    .map(|&num| (num + 1).to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                if !type_cassettes.is_empty() {
                    status_message.push(
                        if type_cassettes.len() == 1 {
                            format!(
                                "Cassette {cassettes_str} {level_type_singular}. It has a capacity of {capacity}.",
                            )
                        } else {
                            format!(
                                "Cassettes {cassettes_str} {level_type_plural}. They have a combined capacity of {capacity}.",
                            )
                        }
                    );
                }
            }
            status_message.push(format!(
                "{} {media_type} {paper_size} paper.",
                if cassettes.len() == 1 {
                    "It takes"
                } else {
                    "They take"
                }
            ));

            status += Status::Error(status_message.join(" "), 1);
        }
    }
    // dbg!(&obj);
    Ok(status)
}

async fn device_info(host: &str, runtime: Rc<Mutex<JsEngine>>) -> Result<String> {
    // i can't get the full unique location-based names, i think they only exist on papercut and the
    // printer is completely unaware, so this will have to do
    let obj = fetch_object(
        host,
        "js/jssrc/model/dvcinfo/dvcconfig/DvcConfig_Config.model.htm?arg1=0",
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

#[derive(Debug)]
pub struct Printer {
    ip: String,
    device_info: String,
    status: Status,
}

pub async fn check_printer(ip: String, runtime: Rc<Mutex<JsEngine>>) -> Result<Printer> {
    // determines if ip wants http or https
    let host = &http::get_right_host(&ip).await?;
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
    Ok(Printer {
        ip,
        device_info,
        status: staples_obj + toner_obj + status_obj + paper_obj,
    })
}

pub async fn format_check_printer(
    ip: String,
    runtime: Rc<Mutex<JsEngine>>,
    list_all: bool,
) -> (Option<String>, bool) {
    // the bool param is if it errored so we can count them up
    match check_printer(ip.clone(), runtime).await {
        Ok(printer) => match printer.status {
            Status::Ready => {
                if list_all {
                    (
                        Some(format!(
                            "{} ({}) is Ready.",
                            printer.ip, printer.device_info
                        )),
                        false,
                    )
                } else {
                    // None instead of "" so we don't print a blank line when calling println
                    (None, false)
                }
            }
            Status::Error(error, count) => (
                Some(format!(
                    "{} ({}) has {count} error{}:\n    - {}",
                    printer.ip,
                    printer.device_info,
                    if count == 1 { "" } else { "s" },
                    error.replace('\n', "\n    - ")
                )),
                true,
            ),
        },
        // intentionally not doing e:? because the stack trace is massive
        // easier to handle this here than matching later
        Err(e) => (Some(format!("Error checking {ip}: {e}")), true),
    }
}

pub async fn spreadsheet_check_printer(
    ip: String,
    runtime: Rc<Mutex<JsEngine>>,
) -> (Option<String>, bool) {
    // the bool param is if it errored so we can count them up
    match check_printer(ip.clone(), runtime).await {
        Ok(printer) => match printer.status {
            Status::Ready => (Some("Ready".to_string()), false),
            Status::Error(error, _) => (Some(error.replace('\n', ", ")), true),
        },
        // intentionally not doing e:? because the stack trace is massive
        // easier to handle this here than matching later
        Err(e) => (
            Some(format!(
                "Error checking printer: {}",
                e.to_string().replace('\n', ", ")
            )),
            true,
        ),
    }
}
