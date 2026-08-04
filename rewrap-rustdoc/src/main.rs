use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, ErrorKind, Write as _};
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use clap::Parser;

mod md;

#[derive(Debug, Parser)]
struct Args {
    path: PathBuf,

    #[arg(short, long, default_value_t = 100)]
    max_line_len: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    process_path(&args.path, args.max_line_len.into())?;
    Ok(())
}

fn process_path(path: &Path, max_line_len: usize) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            process_path(&entry_path, max_line_len)?;
        }
    } else if path.extension().is_some_and(|ext| ext == "rs") {
        process_file(path, max_line_len)?;
    }

    Ok(())
}

fn process_file(input_path: &Path, max_line_len: usize) -> Result<()> {
    println!("Processing {}", input_path.display());

    let input_file = BufReader::new(File::open(input_path)?);

    let (mut output_file, output_path) = {
        let tmp_path = input_path.with_added_extension("tmp");
        let mut path = tmp_path.clone();
        let mut counter = 0;

        loop {
            let result = OpenOptions::new().create_new(true).write(true).open(&path);

            match result {
                Ok(file) => break (file, path),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                    counter += 1;
                    path = tmp_path.with_added_extension(counter.to_string());
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    let mut heading_and_rustdoc: Option<(Heading, Rustdoc)> = None;

    for line in input_file.lines() {
        let line = line?;
        if let Some((heading, rustdoc)) = &mut heading_and_rustdoc {
            if let Some(ln) = heading.strip_from(&line) {
                rustdoc.push_line(ln)?;
            } else {
                rewrap(&mut output_file, rustdoc, heading, max_line_len)?;
                heading_and_rustdoc = None;
                writeln!(output_file, "{line}")?;
            }
        } else {
            if let Some((heading, ln)) = Heading::extract_from(&line) {
                let mut rustdoc = Rustdoc::new();
                rustdoc.push_line(ln)?;
                heading_and_rustdoc = Some((heading, rustdoc));
            } else {
                writeln!(output_file, "{line}")?;
            }
        }
    }

    if let Some((heading, doc_lines)) = heading_and_rustdoc {
        rewrap(&mut output_file, &doc_lines, &heading, max_line_len)?;
    }

    fs::rename(output_path, input_path)?;

    Ok(())
}

fn rewrap(
    output_file: &mut File,
    rustdoc: &Rustdoc,
    heading: &Heading,
    max_line_len: usize,
) -> Result<()> {
    let node = markdown::to_mdast(&rustdoc.0, &markdown::ParseOptions::default())
        .map_err(|e| anyhow!(e))?;

    let output_lines = md::lines_from_node(node, max_line_len - heading.0.chars().count() - 1);
    for line in output_lines {
        heading.write_line(&mut *output_file, &line)?;
    }

    Ok(())
}

struct Rustdoc(String);

impl Rustdoc {
    fn new() -> Self {
        Rustdoc(String::new())
    }

    fn push_line(&mut self, line: &str) -> Result<()> {
        writeln!(&mut self.0, "{line}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadingMarker {
    External,
    Internal,
}

impl HeadingMarker {
    fn str(self) -> &'static str {
        match self {
            HeadingMarker::External => "///",
            HeadingMarker::Internal => "//!",
        }
    }
}

const HEADING_MARKERS: [HeadingMarker; 2] = [HeadingMarker::External, HeadingMarker::Internal];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Heading(String);

impl Heading {
    fn extract_from(line: &str) -> Option<(Self, &str)> {
        let space_len = line
            .char_indices()
            .find_map(|(i, c)| (c != ' ').then_some(i))
            .unwrap_or(0);
        let trimmed_line = &line[space_len..];

        let marker_and_stripped_line = HEADING_MARKERS.iter().find_map(|marker| {
            trimmed_line
                .strip_prefix(marker.str())
                .map(|stripped_line| (*marker, stripped_line))
        });

        marker_and_stripped_line.and_then(|(marker, stripped_line)| {
            Self::normalize_stripped_line(stripped_line).map(|ln| {
                let h = line[..space_len + marker.str().chars().count()].to_string();
                (Heading(h), ln)
            })
        })
    }

    fn strip_from<'b>(&self, line: &'b str) -> Option<&'b str> {
        line.strip_prefix(&self.0)
            .and_then(Self::normalize_stripped_line)
    }

    fn write_line<W: io::Write>(&self, mut output: W, mut line: &str) -> io::Result<()> {
        write!(output, "{}", self.0)?;

        line = line.trim_end();
        if !line.is_empty() {
            write!(output, " {line}")?;
        }

        writeln!(output)?;

        Ok(())
    }

    fn normalize_stripped_line(line: &str) -> Option<&str> {
        let first_char = line.chars().next();
        match first_char {
            Some(' ') => Some(&line[1..]),
            Some(_) => None,
            None => Some(line),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extract_heading {
        use super::*;

        #[test]
        fn cases() {
            assert_eq!(
                Heading::extract_from("/// abc"),
                Some((Heading("///".to_string()), "abc")),
            );
            assert_eq!(
                Heading::extract_from("//! abc"),
                Some((Heading("//!".to_string()), "abc"))
            );
            assert_eq!(
                Heading::extract_from("    /// abc"),
                Some((Heading("    ///".to_string()), "abc"))
            );
            assert_eq!(
                Heading::extract_from("    //! abc"),
                Some((Heading("    //!".to_string()), "abc"))
            );

            assert_eq!(Heading::extract_from("abc"), None);
            assert_eq!(Heading::extract_from("// abc"), None);
            assert_eq!(Heading::extract_from("///abc"), None);
            assert_eq!(Heading::extract_from("//// abc"), None);
            assert_eq!(Heading::extract_from("//a bc"), None);
            assert_eq!(Heading::extract_from("    abc"), None);
            assert_eq!(Heading::extract_from("    // abc"), None);
            assert_eq!(Heading::extract_from("    ///abc"), None);
            assert_eq!(Heading::extract_from("    //// abc"), None);
            assert_eq!(Heading::extract_from("    //a bc"), None);
        }
    }
}
