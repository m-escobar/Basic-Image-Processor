use std::ffi::OsString;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mirage")]
#[command(about = "An image processing application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Blur {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
        #[arg(
            short,
            long,
            value_name = "BLUR_AMOUNT",
            default_value = "2.0",
            num_args = 0..=1,
        )]
        blur_amount: f32,
    },
    #[command(arg_required_else_help = true)]
    Fractal {
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
    },
    #[command(arg_required_else_help = true)]
    Invert {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
    },
    #[command(arg_required_else_help = true)]
    Grayscale {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
    },
    #[command(arg_required_else_help = true)]
    Brighten {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
        #[arg(
            short,
            long,
            value_name = "BRIGHT_AMOUNT",
            default_value = "50",
            num_args = 0..=1,
        )]
        bright_amount: i32,
    },
    #[command(arg_required_else_help = true)]
    Crop {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
        #[arg(
            short,
            long,
            value_name = "COORDS",
            num_args = 4,
            required = true
        )]
        coords: Vec<u32>,
    },
    #[command(arg_required_else_help = true)]
    Rotate {
        #[arg(value_name = "INPUT_FILE")]
        infile: Option<OsString>,
        #[arg(value_name = "OUTPUT_FILE")]
        outfile: Option<OsString>,
        #[arg(
            short,
            long,
            value_name = "DEGREE_MOUNT",
            default_value = "90",
            num_args = 0..=1,
        )]
        degree_amount: u32,
    },
}


fn main() {
    let args = Cli::parse();
    
    match args.command {
       Commands::Blur {
           infile,
           outfile,
           blur_amount
       } => blur(infile.unwrap().into_string().unwrap(),
                 outfile.unwrap().into_string().unwrap(),
                 blur_amount
                ),

        Commands::Fractal {
            outfile,
        } => fractal(outfile.unwrap().into_string().unwrap()),

        Commands::Invert {
            infile,
            outfile,
        } => invert(infile.unwrap().into_string().unwrap(),
                    outfile.unwrap().into_string().unwrap()
             ),

        Commands::Grayscale {
            infile,
            outfile,
        } => grayscale(infile.unwrap().into_string().unwrap(),
                       outfile.unwrap().into_string().unwrap()
             ),

        Commands::Brighten {
            infile,
            outfile,
            bright_amount
        } => brighten(infile.unwrap().into_string().unwrap(),
                      outfile.unwrap().into_string().unwrap(),
                      bright_amount
             ),

        Commands::Crop {
            infile,
            outfile,
            coords,
        } => {
                let coord = coords.to_vec();
                crop(infile.unwrap().into_string().unwrap(),
                     outfile.unwrap().into_string().unwrap(),
                     coord[0],
                     coord[1],
                     coord[2],
                     coord[3]
                )
        },

        Commands::Rotate {
            infile,
            outfile,
            degree_amount
        } => {
                if degree_amount == 90 || degree_amount == 180 || degree_amount == 270 {
                    rotate(infile.unwrap().into_string().unwrap(),
                           outfile.unwrap().into_string().unwrap(),
                           degree_amount
                    )
                } else {
                    println!("Valid rotation degrees are: 90, 180, and 270")
                };
        }
    }
}


fn blur(infile: String, outfile: String, blur_amount: f32) {
    // let infile = command[0];

    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(blur_amount);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}


// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}


fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();

    img.save(outfile).expect("Failed writing OUTFILE.");
}


fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.grayscale();

    img2.save(outfile).expect("Failed writing OUTFILE.");
}


fn brighten(infile: String, outfile: String, bright_amount: i32) {
    // Positive numbers brighten the image. Negative numbers darken it.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(bright_amount);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}


fn crop(infile: String, outfile: String, coord_x1: u32, coord_x2: u32, coord_y1: u32, coord_y2: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.crop(coord_x1, coord_x2, coord_y1, coord_y2);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}


fn rotate(infile: String, outfile: String, rotation_degree: u32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = match rotation_degree {
        90 =>  img.rotate90(),
        180 => img.rotate180(),
        _ => img.rotate270()
    };

    img2.save(outfile).expect("Failed writing OUTFILE.");
}
