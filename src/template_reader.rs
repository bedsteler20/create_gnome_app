use std::fs;

pub struct FsTemplateReader {
    root: String,
}

impl FsTemplateReader {
    pub fn new(root: &str) -> Self {
        FsTemplateReader {
            root: root.to_string(),
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
}

impl TemplateReader for FsTemplateReader {
    fn list_files(&self) -> Vec<String> {
        self.walk_dir(&self.root)
    }

    fn read_file(&self, file: &str) -> String {
        fs::read_to_string(format!("{}/{}", self.root, file)).unwrap()
    }
}

pub trait TemplateReader {
    fn list_files(&self) -> Vec<String>;
    fn read_file(&self, file: &str) -> String;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_files() {
        let reader = FsTemplateReader::new("templates/python");
        let files = reader.list_files();
        println!("{:?}", files);
        assert!(files.contains(&"requirements.txt".to_string()));
        assert!(files.contains(&".vscode/launch.json".to_string()));
        assert!(files.contains(&"meson.build".to_string()));
    }

    #[test]
    fn test_read_file() {
        let reader = FsTemplateReader::new("templates/python");
        let file = reader.read_file("meson.build");
        assert!(file.contains("project('{{dashCase projectName}}"));
    }
}
