use clap::{Parser, ValueEnum};
use lzjd::{LZDict, Murmur3HashState};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;

mod stats;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Expecting a directory structure like: ./studentID/
    OneAssignment,
    /// Expecting a directory structure like: ./assignment/studentID/
    AllAssignments,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    /// Use SSDeep for text documents, including source code
    Ssdeep,
    /// Use LZJD for binary documents, such as PDF and popular Office document formats
    Lzjd,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,
    #[arg(value_enum)]
    algo: Algorithm,
    /// The directory to check
    dir: String,
    /// File extensions to check, others will be ignored
    exts: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    if args.mode == Mode::OneAssignment {
        walk_one_assignment(&args.dir, args.algo, &args.exts);
    } else {
        let paths = std::fs::read_dir(&args.dir).unwrap();
        for path in paths.flatten() {
            if path.file_type().unwrap().is_dir() {
                let dir = path.path();
                let dir = dir.to_str().unwrap();
                walk_one_assignment(&dir.to_string(), args.algo, &args.exts);
                println!();
            }
        }
    }
}

fn walk_one_assignment(dir: &String, algo: Algorithm, exts: &Option<Vec<String>>) {
    let mut data: HashMap<String, Vec<u8>> = HashMap::new();

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(e) => {
                if e.file_type().is_file() {
                    let mut process_this_file = true;
                    if exts.is_some() {
                        process_this_file = false;
                        for ext in exts.iter().flatten() {
                            if e.path().to_str().unwrap().ends_with(ext) {
                                process_this_file = true;
                                break;
                            }
                        }
                    }
                    if process_this_file {
                        let mut f = std::fs::File::open(e.path()).unwrap();
                        let mut temp = Vec::new();
                        f.read_to_end(&mut temp).unwrap();
                        if temp.is_empty() {
                            eprintln!("Skipping empty file {:?}", e.path());
                            continue;
                        }
                        let mut dir = e.path().to_path_buf();
                        dir.pop();
                        let dir = dir.into_os_string();
                        let dir = dir.to_str().unwrap();
                        if data.contains_key(dir) {
                            data.get_mut(dir).unwrap().append(&mut temp);
                        } else {
                            data.insert(dir.to_string(), temp);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error {:?}", e);
            }
        }
    }

    if data.is_empty() {
        eprintln!("No files found.");
        return;
    }

    let mut hashes: HashMap<String, String> = HashMap::new();
    for (dir, file_data) in data.iter() {
        if algo == Algorithm::Lzjd {
            let build_hasher = Murmur3HashState::new();
            let h = LZDict::from_bytes_stream(file_data.iter().copied(), &build_hasher);
            hashes.insert(dir.clone(), h.to_string());
        } else {
            hashes.insert(dir.clone(), ssdeep::hash(file_data).unwrap());
        }
    }

    let mut similarities: HashMap<String, i8> = HashMap::new();
    let mut similarites_stats = stats::Similarities::new();
    for (dir_outer, hash_outer) in hashes.iter() {
        for (dir_inner, hash_inner) in hashes.iter() {
            if dir_inner.eq(dir_outer) {
                continue;
            }
            if !similarities.contains_key(format!("{}|{}", dir_inner, dir_outer).as_str())
                && !similarities.contains_key(format!("{}|{}", dir_outer, dir_inner).as_str())
            {
                let similarity = match algo {
                    Algorithm::Lzjd => {
                        let a = lzjd::LZDict::from_base64_string(hash_inner).unwrap();
                        let b = lzjd::LZDict::from_base64_string(hash_outer).unwrap();
                        let result = a.similarity(&b) * 100.0;
                        result as i8
                    }
                    Algorithm::Ssdeep => {
                        ssdeep::compare(hash_inner.as_bytes(), hash_outer.as_bytes()).unwrap()
                    }
                };

                if similarity > 0 {
                    similarities.insert(format!("{}|{}", dir_inner, dir_outer), similarity);
                }
                similarites_stats.add(similarity);
            }
        }
    }

    if similarites_stats.empty() || similarites_stats.avg() == 0.00 {
        eprintln!("Nothing is similar.");
        if exts.is_none() {
            eprintln!(
                "Maybe try limiting by file extension? Run with `--help` for more information."
            );
        }
        return;
    }

    println!(
        "Average of {} comparisons: {:.2}, Std. Dev. {:.2}",
        similarites_stats.len(),
        similarites_stats.avg(),
        similarites_stats.std_dev()
    );
    println!(
        "Excluding {} zeros, average of {} comparisons: {:.2}, Std. Dev. {:.2}",
        similarites_stats.num_zeroes(),
        similarites_stats.len_non_zeroes(),
        similarites_stats.avg_non_zeroes(),
        similarites_stats.std_dev_non_zeroes()
    );

    for (dirs, similarity) in similarities.iter() {
        let mut dir_parts = dirs.split('|');
        let first = dir_parts.next().unwrap();
        let second = dir_parts.next().unwrap();
        if *similarity >= 95i8 {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            writeln!(&mut stdout, "{} vs {}: {}", first, second, similarity).unwrap();
            stdout.reset().unwrap();
        } else {
            println!("{} vs {}: {}", first, second, similarity);
        }
    }
}
