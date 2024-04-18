use std::io::stdin;
use std::sync::Arc;

use tokio::sync::Mutex;

mod check_one_printer;
mod http;
mod js;
mod r#static;

#[tokio::main]
async fn main() {
    // for debugging
    // let async_runtime = Arc::new(Mutex::new(js::init()));
    // dbg!(check_one_printer::check_printer(String::from("165.134.134.90"), async_runtime).await.unwrap());

    println!("🖨️ Welcome to Kyocera Printer Monitor!\n");

    // get user preferences
    let list_all: bool;
    let sin = stdin();
    loop {
        println!(
            "Choose an option:\n\
        1) Check only for errors\n\
        2) List all statuses of printer\n\
        3) Exit"
        );
        let mut s = String::new();
        sin.read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim() {
            "1" => {
                list_all = false;
                break;
            }
            "2" => {
                list_all = true;
                break;
            }
            "3" => {
                return;
            }
            e => {
                println!("Invalid option: {}", e);
            }
        }
    }
    // get IPs
    println!(
        "Enter in the IPs of the printers, separated by newlines. Press enter twice when you are done."
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

    println!(
        "Checking {} printer{}...",
        ips.len(),
        if ips.len() == 1 { "" } else { "s" }
    );

    // JS runtime wrapped in magic async stuff
    let async_runtime = Arc::new(Mutex::new(js::init()));

    // weird fuckery with tokio to get it to do what i want
    let mut joinset = tokio::task::JoinSet::new();
    let localset = tokio::task::LocalSet::new();
    localset
        .run_until(async {
            // spawn all the processes
            let len = ips.len();
            for ip in ips {
                let async_runtime_clone = async_runtime.clone();
                joinset.spawn_local(async move {
                    check_one_printer::format_check_printer(
                        ip.to_owned(),
                        async_runtime_clone,
                        list_all,
                    )
                    .await
                });
            }
            let mut errors = 0;
            // progress bar for fun
            let bar = indicatif::ProgressBar::new(len as u64);
            bar.inc(0); // get it to display

            // get each result as it comes in
            while let Some(result) = joinset.join_next().await {
                bar.inc(1);
                match result {
                    Ok((msg_option, errored)) => {
                        if errored {
                            errors += 1;
                        }
                        if let Some(printmsg) = msg_option {
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
                "\nFinished checking {len} printer{}. Found {errors} with errors.",
                if len == 1 { "" } else { "s" }
            );
        })
        .await;
    println!("Press enter to exit program.");
    let mut s = String::new();
    sin.read_line(&mut s)
        .expect("Did not enter a correct string");
}
