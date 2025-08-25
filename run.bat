cargo fmt
if errorlevel 1 exit /b 1
@REM cargo clippy -- -D warnings
if errorlevel 1 exit /b 1
cargo test
