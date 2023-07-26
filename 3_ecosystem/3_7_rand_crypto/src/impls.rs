use rand::{thread_rng, Rng};
use ring::{
    self,
    rand::{Random, SystemRandom},
};
use sha3::{digest::Digest, Sha3_256};
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

struct PassGenConfig<'a, T> {
    len: usize,
    symbols: &'a [T],
}

fn generate_password(cfg: &PassGenConfig<char>) -> String {
    let mut buff = String::with_capacity(cfg.len);
    for _ in 0..cfg.len {
        let symbol = select_rand_val_crypto(cfg.symbols);
        buff.push(*symbol);
    }

    buff
}

fn select_rand_val_crypto<T>(values: &[T]) -> &T {
    let sys_random = SystemRandom::new();
    let res: Random<[u8; 8]> = ring::rand::generate(&sys_random).unwrap();
    // It doesn't really matter if we pick big or little endinan here
    let idx = usize::from_be_bytes(res.expose());
    &values[idx]
}

fn new_access_token() -> String {
    let cfg = PassGenConfig {
        len: 64,
        symbols: &utils::SYMBOLS_VEC,
    };

    generate_password(&cfg)
}

fn get_file_hash(path: &Path) -> io::Result<String> {
    const BUFFER_SIZE: usize = 1024;

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    let mut hasher = Sha3_256::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count])
    }

    Ok(format!("{:X}", hasher.finalize()))
}

fn hash_password(password: &str) -> argon2::Result<String> {
    let salt_cfg = PassGenConfig {
        len: 32,
        symbols: &utils::SYMBOLS_VEC,
    };
    let salt = generate_password(&salt_cfg);

    utils::hash_password_with_salt(password, &salt)
}

mod utils {
    use once_cell::sync::Lazy;

    pub fn hash_password_with_salt(password: &str, salt: &str) -> argon2::Result<String> {
        let password = password.as_bytes();
        let salt = salt.as_bytes();
        let config = argon2::Config::default();

        argon2::hash_encoded(password, salt, &config)
    }

    pub const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    pub static SYMBOLS_VEC: Lazy<Vec<char>> = Lazy::new(|| SYMBOLS.chars().collect());
}
