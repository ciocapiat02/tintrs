use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Action to perform
    #[arg(required=true)]
    pub action: String,

    /// Input file
    #[arg(required=true)]
    pub input: String,

    /// Output file
    #[arg(required=true)]
    pub output: String,

    /// Colorscheme chosen for the apply function
    #[arg(short, long)]
    pub colorscheme: Option<String>,

    /// Number of colors to extract from the image
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// Show results in a window
    #[arg(short, long, default_value_t = false)]
    pub show: bool,

    /// Amount of blur filter
    #[arg(short, long, default_value_t = 1)]
    pub blur_amount: u8,
}
