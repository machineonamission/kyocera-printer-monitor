# 🖨️ Kyocera Printer Monitor

_a highly specialized Rust program designed to make monitoring a fleet of Kyocera printers dead easy_

## ❓ What is this?

Kyocera Printer Monitor is a simple downloadable program that can rapidly scan a large amount of Kyocera printers for
errors, such as missing paper or toner. All you need is their IPs.

## 📦 Download

Either download the file
from [the releases tab](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest), or get it from the
table below. The rows are different operating systems, and the columns are different CPU architectures. If you aren't
sure of your architecture, get x86.

|             |                                                                                   **x86** (Intel or AMD)                                                                                    |                                                                                **ARM** (or Apple Silicon)                                                                                |
|:-----------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
| **Windows** | [kyocera-printer-monitor-x86\_64-pc-windows-msvc.exe](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-x86_64-pc-windows-msvc.exe) |  [kyocera-printer-monitor-aarch64-windows-msvc.exe](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-aarch64-windows-msvc.exe)  |
|  **MacOS**  |        [kyocera-printer-monitor-x86\_64-apple-darwin](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-x86_64-apple-darwin)        |      [kyocera-printer-monitor-aarch64-apple-darwin](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-aarch64-apple-darwin)      |
|  **Linux**  |   [kyocera-printer-monitor-x86\_64-unknown-linux-gnu](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-x86_64-unknown-linux-gnu)   | [kyocera-printer-monitor-aarch64-unknown-linux-gnu](https://github.com/reticivis-net/kyocera-printer-monitor/releases/latest/download/kyocera-printer-monitor-aarch64-unknown-linux-gnu) |

## 🚀 How to use

1. Run the downloaded file by double-clicking the executable, or by calling it in your terminal.
2. Choose how the output should be formatted.
    1. **Output in spreadsheet mode**

   This will output the status of each printer on a separate line in the order they're inputted, designed to be pasted
   into a spreadsheet easily. The output will automatically be copied to the clipboard.
    2. **List only the errors**

   This will output only the printers that have errors, and the errors they have.
    3. **List statuses of all printers**

   This will output the status of all printers, regardless of whether they have errors or not.
3. Choose how the IPs of the printers should be inputted.
    1. **Use same printers as last time**

   This will use the same printers as the last time you ran the program.
    2. **Paste from clipboard**

   This will use the contents of your clipboard as the IPs of the printers, separated by newlines. Use this option if
   you're copying a column of a spreadsheet of IPs.
    3. **Enter manually**

   This will prompt you to enter the IPs of the printers manually, separated by newlines.
4. The program will then scan the printers and output the results in the format you chose.

