use image::{DynamicImage, ImageBuffer, RgbaImage, Rgba};
use image::GenericImage;
use rusttype::{point, Font, Scale};

pub fn image(text_input: String) {
    // Load the font
    let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

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

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();
    let mut fullimage: RgbaImage = ImageBuffer::from_pixel(
        glyphs_width + 40, 
        glyphs_height + 40,
        Rgba([bg_colour.0, bg_colour.1, bg_colour.2, bg_colour.3]));

    // set background colour


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
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,

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
    fullimage.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}
