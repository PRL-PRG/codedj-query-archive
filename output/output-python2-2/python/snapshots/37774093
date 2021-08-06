import gtk
import gobject
import gtk.glade
from os.path import abspath

from lib.lib import UMLException

class CApplication(gobject.GObject):
    windows = ()
    glade = None
    wins = {}
    main_window = ''
    
    def __init__(self):
        self.wTrees = {}
        if self.glade is not None:
            self.wTrees[abspath(self.glade)] = self.wTrees[None] = gtk.glade.XML(self.glade)
        for windowClass in self.windows:
            if windowClass.glade is None:
                glade = None
            else:
                glade = abspath(windowClass.glade)
            if glade not in self.wTrees:
                if glade is None:
                    raise UMLException("ApplicationError")
                wTree = self.wTrees[glade] = gtk.glade.XML(glade)
            else:
                wTree = self.wTrees[glade]
            self.wins[windowClass.name] = windowClass(self, wTree)
    
    def GetWindow(self, name):
        return self.wins[name]
    
    def Main(self):
        self.GetWindow(self.main_window).Show()
        gtk.main()
    
    def Quit(self):
        gtk.main_quit()
