#pragma once
// ============================================================================
// camera_db.h - Camera Model Name Resolution Database
// Maps internal/EXIF model strings to their commonly known marketing names
// ============================================================================

#include <string>
#include <unordered_map>
#include <algorithm>

struct CameraInfo {
    std::wstring brand;
    std::wstring model;         // Internal / EXIF model string
    std::wstring marketingName; // Common name as known publicly
};

// Normalize a string for fuzzy matching (lowercase, strip spaces)
inline std::wstring NormalizeModelString(const std::wstring& s) {
    std::wstring out = s;
    std::transform(out.begin(), out.end(), out.begin(), ::towlower);
    // Remove leading/trailing whitespace
    size_t start = out.find_first_not_of(L" \t\r\n");
    size_t end = out.find_last_not_of(L" \t\r\n");
    if (start == std::wstring::npos) return L"";
    return out.substr(start, end - start + 1);
}

// Returns a map of normalized EXIF model -> marketing name
inline const std::unordered_map<std::wstring, std::wstring>& GetCameraModelDB() {
    static const std::unordered_map<std::wstring, std::wstring> db = {
        // =====================================================================
        // CANON - EOS DSLRs
        // =====================================================================
        {L"canon eos-1d x mark iii",          L"Canon EOS-1D X Mark III"},
        {L"canon eos-1d x mark ii",           L"Canon EOS-1D X Mark II"},
        {L"canon eos-1d x",                   L"Canon EOS-1D X"},
        {L"canon eos 5d mark iv",             L"Canon EOS 5D Mark IV"},
        {L"canon eos 5d mark iii",            L"Canon EOS 5D Mark III"},
        {L"canon eos 5d mark ii",             L"Canon EOS 5D Mark II"},
        {L"canon eos 5ds r",                  L"Canon EOS 5DS R"},
        {L"canon eos 5ds",                    L"Canon EOS 5DS"},
        {L"canon eos 6d mark ii",             L"Canon EOS 6D Mark II"},
        {L"canon eos 6d",                     L"Canon EOS 6D"},
        {L"canon eos 7d mark ii",             L"Canon EOS 7D Mark II"},
        {L"canon eos 7d",                     L"Canon EOS 7D"},
        {L"canon eos 77d",                    L"Canon EOS 77D"},
        {L"canon eos 80d",                    L"Canon EOS 80D"},
        {L"canon eos 90d",                    L"Canon EOS 90D"},
        {L"canon eos 70d",                    L"Canon EOS 70D"},
        {L"canon eos 60d",                    L"Canon EOS 60D"},
        {L"canon eos 50d",                    L"Canon EOS 50D"},
        {L"canon eos 40d",                    L"Canon EOS 40D"},
        // Canon Rebel / Kiss variants
        {L"canon eos rebel t8i",              L"Canon EOS Rebel T8i / EOS 850D / EOS Kiss X10i"},
        {L"canon eos 850d",                   L"Canon EOS 850D / Rebel T8i / Kiss X10i"},
        {L"canon eos kiss x10i",              L"Canon EOS Kiss X10i / 850D / Rebel T8i"},
        {L"canon eos rebel t7i",              L"Canon EOS Rebel T7i / EOS 800D / EOS Kiss X9i"},
        {L"canon eos 800d",                   L"Canon EOS 800D / Rebel T7i / Kiss X9i"},
        {L"canon eos rebel t7",               L"Canon EOS Rebel T7 / EOS 2000D / EOS Kiss X90"},
        {L"canon eos 2000d",                  L"Canon EOS 2000D / Rebel T7 / Kiss X90"},
        {L"canon eos rebel t6i",              L"Canon EOS Rebel T6i / EOS 750D / Kiss X8i"},
        {L"canon eos 750d",                   L"Canon EOS 750D / Rebel T6i / Kiss X8i"},
        {L"canon eos rebel t6",               L"Canon EOS Rebel T6 / EOS 1300D / Kiss X80"},
        {L"canon eos 1300d",                  L"Canon EOS 1300D / Rebel T6 / Kiss X80"},
        {L"canon eos rebel t5i",              L"Canon EOS Rebel T5i / EOS 700D / Kiss X7i"},
        {L"canon eos 700d",                   L"Canon EOS 700D / Rebel T5i / Kiss X7i"},
        {L"canon eos rebel t5",               L"Canon EOS Rebel T5 / EOS 1200D / Kiss X70"},
        {L"canon eos 1200d",                  L"Canon EOS 1200D / Rebel T5 / Kiss X70"},
        {L"canon eos rebel t3i",              L"Canon EOS Rebel T3i / EOS 600D / Kiss X5"},
        {L"canon eos 600d",                   L"Canon EOS 600D / Rebel T3i / Kiss X5"},
        {L"canon eos rebel t3",               L"Canon EOS Rebel T3 / EOS 1100D / Kiss X50"},
        {L"canon eos 1100d",                  L"Canon EOS 1100D / Rebel T3 / Kiss X50"},
        {L"canon eos rebel sl3",              L"Canon EOS Rebel SL3 / EOS 250D / Kiss X10"},
        {L"canon eos 250d",                   L"Canon EOS 250D / Rebel SL3 / Kiss X10"},
        {L"canon eos rebel sl2",              L"Canon EOS Rebel SL2 / EOS 200D / Kiss X9"},
        {L"canon eos 200d",                   L"Canon EOS 200D / Rebel SL2 / Kiss X9"},
        {L"canon eos 200d ii",                L"Canon EOS 200D II / Rebel SL3 / Kiss X10"},
        // Canon EOS R Mirrorless
        {L"canon eos r1",                     L"Canon EOS R1"},
        {L"canon eos r3",                     L"Canon EOS R3"},
        {L"canon eos r5",                     L"Canon EOS R5"},
        {L"canon eos r5 c",                   L"Canon EOS R5 C"},
        {L"canon eos r5 mark ii",             L"Canon EOS R5 Mark II"},
        {L"canon eos r6",                     L"Canon EOS R6"},
        {L"canon eos r6 mark ii",             L"Canon EOS R6 Mark II"},
        {L"canon eos r7",                     L"Canon EOS R7"},
        {L"canon eos r8",                     L"Canon EOS R8"},
        {L"canon eos r10",                    L"Canon EOS R10"},
        {L"canon eos r50",                    L"Canon EOS R50"},
        {L"canon eos r100",                   L"Canon EOS R100"},
        {L"canon eos rp",                     L"Canon EOS RP"},
        {L"canon eos r",                      L"Canon EOS R"},
        // Canon EOS M
        {L"canon eos m50 mark ii",            L"Canon EOS M50 Mark II / Kiss M2"},
        {L"canon eos m50",                    L"Canon EOS M50 / Kiss M"},
        {L"canon eos m6 mark ii",             L"Canon EOS M6 Mark II"},
        {L"canon eos m200",                   L"Canon EOS M200"},
        // Canon PowerShot / IXUS
        {L"canon powershot g7 x mark iii",    L"Canon PowerShot G7 X Mark III"},
        {L"canon powershot g7 x mark ii",     L"Canon PowerShot G7 X Mark II"},
        {L"canon powershot g5 x mark ii",     L"Canon PowerShot G5 X Mark II"},
        {L"canon powershot sx740 hs",         L"Canon PowerShot SX740 HS"},
        {L"canon powershot sx70 hs",          L"Canon PowerShot SX70 HS"},

        // =====================================================================
        // NIKON
        // =====================================================================
        // Nikon Z Mirrorless
        {L"nikon z 9",                        L"Nikon Z 9"},
        {L"z 9",                              L"Nikon Z 9"},
        {L"nikon z 8",                        L"Nikon Z 8"},
        {L"z 8",                              L"Nikon Z 8"},
        {L"nikon z 7ii",                      L"Nikon Z 7II"},
        {L"nikon z 7",                        L"Nikon Z 7"},
        {L"nikon z 6iii",                     L"Nikon Z 6III"},
        {L"nikon z 6ii",                      L"Nikon Z 6II"},
        {L"nikon z 6",                        L"Nikon Z 6"},
        {L"nikon z 5",                        L"Nikon Z 5"},
        {L"nikon z 50",                       L"Nikon Z 50"},
        {L"nikon z 30",                       L"Nikon Z 30"},
        {L"nikon z fc",                       L"Nikon Z fc"},
        {L"nikon z f",                        L"Nikon Z f"},
        // Nikon DSLRs
        {L"nikon d6",                         L"Nikon D6"},
        {L"nikon d5",                         L"Nikon D5"},
        {L"nikon d4s",                        L"Nikon D4S"},
        {L"nikon d4",                         L"Nikon D4"},
        {L"nikon d850",                       L"Nikon D850"},
        {L"nikon d810",                       L"Nikon D810"},
        {L"nikon d800e",                      L"Nikon D800E"},
        {L"nikon d800",                       L"Nikon D800"},
        {L"nikon d780",                       L"Nikon D780"},
        {L"nikon d750",                       L"Nikon D750"},
        {L"nikon d610",                       L"Nikon D610"},
        {L"nikon d600",                       L"Nikon D600"},
        {L"nikon d500",                       L"Nikon D500"},
        {L"nikon d7500",                      L"Nikon D7500"},
        {L"nikon d7200",                      L"Nikon D7200"},
        {L"nikon d7100",                      L"Nikon D7100"},
        {L"nikon d5600",                      L"Nikon D5600"},
        {L"nikon d5500",                      L"Nikon D5500"},
        {L"nikon d5300",                      L"Nikon D5300"},
        {L"nikon d3500",                      L"Nikon D3500"},
        {L"nikon d3400",                      L"Nikon D3400"},
        {L"nikon d3300",                      L"Nikon D3300"},

        // =====================================================================
        // SONY
        // =====================================================================
        // Sony Alpha Full-Frame Mirrorless
        {L"ilce-1",                           L"Sony Alpha 1 (A1)"},
        {L"ilce-9m3",                         L"Sony Alpha 9 III (A9 III)"},
        {L"ilce-9m2",                         L"Sony Alpha 9 II (A9 II)"},
        {L"ilce-9",                           L"Sony Alpha 9 (A9)"},
        {L"ilce-7rm5",                        L"Sony Alpha 7R V (A7R V)"},
        {L"ilce-7rm4a",                       L"Sony Alpha 7R IVA (A7R IVA)"},
        {L"ilce-7rm4",                        L"Sony Alpha 7R IV (A7R IV)"},
        {L"ilce-7rm3a",                       L"Sony Alpha 7R IIIA (A7R IIIA)"},
        {L"ilce-7rm3",                        L"Sony Alpha 7R III (A7R III)"},
        {L"ilce-7rm2",                        L"Sony Alpha 7R II (A7R II)"},
        {L"ilce-7r",                          L"Sony Alpha 7R (A7R)"},
        {L"ilce-7sm3",                        L"Sony Alpha 7S III (A7S III)"},
        {L"ilce-7sm2",                        L"Sony Alpha 7S II (A7S II)"},
        {L"ilce-7s",                          L"Sony Alpha 7S (A7S)"},
        {L"ilce-7m4",                         L"Sony Alpha 7 IV (A7 IV)"},
        {L"ilce-7m3",                         L"Sony Alpha 7 III (A7 III)"},
        {L"ilce-7m2",                         L"Sony Alpha 7 II (A7 II)"},
        {L"ilce-7",                           L"Sony Alpha 7 (A7)"},
        {L"ilce-7c2",                         L"Sony Alpha 7C II (A7C II)"},
        {L"ilce-7cr",                         L"Sony Alpha 7CR (A7CR)"},
        {L"ilce-7c",                          L"Sony Alpha 7C (A7C)"},
        // Sony Alpha APS-C Mirrorless
        {L"ilce-6700",                        L"Sony Alpha 6700 (A6700)"},
        {L"ilce-6600",                        L"Sony Alpha 6600 (A6600)"},
        {L"ilce-6500",                        L"Sony Alpha 6500 (A6500)"},
        {L"ilce-6400",                        L"Sony Alpha 6400 (A6400)"},
        {L"ilce-6300",                        L"Sony Alpha 6300 (A6300)"},
        {L"ilce-6100",                        L"Sony Alpha 6100 (A6100)"},
        {L"ilce-6000",                        L"Sony Alpha 6000 (A6000)"},
        // Sony FX / Cinema
        {L"ilme-fx30",                        L"Sony FX30"},
        {L"ilme-fx3",                         L"Sony FX3"},
        {L"ilme-fx6",                         L"Sony FX6"},
        // Sony ZV Vlogging
        {L"zv-e10",                           L"Sony ZV-E10"},
        {L"zv-e10m2",                         L"Sony ZV-E10 II"},
        {L"zv-e1",                            L"Sony ZV-E1"},
        {L"zv-1",                             L"Sony ZV-1"},
        {L"zv-1m2",                           L"Sony ZV-1 II"},
        {L"zv-1f",                            L"Sony ZV-1F"},
        // Sony RX
        {L"dsc-rx100m7",                      L"Sony RX100 VII"},
        {L"dsc-rx100m6",                      L"Sony RX100 VI"},
        {L"dsc-rx100m5a",                     L"Sony RX100 VA"},
        {L"dsc-rx100m5",                      L"Sony RX100 V"},
        {L"dsc-rx10m4",                       L"Sony RX10 IV"},
        {L"dsc-rx1rm2",                       L"Sony RX1R II"},

        // =====================================================================
        // FUJIFILM
        // =====================================================================
        {L"x-t5",                             L"Fujifilm X-T5"},
        {L"x-t4",                             L"Fujifilm X-T4"},
        {L"x-t3",                             L"Fujifilm X-T3"},
        {L"x-t30 ii",                         L"Fujifilm X-T30 II"},
        {L"x-t30",                            L"Fujifilm X-T30"},
        {L"x-t200",                           L"Fujifilm X-T200"},
        {L"x-h2s",                            L"Fujifilm X-H2S"},
        {L"x-h2",                             L"Fujifilm X-H2"},
        {L"x-s20",                            L"Fujifilm X-S20"},
        {L"x-s10",                            L"Fujifilm X-S10"},
        {L"x-e4",                             L"Fujifilm X-E4"},
        {L"x100vi",                           L"Fujifilm X100VI"},
        {L"x100v",                            L"Fujifilm X100V"},
        {L"x100f",                            L"Fujifilm X100F"},
        {L"gfx100s ii",                       L"Fujifilm GFX100S II"},
        {L"gfx100s",                          L"Fujifilm GFX100S"},
        {L"gfx 50s ii",                       L"Fujifilm GFX 50S II"},
        {L"gfx 100 ii",                       L"Fujifilm GFX 100 II"},

        // =====================================================================
        // PANASONIC LUMIX
        // =====================================================================
        {L"dc-s5m2",                          L"Panasonic Lumix S5 II"},
        {L"dc-s5m2x",                         L"Panasonic Lumix S5 IIX"},
        {L"dc-s5",                            L"Panasonic Lumix S5"},
        {L"dc-s1h",                           L"Panasonic Lumix S1H"},
        {L"dc-s1r",                           L"Panasonic Lumix S1R"},
        {L"dc-s1",                            L"Panasonic Lumix S1"},
        {L"dc-gh6",                           L"Panasonic Lumix GH6"},
        {L"dc-gh5m2",                         L"Panasonic Lumix GH5 II"},
        {L"dc-gh5s",                          L"Panasonic Lumix GH5S"},
        {L"dc-gh5",                           L"Panasonic Lumix GH5"},
        {L"dc-g9m2",                          L"Panasonic Lumix G9 II"},
        {L"dc-g9",                            L"Panasonic Lumix G9"},
        {L"dc-g100",                          L"Panasonic Lumix G100"},
        {L"dc-fz1000m2",                      L"Panasonic Lumix FZ1000 II"},
        {L"dc-lx100m2",                       L"Panasonic Lumix LX100 II"},

        // =====================================================================
        // OLYMPUS / OM SYSTEM
        // =====================================================================
        {L"e-m1 mark iii",                    L"Olympus OM-D E-M1 Mark III"},
        {L"e-m1 mark ii",                     L"Olympus OM-D E-M1 Mark II"},
        {L"e-m1x",                            L"Olympus OM-D E-M1X"},
        {L"e-m5 mark iii",                    L"Olympus OM-D E-M5 Mark III"},
        {L"e-m10 mark iv",                    L"Olympus OM-D E-M10 Mark IV"},
        {L"e-m10 mark iii",                   L"Olympus OM-D E-M10 Mark III"},
        {L"om-1 mark ii",                     L"OM System OM-1 Mark II"},
        {L"om-1",                             L"OM System OM-1"},
        {L"om-5",                             L"OM System OM-5"},

        // =====================================================================
        // LEICA
        // =====================================================================
        {L"leica sl3",                        L"Leica SL3"},
        {L"leica sl2-s",                      L"Leica SL2-S"},
        {L"leica sl2",                        L"Leica SL2"},
        {L"leica q3",                         L"Leica Q3"},
        {L"leica q2",                         L"Leica Q2"},
        {L"leica m11",                        L"Leica M11"},
        {L"leica m10-r",                      L"Leica M10-R"},
        {L"leica m10",                        L"Leica M10"},

        // =====================================================================
        // HASSELBLAD
        // =====================================================================
        {L"hasselblad x2d 100c",              L"Hasselblad X2D 100C"},
        {L"hasselblad x1d ii 50c",            L"Hasselblad X1D II 50C"},
        {L"hasselblad 907x 50c",              L"Hasselblad 907X 50C"},
        {L"l2d-20c",                          L"DJI Mavic 3 (Hasselblad L2D-20c)"},

        // =====================================================================
        // DJI (Drones & Action Cameras)
        // =====================================================================
        {L"fc3582",                           L"DJI Mini 4 Pro"},
        {L"fc3411",                           L"DJI Mini 3 Pro"},
        {L"fc3170",                           L"DJI Mavic 3"},
        {L"fc7303",                           L"DJI Air 3"},
        {L"fc3541",                           L"DJI Air 2S"},
        {L"fc3280",                           L"DJI Mavic Air 2"},
        {L"fc220",                            L"DJI Mavic Pro"},
        {L"fc2103",                           L"DJI Mavic Air"},
        {L"fc6310",                           L"DJI Phantom 4 Pro"},
        {L"fc6510",                           L"DJI Phantom 4 Pro V2.0"},
        {L"fc330",                            L"DJI Phantom 4"},
        {L"fc350",                            L"DJI Phantom 3 Professional"},
        {L"fc300x",                           L"DJI Phantom 3 Advanced"},
        {L"fc300s",                           L"DJI Phantom 3 Standard"},
        {L"osmo action 4",                    L"DJI Osmo Action 4"},
        {L"osmo action 3",                    L"DJI Osmo Action 3"},
        {L"osmo pocket 3",                    L"DJI Osmo Pocket 3"},

        // =====================================================================
        // GOPRO
        // =====================================================================
        {L"hero13 black",                     L"GoPro HERO13 Black"},
        {L"hero12 black",                     L"GoPro HERO12 Black"},
        {L"hero11 black",                     L"GoPro HERO11 Black"},
        {L"hero10 black",                     L"GoPro HERO10 Black"},
        {L"hero9 black",                      L"GoPro HERO9 Black"},
        {L"hero8 black",                      L"GoPro HERO8 Black"},
        {L"hero7 black",                      L"GoPro HERO7 Black"},

        // =====================================================================
        // APPLE (iPhones)
        // =====================================================================
        {L"iphone 16 pro max",               L"Apple iPhone 16 Pro Max"},
        {L"iphone 16 pro",                   L"Apple iPhone 16 Pro"},
        {L"iphone 16 plus",                  L"Apple iPhone 16 Plus"},
        {L"iphone 16",                       L"Apple iPhone 16"},
        {L"iphone 15 pro max",               L"Apple iPhone 15 Pro Max"},
        {L"iphone 15 pro",                   L"Apple iPhone 15 Pro"},
        {L"iphone 15 plus",                  L"Apple iPhone 15 Plus"},
        {L"iphone 15",                       L"Apple iPhone 15"},
        {L"iphone 14 pro max",               L"Apple iPhone 14 Pro Max"},
        {L"iphone 14 pro",                   L"Apple iPhone 14 Pro"},
        {L"iphone 14 plus",                  L"Apple iPhone 14 Plus"},
        {L"iphone 14",                       L"Apple iPhone 14"},
        {L"iphone 13 pro max",               L"Apple iPhone 13 Pro Max"},
        {L"iphone 13 pro",                   L"Apple iPhone 13 Pro"},
        {L"iphone 13",                       L"Apple iPhone 13"},
        {L"iphone 12 pro max",               L"Apple iPhone 12 Pro Max"},
        {L"iphone 12 pro",                   L"Apple iPhone 12 Pro"},
        {L"iphone 12",                       L"Apple iPhone 12"},

        // =====================================================================
        // SAMSUNG (Galaxy Phones)
        // =====================================================================
        {L"sm-s928b",                         L"Samsung Galaxy S25 Ultra"},
        {L"sm-s926b",                         L"Samsung Galaxy S25+"},
        {L"sm-s921b",                         L"Samsung Galaxy S25"},
        {L"sm-s918b",                         L"Samsung Galaxy S24 Ultra"},
        {L"sm-s916b",                         L"Samsung Galaxy S24+"},
        {L"sm-s911b",                         L"Samsung Galaxy S24"},
        {L"sm-s908b",                         L"Samsung Galaxy S23 Ultra"},
        {L"sm-s906b",                         L"Samsung Galaxy S23+"},
        {L"sm-s901b",                         L"Samsung Galaxy S23"},
        {L"sm-g998b",                         L"Samsung Galaxy S21 Ultra"},
        {L"sm-g991b",                         L"Samsung Galaxy S21"},
        {L"sm-g988b",                         L"Samsung Galaxy S20 Ultra"},
        {L"sm-g986b",                         L"Samsung Galaxy S20+"},
        {L"sm-g981b",                         L"Samsung Galaxy S20"},
        {L"sm-n986b",                         L"Samsung Galaxy Note 20 Ultra"},
        {L"sm-f946b",                         L"Samsung Galaxy Z Fold 5"},
        {L"sm-f731b",                         L"Samsung Galaxy Z Flip 5"},

        // =====================================================================
        // GOOGLE PIXEL
        // =====================================================================
        {L"pixel 9 pro xl",                  L"Google Pixel 9 Pro XL"},
        {L"pixel 9 pro",                     L"Google Pixel 9 Pro"},
        {L"pixel 9",                         L"Google Pixel 9"},
        {L"pixel 8 pro",                     L"Google Pixel 8 Pro"},
        {L"pixel 8",                         L"Google Pixel 8"},
        {L"pixel 7 pro",                     L"Google Pixel 7 Pro"},
        {L"pixel 7",                         L"Google Pixel 7"},
        {L"pixel 6 pro",                     L"Google Pixel 6 Pro"},
        {L"pixel 6",                         L"Google Pixel 6"},

        // =====================================================================
        // RICOH / PENTAX
        // =====================================================================
        {L"pentax k-3 mark iii",              L"Pentax K-3 Mark III"},
        {L"pentax k-1 mark ii",              L"Pentax K-1 Mark II"},
        {L"pentax k-1",                      L"Pentax K-1"},
        {L"ricoh gr iiix",                   L"Ricoh GR IIIx"},
        {L"ricoh gr iii",                    L"Ricoh GR III"},

        // =====================================================================
        // SIGMA
        // =====================================================================
        {L"sigma fp l",                       L"Sigma fp L"},
        {L"sigma fp",                         L"Sigma fp"},

        // =====================================================================
        // RED Cinema
        // =====================================================================
        {L"dsmc3",                            L"RED V-RAPTOR / V-RAPTOR XL (DSMC3)"},
        {L"dsmc2",                            L"RED DSMC2 Platform (Monstro/Helium/Gemini)"},

        // =====================================================================
        // BLACKMAGIC
        // =====================================================================
        {L"blackmagic pocket cinema camera 6k g2",  L"Blackmagic Pocket Cinema Camera 6K G2"},
        {L"blackmagic pocket cinema camera 6k pro", L"Blackmagic Pocket Cinema Camera 6K Pro"},
        {L"blackmagic pocket cinema camera 6k",     L"Blackmagic Pocket Cinema Camera 6K"},
        {L"blackmagic pocket cinema camera 4k",     L"Blackmagic Pocket Cinema Camera 4K"},
        {L"ursa mini pro 12k",               L"Blackmagic URSA Mini Pro 12K"},
        {L"ursa mini pro 4.6k g2",           L"Blackmagic URSA Mini Pro 4.6K G2"},

        // =====================================================================
        // INSTA360
        // =====================================================================
        {L"insta360 x4",                      L"Insta360 X4"},
        {L"insta360 x3",                      L"Insta360 X3"},
        {L"insta360 one rs",                  L"Insta360 ONE RS"},
        {L"insta360 ace pro",                 L"Insta360 Ace Pro"},
    };
    return db;
}

// Resolve a camera model string to its marketing name
// Returns the marketing name if found, otherwise returns the original model
inline std::wstring ResolveCameraModel(const std::wstring& exifModel) {
    const auto& db = GetCameraModelDB();
    std::wstring normalized = NormalizeModelString(exifModel);

    // Direct match
    auto it = db.find(normalized);
    if (it != db.end()) {
        return it->second;
    }

    // Try partial matching - check if any DB key is contained in the model string
    for (const auto& [key, value] : db) {
        if (normalized.find(key) != std::wstring::npos) {
            return value;
        }
    }

    // No match, return original
    return exifModel;
}

// ============================================================================
// LENS MODEL DATABASE
// Maps internal/EXIF lens strings to their full marketing names.
// ExifTool's MakerNotes often report lens IDs as numeric codes or short
// internal strings — this resolves them to recognizable names.
// ============================================================================
inline const std::unordered_map<std::wstring, std::wstring>& GetLensModelDB() {
    static const std::unordered_map<std::wstring, std::wstring> db = {
        // =====================================================================
        // CANON RF LENSES
        // =====================================================================
        {L"rf14-35mm f4 l is usm",                    L"Canon RF 14-35mm f/4L IS USM"},
        {L"rf15-35mm f2.8 l is usm",                  L"Canon RF 15-35mm f/2.8L IS USM"},
        {L"rf24-70mm f2.8 l is usm",                  L"Canon RF 24-70mm f/2.8L IS USM"},
        {L"rf24-105mm f4 l is usm",                   L"Canon RF 24-105mm f/4L IS USM"},
        {L"rf24-105mm f4-7.1 is stm",                 L"Canon RF 24-105mm f/4-7.1 IS STM"},
        {L"rf28-70mm f2 l usm",                       L"Canon RF 28-70mm f/2L USM"},
        {L"rf70-200mm f2.8 l is usm",                 L"Canon RF 70-200mm f/2.8L IS USM"},
        {L"rf70-200mm f4 l is usm",                   L"Canon RF 70-200mm f/4L IS USM"},
        {L"rf100-400mm f5.6-8 is usm",                L"Canon RF 100-400mm f/5.6-8 IS USM"},
        {L"rf100-500mm f4.5-7.1 l is usm",            L"Canon RF 100-500mm f/4.5-7.1L IS USM"},
        {L"rf200-800mm f6.3-9 is usm",                L"Canon RF 200-800mm f/6.3-9 IS USM"},
        {L"rf35mm f1.8 macro is stm",                 L"Canon RF 35mm f/1.8 Macro IS STM"},
        {L"rf50mm f1.2 l usm",                        L"Canon RF 50mm f/1.2L USM"},
        {L"rf50mm f1.8 stm",                          L"Canon RF 50mm f/1.8 STM"},
        {L"rf85mm f1.2 l usm",                        L"Canon RF 85mm f/1.2L USM"},
        {L"rf85mm f2 macro is stm",                   L"Canon RF 85mm f/2 Macro IS STM"},
        {L"rf100mm f2.8 l macro is usm",              L"Canon RF 100mm f/2.8L Macro IS USM"},
        {L"rf600mm f11 is stm",                       L"Canon RF 600mm f/11 IS STM"},
        {L"rf800mm f11 is stm",                       L"Canon RF 800mm f/11 IS STM"},
        {L"rf16mm f2.8 stm",                          L"Canon RF 16mm f/2.8 STM"},

        // =====================================================================
        // CANON EF LENSES (legacy, still widely used)
        // =====================================================================
        {L"ef24-70mm f2.8l ii usm",                   L"Canon EF 24-70mm f/2.8L II USM"},
        {L"ef24-70mm f2.8l usm",                      L"Canon EF 24-70mm f/2.8L USM"},
        {L"ef24-105mm f4l is ii usm",                 L"Canon EF 24-105mm f/4L IS II USM"},
        {L"ef24-105mm f4l is usm",                    L"Canon EF 24-105mm f/4L IS USM"},
        {L"ef70-200mm f2.8l is iii usm",              L"Canon EF 70-200mm f/2.8L IS III USM"},
        {L"ef70-200mm f2.8l is ii usm",               L"Canon EF 70-200mm f/2.8L IS II USM"},
        {L"ef70-200mm f4l is ii usm",                 L"Canon EF 70-200mm f/4L IS II USM"},
        {L"ef70-200mm f4l is usm",                    L"Canon EF 70-200mm f/4L IS USM"},
        {L"ef100-400mm f4.5-5.6l is ii usm",          L"Canon EF 100-400mm f/4.5-5.6L IS II USM"},
        {L"ef16-35mm f2.8l iii usm",                  L"Canon EF 16-35mm f/2.8L III USM"},
        {L"ef16-35mm f4l is usm",                     L"Canon EF 16-35mm f/4L IS USM"},
        {L"ef50mm f1.2l usm",                         L"Canon EF 50mm f/1.2L USM"},
        {L"ef50mm f1.4 usm",                          L"Canon EF 50mm f/1.4 USM"},
        {L"ef50mm f1.8 stm",                          L"Canon EF 50mm f/1.8 STM (Nifty Fifty)"},
        {L"ef50mm f1.8 ii",                           L"Canon EF 50mm f/1.8 II"},
        {L"ef85mm f1.4l is usm",                      L"Canon EF 85mm f/1.4L IS USM"},
        {L"ef85mm f1.8 usm",                          L"Canon EF 85mm f/1.8 USM"},
        {L"ef100mm f2.8l macro is usm",               L"Canon EF 100mm f/2.8L Macro IS USM"},
        {L"ef135mm f2l usm",                          L"Canon EF 135mm f/2L USM"},
        {L"ef400mm f2.8l is iii usm",                 L"Canon EF 400mm f/2.8L IS III USM"},
        {L"ef-s18-55mm f3.5-5.6 is stm",              L"Canon EF-S 18-55mm f/3.5-5.6 IS STM"},
        {L"ef-s18-135mm f3.5-5.6 is usm",             L"Canon EF-S 18-135mm f/3.5-5.6 IS USM"},
        {L"ef-s55-250mm f4-5.6 is stm",               L"Canon EF-S 55-250mm f/4-5.6 IS STM"},
        {L"ef-s10-18mm f4.5-5.6 is stm",              L"Canon EF-S 10-18mm f/4.5-5.6 IS STM"},

        // =====================================================================
        // NIKON Z LENSES (NIKKOR Z)
        // =====================================================================
        {L"nikkor z 14-24mm f/2.8 s",                 L"Nikon NIKKOR Z 14-24mm f/2.8 S"},
        {L"nikkor z 14-30mm f/4 s",                   L"Nikon NIKKOR Z 14-30mm f/4 S"},
        {L"nikkor z 24-70mm f/2.8 s",                 L"Nikon NIKKOR Z 24-70mm f/2.8 S"},
        {L"nikkor z 24-70mm f/4 s",                   L"Nikon NIKKOR Z 24-70mm f/4 S"},
        {L"nikkor z 24-120mm f/4 s",                  L"Nikon NIKKOR Z 24-120mm f/4 S"},
        {L"nikkor z 24-200mm f/4-6.3 vr",             L"Nikon NIKKOR Z 24-200mm f/4-6.3 VR"},
        {L"nikkor z 28-75mm f/2.8",                   L"Nikon NIKKOR Z 28-75mm f/2.8"},
        {L"nikkor z 70-200mm f/2.8 vr s",             L"Nikon NIKKOR Z 70-200mm f/2.8 VR S"},
        {L"nikkor z 100-400mm f/4.5-5.6 vr s",        L"Nikon NIKKOR Z 100-400mm f/4.5-5.6 VR S"},
        {L"nikkor z 180-600mm f/5.6-6.3 vr",          L"Nikon NIKKOR Z 180-600mm f/5.6-6.3 VR"},
        {L"nikkor z 26mm f/2.8",                      L"Nikon NIKKOR Z 26mm f/2.8"},
        {L"nikkor z 28mm f/2.8",                      L"Nikon NIKKOR Z 28mm f/2.8"},
        {L"nikkor z 35mm f/1.4",                      L"Nikon NIKKOR Z 35mm f/1.4"},
        {L"nikkor z 40mm f/2",                        L"Nikon NIKKOR Z 40mm f/2"},
        {L"nikkor z 50mm f/1.2 s",                    L"Nikon NIKKOR Z 50mm f/1.2 S"},
        {L"nikkor z 50mm f/1.8 s",                    L"Nikon NIKKOR Z 50mm f/1.8 S"},
        {L"nikkor z 85mm f/1.2 s",                    L"Nikon NIKKOR Z 85mm f/1.2 S"},
        {L"nikkor z 85mm f/1.8 s",                    L"Nikon NIKKOR Z 85mm f/1.8 S"},
        {L"nikkor z 135mm f/1.8 s plena",             L"Nikon NIKKOR Z 135mm f/1.8 S Plena"},
        {L"nikkor z mc 50mm f/2.8",                   L"Nikon NIKKOR Z MC 50mm f/2.8 Macro"},
        {L"nikkor z mc 105mm f/2.8 vr s",             L"Nikon NIKKOR Z MC 105mm f/2.8 VR S Macro"},
        {L"nikkor z dx 16-50mm f/3.5-6.3 vr",         L"Nikon NIKKOR Z DX 16-50mm f/3.5-6.3 VR"},
        {L"nikkor z dx 50-250mm f/4.5-6.3 vr",        L"Nikon NIKKOR Z DX 50-250mm f/4.5-6.3 VR"},

        // =====================================================================
        // NIKON F LENSES (legacy AF-S/AF-P)
        // =====================================================================
        {L"af-s nikkor 24-70mm f/2.8e ed vr",         L"Nikon AF-S NIKKOR 24-70mm f/2.8E ED VR"},
        {L"af-s nikkor 24-70mm f/2.8g ed",            L"Nikon AF-S NIKKOR 24-70mm f/2.8G ED"},
        {L"af-s nikkor 70-200mm f/2.8e fl ed vr",     L"Nikon AF-S NIKKOR 70-200mm f/2.8E FL ED VR"},
        {L"af-s nikkor 14-24mm f/2.8g ed",            L"Nikon AF-S NIKKOR 14-24mm f/2.8G ED"},
        {L"af-s nikkor 50mm f/1.4g",                  L"Nikon AF-S NIKKOR 50mm f/1.4G"},
        {L"af-s nikkor 50mm f/1.8g",                  L"Nikon AF-S NIKKOR 50mm f/1.8G"},
        {L"af-s nikkor 85mm f/1.4g",                  L"Nikon AF-S NIKKOR 85mm f/1.4G"},
        {L"af-s nikkor 105mm f/1.4e ed",              L"Nikon AF-S NIKKOR 105mm f/1.4E ED"},
        {L"af-p dx nikkor 18-55mm f/3.5-5.6g vr",     L"Nikon AF-P DX NIKKOR 18-55mm f/3.5-5.6G VR"},
        {L"af-p dx nikkor 70-300mm f/4.5-6.3g ed vr",  L"Nikon AF-P DX NIKKOR 70-300mm f/4.5-6.3G ED VR"},

        // =====================================================================
        // SONY FE (Full-Frame E-mount)
        // =====================================================================
        {L"fe 12-24mm f2.8 gm",                      L"Sony FE 12-24mm f/2.8 GM"},
        {L"fe 16-35mm f2.8 gm",                      L"Sony FE 16-35mm f/2.8 GM"},
        {L"fe 16-35mm f2.8 gm ii",                   L"Sony FE 16-35mm f/2.8 GM II"},
        {L"fe 24-70mm f2.8 gm",                      L"Sony FE 24-70mm f/2.8 GM"},
        {L"fe 24-70mm f2.8 gm ii",                   L"Sony FE 24-70mm f/2.8 GM II"},
        {L"fe 24-105mm f4 g oss",                    L"Sony FE 24-105mm f/4 G OSS"},
        {L"fe 70-200mm f2.8 gm oss",                 L"Sony FE 70-200mm f/2.8 GM OSS"},
        {L"fe 70-200mm f2.8 gm oss ii",              L"Sony FE 70-200mm f/2.8 GM OSS II"},
        {L"fe 100-400mm f4.5-5.6 gm oss",            L"Sony FE 100-400mm f/4.5-5.6 GM OSS"},
        {L"fe 200-600mm f5.6-6.3 g oss",             L"Sony FE 200-600mm f/5.6-6.3 G OSS"},
        {L"fe 20mm f1.8 g",                          L"Sony FE 20mm f/1.8 G"},
        {L"fe 24mm f1.4 gm",                         L"Sony FE 24mm f/1.4 GM"},
        {L"fe 35mm f1.4 gm",                         L"Sony FE 35mm f/1.4 GM"},
        {L"fe 50mm f1.2 gm",                         L"Sony FE 50mm f/1.2 GM"},
        {L"fe 50mm f1.4 gm",                         L"Sony FE 50mm f/1.4 GM"},
        {L"fe 50mm f1.8",                            L"Sony FE 50mm f/1.8"},
        {L"fe 85mm f1.4 gm",                         L"Sony FE 85mm f/1.4 GM"},
        {L"fe 85mm f1.8",                            L"Sony FE 85mm f/1.8"},
        {L"fe 135mm f1.8 gm",                        L"Sony FE 135mm f/1.8 GM"},
        {L"fe 90mm f2.8 macro g oss",                L"Sony FE 90mm f/2.8 Macro G OSS"},
        {L"fe 28-60mm f4-5.6",                       L"Sony FE 28-60mm f/4-5.6"},
        {L"fe 28-70mm f3.5-5.6 oss",                 L"Sony FE 28-70mm f/3.5-5.6 OSS (Kit Lens)"},
        // Sony E (APS-C)
        {L"e 16-55mm f2.8 g",                        L"Sony E 16-55mm f/2.8 G"},
        {L"e 18-135mm f3.5-5.6 oss",                 L"Sony E 18-135mm f/3.5-5.6 OSS"},
        {L"e 10-18mm f4 oss",                        L"Sony E 10-18mm f/4 OSS"},
        {L"e 70-350mm f4.5-6.3 g oss",               L"Sony E 70-350mm f/4.5-6.3 G OSS"},
        {L"e 35mm f1.8 oss",                         L"Sony E 35mm f/1.8 OSS"},
        {L"e pz 16-50mm f3.5-5.6 oss",               L"Sony E PZ 16-50mm f/3.5-5.6 OSS (Kit Lens)"},

        // =====================================================================
        // SIGMA (Contemporary / Art / Sports)
        // =====================================================================
        // Sigma Art Primes
        {L"14mm f1.8 dg hsm | art 017",              L"Sigma 14mm f/1.8 DG HSM Art"},
        {L"20mm f1.4 dg hsm | art 015",              L"Sigma 20mm f/1.4 DG HSM Art"},
        {L"24mm f1.4 dg hsm | art 015",              L"Sigma 24mm f/1.4 DG HSM Art"},
        {L"28mm f1.4 dg hsm | art 019",              L"Sigma 28mm f/1.4 DG HSM Art"},
        {L"35mm f1.4 dg hsm | art 012",              L"Sigma 35mm f/1.4 DG HSM Art"},
        {L"35mm f1.4 dg dn | art 021",               L"Sigma 35mm f/1.4 DG DN Art"},
        {L"40mm f1.4 dg hsm | art 018",              L"Sigma 40mm f/1.4 DG HSM Art"},
        {L"50mm f1.4 dg hsm | art 014",              L"Sigma 50mm f/1.4 DG HSM Art"},
        {L"50mm f1.4 dg dn | art 023",               L"Sigma 50mm f/1.4 DG DN Art"},
        {L"85mm f1.4 dg hsm | art 016",              L"Sigma 85mm f/1.4 DG HSM Art"},
        {L"85mm f1.4 dg dn | art 020",               L"Sigma 85mm f/1.4 DG DN Art"},
        {L"105mm f1.4 dg hsm | art 018",             L"Sigma 105mm f/1.4 DG HSM Art (Bokeh Master)"},
        {L"135mm f1.8 dg hsm | art 017",             L"Sigma 135mm f/1.8 DG HSM Art"},
        // Sigma Art Zooms
        {L"14-24mm f2.8 dg dn | art 019",            L"Sigma 14-24mm f/2.8 DG DN Art"},
        {L"24-70mm f2.8 dg dn | art 019",            L"Sigma 24-70mm f/2.8 DG DN Art"},
        {L"24-70mm f2.8 dg os hsm | art 017",        L"Sigma 24-70mm f/2.8 DG OS HSM Art"},
        {L"18-35mm f1.8 dc hsm | art 013",           L"Sigma 18-35mm f/1.8 DC HSM Art"},
        {L"50-100mm f1.8 dc hsm | art 016",          L"Sigma 50-100mm f/1.8 DC HSM Art"},
        // Sigma Sports
        {L"70-200mm f2.8 dg os hsm | sports 018",    L"Sigma 70-200mm f/2.8 DG OS HSM Sports"},
        {L"100-400mm f5-6.3 dg dn os | contemporary 020", L"Sigma 100-400mm f/5-6.3 DG DN OS Contemporary"},
        {L"100-400mm f5-6.3 dg os hsm | contemporary 017", L"Sigma 100-400mm f/5-6.3 DG OS HSM Contemporary"},
        {L"150-600mm f5-6.3 dg os hsm | sports 014", L"Sigma 150-600mm f/5-6.3 DG OS HSM Sports"},
        {L"150-600mm f5-6.3 dg os hsm | contemporary 015", L"Sigma 150-600mm f/5-6.3 DG OS HSM Contemporary"},
        {L"60-600mm f4.5-6.3 dg dn os | sports 023", L"Sigma 60-600mm f/4.5-6.3 DG DN OS Sports"},

        // =====================================================================
        // TAMRON
        // =====================================================================
        {L"17-28mm f/2.8 di iii rxd (a046)",          L"Tamron 17-28mm f/2.8 Di III RXD"},
        {L"28-75mm f/2.8 di iii rxd (a036)",          L"Tamron 28-75mm f/2.8 Di III RXD"},
        {L"28-75mm f/2.8 di iii vxd g2 (a063)",       L"Tamron 28-75mm f/2.8 Di III VXD G2"},
        {L"28-200mm f/2.8-5.6 di iii rxd (a071)",     L"Tamron 28-200mm f/2.8-5.6 Di III RXD"},
        {L"35-150mm f/2-2.8 di iii vxd (a058)",       L"Tamron 35-150mm f/2-2.8 Di III VXD"},
        {L"70-180mm f/2.8 di iii vxd (a056)",         L"Tamron 70-180mm f/2.8 Di III VXD"},
        {L"70-180mm f/2.8 di iii vxd g2 (a065)",      L"Tamron 70-180mm f/2.8 Di III VXD G2"},
        {L"70-300mm f/4.5-6.3 di iii rxd (a047)",     L"Tamron 70-300mm f/4.5-6.3 Di III RXD"},
        {L"150-500mm f/5-6.7 di iii vc vxd (a057)",   L"Tamron 150-500mm f/5-6.7 Di III VC VXD"},
        {L"50-400mm f/4.5-6.3 di iii vc vxd (a067)",  L"Tamron 50-400mm f/4.5-6.3 Di III VC VXD"},
        {L"11-20mm f/2.8 di iii-a rxd (b060)",        L"Tamron 11-20mm f/2.8 Di III-A RXD"},
        {L"17-70mm f/2.8 di iii-a vc rxd (b070)",     L"Tamron 17-70mm f/2.8 Di III-A VC RXD"},
        {L"sp 24-70mm f/2.8 di vc usd g2 (a032)",     L"Tamron SP 24-70mm f/2.8 Di VC USD G2"},
        {L"sp 70-200mm f/2.8 di vc usd g2 (a025)",    L"Tamron SP 70-200mm f/2.8 Di VC USD G2"},
        {L"sp 15-30mm f/2.8 di vc usd g2 (a041)",     L"Tamron SP 15-30mm f/2.8 Di VC USD G2"},
        {L"sp 150-600mm f/5-6.3 di vc usd g2 (a022)", L"Tamron SP 150-600mm f/5-6.3 Di VC USD G2"},

        // =====================================================================
        // FUJIFILM XF / XC LENSES
        // =====================================================================
        {L"xf16-55mmf2.8 r lm wr",                   L"Fujifilm XF 16-55mm f/2.8 R LM WR"},
        {L"xf16-80mmf4 r ois wr",                    L"Fujifilm XF 16-80mm f/4 R OIS WR"},
        {L"xf18-55mmf2.8-4 r lm ois",                L"Fujifilm XF 18-55mm f/2.8-4 R LM OIS"},
        {L"xf50-140mmf2.8 r lm ois wr",              L"Fujifilm XF 50-140mm f/2.8 R LM OIS WR"},
        {L"xf55-200mmf3.5-4.8 r lm ois",             L"Fujifilm XF 55-200mm f/3.5-4.8 R LM OIS"},
        {L"xf100-400mmf4.5-5.6 r lm ois wr",         L"Fujifilm XF 100-400mm f/4.5-5.6 R LM OIS WR"},
        {L"xf150-600mmf5.6-8 r lm ois wr",           L"Fujifilm XF 150-600mm f/5.6-8 R LM OIS WR"},
        {L"xf23mmf1.4 r",                            L"Fujifilm XF 23mm f/1.4 R"},
        {L"xf23mmf1.4 r lm wr",                      L"Fujifilm XF 23mm f/1.4 R LM WR (mk II)"},
        {L"xf23mmf2 r wr",                           L"Fujifilm XF 23mm f/2 R WR"},
        {L"xf33mmf1.4 r lm wr",                      L"Fujifilm XF 33mm f/1.4 R LM WR"},
        {L"xf35mmf1.4 r",                            L"Fujifilm XF 35mm f/1.4 R"},
        {L"xf35mmf2 r wr",                           L"Fujifilm XF 35mm f/2 R WR"},
        {L"xf56mmf1.2 r",                            L"Fujifilm XF 56mm f/1.2 R"},
        {L"xf56mmf1.2 r wr",                         L"Fujifilm XF 56mm f/1.2 R WR (mk II)"},
        {L"xf90mmf2 r lm wr",                        L"Fujifilm XF 90mm f/2 R LM WR"},
        {L"xc15-45mmf3.5-5.6 ois pz",                L"Fujifilm XC 15-45mm f/3.5-5.6 OIS PZ (Kit Lens)"},
        {L"xc50-230mmf4.5-6.7 ois ii",               L"Fujifilm XC 50-230mm f/4.5-6.7 OIS II"},

        // =====================================================================
        // PANASONIC LUMIX LENSES
        // =====================================================================
        {L"lumix s 24-105mm f4 macro o.i.s.",         L"Panasonic Lumix S 24-105mm f/4 Macro OIS"},
        {L"lumix s pro 50mm f1.4",                    L"Panasonic Lumix S PRO 50mm f/1.4"},
        {L"lumix s pro 70-200mm f2.8 o.i.s.",         L"Panasonic Lumix S PRO 70-200mm f/2.8 OIS"},
        {L"lumix s 50mm f1.8",                        L"Panasonic Lumix S 50mm f/1.8"},
        {L"lumix s 85mm f1.8",                        L"Panasonic Lumix S 85mm f/1.8"},
        {L"lumix s 20-60mm f3.5-5.6",                 L"Panasonic Lumix S 20-60mm f/3.5-5.6"},
        {L"leica dg vario-elmarit 12-60mm f2.8-4.0",  L"Panasonic Leica DG Vario-Elmarit 12-60mm f/2.8-4.0"},
        {L"lumix g vario 12-60mm f3.5-5.6",           L"Panasonic Lumix G Vario 12-60mm f/3.5-5.6"},
        {L"lumix g 25mm f1.7",                        L"Panasonic Lumix G 25mm f/1.7"},
        {L"leica dg nocticron 42.5mm f1.2",           L"Panasonic Leica DG Nocticron 42.5mm f/1.2"},
    };
    return db;
}

// Resolve a lens model string to its full marketing name
inline std::wstring ResolveLensModel(const std::wstring& exifLens) {
    const auto& db = GetLensModelDB();
    std::wstring normalized = NormalizeModelString(exifLens);

    // Direct match
    auto it = db.find(normalized);
    if (it != db.end()) {
        return it->second;
    }

    // Partial match (DB key contained in lens string)
    for (const auto& [key, value] : db) {
        if (normalized.find(key) != std::wstring::npos) {
            return value;
        }
    }

    // Reverse partial match (lens string contained in DB key)
    for (const auto& [key, value] : db) {
        if (key.find(normalized) != std::wstring::npos && normalized.length() > 8) {
            return value;
        }
    }

    return exifLens;
}
