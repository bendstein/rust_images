use std::collections::HashMap;
use colored::Colorize;
use unicode_segmentation::UnicodeSegmentation;
use rs_image::{color, image};

pub struct DrawImageToConsoleSettings {
    ///
    /// Whether to use truecolor when drawing
    /// to console.
    ///
    pub use_truecolor: bool,
    ///
    /// Strings used to represent different pixel opacities
    /// in the console
    ///
    pub pixels: Vec<String>,
}

impl DrawImageToConsoleSettings {
    fn pixel_width(&self) -> usize {
        if self.pixels.is_empty() {
            0_usize
        } 
        else {
            fn gcd(a: usize, b: usize) -> usize {
                let mut x = a;
                let mut y = b;

                while (x % y) > 0 {
                    let r = x % y;
                    x = y;
                    y = r;
                }

                y
            }

            self.pixels
                .iter()
                .map(|o| o.graphemes(true).count())
                .reduce(gcd)
                .unwrap_or(0)
        }
    }
}

pub fn draw_image_to_console(img: image::Image, settings: &DrawImageToConsoleSettings) {
    let _ = colored::control::set_virtual_terminal(true);

    //Write some top padding
    println!();

    //Outer loops is rows
    for j in 0..img.height {
        //Move to the next line
        println!();

        //Inner loop is columns
        for i in 0..img.width {
            //Get color at location
            let color = img.get(i, j).unwrap_or_default();

            //Get string corresponding to opacity
            let pixel_string = get_pixel_string_from_opacity(color, settings);

            //Get console color from given color
            let coloring = get_coloring(color, settings);

            //Apply console color to pixel string
            let colored_string = if let Some(console_color) = coloring {
                colored::ColoredString::from(&pixel_string[..])
                    .color(console_color)
            }
            else {
                colored::ColoredString::from(&pixel_string[..])
            };

            //Print colored string
            print!("{colored_string}");
        }
    }
}

fn get_pixel_string_from_opacity(color: color::RGBA, settings: &DrawImageToConsoleSettings) -> String {
    let pixel_width = settings.pixel_width();

    if pixel_width == 0 {
        String::from("")
    } 
    else {
        let mut pixel_string_part = if color.alpha == 0 {
            String::from(" ")
        } 
        else {
            let mut pixel_string_part = None;

            let alpha_ratio = (color.alpha as f32) / 255_f32;
            let len = settings.pixels.len() as f32;

            for i in 1..=settings.pixels.len() {
                let lower_bound = (len - (i as f32)) / len;
                let upper_bound = (len - (i as f32) + 1_f32) / len;

                if alpha_ratio > lower_bound && alpha_ratio <= upper_bound {
                    let index = f32::max(0_f32, (i as f32) - 1_f32) as usize;
                    pixel_string_part = Some(settings.pixels[index].clone());
                    break;
                }
            }

            pixel_string_part.unwrap_or_else(|| String::from(""))
        };

        while pixel_string_part.graphemes(true).count() < pixel_width {
            pixel_string_part = format!("{pixel_string_part}{pixel_string_part}");
        }

        pixel_string_part
    }
}

fn get_coloring(color: color::RGBA, settings: &DrawImageToConsoleSettings) -> Option<colored::Color> {
    if color.alpha == 0 {
        None
    }
    else if settings.use_truecolor {
        Some(colored::Color::TrueColor { r: color.red, g: color.green, b: color.blue })
    }
    else {
        let default_color = 0x00000000;
        let simple_colors: HashMap<u32, colored::Color> = HashMap::from([
            (0x00000000, colored::Color::Black), //Black
            (0x00008000, colored::Color::Blue), //Dark blue
            (0x00800000, colored::Color::Green), //Dark green
            (0x00808000, colored::Color::Cyan), //Cark cyan
            (0x80000000, colored::Color::Red), //Dark red
            (0x80008000, colored::Color::Magenta), //Dark magenta
            (0x80800000, colored::Color::Yellow), //Dark yellow
            (0x80808000, colored::Color::White), //Dark grey
            (0x0000FF00, colored::Color::BrightBlue), //Blue
            (0x00FF0000, colored::Color::BrightGreen), //Green
            (0x00FFFF00, colored::Color::BrightCyan), //Cyan
            (0xFF000000, colored::Color::BrightRed), //Red
            (0xFF00FF00, colored::Color::BrightMagenta), //Magenta
            (0xFFFF0000, colored::Color::BrightYellow), //Yellow
            (0xC0C0C000, colored::Color::BrightBlack), //Grey
            (0xFFFFFF00, colored::Color::BrightWhite) //White
        ]);

        let (hex, _) = simple_colors.keys()
            .map(|k| (k, color::RGBA {
                    red: ((k >> 24) & 0xFF) as u8,
                    green: ((k >> 16) & 0xFF) as u8,
                    blue: ((k >> 8) & 0xFF) as u8,
                    alpha: color.alpha,
                }.distance_euclidean(&color)))
            .reduce(|(hex_a, distance_a), (hex_b, distance_b)| {
                if distance_a <= distance_b {
                    (hex_a, distance_a)
                }
                else {
                    (hex_b, distance_b)
                }
            }).unwrap_or((&default_color, 0_f32));

        simple_colors.get(hex).copied()
    }
}