namespace {{pascalCase projectName}} {
    [GtkTemplate (ui = "{{pathCase appId}}/window.ui")]
    public class Window : Adw.ApplicationWindow {
        [GtkChild]
        private unowned Gtk.Label label;

        public Window (Gtk.Application app) {
            Object (application: app);
        }
    }
}