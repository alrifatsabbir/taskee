@echo off
echo Starting Taskee Official Web Server (100% Pure Rust)...
cd /d "%~dp0"
if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    call "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
)
cargo run -p taskee-website
pause
