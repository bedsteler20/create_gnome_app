#include "config.h"

#include "{{dashCase projectName}}-window.h"

struct _{{pascalCase projectName}}Window
{
	AdwApplicationWindow  parent_instance;

	/* Template widgets */
	AdwHeaderBar        *header_bar;
	GtkLabel            *label;
};

G_DEFINE_FINAL_TYPE ({{pascalCase projectName}}Window, {{snakeCase projectName}}_window, ADW_TYPE_APPLICATION_WINDOW)

static void
{{snakeCase projectName}}_window_class_init ({{pascalCase projectName}}WindowClass *klass)
{
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

	gtk_widget_class_set_template_from_resource (widget_class, "{{pathCase appId}}/{{dashCase projectName}}-window.ui");
	gtk_widget_class_bind_template_child (widget_class, {{pascalCase projectName}}Window, header_bar);
	gtk_widget_class_bind_template_child (widget_class, {{pascalCase projectName}}Window, label);
}

static void
{{snakeCase projectName}}_window_init ({{pascalCase projectName}}Window *self)
{
	gtk_widget_init_template (GTK_WIDGET (self));
}