use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::{Arg, Command};

struct Tab2Space {
    tab_width: usize,
    safe_mode: bool,
    erase_trailing_space: bool,
    code_exts: Vec<&'static str>,
}

impl Tab2Space {
    fn new(tab_width: usize, safe_mode: bool, erase_trailing_space: bool, code_exts: Vec<&'static str>) -> Self {
        Tab2Space {
            tab_width,
            safe_mode,
            erase_trailing_space,
            code_exts,
        }
    }

    fn tab2space_line(&self, line: &str) -> String {
        let mut line_idx = 0;
        let mut newline = String::new();
        let mut current_line = line;

        while let Some(tab_idx) = current_line.find('\t') {
            newline.push_str(&current_line[..tab_idx]);
            line_idx += current_line[..tab_idx].chars().count();

            let tab_count = current_line.chars().take_while(|&c| c == '\t').count();
            let space_num = if tab_count * self.tab_width >= (line_idx % self.tab_width) {
                tab_count * self.tab_width - (line_idx % self.tab_width)
            } else {
                0
            };
            
            line_idx += space_num;
            newline.push_str(&" ".repeat(space_num));
            current_line = &current_line[tab_idx + tab_count..];
        }

        newline.push_str(current_line);
        if self.erase_trailing_space {
            newline = newline.trim_end().to_string();
        }
        newline
    }

    fn tab2space_file(&self, input_file_path: &Path, output_file_path: Option<&Path>) -> io::Result<()> {
        println!("[Process] Parsing {:?}", input_file_path);
        let output_file_path = if let Some(path) = output_file_path {
            path.to_path_buf()
        } else {
            if self.safe_mode {
                let mut new_path = input_file_path.with_extension("");
                new_path.set_extension("notab");
                new_path
            } else {
                input_file_path.to_path_buf()
            }
        };

        let content = fs::read_to_string(input_file_path)?;

        let new_content: String = content
            .split_inclusive(['\n'])
            .map(|line| {
                // Preserve the line ending (\n or \r\n) by splitting based on the length of the line ending
                if line.ends_with("\r\n") {
                    let (line_content, line_ending) = line.split_at(line.len() - 2);
                    format!("{}{}", self.tab2space_line(line_content), line_ending)
                } else if line.ends_with('\n') {
                    let (line_content, line_ending) = line.split_at(line.len() - 1);
                    format!("{}{}", self.tab2space_line(line_content), line_ending)
                } else {
                    self.tab2space_line(line)
                }
            })
            .collect();

        if let Some(parent) = output_file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output_file = File::create(output_file_path)?;
        write!(output_file, "{}", new_content)?;

        Ok(())
    }

    fn all_code_files(&self, folder: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(self.all_code_files(&path)?);
            } else if let Some(ext) = path.extension() {
                if self.code_exts.iter().any(|&e| ext == e) {
                    files.push(path);
                }
            }
        }
        Ok(files)
    }

    fn tab2space_all_codes(&self, input_folder: &Path) -> io::Result<()> {
        let output_folder = if self.safe_mode {
            input_folder.with_file_name(format!("{}_notab", input_folder.display()))
        } else {
            input_folder.to_path_buf()
        };

        for file_name in self.all_code_files(input_folder)? {
            let input_file_path = input_folder.join(&file_name);
            let output_file_path = output_folder.join(&file_name);
            if let Err(e) = self.tab2space_file(&input_file_path, Some(&output_file_path)) {
                eprintln!("[Warning] Failed to process {:?}: {:?}", input_file_path, e);
            }
        }
        Ok(())
    }
}

fn main() {
    let matches = Command::new("Tab2Space")
        .version("1.0")
        .author("Rust Conversion")
        .about("Convert tabs to spaces in files or folders")
        .arg(Arg::new("path")
            .help("The path of target")
            .required(true)
            .index(1))
        .arg(Arg::new("is_folder")
            .short('f')
            .long("is_folder")
            .help("Add flag if target is folder")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("overwrite")
            .short('o')
            .long("overwrite")
            .help("Use unsafe mode")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("rtw")
            .short('s')
            .long("rtw")
            .help("Remove trailing space")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("tab_width")
            .short('w')
            .long("tab_width")
            .help("Width of tab")
            .num_args(1)
            .default_value("4"))
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let is_folder = !!matches.get_one::<bool>("is_folder").unwrap();
    let safe_mode = !matches.get_one::<bool>("overwrite").unwrap();
    let erase_trailing_space = !!matches.get_one::<bool>("rtw").unwrap();
    let tab_width: usize = matches.get_one::<String>("tab_width").unwrap().parse().unwrap();

    let t2s = Tab2Space::new(tab_width, safe_mode, erase_trailing_space, vec!["c", "cc", "h", "py", "cs"]);
    let path = Path::new(path);

    if is_folder {
        if let Err(e) = t2s.tab2space_all_codes(&path) {
            eprintln!("[Error] {:?}", e);
        }
    } else {
        if let Err(e) = t2s.tab2space_file(path, None) {
            eprintln!("[Error] {:?}", e);
        }
    }
}
