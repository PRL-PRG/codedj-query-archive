"""
	battery.py - A Battery Status Monitor for ROX 
	(based largely on Baroque by Tilo Riemer) 
"""

############################################################################
##
## $Id: baroque.py,v 1.13 2004/01/08 04:26:11 rds Exp $
##
## Copyright (C) 2002-2003 Rds <rds@rdsarts.com> and 
##              Tilo Riemer <riemer@lincvs.org>
## All rights reserved.
##
## Baroque is a merge of BatMonitor and the old Baroque
##
## Redistribution and use in source and binary forms, with or without
## modification, are permitted provided that the following conditions
## are met:
##
## 1. Redistributions of source code must retain the above copyright
##    notice, this list of conditions and the following disclaimer.
## 2. Redistributions in binary form must reproduce the above copyright
##    notice, this list of conditions and the following disclaimer in the
##    documentation and/or other materials provided with the distribution.
## 3. The name of the author may not be used to endorse or promote products
##    derived from this software without specific prior written permission.
##
## THIS SOFTWARE IS PROVIDED BY THE AUTHOR `AS IS'' AND ANY EXPRESS OR
## IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
## OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
## IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
## INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
## NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
## DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
## THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
## (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
## THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
##
###############################################################################

# standard library modules
import sys, os, time, gtk, gobject, rox
from rox import applet, filer, tasks
from rox.options import Option

# globals
APP_NAME = 'Lithium'
APP_DIR = rox.app_dir
APP_SIZE = [28, 28]

# Options.xml processing
from rox import Menu
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')

#Options go here
WARN = Option('warn', True)
WARN_LEVEL = Option('warn_level', 10)
TIMER = Option('timeout', "1000")
THEME = Option('theme', 'Color')

#Enable notification of options changes
rox.app_options.notify()

BATT_TYPE = -1	# 0 = ACPI, 1 = APM

# notification
try:
	HAVE_NOTIFY = False
	import pynotify
	if pynotify.init(APP_NAME):
		HAVE_NOTIFY = True
except:
	pass



# Initalize the battery object
try:
	import acpi
	BATTERY = acpi.Acpi()
	BATT_TYPE = 0
	OFFLINE = acpi.OFFLINE
except (ImportError, NotImplementedError, EnvironmentError):
	try:
		import pmu #for PowerMac support
		BATTERY = pmu.Pmu()
		BATT_TYPE = 1
		OFFLINE = pmu.OFFLINE
	except (ImportError, NotImplementedError, EnvironmentError):
		try:
			import apm
			BATTERY = apm.Apm()
			BATT_TYPE = 1
			OFFLINE = apm.OFFLINE
		except (ImportError, NotImplementedError, EnvironmentError):
			rox.croak(_("Sorry, but we could not load a Power Management module. Your system is not configured with power management support."))
		
		
class Battery(applet.Applet):
	"""A Battery Status Monitor Applet"""
	
	warned = False
	msg = 0
	vertical = False

	def __init__(self, id):
		"""Initialize applet."""
		applet.Applet.__init__(self, id)

		# load the applet icon
		self.image = gtk.Image()
		self.load_icons()		
		self.pixbuf = self.images[0]
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

		# user adjustable timer to control how often hardware is polled
		self.timer = gobject.timeout_add(int(TIMER.int_value) * 100, self.update_display)
		
		self.update_display()
		self.update_tooltip()
		self.show()	
		
	def load_icons(self):
		"""load the icons for the selected theme"""		
		theme = THEME.value
		self.images = []
		
		for name in [
			'battery0',
			'battery10', 'battery20', 'battery40', 'battery60',
			'battery80', 'battery100',
			'charging10', 'charging20', 'charging40', 'charging60',
			'charging80', 'charging100',
			]:
			self.images.append(gtk.gdk.pixbuf_new_from_file(os.path.join(rox.app_dir, 'themes', theme, name+'.svg')))

	def notify(self, string):
		if HAVE_NOTIFY:
			n = pynotify.Notification(APP_NAME, string, 'battery-caution')
			n.set_urgency(pynotify.URGENCY_CRITICAL)
			n.show()
		else:
			rox.info(string)
		
	def update_display(self):
		"""Updates all the parts of our applet, and cleans it up if needed."""
		BATTERY.update()
		percent = BATTERY.percent()

		if WARN.value == 'True':
			if (BATTERY.charging_state() == OFFLINE and percent <= WARN_LEVEL.int_value):
				if self.warned == False:
					self.notify(_("Warning. Battery is currently at %d%%") % (BATTERY.percent(),))
					self.warned = True
			else:
				self.warned = False
				
		if percent == 0:
			index = 0
		elif percent < 20:
			index = 1
		elif percent < 40:
			index = 2
		elif percent < 60:
			index = 3
		elif percent < 80:
			index = 4
		elif percent < 100:
			index = 5
		else:
			index = 6
			
		if BATTERY.charging_state():
			index += 6
		
		pb = self.images[index]
		self.pixbuf = pb
		self.resize_image(self.size)

		self.update_tooltip()

		return 1 # to keep the timer going!


	def update_tooltip(self):
		self.tooltips.set_tip(self, self.status() + self.percent() + '%')

	def status(self):
		txt = _("Unknown")
		BATTERY.update()
		if BATTERY.charging_state() == 1:
			txt = _("AC Online: ")
		elif BATTERY.charging_state() == 2:
			txt = _("Charging: ")
		else:
			# Discharing from the battery
			if BATT_TYPE == 1:
				temp2 = BATTERY.time()
				temp = int(temp2 / 60)
				temp2 -= (temp * 60)
				if temp < 0:
					txt = _("Calculating... ")
				else:
					txt = "%s (%d:%02d) " % (_("Battery"), temp, temp2)
			else:
				try:
					temp = BATTERY.estimated_lifetime()
					temp2 = int(60 * (temp - int(temp)))
					txt = "%s (%d:%02d) " % (_("Battery"), temp, temp2)
				except ValueError:
					txt = _("Charging")
		return txt

	def percent(self):
		return str(BATTERY.percent())
	
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

	def button_press(self, window, event):
		"""Handle mouse clicks by popping up the matching menu."""
#		if event.button == 1:
#			self.run_it()
#		if event.button == 2:
#			self.checkit()
		if event.button == 3:
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
		if THEME.has_changed:
			self.load_icons()
			self.update_display()
		
		if TIMER.has_changed:
			if self.timer: gobject.source_remove(self.timer)		
			self.timer = gobject.timeout_add(int(TIMER.int_value) * 100, self.update_display)		 

	def show_options(self, button=None):
		"""Open the options edit dialog."""
		rox.edit_options()

	def get_info(self):
		"""Display an InfoWin self."""
		from rox import InfoWin
		InfoWin.infowin(APP_NAME)
		
	def build_appmenu(self):
		"""Build the right-click app menu."""
		items = []
		items.append(Menu.Action(_('Info...'), 'get_info', '', gtk.STOCK_DIALOG_INFO))
		items.append(Menu.Action(_('Options...'), 'show_options', '', gtk.STOCK_PREFERENCES))
		items.append(Menu.Separator())
		items.append(Menu.Action(_('Close'), 'quit', '', gtk.STOCK_CLOSE))
		self.appmenu = Menu.Menu('other', items)
		self.appmenu.attach(self, self)

	def quit(self, *args):
		"""Quit applet and close everything."""
		if self.timer: gobject.source_remove(self.timer)		
		self.destroy()

