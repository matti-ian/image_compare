//this program compares the pixels of two images, randomly choosing the pixels to compare and returning a percentage match in the search set.
//the user specifies the number of pixels to use. A warning will be printed if the number of pixels chosen is too small.

use core::num;
use std::fs::File;
use std::path::Path;
use image::{GenericImageView, Pixel};
use rand::Rng;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Image Comparator")
        .version("1.0")
        .author("")
        .about("Compares two images at the pixel level,randomly choosing the pixels based on arguments")
        .arg(Arg::new("image1")
            .help("The first image file path")
            .required(true)
            .index(1))
        .arg(Arg::new("image2")
            .help("The second image file path")
            .required(true)
            .index(2))
        .arg(Arg::new("pixels")
            .help("The number of pixels to compare")
            .required(true)
            .index(3))
        .get_matches();

    let image1_path = matches.get_one::<String>("image1").unwrap();
    let image2_path = matches.get_one::<String>("image2").unwrap();
    let mut num_pixels: usize = matches.get_one::<String>("pixels").unwrap().parse().expect("Invalid number of pixels");

    // Load the images
    let img1 = image::open(&Path::new(image1_path)).expect("Failed to open image1");
    let img2 = image::open(&Path::new(image2_path)).expect("Failed to open image2");

    // Print metadata
    println!("Image 1: dimensions = {:?}, color type = {:?}", img1.dimensions(), img1.color());
    println!("Image 2: dimensions = {:?}, color type = {:?}", img2.dimensions(), img2.color());
    

    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();
    
    let mut max_width=0; //assume width1 has the smaller width.
    let mut max_height=0;//likewise
    
    if width1 > width2 {
        max_width = width2;
        
    }
    else{
        max_width = width1;
    }
    if height1 > height2{
        max_height = height2;        
    }
    else{
        max_height = height1;
    }
    
    println!("Max_width:{max_width}");
    println!("Max_height:{max_height}");
    
    if (max_height * max_width) < num_pixels as u32{
        num_pixels = (max_height * max_width) as usize;
        println!("Number of pixels is too high, reassigning to: {num_pixels}");
    }
    
    println!("{num_pixels} out of {:?} will be compared",max_height*max_width);
    
    let mut rng = rand::thread_rng();
    let mut matches = 0;
    println!("Comparing images...");
    for _ in 0..num_pixels {
        //get a random pixel within the max range.
        let x = rng.gen_range(0..max_width);
        let y = rng.gen_range(0..max_height);

        let pixel1 = img1.get_pixel(x, y);
        let pixel2 = img2.get_pixel(x, y);

        if pixel1 == pixel2 {
            matches += 1;
        }
    }

    let percentage_match = (matches as f64 / num_pixels as f64) * 100.0;
    println!("Percentage match: {:.2}%", percentage_match);
}
