
namespace {{pascalCase projectName}} {
    public class Application : Adw.Application {
        public Application () {
            Object (application_id: "{{appId}}", flags: ApplicationFlags.DEFAULT_FLAGS);
        }

        construct {
            ActionEntry[] action_entries = {
                { "about", this.on_about_action },
                { "preferences", this.on_preferences_action },
                { "quit", this.quit }
            };
            this.add_action_entries (action_entries, this);
            this.set_accels_for_action ("app.quit", {"<primary>q"});
        }

        public override void activate () {
            base.activate ();
            var win = this.active_window;
            if (win == null) {
                win = new {{pascalCase projectName}}.Window (this);
            }
            win.present ();
        }

        private void on_about_action () {
            var about = new Adw.AboutWindow () {
                transient_for = this.active_window,
                application_name = "{{dashCase projectName}}",
                application_icon = "{{appId}}",
                version = "0.1.0",
            };

            about.present ();
        }

        private void on_preferences_action () {
            message ("app.preferences action activated");
        }
    }
}