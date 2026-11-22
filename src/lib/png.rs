// import
use image::{ImageBuffer, Rgb};


pub fn write_png(pixel_list: Vec<[u8; 3]>, output_file: String){
    // define png dimention
    let width = pixel_list.len() as u32;
    let height = 1;

    // convert for png
    let raw_bytes: Vec<u8> = pixel_list.iter().flatten().copied().collect();

    // create image
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, raw_bytes)
            .expect("Buffer size does not match image dimensions");

    img.save(output_file).expect("Failed to save PNG");
}
