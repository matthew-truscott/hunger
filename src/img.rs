use super::roster;

use std::fs;
use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageBuffer, RgbaImage, Rgba};
use image::GenericImage;
use image::io::Reader;
use rusttype::{point, Font, Scale};
use std::cmp;

// pub fn mount_image<T: GenericImageView>(
//     (x, y): (u32, u32),
//     input: T,
//     clearValue: T::Pixel,
// ) -> ImageBuffer<T::Pixel, Vec<<T::Pixel as Pixel>::Subpixel>>
// where
//     T::Pixel: 'static,
// {
//     let mut imageBuffer = ImageBuffer::from_pixel(x, y, clearValue);
//     imageBuffer.copy_from(&input, 0, 0);
//     imageBuffer
// }

static AVATAR_DIMENSION: u32 = 128;
static AVATAR_PADDING: u32 = 32;
static GLYPH_PADDING: u32 = 64;
static MAX_IMAGE_WIDTH: u32 = 1200;

pub fn get_current_dir() -> PathBuf {
    std::env::current_dir().ok().expect("Something went wrong trying to read in the current directory (check permissions)")
}

pub fn init_thumbs(roster: &roster::Roster) {
    for i in 0..roster.len() {
        let image_name = roster.get_avatar(i).expect("Failed to read image name");
        println!("saving {} as thumbnail...", image_name);
        let avatar_reader = match roster.get_avatar(i) {
            Some(avatar_path) => {
                let mut input_path: PathBuf = get_current_dir();
                input_path.push("input");
                input_path.push(Path::new(&avatar_path));
                match Reader::open(input_path) {
                    Ok(result) => Some(result),
                    Err(error) => {
                        println!("error: {}", error);
                        None
                    }
                }
            },
            None => None
        };
    
        let avatar: DynamicImage = match avatar_reader {
            Some(reader) => {
                match reader.decode() {
                    Ok(result) => result.thumbnail(AVATAR_DIMENSION, AVATAR_DIMENSION),
                    Err(_) => {
                        println!("warning: avatar image could not be decoded");
                        DynamicImage::new_rgba8(AVATAR_DIMENSION, AVATAR_DIMENSION)
                    }
                }
            },
            None => DynamicImage::new_rgba8(AVATAR_DIMENSION, AVATAR_DIMENSION)
        };

        match fs::create_dir("thumbs") {
            Ok(_) => (),
            Err(_) => ()
        };

        let truncated_image_name: &str = match Path::new(&image_name).file_stem() {
            Some(stem) => stem.to_str().expect("Path to string conversion failed!"),
            None => &image_name
        };
        let save_str: String = format!("thumbs/{}.png", truncated_image_name);
        let save_path: &Path = Path::new(&save_str);
        if save_path.exists() {
            continue;
        }
        match avatar.save(save_path) {
            Ok(_) => (),
            Err(_) => println!("Saving the thumbnail failed!")
        };
    };
}

pub fn get_thumb(avatar_path: String) -> DynamicImage {
    let avatar_reader = match Reader::open(format!("{}", avatar_path)) {
        Ok(result) => Some(result),
        Err(error) => {
            println!("error: {}", error);
            None
        }
    };
    let avatar: DynamicImage = match avatar_reader {
        Some(reader) => {
            match reader.decode() {
                Ok(result) => result,
                Err(_) => {
                    println!("warning: avatar image could not be decoded");
                    DynamicImage::new_rgba8(AVATAR_DIMENSION, AVATAR_DIMENSION)
                }
            }
        },
        None => DynamicImage::new_rgba8(AVATAR_DIMENSION, AVATAR_DIMENSION)
    };
    avatar
}

pub fn image(text_input: String, game_roster: &roster::Roster, action_members: &Vec<usize>, idx: &u32) {
    // Load the font
    let mut font_path: PathBuf = get_current_dir();
    font_path.push("fonts");
    // TODO give user power to change this
    font_path.push("Roboto-Regular.ttf");
    let font_data = fs::read(font_path).expect("Error reading font data");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_vec(font_data).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(16.0);

    // The text to render
    let text = text_input.as_str();

    // Use a dark red colour
    let colour = (150, 0, 0);
    let bg_colour = (255, 255, 255, 255);

    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, 0.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let number_avatar_horizontal: u32 = (MAX_IMAGE_WIDTH - AVATAR_PADDING) / (AVATAR_DIMENSION + AVATAR_PADDING);
    let number_rows: u32 = (action_members.len() as u32) / number_avatar_horizontal + 1;
    let number_columns: u32 = cmp::min(action_members.len() as u32, number_avatar_horizontal);

    let avatar_block_width: u32 = (AVATAR_DIMENSION + AVATAR_PADDING) * number_columns + AVATAR_PADDING;
    let avatar_block_height: u32 = if action_members.len() == 0 {
        GLYPH_PADDING
    } else {
        (AVATAR_DIMENSION + AVATAR_PADDING) * number_rows + AVATAR_PADDING
    };
    let glyph_block_width: u32 = glyphs_width + (2 * GLYPH_PADDING);

    let image_height: u32 = avatar_block_height + glyphs_height + GLYPH_PADDING;
    let image_width: u32 = if avatar_block_width > glyph_block_width {
        avatar_block_width
    } else {
        glyph_block_width
    };
    let avatar_block_left: u32 = if avatar_block_width > glyph_block_width {
        AVATAR_PADDING
    } else {
        (image_width - avatar_block_width) / 2 - 1 + AVATAR_PADDING
    };
    let glyph_block_left: u32 = if avatar_block_width > glyph_block_width {
        (image_width - glyphs_width) / 2 - 1
    } else {
        GLYPH_PADDING
    };

    // Create a new rgba image with some padding
    //let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();
    let mut fullimage: RgbaImage = ImageBuffer::from_pixel(
        image_width, 
        image_height,
        Rgba([bg_colour.0, bg_colour.1, bg_colour.2, bg_colour.3]));

    // Load in image from file
    for (i, a) in action_members.iter().enumerate() {
        let image_name = game_roster.get_avatar(*a).expect("Failed to read image name");
        let truncated_image_name: &str = match Path::new(&image_name).file_stem() {
            Some(stem) => stem.to_str().expect("Path to string conversion failed!"),
            None => &image_name
        };
        let image_file_str: String = format!("thumbs/{}.png", truncated_image_name);
        let avatar = get_thumb(image_file_str);
        let px: u32 = (i as u32) % number_columns;
        let py: u32 = (i as u32) / number_columns;
        println!("{}, {}", px, py);
        match fullimage.copy_from(&avatar,
                avatar_block_left + (AVATAR_PADDING + AVATAR_DIMENSION) * px, 
                AVATAR_PADDING + (AVATAR_PADDING + AVATAR_DIMENSION) * py) {
            Ok(_) => (),
            Err(_) => ()
        };
    }

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                // Turn the coverage into an alpha value
                let r = (colour.0 as f32 * (v) + bg_colour.0 as f32 * (1.0 - v)) as u8;
                let g = (colour.1 as f32 * (v) + bg_colour.1 as f32 * (1.0 - v)) as u8;
                let b = (colour.2 as f32 * (v) + bg_colour.2 as f32 * (1.0 - v)) as u8;
                let a = (255.0) as u8;
                fullimage.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + glyph_block_left + bounding_box.min.x as u32,
                    y + avatar_block_height + bounding_box.min.y as u32,

                    Rgba([r, g, b, a]),
                )
            });
        }
    }

    //for x in image.width() {
    //    for y in image.height() {
    //        let text_pix = image.get_pixel(x, y);
    //        let root_pix = fullimage.get_pixel_mut(x, y);
            //root_pix.
    //    }
    //}

    // Save the image to a png file
    match fs::create_dir("output") {
        Ok(_) => (),
        Err(_) => ()
    };
    let mut output_path: PathBuf = get_current_dir();
    let fullimage_name: String = format!("hg{:03}.png", idx);
    output_path.push("output");
    output_path.push(Path::new(fullimage_name.as_str()));
    fullimage.save(output_path).unwrap();
}
