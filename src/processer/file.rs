use std::collections::HashMap;

//TODO: add methods to make it easy to update these values

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Section {
    pub properties: HashMap<String, String>
}

impl Section {
    pub fn new() -> Self {
        Section { properties: HashMap::new() }
    }
    pub fn get_property(self, name: &str) -> String {
        match self.properties.get(name){
            Some(item) => item.to_string(),
            None => "".to_string()
        }
    }
}