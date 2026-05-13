mod cli;
pub mod orchestrator;
pub mod io;

use cli::parser::parser_args;
use crate::cli::bitflags::Flags;
use crate::cli::cp_data::CpData;
use crate::cli::validation::validation;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
    
    if !validation(&cp_data) {
        return;
    }
    
    let jobs: Vec<crate::orchestrator::job::Job> = orchestrator::job::Job::create_job(&cp_data);
    io::strategy::Strategy::determine_strategy(jobs);
    // control::state::State::run()
    // integrity::integrity::Integrity::verify_integrity();
}

