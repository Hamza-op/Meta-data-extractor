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

:: Check if payload.zip is available for embedding
set "EMBED_FLAG="
if exist "src\payload.zip" (
    echo [INFO] Found payload.zip - will embed into MetaLens.exe
    set "EMBED_FLAG=/dEMBED_EXIFTOOL"
) else if exist "payload.zip" (
    echo [INFO] Found payload.zip - copying to src\ for embedding
    copy /y payload.zip src\payload.zip >nul
    set "EMBED_FLAG=/dEMBED_EXIFTOOL"
) else (
    echo [INFO] payload.zip not found - building WITHOUT embedded ExifTool
    echo [INFO] To embed: put the ExifTool exe (+ files) into payload.zip in the src folder
    echo.
)

echo.
echo [BUILD] Compiling resource file...
rc /nologo %EMBED_FLAG% /fo bin\resource.res src\resource.rc
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

if defined EMBED_FLAG (
    echo   Mode:   STANDALONE (ExifTool embedded)
    echo           Just run MetaLens.exe - nothing else needed!
) else (
    echo   Mode:   EXTERNAL (ExifTool required separately)
    echo   Setup:  Place exiftool.exe next to MetaLens.exe
)

echo ============================================
echo.
pause
