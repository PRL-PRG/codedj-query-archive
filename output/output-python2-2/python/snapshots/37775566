import gtk
import gobject
import gtk.glade

import sys
from os.path import abspath
import gettext
import getopt

class CApplication(gobject.GObject):
    windows = ()
    glade = None
    wins = {}
    main_window = ''
    version = '?'
    textdomain = None
    localespath = None
    
    def __init__(self):
        gobject.GObject.__init__(self)
        
        gtk.glade.set_custom_handler(self.__get_custom_handler)
        f_globals = sys._getframe().f_back.f_globals
        if '__version__' in f_globals:
            self.version = f_globals['__version__']
        
        arg_fncs = {}
        long_params = ['help', 'version']
        short_params = 'hv'
        help = [('-h, --help', 'Display this help'), ('-v, --version', 'Display program version')]
        others = None
        for fncname in dir(self):
            if not fncname.startswith('__') and not fncname.endswith('__'):
                fnc = getattr(self, fncname)
                if callable(fnc):
                    if hasattr(fnc, 'argv_opts'):
                        short, long, hasparam = fnc.argv_opts
                        x = []
                        if short is not None:
                            arg_fncs[short] = fnc
                            short_params += short[1]
                            if hasparam:
                                short_params += ':'
                            x.append(short)
                        if long is not None:
                            arg_fncs[long] = fnc
                            long_params.append(long[2:])
                            if hasparam:
                                long_params[-1] += '='
                            x.append(long)
                        if long is None and short is None:
                            assert others is None
                            others = fnc
                        else:
                            help.append((', '.join(x), fnc.__doc__))
        def display_help():
            l = 0
            for cmd, hlp in help:
                if len(cmd) > l:
                    l = len(cmd)
            l = int(l/4.0+0.9)*4
            print "usage: %s [options]"%sys.argv[0],
            if others is None:
                print
            else:
                print "[arguments]"
            print
            print "options:"
            for cmdline in help:
                print ("  %-"+str(l)+"s %s")%cmdline
            if others is not None:
                print
                print "arguments:"
                print " "*(l+3)+others.__doc__
            sys.exit()
        try:
            optlist, args = getopt.getopt(sys.argv[1:], short_params, long_params)
        except getopt.error, e:
            if e.opt:
                print 'unknown option "%s"'%e.opt
            else:
                print 'unknown error'
            display_help()
        for opt, val in optlist:
            if opt in arg_fncs:
                gobject.idle_add(arg_fncs[opt], val)
            elif opt in ('-h', '--help'):
                display_help()
            elif opt in ('-v', '--version'):
                print self.GetVersion()
                sys.exit()
            else:
                print 'unknown option "%s"'%opt
                display_help()
        if not others:
            if args:
                print 'Program takes no argument'
                display_help()
        else:
            if args:
                gobject.idle_add(others, *args)
        
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
            
        self.GetRelativeFile = wTree.relative_file
    
    def __get_custom_handler(self, glade, function_name, widget_name, str1, str2, int1, int2):
        if not hasattr(self, 'cw_'+function_name):
            raise Exception("Unknown custom widget handler function 'cw_%s' (widget '%s')"%(function_name, widget_name))
        handler = getattr(self, 'cw_'+function_name)
        ret = handler(str1, str2, int1, int2)
        if not isinstance(ret, gtk.Widget):
            raise Exception("Return from custom widget handler function 'cw_%s' (widget '%s') must be gtk widget"%(function_name, widget_name))
        return ret
        
    def GetVersion(self):
        return self.version
    
    def GetWindow(self, name):
        return self.wins[name]
    
    def Main(self):
        self.GetWindow(self.main_window).Show()
        gtk.main()
    
    def Quit(self):
        gtk.main_quit()
