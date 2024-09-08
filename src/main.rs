enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}


struct Sizes {
    size: FileSize,
}

impl Sizes {
    fn new(size:u64, unit:String) -> Sizes {
        let size = match unit.as_str() {
            "bt" => FileSize::Bytes(size as u64),
            "kb" => FileSize::Kilobytes(size as f64),
            "mb" => FileSize::Megabytes(size as f64),
            "gb" => FileSize::Gigabytes(size as f64),
            _ => FileSize::Bytes(size as u64),
        };
        Sizes { size }
    }

    fn bytes(&self) -> u64 {
        match self.size {
            FileSize::Bytes(size) => size,
            FileSize::Kilobytes(size) => (size * 1024.0) as u64,
            FileSize::Megabytes(size) => (size * 1024.0 * 1024.0) as u64,
            FileSize::Gigabytes(size) => (size * 1024.0 * 1024.0 * 1024.0) as u64,
        }
    }

    fn kilobytes(&self) -> f64 {
        match self.size {
            FileSize::Bytes(size) => size as f64 / 1024.0,
            FileSize::Kilobytes(size) => size,
            FileSize::Megabytes(size) => size * 1024.0,
            FileSize::Gigabytes(size) => size * 1024.0 * 1024.0,
        }
    }

    fn megabytes(&self) -> f64 {
        match self.size {
            FileSize::Bytes(size) => size as f64 / 1024.0 / 1024.0,
            FileSize::Kilobytes(size) => size / 1024.0,
            FileSize::Megabytes(size) => size,
            FileSize::Gigabytes(size) => size * 1024.0,
        }
    }

    fn gigabytes(&self) -> f64 {
        match self.size {
            FileSize::Bytes(size) => size as f64 / 1024.0 / 1024.0 / 1024.0,
            FileSize::Kilobytes(size) => size / 1024.0 / 1024.0,
            FileSize::Megabytes(size) => size / 1024.0,
            FileSize::Gigabytes(size) => size,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let size;
    let unit:String;

    match args.len() {
        1 => {
            eprintln!("Usage: {} <size> <bt|kb|mb|gb>",args[0]);
            std::process::exit(1);
        }
        2 => {
            let to_split = args[1].to_string();
            let splitted = to_split.split_whitespace().collect::<Vec<&str>>();
            if splitted.len() == 2 {
                size = splitted[0].parse::<u64>().unwrap();
                unit = splitted[1].to_ascii_lowercase();
            } else {
                eprintln!("Usage: {} <size> <bt|kb|mb|gb>", args[0]);
                std::process::exit(1);
            }
            // Example output
            // Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
        }
        _ => {
            eprintln!("Usage: {} <size> <bt|kb|mb|gb>", args[0]);
            std::process::exit(1);            
        }
    }

    let sizes = Sizes::new(size, unit);
    println!("Sizes {{ bytes: \"{}\",  kilobytes: \"{}\",  megabytes: \"{}\",  gigabytes: \"{}\" }}",             
        sizes.bytes(),
        sizes.kilobytes(),
        sizes.megabytes(),
        sizes.gigabytes());
    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("Size is: {}.", args[0]);
}