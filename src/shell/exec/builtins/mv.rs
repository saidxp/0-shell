use crate::shell::Shell;
use crate::shell::exec::helper::format_io_error;
use crate::shell::parse::Cmd;
use std::fs;

pub fn mv(_shell: &mut Shell, cmd: &Cmd) {
    if cmd.args.len() < 2 {
        eprintln!("mv: missing file operand");
        return;
    }

    let src = &cmd.args[0];
    let dst = &cmd.args[1];

    if let Err(rename_err) = fs::rename(src, dst) {
        match fs::metadata(src) {
            Ok(m) if m.is_dir() => {
                eprintln!(
                    "mv: cannot move directory '{}': {}",
                    src,
                    format_io_error(&rename_err)
                );
            }
            Ok(_) => {
                if let Err(copy_err) = fs::copy(src, dst) {
                    eprintln!(
                        "mv: cannot move '{}' to '{}': {}",
                        src,
                        dst,
                        format_io_error(&copy_err)
                    );
                    return;
                }
                if let Err(remove_err) = fs::remove_file(src) {
                    eprintln!(
                        "mv: cannot remove '{}': {}",
                        src,
                        format_io_error(&remove_err)
                    );
                }
            }
            Err(_) => eprintln!("mv: cannot stat '{}': {}", src, format_io_error(&rename_err)),
        }
    }
}
