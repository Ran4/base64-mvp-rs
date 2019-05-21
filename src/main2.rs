use std::env::args;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn read_from_file(filename: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn get_filename() -> Result<String, Option<std::io::Error>> {
    args().nth(1).ok_or(Some(std::io::Error::new(
        ErrorKind::InvalidInput,
        "Could not get filename from arguments",
    )))
}

fn main() {
    get_filename()
        .and_then(|filename| {
            read_from_file(&filename).map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("{}: No such file", filename);
                    None
                }
                _ => Some(e),
            })
        })
        .and_then(|contents| {
            println!("{}", base64::encode(&contents));
            Ok(())
        })
        .map_err(|eopt: Option<std::io::Error>| {
            eopt.map(|e| {
                eprintln!("{}", e);
                None::<Option<std::io::Error>>
            })
        });
}
