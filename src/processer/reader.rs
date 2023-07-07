use crate::processer::{FileConf, Section};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_to_file_conf(file_content: &File) -> Result<FileConf, std::io::Error> {
    let reader = BufReader::new(file_content);
    let mut file_conf = FileConf::new();
    let mut last_section: String = "".to_owned();
    let skip_chars = &['#', ';'];
    for line in reader.lines().filter_map(Result::ok) {
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
        if let [key, value] = line.clone().splitn(2, '=').collect::<Vec<&str>>()[..] {
            let section = file_conf.sections.get_mut(&last_section.clone()).unwrap();
            section.properties.insert(key.to_string(), value.to_owned());
        }
    }
    Ok(file_conf)
}

fn extract_section_name<'a>(line: &'a String) -> Option<&'a str> {
    line.split('[')
        .nth(1)
        .and_then(|substring| substring.split(']').next())
}