// ============================================================================
// MetaLens - Deep Metadata Analyzer
// A lightweight Win32 GUI wrapper around ExifTool
// ============================================================================

#ifndef UNICODE
#define UNICODE
#endif
#ifndef _UNICODE
#define _UNICODE
#endif

#define _WIN32_WINNT 0x0A00
#define WIN32_LEAN_AND_MEAN

#include <windows.h>
#include <windowsx.h>
#include <commctrl.h>
#include <shellapi.h>
#include <shlwapi.h>
#include <uxtheme.h>
#include <dwmapi.h>
#include <commdlg.h>
#include <shlobj.h>

#include <string>
#include <vector>
#include <map>
#include <algorithm>
#include <sstream>
#include <fstream>
#include <functional>
#include <mutex>

#include "resource.h"
#include "camera_db.h"

#pragma comment(lib, "comctl32.lib")
#pragma comment(lib, "uxtheme.lib")
#pragma comment(lib, "dwmapi.lib")
#pragma comment(lib, "shlwapi.lib")
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "ole32.lib")
#pragma comment(lib, "comdlg32.lib")

// ============================================================================
// Color Theme (Dark, Premium)
// ============================================================================
namespace Theme {
    constexpr COLORREF BgDark       = RGB(18, 18, 24);
    constexpr COLORREF BgPanel      = RGB(26, 26, 36);
    constexpr COLORREF BgHeader     = RGB(32, 32, 44);
    constexpr COLORREF BgInput      = RGB(36, 36, 50);
    constexpr COLORREF BgHover      = RGB(42, 42, 58);
    constexpr COLORREF BgSelected   = RGB(55, 48, 107);
    constexpr COLORREF BgGroupHdr   = RGB(28, 28, 42);

    constexpr COLORREF TextPrimary  = RGB(230, 230, 240);
    constexpr COLORREF TextSecondary= RGB(160, 160, 180);
    constexpr COLORREF TextMuted    = RGB(100, 100, 120);
    constexpr COLORREF TextAccent   = RGB(130, 120, 255);
    constexpr COLORREF TextGreen    = RGB(80, 220, 140);
    constexpr COLORREF TextOrange   = RGB(255, 180, 80);
    constexpr COLORREF TextCyan     = RGB(80, 200, 240);

    constexpr COLORREF AccentPurple = RGB(110, 90, 255);
    constexpr COLORREF AccentBlue   = RGB(60, 130, 255);
    constexpr COLORREF Border       = RGB(50, 50, 70);
    constexpr COLORREF BorderFocus  = RGB(110, 90, 255);

    constexpr COLORREF BtnBg        = RGB(110, 90, 255);
    constexpr COLORREF BtnHover     = RGB(130, 110, 255);
    constexpr COLORREF BtnText      = RGB(255, 255, 255);
    constexpr COLORREF BtnSecBg     = RGB(42, 42, 58);
    constexpr COLORREF BtnSecHover  = RGB(55, 55, 75);

    constexpr COLORREF DropzoneBg   = RGB(22, 22, 32);
    constexpr COLORREF DropzoneBorder= RGB(70, 60, 140);
}

// ============================================================================
// Data Structures
// ============================================================================
struct MetadataEntry {
    std::wstring group;
    std::wstring tag;
    std::wstring value;
    std::wstring resolvedName; // For camera model resolution
};

struct AppState {
    HWND hWnd           = nullptr;
    HWND hListView      = nullptr;
    HWND hSearchEdit    = nullptr;
    HWND hTabControl    = nullptr;
    HWND hStatusBar     = nullptr;
    HWND hDropLabel     = nullptr;
    HWND hFilePathLabel = nullptr;
    HWND hOpenBtn       = nullptr;
    HWND hExportBtn     = nullptr;
    HWND hClearBtn      = nullptr;
    HWND hCopyBtn       = nullptr;

    HFONT hFontUI       = nullptr;
    HFONT hFontMono     = nullptr;
    HFONT hFontTitle    = nullptr;
    HFONT hFontSmall    = nullptr;
    HFONT hFontIcon     = nullptr;

    HBRUSH hBrDark      = nullptr;
    HBRUSH hBrPanel     = nullptr;
    HBRUSH hBrInput     = nullptr;
    HBRUSH hBrHeader    = nullptr;

    std::wstring currentFile;
    std::wstring exifToolPath;

    std::vector<MetadataEntry> allEntries;
    std::vector<MetadataEntry> filteredEntries;
    std::vector<std::wstring>  groups;
    int activeGroupIndex = 0; // 0 = "All"

    std::wstring searchQuery;
    bool isLoading       = false;
    bool fileLoaded      = false;

    // Drag hover state
    bool isDragHover     = false;
};

static AppState g_app;

// ============================================================================
// Forward Declarations
// ============================================================================
LRESULT CALLBACK WndProc(HWND, UINT, WPARAM, LPARAM);
void CreateUIControls(HWND hWnd);
void LayoutControls(HWND hWnd);
void LoadFileMetadata(const std::wstring& filePath);
void PopulateListView();
void FilterEntries();
void UpdateStatusBar();
void PaintDropZone(HWND hWnd, HDC hdc);
std::wstring FindExifTool();
std::wstring RunExifTool(const std::wstring& filePath);
void ParseExifToolOutput(const std::wstring& output);
void ExportToFile();
void CopyToClipboard();
std::wstring OpenFileDialog(HWND hWnd);

// ============================================================================
// Utility: Wide string helpers
// ============================================================================
std::wstring Utf8ToWide(const std::string& utf8) {
    if (utf8.empty()) return {};
    int len = MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), (int)utf8.size(), nullptr, 0);
    std::wstring wide(len, 0);
    MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), (int)utf8.size(), &wide[0], len);
    return wide;
}

std::string WideToUtf8(const std::wstring& wide) {
    if (wide.empty()) return {};
    int len = WideCharToMultiByte(CP_UTF8, 0, wide.c_str(), (int)wide.size(), nullptr, 0, nullptr, nullptr);
    std::string utf8(len, 0);
    WideCharToMultiByte(CP_UTF8, 0, wide.c_str(), (int)wide.size(), &utf8[0], len, nullptr, nullptr);
    return utf8;
}

std::wstring ToLower(const std::wstring& s) {
    std::wstring out = s;
    std::transform(out.begin(), out.end(), out.begin(), ::towlower);
    return out;
}

// ============================================================================
// Utility: Draw rounded rectangle
// ============================================================================
void DrawRoundRect(HDC hdc, const RECT& rc, int radius, COLORREF fill, COLORREF border) {
    HBRUSH hBr = CreateSolidBrush(fill);
    HPEN hPen = CreatePen(PS_SOLID, 1, border);
    HBRUSH oldBr = (HBRUSH)SelectObject(hdc, hBr);
    HPEN oldPen = (HPEN)SelectObject(hdc, hPen);
    RoundRect(hdc, rc.left, rc.top, rc.right, rc.bottom, radius, radius);
    SelectObject(hdc, oldBr);
    SelectObject(hdc, oldPen);
    DeleteObject(hBr);
    DeleteObject(hPen);
}

// ============================================================================
// Custom drawn button structure
// ============================================================================
struct BtnInfo {
    HWND hwnd;
    const wchar_t* text;
    const wchar_t* icon;
    bool isPrimary;
    bool isHovered;
};

static std::map<HWND, BtnInfo> g_buttons;

// ============================================================================
// Entry Point
// ============================================================================
int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE, LPWSTR lpCmdLine, int nCmdShow) {
    // Init COM
    OleInitialize(nullptr);

    // Init Common Controls
    INITCOMMONCONTROLSEX icc = { sizeof(icc), ICC_LISTVIEW_CLASSES | ICC_TAB_CLASSES | ICC_BAR_CLASSES };
    InitCommonControlsEx(&icc);

    // Find ExifTool
    g_app.exifToolPath = FindExifTool();

    // Register Window Class
    const wchar_t CLASS_NAME[] = L"MetaLensMainWindow";
    WNDCLASSEXW wc = {};
    wc.cbSize        = sizeof(wc);
    wc.style         = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc   = WndProc;
    wc.hInstance      = hInstance;
    wc.hCursor       = LoadCursor(nullptr, IDC_ARROW);
    wc.hbrBackground = CreateSolidBrush(Theme::BgDark);
    wc.lpszClassName = CLASS_NAME;
    wc.hIcon         = LoadIcon(nullptr, IDI_APPLICATION);
    wc.hIconSm       = LoadIcon(nullptr, IDI_APPLICATION);
    RegisterClassExW(&wc);

    // Create window
    HWND hWnd = CreateWindowExW(
        WS_EX_ACCEPTFILES,
        CLASS_NAME,
        L"MetaLens — Deep Metadata Analyzer",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, 1200, 800,
        nullptr, nullptr, hInstance, nullptr
    );

    if (!hWnd) return 1;

    // Enable dark title bar (Windows 10+)
    BOOL useDarkMode = TRUE;
    DwmSetWindowAttribute(hWnd, 20 /* DWMWA_USE_IMMERSIVE_DARK_MODE */, &useDarkMode, sizeof(useDarkMode));

    // Enable backdrop (Windows 11)
    enum { DWMSBT_MAINWINDOW = 2 };
    int backdropType = DWMSBT_MAINWINDOW;
    DwmSetWindowAttribute(hWnd, 38 /* DWMWA_SYSTEMBACKDROP_TYPE */, &backdropType, sizeof(backdropType));

    ShowWindow(hWnd, nCmdShow);
    UpdateWindow(hWnd);

    // Handle command-line file argument
    if (lpCmdLine && wcslen(lpCmdLine) > 0) {
        std::wstring cmdFile = lpCmdLine;
        // Remove quotes if present
        if (cmdFile.front() == L'"' && cmdFile.back() == L'"') {
            cmdFile = cmdFile.substr(1, cmdFile.length() - 2);
        }
        if (PathFileExistsW(cmdFile.c_str())) {
            PostMessage(hWnd, WM_USER + 100, 0, (LPARAM)_wcsdup(cmdFile.c_str()));
        }
    }

    // Message loop
    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    OleUninitialize();
    return (int)msg.wParam;
}

// ============================================================================
// ExifTool Discovery
// ============================================================================
std::wstring FindExifTool() {
    // 1. Check alongside our exe
    wchar_t exePath[MAX_PATH];
    GetModuleFileNameW(nullptr, exePath, MAX_PATH);
    PathRemoveFileSpecW(exePath);

    std::wstring candidates[] = {
        std::wstring(exePath) + L"\\exiftool.exe",
        std::wstring(exePath) + L"\\exiftool(-k).exe",
        std::wstring(exePath) + L"\\tools\\exiftool.exe",
    };

    for (auto& c : candidates) {
        if (PathFileExistsW(c.c_str())) return c;
    }

    // 2. Check PATH
    wchar_t found[MAX_PATH];
    if (SearchPathW(nullptr, L"exiftool.exe", nullptr, MAX_PATH, found, nullptr)) {
        return found;
    }
    if (SearchPathW(nullptr, L"exiftool", nullptr, MAX_PATH, found, nullptr)) {
        return found;
    }

    // 3. Common install locations
    std::wstring commonPaths[] = {
        L"C:\\exiftool\\exiftool.exe",
        L"C:\\Windows\\exiftool.exe",
        L"C:\\Program Files\\ExifTool\\exiftool.exe",
        L"C:\\Program Files (x86)\\ExifTool\\exiftool.exe",
    };
    for (auto& p : commonPaths) {
        if (PathFileExistsW(p.c_str())) return p;
    }

    return L""; // Not found
}

// ============================================================================
// Run ExifTool as subprocess and capture output
// ============================================================================
std::wstring RunExifTool(const std::wstring& filePath) {
    if (g_app.exifToolPath.empty()) {
        return L"ERROR: ExifTool not found. Place exiftool.exe next to MetaLens.exe or add it to PATH.";
    }

    // Build command: exiftool -All -G:1 -a -u -s -f "<file>"
    // -All   = all tags
    // -G:1   = print group name (family 1: specific group)
    // -a     = allow duplicate tags
    // -u     = extract unknown tags
    // -s     = short output (tag names, not descriptions) - removed for readability
    // -f     = force print all specified tags (even if empty)
    // -n removed to keep human-readable values
    std::wstring cmdLine = L"\"" + g_app.exifToolPath + L"\" -All -G:1 -a -u -f -E \"" + filePath + L"\"";

    // Create pipe for stdout
    SECURITY_ATTRIBUTES sa = { sizeof(sa), nullptr, TRUE };
    HANDLE hReadPipe, hWritePipe;
    if (!CreatePipe(&hReadPipe, &hWritePipe, &sa, 0))
        return L"ERROR: Failed to create pipe.";

    SetHandleInformation(hReadPipe, HANDLE_FLAG_INHERIT, 0);

    STARTUPINFOW si = { sizeof(si) };
    si.dwFlags = STARTF_USESTDHANDLES | STARTF_USESHOWWINDOW;
    si.hStdOutput = hWritePipe;
    si.hStdError = hWritePipe;
    si.wShowWindow = SW_HIDE;

    PROCESS_INFORMATION pi = {};
    BOOL ok = CreateProcessW(
        nullptr, (LPWSTR)cmdLine.c_str(),
        nullptr, nullptr, TRUE,
        CREATE_NO_WINDOW, nullptr, nullptr,
        &si, &pi
    );

    CloseHandle(hWritePipe);

    if (!ok) {
        CloseHandle(hReadPipe);
        return L"ERROR: Failed to launch ExifTool. Command: " + cmdLine;
    }

    // Read all output
    std::string output;
    char buffer[8192];
    DWORD bytesRead;
    while (ReadFile(hReadPipe, buffer, sizeof(buffer) - 1, &bytesRead, nullptr) && bytesRead > 0) {
        buffer[bytesRead] = 0;
        output.append(buffer, bytesRead);
    }

    WaitForSingleObject(pi.hProcess, 10000);
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    CloseHandle(hReadPipe);

    return Utf8ToWide(output);
}

// ============================================================================
// Parse ExifTool Grouped Output
// ============================================================================
void ParseExifToolOutput(const std::wstring& output) {
    g_app.allEntries.clear();
    g_app.groups.clear();

    std::wistringstream stream(output);
    std::wstring line;
    std::map<std::wstring, int> groupSet;

    std::wstring foundModel;
    std::wstring foundMake;
    std::wstring foundLensModel;
    std::wstring foundLensId;
    std::wstring foundLensType;
    std::wstring foundLensInfo;

    while (std::getline(stream, line)) {
        if (line.empty()) continue;

        // Format: [GroupName]     Tag Name                 : Value
        // ExifTool with -G:1 outputs: [Group]  Tag : Value

        MetadataEntry entry;

        // Find group in brackets
        size_t bracketOpen = line.find(L'[');
        size_t bracketClose = line.find(L']');
        if (bracketOpen != std::wstring::npos && bracketClose != std::wstring::npos && bracketClose > bracketOpen) {
            entry.group = line.substr(bracketOpen + 1, bracketClose - bracketOpen - 1);
            line = line.substr(bracketClose + 1);
        } else {
            entry.group = L"Other";
        }

        // Find tag : value split
        size_t colonPos = line.find(L':');
        if (colonPos == std::wstring::npos) continue;

        entry.tag = line.substr(0, colonPos);
        entry.value = line.substr(colonPos + 1);

        // Trim whitespace
        auto trim = [](std::wstring& s) {
            size_t start = s.find_first_not_of(L" \t\r\n");
            size_t end = s.find_last_not_of(L" \t\r\n");
            if (start == std::wstring::npos) { s.clear(); return; }
            s = s.substr(start, end - start + 1);
        };
        trim(entry.group);
        trim(entry.tag);
        trim(entry.value);

        if (entry.tag.empty()) continue;

        // Collect Make/Model/Lens for resolution
        std::wstring tagLower = ToLower(entry.tag);
        if (tagLower == L"camera model name" || tagLower == L"model") {
            if (foundModel.empty()) foundModel = entry.value;
        }
        if (tagLower == L"make") {
            if (foundMake.empty()) foundMake = entry.value;
        }
        // Lens detection — check multiple tag variants
        if (tagLower == L"lens model" || tagLower == L"lens") {
            if (foundLensModel.empty()) foundLensModel = entry.value;
        }
        if (tagLower == L"lens id" || tagLower == L"lensid") {
            if (foundLensId.empty()) foundLensId = entry.value;
        }
        if (tagLower == L"lens type" || tagLower == L"lenstype") {
            if (foundLensType.empty()) foundLensType = entry.value;
        }
        if (tagLower == L"lens info" || tagLower == L"lensinfo") {
            if (foundLensInfo.empty()) foundLensInfo = entry.value;
        }

        // Track groups
        if (groupSet.find(entry.group) == groupSet.end()) {
            groupSet[entry.group] = (int)g_app.groups.size();
            g_app.groups.push_back(entry.group);
        }

        g_app.allEntries.push_back(entry);
    }

    // Ensure "Camera Info" group exists for synthetic entries
    auto ensureCameraInfoGroup = [&]() {
        if (groupSet.find(L"Camera Info") == groupSet.end()) {
            groupSet[L"Camera Info"] = (int)g_app.groups.size();
            g_app.groups.insert(g_app.groups.begin(), L"Camera Info");
        }
    };

    // Resolve camera model name and inject as synthetic entry
    if (!foundModel.empty()) {
        std::wstring resolved = ResolveCameraModel(foundModel);
        if (resolved != foundModel) {
            MetadataEntry synth;
            synth.group = L"Camera Info";
            synth.tag = L"\xD83D\xDCF7 Identified Camera";  // 📷
            synth.value = foundModel + L"  \x2192  " + resolved;  // →

            g_app.allEntries.insert(g_app.allEntries.begin(), synth);
            ensureCameraInfoGroup();
        }
    }

    // Resolve lens model — try each lens source in priority order
    // Priority: Lens Model > Lens ID > Lens Type > Lens Info
    std::wstring bestLensStr;
    if (!foundLensModel.empty() && foundLensModel != L"-" && foundLensModel != L"Unknown") {
        bestLensStr = foundLensModel;
    } else if (!foundLensId.empty() && foundLensId != L"-" && foundLensId != L"Unknown") {
        bestLensStr = foundLensId;
    } else if (!foundLensType.empty() && foundLensType != L"-" && foundLensType != L"Unknown") {
        bestLensStr = foundLensType;
    } else if (!foundLensInfo.empty() && foundLensInfo != L"-" && foundLensInfo != L"Unknown") {
        bestLensStr = foundLensInfo;
    }

    if (!bestLensStr.empty()) {
        std::wstring resolvedLens = ResolveLensModel(bestLensStr);

        MetadataEntry lensEntry;
        lensEntry.group = L"Camera Info";
        if (resolvedLens != bestLensStr) {
            lensEntry.tag = L"\xD83D\xDD2D Identified Lens";  // 🔭
            lensEntry.value = bestLensStr + L"  \x2192  " + resolvedLens;  // →
        } else {
            // Even without DB match, still surface the lens in Camera Info
            lensEntry.tag = L"\xD83D\xDD2D Lens";  // 🔭
            lensEntry.value = bestLensStr;
        }

        // Insert after camera identification (position 0 or 1)
        int insertPos = 0;
        if (!g_app.allEntries.empty() &&
            g_app.allEntries[0].tag.find(L"Identified Camera") != std::wstring::npos) {
            insertPos = 1;
        }
        g_app.allEntries.insert(g_app.allEntries.begin() + insertPos, lensEntry);
        ensureCameraInfoGroup();
    }

    // Sort groups nicely
    // Priority order: Camera Info, ExifIFD, IFD0, EXIF, XMP, IPTC, MakerNotes, ...
    auto groupPriority = [](const std::wstring& g) -> int {
        std::wstring gl = ToLower(g);
        if (gl == L"camera info")    return 0;
        if (gl.find(L"exififd") != std::wstring::npos || gl.find(L"exif") != std::wstring::npos) return 1;
        if (gl.find(L"ifd0") != std::wstring::npos) return 2;
        if (gl.find(L"makernotes") != std::wstring::npos) return 3;
        if (gl.find(L"xmp") != std::wstring::npos) return 4;
        if (gl.find(L"iptc") != std::wstring::npos) return 5;
        if (gl.find(L"icc") != std::wstring::npos) return 6;
        if (gl.find(L"composite") != std::wstring::npos) return 7;
        if (gl.find(L"file") != std::wstring::npos) return 8;
        if (gl.find(L"quicktime") != std::wstring::npos || gl.find(L"track") != std::wstring::npos) return 3;
        return 10;
    };
    std::sort(g_app.groups.begin(), g_app.groups.end(),
        [&](const std::wstring& a, const std::wstring& b) {
            return groupPriority(a) < groupPriority(b);
        });
}

// ============================================================================
// Create Fonts & Brushes
// ============================================================================
void CreateResources() {
    // UI Font - Segoe UI
    g_app.hFontUI = CreateFontW(
        -15, 0, 0, 0, FW_REGULAR, FALSE, FALSE, FALSE,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Segoe UI"
    );

    // Mono font for values
    g_app.hFontMono = CreateFontW(
        -14, 0, 0, 0, FW_REGULAR, FALSE, FALSE, FALSE,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, FIXED_PITCH | FF_MODERN, L"Cascadia Code"
    );

    // Title font
    g_app.hFontTitle = CreateFontW(
        -22, 0, 0, 0, FW_SEMIBOLD, FALSE, FALSE, FALSE,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Segoe UI"
    );

    // Small font
    g_app.hFontSmall = CreateFontW(
        -12, 0, 0, 0, FW_REGULAR, FALSE, FALSE, FALSE,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Segoe UI"
    );

    // Icon font (Segoe MDL2 Assets or fallback)
    g_app.hFontIcon = CreateFontW(
        -18, 0, 0, 0, FW_REGULAR, FALSE, FALSE, FALSE,
        DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Segoe MDL2 Assets"
    );

    // Brushes
    g_app.hBrDark   = CreateSolidBrush(Theme::BgDark);
    g_app.hBrPanel  = CreateSolidBrush(Theme::BgPanel);
    g_app.hBrInput  = CreateSolidBrush(Theme::BgInput);
    g_app.hBrHeader = CreateSolidBrush(Theme::BgHeader);
}

// ============================================================================
// UI Construction
// ============================================================================
void CreateUIControls(HWND hWnd) {
    CreateResources();
    HINSTANCE hInst = (HINSTANCE)GetWindowLongPtr(hWnd, GWLP_HINSTANCE);

    // --- Search Edit ---
    g_app.hSearchEdit = CreateWindowExW(
        0, L"EDIT", L"",
        WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL | WS_TABSTOP,
        0, 0, 300, 32,
        hWnd, (HMENU)IDC_SEARCH_EDIT, hInst, nullptr
    );
    SendMessage(g_app.hSearchEdit, WM_SETFONT, (WPARAM)g_app.hFontUI, TRUE);
    SendMessage(g_app.hSearchEdit, EM_SETCUEBANNER, TRUE, (LPARAM)L"  \xD83D\xDD0D  Search tags and values...");

    // --- Tab Control ---
    g_app.hTabControl = CreateWindowExW(
        0, WC_TABCONTROL, L"",
        WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | TCS_HOTTRACK | TCS_TABS,
        0, 0, 100, 30,
        hWnd, (HMENU)IDC_TAB_CONTROL, hInst, nullptr
    );
    SendMessage(g_app.hTabControl, WM_SETFONT, (WPARAM)g_app.hFontSmall, TRUE);

    // Initial "All" tab
    TCITEMW tie = {};
    tie.mask = TCIF_TEXT;
    tie.pszText = (LPWSTR)L"All";
    TabCtrl_InsertItem(g_app.hTabControl, 0, &tie);

    // --- ListView ---
    g_app.hListView = CreateWindowExW(
        0, WC_LISTVIEW, L"",
        WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_SINGLESEL |
        LVS_SHOWSELALWAYS | LVS_NOSORTHEADER | WS_TABSTOP,
        0, 0, 100, 100,
        hWnd, (HMENU)IDC_LISTVIEW, hInst, nullptr
    );

    // ListView extended styles
    ListView_SetExtendedListViewStyle(g_app.hListView,
        LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES | LVS_EX_DOUBLEBUFFER |
        LVS_EX_INFOTIP);

    SendMessage(g_app.hListView, WM_SETFONT, (WPARAM)g_app.hFontUI, TRUE);

    // Dark mode for ListView
    SetWindowTheme(g_app.hListView, L"DarkMode_Explorer", nullptr);
    SetWindowTheme(g_app.hTabControl, L"DarkMode_Explorer", nullptr);
    SetWindowTheme(g_app.hSearchEdit, L"DarkMode_CFD", nullptr);

    // Add columns
    LVCOLUMNW lvc = {};
    lvc.mask = LVCF_FMT | LVCF_WIDTH | LVCF_TEXT;

    lvc.fmt = LVCFMT_LEFT;
    lvc.cx = 160;
    lvc.pszText = (LPWSTR)L"Group";
    ListView_InsertColumn(g_app.hListView, 0, &lvc);

    lvc.cx = 280;
    lvc.pszText = (LPWSTR)L"Tag";
    ListView_InsertColumn(g_app.hListView, 1, &lvc);

    lvc.cx = 600;
    lvc.pszText = (LPWSTR)L"Value";
    ListView_InsertColumn(g_app.hListView, 2, &lvc);

    // --- Buttons ---
    auto createBtn = [&](int id, const wchar_t* text, const wchar_t* icon, bool primary) -> HWND {
        HWND hBtn = CreateWindowExW(
            0, L"BUTTON", text,
            WS_CHILD | WS_VISIBLE | BS_OWNERDRAW | WS_TABSTOP,
            0, 0, 100, 36,
            hWnd, (HMENU)(INT_PTR)id, hInst, nullptr
        );
        g_buttons[hBtn] = { hBtn, text, icon, primary, false };
        return hBtn;
    };

    g_app.hOpenBtn   = createBtn(IDC_OPEN_BTN,   L"Open File",    L"\xE838", true);   // FolderOpen
    g_app.hExportBtn = createBtn(IDC_EXPORT_BTN,  L"Export",       L"\xE78C", false);  // Save
    g_app.hCopyBtn   = createBtn(IDC_COPY_BTN,    L"Copy All",     L"\xE8C8", false);  // Copy
    g_app.hClearBtn  = createBtn(IDC_CLEAR_BTN,   L"Clear",        L"\xE74D", false);  // Delete

    // --- Status Bar ---
    g_app.hStatusBar = CreateWindowExW(
        0, STATUSCLASSNAME, L"",
        WS_CHILD | WS_VISIBLE | SBARS_SIZEGRIP,
        0, 0, 0, 0,
        hWnd, (HMENU)IDC_STATUS_BAR, hInst, nullptr
    );
    SendMessage(g_app.hStatusBar, WM_SETFONT, (WPARAM)g_app.hFontSmall, TRUE);
    SetWindowTheme(g_app.hStatusBar, L"DarkMode_Explorer", nullptr);

    int parts[] = { 350, 600, -1 };
    SendMessage(g_app.hStatusBar, SB_SETPARTS, 3, (LPARAM)parts);
    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0, (LPARAM)L"  Ready — Drop a file or click Open");
    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 1, (LPARAM)L"  No file loaded");

    std::wstring exifStatus = g_app.exifToolPath.empty()
        ? L"  \x26A0 ExifTool not found"
        : L"  \x2705 ExifTool ready";
    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 2, (LPARAM)exifStatus.c_str());

    g_app.hWnd = hWnd;
}

// ============================================================================
// Layout
// ============================================================================
void LayoutControls(HWND hWnd) {
    RECT rc;
    GetClientRect(hWnd, &rc);
    int w = rc.right - rc.left;
    int h = rc.bottom - rc.top;

    int margin = 12;
    int topBarH = 48;
    int tabH = 32;
    int statusH = 24;

    // Send status bar size message
    SendMessage(g_app.hStatusBar, WM_SIZE, 0, 0);
    RECT statusRect;
    GetWindowRect(g_app.hStatusBar, &statusRect);
    statusH = statusRect.bottom - statusRect.top;

    int y = margin;

    // Top bar: [Search] [Open] [Export] [Copy] [Clear]
    int btnW = 100;
    int btnH = 36;
    int searchW = w - margin * 2 - (btnW + 8) * 4;
    if (searchW < 200) searchW = 200;

    MoveWindow(g_app.hSearchEdit, margin, y + 2, searchW, 32, TRUE);
    int bx = margin + searchW + 12;
    MoveWindow(g_app.hOpenBtn,   bx, y, btnW, btnH, TRUE); bx += btnW + 8;
    MoveWindow(g_app.hExportBtn, bx, y, btnW, btnH, TRUE); bx += btnW + 8;
    MoveWindow(g_app.hCopyBtn,   bx, y, btnW, btnH, TRUE); bx += btnW + 8;
    MoveWindow(g_app.hClearBtn,  bx, y, btnW, btnH, TRUE);
    y += topBarH;

    // Tab control
    MoveWindow(g_app.hTabControl, margin, y, w - margin * 2, tabH, TRUE);
    y += tabH + 4;

    // ListView fills remaining space
    int lvH = h - y - statusH - margin;
    if (lvH < 100) lvH = 100;
    MoveWindow(g_app.hListView, margin, y, w - margin * 2, lvH, TRUE);
}

// ============================================================================
// Load File Metadata
// ============================================================================
void LoadFileMetadata(const std::wstring& filePath) {
    g_app.isLoading = true;
    g_app.currentFile = filePath;

    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0, (LPARAM)L"  \x23F3 Analyzing file...");
    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 1, (LPARAM)(L"  " + filePath).c_str());
    UpdateWindow(g_app.hWnd);

    std::wstring output = RunExifTool(filePath);

    if (output.substr(0, 5) == L"ERROR") {
        MessageBoxW(g_app.hWnd, output.c_str(), L"MetaLens Error", MB_ICONERROR | MB_OK);
        g_app.isLoading = false;
        return;
    }

    ParseExifToolOutput(output);

    // Update tabs
    TabCtrl_DeleteAllItems(g_app.hTabControl);
    TCITEMW tie = {};
    tie.mask = TCIF_TEXT;
    tie.pszText = (LPWSTR)L"All";
    TabCtrl_InsertItem(g_app.hTabControl, 0, &tie);

    for (int i = 0; i < (int)g_app.groups.size(); i++) {
        tie.pszText = (LPWSTR)g_app.groups[i].c_str();
        TabCtrl_InsertItem(g_app.hTabControl, i + 1, &tie);
    }

    g_app.activeGroupIndex = 0;
    TabCtrl_SetCurSel(g_app.hTabControl, 0);

    g_app.searchQuery.clear();
    SetWindowTextW(g_app.hSearchEdit, L"");

    g_app.fileLoaded = true;
    g_app.isLoading = false;

    FilterEntries();
    PopulateListView();
    UpdateStatusBar();
    InvalidateRect(g_app.hWnd, nullptr, TRUE);
}

// ============================================================================
// Filter Entries
// ============================================================================
void FilterEntries() {
    g_app.filteredEntries.clear();
    std::wstring query = ToLower(g_app.searchQuery);

    std::wstring activeGroup;
    if (g_app.activeGroupIndex > 0 && g_app.activeGroupIndex <= (int)g_app.groups.size()) {
        activeGroup = g_app.groups[g_app.activeGroupIndex - 1];
    }

    for (const auto& e : g_app.allEntries) {
        // Group filter
        if (!activeGroup.empty() && e.group != activeGroup) continue;

        // Search filter
        if (!query.empty()) {
            std::wstring tagL = ToLower(e.tag);
            std::wstring valL = ToLower(e.value);
            std::wstring grpL = ToLower(e.group);
            if (tagL.find(query) == std::wstring::npos &&
                valL.find(query) == std::wstring::npos &&
                grpL.find(query) == std::wstring::npos) {
                continue;
            }
        }

        g_app.filteredEntries.push_back(e);
    }
}

// ============================================================================
// Populate ListView
// ============================================================================
void PopulateListView() {
    SendMessage(g_app.hListView, WM_SETREDRAW, FALSE, 0);
    ListView_DeleteAllItems(g_app.hListView);

    for (int i = 0; i < (int)g_app.filteredEntries.size(); i++) {
        const auto& e = g_app.filteredEntries[i];

        LVITEMW lvi = {};
        lvi.mask = LVIF_TEXT;
        lvi.iItem = i;

        lvi.iSubItem = 0;
        lvi.pszText = (LPWSTR)e.group.c_str();
        ListView_InsertItem(g_app.hListView, &lvi);

        ListView_SetItemText(g_app.hListView, i, 1, (LPWSTR)e.tag.c_str());
        ListView_SetItemText(g_app.hListView, i, 2, (LPWSTR)e.value.c_str());
    }

    SendMessage(g_app.hListView, WM_SETREDRAW, TRUE, 0);
    InvalidateRect(g_app.hListView, nullptr, TRUE);
}

// ============================================================================
// Update Status Bar
// ============================================================================
void UpdateStatusBar() {
    std::wstring status;
    if (g_app.fileLoaded) {
        status = L"  Showing " + std::to_wstring(g_app.filteredEntries.size())
               + L" / " + std::to_wstring(g_app.allEntries.size()) + L" fields";
    } else {
        status = L"  Ready — Drop a file or click Open";
    }
    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0, (LPARAM)status.c_str());

    if (!g_app.currentFile.empty()) {
        // Show filename
        std::wstring fname = PathFindFileNameW(g_app.currentFile.c_str());
        SendMessage(g_app.hStatusBar, SB_SETTEXTW, 1, (LPARAM)(L"  " + fname).c_str());
    }
}

// ============================================================================
// Open File Dialog
// ============================================================================
std::wstring OpenFileDialog(HWND hWnd) {
    wchar_t szFile[MAX_PATH] = {};

    OPENFILENAMEW ofn = {};
    ofn.lStructSize = sizeof(ofn);
    ofn.hwndOwner = hWnd;
    ofn.lpstrFile = szFile;
    ofn.nMaxFile = MAX_PATH;
    ofn.lpstrFilter =
        L"All Supported\0*.jpg;*.jpeg;*.png;*.tiff;*.tif;*.bmp;*.gif;*.webp;*.heic;*.heif;*.avif;"
        L"*.cr2;*.cr3;*.nef;*.nrw;*.arw;*.srf;*.sr2;*.orf;*.rw2;*.raf;*.dng;*.pef;*.3fr;*.iiq;*.rwl;*.x3f;"
        L"*.mp4;*.mov;*.avi;*.mkv;*.wmv;*.flv;*.webm;*.m4v;*.3gp;*.mts;*.m2ts;"
        L"*.mp3;*.wav;*.flac;*.aac;*.ogg;*.wma;*.m4a;"
        L"*.pdf;*.psd;*.ai;*.eps;*.svg\0"
        L"Photos (JPEG, PNG, TIFF, WebP, HEIC)\0*.jpg;*.jpeg;*.png;*.tiff;*.tif;*.bmp;*.gif;*.webp;*.heic;*.heif;*.avif\0"
        L"RAW Files (CR2, CR3, NEF, ARW, DNG, ...)\0*.cr2;*.cr3;*.nef;*.nrw;*.arw;*.srf;*.sr2;*.orf;*.rw2;*.raf;*.dng;*.pef;*.3fr;*.iiq;*.rwl;*.x3f\0"
        L"Video Files (MP4, MOV, MKV, ...)\0*.mp4;*.mov;*.avi;*.mkv;*.wmv;*.flv;*.webm;*.m4v;*.3gp;*.mts;*.m2ts\0"
        L"Audio Files\0*.mp3;*.wav;*.flac;*.aac;*.ogg;*.wma;*.m4a\0"
        L"All Files\0*.*\0";
    ofn.nFilterIndex = 1;
    ofn.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST | OFN_EXPLORER;

    if (GetOpenFileNameW(&ofn)) {
        return szFile;
    }
    return L"";
}

// ============================================================================
// Export to Text File
// ============================================================================
void ExportToFile() {
    if (g_app.allEntries.empty()) return;

    wchar_t szFile[MAX_PATH] = {};
    wcscpy_s(szFile, L"metadata_export.txt");

    OPENFILENAMEW ofn = {};
    ofn.lStructSize = sizeof(ofn);
    ofn.hwndOwner = g_app.hWnd;
    ofn.lpstrFile = szFile;
    ofn.nMaxFile = MAX_PATH;
    ofn.lpstrFilter = L"Text File\0*.txt\0CSV File\0*.csv\0All Files\0*.*\0";
    ofn.lpstrDefExt = L"txt";
    ofn.Flags = OFN_OVERWRITEPROMPT;

    if (!GetSaveFileNameW(&ofn)) return;

    std::wofstream ofs(szFile);
    ofs << L"MetaLens Metadata Export\n";
    ofs << L"========================\n";
    ofs << L"File: " << g_app.currentFile << L"\n";
    ofs << L"Total Fields: " << g_app.allEntries.size() << L"\n\n";

    std::wstring lastGroup;
    for (const auto& e : g_app.allEntries) {
        if (e.group != lastGroup) {
            ofs << L"\n--- " << e.group << L" ---\n";
            lastGroup = e.group;
        }
        ofs << L"  " << e.tag << L" : " << e.value << L"\n";
    }
    ofs.close();

    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0, (LPARAM)L"  \x2705 Exported successfully");
}

// ============================================================================
// Copy All to Clipboard
// ============================================================================
void CopyToClipboard() {
    if (g_app.filteredEntries.empty()) return;

    std::wstring text;
    text += L"MetaLens Metadata — " + g_app.currentFile + L"\r\n";
    text += L"═══════════════════════════════════════\r\n\r\n";

    std::wstring lastGroup;
    for (const auto& e : g_app.filteredEntries) {
        if (e.group != lastGroup) {
            text += L"\r\n── " + e.group + L" ──\r\n";
            lastGroup = e.group;
        }
        text += L"  " + e.tag + L"  :  " + e.value + L"\r\n";
    }

    if (!OpenClipboard(g_app.hWnd)) return;
    EmptyClipboard();

    size_t bytes = (text.size() + 1) * sizeof(wchar_t);
    HGLOBAL hMem = GlobalAlloc(GMEM_MOVEABLE, bytes);
    if (hMem) {
        void* pMem = GlobalLock(hMem);
        memcpy(pMem, text.c_str(), bytes);
        GlobalUnlock(hMem);
        SetClipboardData(CF_UNICODETEXT, hMem);
    }
    CloseClipboard();

    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0,
        (LPARAM)(L"  \x2705 Copied " + std::to_wstring(g_app.filteredEntries.size()) + L" fields").c_str());
}

// ============================================================================
// Custom Paint: Drop Zone Overlay (when no file loaded)
// ============================================================================
void PaintDropZone(HWND hWnd, HDC hdc) {
    RECT rc;
    GetClientRect(hWnd, &rc);

    // Only paint in the listview area
    RECT lvRect;
    GetWindowRect(g_app.hListView, &lvRect);
    MapWindowPoints(HWND_DESKTOP, hWnd, (POINT*)&lvRect, 2);

    // Fill with dark background
    HBRUSH hBr = CreateSolidBrush(Theme::DropzoneBg);
    FillRect(hdc, &lvRect, hBr);
    DeleteObject(hBr);

    // Draw dashed border
    HPEN hPen = CreatePen(PS_DASH, 2, Theme::DropzoneBorder);
    HPEN oldPen = (HPEN)SelectObject(hdc, hPen);
    HBRUSH oldBr = (HBRUSH)SelectObject(hdc, GetStockObject(NULL_BRUSH));
    int inset = 20;
    RoundRect(hdc, lvRect.left + inset, lvRect.top + inset,
              lvRect.right - inset, lvRect.bottom - inset, 16, 16);
    SelectObject(hdc, oldPen);
    SelectObject(hdc, oldBr);
    DeleteObject(hPen);

    // Center text
    SetBkMode(hdc, TRANSPARENT);

    int cy = (lvRect.top + lvRect.bottom) / 2;

    // Drop icon
    SelectObject(hdc, g_app.hFontTitle);
    SetTextColor(hdc, g_app.isDragHover ? Theme::AccentPurple : Theme::TextMuted);
    RECT iconRect = { lvRect.left, cy - 60, lvRect.right, cy - 20 };
    DrawTextW(hdc, L"\xD83D\xDCC2", -1, &iconRect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);

    // "Drop file here" text
    SelectObject(hdc, g_app.hFontTitle);
    SetTextColor(hdc, g_app.isDragHover ? Theme::TextAccent : Theme::TextSecondary);
    RECT textRect = { lvRect.left, cy - 20, lvRect.right, cy + 15 };
    DrawTextW(hdc, g_app.isDragHover ? L"Release to analyze" : L"Drop a file here to analyze", -1, &textRect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);

    // Subtitle
    SelectObject(hdc, g_app.hFontSmall);
    SetTextColor(hdc, Theme::TextMuted);
    RECT subRect = { lvRect.left, cy + 20, lvRect.right, cy + 45 };
    DrawTextW(hdc, L"Supports photos, RAW (CR2, NEF, ARW), videos (MP4, MOV), and 400+ formats", -1, &subRect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);

    if (g_app.exifToolPath.empty()) {
        SetTextColor(hdc, Theme::TextOrange);
        RECT warnRect = { lvRect.left, cy + 55, lvRect.right, cy + 80 };
        DrawTextW(hdc, L"\x26A0  ExifTool not found — place exiftool.exe next to MetaLens.exe", -1, &warnRect, DT_CENTER | DT_SINGLELINE | DT_VCENTER);
    }
}

// ============================================================================
// Custom Draw Handler for ListView
// ============================================================================
LRESULT HandleListViewCustomDraw(LPNMLVCUSTOMDRAW pCD) {
    switch (pCD->nmcd.dwDrawStage) {
    case CDDS_PREPAINT:
        return CDRF_NOTIFYITEMDRAW;

    case CDDS_ITEMPREPAINT:
        return CDRF_NOTIFYSUBITEMDRAW;

    case CDDS_ITEMPREPAINT | CDDS_SUBITEM: {
        int row = (int)pCD->nmcd.dwItemSpec;
        int col = pCD->iSubItem;

        // Alternate row coloring
        if (row % 2 == 0) {
            pCD->clrTextBk = Theme::BgPanel;
        } else {
            pCD->clrTextBk = Theme::BgDark;
        }

        // Color by column
        switch (col) {
        case 0: // Group
            pCD->clrText = Theme::TextAccent;
            break;
        case 1: // Tag
            pCD->clrText = Theme::TextCyan;
            break;
        case 2: // Value
            pCD->clrText = Theme::TextPrimary;
            break;
        }

        // Highlight "Identified Camera" and "Identified Lens" rows
        if (row < (int)g_app.filteredEntries.size()) {
            const auto& entry = g_app.filteredEntries[row];
            if (entry.tag.find(L"Identified Camera") != std::wstring::npos) {
                pCD->clrText = Theme::TextGreen;
                pCD->clrTextBk = RGB(20, 40, 30);
            }
            if (entry.tag.find(L"Identified Lens") != std::wstring::npos ||
                (entry.group == L"Camera Info" && entry.tag.find(L"Lens") != std::wstring::npos)) {
                pCD->clrText = Theme::TextOrange;
                pCD->clrTextBk = RGB(40, 32, 18);
            }
        }

        return CDRF_NEWFONT;
    }
    }
    return CDRF_DODEFAULT;
}

// ============================================================================
// Owner-Draw Buttons
// ============================================================================
void DrawCustomButton(LPDRAWITEMSTRUCT pDIS) {
    auto it = g_buttons.find(pDIS->hwndItem);
    if (it == g_buttons.end()) return;

    BtnInfo& btn = it->second;
    HDC hdc = pDIS->hDC;
    RECT rc = pDIS->rcItem;

    bool pressed = (pDIS->itemState & ODS_SELECTED) != 0;
    bool focused = (pDIS->itemState & ODS_FOCUS) != 0;

    COLORREF bgColor, textColor, borderColor;
    if (btn.isPrimary) {
        bgColor = pressed ? RGB(90, 70, 220) : (btn.isHovered ? Theme::BtnHover : Theme::BtnBg);
        textColor = Theme::BtnText;
        borderColor = Theme::AccentPurple;
    } else {
        bgColor = pressed ? RGB(35, 35, 50) : (btn.isHovered ? Theme::BtnSecHover : Theme::BtnSecBg);
        textColor = Theme::TextSecondary;
        borderColor = Theme::Border;
    }

    DrawRoundRect(hdc, rc, 8, bgColor, borderColor);

    SetBkMode(hdc, TRANSPARENT);
    SetTextColor(hdc, textColor);

    // Draw icon + text
    SelectObject(hdc, g_app.hFontUI);
    std::wstring label = btn.text;
    DrawTextW(hdc, label.c_str(), -1, &rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
}

// ============================================================================
// Window Procedure
// ============================================================================
LRESULT CALLBACK WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch (msg) {
    case WM_CREATE:
        CreateUIControls(hWnd);
        LayoutControls(hWnd);
        return 0;

    case WM_SIZE:
        LayoutControls(hWnd);
        return 0;

    case WM_GETMINMAXINFO: {
        MINMAXINFO* mmi = (MINMAXINFO*)lParam;
        mmi->ptMinTrackSize.x = 700;
        mmi->ptMinTrackSize.y = 500;
        return 0;
    }

    case WM_ERASEBKGND: {
        HDC hdc = (HDC)wParam;
        RECT rc;
        GetClientRect(hWnd, &rc);
        FillRect(hdc, &rc, g_app.hBrDark);
        return 1;
    }

    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC hdc = BeginPaint(hWnd, &ps);

        // Draw app title in top-left area above search (if space allows)
        // Draw drop zone if no file loaded
        if (!g_app.fileLoaded && g_app.hListView) {
            PaintDropZone(hWnd, hdc);
        }

        EndPaint(hWnd, &ps);
        return 0;
    }

    case WM_CTLCOLOREDIT: {
        HDC hdcEdit = (HDC)wParam;
        SetBkColor(hdcEdit, Theme::BgInput);
        SetTextColor(hdcEdit, Theme::TextPrimary);
        return (LRESULT)g_app.hBrInput;
    }

    case WM_CTLCOLORSTATIC: {
        HDC hdcStatic = (HDC)wParam;
        SetBkColor(hdcStatic, Theme::BgDark);
        SetTextColor(hdcStatic, Theme::TextSecondary);
        return (LRESULT)g_app.hBrDark;
    }

    // Drag & Drop
    case WM_DROPFILES: {
        HDROP hDrop = (HDROP)wParam;
        wchar_t filePath[MAX_PATH];
        DragQueryFileW(hDrop, 0, filePath, MAX_PATH);
        DragFinish(hDrop);
        g_app.isDragHover = false;
        LoadFileMetadata(filePath);
        return 0;
    }

    // Custom message: load file from command line
    case WM_USER + 100: {
        wchar_t* filePath = (wchar_t*)lParam;
        if (filePath) {
            LoadFileMetadata(filePath);
            free(filePath);
        }
        return 0;
    }

    // Tab changed
    case WM_NOTIFY: {
        NMHDR* pNMHDR = (NMHDR*)lParam;

        if (pNMHDR->hwndFrom == g_app.hTabControl && pNMHDR->code == TCN_SELCHANGE) {
            g_app.activeGroupIndex = TabCtrl_GetCurSel(g_app.hTabControl);
            FilterEntries();
            PopulateListView();
            UpdateStatusBar();
            return 0;
        }

        if (pNMHDR->hwndFrom == g_app.hListView && pNMHDR->code == NM_CUSTOMDRAW) {
            LRESULT result = HandleListViewCustomDraw((LPNMLVCUSTOMDRAW)lParam);
            SetWindowLongPtr(hWnd, DWLP_MSGRESULT, result);
            return result;
        }

        // Double-click to copy single value
        if (pNMHDR->hwndFrom == g_app.hListView && pNMHDR->code == NM_DBLCLK) {
            int sel = ListView_GetNextItem(g_app.hListView, -1, LVNI_SELECTED);
            if (sel >= 0 && sel < (int)g_app.filteredEntries.size()) {
                const auto& entry = g_app.filteredEntries[sel];
                std::wstring text = entry.tag + L": " + entry.value;
                if (OpenClipboard(hWnd)) {
                    EmptyClipboard();
                    size_t bytes = (text.size() + 1) * sizeof(wchar_t);
                    HGLOBAL hMem = GlobalAlloc(GMEM_MOVEABLE, bytes);
                    if (hMem) {
                        memcpy(GlobalLock(hMem), text.c_str(), bytes);
                        GlobalUnlock(hMem);
                        SetClipboardData(CF_UNICODETEXT, hMem);
                    }
                    CloseClipboard();
                    SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0,
                        (LPARAM)(L"  \x2705 Copied: " + entry.tag).c_str());
                }
            }
            return 0;
        }

        break;
    }

    case WM_COMMAND: {
        int id = LOWORD(wParam);
        int notif = HIWORD(wParam);

        if (id == IDC_SEARCH_EDIT && notif == EN_CHANGE) {
            wchar_t buf[256] = {};
            GetWindowTextW(g_app.hSearchEdit, buf, 256);
            g_app.searchQuery = buf;
            FilterEntries();
            PopulateListView();
            UpdateStatusBar();
            return 0;
        }

        if (id == IDC_OPEN_BTN) {
            std::wstring file = OpenFileDialog(hWnd);
            if (!file.empty()) {
                LoadFileMetadata(file);
            }
            return 0;
        }

        if (id == IDC_EXPORT_BTN) {
            ExportToFile();
            return 0;
        }

        if (id == IDC_COPY_BTN) {
            CopyToClipboard();
            return 0;
        }

        if (id == IDC_CLEAR_BTN) {
            g_app.allEntries.clear();
            g_app.filteredEntries.clear();
            g_app.groups.clear();
            g_app.currentFile.clear();
            g_app.fileLoaded = false;
            g_app.searchQuery.clear();
            g_app.activeGroupIndex = 0;
            SetWindowTextW(g_app.hSearchEdit, L"");
            ListView_DeleteAllItems(g_app.hListView);
            TabCtrl_DeleteAllItems(g_app.hTabControl);
            TCITEMW tie = {};
            tie.mask = TCIF_TEXT;
            tie.pszText = (LPWSTR)L"All";
            TabCtrl_InsertItem(g_app.hTabControl, 0, &tie);
            SendMessage(g_app.hStatusBar, SB_SETTEXTW, 0, (LPARAM)L"  Ready — Drop a file or click Open");
            SendMessage(g_app.hStatusBar, SB_SETTEXTW, 1, (LPARAM)L"  No file loaded");
            InvalidateRect(hWnd, nullptr, TRUE);
            return 0;
        }

        break;
    }

    // Owner-draw buttons
    case WM_DRAWITEM: {
        LPDRAWITEMSTRUCT pDIS = (LPDRAWITEMSTRUCT)lParam;
        if (pDIS->CtlType == ODT_BUTTON) {
            DrawCustomButton(pDIS);
            return TRUE;
        }
        break;
    }

    case WM_DESTROY:
        if (g_app.hFontUI) DeleteObject(g_app.hFontUI);
        if (g_app.hFontMono) DeleteObject(g_app.hFontMono);
        if (g_app.hFontTitle) DeleteObject(g_app.hFontTitle);
        if (g_app.hFontSmall) DeleteObject(g_app.hFontSmall);
        if (g_app.hFontIcon) DeleteObject(g_app.hFontIcon);
        if (g_app.hBrDark) DeleteObject(g_app.hBrDark);
        if (g_app.hBrPanel) DeleteObject(g_app.hBrPanel);
        if (g_app.hBrInput) DeleteObject(g_app.hBrInput);
        if (g_app.hBrHeader) DeleteObject(g_app.hBrHeader);
        PostQuitMessage(0);
        return 0;
    }

    return DefWindowProc(hWnd, msg, wParam, lParam);
}
