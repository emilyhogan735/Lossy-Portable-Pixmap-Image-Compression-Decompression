use csc411_image::{Read, Rgb, RgbImage, Write};
use array2::Array2;
use csc411_rpegio;
use crate::trimming;
use crate::packing;
use crate::unpacking;

/// function that compresses a ppm image into a rpeg image
/// 
/// # Arguments:
/// * `filename`: A string option
pub fn compress(filename: Option<&str>) {

    // Read a PPM image from a file specified on the command line or from standard input
    let img = RgbImage::read(filename.as_deref()).unwrap();
    // Put pixels in an Array2
    let pixel_array = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();

    // If necessary, trim the last row, column, or both row and column of the image so that the width and height of your image are even numbers
    let trimmed_array = trimming::trim(pixel_array);

    // Change to a floating-point representation by dividing by the image denominator
    // map transform pixels from an Array2<T> to an Array2<U> which contains pixels of pixel_array.map(|pixel| pixel);
    let array2_as_floats = trimmed_array.map(|pixel| (pixel.red as f32 / 255.0, pixel.green as f32 / 255.0, pixel.blue as f32 / 255.0) );

    // transform each pixel from RGB color space into component video color space (Y/PB/PR) using map function
    let array2_as_ypbpr = array2_as_floats.map(|(red, green, blue)| ({
        let y = 0.299 * red + 0.587 * green + 0.114 * blue; 
        let pb = -0.168736 * red - 0.331264 * green + 0.5 * blue;
        let pr = 0.5 * red - 0.418688 * green - 0.081312 * blue;
        (y, pb, pr)
    }) );

    // Pack each 2-by-2 block into a 32-bit word
    let word_vec = packing::pack(array2_as_ypbpr.clone());

    // Generate rpeg data by compressing a .ppm
    let width = array2_as_ypbpr.width();
    let height = array2_as_ypbpr.height();
    let words = word_vec;
    let compressed_data: Vec<[u8; 4]> = words.into_iter().map(u32::to_be_bytes).collect();

    // Output the rpeg image data to stdout
    csc411_rpegio::output_rpeg_data(&compressed_data, width, height).unwrap();

}


/// function that decompresses a rpeg image into a ppm image
/// 
/// # Arguments:
/// * `filename`: A string option
pub fn decompress(filename: Option<&str>) {

    // Read in the PPM header containing the width, height, and compressed data of the image
    let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(filename).unwrap();

    // Interpret the `[u8; 4]`'s as big-endian `u32`s and collect them into a vector
    let word_vec: Vec<u32> = compressed_data.into_iter().map(u32::from_be_bytes).collect();

    // Unpack each 32-bit word into a Vec containing the Y/Pb/Pr values in each 2x2 block of pixels in the image
    let ypbpr_data = unpacking::unpack(word_vec, width, height);

    // Create new Array2 containing Y/Pb/Pr values as floats
    let array2_as_ypbpr = Array2::from_row_major(width, height, ypbpr_data.into_iter().map(|ypbpr| ypbpr.unwrap()).collect()).unwrap();

    // transform each pixel from component video color space (Y/PB/PR) into RGB color space using map function
    let array2_as_ppm = array2_as_ypbpr.map(|(y, pb, pr)| ({
        let red = 1.0 * y + 0.0 * pb + 1.402 * pr;
        let green = 1.0 * y - 0.344136 * pb - 0.714136 * pr;
        let blue = 1.0 * y + 1.772 * pb + 0.0 * pr;
        (red, green, blue)
    }) );

    // convert from floats to RGB values
    let array2_as_rbg = array2_as_ppm.map(|(red, green, blue)| Rgb{red: (red * 255.0) as u16, green: (green * 255.0) as u16, blue: (blue * 255.0) as u16});

    // Write decompressed binary image to standard output using csc411_image
    let rgb_image = RgbImage {
        pixels: array2_as_rbg.iter_row_major().map(|(_, _, pix)| pix.clone()).collect(),
        width: array2_as_rbg.width() as u32, // not sure if `as u32` is necessary
        height: array2_as_rbg.height() as u32,
        denominator: 255
    };

    // output decompressed image data to standard output
    rgb_image.write(None).unwrap();

}