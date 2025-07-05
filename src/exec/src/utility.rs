use std::io::{BufRead, BufReader};

pub fn load_file_by_lines<P>(file_path: &str, file_type: &str, has_error: &mut bool, func: P)
where
    P: FnMut(&str) -> Result<(), String>,
{
    let mut invoke = func;
    match std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(file_path)
    {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader
                .lines()
                .map_while(Result::ok)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && !s.starts_with('#'))
                .for_each(|line| {
                    if let Err(e) = invoke(&line) {
                        error!(
                            "Invalid {}: \"{}\" in file {}, {}, ignore this line",
                            file_type, line, file_path, e
                        );
                        *has_error = true;
                    }
                })
        }
        Err(e) => {
            error!(
                "Try to open {} file {} failed, {}, ignore this file",
                file_type, file_path, e
            );
            *has_error = true;
        }
    }
}
