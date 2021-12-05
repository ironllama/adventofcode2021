@ECHO OFF
rustc --out-dir=target %1.rs && target\%1.exe