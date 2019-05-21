use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file(filename: &String) -> io::Result<String> {
    let mut file_contents = String::new();
    File::open(filename)?.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

enum Action {
    #[allow(dead_code)]
    Encode,
    Decode,
}

fn perform_action(action: &Action, contents: &String) -> io::Result<()> {
    match action {
        Action::Encode => println!("{}", base64::encode(contents)),
        Action::Decode => {
            let decoded_bytes = base64::decode(contents).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Could not decode contents: {}", e),
                )
            })?;
            println!("{}", String::from_utf8_lossy(&decoded_bytes));
        }
    };
    Ok(())
}

fn get_contents(filename: &String) -> io::Result<String> {
    read_from_file(filename).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}: No such file", filename),
        ),
        _ => e,
    })
}

fn parse_args() -> io::Result<(String, Action)> {
    use clap::{App, Arg};
    let matches = App::new("b64")
        .arg(
            Arg::with_name("FILE")
                .takes_value(true)
                .help("File to base64 encode/decode"),
        )
        .arg(
            Arg::with_name("DECODE")
                .short("-d")
                .long("--decode")
                .help("Decode data"),
        )
        .get_matches();

    let filename = matches.value_of("FILE").ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Could not get filename from arguments",
    ))?;

    let action = if matches.is_present("DECODE") {
        Action::Decode
    } else {
        Action::Encode
    };

    Ok((filename.to_string(), action))
}

fn main() {
    if let Err(e) = parse_args().and_then(|(filename, action)| {
        get_contents(&filename).and_then(|contents| perform_action(&action, &contents))
    }) {
        eprintln!("{}", e);
    }
}
