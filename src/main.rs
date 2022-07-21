use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

/// Counts lines.
///
/// WARNING: Silently ignores invalid files. Could be invalid UTF-8, aliases to nonexistent files,
/// etc.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Directory to recursively traverse.
    dir: PathBuf,

    /// Show stats for each filename extension.
    #[clap(short = 'A')]
    exts: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineStats {
    total: u64,
    empty: u64,
}

impl LineStats {
    fn new() -> LineStats {
        LineStats { total: 0, empty: 0 }
    }

    fn add_empty_line(&mut self) {
        self.total += 1;
        self.empty += 1;
    }

    fn add_non_empty_line(&mut self) {
        self.total += 1;
    }
}

impl AddAssign for LineStats {
    fn add_assign(&mut self, rhs: LineStats) {
        *self = Self {
            total: self.total + rhs.total,
            empty: self.empty + rhs.empty,
        }
    }
}

fn count_file_lines(file_path: &Path) -> io::Result<LineStats> {
    let mut lines = LineStats::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            lines.add_empty_line();
        } else {
            lines.add_non_empty_line();
        }
    }

    Ok(lines)
}

fn count_all_lines(dir: &Path, mut lines: LineStats) -> io::Result<LineStats> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                lines = count_all_lines(&path, lines)?;
            } else {
                // Silently ignore invalid files
                if let Ok(file_lines) = count_file_lines(&path) {
                    lines += file_lines;
                }
            }
        }
    }

    Ok(lines)
}

fn main() {
    let args = Args::parse();

    if args.exts {
    } else {
        let all_lines = match count_all_lines(&args.dir, LineStats::new()) {
            Ok(it) => it,
            Err(err) => panic!("{}", err),
        };

        println!("There are {} lines of code.", all_lines.total);
        println!("There are {} empty lines.", all_lines.empty);
        println!(
            "{}% of the lines are empty.",
            (all_lines.empty as f32 / all_lines.total as f32) * 100.
        )
    }
}
