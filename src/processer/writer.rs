use std::fs::File;
use std::io::Write;
use crate::processer::FileConf;


pub fn write_to_file(file_conf: &FileConf, path: &str) -> Result<(),  std::io::Error> {
    let mut file = File::create(path)?;

    for (section_name, section) in file_conf.sections.iter() {
        writeln!(file, "[{}]", section_name)?;

        for (key, val) in section.properties.iter() {
            writeln!(file, "{}={}", key, val)?;
        }

        writeln!(file)?;
    }
    
    Ok(())
}