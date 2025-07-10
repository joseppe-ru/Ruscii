use std::ascii::AsciiExt;
use std::io::Cursor;
use image::{imageops, GenericImage, GenericImageView, ImageReader, Rgba};
use image::{open, Rgb, Luma};

use std::fs::File;
use std::io::{self, Write};
use std::str::Chars;
use imageproc;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
/*
// A HashMap to store color codes for easy access.
let colors: HashMap<&str, &str> = [
// The universal reset code to stop any color.
("reset", "\x1b[0m"),

// Standard Colors
("black", "\x1b[30m"),
("red", "\x1b[31m"),
("green", "\x1b[32m"),
("yellow", "\x1b[33m"),
("blue", "\x1b[34m"),
("magenta", "\x1b[35m"),
("cyan", "\x1b[36m"),
("white", "\x1b[37m"),

// Bright (High Intensity) Colors
("bright_black", "\x1b[90m"), // Often appears as gray
("bright_red", "\x1b[91m"),
("bright_green", "\x1b[92m"),
("bright_yellow", "\x1b[93m"),
("bright_blue", "\x1b[94m"),
("bright_magenta", "\x1b[95m"),
("bright_cyan", "\x1b[96m"),
("bright_white", "\x1b[97m"),
].iter().cloned().collect();

// --- Example Usage ---

// Get the codes from the HashMap.
let red_start = colors.get("red").unwrap();
let green_start = colors.get("green").unwrap();
let reset_code = colors.get("reset").unwrap();

// Print multiple colors on a single line.
println!(
    "Default text, then {}{}{}, then {}{}{}, and back to default.",
    red_start,   // Start red
    "this is red",
    reset_code,  // Stop red
    green_start, // Start green
    "this is green",
    reset_code   // Stop green
);
*/




fn main() {
    let path ="../images/digital-home.png";
    let img_raw = image::open(path).unwrap();
    let img_raw = img_raw.resize(128, 128, imageops::FilterType::Gaussian);


    // Graustufenbild
    let grayscale = imageops::colorops::grayscale(&img_raw);

    // Glättung
    let blured = gaussian_blur_f32(&grayscale, 0.1);

    // Merkmals extraktion Kanten mittels Kenny
    let edges = canny(&blured, 50.0, 80.0);
    //let edges = canny(&grayscale, 50.0, 100.0);
    let mut feature_map = img_raw.clone();
    //let mut feature_map = img_raw.resize(128, 128, imageops::FilterType::Gaussian);
    let edge_color = Rgba([255u8, 0u8, 0u8, 255u8]); // Rote Farbe für die Kanten

    // Zeichnen Sie die erkannten Kanten auf das Ausgabebild
    for (x, y, pixel) in edges.enumerate_pixels() {
        if pixel[0] > 0 {
            feature_map.put_pixel(x, y, edge_color);
        }
    }
    // Save the new grayscale image
    feature_map.save("../images/features.png");
    blured.save("../images/blured.png").unwrap();
    edges.save("../images/edges.png").unwrap();



    //let (width, height) = grayscale.dimensions();
    //let mut feature_map = image::ImageBuffer::new(width, height);





    let mut ascii_buffer: String = String::new();

    //let ascii_map = imageops::resize(&edges,128, 128, imageops::FilterType::Gaussian);
    let ascii_map = edges.clone();
    for (x, y, pixel) in ascii_map.enumerate_pixels() {
        if x==0 {ascii_buffer.push('\n')}
        if pixel[0] == 0 {
            ascii_buffer.push(' ');
        }
        else {
            //let c = get_avg_ascii_char(pixel[0]);
            ascii_buffer.push('#');
        }
    }





    // schreiben die average
    //let mut file = File::create("../images/output.txt").unwrap();
    //let text_string: String = digit_buffer.iter().collect();
    //file.write_all(text_string.as_bytes()).unwrap();
    //file.write_all(b"\n"); // Neue Zeile nach jeder Zeile


    // schreiben den ascii
    let mut file = File::create("../images/ascii.txt").unwrap();
    file.write_all(ascii_buffer.as_bytes()).unwrap();
    file.write_all(b"\n");


}

fn get_avg_ascii_char(avg:u8)->char{
    let ordered_ascii = String::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`'.");
    // map pixels to ascii-charackter
    let elem_n = (avg as f32 / u8::MAX as f32 * ordered_ascii.len() as f32) as usize;
    // output ascii char
    ordered_ascii.chars().nth(elem_n).unwrap()
}