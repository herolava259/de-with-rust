enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

// write function: Extend the program by implementing a function that takes a file size in bytes as input
// and returns the file size representation in the largest possible unit. For example, if
// the input is 2500 bytes, the function should return "2.44 KB". Invoke this function
// with different file size values and print the results to ensure the correct conversion.



fn format_size(size: u64) -> String {

    const KB_CONVERSION: u64 = 1 << 10;
    const MB_CONVERSION: u64 = 1 << 20;
    const GB_CONVERSION: u64 = 1 << 30;
    const TB_CONVERSION: u64 = 1 << 40;

    let filesize = match size {
        0_u64..=999_u64 => FileSize::Bytes(size as f64),
        1000_u64..=999999_u64 => FileSize::Kilobytes(size as f64 / KB_CONVERSION as f64),
        1_000_000_u64..=999_999_999_u64 => FileSize::Megabytes(size as f64 / MB_CONVERSION as f64),
        1_000_000_000_u64..=999_999_999_999_u64 => FileSize::Gigabytes(size as f64 / GB_CONVERSION as f64),
        _ => FileSize::Terabytes(size as f64 / TB_CONVERSION as f64),
    };

    match filesize {
        FileSize::Bytes(b) => format!("{} B", (b*100.0).round() / 100.0),
        FileSize::Kilobytes(kb) => format!("{} KB", (kb*100.0).round() / 100.0),
        FileSize::Megabytes(mb) => format!("{} MB", (mb*100.0).round() / 100.0),
        FileSize::Gigabytes(gb) => format!("{} GB", (gb*100.0).round() / 100.0),
        FileSize::Terabytes(tb) => format!("{} TB", (tb*100.0).round() / 100.0),
    }

    
}

fn main() {
    let result = format_size(2500);
    println!("{}", result)
}
