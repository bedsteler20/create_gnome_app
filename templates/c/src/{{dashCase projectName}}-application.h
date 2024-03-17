#pragma once

#include <adwaita.h>

G_BEGIN_DECLS

#define {{constCase projectName}}_TYPE_APPLICATION ({{snakeCase projectName}}_application_get_type())

G_DECLARE_FINAL_TYPE ({{pascalCase projectName}}Application, {{snakeCase projectName}}_application, {{constCase projectName}}, APPLICATION, AdwApplication)

{{pascalCase projectName}}Application *{{snakeCase projectName}}_application_new (const char        *application_id,
                                                 GApplicationFlags  flags);

G_END_DECLS