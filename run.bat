cargo fmt
if errorlevel 1 exit /b 1
cargo clippy -- -D warnings
if errorlevel 1 exit /b 1
cargo test
