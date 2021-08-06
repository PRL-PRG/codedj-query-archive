import gtk
import gobject
import gtk.glade
from os.path import abspath
import gettext

class CApplication(gobject.GObject):
    windows = ()
    glade = None
    wins = {}
    main_window = ''
    textdomain = None
    localespath = None
    
    def __init__(self):
        gtk.glade.set_custom_handler(self.__get_custom_handler)
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
                    raise Exception("Glade file is not set for window '%s'"%windowClass.name)
                wTree = self.wTrees[glade] = gtk.glade.XML(glade)
            else:
                wTree = self.wTrees[glade]
            self.wins[windowClass.name] = windowClass(self, wTree)
        if self.textdomain is not None:
            try:
                translation = gettext.translation(self.textdomain, self.localespath)
                translation.install()
            except:
                if isinstance(__builtins__, dict):
                    __builtins__['_'] = lambda text: text
                else:
                    __builtins__._ = lambda text: text
            
            if self.localespath is not None:
                gtk.glade.bindtextdomain(self.textdomain, self.localespath)
            gtk.glade.textdomain(self.textdomain)
    
    def __get_custom_handler(self, glade, function_name, widget_name, str1, str2, int1, int2):
        if not hasattr(self, 'cw_'+function_name):
            raise Exception("Unknown custom widget handler function 'cw_%s' (widget '%s')"%(function_name, widget_name))
        handler = getattr(self, 'cw_'+function_name)
        ret = handler(str1, str2, int1, int2)
        if not isinstance(ret, gtk.Widget):
            raise Exception("Return from custom widget handler function 'cw_%s' (widget '%s') must be gtk widget"%(function_name, widget_name))
        return ret
    
    def GetWindow(self, name):
        return self.wins[name]
    
    def Main(self):
        self.GetWindow(self.main_window).Show()
        gtk.main()
    
    def Quit(self):
        gtk.main_quit()
