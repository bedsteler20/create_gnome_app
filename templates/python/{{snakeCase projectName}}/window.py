{{license}}

from gi.repository import Adw
from gi.repository import Gtk

@Gtk.Template(resource_path='{{pathCase appId}}/window.ui')
class {{pascalCase projectName}}Window(Adw.ApplicationWindow):
    __gtype_name__ = '{{pascalCase projectName}}Window'

    label = Gtk.Template.Child()

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
