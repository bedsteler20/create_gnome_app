#pragma once

#include <adwaita.h>

G_BEGIN_DECLS

#define {{constCase projectName}}_TYPE_WINDOW ({{snakeCase projectName}}_window_get_type())

G_DECLARE_FINAL_TYPE ({{pascalCase projectName}}Window, {{snakeCase projectName}}_window, {{constCase projectName}}, WINDOW, AdwApplicationWindow)

G_END_DECLS