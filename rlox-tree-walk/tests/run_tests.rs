use assert_cmd::cargo::*;
use predicates::prelude::*;
use walkdir::WalkDir;
use std::collections::HashSet;

#[test]
fn run_tests_interpreter() -> Result<(), Box<dyn std::error::Error>> {
    // Tests that evaluate runtime errors
    let mut error_test_files: HashSet<&str> = HashSet::new();
    error_test_files.insert("inner_var_not_leaked.lox");

    let mut total = 0;
    let mut passed = 0;

    for entry in WalkDir::new("./tests/").into_iter().filter_map(|e| e.ok()) {
        match entry.path().extension() {
            Some(ext) => {
                if !ext.eq_ignore_ascii_case("lox") {
                    continue;
                }
            },
            None => continue,
        }

        total += 1;

        let input_file: String = entry.path().to_str().unwrap().to_string();
        let output_file: String = input_file.strip_suffix(".lox").unwrap().to_string() + ".out";

        println!("Testing: '{input_file}'.");

        let expected_output: String = std::fs::read_to_string(&output_file)
                                        .unwrap_or_else(|_| panic!("Couldn't open {output_file} file."))
                                        .trim_end()
                                        .to_string() + "\n";

        let mut cmd = cargo_bin_cmd!("rlox-tree-walk");
        cmd.arg(input_file);

        if error_test_files.contains(entry.file_name().to_str().unwrap()) {
            cmd.assert().failure().stdout(predicate::str::diff(expected_output));
        } else {
            cmd.assert().success().stdout(predicate::str::diff(expected_output));
        }

        passed += 1;
    }

    println!("\nTotal: {total}\n\tPassed: {passed}; Failed: {};", total-passed);

    Ok(())
}