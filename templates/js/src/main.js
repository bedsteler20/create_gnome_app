{{license}}
import GObject from "gi://GObject";
import Gio from "gi://Gio";
import Gtk from "gi://Gtk?version=4.0";
import Adw from "gi://Adw?version=1";

import { {{pascalCase projectName}}Window } from "./window.js";

pkg.initGettext();
pkg.initFormat();

export const {{pascalCase projectName}}Application = GObject.registerClass(
    class {{pascalCase projectName}}Application extends Adw.Application {
        constructor() {
            super({
                application_id: "{{appId}}",
                flags: Gio.ApplicationFlags.DEFAULT_FLAGS,
            });

            const quit_action = new Gio.SimpleAction({ name: "quit" });
            quit_action.connect("activate", (action) => {
                this.quit();
            });
            this.add_action(quit_action);
            this.set_accels_for_action("app.quit", ["<primary>q"]);

            const show_about_action = new Gio.SimpleAction({ name: "about" });
            show_about_action.connect("activate", (action) => {
                let aboutParams = {
                    transient_for: this.active_window,
                    application_name: "{{dashCase projectName}}",
                    application_icon: "{{appId}}",
                    version: "0.1.0",
                };
                const aboutWindow = new Adw.AboutWindow(aboutParams);
                aboutWindow.present();
            });
            this.add_action(show_about_action);
        }

        vfunc_activate() {
            let { active_window } = this;

            if (!active_window) active_window = new {{pascalCase projectName}}Window(this);

            active_window.present();
        }
    }
);

export function main(argv) {
    const application = new {{pascalCase projectName}}Application();
    return application.runAsync(argv);
}
