use std::collections::HashMap;

#[derive(Debug)]
pub struct FileConf {
    pub sections: HashMap<String, Section>
}

impl FileConf {
    pub fn new() -> Self {
        FileConf{
            sections: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub properties: HashMap<String, String>
}

impl Section {
    pub fn new() -> Self {
        Section { properties: HashMap::new() }
    }
}