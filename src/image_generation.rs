use image::{ImageBuffer, Luma};

pub(crate) fn image_generation(data: Vec<u8>) {
    let n = 21; // Matrix dimension
    let block_size = 20; // Each element becomes a 6x6 block
    let border_blocks = 4; // Border thickness in blocks
    let size = n * block_size + 2 * border_blocks * block_size; // Total image size

    // Create a white background image
    let mut img = ImageBuffer::from_fn(size as u32, size as u32, |_, _| Luma([255u8]));

    // Populate the image with scaled matrix data
    for (index, &value) in data.iter().enumerate() {
        let (row, col) = (index / n, index % n);
        let color = if value == 0 { 255u8 } else { 0u8 };
        for y in 0..block_size {
            for x in 0..block_size {
                img.put_pixel(
                    (col * block_size + x + border_blocks * block_size) as u32,
                    (row * block_size + y + border_blocks * block_size) as u32,
                    Luma([color]),
                );
            }
        }
    }

    img.save("output.png").expect("Failed to save the image");
    println!("Image saved as output.png");
}
