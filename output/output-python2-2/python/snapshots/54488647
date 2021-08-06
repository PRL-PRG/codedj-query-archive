"""
	volumecontrol.py (a volume control widget)

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
from rox import g
import gobject

CHANNEL_LEFT	= 0
CHANNEL_RIGHT	= 1
CHANNEL_MONO	= 2

#bitmask values
_STEREO	= 1
_LOCK	= 2
_REC	= 4
_MUTE	= 8

class VolumeControl(g.Frame):
	"""
	A Class that implements a volume control (stereo or mono) for a sound card
	mixer.  Each instance represents one mixer channel on the sound card.
	"""
	def __init__(self, control, option_mask, option_value, show_value, label=None, vertical=True):
		"""
		Create a volume control widget
		'control' specifies the ossaudio device mixer control (e.g. ossaudiodev.SOUND_VOLUME_MASTER).
		'option_mask' configures the widget while 'option_value' sets the actual
		value of the corresponding mask (e.g. 'option_mask |= _MUTE' shows the mute
		checkbox while 'option_value |= _MUTE' causes it to be checked by default)
		'show_value' controls whether the volume text is displayed or not.
		'label' is the name of the control (e.g. 'PCM).
		'vertical' sets the widget's orientation (used for vertical panels)

		The widget supports two signals 'volume_changed' and 'volume_setting_toggled'.
		'volume_changed' always sends left and right volume settings regardless of
		whether the control is locked or mono.

		'volume_setting_toggled' notifies the parent of changes in the optional checkboxes.
		"""
		g.Frame.__init__(self, label)

		self.rec = self.lock = self.stereo = self.mute = False
		if option_mask & _LOCK:
			self.lock = True
			self.channel_locked = option_value & _LOCK

		if option_mask & _REC:
			self.rec = True
			self.channel_rec = option_value & _REC

		if option_mask & _MUTE:
			self.mute = True
			self.channel_muted = option_value & _MUTE

		if option_mask & _STEREO:
			self.stereo = True

		self.control = control
		self.vol_left = self.vol_right = 0
		self.set_size_request(60, 200)

		vbox = g.VBox()
		self.add(vbox)
		hbox = g.HBox()
		vbox.pack_start(hbox)

		self.volume1 = g.Adjustment(0.0, 0.0, 100.0, 1.0, 10.0, 0.0)
		if self.stereo:
			self.volume1.connect('value_changed', self.value_changed,
						control, CHANNEL_LEFT)
		else:
			self.volume1.connect('value_changed', self.value_changed,
						control, CHANNEL_MONO)

		if vertical:
			volume1_control = g.VScale(self.volume1)
			volume1_control.set_inverted(True)
		else:
			volume1_control = g.HScale(self.volume1)
			volume1_control.set_value_pos(g.POS_RIGHT)
		volume1_control.set_draw_value(show_value)
		volume1_control.set_digits(0)
		hbox.pack_start(volume1_control)

		if self.stereo:
			self.volume2 = g.Adjustment(0.0, 0.0, 100.0, 1.0, 10.0, 0.0)
			self.volume2.connect('value_changed', self.value_changed,
						control, CHANNEL_RIGHT)

			if vertical:
				volume2_control = g.VScale(self.volume2)
				volume2_control.set_inverted(True)
			else:
				volume2_control = g.HScale(self.volume2)
				volume2_control.set_value_pos(g.POS_RIGHT)
			volume2_control.set_draw_value(show_value)
			volume2_control.set_digits(0)
			hbox.pack_start(volume2_control)

		if self.rec:
			rec_check = g.CheckButton(label=_('Rec.'))
			rec_check.set_active(self.channel_rec)
			rec_check.connect('toggled', self.check, control, _REC)
			vbox.pack_end(rec_check, False, False)
			self.rec_check = rec_check

		if self.mute:
			mute_check = g.CheckButton(label=_('Mute'))
			mute_check.set_active(self.channel_muted)
			mute_check.connect('toggled', self.check, control, _MUTE)
			vbox.pack_end(mute_check, False, False)

		if self.stereo and self.lock:
			lock_check = g.CheckButton(label=_('Lock'))
			lock_check.set_active(self.channel_locked)
			lock_check.connect('toggled', self.check, control, _LOCK)
			vbox.pack_end(lock_check, False, False)

		self.show_all()


	def set_level(self, level):
		"""
		Allow the volume settings to be passed in from the parent.
		'level' is a tuple of integers from 0-100 as (left, right).
		"""
		self.volume1.set_value(level[0])
		if self.stereo:
			self.volume2.set_value(level[1])

	def set_recsrc(self, val):
		try:
			self.rec_check.set_active(val)
		except:
			pass

	def get_level(self):
		"""
		Return the current widget's volume settings as a tuple of
		integers from 0-100 as (left, right)
		"""
		return (self.vol_left, self.vol_right)

	def value_changed(self, vol, control, channel):
		"""
		Track changes in the volume controls and pass them back to the parent
		via the 'volume_changed' signal.
		"""
		if channel == CHANNEL_LEFT:
			self.vol_left = int(vol.get_value())
			if self.lock and self.channel_locked:
				self.volume2.set_value(vol.get_value())

		elif channel == CHANNEL_RIGHT:
			self.vol_right = int(vol.get_value())
			if self.lock and self.channel_locked:
				self.volume1.set_value(vol.get_value())

		else:
			self.vol_left = self.vol_right = int(vol.get_value())
		self.emit("volume_changed", control, self.vol_left, self.vol_right)


	def check(self, button, control, id):
		"""
		Process the various checkboxes and signal the parent when they change
		via the 'volume_setting_changed' signal.
		"""
		if id == _LOCK:
			self.channel_locked = not self.channel_locked
			if self.channel_locked:
				avg_vol = (self.vol_left+self.vol_right)/2
				self.volume1.set_value(avg_vol)
				self.volume2.set_value(avg_vol)
		elif id == _MUTE:
			self.channel_muted = not self.channel_muted
		elif id == _REC:
			self.channel_rec = not self.channel_rec
		self.emit('volume_setting_toggled', control, id, button.get_active())


#I need these to be called only once, not each time an instance is created.
gobject.signal_new('volume_changed', VolumeControl,
		gobject.SIGNAL_RUN_LAST, gobject.TYPE_BOOLEAN,
		(gobject.TYPE_INT, gobject.TYPE_INT, gobject.TYPE_INT))

gobject.signal_new('volume_setting_toggled', VolumeControl,
		gobject.SIGNAL_RUN_LAST, gobject.TYPE_BOOLEAN,
		(gobject.TYPE_INT, gobject.TYPE_INT, gobject.TYPE_INT))

