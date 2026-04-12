use crate::cli::bitflags::Flags;
//TODO: return a better object here, maybe a struct of flags + file dest/src
pub fn parser_args(args: &[String], mut flags: Flags)
{
    let mut files: Vec<String> = Vec::new();

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

    if files.len() < 2 {
        eprintln!("cpz: missing file operand");
        std::process::exit(1);
    }

    let dest: &String = files.last().unwrap();
    let srcs: &[String] = &files[..files.len() - 1];
}

fn check_flag(c: char, flags: &mut Flags)
{
    match c
    {
        'r' => flags.insert(Flags::RECURSIVE),
        'i' => flags.insert(Flags::INTERACTIVE),
        'f' => flags.insert(Flags::FORCE),
        'u' => flags.insert(Flags::UPDATE),
        'v' => flags.insert(Flags::VERBOSE),
        _ =>  {
            eprintln!("cpz: invalid option -- '{}'", c);
            std::process::exit(1);
        },
    }
}

fn check_if_file_exists(path: &str) -> bool
{
    return std::path::Path::new(path).exists();
}