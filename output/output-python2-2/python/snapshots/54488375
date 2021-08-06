"""
	postal.py - An imap folder checker panel applet for ROX 

	Copyright 2005-2006 Kenneth Hayber <ken@hayber.us>,
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
import sys, os, time, gtk, gobject, rox, getpass, pango
from rox import applet, filer, tasks
from rox.options import Option

# globals
APP_NAME = 'Postal'
APP_DIR = rox.app_dir
APP_SIZE = [28, 150]

# Options.xml processing
from rox import Menu
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')

#Options go here
SERVER = Option('server', 'localhost')
PORT = Option('port', '143')
MAILBOXES = Option('mailboxes', 'Inbox')
POLLTIME = Option('polltime', 10)
USERNAME = Option('username', getpass.getuser())
PASSWORD = Option('password', '')
MAILER = Option('mailer', 'thunderbird')

#Enable notification of options changes
rox.app_options.notify()

def which(filename):
	'''Return the full path of an executable if found on the path'''
	if (filename == None) or (filename == ''):
		return None

	env_path = os.getenv('PATH').split(':')
	for p in env_path:
		if os.access(p+'/'+filename, os.X_OK):
			return p+'/'+filename
	return None


import imaplib

class IMAPCheck(applet.Applet):
	"""An Applet (no, really)"""
	
	total_unseen = 0
	size = 0

	def __init__(self, id):
		"""Initialize applet."""

		applet.Applet.__init__(self, id)

		# load the applet icon
		self.image = gtk.Image()
		self.nomail = gtk.gdk.pixbuf_new_from_file(os.path.join(APP_DIR, 'images', 'nomail.png'))
		self.ismail = gtk.gdk.pixbuf_new_from_file(os.path.join(APP_DIR, 'images', 'mail.png'))
		self.pixbuf = self.nomail 
		self.image.set_from_pixbuf(self.pixbuf)
		self.resize_image(8)
		self.add(self.image)
		
		self.vertical = self.get_panel_orientation() in ('Right', 'Left')
		if self.vertical:
			self.set_size_request(8, -1)
		else:
			self.set_size_request(-1, 8)

		# set the tooltip
		self.tooltips = gtk.Tooltips()

		# menus
		self.build_appmenu()

		# event handling
		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.connect('size-allocate', self.resize)
		self.connect('delete_event', self.quit)
		rox.app_options.add_notify(self.get_options)
		
		self.checkit()
#		self.update = gobject.timeout_add(100, self.check_mail)
		
	def checkit(self):
		print >>sys.stderr, "checkit starts..."
		tasks.Task(self.check_mail())
		print >>sys.stderr, "checkit ends..."
		
	def check_mail(self):
#		print >>sys.stderr, "check_mail starts..."
		try:
			im = imaplib.IMAP4(SERVER.value)
			im.login(USERNAME.value, PASSWORD.value)
		except:
			self.tooltips.set_tip(self, _("Error"), tip_private=None)
			rox.report_exception()
			self.update = gobject.timeout_add(POLLTIME.int_value * 60000, self.checkit)
#			print >>sys.stderr, "check_mail error..."
			return #don't care, we'll try again later
		
		mailboxes = MAILBOXES.value.split(',')
		results = "" #SERVER.value+":\n"
		self.total_unseen = 0
		
		for mailbox in mailboxes:
			mailbox = mailbox.strip()
			result = im.select(mailbox)
			if result[0] == 'OK':
				if result[1][0] == '':
					count = 0
				else:
					count = int(result[1][0])
			else:
				count = -1
			if count == -1:
#				print >>sys.stderr, "check_mail yield, nomail..."
				yield None
				
			result = im.search(None, "UNSEEN")
			if result[0] == 'OK':
				if result[1][0] == '':
					unseen = 0
				else:
					unseen = len(result[1][0].split())
					self.total_unseen += unseen
			else:
				unseen = -1
			if count > 0:
				results += "%s (%d/%d)\n" % (mailbox, unseen, count)
#			print >>sys.stderr, "check_mail yield, normal..."
			yield None
					
		self.tooltips.set_tip(self, str(results[:-1]), tip_private=None)
		if self.total_unseen:
			self.pixbuf = self.ismail
		else:
			self.pixbuf = self.nomail
		self.image.set_from_pixbuf(self.pixbuf)
		self.resize_image(self.size)

		try: im.close()
		except: pass
		
		self.update = gobject.timeout_add(POLLTIME.int_value * 60000, self.checkit)
#		print >>sys.stderr, "check_mail ends..."
		
	def run_it(self):
		"""Open the given file with ROX."""
		try:
			rox.filer.spawn_rox((which(MAILER.value),))
		except:
			rox.report_exception()

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
		scaled_pixbuf = self.pixbuf.scale_simple(size, size, gtk.gdk.INTERP_BILINEAR)
		self.image.set_from_pixbuf(scaled_pixbuf)
		self.size = size

#draw the total new mail count on top of the icon		
#		if self.window:
#			gc = self.window.new_gc() 
#			layout = self.create_pango_layout('')
#			layout.set_markup("<b>%d</b>" % self.total_unseen)
#			self.window.draw_layout(gc, 3, 3, layout, gtk.gdk.color_parse("black"), None)
#			self.window.draw_layout(gc, 2, 2, layout, gtk.gdk.color_parse("red"), None)
		

	def button_press(self, window, event):
		"""Handle mouse clicks by popping up the matching menu."""

		if event.button == 1:
			self.run_it()
		elif event.button == 2:
			self.check_mail()
		elif event.button == 3:
			self.appmenu.popup(self, event, self.position_menu)

	def get_panel_orientation(self):
		""" Return panel orientation and margin for displaying a popup menu.
			Position in ('Top', 'Bottom', 'Left', 'Right').
		"""
		pos = self.socket.property_get('_ROX_PANEL_MENU_POS', 'STRING', False)
		if pos: pos = pos[2]
		if pos:
			side, margin = pos.split(',')
			margin = int(margin)
		else:
			side, margin = None, 2
		return side

	def get_options(self, widget=None, rebuild=False, response=False):
		"""Used as the notify callback when options change."""
		pass

	def show_options(self, button=None):
		"""Open the options edit dialog."""
		rox.edit_options()

	def get_info(self):
		"""Display an InfoWin box."""
		pass
		
	def build_appmenu(self):
		"""Build the right-click app menu."""

		items = []
		items.append(Menu.Action(_('Check mail'), 'checkit', '', gtk.STOCK_REFRESH))
		items.append(Menu.Action(_('Mail Client'), 'run_it', '', gtk.STOCK_EXECUTE))
		items.append(Menu.Separator())
		items.append(Menu.Action(_('Info...'), 'get_info', '', gtk.STOCK_DIALOG_INFO))
		items.append(Menu.Action(_('Options...'), 'show_options', '', gtk.STOCK_PREFERENCES))
		items.append(Menu.Separator())
		items.append(Menu.Action(_('Close'), 'quit', '', gtk.STOCK_CLOSE))
		self.appmenu = Menu.Menu('other', items)
		self.appmenu.attach(self, self)

	def quit(self, *args):
		"""Quit applet and close everything."""
		self.destroy()
