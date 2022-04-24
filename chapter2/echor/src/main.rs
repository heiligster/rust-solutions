use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Philipp Mueller <heiligster@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text to print to the terminal")
                .required(true)
                .min_values(1)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("no-newline")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let input = matches.values_of_lossy("text").unwrap().join(" ");
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", input, if omit_newline { "" } else { "\n" });
}
