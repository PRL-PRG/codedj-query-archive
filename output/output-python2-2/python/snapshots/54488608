"""
	volume.py (a volume control applet for the ROX Panel)

	Copyright 2004 Kenneth Hayber <ken@hayber.us>
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

import rox, sys, gtk
from rox import app_options, applet, Menu, InfoWin
from rox.options import Option
from volumecontrol import VolumeControl

try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Volume'
APP_DIR = rox.app_dir
APP_SIZE = [28, 150]

#Options.xml processing
from rox import choices
choices.migrate(APP_NAME, 'hayber.us')
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
VOLUME_CONTROL = Option('volume_control', 'VOLUME')
SHOW_ICON = Option('show_icon', True)
SHOW_BAR = Option('show_bar', False)

rox.app_options.notify()

MIXER_CONTROLS = {
	'VOLUME':ossaudiodev.SOUND_MIXER_VOLUME,
	'PCM':ossaudiodev.SOUND_MIXER_PCM,
	'Other':ossaudiodev.SOUND_MIXER_ALTPCM
	}

class Volume(applet.Applet):
	"""An applet to control a sound card Master or PCM volume"""
	def __init__(self, filename):
		applet.Applet.__init__(self, filename)

		self.vertical = self.get_panel_orientation() in ('Right', 'Left')
		if self.vertical:
			self.set_size_request(8, -1)
			self.box = gtk.VBox()
			bar_orient = gtk.PROGRESS_LEFT_TO_RIGHT
		else:
			self.set_size_request(-1, 8)
			self.box = gtk.HBox()
			bar_orient = gtk.PROGRESS_BOTTOM_TO_TOP

		self.add(self.box)

		self.image = gtk.Image()
		self.pixbuf = gtk.gdk.pixbuf_new_from_file(APP_DIR+'/images/volume.svg')
		self.image.set_from_pixbuf(self.pixbuf)
		self.size = 0
		self.box.pack_start(self.image)

		self.bar = gtk.ProgressBar()
		self.bar.set_orientation(bar_orient)
		self.bar.set_size_request(12,12)
		self.box.pack_end(self.bar)

		tooltips = gtk.Tooltips()
		tooltips.set_tip(self, _('Volume control'), tip_private=None)

		rox.app_options.add_notify(self.get_options)
		self.connect('size-allocate', self.event_callback)
		self.connect('scroll_event', self.button_scroll)

		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.menu = Menu.Menu('main', [
			Menu.Action(_('Mixer'), 'run_mixer', ''),
			Menu.Separator(),
			Menu.Action(_('Options'), 'show_options', '', gtk.STOCK_PREFERENCES),
			Menu.Action(_('Info'), 'get_info', '', gtk.STOCK_DIALOG_INFO),
			Menu.Action(_('Close'), 'quit', '', gtk.STOCK_CLOSE),
			])
		self.menu.attach(self, self)

		self.thing = None
		self.mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		self.get_volume(MIXER_CONTROLS[VOLUME_CONTROL.value])

		self.show_all()
		self.show()
		if not SHOW_ICON.int_value:
			self.image.hide()
		if not SHOW_BAR.int_value:
			self.bar.hide()



	def button_scroll(self, window, event):
		channel = MIXER_CONTROLS[VOLUME_CONTROL.value]
		current_volume = self.get_volume(channel)
		if event.direction == 0:
			self.set_volume((current_volume[0]+2, current_volume[1]+2), channel)
		elif event.direction == 1:
			self.set_volume((current_volume[0]-2, current_volume[1]-2), channel)

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
		scaled_pixbuf = self.pixbuf.scale_simple(size-4, size-4, gtk.gdk.INTERP_BILINEAR)
		self.image.set_from_pixbuf(scaled_pixbuf)
		self.size = size

	def button_press(self, window, event):
		"""Show/Hide the volume control on button 1 and the menu on button 3"""
		if event.button == 1:
			if not self.hide_volume():
				self.show_volume(event)
		elif event.button == 3:
			self.hide_volume()
			self.menu.popup(self, event, self.position_menu)

	def hide_volume(self, event=None):
		"""Destroy the popup volume control"""
		if self.thing:
#			self.mixer = None
			self.thing.destroy()
			self.thing = None
			return True
		return False

	def get_panel_orientation(self):
		"""Return the panel orientation ('Top', 'Bottom', 'Left', 'Right')
		and the margin for displaying a popup menu"""
		pos = self.socket.property_get('_ROX_PANEL_MENU_POS', 'STRING', False)
		if pos: pos = pos[2]
		if pos:
			side, margin = pos.split(',')
			margin = int(margin)
		else:
			side, margin = None, 2
		return side

	def set_position(self):
		"""Set the position of the popup"""
		side = self.get_panel_orientation()

		# widget (x, y, w, h, bits)
		geometry = self.socket.get_geometry()

		if side == 'Bottom':
			vertical = True
			self.thing.set_size_request(APP_SIZE[0], APP_SIZE[1])
			self.thing.move(self.socket.get_origin()[0],
						self.socket.get_origin()[1]-APP_SIZE[1])
		elif side == 'Top':
			vertical = True
			self.thing.set_size_request(APP_SIZE[0], APP_SIZE[1])
			self.thing.move(self.socket.get_origin()[0],
						self.socket.get_origin()[1]+geometry[3])
		elif side == 'Left':
			vertical = False
			self.thing.set_size_request(APP_SIZE[1], APP_SIZE[0])
			self.thing.move(self.socket.get_origin()[0]+geometry[2],
						self.socket.get_origin()[1])
		elif side == 'Right':
			vertical = False
			self.thing.set_size_request(APP_SIZE[1], APP_SIZE[0])
			self.thing.move(self.socket.get_origin()[0]-APP_SIZE[1],
						self.socket.get_origin()[1])
		return vertical

	def show_volume(self, event):
		"""Display the popup volume control"""
#		self.mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)

		self.thing = gtk.Window(type=gtk.WINDOW_POPUP)
		self.thing.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_MENU)
		self.thing.set_decorated(False)

		vertical = self.set_position()
		self.volume = VolumeControl(MIXER_CONTROLS[VOLUME_CONTROL.value],
						0, 0, True, None, vertical)
		self.volume.set_level(self.get_volume(MIXER_CONTROLS[VOLUME_CONTROL.value]))
		self.volume.connect("volume_changed", self.adjust_volume)

		self.thing.add(self.volume)
		self.thing.show_all()
		self.thing.show()

	def adjust_volume(self, vol, channel, vol_left, vol_right):
		"""Set the playback volume"""
		self.set_volume((vol_left, vol_right), channel)

	def set_volume(self, volume, channel):
		"""Send the volume setting(s) to the mixer """
		self.mixer.set(channel, volume)
		self.bar.set_fraction(max(volume[0], volume[1])/100.0)

	def get_volume(self, channel):
		"""Get the volume settings from the mixer"""
		vol = self.mixer.get(channel)
		self.bar.set_fraction(max(vol[0], vol[1])/100.0)
		return (vol[0], vol[1])

	def get_options(self):
		"""Used as the notify callback when options change"""
		if VOLUME_CONTROL.has_changed:
			self.get_volume(MIXER_CONTROLS[VOLUME_CONTROL.value])

		if SHOW_BAR.has_changed:
			if SHOW_BAR.int_value:
				self.bar.show()
			else:
				self.bar.hide()

		if SHOW_ICON.has_changed:
			if SHOW_ICON.int_value:
				self.image.show()
			else:
				self.image.hide()

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def get_info(self):
		"""Display an InfoWin box"""
		InfoWin.infowin(APP_NAME)

	def run_mixer(self, button=None):
		from rox import filer
		filer.spawn_rox((APP_DIR,))

	def quit(self):
		"""Quit"""
		self.destroy()

