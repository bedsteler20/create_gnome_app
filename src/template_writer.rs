use std::{fs, path::Path};


pub struct TemplateWriter {
    root: String,
}

impl TemplateWriter {
    pub fn new(root: &str) -> Self {
        TemplateWriter {
            root: root.to_string(),
        }
    }

    pub fn write_file(&self, file: &str, content: &str) {
        let path = format!("{}/{}", self.root, file);
        let parent = Path::new(&path).parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::write(path, content).unwrap();
    }
}