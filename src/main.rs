use self::cli::Args;
use self::passgen::PassGen;
use anyhow::Result;
use eoe::ExitOnError;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

mod cli;
mod passgen;

fn run() -> Result<()> {
    match Args::parse() {
        Args::None => (),

        Args::PassGen {
            digits,
            lowercase,
            uppercase,
            special,
            unique,
        } => {
            let mut passgen = PassGen {
                digits,
                lowercase,
                uppercase,
                special,
                rng: ChaCha20Rng::try_from_os_rng()?,
            };
            let password = if unique {
                passgen.generate_unique()?
            } else {
                passgen.generate()
            };
            println!("{password}");
        }
    }

    Ok(())
}

fn main() {
    run().exit_on_error();
}
