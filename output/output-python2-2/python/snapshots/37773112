# setup.py
from distutils.core import setup
import py2exe
import glob

opts = {
    "py2exe": {
        "includes": "pango,atk,gobject,cairo,pangocairo",
        "dll_excludes": [
        "iconv.dll","intl.dll","libatk-1.0-0.dll",
        "libgdk_pixbuf-2.0-0.dll","libgdk-win32-2.0-0.dll",
        "libglib-2.0-0.dll","libgmodule-2.0-0.dll",
        "libgobject-2.0-0.dll","libgthread-2.0-0.dll",
        "libgtk-win32-2.0-0.dll","libpango-1.0-0.dll",
        "libpangowin32-1.0-0.dll"],
        "compressed": 1,
        "optimize": 2,
        #"ascii": 1,
        }
    }

setup(
    name = "UML .FRI",
    description = "Free UML based CASE tool",
    version = "1.0-alpha",
    windows = [
        {"script": "main.py",
        "icon_resources": [(1, "doc/Logo/icon.ico")],
        "dest_base": "bin\uml_fri"
        }
    ],
    zipfile = 'lib/libs.dll',
    options=opts,
    data_files=[("gui", glob.glob("gui/*.png")+glob.glob("gui/*.glade")),
                ("etc", glob.glob("etc/*.xml")),
                ("etc/templates", glob.glob("etc/templates/*.frit")),
                ("etc/uml/connections", glob.glob("etc/uml/connections/*.xml")),
                ("etc/uml/diagrams", glob.glob("etc/uml/diagrams/*.xml")),
                ("etc/uml/elements", glob.glob("etc/uml/elements/*.xml")),
                ("etc/uml/icons", glob.glob("etc/uml/icons/*.png")),
                ("etc/uml/versions", glob.glob("etc/uml/versions/*.xml")),
                ("img", glob.glob("img/*.png")),
                ("locale", []),
                (".", ["ABOUT", "README", "LICENSE"])
    ],
)