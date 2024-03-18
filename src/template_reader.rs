use std::fs;

#[cfg(not(feature = "bundle_templates"))]
pub struct TemplateReader {
    root: String,
}

#[cfg(not(feature = "bundle_templates"))]
impl TemplateReader {
    pub fn new(root: String) -> Self {
        TemplateReader {
            root: format!("{}/templates/{}", env!("CARGO_MANIFEST_DIR"), root),
        }
    }

    fn walk_dir(&self, dir: &str) -> Vec<String> {
        fs::read_dir(&dir)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    self.walk_dir(&path.to_string_lossy())
                } else {
                    vec![path.to_string_lossy().to_string()]
                }
            })
            .flatten()
            .map(|path| path.replace(&self.root, ""))
            .map(|path| {
                if path.starts_with("/") {
                    path[1..].to_string()
                } else {
                    path
                }
            })
            .collect()
    }

    pub fn list_files(&self) -> Vec<String> {
        self.walk_dir(&self.root)
    }

    pub fn read_file(&self, file: &str) -> String {
        fs::read_to_string(format!("{}/{}", self.root, file)).unwrap()
    }
}

#[cfg(feature = "bundle_templates")]
static TEMPLATE_DIR: include_dir::Dir = include_dir::include_dir!("templates");
#[cfg(feature = "bundle_templates")]
pub struct TemplateReader {
    language: String,
}

#[cfg(feature = "bundle_templates")]
impl TemplateReader {
    pub fn new(language: String) -> Self {
        TemplateReader {
            language: language.to_string(),
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
        self.walk_dir(&TEMPLATE_DIR.get_dir(&self.language).unwrap())
    }

    pub fn read_file(&self, file: &str) -> String {
        TEMPLATE_DIR
            .get_file(file)
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_files() {
        let reader = TemplateReader::new("templates/python".to_string());
        let files = reader.list_files();
        assert!(files.contains(&"requirements.txt".to_string()));
        assert!(files.contains(&".vscode/launch.json".to_string()));
        assert!(files.contains(&"meson.build".to_string()));
    }

    #[test]
    fn test_read_file() {
        let reader = TemplateReader::new("templates/python".to_string());
        let file = reader.read_file("meson.build");
        assert!(file.contains("project('{{dashCase projectName}}"));
    }
}
