use once_cell::sync::Lazy;
use std::collections::HashMap;

static CAMERA_DB: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Canon EOS DSLRs
    m.insert("canon eos-1d x mark iii", "Canon EOS-1D X Mark III");
    m.insert("canon eos-1d x mark ii", "Canon EOS-1D X Mark II");
    m.insert("canon eos-1d x", "Canon EOS-1D X");
    m.insert("canon eos-1d mark iv", "Canon EOS-1D Mark IV");
    m.insert("canon eos-1ds mark iii", "Canon EOS-1Ds Mark III");
    m.insert("canon eos 5d mark iv", "Canon EOS 5D Mark IV");
    m.insert("canon eos 5d mark iii", "Canon EOS 5D Mark III");
    m.insert("canon eos 5d mark ii", "Canon EOS 5D Mark II");
    m.insert("canon eos 5d", "Canon EOS 5D");
    m.insert("canon eos 5ds r", "Canon EOS 5DS R");
    m.insert("canon eos 5ds", "Canon EOS 5DS");
    m.insert("canon eos 6d mark ii", "Canon EOS 6D Mark II");
    m.insert("canon eos 6d", "Canon EOS 6D");
    m.insert("canon eos 7d mark ii", "Canon EOS 7D Mark II");
    m.insert("canon eos 7d", "Canon EOS 7D");
    m.insert("canon eos 90d", "Canon EOS 90D");
    m.insert("canon eos 80d", "Canon EOS 80D");
    m.insert("canon eos 77d", "Canon EOS 77D");
    m.insert("canon eos 70d", "Canon EOS 70D");
    m.insert("canon eos 60d", "Canon EOS 60D");
    m.insert("canon eos 50d", "Canon EOS 50D");
    m.insert("canon eos 40d", "Canon EOS 40D");
    m.insert("canon eos 30d", "Canon EOS 30D");
    m.insert("canon eos 2000d", "Canon EOS 2000D / Rebel T7 / Kiss X90");
    m.insert("canon eos rebel t7", "Canon EOS Rebel T7 / EOS 2000D");
    m.insert("canon eos 1300d", "Canon EOS 1300D / Rebel T6 / Kiss X80");
    m.insert("canon eos 1200d", "Canon EOS 1200D / Rebel T5 / Kiss X70");
    m.insert("canon eos 1100d", "Canon EOS 1100D / Rebel T3 / Kiss X50");
    m.insert("canon eos 4000d", "Canon EOS 4000D / Rebel T100 / Kiss X90");
    m.insert("canon eos 850d", "Canon EOS 850D / Rebel T8i / Kiss X10i");
    m.insert("canon eos 800d", "Canon EOS 800D / Rebel T7i / Kiss X9i");
    m.insert("canon eos 750d", "Canon EOS 750D / Rebel T6i / Kiss X8i");
    m.insert("canon eos 700d", "Canon EOS 700D / Rebel T5i / Kiss X7i");
    m.insert("canon eos 650d", "Canon EOS 650D / Rebel T4i / Kiss X6i");
    m.insert("canon eos 600d", "Canon EOS 600D / Rebel T3i / Kiss X5");
    m.insert("canon eos 550d", "Canon EOS 550D / Rebel T2i / Kiss X4");
    m.insert("canon eos 500d", "Canon EOS 500D / Rebel T1i / Kiss X3");
    m.insert("canon eos 450d", "Canon EOS 450D / Rebel XSi / Kiss X2");
    m.insert("canon eos 250d", "Canon EOS 250D / Rebel SL3 / Kiss X10");
    m.insert("canon eos 200d", "Canon EOS 200D / Rebel SL2 / Kiss X9");
    // Canon EOS R & M
    m.insert("canon eos r1", "Canon EOS R1");
    m.insert("canon eos r3", "Canon EOS R3");
    m.insert("canon eos r5 mark ii", "Canon EOS R5 Mark II");
    m.insert("canon eos r5 c", "Canon EOS R5 C");
    m.insert("canon eos r5", "Canon EOS R5");
    m.insert("canon eos r6 mark ii", "Canon EOS R6 Mark II");
    m.insert("canon eos r6", "Canon EOS R6");
    m.insert("canon eos r7", "Canon EOS R7");
    m.insert("canon eos r8", "Canon EOS R8");
    m.insert("canon eos r10", "Canon EOS R10");
    m.insert("canon eos r50", "Canon EOS R50");
    m.insert("canon eos r100", "Canon EOS R100");
    m.insert("canon eos rp", "Canon EOS RP");
    m.insert("canon eos r", "Canon EOS R");
    m.insert("canon eos m50 mark ii", "Canon EOS M50 Mark II / Kiss M2");
    m.insert("canon eos m50", "Canon EOS M50 / Kiss M");
    m.insert("canon eos m6 mark ii", "Canon EOS M6 Mark II");
    m.insert("canon eos m6", "Canon EOS M6");
    m.insert("canon eos m5", "Canon EOS M5");
    m.insert("canon eos m200", "Canon EOS M200");
    m.insert("canon eos m100", "Canon EOS M100");
    // Nikon Z
    m.insert("nikon z 9", "Nikon Z 9");
    m.insert("z 9", "Nikon Z 9");
    m.insert("nikon z 8", "Nikon Z 8");
    m.insert("z 8", "Nikon Z 8");
    m.insert("nikon z 7ii", "Nikon Z 7II");
    m.insert("nikon z 7 ii", "Nikon Z 7II");
    m.insert("z 7ii", "Nikon Z 7II");
    m.insert("nikon z 7", "Nikon Z 7");
    m.insert("z 7", "Nikon Z 7");
    m.insert("nikon z 6iii", "Nikon Z 6III");
    m.insert("nikon z 6 iii", "Nikon Z 6III");
    m.insert("z 6iii", "Nikon Z 6III");
    m.insert("nikon z 6ii", "Nikon Z 6II");
    m.insert("nikon z 6 ii", "Nikon Z 6II");
    m.insert("z 6ii", "Nikon Z 6II");
    m.insert("nikon z 6", "Nikon Z 6");
    m.insert("z 6", "Nikon Z 6");
    m.insert("nikon z 5", "Nikon Z 5");
    m.insert("z 5", "Nikon Z 5");
    m.insert("nikon z 50", "Nikon Z 50");
    m.insert("z 50", "Nikon Z 50");
    m.insert("nikon z 30", "Nikon Z 30");
    m.insert("z 30", "Nikon Z 30");
    m.insert("nikon z fc", "Nikon Z fc");
    m.insert("nikon z f", "Nikon Z f");
    // Nikon DSLRs
    m.insert("nikon d6", "Nikon D6");
    m.insert("nikon d5", "Nikon D5");
    m.insert("nikon d4s", "Nikon D4S");
    m.insert("nikon d4", "Nikon D4");
    m.insert("nikon d3s", "Nikon D3S");
    m.insert("nikon d3x", "Nikon D3X");
    m.insert("nikon d3", "Nikon D3");
    m.insert("nikon d850", "Nikon D850");
    m.insert("nikon d810a", "Nikon D810A");
    m.insert("nikon d810", "Nikon D810");
    m.insert("nikon d800e", "Nikon D800E");
    m.insert("nikon d800", "Nikon D800");
    m.insert("nikon d780", "Nikon D780");
    m.insert("nikon d750", "Nikon D750");
    m.insert("nikon d700", "Nikon D700");
    m.insert("nikon d610", "Nikon D610");
    m.insert("nikon d600", "Nikon D600");
    m.insert("nikon d500", "Nikon D500");
    m.insert("nikon d7500", "Nikon D7500");
    m.insert("nikon d7200", "Nikon D7200");
    m.insert("nikon d7100", "Nikon D7100");
    m.insert("nikon d7000", "Nikon D7000");
    m.insert("nikon d5600", "Nikon D5600");
    m.insert("nikon d5500", "Nikon D5500");
    m.insert("nikon d5300", "Nikon D5300");
    m.insert("nikon d5200", "Nikon D5200");
    m.insert("nikon d5100", "Nikon D5100");
    m.insert("nikon d5000", "Nikon D5000");
    m.insert("nikon d3500", "Nikon D3500");
    m.insert("nikon d3400", "Nikon D3400");
    m.insert("nikon d3300", "Nikon D3300");
    m.insert("nikon d3200", "Nikon D3200");
    m.insert("nikon d3100", "Nikon D3100");
    m.insert("nikon d3000", "Nikon D3000");
    m.insert("nikon d300s", "Nikon D300S");
    m.insert("nikon d300", "Nikon D300");
    m.insert("nikon d90", "Nikon D90");
    // Sony Mirrorless & Compacts
    m.insert("ilce-1", "Sony Alpha 1 (A1)");
    m.insert("ilce-9m3", "Sony Alpha 9 III (A9 III)");
    m.insert("ilce-9m2", "Sony Alpha 9 II (A9 II)");
    m.insert("ilce-9", "Sony Alpha 9 (A9)");
    m.insert("ilce-7rm5", "Sony Alpha 7R V (A7R V)");
    m.insert("ilce-7rm4", "Sony Alpha 7R IV (A7R IV)");
    m.insert("ilce-7rm3", "Sony Alpha 7R III (A7R III)");
    m.insert("ilce-7rm2", "Sony Alpha 7R II (A7R II)");
    m.insert("ilce-7r", "Sony Alpha 7R (A7R)");
    m.insert("ilce-7sm3", "Sony Alpha 7S III (A7S III)");
    m.insert("ilce-7sm2", "Sony Alpha 7S II (A7S II)");
    m.insert("ilce-7s", "Sony Alpha 7S (A7S)");
    m.insert("ilce-7m4", "Sony Alpha 7 IV (A7 IV)");
    m.insert("ilce-7m3", "Sony Alpha 7 III (A7 III)");
    m.insert("ilce-7m2", "Sony Alpha 7 II (A7 II)");
    m.insert("ilce-7", "Sony Alpha 7 (A7)");
    m.insert("ilce-7c2", "Sony Alpha 7C II (A7C II)");
    m.insert("ilce-7cr", "Sony Alpha 7CR (A7CR)");
    m.insert("ilce-7c", "Sony Alpha 7C (A7C)");
    m.insert("ilce-6700", "Sony Alpha 6700 (A6700)");
    m.insert("ilce-6600", "Sony Alpha 6600 (A6600)");
    m.insert("ilce-6500", "Sony Alpha 6500 (A6500)");
    m.insert("ilce-6400", "Sony Alpha 6400 (A6400)");
    m.insert("ilce-6300", "Sony Alpha 6300 (A6300)");
    m.insert("ilce-6100", "Sony Alpha 6100 (A6100)");
    m.insert("ilce-6000", "Sony Alpha 6000 (A6000)");
    m.insert("ilme-fx30", "Sony FX30");
    m.insert("ilme-fx3", "Sony FX3");
    m.insert("zv-e10 ii", "Sony ZV-E10 II");
    m.insert("zv-e10", "Sony ZV-E10");
    m.insert("zv-e1", "Sony ZV-E1");
    m.insert("zv-1 ii", "Sony ZV-1 II");
    m.insert("zv-1", "Sony ZV-1");
    m.insert("zv-1f", "Sony ZV-1F");
    m.insert("dsc-rx100m7", "Sony RX100 VII");
    m.insert("dsc-rx100m6", "Sony RX100 VI");
    m.insert("dsc-rx100m5a", "Sony RX100 V (A)");
    m.insert("dsc-rx100m5", "Sony RX100 V");
    m.insert("dsc-rx100m4", "Sony RX100 IV");
    m.insert("dsc-rx100m3", "Sony RX100 III");
    m.insert("dsc-rx100m2", "Sony RX100 II");
    m.insert("dsc-rx100", "Sony RX100");
    m.insert("dsc-rx10m4", "Sony RX10 IV");
    m.insert("dsc-rx10m3", "Sony RX10 III");
    m.insert("dsc-rx10m2", "Sony RX10 II");
    m.insert("dsc-rx10", "Sony RX10");
    m.insert("dsc-rx1rm2", "Sony RX1R II");
    m.insert("dsc-rx1r", "Sony RX1R");
    m.insert("dsc-rx1", "Sony RX1");
    m.insert("slt-a99m2", "Sony Alpha 99 II");
    m.insert("slt-a99v", "Sony Alpha 99");
    m.insert("slt-a77v", "Sony Alpha 77");
    // Fujifilm
    m.insert("x-t50", "Fujifilm X-T50");
    m.insert("x-t5", "Fujifilm X-T5");
    m.insert("x-t4", "Fujifilm X-T4");
    m.insert("x-t3", "Fujifilm X-T3");
    m.insert("x-t2", "Fujifilm X-T2");
    m.insert("x-t1", "Fujifilm X-T1");
    m.insert("x-t30 ii", "Fujifilm X-T30 II");
    m.insert("x-t30", "Fujifilm X-T30");
    m.insert("x-t20", "Fujifilm X-T20");
    m.insert("x-t10", "Fujifilm X-T10");
    m.insert("x-h2s", "Fujifilm X-H2S");
    m.insert("x-h2", "Fujifilm X-H2");
    m.insert("x-s20", "Fujifilm X-S20");
    m.insert("x-s10", "Fujifilm X-S10");
    m.insert("x-pro3", "Fujifilm X-Pro3");
    m.insert("x-pro2", "Fujifilm X-Pro2");
    m.insert("x-pro1", "Fujifilm X-Pro1");
    m.insert("x-e4", "Fujifilm X-E4");
    m.insert("x-e3", "Fujifilm X-E3");
    m.insert("x-e2s", "Fujifilm X-E2S");
    m.insert("x-e2", "Fujifilm X-E2");
    m.insert("x-e1", "Fujifilm X-E1");
    m.insert("x100vi", "Fujifilm X100VI");
    m.insert("x100v", "Fujifilm X100V");
    m.insert("x100f", "Fujifilm X100F");
    m.insert("x100t", "Fujifilm X100T");
    m.insert("x100s", "Fujifilm X100S");
    m.insert("x100", "Fujifilm X100");
    m.insert("gfx100s ii", "Fujifilm GFX100S II");
    m.insert("gfx 100 ii", "Fujifilm GFX 100 II");
    m.insert("gfx 100s", "Fujifilm GFX 100S");
    m.insert("gfx 100", "Fujifilm GFX 100");
    m.insert("gfx 50s ii", "Fujifilm GFX 50S II");
    m.insert("gfx 50r", "Fujifilm GFX 50R");
    // Panasonic
    m.insert("dc-s9", "Panasonic Lumix S9");
    m.insert("dc-s5m2", "Panasonic Lumix S5 II");
    m.insert("dc-s5", "Panasonic Lumix S5");
    m.insert("dc-s1h", "Panasonic Lumix S1H");
    m.insert("dc-s1r", "Panasonic Lumix S1R");
    m.insert("dc-s1", "Panasonic Lumix S1");
    m.insert("dc-gh6", "Panasonic Lumix GH6");
    m.insert("dc-gh5m2", "Panasonic Lumix GH5 II");
    m.insert("dc-gh5", "Panasonic Lumix GH5");
    m.insert("dmc-gh5s", "Panasonic Lumix GH5S");
    m.insert("dmc-gh5", "Panasonic Lumix GH5");
    m.insert("dmc-gh4", "Panasonic Lumix GH4");
    m.insert("dmc-gh3", "Panasonic Lumix GH3");
    m.insert("dc-g9m2", "Panasonic Lumix G9 II");
    m.insert("dc-g9", "Panasonic Lumix G9");
    m.insert("dmc-g85", "Panasonic Lumix G85");
    m.insert("dmc-g7", "Panasonic Lumix G7");
    m.insert("dmc-gx85", "Panasonic Lumix GX85");
    m.insert("dmc-gx8", "Panasonic Lumix GX8");
    m.insert("dmc-gx7", "Panasonic Lumix GX7");
    m.insert("dc-gx9", "Panasonic Lumix GX9");
    m.insert("dmc-lx100", "Panasonic Lumix LX100");
    m.insert("dc-lx100m2", "Panasonic Lumix LX100 II");
    m.insert("dmc-fz1000", "Panasonic Lumix FZ1000");
    // Olympus / OM System
    m.insert("om-1 mark ii", "OM System OM-1 Mark II");
    m.insert("om-1", "OM System OM-1");
    m.insert("om-5", "OM System OM-5");
    m.insert("e-m1 mark iii", "Olympus OM-D E-M1 Mark III");
    m.insert("e-m1 mark ii", "Olympus OM-D E-M1 Mark II");
    m.insert("e-m1", "Olympus OM-D E-M1");
    m.insert("e-m1x", "Olympus OM-D E-M1X");
    m.insert("e-m5 mark iii", "Olympus OM-D E-M5 Mark III");
    m.insert("e-m5 mark ii", "Olympus OM-D E-M5 Mark II");
    m.insert("e-m5", "Olympus OM-D E-M5");
    m.insert("e-m10 mark iv", "Olympus OM-D E-M10 Mark IV");
    m.insert("e-m10 mark iii", "Olympus OM-D E-M10 Mark III");
    m.insert("e-m10 mark ii", "Olympus OM-D E-M10 Mark II");
    m.insert("e-m10", "Olympus OM-D E-M10");
    m.insert("pen-f", "Olympus PEN-F");
    m.insert("e-p7", "Olympus PEN E-P7");
    m.insert("e-pl10", "Olympus PEN E-PL10");
    m.insert("e-pl9", "Olympus PEN E-PL9");
    m.insert("tg-7", "Olympus Tough TG-7");
    m.insert("tg-6", "Olympus Tough TG-6");
    m.insert("tg-5", "Olympus Tough TG-5");
    // Leica
    m.insert("leica sl3", "Leica SL3");
    m.insert("leica sl2-s", "Leica SL2-S");
    m.insert("leica sl2", "Leica SL2");
    m.insert("leica sl", "Leica SL (Typ 601)");
    m.insert("leica q3", "Leica Q3");
    m.insert("leica q2", "Leica Q2");
    m.insert("leica q", "Leica Q (Typ 116)");
    m.insert("leica m11-p", "Leica M11-P");
    m.insert("leica m11 monochrom", "Leica M11 Monochrom");
    m.insert("leica m11", "Leica M11");
    m.insert("leica m10-r", "Leica M10-R");
    m.insert("leica m10-p", "Leica M10-P");
    m.insert("leica m10", "Leica M10");
    m.insert("leica m9", "Leica M9");
    m.insert("leica m8", "Leica M8");
    m.insert("leica cl", "Leica CL");
    m.insert("leica tl2", "Leica TL2");
    m.insert("leica d-lux 8", "Leica D-Lux 8");
    m.insert("leica d-lux 7", "Leica D-Lux 7");
    // Hasselblad
    m.insert("hasselblad x2d 100c", "Hasselblad X2D 100C");
    m.insert("l2d-20c", "DJI Mavic 3 (Hasselblad L2D-20c)");
    // DJI
    m.insert("fc3582", "DJI Mini 4 Pro");
    m.insert("fc3411", "DJI Mini 3 Pro");
    m.insert("fc3170", "DJI Mavic 3");
    m.insert("fc7303", "DJI Air 3");
    m.insert("fc3541", "DJI Air 2S");
    m.insert("fc220", "DJI Mavic Pro");
    m.insert("fc6310", "DJI Phantom 4 Pro");
    m.insert("mavic 3 pro", "DJI Mavic 3 Pro");
    m.insert("mavic 3 classic", "DJI Mavic 3 Classic");
    m.insert("mavic 3", "DJI Mavic 3");
    m.insert("dji mavic 3", "DJI Mavic 3");
    m.insert("air 3s", "DJI Air 3S");
    m.insert("air 3", "DJI Air 3");
    m.insert("air 2s", "DJI Air 2S");
    m.insert("air 2", "DJI Air 2");
    m.insert("mini 4 pro", "DJI Mini 4 Pro");
    m.insert("mini 3 pro", "DJI Mini 3 Pro");
    m.insert("mini 4", "DJI Mini 4");
    m.insert("mini 3", "DJI Mini 3");
    m.insert("mini 2", "DJI Mini 2");
    m.insert("inspire 3", "DJI Inspire 3");
    m.insert("inspire 2", "DJI Inspire 2");
    m.insert("osmo pocket 3", "DJI Osmo Pocket 3");
    m.insert("osmo pocket 2", "DJI Osmo Pocket 2");
    m.insert("osmo pocket", "DJI Osmo Pocket");
    m.insert("osmo action 4", "DJI Osmo Action 4");
    m.insert("osmo action 3", "DJI Osmo Action 3");
    m.insert("osmo action", "DJI Osmo Action");
    // GoPro
    m.insert("hero13 black", "GoPro HERO13 Black");
    m.insert("hero12 black", "GoPro HERO12 Black");
    m.insert("hero11 black", "GoPro HERO11 Black");
    m.insert("hero10 black", "GoPro HERO10 Black");
    m.insert("hero9 black", "GoPro HERO9 Black");
    m.insert("hero8 black", "GoPro HERO8 Black");
    m.insert("hero7 black", "GoPro HERO7 Black");
    m.insert("hero6 black", "GoPro HERO6 Black");
    m.insert("hero5 black", "GoPro HERO5 Black");
    m.insert("gopro hero", "GoPro HERO");
    // Apple
    m.insert("iphone 16 pro max", "Apple iPhone 16 Pro Max");
    m.insert("iphone 16 pro", "Apple iPhone 16 Pro");
    m.insert("iphone 16 plus", "Apple iPhone 16 Plus");
    m.insert("iphone 16", "Apple iPhone 16");
    m.insert("iphone 15 pro max", "Apple iPhone 15 Pro Max");
    m.insert("iphone 15 pro", "Apple iPhone 15 Pro");
    m.insert("iphone 15 plus", "Apple iPhone 15 Plus");
    m.insert("iphone 15", "Apple iPhone 15");
    m.insert("iphone 14 pro max", "Apple iPhone 14 Pro Max");
    m.insert("iphone 14 pro", "Apple iPhone 14 Pro");
    m.insert("iphone 14 plus", "Apple iPhone 14 Plus");
    m.insert("iphone 14", "Apple iPhone 14");
    m.insert("iphone 13 pro max", "Apple iPhone 13 Pro Max");
    m.insert("iphone 13 pro", "Apple iPhone 13 Pro");
    m.insert("iphone 13 mini", "Apple iPhone 13 Mini");
    m.insert("iphone 13", "Apple iPhone 13");
    m.insert("iphone 12 pro max", "Apple iPhone 12 Pro Max");
    m.insert("iphone 12 pro", "Apple iPhone 12 Pro");
    m.insert("iphone 12 mini", "Apple iPhone 12 Mini");
    m.insert("iphone 12", "Apple iPhone 12");
    m.insert("iphone 11 pro max", "Apple iPhone 11 Pro Max");
    m.insert("iphone 11 pro", "Apple iPhone 11 Pro");
    m.insert("iphone 11", "Apple iPhone 11");
    m.insert("iphone xs max", "Apple iPhone XS Max");
    m.insert("iphone xs", "Apple iPhone XS");
    m.insert("iphone xr", "Apple iPhone XR");
    m.insert("iphone x", "Apple iPhone X");
    // Samsung
    m.insert("sm-s938b", "Samsung Galaxy S25 Ultra");
    m.insert("sm-s928b", "Samsung Galaxy S24 Ultra");
    m.insert("sm-s918b", "Samsung Galaxy S23 Ultra");
    m.insert("sm-s908b", "Samsung Galaxy S22 Ultra");
    m.insert("sm-f956b", "Samsung Galaxy Z Fold 6");
    m.insert("sm-f741b", "Samsung Galaxy Z Flip 6");
    m.insert("galaxy s25 ultra", "Samsung Galaxy S25 Ultra");
    m.insert("galaxy s25+", "Samsung Galaxy S25+");
    m.insert("galaxy s25", "Samsung Galaxy S25");
    m.insert("galaxy s24 ultra", "Samsung Galaxy S24 Ultra");
    m.insert("galaxy s24+", "Samsung Galaxy S24+");
    m.insert("galaxy s24", "Samsung Galaxy S24");
    m.insert("galaxy s23 ultra", "Samsung Galaxy S23 Ultra");
    m.insert("galaxy s23+", "Samsung Galaxy S23+");
    m.insert("galaxy s23", "Samsung Galaxy S23");
    m.insert("galaxy s22 ultra", "Samsung Galaxy S22 Ultra");
    m.insert("galaxy z fold 6", "Samsung Galaxy Z Fold 6");
    m.insert("galaxy z flip 6", "Samsung Galaxy Z Flip 6");
    // Google
    m.insert("pixel 9 pro xl", "Google Pixel 9 Pro XL");
    m.insert("pixel 9 pro fold", "Google Pixel 9 Pro Fold");
    m.insert("pixel 9 pro", "Google Pixel 9 Pro");
    m.insert("pixel 9", "Google Pixel 9");
    m.insert("pixel 8 pro", "Google Pixel 8 Pro");
    m.insert("pixel 8", "Google Pixel 8");
    m.insert("pixel 7 pro", "Google Pixel 7 Pro");
    m.insert("pixel 7", "Google Pixel 7");
    m.insert("pixel 6 pro", "Google Pixel 6 Pro");
    m.insert("pixel 6", "Google Pixel 6");
    // Ricoh / Pentax
    m.insert("pentax k-3 mark iii", "Pentax K-3 Mark III");
    m.insert("pentax k-1 mark ii", "Pentax K-1 Mark II");
    m.insert("ricoh gr iiix", "Ricoh GR IIIx");
    m.insert("ricoh gr iii", "Ricoh GR III");
    // Sigma
    m.insert("sigma fp l", "Sigma fp L");
    m.insert("sigma fp", "Sigma fp");
    // Insta360
    m.insert("insta360 ace pro", "Insta360 Ace Pro");
    m.insert("insta360 ace", "Insta360 Ace");
    m.insert("insta360 x4", "Insta360 X4");
    m.insert("insta360 x3", "Insta360 X3");
    m.insert("insta360 one x2", "Insta360 ONE X2");
    m.insert("insta360 one r", "Insta360 ONE R");
    m.insert("insta360 go 3", "Insta360 GO 3");
    m.insert("insta360 go 2", "Insta360 GO 2");
    m.insert("insta360 go", "Insta360 GO");
    // RED Digital Cinema
    m.insert("red v-raptor", "RED V-Raptor");
    m.insert("red komodo", "RED Komodo");
    m.insert("red monstro", "RED Monstro");
    m.insert("red gemini", "RED Gemini");
    m.insert("red helium", "RED Helium");
    m.insert("red scarlet", "RED Scarlet");
    m.insert("red epic", "RED Epic");
    m.insert("red one", "RED One");
    // ARRI
    m.insert("alexa mini lf", "ARRI ALEXA Mini LF");
    m.insert("alexa mini", "ARRI ALEXA Mini");
    m.insert("alexa lf", "ARRI ALEXA LF");
    m.insert("alexa 35", "ARRI ALEXA 35");
    m.insert("alexa sxt", "ARRI ALEXA SXT");
    m.insert("amira", "ARRI Amira");
    // Blackmagic Design
    m.insert("pocket cinema camera 4k", "Blackmagic Pocket Cinema Camera 4K");
    m.insert("bmpcc4k", "Blackmagic Pocket Cinema Camera 4K");
    m.insert("pocket cinema camera 6k pro", "Blackmagic Pocket Cinema Camera 6K Pro");
    m.insert("pocket cinema camera 6k", "Blackmagic Pocket Cinema Camera 6K");
    m.insert("bmpcc6k", "Blackmagic Pocket Cinema Camera 6K");
    m.insert("ursa mini pro", "Blackmagic URSA Mini Pro");
    m.insert("cinema camera 6k", "Blackmagic Cinema Camera 6K");
    // Sony Cinema Line
    m.insert("ilme-fx6", "Sony FX6 (ILME-FX6)");
    m.insert("pxw-fx9", "Sony FX9 (PXW-FX9)");
    m.insert("ilme-fx3", "Sony FX3 (ILME-FX3)");
    m.insert("ilme-fx30", "Sony FX30 (ILME-FX30)");
    m.insert("mpc-3610", "Sony Venice (MPC-3610)");
    m.insert("venice 2", "Sony Venice 2");
    m.insert("venice", "Sony Venice");
    // Canon Cinema EOS
    m.insert("eos c70", "Canon EOS C70");
    m.insert("eos c100", "Canon EOS C100");
    m.insert("eos c200", "Canon EOS C200");
    m.insert("eos c300", "Canon EOS C300");
    m.insert("eos c500", "Canon EOS C500");
    m.insert("eos c700", "Canon EOS C700");
    m
});

static LENS_DB: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Canon RF
    m.insert("rf24-70mm f2.8 l is usm", "Canon RF 24-70mm f/2.8L IS USM");
    m.insert("rf24-105mm f4 l is usm", "Canon RF 24-105mm f/4L IS USM");
    m.insert("rf70-200mm f2.8 l is usm", "Canon RF 70-200mm f/2.8L IS USM");
    m.insert("rf50mm f1.2 l usm", "Canon RF 50mm f/1.2L USM");
    m.insert("rf50mm f1.8 stm", "Canon RF 50mm f/1.8 STM");
    m.insert("rf85mm f1.2 l usm", "Canon RF 85mm f/1.2L USM");
    m.insert("rf100mm f2.8 l macro is usm", "Canon RF 100mm f/2.8L Macro IS USM");
    m.insert("rf15-35mm f2.8 l is usm", "Canon RF 15-35mm f/2.8L IS USM");
    m.insert("rf100-500mm f4.5-7.1 l is usm", "Canon RF 100-500mm f/4.5-7.1L IS USM");
    // Canon EF
    m.insert("ef24-70mm f2.8l ii usm", "Canon EF 24-70mm f/2.8L II USM");
    m.insert("ef70-200mm f2.8l is iii usm", "Canon EF 70-200mm f/2.8L IS III USM");
    m.insert("ef50mm f1.8 stm", "Canon EF 50mm f/1.8 STM (Nifty Fifty)");
    m.insert("ef50mm f1.4 usm", "Canon EF 50mm f/1.4 USM");
    m.insert("ef85mm f1.4l is usm", "Canon EF 85mm f/1.4L IS USM");
    m.insert("ef135mm f2l usm", "Canon EF 135mm f/2L USM");
    // Nikon Z
    m.insert("nikkor z 24-70mm f/2.8 s", "Nikon NIKKOR Z 24-70mm f/2.8 S");
    m.insert("nikkor z 70-200mm f/2.8 vr s", "Nikon NIKKOR Z 70-200mm f/2.8 VR S");
    m.insert("nikkor z 50mm f/1.2 s", "Nikon NIKKOR Z 50mm f/1.2 S");
    m.insert("nikkor z 50mm f/1.8 s", "Nikon NIKKOR Z 50mm f/1.8 S");
    m.insert("nikkor z 85mm f/1.2 s", "Nikon NIKKOR Z 85mm f/1.2 S");
    m.insert("nikkor z 135mm f/1.8 s plena", "Nikon NIKKOR Z 135mm f/1.8 S Plena");
    m.insert("nikkor z 14-24mm f/2.8 s", "Nikon NIKKOR Z 14-24mm f/2.8 S");
    // Sony FE
    m.insert("fe 24-70mm f2.8 gm ii", "Sony FE 24-70mm f/2.8 GM II");
    m.insert("fe 24-70mm f2.8 gm", "Sony FE 24-70mm f/2.8 GM");
    m.insert("fe 70-200mm f2.8 gm oss ii", "Sony FE 70-200mm f/2.8 GM OSS II");
    m.insert("fe 70-200mm f2.8 gm oss", "Sony FE 70-200mm f/2.8 GM OSS");
    m.insert("fe 50mm f1.2 gm", "Sony FE 50mm f/1.2 GM");
    m.insert("fe 50mm f1.4 gm", "Sony FE 50mm f/1.4 GM");
    m.insert("fe 85mm f1.4 gm", "Sony FE 85mm f/1.4 GM");
    m.insert("fe 135mm f1.8 gm", "Sony FE 135mm f/1.8 GM");
    m.insert("fe 16-35mm f2.8 gm ii", "Sony FE 16-35mm f/2.8 GM II");
    m.insert("fe 24-105mm f4 g oss", "Sony FE 24-105mm f/4 G OSS");
    m.insert("fe 200-600mm f5.6-6.3 g oss", "Sony FE 200-600mm f/5.6-6.3 G OSS");
    // Sigma Art
    m.insert("35mm f1.4 dg hsm | art 012", "Sigma 35mm f/1.4 DG HSM Art");
    m.insert("35mm f1.4 dg dn | art 021", "Sigma 35mm f/1.4 DG DN Art");
    m.insert("50mm f1.4 dg hsm | art 014", "Sigma 50mm f/1.4 DG HSM Art");
    m.insert("85mm f1.4 dg hsm | art 016", "Sigma 85mm f/1.4 DG HSM Art");
    m.insert("105mm f1.4 dg hsm | art 018", "Sigma 105mm f/1.4 DG HSM Art (Bokeh Master)");
    m.insert("24-70mm f2.8 dg dn | art 019", "Sigma 24-70mm f/2.8 DG DN Art");
    m.insert("14-24mm f2.8 dg dn | art 019", "Sigma 14-24mm f/2.8 DG DN Art");
    // Tamron
    m.insert("28-75mm f/2.8 di iii vxd g2 (a063)", "Tamron 28-75mm f/2.8 Di III VXD G2");
    m.insert("70-180mm f/2.8 di iii vxd g2 (a065)", "Tamron 70-180mm f/2.8 Di III VXD G2");
    m.insert("35-150mm f/2-2.8 di iii vxd (a058)", "Tamron 35-150mm f/2-2.8 Di III VXD");
    m.insert("150-500mm f/5-6.7 di iii vc vxd (a057)", "Tamron 150-500mm f/5-6.7 Di III VC VXD");
    // Fujifilm
    m.insert("xf16-55mmf2.8 r lm wr", "Fujifilm XF 16-55mm f/2.8 R LM WR");
    m.insert("xf50-140mmf2.8 r lm ois wr", "Fujifilm XF 50-140mm f/2.8 R LM OIS WR");
    m.insert("xf56mmf1.2 r", "Fujifilm XF 56mm f/1.2 R");
    m.insert("xf23mmf1.4 r lm wr", "Fujifilm XF 23mm f/1.4 R LM WR (mk II)");
    m.insert("xf35mmf1.4 r", "Fujifilm XF 35mm f/1.4 R");
    m.insert("xf90mmf2 r lm wr", "Fujifilm XF 90mm f/2 R LM WR");
    m
});

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

pub fn resolve_camera_model(exif_model: &str) -> String {
    let norm = normalize(exif_model);
    if let Some(v) = CAMERA_DB.get(norm.as_str()) {
        return v.to_string();
    }
    for (k, v) in CAMERA_DB.iter() {
        if norm.contains(k) {
            return v.to_string();
        }
    }
    exif_model.to_string()
}

pub fn resolve_lens_model(exif_lens: &str) -> String {
    let norm = normalize(exif_lens);
    if let Some(v) = LENS_DB.get(norm.as_str()) {
        return v.to_string();
    }
    for (k, v) in LENS_DB.iter() {
        if norm.contains(k) {
            return v.to_string();
        }
    }
    for (k, v) in LENS_DB.iter() {
        if k.contains(norm.as_str()) && norm.len() > 8 {
            return v.to_string();
        }
    }
    exif_lens.to_string()
}
