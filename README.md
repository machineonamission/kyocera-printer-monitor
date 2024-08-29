# 🖨️ Kyocera Printer Monitor

_a highly specialized Rust program designed to make monitoring a fleet of Kyocera printers dead easy_

## ❓ What is this?

Kyocera Printer Monitor is a simple downloadable program that can rapidly scan a large amount of Kyocera printers for
errors, such as missing paper or toner. All you need is their IPs.

## 🚀 How to use

1. Download the latest release from the [releases page](./releases/latest). Make sure you're downloading the correct
   version for your operating system (Windows, Mac, or Linux) and CPU (x86 or ARM).
2. Run it by double-clicking the executable, or by calling it in your terminal.
3. Choose how the output should be formatted.
    1. **Output in spreadsheet mode**

   This will output the status of each printer on a separate line in the order they're inputted, designed to be pasted
   into a spreadsheet easily. The output will automatically be copied to the clipboard.
    2. **List only the errors**

   This will output only the printers that have errors, and the errors they have.
    3. **List statuses of all printers**

   This will output the status of all printers, regardless of whether they have errors or not.
4. Choose how the IPs of the printers should be inputted.
    1. **Use same printers as last time**

   This will use the same printers as the last time you ran the program.
    2. **Paste from clipboard**

   This will use the contents of your clipboard as the IPs of the printers, separated by newlines. Use this option if
   you're copying a column of a spreadsheet of IPs.
    3. **Enter manually**

   This will prompt you to enter the IPs of the printers manually, separated by newlines.
5. The program will then scan the printers and output the results in the format you chose.

