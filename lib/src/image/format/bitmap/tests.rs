use std::collections::HashSet;

use super::*;
use crate::image::*;

///
/// Test whether 2 bitmaps are equivalent, even if their data isn't exactly
/// equal
/// 
fn test_equivalence(a: &Bitmap, b: &Bitmap) -> Result<(), String> {
    let mut diffs = Vec::<String>::new();

    //Header
    if a.header.signature != b.header.signature {
        diffs.push(format!("Header: signature mismatch ({}/{})!", a.header.signature, b.header.signature));
    }

    if a.header.file_size != b.header.file_size {
        diffs.push(format!("Header: file size mismatch ({}/{})!", a.header.file_size, b.header.file_size));
    }

    if a.header.reserved != b.header.reserved {
        diffs.push(format!("Header: reserved mismatch ({}/{})!", a.header.reserved, b.header.reserved));
    }

    if a.header.data_offset != b.header.data_offset {
        diffs.push(format!("Header: data offset mismatch ({}/{})!", a.header.data_offset, b.header.data_offset));
    }

    //Info header
    if a.info_header.size != b.info_header.size {
        diffs.push(format!("Info Header: size mismatch ({}/{})!", a.info_header.size, b.info_header.size));
    }

    if a.info_header.width != b.info_header.width {
        if a.info_header.width == b.info_header.width.abs() {
            //Reversed width indicates the columns are mirrored, so they might still be equivalent
        }
        else {
            diffs.push(format!("Info Header: width mismatch ({}/{})!", a.info_header.width, b.info_header.width));
        }
    }

    if a.info_header.height != b.info_header.height {
        if a.info_header.height == b.info_header.height.abs() {
            //Reversed height indicates the rows are mirrored, so they might still be equivalent
        }
        else {
            diffs.push(format!("Info Header: height mismatch ({}/{})!", a.info_header.height, b.info_header.height));
        }
    }

    if a.info_header.planes != b.info_header.planes {
        diffs.push(format!("Info Header: planes mismatch ({}/{})!", a.info_header.planes, b.info_header.planes));
    }

    if a.info_header.bit_depth != b.info_header.bit_depth {
        diffs.push(format!("Info Header: bit depth mismatch ({}/{})!", a.info_header.bit_depth, b.info_header.bit_depth));
    }

    if a.info_header.compression != b.info_header.compression {
        diffs.push(format!("Info Header: compression mismatch ({}/{})!", a.info_header.compression, b.info_header.compression));
    }

    if a.info_header.image_size != b.info_header.image_size {
        diffs.push(format!("Info Header: image size mismatch ({}/{})!", a.info_header.image_size, b.info_header.image_size));
    }

    if a.info_header.y_pixels_per_meter != b.info_header.y_pixels_per_meter {
        if a.info_header.y_pixels_per_meter == b.info_header.y_pixels_per_meter.abs() {
            //Reversed y-resolution indicates the columns are mirrored, so they might still be equivalent
        }
        else {
            diffs.push(format!("Info Header: vertical resolution mismatch ({}/{})!", a.info_header.y_pixels_per_meter, b.info_header.y_pixels_per_meter));
        }
    }

    if a.info_header.x_pixels_per_meter != b.info_header.x_pixels_per_meter {
        if a.info_header.x_pixels_per_meter == b.info_header.x_pixels_per_meter.abs() {
            //Reversed x-resolution indicates the columns are mirrored, so they might still be equivalent
        }
        else {
            diffs.push(format!("Info Header: horizontal resolution mismatch ({}/{})!", a.info_header.x_pixels_per_meter, b.info_header.x_pixels_per_meter));
        }
    }

    if a.info_header.colors_used != b.info_header.colors_used {
        //colors_used should only matter with < 16bit bitmaps
        if a.info_header.bit_depth == b.info_header.bit_depth && a.info_header.bit_depth < 16 {
            diffs.push(format!("Info Header: colors used mismatch ({}/{})!", a.info_header.colors_used, b.info_header.colors_used));
        }
    }

    if a.info_header.important_colors != b.info_header.important_colors {
        //important_colors should only matter with < 16bit bitmaps
        if a.info_header.bit_depth == b.info_header.bit_depth && a.info_header.bit_depth < 16 {
            diffs.push(format!("Info Header: important colors mismatch ({}/{})!", a.info_header.important_colors, b.info_header.important_colors));
        }
    }

    //Color table
    //Should only matter with < 16bit bitmaps
    if a.info_header.bit_depth == b.info_header.bit_depth && a.info_header.bit_depth < 16 {
        let mut table_a: HashSet<color::ARGB> = HashSet::new();
        let mut table_b: HashSet<color::ARGB> = HashSet::new();

        for color in &a.color_table.colors {
            table_a.insert(*color);
        }

        for color in &b.color_table.colors {
            table_b.insert(*color);
        }

        for diff_a in table_a.difference(&table_b) {
            diffs.push(format!("Color table difference; {} is in bitmap a but not bitmap b.", diff_a.as_u32(true)));
        }

        for diff_b in table_b.difference(&table_a) {
            diffs.push(format!("Color table difference; {} is in bitmap b but not bitmap a.", diff_b.as_u32(true)));
        }
    }

    //Pixel data
    match &a.pixels.pixels {
        BitmapPixelData::Colors(a_pixels) => {
            match &b.pixels.pixels {
                BitmapPixelData::Colors(b_pixels) => { 
                    //Get scanlines from a, reversing them if height xor vertical resolution is negative            
                    let scanlines_a: Vec<&[color::ARGB]> = if (a.info_header.height < 0) ^ (a.info_header.y_pixels_per_meter < 0) {
                        a_pixels.chunks_exact(a.info_header.width.unsigned_abs() as usize).rev().collect()
                    }
                    else {
                        a_pixels.chunks_exact(a.info_header.width.unsigned_abs() as usize).collect()
                    };           
                    
                    //Get scanlines from b, reversing them if height xor vertical resolution is negative            
                    let scanlines_b: Vec<&[color::ARGB]> = if (b.info_header.height < 0) ^ (b.info_header.y_pixels_per_meter < 0) {
                        b_pixels.chunks_exact(b.info_header.width.unsigned_abs() as usize).rev().collect()
                    }
                    else {
                        b_pixels.chunks_exact(a.info_header.width.unsigned_abs() as usize).collect()
                    }; 

                    for i in 0..scanlines_a.len().max(scanlines_b.len()) {
                        let scanline_a = if scanlines_a.len() > i {
                            Some(scanlines_a[i])
                        }
                        else {
                            None
                        };

                        let scanline_b = if scanlines_b.len() > i {
                            Some(scanlines_b[i])
                        }
                        else {
                            None
                        };

                        if scanline_a.is_none() && scanline_b.is_some() {
                            diffs.push(format!("Row {i} exists in bitmap b but not a."));
                            continue;
                        }
                        else if scanline_a.is_some() && scanline_b.is_none() {
                            diffs.push(format!("Row {i} exists in bitmap a but not b."));
                            continue;
                        }
                        else if scanline_a.is_none() && scanline_b.is_none() {
                            continue;
                        }

                        let scanline_a = scanline_a.unwrap();
                        let scanline_b = scanline_b.unwrap();

                        //Get pixels from scanline_a, reversing them if width xor horizontal resolution is negative            
                        let cols_a: Vec<color::ARGB> = if (a.info_header.width < 0) ^ (a.info_header.x_pixels_per_meter < 0) {
                            scanline_a.iter().rev().copied().collect()
                        }
                        else {
                            Vec::from(scanline_a)
                        };

                        //Get pixels from scanline_b, reversing them if width xor horizontal resolution is negative            
                        let cols_b: Vec<color::ARGB> = if (b.info_header.width < 0) ^ (b.info_header.x_pixels_per_meter < 0) {
                            scanline_b.iter().rev().copied().collect()
                        }
                        else {
                            Vec::from(scanline_b)
                        };

                        for j in 0..cols_a.len().max(cols_b.len()) {
                            let pixel_a = if cols_a.len() > j {
                                Some(cols_a[j])
                            }
                            else {
                                None
                            };

                            let pixel_b = if cols_b.len() > j {
                                Some(cols_b[j])
                            }
                            else {
                                None
                            };

                            if pixel_a.is_none() && pixel_b.is_some() {
                                diffs.push(format!("Row {i}, Column {j} exists in bitmap b but not a."));
                                continue;
                            }
                            else if pixel_a.is_some() && pixel_b.is_none() {
                                diffs.push(format!("Row {i}, Column {j} exists in bitmap a but not b."));
                                continue;
                            }
                            else if pixel_a.is_none() && pixel_b.is_none() {
                                continue;
                            }
    
                            let pixel_a = pixel_a.unwrap();
                            let pixel_b = pixel_b.unwrap();

                            if pixel_a != pixel_b {
                                diffs.push(format!("Pixel {i}/{j} mismatch between bitmaps."));
                            }
                        }
                    }
                },
                BitmapPixelData::Indices(_) => {
                    diffs.push(String::from("Pixel data type mismatch been bitmaps: pixels vs indices."));
                }
            }
        },
        BitmapPixelData::Indices(a_indices) => {
            match &b.pixels.pixels {
                BitmapPixelData::Colors(_) => {
                    diffs.push(String::from("Pixel data type mismatch been bitmaps: indices vs pixels."));
                },
                BitmapPixelData::Indices(b_indices) => {
                    //Get scanlines from a, reversing them if height xor vertical resolution is negative            
                    let scanlines_a: Vec<&[u8]> = if (a.info_header.height < 0) ^ (a.info_header.y_pixels_per_meter < 0) {
                        a_indices.chunks_exact(a.info_header.width.unsigned_abs() as usize).rev().collect()
                    }
                    else {
                        a_indices.chunks_exact(a.info_header.width.unsigned_abs() as usize).collect()
                    };           
                    
                    //Get scanlines from b, reversing them if height xor vertical resolution is negative            
                    let scanlines_b: Vec<&[u8]> = if (b.info_header.height < 0) ^ (b.info_header.y_pixels_per_meter < 0) {
                        b_indices.chunks_exact(b.info_header.width.unsigned_abs() as usize).rev().collect()
                    }
                    else {
                        b_indices.chunks_exact(a.info_header.width.unsigned_abs() as usize).collect()
                    }; 

                    for i in 0..scanlines_a.len().max(scanlines_b.len()) {
                        let scanline_a = if scanlines_a.len() > i {
                            Some(scanlines_a[i])
                        }
                        else {
                            None
                        };

                        let scanline_b = if scanlines_b.len() > i {
                            Some(scanlines_b[i])
                        }
                        else {
                            None
                        };

                        if scanline_a.is_none() && scanline_b.is_some() {
                            diffs.push(format!("Row {i} exists in bitmap b but not a."));
                            continue;
                        }
                        else if scanline_a.is_some() && scanline_b.is_none() {
                            diffs.push(format!("Row {i} exists in bitmap a but not b."));
                            continue;
                        }
                        else if scanline_a.is_none() && scanline_b.is_none() {
                            continue;
                        }

                        let scanline_a = scanline_a.unwrap();
                        let scanline_b = scanline_b.unwrap();

                        //Get pixels from scanline_a, reversing them if width xor horizontal resolution is negative            
                        let cols_a: Vec<u8> = if (a.info_header.width < 0) ^ (a.info_header.x_pixels_per_meter < 0) {
                            scanline_a.iter().rev().copied().collect()
                        }
                        else {
                            Vec::from(scanline_a)
                        };

                        //Get pixels from scanline_b, reversing them if width xor horizontal resolution is negative            
                        let cols_b: Vec<u8> = if (b.info_header.width < 0) ^ (b.info_header.x_pixels_per_meter < 0) {
                            scanline_b.iter().rev().copied().collect()
                        }
                        else {
                            Vec::from(scanline_b)
                        };

                        for j in 0..cols_a.len().max(cols_b.len()) {
                            let pixel_a = if cols_a.len() > j {
                                Some(cols_a[j])
                            }
                            else {
                                None
                            };

                            let pixel_b = if cols_b.len() > j {
                                Some(cols_b[j])
                            }
                            else {
                                None
                            };

                            if pixel_a.is_none() && pixel_b.is_some() {
                                diffs.push(format!("Row {i}, Column {j} exists in bitmap b but not a."));
                                continue;
                            }
                            else if pixel_a.is_some() && pixel_b.is_none() {
                                diffs.push(format!("Row {i}, Column {j} exists in bitmap a but not b."));
                                continue;
                            }
                            else if pixel_a.is_none() && pixel_b.is_none() {
                                continue;
                            }
    
                            let pixel_a = a.color_table_color(pixel_a.unwrap() as usize);
                            let pixel_b = b.color_table_color(pixel_b.unwrap() as usize);



                            if (pixel_a.is_some() ^ pixel_b.is_some()) || (pixel_a.is_some() && pixel_a.unwrap() != pixel_b.unwrap()) {
                                diffs.push(format!("Pixel {i}/{j} mismatch between bitmaps."));
                            }
                        }
                    }
                }
            }
        }
    }

    if diffs.is_empty() {
        Ok(())
    }
    else {
        Err(diffs.join(" --  "))
    }
}

///
/// A 24-bit bitmap's equivalent representations as raw bytes, a Bitmap, and an Image
/// 
fn input_24_1() -> (&'static [u8], Bitmap, Image) {
    let input_bytes: &[u8] = &[
        //Header
        0x42, 0x4D, //Signature = 19778 = BM
        0x66, 0x00, 0x00, 0x00, //File size = 102
        0x00, 0x00, 0x00, 0x00, //Reserved = 0
        0x36, 0x00, 0x00, 0x00, //Data offset = 54

        //Info Header
        0x28, 0x00, 0x00, 0x00, //Info Header Size = 40
        0x04, 0x00, 0x00, 0x00, //Width = 4
        0x04, 0x00, 0x00, 0x00, //Height = 4
        0x01, 0x00, //Planes = 1
        0x18, 0x00, //Bit depth = 24
        0x00, 0x00, 0x00, 0x00, //Compression = 0
        0x00, 0x00, 0x00, 0x00, //Image size = 0
        0xC4, 0x0E, 0x00, 0x00, //X Resolution = 3780
        0xC4, 0x0E, 0x00, 0x00, //Y Resolution = 3780
        0x00, 0x00, 0x00, 0x00, //Colors used = 0
        0x00, 0x00, 0x00, 0x00, //Important colors = 0

        //Color table (omitted because bit depth 24)
        
        //Pixel data
        //Row 3
        0x00, 0x00, 0x00, //Column 0
        0x00, 0x00, 0xFF, //Column 1
        0x00, 0xFF, 0x00, //Column 2
        0x00, 0xFF, 0xFF, //Column 3

        //Row 2
        0xFF, 0x00, 0x00, //Column 0
        0xFF, 0x00, 0xFF, //Column 1
        0xFF, 0xFF, 0x00, //Column 2
        0xFF, 0xFF, 0xFF, //Column 3

        //Row 1
        0x00, 0x00, 0x00, //Column 0
        0x00, 0x00, 0xCC, //Column 1
        0x00, 0xCC, 0x00, //Column 2
        0x00, 0xCC, 0xCC, //Column 3

        //Row 0
        0xCC, 0x00, 0x00, //Column 0
        0xCC, 0x00, 0xCC, //Column 1
        0xCC, 0xCC, 0x00, //Column 2
        0xCC, 0xCC, 0xCC //Column 3
    ];

    let bitmap = Bitmap {
        header: BitmapHeader { 
            signature: 0x4D_42_u16, 
            file_size: 0x66_u32, 
            reserved: 0x00_u32, 
            data_offset: 0x36_u32 
        },
        info_header: BitmapInfoHeader { 
            size: 0x28_u32, 
            width: 0x04_i32, 
            height: 0x04_i32, 
            planes: 0x01_u16, 
            bit_depth: 0x18_u16, 
            compression: 0x00_u32, 
            image_size: 0x00_u32, 
            x_pixels_per_meter: 0x0E_C4_i32, 
            y_pixels_per_meter: 0x0E_C4_i32, 
            colors_used: 0x00_u32, 
            important_colors: 0x00_u32 
        },
        color_table: BitmapColorTable { 
            colors: Vec::new() 
        },
        pixels: BitmapPixels { 
            pixels: BitmapPixelData::Colors(vec![
                color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00 },
                color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF },
                color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xCC, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0xCC, blue: 0x00 },
                color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0x00, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xCC, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0xCC, blue: 0xCC }
            ])
        },
    };
    
    let image = Image {
        width: 4_usize,
        height: 4_usize,
        pixels: vec![
            color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0x00, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xCC, blue: 0xCC }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0xCC, blue: 0xCC },
            color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xCC, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xCC, green: 0xCC, blue: 0x00 },
            color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF },
            color::ARGB { alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00 }, color::ARGB { alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00 }
        ],
    };

    (input_bytes, bitmap, image)
}

#[test]
fn bitmap_from_bytes_24_1() -> Result<(), String> {
    let (input_bytes, expected, _) = input_24_1();

    let bitmap = Bitmap::try_from(input_bytes)?;

    test_equivalence(&bitmap, &expected)?;

    Ok(())
}

#[test]
fn bytes_from_bitmap_24_1() -> Result<(), String> {
    let (expected, bitmap, _) = input_24_1();

    let bitmap_bytes = &Vec::try_from(bitmap)?[..];

    if *bitmap_bytes != *expected {
        Err(String::from("The bytes created from the provided bitmap do not match the expected bytes."))
    }
    else {
        Ok(())
    }
}

#[test]
fn image_from_bitmap_24_1() -> Result<(), String> {
    let(_, bitmap, expected) = input_24_1();

    let bitmap_image = Image::try_convert_from(bitmap, ())?;

    if bitmap_image != expected {
        Err(String::from("The image created from the provided bitmap does not match the expected image."))
    }
    else {
        Ok(())
    }
}

#[test]
fn bitmap_from_image_24_1() -> Result<(), String> {
    let(_, expected, image) = input_24_1();

    let image_bitmap = Bitmap::try_convert_from(image, BitmapConvertData {
        bit_depth: 24,
        compression: 0,
        x_pixels_per_meter: 3780,
        y_pixels_per_meter: 3780,
    })?;

    test_equivalence(&image_bitmap, &expected)?;

    Ok(())
}

