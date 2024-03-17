import GObject from "gi://GObject";
import Gtk from "gi://Gtk";
import Adw from "gi://Adw";
import { registerClass } from "gjs-decorators";

@registerClass({
    GTypeName: "{{pascalCase projectName}}Window",
    Template: "resource://{{pathCase appId}}/window.ui",
    InternalChildren: ["label"],
})
export class {{pascalCase projectName}}Window extends Adw.ApplicationWindow {
    constructor(application) {
        super({ application });
    }
}
