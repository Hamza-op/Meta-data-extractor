cmake_minimum_required(VERSION 3.20)
project(MetaLens VERSION 1.0.0 LANGUAGES CXX RC)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Win32 subsystem (no console window)
set(CMAKE_WIN32_EXECUTABLE ON)

add_executable(MetaLens WIN32
    src/main.cpp
    src/resource.rc
)

target_include_directories(MetaLens PRIVATE src)

target_link_libraries(MetaLens PRIVATE
    comctl32
    uxtheme
    dwmapi
    shlwapi
    gdi32
    user32
    shell32
    ole32
    comdlg32
)

# Enable Unicode
target_compile_definitions(MetaLens PRIVATE UNICODE _UNICODE)

# Manifest for modern controls
target_compile_definitions(MetaLens PRIVATE _WIN32_WINNT=0x0A00)

# Optimization for Release
if(CMAKE_BUILD_TYPE STREQUAL "Release")
    target_compile_options(MetaLens PRIVATE /O2 /GL)
    target_link_options(MetaLens PRIVATE /LTCG)
endif()
