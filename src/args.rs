use structopt::StructOpt;
use crate::one_char::OneChar;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
pub struct Args {
    /// Verbose mode, show all fonts in a font family
    #[structopt(short, long)]
    verbose: bool,

    /// Preview character render result using output font in browser
    #[structopt(short, long)]
    preview: bool,

    /// The character
    #[structopt(name = "CHAR")]
    the_char: OneChar,
}

pub fn get() -> Args {
    Args::from_args()
}
