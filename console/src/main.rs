mod constants;
mod output_type;
mod console;

use std::{collections::HashMap, time::SystemTime};
use console::DrawImageToConsoleSettings;
use parse_args::argparser;
use rs_image::{*, convert::ConvertableFrom};
use image::format::bitmap::Bitmap;

use crate::output_type::OutputType;

fn main() -> Result<(), String> {
    //Parse command line arguments
    let args: HashMap<String, String> = argparser::parse_args_with_opts(
        std::env::args(),
        argparser::ParseArgsSettings::init(
            String::from(constants::args::ARGUMENT_PREFIX),
            String::from(constants::args::ARGUMENT_DELIMITER)
        ))
        .map_err(|err| format!("Failed to parse arguments: {}", err.join(", ")))?
        .iter()
        .map(|arg| arg.to_key_value_pair())
        .collect();

    //Get output type
    let output_type_arg = args.get(constants::args::keys::OUTPUT_TYPE).unwrap_or(&String::from("")).to_ascii_lowercase();

    let output_type = if output_type_arg == *constants::args::values::output_type::FILE {
        OutputType::WriteToFile
    }
    else if output_type_arg == *constants::args::values::output_type::DRAW {
        OutputType::DrawToConsole
    }
    else {
        OutputType::default()
    };

    //Get image file path from args
    let file_path = args.get(constants::args::keys::FILE_PATH)
        .map_or_else(|| Err(format!("Missing required argument: '{}'.", constants::args::keys::FILE_PATH)), Ok)?;

    //Get image file bytes
    let bytes = rs_image::utility::file::get_file_bytes(file_path)
        .map_err(|err| err.to_string())?;

    //Parse bytes to bitmap
    let bitmap = Bitmap::try_from(bytes)?;

    match output_type {
        OutputType::WriteToFile => {
            let img = image::Image::try_convert_from(bitmap, ())?;

            let bmp = Bitmap::try_convert_from(img, image::format::bitmap::BitmapConvertData {
                bit_depth: 24,
                compression: 0
            })?;

            let reversed = Vec::try_from(bmp)?;

            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap_or_default()
                .as_millis();
        
            let out_path = format!("output/bmp/img{time}.bmp");
        
            rs_image::utility::file::write_file_bytes(&out_path, &reversed)
                .map_err(|err| err.to_string())?;
        
            println!("Wrote file {out_path}");

            Ok(())
        },
        OutputType::DrawToConsole => {
            let truecolor_disabled_arg = args.get(constants::args::keys::FORCE_DISABLE_TRUECOLOR)
                .map_or("", |v| v.as_str());

            let truecolor_env = std::env::var(constants::env::keys::TRUECOLOR_ENABLED).unwrap_or_else(|_| String::from(""));

            let truecolor_enabled = !truecolor_disabled_arg.to_ascii_lowercase().eq(&true.to_string())
                && [constants::env::values::TRUECOLOR_ENABLED_24BIT, 
                    constants::env::values::TRUECOLOR_ENABLED_TRUECOLOR
                ].contains(&truecolor_env.as_str());

            let img = image::Image::try_convert_from(bitmap, ())?;

            let bmp = Bitmap::try_convert_from(img, image::format::bitmap::BitmapConvertData {
                bit_depth: 24,
                compression: 0
            })?;

            let img = image::Image::try_convert_from(bmp, ())?;

            let pixels: Vec<String> = constants::draw_to_console::PIXEL_STRINGS
                .split(constants::draw_to_console::PIXEL_STRINGS_DELIMITER)
                .map(String::from)
                .collect();

            console::draw_image_to_console(img, &DrawImageToConsoleSettings {
                use_truecolor: truecolor_enabled,
                pixels
            });

            Ok(())
        }
    }
}