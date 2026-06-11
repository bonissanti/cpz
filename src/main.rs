mod cli;
pub mod orchestrator;
pub mod io;
pub mod utils;
use crate::cli::bitflags::Flags;
use crate::cli::cp_data::CpData;
use crate::cli::validation::validation;
use crate::io::strategy::Strategy;
use crate::orchestrator::control::control_state::ControlState;
use cli::parser::parser_args;
use signal_hook::consts::{SIGCONT, SIGINT, SIGSTOP};
use signal_hook::iterator::Signals;
use std::sync::Arc;

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
    let strategy: Strategy = io::strategy::Strategy::determine_strategy(jobs);
    let control: Arc<ControlState> = Arc::new(ControlState::new());

    let control_clone = Arc::clone(&control);
    std::thread::spawn(move || {
        signal_watcher(control_clone);
    });
    
    strategy.execute(control);
    // integrity::integrity::Integrity::verify_integrity();
}

fn signal_watcher(ctrl: Arc<ControlState>) {
    let mut signals = Signals::new(&[SIGSTOP, SIGCONT, SIGINT]).unwrap();
    for signal in signals.forever() {
        match signal {
            SIGSTOP => ctrl.pause(),
            SIGCONT => ctrl.resume(),
            SIGINT => { ctrl.cancel(); break; },
            _ => {}
        }
    }
}