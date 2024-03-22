# Lossy-Portable-Pixmap-Image-Compression-Decompression
Rust program that converts full-color PPM images to and from compressed binary image files

I have utilized the Array2 module written by Professor Noah Daniels at URI for this assignment.

The main function in rpeg accepts an option (-c for compression or -d for decompression) as well as a file name for the initial image and a file name for the output image. If the user inputs -c, the image will be compressed. The pixel data is stored as an array2. The image is trimmed and the pixel data is transferred into a trimmed array in the trimming module if the rows/cols of the image are odd. The RGB values are changed to floats. The pixels are converted from RGB into component video color space format. Each chunk in a 2x2 block of pixels is iterated over for all the pixels in the packing module. Within this module, I find the Y values for each pixel and the average Pb and Pr indexes. The 4 Y values are then transformed into cosine coeffecients a, b, c, and d. The a, b, c, d, Pr, and Pb values are then packed into a single 32bit word for each 2x2 block of pixels in the image. The words are pushed into a vector and then compressed. The final compressed image is then output to standard output using the rpegio crate. The decompression is the inverse of the compression. The data is read in and put into a vector of 32bit words. All of these words are unpacked into Y/Pb/Pr values using the unpacking module. Then, they are transformed from component video color space (Y/PB/PR) into RGB color space using a map. They are then converted from floats to RGB values. Finally, the decompressed image is written to standard output using the csc411 image crate.
