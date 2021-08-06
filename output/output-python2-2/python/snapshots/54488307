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
import socket
socket.setdefaulttimeout(5) #set a reasonable timeout for imaplib & poplib

import sys, os, time, gtk, gobject, rox, getpass, popen2, ConfigParser
from rox import applet, filer, tasks, basedir
from rox.options import Option
from rox.tasks import *

# globals
APP_NAME = 'Postal'
APP_SITE = 'hayber.us'
APP_DIR = rox.app_dir
APP_SIZE = [28, 28]
APP_CFG = 'Accounts.ini'


HAVE_NOTIFY = False
try:
	import pynotify
	if pynotify.init(APP_NAME):
		HAVE_NOTIFY = True
except:
	pass


# Options.xml processing
from rox import Menu
rox.setup_app_options(APP_NAME, site=APP_SITE)
Menu.set_save_name(APP_NAME, site=APP_SITE)

#Options go here
MAILER = Option('mailer', 'thunderbird')
SOUND = Option('sound', '')

#Enable notification of options changes
rox.app_options.notify()


#Configure mailbox handling
import imap_check, pop_check, mbox_check
CHECKERS = {
	'IMAP':imap_check.IMAPChecker,
	'POP':pop_check.POPChecker,
	'MBOX':mbox_check.MBOXChecker,
}


class Postal(applet.Applet):
	"""A Mail Checking Applet"""

	def __init__(self, id):
		"""Initialize applet."""
		if id == -1:
			return  # to support --accounts option

		applet.Applet.__init__(self, id)

		# load the applet icon
		self.image = gtk.Image()
		self.nomail = gtk.gdk.pixbuf_new_from_file(os.path.join(APP_DIR, 'images', 'nomail.svg'))
		self.errimg = gtk.gdk.pixbuf_new_from_file(os.path.join(APP_DIR, 'images', 'error.svg'))
		self.ismail = gtk.gdk.pixbuf_new_from_file(os.path.join(APP_DIR, 'images', 'mail.svg'))
		self.pixbuf = self.nomail 
		self.size = 0
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

		# build the mailbox list
		try:
			self.load_accounts()
		except:
			rox.info(_("Problem loading accounts.  Launching Account Editor..."))
			self.edit_accounts()

		# start the main task
		Task(self.check_mail())


	def load_accounts(self):
		"""Load all accounts from config file as dictionaries"""
		self.mailboxes = []

		filename = os.path.join(basedir.save_config_path(APP_SITE, APP_NAME), APP_CFG)
		if not os.access(filename, os.R_OK or os.W_OK):
			raise IOError
		cfg = ConfigParser.ConfigParser()
		cfg.read(filename)

		for section in cfg.sections():
			config = {}
			for item in cfg.items(section):
				config[item[0]] = item[1]
			self.mailboxes.append(CHECKERS[config['protocol']](config))


	def save_accounts(self):
		cfg = ConfigParser.ConfigParser()
		for mb in self.mailboxes:
			cfg.add_section(mb.name)
			for key in mb.__dict__:
				if not key in ['server', 'name', 'protocol', 'folders', 'polltime',
						   'username', 'password', 'port', 'ssl', 'apop', 'filename']:
					continue
				value = mb.__dict__[key] 
				if isinstance(value, list):
					cfg.set(mb.name, key, ','.join(value))
				else:
					cfg.set(mb.name, key, value)
		filename = os.path.join(basedir.save_config_path(APP_SITE, APP_NAME), APP_CFG)
		cfg.write(open(filename, 'w'))		


	def edit_accounts(self):
		"""Edit the accounts list and save to the config file."""
		import accounts
		dlg = accounts.AccountList(self.mailboxes)
		dlg.run()
		dlg.destroy()
		self.save_accounts()


	def check_mail(self):
		"""
		This is the main task for the applet.  It's job is to gather results 
		from each mailbox checker and update the UI.  It does this periodically
		based on the polltime. Each time we wake up from the timeout, we fire 
		all checker tasks and then yield on their blockers. As each blocker
		triggers, we wake up again and process the results.  In some cases more
		than one blocker may have triggered, so we update the UI for all 
		blocker.happened.
		"""
		def timeout(mailbox):
			return mailbox.blocker.happened and isinstance(mailbox.blocker, TimeoutBlocker)

		while True:
			blockers = []
			for mailbox in self.mailboxes:
				if (mailbox.blocker is None) or timeout(mailbox):
					mailbox.blocker = Blocker()
					Task(mailbox.check())
				elif mailbox.blocker.happened:
					self.update(mailbox)
					mailbox.blocker = TimeoutBlocker(mailbox.polltime * 60)
				blockers.append(mailbox.blocker)

			# in case there are no accounts, sleep for 10 seconds			
			if not len(blockers):
				blockers.append(TimeoutBlocker(10))

			yield blockers


	def force_check(self):
		"""Trigger all pending TimeoutBlockers."""
		for mailbox in self.mailboxes:
			if mailbox.blocker and not mailbox.blocker.happened:
				if isinstance(mailbox.blocker, TimeoutBlocker):
					mailbox.blocker.trigger()


	def update(self, mailbox):
		"""Update the display"""
		unseen = 0			
		results = ""
		for box in self.mailboxes:
			results += box.results
			unseen += box.unseen

		if not len(results):
			results = _("No Mail")
		self.tooltips.set_tip(self, results.strip(), tip_private=None)

		if unseen:
			self.pixbuf = self.ismail
		else:
			self.pixbuf = self.nomail
		self.resize_image(self.size)

		if mailbox.unseen > mailbox.prev_total:
			if HAVE_NOTIFY:
				n = pynotify.Notification(_("New Mail has arrived."), 
								mailbox.results.strip(), "mail-message-new")
				n.add_action("mailer", _("Read Mail"), self.run_it)
				n.attach_to_widget(self)
				n.set_category("email.arrived")
				n.show()
			if len(SOUND.value):
				Task(self.play_sound())
		
		# don't report the same 'new' mail again
		mailbox.prev_total = mailbox.unseen

	
	def run_it(self, *action):
		"""Run the Mailer command."""

		def which(filename):
			"""Return the full path of an executable if found on the path"""
			if (filename == None) or (filename == ''):
				return None
		
			env_path = os.getenv('PATH').split(':')
			for p in env_path:
				if os.access(p+'/'+filename, os.X_OK):
					return p+'/'+filename
			return None		

		try:
			rox.filer.spawn_rox((which(MAILER.value),))
		except:
			rox.report_exception()


	def resize(self, widget, rectangle):
		"""Called when the panel sends a size."""
		if self.vertical:
			size = rectangle[2] -2
		else:
			size = rectangle[3] -2
		if size != self.size:
			self.resize_image(size)


	def resize_image(self, size):
		"""Resize the application image."""
		scaled_pixbuf = self.pixbuf.scale_simple(size, size, gtk.gdk.INTERP_BILINEAR)
		self.image.set_from_pixbuf(scaled_pixbuf)
		self.size = size

		
	def play_sound(self):
		"""Play a sound"""
		process = popen2.Popen3(SOUND.value)
		# let stuff happen while playing the sound (the command must write to stdout)
		yield InputBlocker(process.fromchild)
		process.wait()

		
	def button_press(self, window, event):
		"""Handle mouse clicks by popping up the matching menu."""
		if event.button == 1:
			self.run_it()
		elif event.button == 2:
			self.force_check()
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


	def build_appmenu(self):
		"""Build the right-click app menu."""
		items = []
		items.append(Menu.Action(_('Check mail'), 'force_check', '', gtk.STOCK_REFRESH))
		items.append(Menu.Action(_('Mail Client'), 'run_it', '', gtk.STOCK_EXECUTE))
		items.append(Menu.Separator())
		items.append(Menu.Action(_('Options...'), 'show_options', '', gtk.STOCK_PREFERENCES))
		items.append(Menu.Action(_('Accounts...'), 'edit_accounts', ''))
		items.append(Menu.Action(_('Reload...'), 'load_accounts', ''))
		items.append(Menu.Separator())
		items.append(Menu.Action(_('Close'), 'quit', '', gtk.STOCK_CLOSE))
		self.appmenu = Menu.Menu('other', items)
		self.appmenu.attach(self, self)


	def quit(self, *args):
		"""Quit applet and close everything."""

		# TimeoutBlockers won't let the app exit while they are waiting...
		for mailbox in self.mailboxes:
			if mailbox.blocker and not mailbox.blocker.happened:
				if isinstance(mailbox.blocker, TimeoutBlocker):
					rox.toplevel_unref()

		self.destroy()
