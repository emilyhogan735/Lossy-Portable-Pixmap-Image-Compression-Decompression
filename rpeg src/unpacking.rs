use bitpack::bitpack;

/// function that unpacks data from 32bit words into a vector containing Y/Pb/Pr values, returns a Vec<Option<(f32, f32, f32)>>
/// 
/// # Arguments:
/// * `word_vec`: A vector containing 32bit words
/// * `width`: the width of the image
/// * `height`: the height of the image
pub fn unpack(word_vec: Vec<u32>, width: usize, height: usize) -> Vec<Option<(f32, f32, f32)>> {

    // initialize empty array2 for Y/Pb/Pr data and row/column
    let mut ypbpr_data = vec![None; width * height];
    let mut row = 0;
    let mut col = 0;
    // loop through each word containing a 2x2 block of pixels
    for word in word_vec {

        // Unpack each 2-by-2 block from a 32-bit word into the a, b, c, d, pr, and pb values for each pixel
        let (a, b, c, d, pr_index, pb_index) = word_unpacking(word);
        // convert ints to floats and make sure they are in the correct range
        let a = a as f32 / 511.0;
        let b = b as f32 / 50.0;
        let c = c as f32 / 50.0;
        let d = d as f32 / 50.0;

        // Using a discrete cosine transform (DCT), transform the four cosine coeffecients a, b, c, and d into Y (luminance/luma) values of the pixels
        let y_val1 = a - b - c + d;
        let y_val2 = a - b + c - d;
        let y_val3 = a + b - c - d;
        let y_val4 = a + b + c + d;

        // Convert the PB and PR elements to chroma values using the chroma_of_index function
        let pb = csc411_arith::chroma_of_index(pb_index);
        let pr = csc411_arith::chroma_of_index(pr_index);

        // push Y/Pb/Pr values for each chunk in 2x2 block into ypbpr_data vector
        ypbpr_data[row * width + col] = Some((y_val1, pb, pr));
        ypbpr_data[row * width + col + 1] = Some((y_val2, pb, pr));
        ypbpr_data[(row + 1) * width + col] = Some((y_val3, pb, pr));
        ypbpr_data[(row + 1) * width + col + 1] = Some((y_val4, pb, pr));

        // update row/col values
        col += 2;
        if col >= width {
            col = 0;
            row += 2;
        }
    }
    return ypbpr_data;
}

// function to unpack multiple values from a single u32 word, returns a u64, i64, i64, i64, usize, and usize
/// 
/// # Arguments:
/// * `word`: a u32 containing Y/Pb/Pr values
pub fn word_unpacking(word: u32) -> (u64, i64, i64, i64, usize, usize) {

    // pass getu/gets the word, width, and lsb to get each a/b/c/d/pb/pr value
    let a = bitpack::getu(word as u64, 9, 23);
    let b = bitpack::gets(word as u64, 5, 18); 
    let c = bitpack::gets(word as u64, 5, 13); 
    let d = bitpack::gets(word as u64, 5, 8);
    let pb_index = bitpack::getu(word as u64, 4, 4) as usize; 
    let pr_index = bitpack::getu(word as u64, 4, 0) as usize; 

    // return final values in a tuple
    return (a, b, c, d, pr_index, pb_index);
}