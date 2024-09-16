use crate::processer::{FileConf, Section};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PLACE_HOLDER_SECTION: &str = "*placeholder*";

pub fn read_to_file_conf(file: &std::fs::File) -> Result<FileConf, std::io::Error> {
    read_from(file)
}

pub fn read_from<R: std::io::Read>(readable: R) -> Result<FileConf, std::io::Error> {
    let reader = BufReader::new(readable);
    let mut file_conf = FileConf::new();
    let mut last_section: String = PLACE_HOLDER_SECTION.to_owned();
    file_conf
        .sections
        .insert(last_section.clone(), Section::new());
    let skip_chars = &['#', ';'];
    for line in reader.lines().map_while(Result::ok) {
        if skip_chars.iter().any(|&c| line.starts_with(c)) {
            continue; // Skip the line if it starts with any character in `skip_chars`
        }
        if let Some(extracted_substring) = extract_section_name(&line) {
            last_section = extracted_substring.to_string();
            file_conf
                .sections
                .insert(last_section.clone(), Section::new());
            continue;
        }
        // Getting a config value that is a flag not an examples
        if !line.contains('=') {
            let section = file_conf.sections.get_mut(&last_section.clone()).unwrap();
            section.set_property(&line, "");
            continue;
        }

        if let [key, value] = line.clone().splitn(2, '=').collect::<Vec<&str>>()[..] {
            let section = file_conf.sections.get_mut(&last_section.clone()).unwrap();
            section.set_property(key, value);
        }
    }
    Ok(file_conf)
}

pub fn read_to_file_conf_mut(file_content: &File) -> Result<&mut FileConf, std::io::Error> {
    let mut_conf = read_to_file_conf(file_content)?;
    let boxed_conf = Box::new(mut_conf);
    let mut_ref = Box::leak(boxed_conf);
    Ok(mut_ref)
}

fn extract_section_name(line: &str) -> Option<&str> {
    line.split('[')
        .nth(1)
        .and_then(|substring| substring.split(']').next())
}
