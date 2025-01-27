use std::io::stdin;
use std::rc::Rc;

use anyhow::Result;
use arboard::Clipboard;
use kg_js::JsEngine;
use tokio::sync::Mutex;

mod check_one_printer;
mod http;
mod js;
mod json_utils;
mod remember;
mod r#static;
mod status;
mod update;

#[derive(PartialEq, Copy, Clone)]
enum OutputMode {
    Spreadsheet,
    ListErrors,
    ListAll,
}

fn credits() {
    println!(
        "🖨️ Kyocera Printer Monitor v{} by Melody aka Machine on a Mission\n\
        https://machineonamission.me/\n\
        Source code and downloads: https://github.com/machineonamission/kyocera-printer-monitor",
        env!("CARGO_PKG_VERSION")
    );
    println!("Press enter to continue.");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
}

async fn core() -> Result<()> {
    // for debugging
    // let async_runtime = Rc::new(Mutex::new(js::init()));
    // dbg!(check_one_printer::check_printer(String::from("165.134.48.176"), async_runtime).await.unwrap());
    // dbg!(http::get_right_host("165.134.48.176").await.unwrap());
    // dbg!(http::get_right_host("10.170.16.1").await.unwrap());
    // return;

    println!(
        "🖨️ Welcome to Kyocera Printer Monitor v{}!\n",
        env!("CARGO_PKG_VERSION")
    );
    tokio::spawn(async move {
        if let Err(e) = update::check_for_updates().await {
            eprintln!("Error checking for updates: {e}\n\
            You can see the latest version here: https://github.com/machineonamission/kyocera-printer-monitor/releases/latest");
        }
    });

    // get user preferences
    let outmode: OutputMode;
    let sin = stdin();
    loop {
        println!(
            "How should the results be shown?:\n\
        1) Output in spreadsheet mode\n\
        2) List only the errors\n\
        3) List statuses of all printers\n\
        4) About\n\
        5) Exit"
        );
        let mut s = String::new();
        sin.read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim() {
            "1" => {
                outmode = OutputMode::Spreadsheet;
                break;
            }
            "2" => {
                outmode = OutputMode::ListErrors;
                break;
            }
            "3" => {
                outmode = OutputMode::ListAll;
                break;
            }
            "4" => {
                credits();
            }
            "5" => {
                return Ok(());
            }
            e => {
                println!("Invalid option: {}", e);
            }
        }
    }

    // get IPs
    let mut ips = Vec::new();
    let sin = stdin();
    let mut retrieved_from_file;
    loop {
        println!(
            "How will the printers be inputted?:\n\
        1) Use same printers as last time\n\
        2) Paste from clipboard\n\
        3) Enter manually\n\
        4) Exit"
        );
        retrieved_from_file = false;
        let mut s = String::new();
        sin.read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim() {
            "1" => {
                // reuse mode
                let ok = remember::printer_list_exists()?;
                if ok {
                    ips = remember::read_printer_list()?;
                    retrieved_from_file = true;
                } else {
                    // i forgor 💀
                    println!("No saved printer list found. Run the program once to save a list.");
                    continue; // ask again for input, maybe the user didnt know
                }
            }
            "2" => {
                // paste mode
                println!("Pasting from clipboard...");
                let mut clipboard = Clipboard::new()?;
                for line in clipboard.get_text()?.lines() {
                    ips.push(line.trim().to_string());
                }
            }
            "3" => {
                // manual input mode
                println!(
                    "Enter in the IPs of the printers, separated by newlines. Press enter twice when you are done."
                );
                // repeatedly loop over stdin until we get an empty line, so the user can input multiple lines
                loop {
                    let mut s = String::new();
                    sin.read_line(&mut s)
                        .expect("Did not enter a correct string");
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        break;
                    }
                    ips.push(trimmed.to_string());
                }
            }
            "4" => {
                // exit program
                return Ok(());
            }
            e => {
                println!("Invalid option: {}", e);
                continue;
            }
        }
        // clear empty entries
        ips.retain(|s| !s.trim().is_empty());
        // double check we actually got printers just for idiotproofing sake
        if ips.is_empty() {
            println!("Error: 0 printers entered.");
            continue;
        } else {
            // we have a list with ips! yippee!
            break;
        }
    }
    // it would be dumb to save the list if we just got it from a file
    if !retrieved_from_file {
        // match here cause there's no point quitting the program if this fails
        match remember::store_printer_list(&ips) {
            Ok(_) => {
                println!("Printer list saved to your computer! Choose \"Use same printers as last time\" next time to use it again.");
            }
            Err(e) => {
                println!("Error saving printer list: {:?}", e);
            }
        }
    }

    let ipslen = ips.len();

    println!(
        "Checking {} printer{}...",
        ipslen,
        if ipslen == 1 { "" } else { "s" }
    );

    // JS runtime wrapped in magic async stuff
    let async_runtime = Rc::new(Mutex::new(JsEngine::new()?));

    // results for spreadsheet mode
    let mut results = vec![String::new(); ipslen];

    // weird fuckery with tokio to get it to do what i want
    let mut joinset = tokio::task::JoinSet::new();
    let localset = tokio::task::LocalSet::new();
    localset
        .run_until(async {
            // spawn all the processes
            for (i, ip) in ips.into_iter().enumerate() {
                let async_runtime_clone = async_runtime.clone();
                if let OutputMode::Spreadsheet = outmode {
                    joinset.spawn_local(async move {
                        (
                            i,
                            check_one_printer::spreadsheet_check_printer(
                                ip.to_owned(),
                                async_runtime_clone,
                            )
                            .await,
                        )
                    });
                } else {
                    joinset.spawn_local(async move {
                        (
                            i,
                            check_one_printer::format_check_printer(
                                ip.to_owned(),
                                async_runtime_clone,
                                outmode == OutputMode::ListAll,
                            )
                            .await,
                        )
                    });
                }
            }
            let mut errors = 0;
            // progress bar for fun
            let bar = indicatif::ProgressBar::new(ipslen as u64);
            bar.inc(0); // get it to display

            // get each result as it comes in
            while let Some(result) = joinset.join_next().await {
                bar.inc(1);
                match result {
                    Ok((index, (msg_option, errored))) => {
                        if errored {
                            errors += 1;
                        }
                        if let OutputMode::Spreadsheet = outmode {
                            results[index] = msg_option.unwrap_or_default();
                        } else if let Some(printmsg) = msg_option {
                            bar.println(printmsg);
                        }
                    }
                    Err(e) => {
                        bar.println("JoinError occured.");
                        bar.println(e.to_string());
                    }
                };
            }
            bar.finish_and_clear();
            println!(
                "\nFinished checking {ipslen} printer{}. Found {errors} with errors.",
                if ipslen == 1 { "" } else { "s" }
            );

            if let OutputMode::Spreadsheet = outmode {
                let fullout = results.join("\n");
                println!("\n{}\n", fullout);
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(fullout)?;
                println!("Results copied to clipboard.\n");
            }
            anyhow::Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // this program is designed to be run by double-clicking and the OS is responsible for bringing
    // up a terminal. those usually close immediately on exit, so wrapping this is for the best.
    // the only errors that happen are clipboard errors but it gives me room in the future i suppose
    if let Err(e) = core().await {
        eprintln!("Unrecoverable error occurred: {:?}", e);
        eprintln!("Please report this on the GitHub: https://github.com/machineonamission/kyocera-printer-monitor/issues/new")
    }

    println!("Press enter to exit program.");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
}
