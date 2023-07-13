use crate::processer::FileConf;
use std::fs::File;
use std::io::Write;

use super::PLACE_HOLDER_SECTION;

pub fn write_to_file(file_conf: &FileConf, path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    //TODO: write placeholder section first
    let first_section = file_conf.sections.get(PLACE_HOLDER_SECTION).unwrap();
    for (key, val) in first_section.clone().get_properties().clone().iter() {
        writeln!(file, "{}={}", key, val)?;
    }
    for (section_name, section) in file_conf.sections.iter() {
        if section_name == PLACE_HOLDER_SECTION {
            continue;
        }

        writeln!(file, "[{}]", section_name)?;
        for (key, val) in section.clone().get_properties().clone().iter() {
            writeln!(file, "{}={}", key, val)?;
        }

        writeln!(file)?;
    }

    Ok(())
}
