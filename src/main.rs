use std::io::stdin;
use std::rc::Rc;

use arboard::Clipboard;
use tokio::sync::Mutex;

mod check_one_printer;
mod http;
mod js;
mod r#static;

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Spreadsheet,
    ListErrors,
    ListAll,
}

#[tokio::main]
async fn main() {
    // for debugging
    // let async_runtime = Rc::new(Mutex::new(js::init()));
    // dbg!(check_one_printer::check_printer(String::from("165.134.48.176"), async_runtime).await.unwrap());
    // dbg!(http::get_right_host("165.134.48.176").await.unwrap());
    // dbg!(http::get_right_host("10.170.16.1").await.unwrap());
    // return;

    println!("🖨️ Welcome to Kyocera Printer Monitor!\n");

    // get user preferences
    let mode: Mode;
    let sin = stdin();
    loop {
        println!(
            "Choose an option:\n\
        1) Output in spreadsheet mode\n\
        2) Print only the errors\n\
        3) List all statuses of printers\n\
        4) Exit"
        );
        let mut s = String::new();
        sin.read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim() {
            "1" => {
                mode = Mode::Spreadsheet;
                break;
            }
            "2" => {
                mode = Mode::ListErrors;
                break;
            }
            "3" => {
                mode = Mode::ListAll;
                break;
            }
            "4" => {
                return;
            }
            e => {
                println!("Invalid option: {}", e);
            }
        }
    }
    // get IPs
    println!(
        "Enter in the IPs of the printers, separated by newlines. Press enter twice when you are done.\nOR\nPress enter to paste from the clipboard."
    );
    let mut ips = Vec::new();
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

    if ips.is_empty() {
        println!("Pasting from clipboard...");
        // no i don't like unwrapping but who cares it's in main its FINE
        let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
        for line in clipboard
            .get_text()
            .expect("Failed to paste from clipboard")
            .lines()
        {
            ips.push(line.to_string());
        }
    }

    let ipslen = ips.len();

    println!(
        "Checking {} printer{}...",
        ipslen,
        if ipslen == 1 { "" } else { "s" }
    );

    // JS runtime wrapped in magic async stuff
    let async_runtime = Rc::new(Mutex::new(js::init()));

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
                if let Mode::Spreadsheet = mode {
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
                                mode == Mode::ListAll,
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
                        if let Mode::Spreadsheet = mode {
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

            if let Mode::Spreadsheet = mode {
                let fullout = results.join("\n");
                println!("\n{}\n", fullout);
                // no i don't like unwrapping but who cares it's in main its FINE
                let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
                clipboard
                    .set_text(fullout)
                    .expect("Failed to copy to clipboard");
                println!("Results copied to clipboard.\n");
            }
        })
        .await;
    println!("Press enter to exit program.");
    let mut s = String::new();
    sin.read_line(&mut s)
        .expect("Did not enter a correct string");
}
