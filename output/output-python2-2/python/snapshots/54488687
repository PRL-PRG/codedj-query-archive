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

try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Volume'
APP_DIR = rox.app_dir

APP_SIZE = [20, 100]

from rox.options import Option

rox.setup_app_options(APP_NAME)

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
VOLUME_CONTROL = Option('volume_control', 'VOLUME')

rox.app_options.notify()
Menu.set_save_name(APP_NAME)

menu = Menu.Menu('main', [
	(_('/Options'),'show_options','<StockItem>','', g.STOCK_PREFERENCES),
	(_('/Info'),'get_info','<StockItem>','', g.STOCK_DIALOG_INFO),
	(_('/Close'),'quit','<StockItem>','', g.STOCK_CLOSE),
	])

MIXER_CONTROLS = {
	'VOLUME':ossaudiodev.SOUND_MIXER_VOLUME,
	'PCM':ossaudiodev.SOUND_MIXER_PCM,
	'Other':ossaudiodev.SOUND_MIXER_ALTPCM
	}

class Volume(applet.Applet):
	"""description"""
	def __init__(self, filename):
		"""description"""
		applet.Applet.__init__(self, filename)
		self.thing = None
		image_vol = g.Image()
		image_vol.set_from_file(APP_DIR+'/images/stock_volume.svg')
		self.add(image_vol)
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		menu.attach(self, self)
		tooltips = g.Tooltips()
		tooltips.set_tip(self, _("Volume control"), tip_private=None)

	def button_press(self, window, event):
		"""description"""
		if event.button == 1:
			if not self.hide_volume():
				self.show_volume(event)
		elif event.button == 3:
			menu.popup(self, event, self.position_menu)

	def hide_volume(self, event=None):
		"""description"""
		if self.thing:
			self.thing.destroy()
			self.thing = None
			return True
		return False

	def show_volume(self, event):
		"""description"""
		self.thing = g.Window(type=g.WINDOW_POPUP)
		self.thing.set_type_hint(g.gdk.WINDOW_TYPE_HINT_MENU)
		self.thing.set_decorated(False)
		self.thing.set_size_request(APP_SIZE[0], APP_SIZE[1])
		#self.socket is a gdk window for the widget in the panel.
		#get_origin gets its screen location
		self.thing.move(self.socket.get_origin()[0]+APP_SIZE[0]/4,
						self.socket.get_origin()[1]-APP_SIZE[1])
		self.volume = g.Adjustment(0.5, 0.0, 1.0, 0.1, 0.1, 0.0)
		self.volume.connect('value_changed', self.adjust_volume)
		self.volume_control = g.VScale(self.volume)
		self.volume_control.set_draw_value(True)
		self.volume_control.set_inverted(True)
		self.volume_control.set_size_request(APP_SIZE[0], APP_SIZE[1])
		self.volume_control.set_value(self.get_volume())
		self.thing.add(self.volume_control)
		self.thing.show_all()
		self.thing.show()

	def adjust_volume(self, vol):
		"""Set the playback volume"""
		self.set_volume(vol.get_value())

	def set_volume(self, volume):
		"""description"""
		vol = int(volume*100)
		mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		if mixer != None:
			mixer.set(MIXER_CONTROLS[VOLUME_CONTROL.value], (vol, vol))

	def get_volume(self):
		"""description"""
		mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		if mixer != None:
			vol = mixer.get(MIXER_CONTROLS[VOLUME_CONTROL.value])
			return float(max(vol[0], vol[1]))/100

	def get_options(self):
		"""Used as the notify callback when options change"""
		if SHUFFLE.has_changed:
			self.shuffle.set_active(SHUFFLE.int_value)

		if REPEAT.has_changed:
			self.repeat.set_active(REPEAT.int_value)

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def quit(self):
		"""description"""
		self.destroy()

