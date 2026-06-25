use assert_cmd::cargo::*;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::str::from_utf8;
use std::panic::catch_unwind;

const TESTS_DIRECTORY: &str = "./tests";
const STDOUT_MARK: &str = "// STDOUT";
const STDERR_MARK: &str = "// STDERR";


#[test]
fn run_all_snapshot_tests() -> Result<(), Error> {
    let mut total: i32 = 0;
    let mut passed: i32 = 0;

    let paths: std::fs::ReadDir = std::fs::read_dir(TESTS_DIRECTORY).unwrap();
    for path in paths {
        let dir_path = match path {
            Ok(dir_entry) => {
                if !dir_entry.file_type().unwrap().is_dir() {
                    continue;
                }
                dir_entry.path()
            },
            Err(e) => {
                eprintln!("Error when reading the {} directory: {}", TESTS_DIRECTORY, e);
                continue;
            }
        };

        println!("--- Testing for '{}' ---", dir_path.to_str().unwrap());

        let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir_path).unwrap().map(|e| e.unwrap().path()).collect();
        entries.sort();
        
        let chunks = entries.chunks_exact(2);
        if chunks.remainder().len() != 0 {
            eprintln!("Failed to fetch (input, output) pairs: please make sure the directory is well-structured, with one '.out' file per '.in' file.");
            continue;
        }
        let input_output_pairs: Vec<(&OsStr, &OsStr)> = 
            chunks.map(|chunk| (chunk[0].as_os_str(), chunk[1].as_os_str())).collect();

        total += input_output_pairs.len() as i32;

        for input_output_pair in input_output_pairs {
            let input_path = input_output_pair.0;
            let output_path = input_output_pair.1;

            print!("\tRunning test {}: ", input_path.display());
            let _ = std::io::stdout().flush();

            let mut expected_stdout: String = String::from("");
            let mut expected_stderr: String = String::from("");
            let mut to_stdout: bool = true;
            for line in std::fs::read_to_string(output_path).unwrap().lines() {
                if line.eq_ignore_ascii_case(STDOUT_MARK) {
                    to_stdout = true;
                    continue;
                }
                if line.eq_ignore_ascii_case(STDERR_MARK) {
                    to_stdout = false;
                    continue;
                }

                if to_stdout {
                    expected_stdout.push_str(line);
                    expected_stdout.push('\n');
                } else {
                    expected_stderr.push_str(line);
                    expected_stderr.push('\n');
                }
            }

            let cmd = cargo_bin_cmd!("rlox-tree-walk").arg(input_path).output().expect("Command execution failed.");
            let cmd_stdout: String = from_utf8(&cmd.stdout).expect("Error when trying to read stdout: contains non-utf8 characters.").to_string();
            let cmd_stderr: String = from_utf8(&cmd.stderr).expect("Error when trying to read stderr: contains non-utf8 characters.").to_string();

            match catch_unwind(|| {
                assert_eq!(expected_stdout, cmd_stdout);
            }) {
                Ok(_) => (),
                Err(_) => continue,
            }
            match catch_unwind(|| {
                assert_eq!(expected_stderr, cmd_stderr);
            }) {
                Ok(_) => (),
                Err(_) => continue,
            }

            println!("\n\t\tPASSED");
            passed += 1;
        }

        println!();
    }

    println!("Passed: {}\nTotal: {}", passed, total);
    Ok(())
}