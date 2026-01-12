use clap::Parser;

mod colorscheme;
mod effect;
mod format_converter;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Action to perform
    #[arg(required=true)]
    action: String,

    /// Input file
    #[arg(required=true)]
    input: String,

    /// Output file
    #[arg(required=true)]
    output: String,

    /// Colorscheme chosen for the apply function
    #[arg(short, long)]
    colorscheme: Option<String>,

    /// Number of colors to extract from the image
    #[arg(short, long, default_value_t = 16)]
    length: u8,

    /// Show results in a window
    #[arg(short, long, default_value_t = false)]
    show: bool,

    /// Amount of blur filter
    #[arg(short, long, default_value_t = 1)]
    blur_amount: u8,
}

fn main() {
    let args = Args::parse();
    
    if args.action == "extract" {
       println!("Extracting {} colors from {} and saving to {}", args.length, args.input, args.output);
       // Call extract function here
    }

    else if args.action == "apply" {
       if let Some(colorscheme) = args.colorscheme {
           println!("Applying colorscheme {} from {} to {} and saving to {}", colorscheme, args.input, args.output, args.output);
           // Call apply function here
       } else {
           eprintln!("Colorscheme is required for apply action");
       }
    } 

    else if args.action == "blur" {
        println!("Applying blur of amount {} to {} and saving to {}", args.blur_amount, args.input, args.output);
    }

    else {
        eprintln!("Unknown action: {}", args.action);
   } 
}
