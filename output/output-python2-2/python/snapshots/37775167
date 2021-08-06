try:
    import pygtk
except ImportError:
    pygtk = None

try:
    import gtk
    import gtk.glade
    import gtk.gdk
    import gtk.keysyms
except ImportError:
    gtk = None

try:
    import gobject
except ImportError:
    gobject = None

try:
    import pango
except ImportError:
    pango = None

try:
    import cairo
except ImportError:
    cairo = None

try:
    import pangocairo
except ImportError:
    pangocairo = None

def check():
    """
    Check wether pygtk library is installed, or not
    
    @raise AssertionError: if gtk support is missing
    """
    assert gobject is not None and gtk is not None, "PyGTK 2.x must be installed"
    assert pango is not None, "PyGTK have no pango support"
    assert cairo is not None and pangocairo is not None, "PyGTK have no cairo support"
    
    # pygtk.require('2.0')
