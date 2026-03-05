use once_cell::sync::Lazy;
use std::collections::HashMap;

static CAMERA_DB: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Canon EOS DSLRs
    m.insert("canon eos-1d x mark iii", "Canon EOS-1D X Mark III");
    m.insert("canon eos-1d x mark ii", "Canon EOS-1D X Mark II");
    m.insert("canon eos-1d x", "Canon EOS-1D X");
    m.insert("canon eos 5d mark iv", "Canon EOS 5D Mark IV");
    m.insert("canon eos 5d mark iii", "Canon EOS 5D Mark III");
    m.insert("canon eos 5d mark ii", "Canon EOS 5D Mark II");
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
    m.insert("canon eos 2000d", "Canon EOS 2000D / Rebel T7 / Kiss X90");
    m.insert("canon eos rebel t7", "Canon EOS Rebel T7 / EOS 2000D");
    m.insert("canon eos 850d", "Canon EOS 850D / Rebel T8i");
    m.insert("canon eos 800d", "Canon EOS 800D / Rebel T7i");
    m.insert("canon eos 750d", "Canon EOS 750D / Rebel T6i");
    m.insert("canon eos 250d", "Canon EOS 250D / Rebel SL3");
    m.insert("canon eos 200d", "Canon EOS 200D / Rebel SL2");
    // Canon EOS R
    m.insert("canon eos r1", "Canon EOS R1");
    m.insert("canon eos r3", "Canon EOS R3");
    m.insert("canon eos r5", "Canon EOS R5");
    m.insert("canon eos r5 mark ii", "Canon EOS R5 Mark II");
    m.insert("canon eos r6", "Canon EOS R6");
    m.insert("canon eos r6 mark ii", "Canon EOS R6 Mark II");
    m.insert("canon eos r7", "Canon EOS R7");
    m.insert("canon eos r8", "Canon EOS R8");
    m.insert("canon eos r10", "Canon EOS R10");
    m.insert("canon eos r50", "Canon EOS R50");
    m.insert("canon eos r100", "Canon EOS R100");
    m.insert("canon eos rp", "Canon EOS RP");
    m.insert("canon eos r", "Canon EOS R");
    m.insert("canon eos m50", "Canon EOS M50 / Kiss M");
    // Nikon Z
    m.insert("nikon z 9", "Nikon Z 9");
    m.insert("z 9", "Nikon Z 9");
    m.insert("nikon z 8", "Nikon Z 8");
    m.insert("z 8", "Nikon Z 8");
    m.insert("nikon z 7ii", "Nikon Z 7II");
    m.insert("nikon z 7", "Nikon Z 7");
    m.insert("nikon z 6iii", "Nikon Z 6III");
    m.insert("nikon z 6ii", "Nikon Z 6II");
    m.insert("nikon z 6", "Nikon Z 6");
    m.insert("nikon z 5", "Nikon Z 5");
    m.insert("nikon z 50", "Nikon Z 50");
    m.insert("nikon z 30", "Nikon Z 30");
    m.insert("nikon z fc", "Nikon Z fc");
    m.insert("nikon z f", "Nikon Z f");
    // Nikon DSLRs
    m.insert("nikon d6", "Nikon D6");
    m.insert("nikon d5", "Nikon D5");
    m.insert("nikon d850", "Nikon D850");
    m.insert("nikon d810", "Nikon D810");
    m.insert("nikon d780", "Nikon D780");
    m.insert("nikon d750", "Nikon D750");
    m.insert("nikon d500", "Nikon D500");
    m.insert("nikon d7500", "Nikon D7500");
    m.insert("nikon d7200", "Nikon D7200");
    m.insert("nikon d5600", "Nikon D5600");
    m.insert("nikon d3500", "Nikon D3500");
    // Sony
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
    m.insert("ilce-7m4", "Sony Alpha 7 IV (A7 IV)");
    m.insert("ilce-7m3", "Sony Alpha 7 III (A7 III)");
    m.insert("ilce-7m2", "Sony Alpha 7 II (A7 II)");
    m.insert("ilce-7", "Sony Alpha 7 (A7)");
    m.insert("ilce-7c2", "Sony Alpha 7C II (A7C II)");
    m.insert("ilce-7cr", "Sony Alpha 7CR (A7CR)");
    m.insert("ilce-7c", "Sony Alpha 7C (A7C)");
    m.insert("ilce-6700", "Sony Alpha 6700 (A6700)");
    m.insert("ilce-6600", "Sony Alpha 6600 (A6600)");
    m.insert("ilce-6400", "Sony Alpha 6400 (A6400)");
    m.insert("ilce-6300", "Sony Alpha 6300 (A6300)");
    m.insert("ilce-6100", "Sony Alpha 6100 (A6100)");
    m.insert("ilce-6000", "Sony Alpha 6000 (A6000)");
    m.insert("ilme-fx30", "Sony FX30");
    m.insert("ilme-fx3", "Sony FX3");
    m.insert("zv-e10", "Sony ZV-E10");
    m.insert("zv-e1", "Sony ZV-E1");
    m.insert("zv-1", "Sony ZV-1");
    m.insert("dsc-rx100m7", "Sony RX100 VII");
    // Fujifilm
    m.insert("x-t5", "Fujifilm X-T5");
    m.insert("x-t4", "Fujifilm X-T4");
    m.insert("x-t3", "Fujifilm X-T3");
    m.insert("x-t30 ii", "Fujifilm X-T30 II");
    m.insert("x-t30", "Fujifilm X-T30");
    m.insert("x-h2s", "Fujifilm X-H2S");
    m.insert("x-h2", "Fujifilm X-H2");
    m.insert("x-s20", "Fujifilm X-S20");
    m.insert("x-s10", "Fujifilm X-S10");
    m.insert("x100vi", "Fujifilm X100VI");
    m.insert("x100v", "Fujifilm X100V");
    m.insert("gfx100s ii", "Fujifilm GFX100S II");
    m.insert("gfx 100 ii", "Fujifilm GFX 100 II");
    // Panasonic
    m.insert("dc-s5m2", "Panasonic Lumix S5 II");
    m.insert("dc-s5", "Panasonic Lumix S5");
    m.insert("dc-s1h", "Panasonic Lumix S1H");
    m.insert("dc-s1r", "Panasonic Lumix S1R");
    m.insert("dc-s1", "Panasonic Lumix S1");
    m.insert("dc-gh6", "Panasonic Lumix GH6");
    m.insert("dc-gh5", "Panasonic Lumix GH5");
    m.insert("dc-g9m2", "Panasonic Lumix G9 II");
    m.insert("dc-g9", "Panasonic Lumix G9");
    // Olympus / OM System
    m.insert("om-1 mark ii", "OM System OM-1 Mark II");
    m.insert("om-1", "OM System OM-1");
    m.insert("om-5", "OM System OM-5");
    m.insert("e-m1 mark iii", "Olympus OM-D E-M1 Mark III");
    // Leica
    m.insert("leica sl3", "Leica SL3");
    m.insert("leica q3", "Leica Q3");
    m.insert("leica q2", "Leica Q2");
    m.insert("leica m11", "Leica M11");
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
    // GoPro
    m.insert("hero13 black", "GoPro HERO13 Black");
    m.insert("hero12 black", "GoPro HERO12 Black");
    m.insert("hero11 black", "GoPro HERO11 Black");
    m.insert("hero10 black", "GoPro HERO10 Black");
    // Apple
    m.insert("iphone 16 pro max", "Apple iPhone 16 Pro Max");
    m.insert("iphone 16 pro", "Apple iPhone 16 Pro");
    m.insert("iphone 15 pro max", "Apple iPhone 15 Pro Max");
    m.insert("iphone 15 pro", "Apple iPhone 15 Pro");
    m.insert("iphone 14 pro max", "Apple iPhone 14 Pro Max");
    m.insert("iphone 14 pro", "Apple iPhone 14 Pro");
    m.insert("iphone 13 pro max", "Apple iPhone 13 Pro Max");
    // Samsung
    m.insert("sm-s928b", "Samsung Galaxy S25 Ultra");
    m.insert("sm-s918b", "Samsung Galaxy S24 Ultra");
    m.insert("sm-s908b", "Samsung Galaxy S23 Ultra");
    // Google
    m.insert("pixel 9 pro xl", "Google Pixel 9 Pro XL");
    m.insert("pixel 9 pro", "Google Pixel 9 Pro");
    m.insert("pixel 8 pro", "Google Pixel 8 Pro");
    m.insert("pixel 7 pro", "Google Pixel 7 Pro");
    // Ricoh / Pentax
    m.insert("pentax k-3 mark iii", "Pentax K-3 Mark III");
    m.insert("pentax k-1 mark ii", "Pentax K-1 Mark II");
    m.insert("ricoh gr iiix", "Ricoh GR IIIx");
    m.insert("ricoh gr iii", "Ricoh GR III");
    // Sigma
    m.insert("sigma fp l", "Sigma fp L");
    m.insert("sigma fp", "Sigma fp");
    // Insta360
    m.insert("insta360 x4", "Insta360 X4");
    m.insert("insta360 x3", "Insta360 X3");
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
