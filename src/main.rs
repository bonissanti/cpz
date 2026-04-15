mod cli;

use cli::parser::parser_args;
use crate::cli::bitflags::Flags;
use crate::cli::cp_data;
use crate::cli::cp_data::CpData;

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

    let cp_data: CpData = parser_args(& args, flags);

    
}
