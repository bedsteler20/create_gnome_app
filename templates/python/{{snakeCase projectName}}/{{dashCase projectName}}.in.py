#!@PYTHON@
import os
import sys
import signal
import locale
import gettext

VERSION = '@VERSION@'
pkgdatadir = '@pkgdatadir@'
localedir = '@localedir@'

sys.path.insert(1, pkgdatadir)
signal.signal(signal.SIGINT, signal.SIG_DFL)
locale.bindtextdomain('{{dashCase projectName}}', localedir)
locale.textdomain('{{dashCase projectName}}')
gettext.install('{{dashCase projectName}}', localedir)

if __name__ == '__main__':
    import gi

    gi.require_version('Gtk', '4.0')
    gi.require_version('Adw', '1')

    from gi.repository import Gio
    resource = Gio.Resource.load(os.path.join(pkgdatadir, '{{dashCase projectName}}.gresource'))
    resource._register()

    from {{snakeCase projectName}} import main
    sys.exit(main.main(VERSION))
