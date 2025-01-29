use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use directories::ProjectDirs;

static FILENAME: &str = "printer_list.csv";

fn list_file() -> Result<PathBuf> {
    let binding = match ProjectDirs::from("net", "reticivis", "kyocera-printer-monitor") {
        Some(dirs) => dirs,
        None => return Err(anyhow::anyhow!("Could not find data directory")),
    };
    let dir = binding.data_dir();
    fs::create_dir_all(dir)?;
    Ok(dir.join(FILENAME))
}

pub fn store_printer_list(printers: &[String]) -> Result<()> {
    let printer_list = printers.join("\n");

    fs::write(list_file()?, printer_list)?;
    Ok(())
}

pub fn read_printer_list() -> Result<Vec<String>> {
    let printer_list = fs::read_to_string(list_file()?)?;
    Ok(printer_list.lines().map(|s| s.to_string()).collect())
}

pub fn printer_list_exists() -> Result<bool> {
    Ok(list_file()?.exists())
}
