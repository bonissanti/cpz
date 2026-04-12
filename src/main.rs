mod cli;

use cli::parser::parser_args;
use crate::cli::bitflags::Flags;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flags: Flags = Flags::empty();

    if args.len() == 1 {
        println!("cpz: missing file operand");
        return;
    }

    else if args.len() == 2 {
        println!("cpz: missing destination file operand after '{}'", args[1]);
        return;
    }

    parser_args(& args, flags);
}
