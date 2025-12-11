// use crate::{EXE, PS};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process::{Child, Command, Output, Stdio},
};

#[macro_export]
macro_rules! input {
    () => {{
        ::std::io::Write::flush(&mut ::std::io::stdout()).expect("Failed to flush stdio");
        let mut input: String = String::new();
        ::std::io::stdin().read_line(&mut input).expect("Failed to read user input");
        input.trim().to_string()
    }};
    ($($arg:tt)*) => {{
        print!($($arg)*);
        input!()
    }};
}

// pub fn command_exists(command: &str) -> bool {
//     #[cfg(not(windows))]
//     const PATH_SEP: &str = ":";
//     #[cfg(windows)]
//     const PATH_SEP: &str = ";";

//     if let Ok(path) = std::env::var("PATH") {
//         for p in path.split(PATH_SEP) {
//             // windows: ..\\my_prgm.exe
//             // unix: ../my_prgm
//             let p_str: String = format!("{}{PS}{}{EXE}", p, command);
//             if std::fs::metadata(p_str).is_ok() {
//                 return true;
//             }
//         }
//     }
//     false
// }

pub fn spawn_cmd(
    program: &str,
    args: &[&str],
    cwd: Option<&str>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
) -> Child {
    Command::new(program)
        .args(args)
        .stdin(stdin.unwrap_or(Stdio::inherit()))
        .stdout(stdout.unwrap_or(Stdio::inherit()))
        .stderr(stderr.unwrap_or(Stdio::inherit()))
        .current_dir(cwd.unwrap_or("."))
        .spawn()
        .unwrap()
}

// pub fn spawn_cmd_stdin(program: &str, args: &[&str], cwd: Option<&str>, stdin: Stdio) -> Child {
//     spawn_cmd(program, args, cwd, Some(stdin), None, None)
// }

// pub fn spawn_cmd_stdout(
//     program: &str,
//     args: &[&str],
//     cwd: Option<&str>,
//     stdout: Stdio,
//     stderr: Option<Stdio>,
// ) -> Child {
//     spawn_cmd(program, args, cwd, None, Some(stdout), stderr)
// }

pub fn run_cmd(program: &str, args: &[&str], cwd: Option<&str>) -> Result<(), CommandExitError> {
    wrap_exit(
        spawn_cmd(program, args, cwd, None, None, None)
            .wait()
            .unwrap(),
    )
}

// pub fn run_cmd_stdin(program: &str, args: &[&str], cwd: Option<&str>, stdin: Stdio) -> Result<(), CommandExitError> {
//     wrap_exit(
//         spawn_cmd(program, args, cwd, Some(stdin), None, None)
//         .wait()
//         .unwrap()
//     )
// }

pub fn run_cmd_stdout(
    program: &str,
    args: &[&str],
    cwd: Option<&str>,
    stdout: Stdio,
    stderr: Option<Stdio>,
) -> Result<(), CommandExitError> {
    wrap_exit(
        spawn_cmd(program, args, cwd, None, Some(stdout), stderr)
        .wait()
        .unwrap()
    )
}

pub fn pipe_cmd_output(program: &str, args: &[&str], cwd: Option<&str>, outfile: String) -> Result<(), CommandExitError> {
    let output: Output = Command::new(program)
        .args(args)
        .current_dir(cwd.unwrap_or("."))
        .output()
        .unwrap();
    let path: &Path = Path::new(&outfile);
    let mut file: BufWriter<File> = BufWriter::new(File::create(path).unwrap());
    writeln!(
        &mut file,
        "====== STDOUT ======\n{}\n\n====== STDERR ======\n{}",
        str::from_utf8(&output.stdout).unwrap(),
        str::from_utf8(&output.stderr).unwrap()
    )
    .unwrap();
    file.flush().unwrap();
    wrap_exit(output.status)
}

pub struct CommandExitError {
    code: Option<i32>,
}

impl std::fmt::Debug for CommandExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("CommandExitError").field("code", &self.code).finish()
        f.write_fmt(format_args!("Command exited with a non-zero status code: {:?}", &self.code))
    }
}

pub fn wrap_exit(exit_status: std::process::ExitStatus) -> Result<(), CommandExitError> {
    let code: Option<i32> = exit_status.code();
    if code.is_none() || code.unwrap() != 0 {
        return Err(CommandExitError { code });
    }
    Ok(())
}
