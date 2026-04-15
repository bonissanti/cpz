use crate::cli::bitflags::Flags;
use crate::cli::cp_data::CpData;

pub fn parser_args(args: &[String], mut flags: Flags) -> CpData
{
    let mut files: Vec<String> = Vec::new();

    //TODO: check which flags are valid - encapsulate in a function
    for arg in args
    {
        if arg.starts_with('-')
        {
            for c in arg.chars().skip(1) {
                check_flag(c, &mut flags);
            }
        }
        else {
            files.push(arg.to_string());
        }
    }

    //TODO: check if files exist - encapsulate in a function
    if files.len() < 2 {
        eprintln!("cpz: missing file operand");
        std::process::exit(1);
    }

    let dest: &String = files.last().unwrap();
    let srcs: &[String] = &files[..files.len() - 1];

    return CpData { flags, sources: srcs.to_vec(), destination: dest.to_string() };
}

fn check_flag(c: char, flags: &mut Flags)
{
    match c
    {
        'r' => flags.insert(Flags::RECURSIVE),
        'i' => flags.insert(Flags::NO_DEREFERENCE),
        'f' => flags.insert(Flags::FORCE),
        'u' => flags.insert(Flags::UPDATE),
        'v' => flags.insert(Flags::VERBOSE),
        _ =>  {
            eprintln!("cpz: invalid option -- '{}'", c);
            std::process::exit(1);
        },
    }
}