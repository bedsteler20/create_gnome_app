import "@girs/gjs";
import "@girs/gjs/dom";
import "@girs/gtk-4.0";
import "@girs/adw-1";

import GObject from "gi://GObject";
import Gio from "gi://Gio";
import Gtk from "gi://Gtk?version=4.0";
import Adw from "gi://Adw?version=1";

import { {{pascalCase projectName}}Window } from "./window.js";
import { registerClass } from "gjs-decorators";

pkg.initGettext();
pkg.initFormat();

@registerClass()
class {{pascalCase projectName}}Application extends Adw.Application {
    constructor() {
        super({
            applicationId: "{{appId}}",
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
                transient_for: this.activeWindow,
                application_name: "{{dashCase projectName}}",
                application_icon: "{{appId}}",
                developer_name: "UwU Cameron Dehning",
                version: "0.1.0",
                developers: ["Cameron Dehning"],
                copyright: "© 2024 Cameron Dehning",
            };
            const aboutWindow = new Adw.AboutWindow(aboutParams);
            aboutWindow.present();
        });
        this.add_action(show_about_action);
    }

    vfunc_activate() {
        let { activeWindow } = this;

        if (!activeWindow) activeWindow = new {{pascalCase projectName}}Window(this);

        activeWindow.present();
    }
}

export function main(argv) {
    const application = new {{pascalCase projectName}}Application();
    return application.runAsync(argv);
}
