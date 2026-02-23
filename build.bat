@echo off
setlocal

echo ============================================
echo   MetaLens Build Script
echo ============================================
echo.

:: Find Visual Studio
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" (
    echo [ERROR] Visual Studio not found. Install Visual Studio Build Tools.
    echo Download: https://visualstudio.microsoft.com/downloads/
    pause
    exit /b 1
)

for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
    set "VS_PATH=%%i"
)

if not defined VS_PATH (
    echo [ERROR] Visual Studio C++ tools not found.
    pause
    exit /b 1
)

echo [INFO] Found Visual Studio at: %VS_PATH%
echo.

:: Setup environment
call "%VS_PATH%\VC\Auxiliary\Build\vcvarsall.bat" x64 >nul 2>&1

:: Create build directory
if not exist "build" mkdir build
cd build

:: Configure with CMake
echo [BUILD] Configuring with CMake...
cmake .. -G "NMake Makefiles" -DCMAKE_BUILD_TYPE=Release
if errorlevel 1 (
    echo.
    echo [ERROR] CMake configuration failed.
    echo.
    echo Alternative: Try building with the simple compile script instead:
    echo   compile.bat
    pause
    exit /b 1
)

:: Build
echo.
echo [BUILD] Compiling MetaLens...
nmake
if errorlevel 1 (
    echo [ERROR] Build failed.
    pause
    exit /b 1
)

echo.
echo ============================================
echo   BUILD SUCCESSFUL
echo ============================================
echo   Output: build\MetaLens.exe
echo.
echo   Next steps:
echo   1. Download ExifTool: https://exiftool.org
echo   2. Place exiftool.exe next to MetaLens.exe
echo   3. Run MetaLens.exe
echo ============================================
echo.

cd ..
pause
