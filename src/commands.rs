use std::io::Write;
use std::str::FromStr;

use crate::args::{self, Command, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs::File;

fn print(args: PrintArgs) {
    println!("Print: {:?}", args);
    let file = Png::from_file(args.file_path).unwrap();
    file.chunks().iter().for_each(|c: &Chunk| {
        println!("{:#x?}", c);
    });
}

fn remove(args: RemoveArgs) {
    println!("Remove: {:?}", args);
    match Png::from_file(args.file_path) {
        Ok(mut f) => {
            let r = f.remove_first_chunk(&args.chunk_type).unwrap();
            println!(
                "Removed chunk with type {:#?} and message {:#?}",
                args.chunk_type,
                r.data_as_string(),
            );
        }
        Err(e) => println!("Error {:?}", e),
    };
}

fn decode(args: DecodeArgs) {
    println!("Decode: {:?}", args);
    match Png::from_file(args.file_path) {
        Ok(f) => {
            let c = f.chunk_by_type(&args.chunk_type).unwrap();
            println!("{:#?}", c.data_as_string());
        }
        Err(e) => println!("Error {:?}", e),
    };
}

fn encode(args: EncodeArgs) {
    println!("Encode: {:?}", args);
    match Png::from_file(&args.file_path) {
        Ok(mut f) => {
            f.append_chunk(Chunk::new(
                ChunkType::from_str(&args.chunk_type).unwrap(),
                args.message.into_bytes(),
            ));
            let mut file = File::create(&args.file_path).unwrap();
            let _ = file.write_all(&f.as_bytes());
        }
        Err(e) => println!("Error {:?}", e),
    };
}

pub fn run(args: Command) {
    match args {
        args::Command::Encode(encode_args) => {
            encode(encode_args);
        }
        args::Command::Print(print_args) => {
            print(print_args);
        }
        args::Command::Remove(remove_args) => {
            remove(remove_args);
        }
        args::Command::Decode(decode_args) => {
            decode(decode_args);
        }
    }
}
