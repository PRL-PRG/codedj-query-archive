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

import rox
from rox import g, app_options, applet, Menu, InfoWin
from volumecontrol import VolumeControl

try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Volume'
APP_DIR = rox.app_dir
APP_SIZE = [28, 150]

from rox.options import Option

rox.setup_app_options(APP_NAME)

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
VOLUME_CONTROL = Option('volume_control', 'VOLUME')

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
		self.thing = None
		image_vol = g.Image()
		image_vol.set_from_file(APP_DIR+'/images/stock_volume.svg')
		self.add(image_vol)
		tooltips = g.Tooltips()
		tooltips.set_tip(self, _('Volume control'), tip_private=None)

		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			Menu.Action(_('Options'), 'show_options', '', g.STOCK_PREFERENCES),
			Menu.Action(_('Info'), 'get_info', '', g.STOCK_DIALOG_INFO),
			Menu.Action(_('Close'), 'quit', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self, self)


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
			self.mixer = None
			self.thing.destroy()
			self.thing = None
			return True
		return False

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
		return (side, margin)

	def set_position(self):
		"""Set the position of the popup"""
		(side, margin) = self.get_panel_orientation()

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
		self.mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)

		self.thing = g.Window(type=g.WINDOW_POPUP)
		self.thing.set_type_hint(g.gdk.WINDOW_TYPE_HINT_MENU)
		self.thing.set_decorated(False)

		vertical = self.set_position()
		self.volume = VolumeControl(MIXER_CONTROLS[VOLUME_CONTROL.value],
						0, 0, True, None, vertical)
		self.volume.set_level(self.get_volume(MIXER_CONTROLS[VOLUME_CONTROL.value]))
		self.volume.connect("volume_changed", self.adjust_volume)

		self.thing.add(self.volume)
		self.thing.show_all()
		self.thing.show()

	def adjust_volume(self, vol, control, vol_left, vol_right):
		"""Set the playback volume"""
		self.set_volume((vol_left, vol_right), control)

	def set_volume(self, volume, control):
		"""Send the volume setting(s) to the mixer """
		self.mixer.set(control, volume)

	def get_volume(self, control):
		"""Get the volume settings from the mixer"""
		vol = self.mixer.get(control)
		return (vol[0], vol[1])

	def get_options(self):
		"""Used as the notify callback when options change"""
		pass

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def get_info(self):
		"""Display an InfoWin box"""
		InfoWin.infowin(APP_NAME)

	def quit(self):
		"""Quit"""
		self.destroy()

