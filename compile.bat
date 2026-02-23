@echo off
setlocal

echo ============================================
echo   MetaLens - Direct Compile (No CMake)
echo ============================================
echo.

:: Find Visual Studio
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" (
    echo [ERROR] Visual Studio not found.
    echo.
    echo You need one of:
    echo   - Visual Studio 2019/2022 with C++ Desktop workload
    echo   - Visual Studio Build Tools with C++ build tools
    echo.
    echo Download: https://visualstudio.microsoft.com/downloads/
    pause
    exit /b 1
)

for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
    set "VS_PATH=%%i"
)

if not defined VS_PATH (
    echo [ERROR] C++ tools not installed in Visual Studio.
    pause
    exit /b 1
)

echo [INFO] Using Visual Studio at: %VS_PATH%
call "%VS_PATH%\VC\Auxiliary\Build\vcvarsall.bat" x64 >nul 2>&1

:: Create output directory
if not exist "bin" mkdir bin

echo.
echo [BUILD] Compiling resource file...
rc /nologo /fo bin\resource.res src\resource.rc
if errorlevel 1 (
    echo [ERROR] Resource compilation failed.
    pause
    exit /b 1
)

echo [BUILD] Compiling MetaLens...
cl /nologo /std:c++17 /O2 /EHsc /W3 ^
    /DUNICODE /D_UNICODE /D_WIN32_WINNT=0x0A00 ^
    /Isrc ^
    src\main.cpp ^
    bin\resource.res ^
    /Fe:bin\MetaLens.exe ^
    /link /SUBSYSTEM:WINDOWS ^
    comctl32.lib uxtheme.lib dwmapi.lib shlwapi.lib ^
    gdi32.lib user32.lib shell32.lib ole32.lib comdlg32.lib ^
    advapi32.lib
if errorlevel 1 (
    echo.
    echo [ERROR] Compilation failed.
    pause
    exit /b 1
)

:: Clean up intermediate files
del /q *.obj 2>nul

echo.
echo ============================================
echo   BUILD SUCCESSFUL!
echo ============================================
echo   Output: bin\MetaLens.exe
echo.
echo   Setup:
echo   1. Download ExifTool from https://exiftool.org
echo      (Get the Windows Executable zip)
echo   2. Extract and rename to: exiftool.exe
echo   3. Place exiftool.exe in the bin\ folder
echo      (next to MetaLens.exe)
echo   4. Run bin\MetaLens.exe
echo ============================================
echo.
pause
