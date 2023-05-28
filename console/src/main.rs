use std::{collections::HashMap, time::SystemTime};

use parse_args::argparser;
use rs_image::*;
use image::format::{bitmap::Bitmap};

fn main() -> Result<(), String> {
    //Parse command line arguments
    let args: HashMap<String, String> = argparser::parse_args_with_opts(
        std::env::args(),
        argparser::ParseArgsSettings::init(
            String::from(constants::ARGUMENT_PREFIX),
            String::from(constants::ARGUMENT_DELIMITER)
        ))
        .map_err(|err| format!("Failed to parse arguments: {}", err.join(", ")))?
        .iter()
        .map(|arg| arg.to_key_value_pair())
        .collect();

    //Get image file path from args
    let file_path = args.get(constants::keys::FILE_PATH)
        .map_or_else(|| Err(format!("Missing required argument: '{}'.", constants::keys::FILE_PATH)), Ok)?;

    //Get image file bytes
    let bytes = utility::file::get_file_bytes(file_path)
        .map_err(|err| err.to_string())?;

    //Parse bytes to bitmap
    let bitmap = Bitmap::try_from(bytes)?;

    let reversed = Vec::try_from(bitmap)?;

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .unwrap_or_default()
        .as_millis();

    let out_path = format!("output/bmp/img{time}.bmp");

    utility::file::write_file_bytes(&out_path, &reversed)
        .map_err(|err| err.to_string())?;

    println!("Wrote file {out_path}");

    Ok(())
}