use std::{
    fs::File,
    io::{BufReader, BufRead},
};

fn is_file_exists(file: &String) -> bool {
    std::path::Path::new(file).exists()
}

pub async fn cmd(private_key_path: &String, wallets_path: &String, amount: &f64) {
    for (path, name) in [(private_key_path, "--private-key"), (wallets_path, "--wallets")].iter() {
        if !is_file_exists(path) {
            println!("❌ File for {} not found: {}", name, path);
            return;
        };
    }

    let private_key = match File::open(private_key_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => line,
                Some(Err(_)) | None => {
                    println!(
                        "❌ Failed to read the first line of private key file: {}",
                        private_key_path
                    );
                    return;
                }
            }
        }
        Err(_) => {
            println!("❌ Failed to open private key file: {}", private_key_path);
            return;
        }
    };

    println!("🔑 First line of private key file: {}", private_key);
}
