use image::GenericImageView;

use crate::file_format::FileFormat;
use crate::initializer::Initializer;
use crate::ui::UI;

mod capture;
mod character;
mod character_style;
mod common;
mod condition;
mod executable;
mod file_format;
mod initializer;
mod section;
mod switcher;
mod text_input;
mod traits;
mod ui;

fn main() {
    // println!(
    //     "{}",
    //     get_image(
    //         r"C:\Users\ew0nd\Desktop\Screenshot 2022-09-07 235742.png",
    //         6
    //     )
    // );

    handle_yaml();
}
fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    ascii[index as usize]
}

fn get_image(dir: &str, scale: u32) -> String {
    let mut output = String::from("");
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent = (pix[0] / 3 + pix[1] / 3 + pix[2] / 3);
                if pix[3] == 0 {
                    intent = 0;
                }
                output += get_str_ascii(intent);
            }
        }
        if y % (scale * 2) == 0 {
            output += "\n";
        }
    }
    output
}

fn handle_yaml() {
    const ROOT: &str = r"C:\Users\ew0nd\Documents\DialogGame\story1";
    let mut initializer = Initializer::new(ROOT.to_owned(), FileFormat::Yaml);
    initializer.execute();
}
