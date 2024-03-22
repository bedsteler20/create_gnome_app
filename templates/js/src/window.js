{{license}}
import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';
import Adw from 'gi://Adw';

export const {{pascalCase projectName}}Window = GObject.registerClass({
    GTypeName: '{{pascalCase projectName}}Window',
    Template: 'resource:///{{pathCase appId}}/window.ui',
    InternalChildren: ['label'],
}, class {{pascalCase projectName}}Window extends Adw.ApplicationWindow {
    constructor(application) {
        super({ application });
    }
});

