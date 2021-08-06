"""
	mixer.py (an OSS sound mixer for the ROX Desktop)

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
from rox import g, app_options, Menu, InfoWin
from rox.options import Option
from rox.Menu import *
import gobject
try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Mixer'
APP_DIR = rox.app_dir
APP_SIZE = [20, 100]


rox.setup_app_options(APP_NAME)

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
VOLUME_CONTROL = Option('volume_control', 'VOLUME')

rox.app_options.notify()
set_save_name(APP_NAME)


CHANNEL_LEFT = 0
CHANNEL_RIGHT = 1
CHANNEL_MONO = 3
CHANNEL_LOCK = 4
CHANNEL_MUTE = 5
CHANNEL_REC = 6

OSS_CONTROLS = [
	(ossaudiodev.SOUND_MIXER_VOLUME, 'Master'),
	(ossaudiodev.SOUND_MIXER_BASS, 'Bass'),
	(ossaudiodev.SOUND_MIXER_TREBLE, 'Treble'),
	(ossaudiodev.SOUND_MIXER_SYNTH, 'Synth'),
	(ossaudiodev.SOUND_MIXER_PCM, 'PCM'),
	(ossaudiodev.SOUND_MIXER_SPEAKER, 'Speaker'),
	(ossaudiodev.SOUND_MIXER_LINE, 'Line'),
	(ossaudiodev.SOUND_MIXER_MIC, 'Mic'),
	(ossaudiodev.SOUND_MIXER_CD, 'CD'),
	(ossaudiodev.SOUND_MIXER_IMIX, 'iMix'),  # Recording monitor
	(ossaudiodev.SOUND_MIXER_ALTPCM, 'PCM2'),
	(ossaudiodev.SOUND_MIXER_RECLEV, 'Rec Level'),  # Recording level
	(ossaudiodev.SOUND_MIXER_IGAIN,	'In Gain'),  # Input gain
	(ossaudiodev.SOUND_MIXER_OGAIN,	'Out Gain'),  # Output gain
	(14, 'Aux'),
	(15, '15'),
	(16, '16'),
	(17, '17'),
	(18, '18'),
	(19, '19'),
	(20, 'ph In'),
	(21, 'ph Out'),
	(22, 'Video'),
	(23, '23'),
	(24, '24'),
	(25, '25'),
]

class VolumeControl(g.Frame):
	"""
	A Class that implements a volume control (stereo or mono) for a sound card
	mixer.  Each instance represents one mixer channel on the sound card.
	"""
	def __init__(self, control, stereo, rec, recsrc, locked, muted, label=None):
		g.Frame.__init__(self, label)

		self.channel_locked = locked
		self.channel_rec = False
		self.channel_mute = muted
		self.stereo = stereo
		self.vol_left = self.vol_right = 0

		self.set_size_request(60, 200)

		vbox = g.VBox()
		self.add(vbox)
		hbox = g.HBox()
		vbox.pack_start(hbox)

		self.volume1 = g.Adjustment(0.0, 0.0, 100.0, 1.0, 10.0, 0.0)
		if stereo:
			self.volume1.connect('value_changed', self.value_changed,
						control, CHANNEL_LEFT)
		else:
			self.volume1.connect('value_changed', self.value_changed,
						control, CHANNEL_MONO)

		volume1_control = g.VScale(self.volume1)
		volume1_control.set_draw_value(False)
		volume1_control.set_inverted(True)
		hbox.pack_start(volume1_control)

		if stereo:
			self.volume2 = g.Adjustment(0.0, 0.0, 100.0, 1.0, 10.0, 0.0)
			self.volume2.connect('value_changed', self.value_changed,
						control, CHANNEL_RIGHT)

			volume2_control = g.VScale(self.volume2)
			volume2_control.set_draw_value(False)
			volume2_control.set_inverted(True)
			hbox.pack_start(volume2_control)

		if rec:
			rec_check = g.CheckButton(label='Rec.')
			rec_check.set_active(recsrc)
			rec_check.connect('toggled', self.check, control, CHANNEL_REC)
			vbox.pack_end(rec_check, False, False)

		mute_check = g.CheckButton(label='Mute')
		mute_check.set_active(muted)
		mute_check.connect('toggled', self.check, control, CHANNEL_MUTE)
		vbox.pack_end(mute_check, False, False)

		if stereo:
			lock_check = g.CheckButton(label='Lock')
			lock_check.set_active(locked)
			lock_check.connect('toggled', self.check, control, CHANNEL_LOCK)
			vbox.pack_end(lock_check, False, False)

		self.show_all()


	def set_level(self, level):
		self.volume1.set_value(level[0])
		if self.stereo:
			self.volume2.set_value(level[1])

	def get_level(self):
		return (self.vol_left, self.vol_right)

	def value_changed(self, vol, control, channel):
		"""Track changes in the volume controls and pass them back to the parent"""
		if channel == CHANNEL_LEFT:
			self.vol_left = int(vol.get_value())
			if self.channel_locked:
				self.volume2.set_value(vol.get_value())

		elif channel == CHANNEL_RIGHT:
			self.vol_right = int(vol.get_value())
			if self.channel_locked:
				self.volume1.set_value(vol.get_value())

		else:
			self.vol_left = self.vol_right = int(vol.get_value())
		self.emit("volume_changed", control, self.vol_left, self.vol_right)


	def check(self, button, control, id):
		"""Handle all the check buttons"""
		if id == CHANNEL_LOCK:
			self.channel_locked = not self.channel_locked
			if self.channel_locked:
				avg_vol = (self.vol_left+self.vol_right)/2
				self.volume1.set_value(avg_vol)
				self.volume2.set_value(avg_vol)

		elif id == CHANNEL_MUTE:
			self.channel_mute = not self.channel_mute
		else:
			self.channel_rec = not self.channel_rec
		self.emit('volume_setting_toggled', control, id, button.get_active())


#why don't these work as part of the class???
#I need these to be called only once, not each time an instance is created.
gobject.signal_new('volume_changed', VolumeControl,
		gobject.SIGNAL_RUN_LAST, gobject.TYPE_BOOLEAN,
		(gobject.TYPE_INT, gobject.TYPE_INT, gobject.TYPE_INT))

gobject.signal_new('volume_setting_toggled', VolumeControl,
		gobject.SIGNAL_RUN_LAST, gobject.TYPE_BOOLEAN,
		(gobject.TYPE_INT, gobject.TYPE_INT, gobject.TYPE_INT))



class Mixer(rox.Window):
	"""A sound mixer class"""
	def __init__(self):
		rox.Window.__init__(self)

		self.thing = g.HBox()
		self.add(self.thing)

		mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		self.mixer = mixer
		for control in OSS_CONTROLS:
			if mixer.controls() & (1 << control[0]):
				stereo = mixer.stereocontrols() & (1 << control[0])
				rec = mixer.reccontrols() & (1 << control[0])
				recsrc = mixer.get_recsrc() & (1 << control[0])

				volume = VolumeControl(control[0], stereo, rec, recsrc,
							True, False, control[1])

				volume.set_level(self.get_volume(control[0]))
				volume.connect("volume_changed", self.adjust_volume)
				volume.connect("volume_setting_toggled", self.setting_toggled)
				self.thing.pack_start(volume)

		self.thing.show_all()
		self.thing.show()

		menu = Menu('main', [
			Action(_('Options'), 'show_options', '', g.STOCK_PREFERENCES),
			Action(_('Info'), 'get_info', '', g.STOCK_DIALOG_INFO),
			Action(_('Close'), 'quit', '', g.STOCK_CLOSE),
			])
		menu.attach(self, self)


	def button_press(self, window, event):
		"""Display a Menu"""
		if event.button == 1:
			if not self.hide_volume():
				self.show_volume(event)
		elif event.button == 3:
			menu.popup(self, event, self.position_menu)

	def setting_toggled(self, vol, control, id, val):
		"""Mute the channel by setting the volume to 0"""
		if id == CHANNEL_MUTE:
			if val: #mute on
				self.set_volume((0, 0), control)
			else:
				self.set_volume(vol.get_level(), control)

		if id == CHANNEL_LOCK:
			pass

		if id == CHANNEL_REC:
			pass

	def adjust_volume(self, vol, control, volume1, volume2):
		"""Track changes to the volume controls"""
		self.set_volume((volume1, volume2), control)

	def set_volume(self, volume, control):
		"""Set the playback volume"""
		if self.mixer != None:
			self.mixer.set(control, (volume[0], volume[1]))

	def get_volume(self, control):
		"""Get the current sound card setting for specified channel"""
		if self.mixer != None:
			vol = self.mixer.get(control)
			return (vol[0], vol[1])

	def get_options(self):
		"""Used as the notify callback when options change"""
		pass

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def quit(self):
		self.destroy()

