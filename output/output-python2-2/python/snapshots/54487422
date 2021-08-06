"""
	volume.py (a volume control applet for the ROX Panel)

	Copyright 2004 Kenneth Hayber <khayber@socal.rr.com>
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

import rox, sys, os
from rox import g, app_options, applet, Menu, InfoWin, filer
from rox.options import Option


APP_NAME = 'Menu'
APP_DIR = rox.app_dir
APP_SIZE = [28, 150]

#Options.xml processing
from rox import choices
choices.migrate(APP_NAME, 'hayber.us')
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')

APPS = Option('applications', os.path.expanduser('~')+'/Apps')

rox.app_options.notify()

class RoxMenu(applet.Applet):
	"""A Menu Applet"""
	def __init__(self, filename):
		applet.Applet.__init__(self, filename)

		self.mainmenu_items = []
		self.appmenu_items = []
		self.root = APPS.value
		self.factory = g.IconFactory()

		self.vertical = self.get_panel_orientation() in ('Right', 'Left')
		if self.vertical:
			self.set_size_request(8, -1)
		else:
			self.set_size_request(-1, 8)

		self.image = g.Image()
		self.pixbuf = g.gdk.pixbuf_new_from_file(APP_DIR+'/images/menu.svg')
		self.image.set_from_pixbuf(self.pixbuf)
		self.size = 0
		self.add(self.image)

		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.connect('size-allocate', self.event_callback)

		tooltips = g.Tooltips()
		tooltips.set_tip(self, _('Menu'), tip_private=None)

		self.refresh_menu()

		self.appmenu_items.append(Menu.Action(_('Refresh'), 'refresh_menu', '', g.STOCK_REFRESH))
		self.appmenu_items.append(Menu.Action(_('Info'), 'get_info', '', g.STOCK_DIALOG_INFO))
		self.appmenu_items.append(Menu.Action(_('Options'), 'show_options', '', g.STOCK_PREFERENCES))
		self.appmenu_items.append(Menu.Separator())
		self.appmenu_items.append(Menu.Action(_('Close'), 'quit', '', g.STOCK_CLOSE))

		self.appmenu = Menu.Menu('appmenu', self.appmenu_items)
		self.appmenu.attach(self, self)

	def run_it(self, args=None):
		#print >>sys.stderr, args
		try:
			filer.spawn_rox((args,))
		except:
			rox.info(args)

	def load_icons(self, name):
		# Load icons
		path = self.root+name+'/.DirIcon'
		pixbuf = g.gdk.pixbuf_new_from_file(path)
		if not pixbuf:
			print >>sys.stderr, "Can't load stock icon '%s'" % name
		g.stock_add([(name, name, 0, 0, "")])
		self.factory.add(name, g.IconSet(pixbuf = pixbuf))

	def process_dir(self, directory):
		"""Walk a directory adding all files found"""
		def visit(dirname, names):
			if 'AppRun' in names:
				offset = len(self.root)
				file = dirname[offset:]
				self.load_icons(file)
				self.mainmenu_items.append((dirname, file))
			else:
				self.process_dir(dirname)

		dirs = os.listdir(directory)
		for dir in dirs:
			try:
				tmp = os.path.join(directory, dir)
				visit(tmp, os.listdir(tmp))
			except:
				pass

	def build_menu(self):
		menu = []
		for (dirname, file) in self.mainmenu_items:
			it = Menu.Action(file, 'run_it', '', file, (dirname,))
			menu.append(it)
		return menu

	def event_callback(self, widget, rectangle):
		"""Called when the panel sends a size."""
		if self.vertical:
			size = rectangle[2]
		else:
			size = rectangle[3]
		if size != self.size:
			self.resize_image(size)

	def resize_image(self, size):
		"""Called to resize the image."""
		#I like the look better with the -4, there is no technical reason for it.
		scaled_pixbuf = self.pixbuf.scale_simple(size-4, size-4, g.gdk.INTERP_BILINEAR)
		self.image.set_from_pixbuf(scaled_pixbuf)
		self.size = size

	def button_press(self, window, event):
		"""Menu popup"""
		if event.button == 1:
			self.mainmenu.popup(self, event, self.position_menu)

		if event.button == 3:
			self.appmenu.popup(self, event, self.position_menu)

	def get_panel_orientation(self):
		"""Return the panel orientation ('Top', 'Bottom', 'Left', 'Right')
		and the margin for displaying a popup menu"""
		pos = self.socket.property_get('_ROX_PANEL_MENU_POS', 'STRING', g.FALSE)
		if pos: pos = pos[2]
		if pos:
			side, margin = pos.split(',')
			margin = int(margin)
		else:
			side, margin = None, 2
		return side

	def get_options(self):
		"""Used as the notify callback when options change"""
		pass

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def get_info(self):
		"""Display an InfoWin box"""
		InfoWin.infowin(APP_NAME)

	def refresh_menu(self):
		self.mainmenu_items = []
		self.process_dir(self.root)
		self.factory.add_default()
		self.mainmenu_items.sort()
		self.mainmenu = Menu.Menu('main', self.build_menu())
		self.mainmenu.attach(self, self)

	def quit(self):
		"""Quit"""
		self.destroy()

