#include "config.h"

#include "{{dashCase projectName}}-application.h"
#include "{{dashCase projectName}}-window.h"

struct _{{pascalCase projectName}}Application
{
	AdwApplication parent_instance;
};

G_DEFINE_TYPE ({{pascalCase projectName}}Application, {{snakeCase projectName}}_application, ADW_TYPE_APPLICATION)

{{pascalCase projectName}}Application *
{{snakeCase projectName}}_application_new (const char        *application_id,
                            GApplicationFlags  flags)
{
	g_return_val_if_fail (application_id != NULL, NULL);

	return g_object_new ({{constCase projectName}}_TYPE_APPLICATION,
	                     "application-id", application_id,
	                     "flags", flags,
	                     NULL);
}

static void
{{snakeCase projectName}}_application_activate (GApplication *app)
{
	GtkWindow *window;

	g_assert ({{constCase projectName}}_IS_APPLICATION (app));

	window = gtk_application_get_active_window (GTK_APPLICATION (app));

	if (window == NULL)
		window = g_object_new ({{constCase projectName}}_TYPE_WINDOW,
		                       "application", app,
		                       NULL);

	gtk_window_present (window);
}

static void
{{snakeCase projectName}}_application_class_init ({{pascalCase projectName}}ApplicationClass *klass)
{
	GApplicationClass *app_class = G_APPLICATION_CLASS (klass);

	app_class->activate = {{snakeCase projectName}}_application_activate;
}

static void
{{snakeCase projectName}}_application_about_action (GSimpleAction *action,
                                     GVariant      *parameter,
                                     gpointer       user_data)
{
	{{pascalCase projectName}}Application *self = user_data;
	GtkWindow *window = NULL;

	g_assert ({{constCase projectName}}_IS_APPLICATION (self));

	window = gtk_application_get_active_window (GTK_APPLICATION (self));

	adw_show_about_window (window,
	                       "application-name", "{{dashCase projectName}}",
	                       "application-icon", "{{appId}}",
	                       "version", "0.1.0",
	                       NULL);
}

static void
{{snakeCase projectName}}_application_quit_action (GSimpleAction *action,
                                    GVariant      *parameter,
                                    gpointer       user_data)
{
	{{pascalCase projectName}}Application *self = user_data;

	g_assert ({{constCase projectName}}_IS_APPLICATION (self));

	g_application_quit (G_APPLICATION (self));
}

static const GActionEntry app_actions[] = {
	{ "quit", {{snakeCase projectName}}_application_quit_action },
	{ "about", {{snakeCase projectName}}_application_about_action },
};

static void
{{snakeCase projectName}}_application_init ({{pascalCase projectName}}Application *self)
{
	g_action_map_add_action_entries (G_ACTION_MAP (self),
	                                 app_actions,
	                                 G_N_ELEMENTS (app_actions),
	                                 self);
	gtk_application_set_accels_for_action (GTK_APPLICATION (self),
	                                       "app.quit",
	                                       (const char *[]) { "<primary>q", NULL });
}