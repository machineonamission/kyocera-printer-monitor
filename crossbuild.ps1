$commands = @(
    "cross build --target x86_64-unknown-linux-gnu --release",
    "cross build --target aarch64-unknown-linux-gnu --release",
    "cross build --target x86_64-apple-darwin --release",
    "cross build --target aarch64-apple-darwin --release",
#    "cross build --target aarch64-pc-windows-msvc --release",
    "cargo build --release"
)

$commands | ForEach-Object -Parallel {
    Invoke-Expression $_
}

Remove-Item -LiteralPath "./target/cross" -Force -Recurse
New-Item -ItemType directory -Path "./target/cross"

Copy-Item -Path "./target/aarch64-apple-darwin/release/kyocera-printer-monitor" -Destination "./target/cross/kyocera-printer-monitor-aarch64-apple-darwin"
Copy-Item -Path "./target/x86_64-apple-darwin/release/kyocera-printer-monitor" -Destination "./target/cross/kyocera-printer-monitor-x86_64-apple-darwin"
Copy-Item -Path "./target/aarch64-unknown-linux-gnu/release/kyocera-printer-monitor" -Destination "./target/cross/kyocera-printer-monitor-aarch64-unknown-linux-gnu"
Copy-Item -Path "./target/x86_64-unknown-linux-gnu/release/kyocera-printer-monitor" -Destination "./target/cross/kyocera-printer-monitor-x86_64-unknown-linux-gnu"
# Copy-Item -Path "./target/aarch64-pc-windows-msvc/release/kyocera-printer-monitor" -Destination "./target/cross/kyocera-printer-monitor-aarch64-pc-windows-msvc"
Copy-Item -Path "./target/release/kyocera-printer-monitor.exe" -Destination "./target/cross/kyocera-printer-monitor-x86_64-pc-windows-msvc.exe"
