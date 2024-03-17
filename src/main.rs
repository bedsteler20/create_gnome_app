mod engine;
mod formatters;
mod project_config;
mod template_reader;
mod template_writer;
mod generator;

fn main() {
    let config = project_config::ProjectConfig {
        project_name: "My Project".to_string(),
        app_id: "uwu.owo.nayUwU".to_string(),
    };
    let engine = engine::EngineBuilder::new()
        .add_formatter("snakeCase", formatters::snake_case)
        .add_formatter("dashCase", formatters::dash_case)
        .add_formatter("camelCase", formatters::camel_case)
        .add_formatter("pascalCase", formatters::pascal_case)
        .add_formatter("pathCase", formatters::path_case)
        .add_formatter("constCase", formatters::const_case)
        .add_variable("projectName", config.project_name)
        .add_variable("appId", config.app_id)
        .build();

    let reader = template_reader::FsTemplateReader::new("templates/c");
    let writer = template_writer::TemplateWriter::new("output");

    let generator = generator::Generator::new(engine, writer, reader);

    generator.generate();
}
