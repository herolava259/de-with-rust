use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

enum FileSizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

struct FileSizeInfo {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl FileSizeInfo {
    fn convert_to(file_size: &FileSize, dest_unit: FileSizeUnit) -> f64 {
        const KB_CONVERSION: u64 = 1 << 10;
        const MB_CONVERSION: u64 = 1 << 20;
        const GB_CONVERSION: u64 = 1 << 30;
        //const TB_CONVERSION: u64 = 1 << 40;

        let src_rate: u64 = match file_size {
            FileSize::Bytes(_) => 1,
            FileSize::Kilobytes(_) => KB_CONVERSION,
            FileSize::Megabytes(_) => MB_CONVERSION,
            FileSize::Gigabytes(_) => GB_CONVERSION,
        };

        let dst_rate: u64 = match dest_unit {
            FileSizeUnit::Bytes => 1,
            FileSizeUnit::Kilobytes => KB_CONVERSION,
            FileSizeUnit::Megabytes => MB_CONVERSION,
            FileSizeUnit::Gigabytes => GB_CONVERSION,
        };

        let ratio: u64 = src_rate / dst_rate;

        return match file_size {
            FileSize::Bytes(bytes) => *bytes as f64 / ratio as f64,
            FileSize::Kilobytes(kb) => kb * (src_rate as f64 / dst_rate as f64),
            FileSize::Megabytes(mb) => mb * (src_rate as f64 / dst_rate as f64),
            FileSize::Gigabytes(gb) => gb * (src_rate as f64 / dst_rate as f64),
        };
    }

    fn from_filesize(file_size: FileSize) -> Self {
        Self {
            bytes: format!(
                "{} bytes",
                Self::convert_to(&file_size, FileSizeUnit::Bytes)
            ),
            kilobytes: format!(
                "{:.2} KB",
                Self::convert_to(&file_size, FileSizeUnit::Kilobytes)
            ),
            megabytes: format!(
                "{:.2} MB",
                Self::convert_to(&file_size, FileSizeUnit::Megabytes)
            ),
            gigabytes: format!(
                "{:.2} GB",
                Self::convert_to(&file_size, FileSizeUnit::Gigabytes)
            ),
        }
    }
}

fn convert_to_filesize(file_size: String) -> FileSize {
    let words: Vec<&str> = file_size.split_whitespace().collect();

    if words.len() != 2 {
        panic!("Invalid file size format. Expected format: <size> <unit>");
    }

    match words[1].to_lowercase().as_str() {
        "bytes" | "byte" | "b" => {
            FileSize::Bytes(words[0].parse::<u64>().expect("Invalid number format"))
        }
        "kb" | "kilobytes" | "kilobyte" => {
            FileSize::Kilobytes(words[0].parse::<f64>().expect("Invalid number format"))
        }
        "mb" | "megabytes" | "megabyte" => {
            FileSize::Megabytes(words[0].parse::<f64>().expect("Invalid number format"))
        }
        "gb" | "gigabytes" | "gigabyte" => {
            FileSize::Gigabytes(words[0].parse::<f64>().expect("Invalid number format"))
        }
        _ => panic!("Invalid file size unit. Supported units: bytes, KB, MB, GB"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_size = &args[1];

    let filesize = convert_to_filesize(file_size.to_string());

    let file_size_info = FileSizeInfo::from_filesize(filesize);

    println!("File Size Information:");
    println!("Bytes: {}", file_size_info.bytes);
    println!("Kilobytes: {}", file_size_info.kilobytes);
    println!("Megabytes: {}", file_size_info.megabytes);
    println!("Gigabytes: {}", file_size_info.gigabytes);
}
