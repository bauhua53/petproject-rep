use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut script = String::new();

    for _ in 0..10 {
        let num: u8 = rng.gen();
        script.push_str(&format!("println!(\"Random number: {}\");\n", num));
    }

    let mut file = File::create("random_script.rs")?;
    file.write_all(script.as_bytes())?;

    Ok(())
}
