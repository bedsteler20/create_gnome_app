mod engine;
mod formatters;
mod generator;
mod template_reader;
mod template_writer;
use clap::{App, Arg};
use inquire::{Select, Text};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LANGUAGES: [&str; 4] = ["c", "python", "typescript", "vala"];

fn validate_app_id(app_id: &str) -> Result<(), String> {
    let period_count = app_id.chars().filter(|c| *c == '.').count();
    if period_count >= 2 {
        Ok(())
    } else {
        Err("App id must contain at least 2 periods".to_string())
    }
}

fn main() {
    let matches = App::new("create-gnome-app")
        .version(VERSION)
        .arg(
            Arg::new("appId")
                .short('a')
                .long("appId")
                .value_name("APP_ID")
                .help("Sets the app id")
                .takes_value(true)
                .validator(validate_app_id),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("Sets the language")
                .takes_value(true)
                .possible_values(&LANGUAGES),
        )
        .arg(
            Arg::new("projectName")
                .short('p')
                .long("projectName")
                .value_name("PROJECT_NAME")
                .help("Sets the project name")
                .takes_value(true),
        )
        .arg(
            Arg::new("outputDir")
                .short('o')
                .long("outputDir")
                .value_name("OUTPUT_DIR")
                .help("Sets the output directory")
                .takes_value(true),
        )
        .get_matches();

    let app_id = if let Some(app_id) = matches.value_of("appId") {
        app_id.to_string()
    } else {
        loop {
            let app_id = Text::new("Enter the app id:").prompt().unwrap();
            if let Err(err) = validate_app_id(&app_id) {
                eprintln!("{}", err);
                continue;
            }
            break app_id;
        }
    };

    let language = if let Some(language) = matches.value_of("language") {
        language.to_string()
    } else {
        let language = Select::new("Select the language:", (&LANGUAGES).to_vec())
            .prompt()
            .unwrap();
        language.to_string()
    };

    let project_name = if let Some(project_name) = matches.value_of("projectName") {
        project_name.to_string()
    } else {
        Text::new("Enter the project name:").prompt().unwrap()
    };

    let output_dir = if let Some(output_dir) = matches.value_of("outputDir") {
        output_dir.to_string()
    } else {
        format!("./{}", project_name)
    };

    // let template_reader =
    // template_reader::FsTemplateReader::new(&format!("./templates/{}", language));
    let template_reader = template_reader::TemplateReader::new(language);
    let template_writer = template_writer::TemplateWriter::new(&format!("./{}", output_dir));

    let engine = engine::EngineBuilder::new()
        .add_formatter("snakeCase", formatters::snake_case)
        .add_formatter("dashCase", formatters::dash_case)
        .add_formatter("camelCase", formatters::camel_case)
        .add_formatter("pascalCase", formatters::pascal_case)
        .add_formatter("pathCase", formatters::path_case)
        .add_formatter("constCase", formatters::const_case)
        .add_variable("projectName", project_name, true)
        .add_variable("appId", app_id, true)
        .build();

    let generator = generator::Generator::new(engine, template_writer, template_reader);

    generator.generate();
}
