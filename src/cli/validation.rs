use std::fs;
use std::fs::{OpenOptions};
use std::path::Path;
use crate::cli::bitflags::Flags;
use crate::cli::cp_data::CpData;

pub fn validation(cp_data: &CpData) -> bool
{
    for src in cp_data.sources.iter() {
        if let Ok(metadata) = fs::metadata(src)  && metadata.is_dir(){
            if !validation_folder(src, cp_data) {
                return false;
            }
        }
        if !validation_file(src, cp_data) {
            return false;
        }
    }
    true
}

fn validation_folder(src: &String, cp_data: &CpData) -> bool
{
    let src_is_dir = fs::metadata(src).ok().map(|m| m.is_dir()).unwrap_or(false);
    let dest_is_dir = fs::metadata(&cp_data.destination).ok().map(|m| m.is_dir()).unwrap_or(false);

    if src_is_dir && !dest_is_dir {
        eprintln!("cpz: cannot overwrite non-directory '{}' with directory '{}'", cp_data.destination, src);
        return false;
    }

    if src_is_dir && !check_if_recursive_flag_is_present(cp_data) {
        eprintln!("cpz: '{}' is a directory (not copied). Use -r to copy directories", src);
        return false;
    }

    if src_is_dir && check_if_dest_and_src_are_equals(&cp_data.destination, src) {
        eprintln!("cpz: cannot copy a directory, '{}', into itself, '{}'", src, cp_data.destination);
        return false;
    }

    if src_is_dir && check_if_dest_is_subdir_of_src(&cp_data.destination, src) {
        eprintln!("cpz: cannot copy a directory, '{}', into a subdirectory of itself, '{}'", src, cp_data.destination);
        return false;
    }

    return true;
}

fn validation_file(src: &String, cp_data: &CpData) -> bool
{
    if check_if_dest_and_src_are_equals(&cp_data.destination, src){
        eprintln!("cpz: '{}' and '{}' are the same file", src, cp_data.destination);
        return false;
    }

    if !check_if_file_exists(src) {
        eprintln!("cpz: cannot stat '{}': No such file or directory", src);
        return false;
    }

    if !check_if_dest_is_dir(&cp_data.destination, cp_data) {
        eprintln!("cpz: target '{}' is not a directory", cp_data.destination);
        return false;
    }

    if !check_if_src_has_permission_to_read(src) {
        eprintln!("cpz: cannot stat '{}': Permission denied", src);
        return false;
    }

    if check_if_dest_has_permission_to_write(&cp_data.destination) {
        eprintln!("cpz: cannot stat '{}': Permission denied", cp_data.destination);
        return false;
    }
    return true;
}

fn check_if_recursive_flag_is_present(cp_data: &CpData) -> bool
{
    return cp_data.flags.contains(Flags::RECURSIVE);
}

fn check_if_dest_and_src_are_equals(dest: &String, src: &String) -> bool
{
    let src_path = fs::canonicalize(src);
    let dest_path = fs::canonicalize(dest);

    match (src_path, dest_path) {
        (Ok(src_path), Ok(dest_path)) => src_path == dest_path,
        _ => false
    }
}

fn check_if_dest_is_dir(dest: &String, cp_data: &CpData) -> bool
{
    if cp_data.sources.len() > 1 {
        if let Ok(metadata) = fs::metadata(dest)  && !metadata.is_dir(){
            return false;
        }
    }
    return true;
}

fn check_if_dest_is_subdir_of_src(dest: &String, src: &String) -> bool
{
    let src_canonical = fs::canonicalize(src).ok();
    let dest_canonical = fs::canonicalize(dest).ok();

    match (src_canonical, dest_canonical) {
        (Some(src_path), Some(dest_path)) => dest_path.starts_with(src_path),
        _ => false
    }
}

fn check_if_file_exists(file: &String) -> bool
{
    return fs::metadata(file).is_ok();
}

fn check_if_src_has_permission_to_read(src: &String) -> bool
{
    return OpenOptions::new().read(true).open(src).is_ok();
}

fn check_if_dest_has_permission_to_write(dest: &String) -> bool
{
    let path = Path::new(dest);

    if path.exists() {
        return OpenOptions::new().write(true).open(dest).is_ok();
    }
    else {
        path.parent()
            .and_then(|p| fs::metadata(p).ok())
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    }
}