# setup
# install cargo-toolchain
#  https://github.com/cross-rs/cross-toolchains?tab=readme-ov-file#cross-toolchains
# build macos images
#  cargo build-docker-image x86_64-apple-darwin-cross --tag local --build-arg 'MACOS_SDK_URL=https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz'
#  cargo build-docker-image aarch64-apple-darwin-cross --tag local --build-arg 'MACOS_SDK_URL=https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz'

#Remove-Item -LiteralPath ".\target\cross" -Force -Recurse
$path = ".\target\cross"
If(!(test-path -PathType container $path))
{
      New-Item -ItemType Directory -Path $path
}

#$env:CROSS_CONTAINER_OPTS="--network host"


# only one can run at a time anyways cause locks on the build dir but might as well try
# i do know that docker image setup can run in parallel
$commands = @(
    'cross build --target x86_64-unknown-linux-gnu --release; if ($?) {Copy-Item ".\target\x86_64-unknown-linux-gnu\release\kyocera-printer-monitor" -Destination ".\target\cross\kyocera-printer-monitor-x86_64-unknown-linux-gnu"}',
    'cross build --target aarch64-unknown-linux-gnu --release; if ($?) {Copy-Item ".\target\aarch64-unknown-linux-gnu\release\kyocera-printer-monitor" -Destination ".\target\cross\kyocera-printer-monitor-aarch64-unknown-linux-gnu"}',
    'cross build --target x86_64-apple-darwin --release; if ($?) {Copy-Item ".\target\x86_64-apple-darwin\release\kyocera-printer-monitor" -Destination ".\target\cross\kyocera-printer-monitor-x86_64-apple-darwin"}',
    'cross build --target aarch64-apple-darwin --release; if ($?) {Copy-Item ".\target\aarch64-apple-darwin\release\kyocera-printer-monitor" -Destination ".\target\cross\kyocera-printer-monitor-aarch64-apple-darwin"}',
# no'windows for arm because v8 isnt pre-built for it so fuck th't
#   'cross build --target aarch64-pc-windows-msvc --release; # if ($?) {Copy-Item ".\target\aarch64-pc-windows-msvc\release\kyocera-printer-monitor" -Destination ".\target\cross\kyocera-printer-monitor-aarch64-pc-windows-msvc"}',
    'cargo build --release; if ($?) {Copy-Item ".\target\release\kyocera-printer-monitor.exe" -Destination ".\target\cross\kyocera-printer-monitor-x86_64-pc-windows-msvc.exe"}'
)

# i think if you remove -Parallel it makes it go one at a time which might be nicer on your system
$commands | ForEach-Object -Parallel {
    Invoke-Expression $_
}
