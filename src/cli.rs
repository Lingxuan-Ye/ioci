use clap::{Arg, ArgAction, ArgMatches, Command, command, value_parser};
use std::sync::LazyLock;

static MATCHES: LazyLock<ArgMatches> = LazyLock::new(|| {
    command!()
        .subcommands([Command::new("passgen").about("Generate password").args([
            Arg::new("digits")
                .short('d')
                .long("digits")
                .value_name("NUM")
                .value_parser(value_parser!(u8))
                .default_value("4")
                .help("Number of digits"),
            Arg::new("lowercase")
                .short('l')
                .long("lowercase")
                .value_name("NUM")
                .value_parser(value_parser!(u8))
                .default_value("4")
                .help("Number of lowercase letters"),
            Arg::new("uppercase")
                .short('U')
                .long("uppercase")
                .value_name("NUM")
                .value_parser(value_parser!(u8))
                .default_value("4")
                .help("Number of uppercase letters"),
            Arg::new("special")
                .short('s')
                .long("special")
                .value_name("NUM")
                .value_parser(value_parser!(u8))
                .default_value("4")
                .help("Number of special symbols"),
            Arg::new("unique")
                .short('u')
                .long("unique")
                .action(ArgAction::SetTrue)
                .help("Ensure all characters are unique"),
        ])])
        .arg_required_else_help(true)
        .get_matches()
});

pub enum Args {
    None,

    PassGen {
        digits: u8,
        lowercase: u8,
        uppercase: u8,
        special: u8,
        unique: bool,
    },
}

impl Args {
    pub fn parse() -> Self {
        match MATCHES.subcommand() {
            None => Self::None,

            Some(("passgen", matches)) => Self::PassGen {
                digits: matches
                    .get_one::<u8>("digits")
                    .copied()
                    .unwrap_or_else(|| unreachable!()),
                lowercase: matches
                    .get_one::<u8>("lowercase")
                    .copied()
                    .unwrap_or_else(|| unreachable!()),
                uppercase: matches
                    .get_one::<u8>("uppercase")
                    .copied()
                    .unwrap_or_else(|| unreachable!()),
                special: matches
                    .get_one::<u8>("special")
                    .copied()
                    .unwrap_or_else(|| unreachable!()),
                unique: matches.get_flag("unique"),
            },

            _ => unreachable!(),
        }
    }
}
