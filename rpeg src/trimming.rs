use array2::Array2;
use csc411_image::Rgb;

/// function to trim array of pixel data if the rows/columns are odd numbered, returns an Array2<Rgb>
/// 
/// # Arguments:
/// * `pixel_array`: An array2 containing RGB values
pub fn trim(pixel_array: Array2<Rgb>) -> Array2<Rgb> {

    // initialize array
    let mut trimmed_array;
    if (pixel_array.width() % 2 == 1) && (pixel_array.height() % 2 == 1) {
        // create a new Array2 with the trimmed dimentions
        trimmed_array = Array2::new((pixel_array.width() - 1) as usize, (pixel_array.height() - 1) as usize, Rgb{red: 0, green: 0, blue: 0});
    }  
    else if pixel_array.width() % 2 == 1 {
        // create a new Array2 with the trimmed dimentions
        trimmed_array = Array2::new((pixel_array.width() - 1) as usize, pixel_array.height() as usize, Rgb{red: 0, green: 0, blue: 0});
    }
    else if pixel_array.height() == 1 {
        // remove last row and change the height
        trimmed_array = Array2::new(pixel_array.width() as usize, (pixel_array.height() - 1) as usize, Rgb{red: 0, green: 0, blue: 0});
    }
    else {
        trimmed_array = pixel_array.clone();
    }

    // transfer pixel data into trimmed array
    for (col, row, pixel) in pixel_array.iter_row_major() {
        let new_pixel = trimmed_array.get_mut(col, row);
        // skip pixel if out of bounds
        if new_pixel.is_none() {
            continue;
        }
        // update pixel value
        *new_pixel.unwrap() = pixel.clone();
    }
    return trimmed_array;
}