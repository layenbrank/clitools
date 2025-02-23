use anyhow::Ok;
use rand::{prelude::IndexedRandom, seq::SliceRandom};

use crate::opts::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("upper won't be empty"));
    }
    if opts.lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("lower won't be empty"));
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("number won't be empty"));
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("symbol won't be empty"));
    }
    println!("Generate Password: {:?}", chars);

    for _ in 0..(opts.len - password.len() as u8) {
        let random_char = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*random_char);
    }

    // TODO 确保密码包含至少一个大写字母、一个小写字母、一个数字和一个特殊字符

    password.shuffle(&mut rng);

    println!("Generate Password: {}", String::from_utf8(password)?);

    Ok(())
}
