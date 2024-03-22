use std::fs;

pub struct TemplateReader<'a> {
    language: String,
    dir: include_dir::Dir<'a>,
}

impl TemplateReader<'_> {
    pub fn new(language: String, dir: include_dir::Dir<'static>) -> Self {
        TemplateReader {
            language: language.to_string(),
            dir,
        }
    }

    fn walk_dir(&self, dir: &include_dir::Dir) -> Vec<String> {
        let mut files: Vec<String> = vec![];
        for entry in dir.files() {
            files.push(entry.path().to_string_lossy().to_string());
        }

        for entry in dir.dirs() {
            files.extend(self.walk_dir(entry));
        }

        files
    }

    pub fn list_files(&self) -> Vec<String> {
        self.walk_dir(&self.dir.get_dir(&self.language).unwrap())
    }

    pub fn read_file(&self, file: &str) -> String {
        self.dir
            .get_file(file)
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string()
    }
}
