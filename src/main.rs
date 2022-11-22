
use std::fs;
use std::io;
use std::io::Read;

use anyhow::Result;
use clap::Parser;


mod commands;
use crate::commands::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(path) = cli.path {
        let contents = fs::read_to_string(&path)
            .expect("Should have been able to read the file");

        let b64 = base64::encode(&contents);
        dbg!(b64);
        qr2term::print_qr(&contents)?;
        Ok(())
    }
    else {
        let mut vec :Vec<u8> = Vec::with_capacity(commands::cli::default_max_len());
        let mut buffer :&mut  [u8] = &mut [0;commands::cli::default_max_len()];
        let mut stdin = io::stdin(); 
        let mut nbytes = 0;
        
        while let Ok(len) = stdin.read(&mut buffer) {
            nbytes = nbytes + len;
            eprintln!("buffer len: {}", buffer.len());
            eprintln!("read len: {}", len);
            eprintln!("max_len : {}", cli.max_len);
            eprintln!("nbyes : {}", nbytes);
            if len >= cli.max_len {
                return Err(anyhow::anyhow!("Input too large, nbytes:{} max:{}", nbytes, cli.max_len)) 
            }
            if len == 0 {
                break;
            }
            else {
                vec.extend_from_slice(&buffer[..len]);
            }
        }
        println!("vec: {:?}", vec);
        qr2term::print_qr(&vec)?;
        eprintln!("total bytes: {}", nbytes);
        Ok(())
    }
}

