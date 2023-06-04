#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::{color, image, utility, convert::ConvertableFrom};
use image::Image;
use utility::FromBitSlice;
use crate::constants::bitmap;

///
/// A image in bmp format.
/// Bitmap format:
/// http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bitmap {
    pub header: BitmapHeader,
    pub info_header: BitmapInfoHeader,
    pub color_table: BitmapColorTable,
    pub pixels: BitmapPixels,
}

///
/// Bitmap header data, regarding
/// the size of the bitmap and location
/// of the pixel data
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitmapHeader {
    ///
    /// Bitmap signature. Should always be BM.
    ///
    pub signature: u16,
    ///
    /// The actual size of the file, including both headers, the color table,
    /// and the pixel data.
    ///
    pub file_size: u32,
    ///
    /// ??
    ///
    pub reserved: u32,
    ///
    /// The index at which the pixel data begins. Everything prior to this is
    /// header/color table data.
    ///
    pub data_offset: u32,
}

///
/// Bitmap info header data, regarding
/// layout/contents of the bitmap.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitmapInfoHeader {
    ///
    /// Size of this info header.
    ///
    pub size: u32,
    ///
    /// Horizontal width of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// vertically.
    ///
    pub width: i32,
    ///
    /// Vertical height of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// horizontally.
    ///
    pub height: i32,
    ///
    /// Number of planes (?).
    ///
    pub planes: u16,
    ///
    /// Pixel bit depth, i.e. the number of
    /// bits required to represent a color.
    ///
    /// 1, 4, 8: Bits contain index to a color in the color table.
    /// 16, 24, 32: Bits contain color data.
    ///
    pub bit_depth: u16,
    ///
    /// The type of compression used.
    ///     0 = BI_RGB   no compression
    ///     1 = BI_RLE8 8bit RLE encoding
    ///     2 = BI_RLE4 4bit RLE encoding
    ///
    pub compression: u32,
    ///
    /// Compressed size of image.
    /// This can be 0 if compression == 0
    ///
    pub image_size: u32,
    ///
    /// Horizontal resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// vertically.
    ///
    pub x_pixels_per_meter: i32,
    ///
    /// Vertical resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// horizontally.
    ///
    pub y_pixels_per_meter: i32,
    ///
    /// Number of colors used in the bitmap
    ///
    pub colors_used: u32,
    ///
    /// Number of important colors (?)
    /// 0 = all
    ///
    pub important_colors: u32,
}

///
/// Bitmap color definitions.
/// Ordered Red-Green-Blue-Reserved,
/// each 1 byte in size.
/// Present only if bit depth is less than 8.
/// Colors ordered by importance.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitmapColorTable {
    pub colors: Vec<color::ARGB>,
}

///
/// Bitmap pixels are either colors,
/// or indices into the color table
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitmapPixelData {
    Colors(Vec<color::ARGB>),
    Indices(Vec<u8>)
}

impl Default for BitmapPixelData {
    fn default() -> Self {
        Self::Colors(Vec::default())
    }
}

///
/// The actual image data in the bitmap.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitmapPixels {
    pub pixels: BitmapPixelData,
}

///
/// Additional data required to create a bmp image from
/// a grid of pixels
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitmapConvertData {
    ///
    /// Pixel bit depth, i.e. the number of
    /// bits required to represent a color.
    ///
    /// 1, 4, 8: Bits contain index to a color in the color table.
    /// 16, 24, 32: Bits contain color data.
    ///
    pub bit_depth: u16,
    ///
    /// The type of compression used.
    ///     0 = BI_RGB   no compression
    ///     1 = BI_RLE8 8bit RLE encoding
    ///     2 = BI_RLE4 4bit RLE encoding
    ///
    pub compression: u32,
    ///
    /// Horizontal resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// vertically.
    ///
    pub x_pixels_per_meter: i32,
    ///
    /// Vertical resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// horizontally.
    ///
    pub y_pixels_per_meter: i32,
}

impl Bitmap {
    ///
    /// Get the color at index i of the color table, if it exists
    /// 
    pub fn color_table_color(&self, i: usize) -> Option<color::ARGB> {
        if i >= self.color_table.colors.len() {
            None
        }
        else {
            Some(self.color_table.colors[i])
        }
    }

    pub fn formatted_bitstring(&self) -> String {

        fn u16_bits(n: u16) -> [u8; 2] {
            n.to_le_bytes()
        }

        fn u32_bits(n: u32) -> [u8; 4] {
            n.to_le_bytes()
        }

        fn i32_bits(n: i32) -> [u8; 4] {
            n.to_le_bytes()
        }

        fn bitstring(bits: &[u8], skip_last: usize) -> String {
            bits.iter()
                .take(4 - skip_last)
                .map(|n| format!("{:#04X?},", n))
                .reduce(|a, b| format!("{a} {b}"))
                .unwrap()
        }

        let color_table = if self.color_table.colors.is_empty() {
            String::from("//Empty")
        }
        else {
            let mut row = -1;
            self.color_table.colors.iter()
                .map(|color| {
                    row += 1;
                    let color_u32 = color.as_u32(false);
                    format!("{: <22} //Color table entry {row}", bitstring(&u32_bits(color_u32), 0))
                })
                .reduce(|a, b| format!("{a}\n{b}"))
                .unwrap()
        };

        let pixel_data = match &self.pixels.pixels {
            BitmapPixelData::Colors(pixels) => {
                if pixels.is_empty() {
                    String::from("//Empty")
                }
                else {
                    let columns = self.info_header.width.unsigned_abs() as usize;
                    let bytes_per_pixel = (self.info_header.bit_depth as f32 / 8_f32).ceil() as usize;
                    let mut row = -1;
                    pixels.chunks_exact(columns)
                    .map(|scanline| {
                        row += 1;
                        let mut col = -1;

                        let columns = scanline.iter()
                            .map(|pixel| {
                                col += 1;
                                let color_u32 = if self.info_header.bit_depth < 32_u16 {
                                    pixel.with_alpha(0_u8).as_u32(false)
                                }
                                else {
                                    pixel.as_u32(false)
                                };
                                
                                format!("{: <22} //Column {col}", bitstring(&u32_bits(color_u32), 4 - bytes_per_pixel))
                            })
                            .reduce(|a, b| format!("{a}\n{b}"))
                            .unwrap();

                        format!("//Row {row}\n{columns}")
                    })
                    .reduce(|a, b| format!("{a}\n\n{b}"))
                    .unwrap()
                }
            },
            BitmapPixelData::Indices(indices) => {
                if indices.is_empty() {
                    String::from("//Empty")
                }
                else {
                    let columns = self.info_header.width.unsigned_abs() as usize;
                    let mut row = -1;
                    indices.chunks_exact(columns)
                    .map(|scanline| {
                        row += 1;
                        let mut column = -1;

                        let columns = scanline.chunks(4)
                            .map(|index_chunk| {
                                
                                let column_start = column;

                                let s = index_chunk.iter()
                                    .map(|index| {
                                        let s = bitstring(&[*index; 1], 0_usize);

                                        column += 1;
                                        s
                                    })
                                    .reduce(|a, b| format!("{a} {b}"))
                                    .unwrap();

                                format!("{: <22} //Columns {} - {column}", s, column_start + 1)
                            })
                            .reduce(|a, b| format!("{a}\n{b}"))
                            .unwrap();

                        format!("//Row {row}\n{columns}")
                    })
                    .reduce(|a, b| format!("{a}\n\n{b}"))
                    .unwrap()
                }
            }
        };

        format!("
//Header
{: <22} //Signature = {}
{: <22} //File Size = {}
{: <22} //Reserved = {}
{: <22} //Data Offset = {}

//Info Header
{: <22} //Info Header Size = {}
{: <22} //Width = {}
{: <22} //Height = {}
{: <22} //Planes = {}
{: <22} //Bit Depth = {}
{: <22} //Compression = {}
{: <22} //Image Size = {}
{: <22} //X Resolution = {}
{: <22} //Y Resolution = {}
{: <22} //Colors Used = {}
{: <22} //Important Colors = {}

//Color Table
{}

//Pixel Data
{}",
            bitstring(&u16_bits(self.header.signature), 0), self.header.signature,
            bitstring(&u32_bits(self.header.file_size), 0), self.header.file_size,
            bitstring(&u32_bits(self.header.reserved), 0), self.header.reserved,
            bitstring(&u32_bits(self.header.data_offset), 0), self.header.data_offset,

            bitstring(&u32_bits(self.info_header.size), 0), self.info_header.size,
            bitstring(&i32_bits(self.info_header.width), 0), self.info_header.width,
            bitstring(&i32_bits(self.info_header.height), 0), self.info_header.height,
            bitstring(&u16_bits(self.info_header.planes), 0), self.info_header.planes,
            bitstring(&u16_bits(self.info_header.bit_depth), 0), self.info_header.bit_depth,
            bitstring(&u32_bits(self.info_header.compression), 0), self.info_header.compression,
            bitstring(&u32_bits(self.info_header.image_size), 0), self.info_header.image_size,
            bitstring(&i32_bits(self.info_header.x_pixels_per_meter), 0), self.info_header.x_pixels_per_meter,
            bitstring(&i32_bits(self.info_header.y_pixels_per_meter), 0), self.info_header.y_pixels_per_meter,
            bitstring(&u32_bits(self.info_header.colors_used), 0), self.info_header.colors_used,
            bitstring(&u32_bits(self.info_header.important_colors), 0), self.info_header.important_colors,
            color_table,
            pixel_data
        )
    }
}

impl From<Bitmap> for BitmapConvertData {
    fn from(value: Bitmap) -> Self {
        Self 
        { 
            bit_depth: value.info_header.bit_depth, 
            compression: value.info_header.compression,
            x_pixels_per_meter: value.info_header.x_pixels_per_meter,
            y_pixels_per_meter: value.info_header.y_pixels_per_meter
        }
    }
}

impl From<&Bitmap> for BitmapConvertData {
    fn from(value: &Bitmap) -> Self {
        Self 
        { 
            bit_depth: value.info_header.bit_depth, 
            compression: value.info_header.compression,
            x_pixels_per_meter: value.info_header.x_pixels_per_meter,
            y_pixels_per_meter: value.info_header.y_pixels_per_meter
        }
    }
}

///
/// Read a bmp from an array of bytes
///
impl TryFrom<&[u8]> for Bitmap {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut offset: usize = 0;

        fn get_next_bytes<'a, 'b>(buffer: &'a [u8], offset: &'b mut usize, n: usize) -> &'a [u8] {
            let start = *offset;
            *offset += n;
            &buffer[start..*offset]
        }

        let next_u16 =
            |offset: &mut usize| u16::reduce_bit_slice(get_next_bytes(value, offset, 2));

        let next_u32 =
            |offset: &mut usize| u32::reduce_bit_slice(get_next_bytes(value, offset, 4));

        let next_i32 =
            |offset: &mut usize| i32::reduce_bit_slice(get_next_bytes(value, offset, 4));

        //File header
        let header = BitmapHeader {
            signature: next_u16(&mut offset),
            file_size: next_u32(&mut offset),
            reserved: next_u32(&mut offset),
            data_offset: next_u32(&mut offset),
        };

        //Image header
        let info_header = BitmapInfoHeader {
            size: next_u32(&mut offset),
            width: next_i32(&mut offset),
            height: next_i32(&mut offset),
            planes: next_u16(&mut offset),
            bit_depth: next_u16(&mut offset),
            compression: next_u32(&mut offset),
            image_size: next_u32(&mut offset),
            x_pixels_per_meter: next_i32(&mut offset),
            y_pixels_per_meter: next_i32(&mut offset),
            colors_used: next_u32(&mut offset),
            important_colors: next_u32(&mut offset),
        };
        //Color table

        //All data between the current offset and the data offset from the header goes in the color table
        let color_table_length = match (header.data_offset as usize).checked_sub(offset) {
            Some(len) => Ok(len),
            None => Err(String::from(
                "Bitmap data is malformed; data offset points to the info header.",
            )),
        }?;

        let palette: Option<Vec<color::ARGB>> = if color_table_length > 0 {
            let color_table_raw = get_next_bytes(value, &mut offset, color_table_length);

            //Each color in the pallette is 4 bytes, the first 3 representing the Blue, Green and Red intensities respectively, with the last unused or alpha
            Some(
                color_table_raw
                    .chunks(4)
                    .map(|chunk| color::ARGB {
                        blue: *chunk.first().unwrap_or(&0),
                        green: *chunk.get(1).unwrap_or(&0),
                        red: *chunk.get(2).unwrap_or(&0),
                        alpha: *chunk.get(3).unwrap_or(&0),
                    })
                    .collect(),
            )
        } else {
            None
        };

        let color_table = BitmapColorTable {
            colors: palette.unwrap_or_default(),
        };

        //Get pixels in the bitmap
        //bpp = 1, 4 or 8: value of each pixel has a size <= 1 byte, and is an index of the color table
        let pixel_vec: BitmapPixelData = if [1, 4, 8].contains(&info_header.bit_depth) {
            let mut pixel_indices: Vec<u8> = Vec::new();

            //Get the width of the scanline based on bit depth and line width
            let pixels_per_bit = f32::ceil(8_f32 / (info_header.bit_depth as f32)) as usize;
            let scaline_width_temp =
                f32::ceil(f32::abs(info_header.width as f32) / (pixels_per_bit as f32)) as i32;
            let scanline_width = utility::round_to_next_multiple_of_4(scaline_width_temp);

            //Read in each scanline
            loop {
                let mut done = false;
                let mut count = scanline_width;

                // I don't think this should ever happen for a properly-formatted
                // bitmap, but if the scanline goes past the end of the file,
                // truncate it
                if value.len() < offset + scanline_width {
                    count = ((value.len() as i32) - (offset as i32)) as usize;
                    done = true;
                }

                //Get the scanline data
                let scanline = get_next_bytes(value, &mut offset, count);

                // Loop over each bit in the scanline, ignoring 0-padding at the end of the scanline.
                scanline.iter().enumerate().for_each(|(ndx, chunk)| {
                    if ndx < (scaline_width_temp as usize) {
                        //For each pixel in the bit
                        for i in 1..=pixels_per_bit {
                            //If past the width of the line, the rest of the bits are padding
                            if (pixels_per_bit * ndx) + i > (info_header.width as usize) {
                                break;
                            }

                            //Extract the palette index of the (i - 1)th pixel from the byte
                            let index = (*chunk
                                >> (8 - ((info_header.bit_depth as i32) * (i as i32))))
                                & ((2_u16.pow(info_header.bit_depth as u32) - 1) as u8);

                            pixel_indices.push(index);
                        }
                    }
                });

                if done {
                    break;
                }
            }

            BitmapPixelData::Indices(pixel_indices)
        }
        //bpp = 16: value of each pixel is 2 bytes, with each 5 bits representing Blue, Green and Red intensities respectively, and the last bit being unused.
        else if info_header.bit_depth == 16 {
            return Err(String::from("Not implemented for 16-bit images!"));
        }
        //bpp = 24: value of each pixel is 3 bytes, representing Blue, Green and Red intensities respectively
        //bpp = 32: value of each pixel is 4 bytes, representing Alpha, Blue, Green and Red intensities respectively
        else if [24, 32].contains(&info_header.bit_depth) {
            let mut pixel_values: Vec<color::ARGB> = Vec::new();

            //Get scanline width based on line width
            let bytesperpixel = f32::ceil((info_header.bit_depth as f32) / 8_f32) as usize;
            let scaline_width_temp = i32::abs(info_header.width * (bytesperpixel as i32));
            let scanline_width = utility::round_to_next_multiple_of_4(scaline_width_temp);

            //Read in each scanline
            loop {
                let mut done = false;
                let mut count = scanline_width;

                // I don't think this should ever happen for a properly-formatted
                // bitmap, but if the scanline goes past the end of the file,
                // truncate it
                if value.len() < offset + scanline_width {
                    count = ((value.len() as i32) - (offset as i32)) as usize;
                    done = true;
                }

                //Get the scanline data
                let scanline = get_next_bytes(value, &mut offset, count);
                let mut line: Vec<color::ARGB> = Vec::new();

                // Loop over each chunk of 3/4 bytes in the scanline, ignoring 0-padding at the end of the scanline.
                scanline.chunks(bytesperpixel).for_each(|chunk| {
                    //Ignore 0-padding
                    if chunk.len() == bytesperpixel && (line.len() as u32) < info_header.width.unsigned_abs() {
                        //Extract alpha, blue, green, and red from their respective bytes
                        let color = color::ARGB {
                            blue: *chunk.first().unwrap_or(&0),
                            green: *chunk.get(1).unwrap_or(&0),
                            red: *chunk.get(2).unwrap_or(&0),
                            alpha: match bytesperpixel {
                                4 => *chunk.get(3).unwrap_or(&0),
                                _ => 0xFF,
                            },
                        };

                        line.push(color);
                    }
                });

                //Append the scanline
                pixel_values.append(&mut line);

                if done {
                    break;
                }
            }

            BitmapPixelData::Colors(pixel_values)
        } else {
            return Err(format!(
                "Not implemented for {}-bit images!",
                info_header.bit_depth
            ));
        };

        let pixels = BitmapPixels { pixels: pixel_vec };

        Ok(Self {
            header,
            info_header,
            color_table,
            pixels,
        })
    }
}

///
/// Read a bmp from an array of bytes
///
impl TryFrom<Vec<u8>> for Bitmap {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let as_slice = &value[..];
        Self::try_from(as_slice)
    }
}

///
/// Convert a bmp to an array of bytes
///
impl TryFrom<Bitmap> for Vec<u8> {
    type Error = String;

    fn try_from(value: Bitmap) -> Result<Self, Self::Error> {
        //Convert the bitmap pixels to bytes
        let width = value.info_header.width.unsigned_abs() as usize;
        let mut pixel_bytes: Vec<u8> = Vec::new();

        match value.pixels.pixels {
            BitmapPixelData::Indices(ref indices) => {
                for scanline in indices.chunks_exact(width) {
                    let mut bytes: Vec<u8> = Vec::new();

                    if [1, 4, 8].contains(&value.info_header.bit_depth) {
                        let bit_depth_u8 = value.info_header.bit_depth as u8;
                        let pixels_per_bit = f32::ceil(8_f32 / (value.info_header.bit_depth as f32)) as usize;

                        let mut first: bool = true;
                        let mut current: u8 = 0;

                        for (index, color_index) in scanline.iter().enumerate() {
                            //Only take the relevant bits from the color_index
                            let normalized_index = color_index & ((2 << (value.info_header.bit_depth + 1)) - 1) as u8;
                            let index_mod = (index % pixels_per_bit) as u8;

                            //If first index in byte, push current to vector
                            if index_mod == 0 {
                                if first {
                                    first = false;
                                }
                                else {
                                    bytes.push(current.to_be());
                                }

                                current = 0;
                            }

                            //Add to the current byte
                            let shifted_index = normalized_index << (8 - bit_depth_u8 - (index_mod * bit_depth_u8));

                            current += shifted_index;

                            //If last element, push current byte
                            if index == scanline.len() - 1 {
                                bytes.push(current.to_be());
                            }
                        }
                    }

                    //Pad row to a multiple of 4 bytes
                    bytes.resize(utility::round_to_next_multiple_of_4(bytes.len() as i32), 0_u8);
                    pixel_bytes.append(&mut bytes);
                }
            },
            BitmapPixelData::Colors(ref colors) => {
                let bytes_per_pixel = f32::ceil((value.info_header.bit_depth as f32) / 8_f32) as usize;

                for scanline in colors.chunks_exact(width) {
                    let mut bytes: Vec<u8> = Vec::new();

                    for color in scanline {
                        let color_u32 = color.as_u32(false);
                        let mut color_bytes = Vec::from(color_u32.to_le_bytes());
                        color_bytes.truncate(bytes_per_pixel);
                        bytes.append(&mut color_bytes);
                    }

                    //Pad row to a multiple of 4 bytes
                    bytes.resize(utility::round_to_next_multiple_of_4(bytes.len() as i32), 0_u8);
                    pixel_bytes.append(&mut bytes);
                }
            }
        }

        //Concatenate all of the bitmap's bytes together
        Ok([value.header.signature.to_le_bytes().as_slice(), 
            value.header.file_size.to_le_bytes().as_slice(),
            value.header.reserved.to_le_bytes().as_slice(),
            value.header.data_offset.to_le_bytes().as_slice(),
            value.info_header.size.to_le_bytes().as_slice(),
            value.info_header.width.to_le_bytes().as_slice(),
            value.info_header.height.to_le_bytes().as_slice(),
            value.info_header.planes.to_le_bytes().as_slice(),
            value.info_header.bit_depth.to_le_bytes().as_slice(),
            value.info_header.compression.to_le_bytes().as_slice(),
            value.info_header.image_size.to_le_bytes().as_slice(),
            value.info_header.x_pixels_per_meter.to_le_bytes().as_slice(),
            value.info_header.y_pixels_per_meter.to_le_bytes().as_slice(),
            value.info_header.colors_used.to_le_bytes().as_slice(),
            value.info_header.important_colors.to_le_bytes().as_slice(),
            &value.color_table.colors.iter()
                .flat_map(|color| (color.as_u32(false)).to_le_bytes())
                .collect::<Vec<u8>>(),
            &pixel_bytes].concat())
    }
}

///
/// Build an image in bmp format from a grid of pixels and
/// some additional metadata
///
impl ConvertableFrom<Image> for Bitmap {
    type Options = BitmapConvertData;
    type Error = String;

    #[allow(unused_variables)]
    fn try_convert_from(value: Image, options: Self::Options) -> Result<Self, Self::Error> {
        
        let mut color_table: HashMap<u32, u8> = HashMap::new();
        let mut color_table_colors: Vec<color::ARGB> = Vec::new();

        let pixels: BitmapPixelData = if [1, 4, 8].contains(&options.bit_depth) {
            //For bit depth of 1, 4, or 8, construct the color table and set pixels to be indices into the color table
            let mut color_table_indices: Vec<u8> = Vec::new();

            for (rownum, row) in value.iter().enumerate() {
                for pixel in row {
                    let pixel_u32 = pixel.as_u32(true);
                    let color_table_len = color_table.len() as u8;

                    if let Entry::Vacant(e) = color_table.entry(pixel_u32) {
                        e.insert(color_table_len);
                        color_table_colors.push(*pixel);
                    }
    
                    color_table_indices.push(*color_table.get(&pixel_u32).unwrap());
                }
            }

            BitmapPixelData::Indices(color_table_indices)
        }
        else {
            //For any other bit depth, the color table isn't necessary, and the pixel data will be the literal (A)RGB values
            let img_pixels: Vec<color::ARGB> = value.iter()
                .rev()
                .flat_map(|r| r.iter().copied())
                .collect();

            // let mut img_pixels: Vec<color::ARGB> = Vec::new();

            // //Loop over each row and push to vector
            // for j in 0..value.height {
            //     //Bitmap is mirrored horizontally
            //     //let j = value.height - 1 - r;

            //     //Loop over each column
            //     for i in 0..value.width {          
            //         //Get the pixel at the given index (i, j)
            //         let pixel = value.get(i, j).unwrap_or_default();
            //         img_pixels.push(pixel);
            //     }
            // }

            BitmapPixelData::Colors(img_pixels)
        };

        let data_offset: u32 = bitmap::HEADER_SIZE + bitmap::INFO_HEADER_SIZE + (bitmap::COLOR_TABLE_SIZE_FACTOR * color_table.len() as u32);
        
        //The size of the actual pixel data is the number of bytes per pixel times the number of pixels in a row (rounded to a multiple of 4 for padding),
        //times the number of rows
        let bytes_per_pixel = f32::ceil((options.bit_depth as f32) / 8_f32) as usize;
        let image_size = (utility::round_to_next_multiple_of_4((value.width * bytes_per_pixel) as i32) * value.height) as u32;

        Ok(Bitmap { 
            header: BitmapHeader { 
                signature: bitmap::SIGNATURE, 
                file_size: data_offset + image_size,
                reserved: 0_u32,
                data_offset
            }, 
            info_header: BitmapInfoHeader { 
                size: bitmap::INFO_HEADER_SIZE, 
                width: value.width as i32, 
                height: value.height as i32, 
                planes: 1, 
                bit_depth: options.bit_depth, 
                compression: options.compression, 
                image_size: 0_u32, 
                x_pixels_per_meter: options.x_pixels_per_meter,
                y_pixels_per_meter: options.y_pixels_per_meter, 
                colors_used: color_table.len() as u32, 
                important_colors: 0_u32
            }, 
            color_table: BitmapColorTable { 
                colors: color_table_colors
            }, 
            pixels: BitmapPixels { 
                pixels
            }
        })
    }
}

///
/// Build a grid of pixels from an image in bmp format
///
impl ConvertableFrom<Bitmap> for Image {
    type Options = ();
    type Error = String;

    fn try_convert_from(value: Bitmap, _: Self::Options) -> Result<Self, Self::Error> {
        let width = value.info_header.width;
        let height = value.info_header.height;

        let abs_width = width.unsigned_abs();
        let abs_height = height.unsigned_abs();

        let mut pixels: Vec<color::ARGB> = Vec::new();

        //For each row
        for r in 0..abs_height {
            //If height is negative, the image is mirrored horizontally
            let j = match height {
                h if h < 0 => (abs_height - 1) - r,
                _ => r
            };

            //For each column
            for c in 0..abs_width {
                //If width is negative, the image is mirrored vertically
                let i = match width {
                    w if w < 0 => (abs_width - 1) - c,
                    _ => c
                };

                //The bitmap pixels are a flat array; calculate index based off of row and column
                let index = (abs_width * (abs_height - j - 1) + i) as usize;

                //bpp = 1, 4 or 8: each value is an index in the color table
                //otherwise, it is the color values of the pixel    
                let color = match value.pixels.pixels {
                    BitmapPixelData::Indices(ref indices) => value.color_table.colors[indices[index] as usize],
                    BitmapPixelData::Colors(ref colors) => colors[index]
                };

                //Add the pixel to the grid
                pixels.push(color);
            }
        }

        Ok(Image::new_pixels(abs_width as usize, abs_height as usize, pixels))
    }
}
