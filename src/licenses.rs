use chrono::Datelike;

pub const LICENSE_NAMES: [&str; 8] = [
    "agpl_3", "apache_2", "gpl_2", "gpl_3", "lgpl_2_1", "lgpl_3", "mit_x11", "mpl_2",
];

static LICENSE_DIR: include_dir::Dir = include_dir::include_dir!("licenses");

pub fn get_short_license(license_name: &str, language: &str) -> String {
    let binding = glib::real_name();

    let author = match binding.to_str() {
        Some(v) => v.to_string(),
        None => std::env::var("USER").unwrap_or("Unknown Author".to_string()),
    };

    let year = chrono::Local::now().year();
    let license = LICENSE_DIR
        .get_file(format!("{}_short", license_name))
        .unwrap()
        .contents_utf8()
        .unwrap();

    let comment_start = match language {
        "python" => "",
        _ => "/* ",
    };

    let comment_end = match language {
        "python" => "",
        _ => " */",
    };

    let comment_block = match language {
        "python" => "# ",
        _ => " * ",
    };

    let mut buffer = String::new();

    buffer.push_str(comment_start);

    let mut first = true;

    for line in license.lines() {
        if first {
            first = false;
        } else {
            buffer.push_str(comment_block);
        }
        let line = line
            .replace("{{year}}", &year.to_string())
            .replace("{{author}}", &author);

        buffer.push_str(&line);
        buffer.push_str("\n");
    }

    buffer.push_str(comment_end);

    buffer.push_str("\n");

    return buffer;
}
