use clap::Args;
use clap::Parser;

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the input png file into which a message is to be encoded
    #[arg(short, long)]
    pub file_path: String,
    #[arg(short, long)]
    /// 4 character string to use as png chunk type. Invalid if the third character is lowercase.
    pub chunk_type: String,
    #[arg(short, long)]
    /// Message to encode into the file
    pub message: String,
    #[arg(short, long)]
    /// Output path to write new png file to
    pub out_path: Option<Option<String>>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the input png file from which a message is to be decoded
    #[arg(short, long)]
    pub file_path: String,
    /// 4 character string to use as png chunk type. Invalid if the third character is lowercase.
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to the input png file from which an encoded message is to be removed
    #[arg(short, long)]
    pub file_path: String,
    /// 4 character string to use as png chunk type. Invalid if the third character is lowercase.
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to the input png file from which an encoded message is to be printed to stdout
    #[arg(short, long)]
    pub file_path: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Command {
    #[command(name = "encode", about = "encode a message into a png file")]
    Encode(EncodeArgs),
    #[command(name = "decode", about = "decode a message from a png file")]
    Decode(DecodeArgs),
    #[command(name = "remove", about = "remove a message from a png file")]
    Remove(RemoveArgs),
    #[command(name = "print", about = "print a message that is inside a png file")]
    Print(PrintArgs),
}

pub fn parse_commands() -> Result<Command, &'static str> {
    let args = Command::parse();
    Ok(args)
}
