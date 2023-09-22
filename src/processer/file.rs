use std::collections::HashMap;

//TODO: add methods to make it easy to update these values

#[derive(Default, Debug, Clone)]
pub struct FileConf {
    pub sections: HashMap<String, Section>,
}

impl FileConf {
    pub fn new() -> Self {
        FileConf {
            sections: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    properties: HashMap<String, String>,
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Section {
    pub fn new() -> Self {
        Section {
            properties: HashMap::new(),
        }
    }
    pub fn get_properties(self) -> HashMap<String, String> {
        self.properties
    }
    pub fn get_property(&self, name: &str) -> String {
        match self.properties.get(name) {
            Some(item) => item.to_string(),
            None => "".to_string(),
        }
    }
    pub fn set_property(&mut self, name: &str, value: &str) {
        if self.properties.get("name").is_some() {
            self.properties.remove("name");
        }
        self.properties.insert(name.to_owned(), value.to_owned());
    }
}
