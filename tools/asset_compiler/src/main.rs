use std::fs::File;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use image::{GenericImageView, DynamicImage, Rgba};

fn main() {
    let client_assets_dir = "../../backlands-client/src/assets";
    let output_dir = "../../backlands-client/src/assets"; // we will write spr/dat here

    let tiles_img = image::open(format!("{}/tiles.png", client_assets_dir)).expect("Failed to load tiles.png");
    let char_img = image::open(format!("{}/character.png", client_assets_dir)).expect("Failed to load character.png");

    let mut spr_file = File::create(format!("{}/backlands.spr", output_dir)).unwrap();
    let mut dat_file = File::create(format!("{}/backlands.dat", output_dir)).unwrap();

    // -- SPRITE FILE --
    // Header: "BACK" signature
    spr_file.write_u32::<LittleEndian>(0x4241434B).unwrap();

    let mut sprites: Vec<Vec<u8>> = Vec::new();
    let mut sprite_dimensions: Vec<(u16, u16)> = Vec::new();

    // 1. Process tiles.png (4 tiles, 32x32 each)
    let tiles_width = tiles_img.width(); // should be 128
    let tiles_height = tiles_img.height(); // should be 32
    
    for x in (0..tiles_width).step_by(32) {
        let mut pixels = Vec::new();
        for py in 0..32 {
            for px in 0..32 {
                if x + px < tiles_width && py < tiles_height {
                    let p = tiles_img.get_pixel(x + px, py);
                    pixels.extend_from_slice(&p.0);
                } else {
                    pixels.extend_from_slice(&[0, 0, 0, 0]); // transparent fallback
                }
            }
        }
        sprites.push(pixels);
        sprite_dimensions.push((32, 32));
    }

    // 2. Process character.png (3 rows, 6 frames per row, 16x32 each)
    let char_width = char_img.width(); // should be 96
    let char_height = char_img.height(); // should be 96

    for y in (0..char_height).step_by(32) {
        for x in (0..char_width).step_by(16) {
            let mut pixels = Vec::new();
            for py in 0..32 {
                for px in 0..16 {
                    if x + px < char_width && y + py < char_height {
                        let p = char_img.get_pixel(x + px, y + py);
                        pixels.extend_from_slice(&p.0);
                    } else {
                        pixels.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }
            }
            sprites.push(pixels);
            sprite_dimensions.push((16, 32));
        }
    }

    // Write sprite count
    spr_file.write_u16::<LittleEndian>(sprites.len() as u16).unwrap();

    // Write all sprites
    for i in 0..sprites.len() {
        let (w, h) = sprite_dimensions[i];
        spr_file.write_u16::<LittleEndian>(w).unwrap();
        spr_file.write_u16::<LittleEndian>(h).unwrap();
        spr_file.write_all(&sprites[i]).unwrap();
    }

    // -- DATA FILE --
    // Header: "DATA" signature
    dat_file.write_u32::<LittleEndian>(0x44415441).unwrap();

    // We have 4 ground tiles and 3 characters
    let item_count = 4 + 3; 
    dat_file.write_u16::<LittleEndian>(item_count as u16).unwrap();

    // Tiles (ID 0 to 3)
    for i in 0..4 {
        dat_file.write_u8(0).unwrap(); // type 0 = Tile
        dat_file.write_u16::<LittleEndian>(i as u16).unwrap(); // sprite_index
    }

    // Characters (ID 4 to 6)
    // Each character has 6 frames
    let mut char_sprite_index = 4; // characters start at sprite index 4
    for _ in 0..3 {
        dat_file.write_u8(1).unwrap(); // type 1 = Character
        dat_file.write_u16::<LittleEndian>(char_sprite_index as u16).unwrap(); // base sprite_index
        dat_file.write_u8(6).unwrap(); // frames
        char_sprite_index += 6;
    }

    println!("Compiled {} sprites.", sprites.len());
    println!("Saved backlands.spr and backlands.dat");
}
