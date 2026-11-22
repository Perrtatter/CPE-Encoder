// import
use image::{ImageBuffer, Rgb};
use image::ImageReader;


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
