use array2::Array2;
use bitpack::bitpack;

/// function that packs Y/Pb/Pr values into vector of 32bit words, returns a Vec<u32>
/// 
/// # Arguments:
/// * `array2_as_ypbpr`: An array2 containing a tuple of float values
pub fn pack(mut array2_as_ypbpr: Array2<(f32, f32, f32)>) -> Vec<u32> {

    // loop through each chunk in a 2x2 block of pixels
    let mut word_vec = Vec::new();
    for outer_y in 0..(array2_as_ypbpr.height() / 2) {
        for outer_x in 0..(array2_as_ypbpr.width() / 2) {
            let mut block_pb = 0.0;
            let mut block_pr = 0.0;
            let mut y_vec = Vec::new();
            for inner_y in 0..2 {
                for inner_x in 0..2 {
                    // compute the row/column
                    let row =  (outer_y * 2) + inner_y;
                    let col = (outer_x * 2) + inner_x;

                    // get pixel value inside of chunk
                    let pixel_val = array2_as_ypbpr.get_mut(col, row).unwrap();

                    // add up all the PB and PR values in the 2x2 block
                    block_pb += pixel_val.1;
                    block_pr += pixel_val.2;

                    // push y value into vector y_vec
                    y_vec.push(pixel_val.0);
                }
            }
            // For the PB and PR (chroma) elements of the pixels, take the average value of the four pixels in the block
            let pb_avg = block_pb / 4.0;
            let pr_avg = block_pr / 4.0;

            // Convert the PB and PR elements to four-bit values using the index_of_chroma function
            let pb_index = csc411_arith::index_of_chroma(pb_avg);
            let pr_index = csc411_arith::index_of_chroma(pr_avg);

            // Using a discrete cosine transform (DCT), transform the four Y (luminance/luma) values of the pixels into cosine coeffecients a, b, c, and d
            let a = (y_vec[3] + y_vec[2] + y_vec[1] + y_vec[0]) / 4.0;
            let b = (y_vec[3] + y_vec[2] - y_vec[1] - y_vec[0]) / 4.0;
            let c = (y_vec[3] - y_vec[2] + y_vec[1] - y_vec[0]) / 4.0;
            let d = (y_vec[3] - y_vec[2] - y_vec[1] + y_vec[0]) / 4.0;

            // Make sure the values stay in range and change int types
            let a = (a * 511.0).round() as u64;
            let b = ((b.clamp(-0.3, 0.3) * 50.0).round() as i64).clamp(-15, 15);
            let c = ((c.clamp(-0.3, 0.3) * 50.0).round() as i64).clamp(-15, 15);
            let d = ((d.clamp(-0.3, 0.3) * 50.0).round() as i64).clamp(-15, 15);

            // Pack a, b, c, d, PB, and PR into a 32-bit word
            let word = word_packing(a, b, c, d, pb_index, pr_index);
            word_vec.push(word);
        }
    }
    return word_vec;
}

/// function to pack values into a single u32 word, returns a u32
/// 
/// # Arguments:
/// * `a`: a u64 that represents the average brightness of an image
/// * `b`: an i64 that represents the degree to which the image gets brighter as we move from top to bottom
/// * `c`: an i64 that represents the degree to which the image gets brighter as we move from left to right
/// * `d`: an i64 that represents the degree to which the pixels on one diagonal are brighter than the pixels on the other diagonal
/// * `pb`: a usize that represents the degree to which the image transmits color-difference signals proportional to B - Y rgb values
/// * `pr': a usize that represents the degree to which the image transmits color-difference signals proportional to R - Y rgb values
pub fn word_packing(a: u64, b: i64, c: i64, d: i64, pb: usize, pr: usize) -> u32 {

    // initialize word
    let mut word = 0;
    // pass newu in the bitpack module the word, width (in bits), lsb value, and value (a)
    word = bitpack::newu(word, 9, 23, a).unwrap();
    // update word with b,c,d values by passing news the word, width (in bits), lsb value, and value (b,c,d)
    word = bitpack::news(word, 5, 18, b).unwrap(); 
    word = bitpack::news(word, 5, 13, c).unwrap(); 
    word = bitpack::news(word, 5, 8, d).unwrap();
    // update word with pb, pr values by passing newu the word, width (in bits), lsb value, and value (pb, pr)
    word = bitpack::newu(word, 4, 4, pb as u64).unwrap(); 
    word = bitpack::newu(word, 4, 0, pr as u64).unwrap(); 

    // return final packed word
    return word as u32;
}