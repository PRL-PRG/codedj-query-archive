"""
    rmenu.py (a application menu applet for the ROX Panel)

    Copyright 2004-2005 Kenneth Hayber <ken@hayber.us>,
                        Christopher Arndt <chris@chrisarndt.de>
            All rights reserved.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License.

    This program is distributed in the hope that it will be useful
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
"""

# standard library modules
import sys, os
from os.path import dirname, exists, join, isdir, isfile

# third-party modules
import rox
from rox import g, app_options, applet, choices, filer, Menu
from rox.options import Option

# application specific modules
from infowin import infowin

# globals
APP_NAME = 'Menu'
APP_DIR = rox.app_dir
APP_SIZE = [28, 150]

# Options.xml processing
choices.migrate(APP_NAME, 'hayber.us')
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')
APPS = Option('applications', join(os.path.expanduser('~'), 'Apps'))

rox.app_options.notify()

# helper functions
def isapp(name):
    """Check if file is an app dir."""

    apprun = join(name, 'AppRun')
    return isfile(apprun) and os.access(apprun, os.X_OK)

# classes
class RoxMenu(applet.Applet):
    """A Menu Applet"""

    def __init__(self, filename, root=None):
        """Initialize applet.

        filename is passed to Applet.__init__()
        root is the top of the directory hierarchy that is
        scanned for applications to build the menu.
        """

        applet.Applet.__init__(self, filename)

        # load the applet icon
        self.image = g.Image()
        self.pixbuf = g.gdk.pixbuf_new_from_file(join(APP_DIR, 'images',
          'menu.svg'))
        self.image.set_from_pixbuf(self.pixbuf)
        self.resize_image(8)
        self.add(self.image)

        self.vertical = self.get_panel_orientation() in ('Right', 'Left')
        if self.vertical:
            self.set_size_request(8, -1)
        else:
            self.set_size_request(-1, 8)

        # use IconFactory for the icons of the menu items
        self.factory = g.IconFactory()
        self.factory.add_default()

        # set the tooltip
        tooltips = g.Tooltips()
        tooltips.set_tip(self, _("Application starter - "
          "click left to open menu, click right for more options."),
          tip_private=None)

        # menus
        if root:
            self.root = root
        else:
            self.root = APPS.value
        self.refresh_menu()
        self.mainmenu.attach(self, self)
        self.build_appmenu()

        # event handling
        self.add_events(g.gdk.BUTTON_PRESS_MASK)
        self.connect('button-press-event', self.button_press)
        self.connect('size-allocate', self.resize)
        rox.app_options.add_notify(self.get_options)

    def run_it(self, args=None):
        """Open the fiven file with ROX."""

        #print >>sys.stderr, args
        try:
            filer.spawn_rox((args,))
        except:
            rox.info(args)

    def load_icons(self, name, path):
        """Try to load an icon for an application."""

        for icon in ('.DirIcon', 'AppIcon.xpm'):
            iconpath = join(path, icon)
            if exists(iconpath):
                try:
                    pixbuf = g.gdk.pixbuf_new_from_file(iconpath)
                    break
                except:
                    print >>sys.stderr, "Can't load icon '%s'" % iconpath
                    return
        else:
            return
        g.stock_add([(name, name, 0, 0, "")])
        self.factory.add(name, g.IconSet(pixbuf=pixbuf))

    def process_dir(self, directory):
        """Walk a directory recursively and build main menu.

        Directories will be submenus. Empty diectories are ignored.
        Normal files and app dirs are added as normal menu items.
        """

        menu = []
        names = [x for x in os.listdir(directory) if x[0] != '.']
        dirs = [x for x in names if isdir(join(directory, x)) and not
          isapp(join(directory, x))]
        files = [x for x in names if x not in dirs]
        if sys.version_info[:2] <= (2,4):
            files = [(x.lower(), x) for x in files]
            files.sort()
            files = [x[1] for x in files]
            dirs = [(x.lower(), x) for x in dirs]
            dirs.sort()
            dirs = [x[1] for x in dirs]
        else:
            dirs.sort(key=lambda x: x.lower())
            files.sort(key=lambda x: x.lower())
        for dir in dirs:
            path = join(directory, dir)
            submenu = self.process_dir(path)
            if submenu:
                menu.append(Menu.SubMenu(dir, submenu))
        for file in files:
            path = join(directory, file)
            self.load_icons(file, path)
            menu.append(Menu.Action(file, 'run_it', '', file, (path,)))
        return menu


    def resize(self, widget, rectangle):
        """Called when the panel sends a size."""

        if self.vertical:
            size = rectangle[2]
        else:
            size = rectangle[3]
        if size != self.size:
            self.resize_image(size)

    def resize_image(self, size):
        """Resize the application image."""

        # I like the look better with the -4, there is no technical
        # reason for it.
        scaled_pixbuf = self.pixbuf.scale_simple(size-4, size-4,
          g.gdk.INTERP_BILINEAR)
        self.image.set_from_pixbuf(scaled_pixbuf)
        self.size = size

    def button_press(self, window, event):
        """Handle mouse clicks by popping up the matching menu."""

        if event.button == 1:
            if self.mainmenu_items:
                self.mainmenu.popup(self, event, self.position_menu)
            else:
                rox.alert(_("Could not read your application directory.\n\nPlease set the path to your Apps folder in the options!"))
        elif event.button == 2:
            self.open_app_folder()
        elif event.button == 3:
            self.appmenu.popup(self, event, self.position_menu)

    def get_panel_orientation(self):
        """Return panel orientation and margin for displaying a popup menu.

        Position in ('Top', 'Bottom', 'Left', 'Right').
        """

        pos = self.socket.property_get('_ROX_PANEL_MENU_POS', 'STRING', g.FALSE)
        if pos: pos = pos[2]
        if pos:
            side, margin = pos.split(',')
            margin = int(margin)
        else:
            side, margin = None, 2
        return side

    def get_options(self):
        """Used as the notify callback when options change."""

        if APPS.has_changed:
            self.root = APPS.value
            self.refresh_menu()

    def show_options(self, button=None):
        """Open the options edit dialog."""

        rox.edit_options()

    def get_info(self):
        """Display an InfoWin box."""

        iw = getattr(self, 'iw', None)
        if not iw:
            self.iw = iw = infowin(APP_NAME)
        iw.show()

    def refresh_menu(self):
        """Rebuild the menu from disk."""

        if self.root and isdir(self.root):
            self.mainmenu_items = self.process_dir(self.root)
            # XXX is the following really necessary?
            self.factory.add_default()
        else:
            self.mainmenu_items = []
        self.mainmenu = Menu.Menu('main', self.mainmenu_items)

    def build_appmenu(self):
        """Build the right-click app menu."""

        self.appmenu_items = []
        self.appmenu_items.append(Menu.Action(
          _('Open apps folder'), 'open_app_folder', '', g.STOCK_JUMP_TO))
        self.appmenu_items.append(Menu.Action(
          _('Refresh menu'), 'refresh_menu', '', g.STOCK_REFRESH))
        self.appmenu_items.append(Menu.Separator())
        self.appmenu_items.append(Menu.Action(
          _('Info...'), 'get_info', '', g.STOCK_DIALOG_INFO))
        self.appmenu_items.append(Menu.Action(
          _('Options...'), 'show_options', '', g.STOCK_PREFERENCES))
        self.appmenu_items.append(Menu.Separator())
        self.appmenu_items.append(Menu.Action(
          _('Close'), 'quit', '', g.STOCK_CLOSE))
        self.appmenu = Menu.Menu('other', self.appmenu_items)
        self.appmenu.attach(self, self)

    def open_app_folder(self):
        """Open the folder comtaining the appliocations with ROX-Filer."""

        if self.root and isdir(self.root):
            filer.open_dir(self.root)
        else:
            rox.alert(_("Could not open your application directory.\n\nPlease set the path to your Apps folder in the options!"))

    def quit(self):
        """Quit applet and close everything."""

        try:
            self.iw.destroy()
        except AttributeError:
            pass
        except:
            rox.report_exception()
        self.destroy()
