use super::roster;

use std::path::Path;
use std::fs;
use image::{DynamicImage, ImageBuffer, RgbaImage, Rgba};
use image::GenericImage;
use image::io::Reader;
use rusttype::{point, Font, Scale};

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

pub fn init_thumbs(roster: &roster::Roster) -> std::io::Result<()> {
    for i in 0..roster.len() {
        let image_name = roster.get_avatar(i).expect("Failed to read image name");
        println!("saving {} as thumbnail...", image_name);
        let avatar_reader = match roster.get_avatar(i) {
            Some(avatar_path) => {
                match Reader::open(format!("input/{}", avatar_path)) {
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
    Ok(())
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
    let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(16.0);

    // The text to render
    let text = text_input.as_str();
    //let text = "test\ntwo";

    // Use a dark red colour
    let colour = (150, 0, 0);
    let bg_colour = (255, 255, 255, 255);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
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

    let avatar_block_width: u32 = (AVATAR_DIMENSION + AVATAR_PADDING) * (action_members.len() as u32) + AVATAR_PADDING;
    let avatar_block_height: u32 = AVATAR_DIMENSION + (2 * AVATAR_PADDING);
    let image_width: u32 = avatar_block_width.max(glyphs_width + (2 * GLYPH_PADDING));
    let image_height: u32 = (2 * AVATAR_PADDING) + AVATAR_DIMENSION + glyphs_height + (2 * GLYPH_PADDING);
    let avatar_block_left: u32 = (image_width / 2) - (avatar_block_width / 2);
    let glyph_left: u32 = ((image_width / 2) - (glyphs_width / 2) - GLYPH_PADDING).max(avatar_block_left);
    println!("{}, {}", image_width, glyph_left);

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
        match fullimage.copy_from(&avatar, avatar_block_left + AVATAR_PADDING + 
                (AVATAR_PADDING + AVATAR_DIMENSION) * (i as u32), AVATAR_PADDING) {
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
                    x + glyph_left + bounding_box.min.x as u32,
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
    let fullimage_name = format!("output/hg{:03}.png", idx);
    fullimage.save(fullimage_name).unwrap();
    println!("Generated: image_example.png");
}
