@ECHO OFF
set DRIVE=%cd:~0,2%
set HOME=%DRIVE%\MyFiles
set HOME_LIB=%HOME%\lib

set PATH=%HOME_LIB%\GitPortable\bin;%PATH%

set RUSTUP_HOME=C:\Projects\lib\rust\rust
set CARGO_HOME=C:\Projects\lib\rust\cargo
set RUST_PATH=%CARGO_HOME%\bin

set PATH=%RUST_PATH%;%PATH%
