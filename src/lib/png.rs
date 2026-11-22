// import
use image::{ImageBuffer, Rgb};
use image::ImageReader;


pub fn write_png(pixel_list: Vec<[u8; 3]>, output_file: String){
    // make dimensions square
    let side = (pixel_list.len() as f64).sqrt().ceil().max(1.0) as u32;
    let width = side;
    let height = side;

    // convert for png
    let mut raw_bytes: Vec<u8> = pixel_list.iter().flatten().copied().collect();
    raw_bytes.resize(side as usize * side as usize * 3, 0);

    // create image
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, raw_bytes)
            .expect("Buffer size does not match image dimensions");

    img.save(output_file).expect("Failed to save PNG");
}


pub fn read_png(input_file: String) -> Vec<[u8; 3]>{
    // open img
    let img = ImageReader::open(input_file)
        .expect("Erreur")
        .decode()
        .expect("Erreur")
        // only 8 bytes rgb
        .to_rgb8();


    // get pixels
    let pixel_list: Vec<[u8; 3]> = img
        .pixels()
        .map(|p| p.0)
        .collect();


    return pixel_list;
}