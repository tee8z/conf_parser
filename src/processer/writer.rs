use crate::processer::FileConf;
use std::fs::File;

use super::PLACE_HOLDER_SECTION;

pub fn write_to_file(file_conf: &FileConf, path: &str) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    write_to(file_conf, file)
}

pub fn write_to<W: std::io::Write>(
    file_conf: &FileConf,
    mut writeable: W,
) -> Result<(), std::io::Error> {
    //TODO: write placeholder section first
    let first_section = file_conf.sections.get(PLACE_HOLDER_SECTION).unwrap();
    for (key, val) in first_section.clone().get_properties().iter() {
        if val.is_empty() {
            writeln!(writeable, "{}", key)?;
            continue;
        }
        writeln!(writeable, "{}={}", key, val)?;
    }
    for (section_name, section) in file_conf.sections.iter() {
        if section_name == PLACE_HOLDER_SECTION {
            continue;
        }
        if !section_name.is_empty() {
            writeln!(writeable, "[{}]", section_name)?;
        }
        for (key, val) in section.clone().get_properties().clone().iter() {
            if val.is_empty() {
                writeln!(writeable, "{}", key)?;
                continue;
            }
            writeln!(writeable, "{}={}", key, val)?;
        }

        writeln!(writeable)?;
    }

    Ok(())
}
