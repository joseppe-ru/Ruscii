use std::ascii::AsciiExt;
use std::io::Cursor;
use image::{imageops, GenericImage, GenericImageView, ImageBuffer, ImageReader, Rgba};
use image::{open, Rgb, Luma};

use std::fs::File;
use std::io::{self, Write};
use std::str::Chars;
use image::imageops::resize;
use imageproc;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::gradients::{vertical_sobel,horizontal_sobel};
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

//reduced dimensions
const RWIDTH:u32= 64u32;
const RHEIGHT:u32= 64u32;


fn main() {
    let now = std::time::Instant::now();
    let img_raw = image::open("../images/digital-home.png").unwrap();
    //let img_raw = img_raw.resize(128, 128, imageops::FilterType::Gaussian);
    println!("Open Image; Time: {:?}", now.elapsed());

    // graues ild, genauer brauch ichs nicht
    let grey_img = imageops::colorops::grayscale(&img_raw);
    println!("grayscale; Time: {:?}", now.elapsed());

    // Glättung
    let blured_img = gaussian_blur_f32(&grey_img, 1.4);
    blured_img.save("../images/blured.png").unwrap();
    println!("Blured; Time: {:?}", now.elapsed());

    // Feature Image - sind die erkannten features drauf eingezechnet
    //let mut feature_img = img_raw.clone();
    let mut feature_img = img_raw.resize(RWIDTH, RHEIGHT, imageops::FilterType::Gaussian);

    // Merkmals extraktion Kanten mittels Kenny
    let edges = canny(&grey_img, 50.0, 80.0);
    edges.save("../images/edges.png").unwrap();
    println!("Detected Edges; Time: {:?}", now.elapsed());

    // Anstiege mittels Sobel-operator berechnen
    let grad_x = horizontal_sobel(&blured_img);
    let grad_y = vertical_sobel(&blured_img);
    println!("Gradienten; Time: {:?}", now.elapsed());

    //let edges = canny(&grey_img, 50.0, 100.0);
    let edges = resize(&edges, RWIDTH, RHEIGHT, imageops::FilterType::Triangle); // triangle am prziesesten // CatmullRom hat besser olückenerlkennung
    let grad_x = resize(&grad_x, RWIDTH, RHEIGHT, imageops::FilterType::Triangle);
    let grad_y = resize(&grad_y, RWIDTH, RHEIGHT, imageops::FilterType::Triangle);
    println!("resizing; Time: {:?}", now.elapsed());

    //kanten in die Feature map einzeichen
    let edge_color = Rgba([255u8, 0u8, 0u8, 255u8]); // Rote Farbe für die Kanten
    for (x, y, pixel) in edges.enumerate_pixels() {
        if pixel[0] > 0 {
            feature_img.put_pixel(x, y, edge_color);
        }
    }
    feature_img.save("../images/features.png").unwrap();
    println!("Draw Edges; Time: {:?}", now.elapsed());


    let mut ascii_buffer: String = String::new();
    // asciiart erstellen
    draw_ascii_gradients(&mut ascii_buffer, edges, grad_x, grad_y);



/*
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
*/




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

fn draw_ascii_gradients(buffer: &mut String, edges: ImageBuffer<Luma<u8>,Vec<u8>>, grad_x: ImageBuffer<Luma<i16>,Vec<i16>>, grad_y:ImageBuffer<Luma<i16>,Vec<i16>>) {
    for y in 0..RHEIGHT {
        for x in 0..RWIDTH {
            // Prüfen, ob an dieser Stelle eine Kante ist
            if edges.get_pixel(x, y)[0] > 0 {
                // Kante gefunden! Jetzt die Richtung bestimmen.
                let gx = grad_x.get_pixel(x, y)[0] as f32;
                let gy = grad_y.get_pixel(x, y)[0] as f32;

                // Gradientenwinkel -> Kantenwinkel (+90 Grad)
                let gradient_angle_rad = gy.atan2(gx);
                let edge_angle_deg = gradient_angle_rad.to_degrees() + 90.0;

                // Passendes Zeichen für den Winkel auswählen
                let symbol = angle_to_char(edge_angle_deg);
                buffer.push(symbol);
            } else {
                // Keine Kante, also Leerzeichen
                buffer.push(' ');
            }
        }
        buffer.push('\n');
    }
}

fn avg_to_char(avg:u8) ->char{
    let ordered_ascii = String::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`'.");
    // map pixels to ascii-charackter
    let elem_n = (avg as f32 / u8::MAX as f32 * ordered_ascii.len() as f32) as usize;
    // output ascii char
    ordered_ascii.chars().nth(elem_n).unwrap()
}



/// Wandelt einen Kantenwinkel (in Grad) in ein passendes ASCII-Zeichen um.
fn angle_to_char(angle_deg: f32) -> char {
    // Normalisiere den Winkel auf den Bereich [0, 360)
    let normalized_angle = (angle_deg % 360.0 + 360.0) % 360.0;

    // Teile den Kreis in 8 Sektoren (jeweils 45° breit) und weise ein Zeichen zu.
    // Wir zentrieren die Sektoren um die Hauptrichtungen (0°, 45°, 90°, 135°).
    if (normalized_angle >= 337.5) || (normalized_angle < 22.5) {
        '-' // Horizontale Kante
    } else if normalized_angle < 67.5 {
        '/' // Diagonale Kante (unten links nach oben rechts)
    } else if normalized_angle < 112.5 {
        '|' // Vertikale Kante
    } else if normalized_angle < 157.5 {
        '\\' // Diagonale Kante (oben links nach unten rechts)
    } else if normalized_angle < 202.5 {
        '-' // Horizontale Kante
    } else if normalized_angle < 247.5 {
        '/' // Diagonale Kante (unten links nach oben rechts)
    } else if normalized_angle < 292.5 {
        '|' // Vertikale Kante
    } else { // normalized_angle < 337.5
        '\\' // Diagonale Kante (oben links nach unten rechts)
    }
}
