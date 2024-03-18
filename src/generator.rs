use crate::{
    engine::Engine,
    template_reader::TemplateReader,
    template_writer::TemplateWriter,
};

pub struct Generator {
    engine: Engine,
    template_writer: TemplateWriter,
    template_reader: TemplateReader,
}

impl Generator {
    pub fn new(
        engine: Engine,
        template_writer: TemplateWriter,
        template_reader: TemplateReader,
    ) -> Self {
        Generator {
            engine,
            template_writer,
            template_reader,
        }
    }

    pub fn generate(&self) {
        let files = self.template_reader.list_files();
        for file in files {
            let content = self.template_reader.read_file(&file);
            let formatted_content = self.engine.format(&content);
            let output_file = self.engine.format(&file);
            self.template_writer
                .write_file(&output_file, &formatted_content);
        }
    }
}
