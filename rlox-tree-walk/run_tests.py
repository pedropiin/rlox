#!/usr/bin/env python3
"""
Test runner for rlox-tree-walk.

Runs all .lox test files and compares actual output against
expected output embedded in comments.

Expected output format in .lox files:
    // Expected output:
    // line1
    // line2
    // [runtime error on last line]   <-- ignored (not compared)

Usage:
    python3 run_tests.py                    # run all tests
    python3 run_tests.py lox-tests/while/   # run tests in a specific folder
"""

import subprocess
import sys
import re
from pathlib import Path

BINARY = Path("./target/debug/rlox-tree-walk")


def extract_expected_output(test_file: Path) -> list[str] | None:
    """Extract expected output lines from a .lox file. Returns None if no expected output section."""
    lines = test_file.read_text().splitlines()
    collecting = False
    expected = []

    for line in lines:
        if line.strip() == "// Expected output:":
            collecting = True
            continue
        if collecting:
            if line.startswith("// "):
                content = line[3:]
                # Skip meta-comments like [runtime error on last line]
                if re.match(r"^\[.*\]$", content):
                    continue
                expected.append(content)
            elif line.strip() == "//":
                expected.append("")
            else:
                # Stop collecting when we hit a non-comment line
                break

    return expected if collecting else None


def run_test(test_file: Path) -> tuple[bool, str]:
    """Run a single test. Returns (passed, details)."""
    expected = extract_expected_output(test_file)
    if expected is None:
        return True, "skipped (no expected output)"

    try:
        result = subprocess.run(
            [str(BINARY), str(test_file)],
            capture_output=True,
            text=True,
            timeout=10,
        )
        actual_lines = result.stdout.rstrip("\n").splitlines() if result.stdout.strip() else []
    except subprocess.TimeoutExpired:
        return False, "TIMEOUT (10s)"

    if actual_lines == expected:
        return True, ""

    detail_lines = []
    max_lines = max(len(expected), len(actual_lines))
    for i in range(max_lines):
        exp = expected[i] if i < len(expected) else "<missing>"
        act = actual_lines[i] if i < len(actual_lines) else "<missing>"
        marker = "  " if exp == act else "!!"
        detail_lines.append(f"    {marker} expected: {exp!r}")
        if exp != act:
            detail_lines.append(f"    {marker}      got: {act!r}")

    return False, "\n".join(detail_lines)


def main():
    test_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("lox-tests")

    # Build first
    print("Building...")
    build = subprocess.run(["cargo", "build"], capture_output=True, text=True)
    if build.returncode != 0:
        print("Build failed:")
        print(build.stderr)
        sys.exit(1)
    print()

    if not BINARY.exists():
        print(f"Binary not found at {BINARY}")
        sys.exit(1)

    test_files = sorted(test_dir.rglob("*.lox"))
    if not test_files:
        print(f"No .lox files found in {test_dir}")
        sys.exit(1)

    passed = 0
    failed = 0
    skipped = 0

    for test_file in test_files:
        success, details = run_test(test_file)
        if "skipped" in details:
            skipped += 1
            continue
        if success:
            print(f"  \033[32mPASS\033[0m  {test_file}")
            passed += 1
        else:
            print(f"  \033[31mFAIL\033[0m  {test_file}")
            if details:
                print(details)
            failed += 1

    total = passed + failed
    print()
    color = "\033[32m" if failed == 0 else "\033[31m"
    print(f"{color}{passed}/{total} passed, {failed} failed\033[0m", end="")
    if skipped:
        print(f" ({skipped} skipped)", end="")
    print()

    sys.exit(1 if failed > 0 else 0)


if __name__ == "__main__":
    main()
