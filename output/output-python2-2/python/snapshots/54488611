"""
	mixer.py (an OSS sound mixer for the ROX Desktop)

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

import rox, sys
from rox import app_options, Menu, InfoWin, OptionsBox
from rox.options import Option
import gtk, gobject, volumecontrol
from volumecontrol import VolumeControl

try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Mixer'
APP_DIR = rox.app_dir
APP_SIZE = [20, 100]

#Options.xml processing
from rox import choices
choices.migrate('Volume', 'hayber.us')
rox.setup_app_options('Volume', 'Mixer.xml', site='hayber.us')
Menu.set_save_name('Volume', site='hayber.us')

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
SHOW_VALUES = Option('show_values', False)
SHOW_CONTROLS = Option('controls', -1)

MASK_LOCK = Option('lock_mask', -1)
MASK_MUTE = Option('mute_mask', 0)

def build_mixer_controls(box, node, label, option):
	"""Custom Option widget to allow hide/display of each mixer control"""
	frame = gtk.ScrolledWindow()
	frame.set_policy(gtk.POLICY_NEVER, gtk.POLICY_ALWAYS)
	frame.set_size_request(150, 150)
	vbox = gtk.VBox()
	frame.add_with_viewport(vbox)

	controls = {}

	def get_values():
		value = 0
		for x in controls:
			if controls[x].get_active():
				value |= (1 << x)
		return value
	def set_values(): pass
	box.handlers[option] = (get_values, set_values)

	mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
	for channel in OSS_CHANNELS:
		if mixer.controls() & (1 << channel[0]):
			checkbox = controls[channel[0]] = gtk.CheckButton(label=channel[1])
			if option.int_value & (1 << channel[0]):
				checkbox.set_active(True)
			checkbox.connect('toggled', lambda e: box.check_widget(option))
			vbox.pack_start(checkbox)
	mixer.close()
	box.may_add_tip(frame, node)
	return [frame]
OptionsBox.widget_registry['mixer_controls'] = build_mixer_controls

def build_hidden_value(box, node, label, option):
	"""
	A custom Option widget to save/restore a value
	in the Options system without any UI
	"""
	widget = gtk.HBox() #something unobtrusive
	def get_values(): return option.int_value
	def set_values(): pass
	box.handlers[option] = (get_values, set_values)
	return [widget]
OptionsBox.widget_registry['hidden_value'] = build_hidden_value

rox.app_options.notify()


OSS_CHANNELS = [
	(ossaudiodev.SOUND_MIXER_VOLUME, _('Master')),
	(ossaudiodev.SOUND_MIXER_BASS, _('Bass')),
	(ossaudiodev.SOUND_MIXER_TREBLE, _('Treble')),
	(ossaudiodev.SOUND_MIXER_SYNTH, _('Synth')),
	(ossaudiodev.SOUND_MIXER_PCM, _('PCM')),
	(ossaudiodev.SOUND_MIXER_SPEAKER, _('Speaker')),
	(ossaudiodev.SOUND_MIXER_LINE, _('Line')),
	(ossaudiodev.SOUND_MIXER_MIC, _('Mic')),
	(ossaudiodev.SOUND_MIXER_CD, _('CD')),
	(ossaudiodev.SOUND_MIXER_IMIX, _('iMix')),  # Recording monitor
	(ossaudiodev.SOUND_MIXER_ALTPCM, _('PCM2')),
	(ossaudiodev.SOUND_MIXER_RECLEV, _('Rec Level')),  # Recording level
	(ossaudiodev.SOUND_MIXER_IGAIN,	_('In Gain')),  # Input gain
	(ossaudiodev.SOUND_MIXER_OGAIN,	_('Out Gain')),  # Output gain
	(14, _('Aux')),
	(15, _('15')),
	(16, _('16')),
	(17, _('17')),
	(18, _('18')),
	(19, _('19')),
	(20, _('ph In')),
	(21, _('ph Out')),
	(22, _('Video')),
	(23, _('23')),
	(24, _('24')),
	(25, _('25')),
]


class Mixer(rox.Window):
	"""A sound mixer class"""
	def __init__(self):
		rox.Window.__init__(self)

		self.thing = gtk.HBox()
		self.add(self.thing)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)

		mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		self.mixer = mixer

		self.lock_mask = MASK_LOCK.int_value
		self.mute_mask = MASK_MUTE.int_value
		self.rec_mask = mixer.get_recsrc()

		for channel in OSS_CHANNELS:
			#if the mixer supports a channel add it
			if (mixer.controls() & (1 << channel[0])):
				option_mask = option_value = 0

				if mixer.stereocontrols() & (1 << channel[0]):
					option_mask |= volumecontrol._STEREO
					option_mask |= volumecontrol._LOCK

				if self.lock_mask & (1 << channel[0]):
					option_value |= volumecontrol._LOCK

				if mixer.reccontrols() & (1 << channel[0]):
					option_mask |= volumecontrol._REC

				if self.rec_mask & (1 << channel[0]):
					option_value |= volumecontrol._REC

				option_mask |= volumecontrol._MUTE
				if self.mute_mask & (1 << channel[0]):
					option_value |= volumecontrol._MUTE

				volume = VolumeControl(channel[0], option_mask, option_value,
									SHOW_VALUES.int_value, channel[1])
				volume.set_level(self.get_volume(channel[0]))
				volume.connect("volume_changed", self.adjust_volume)
				volume.connect("volume_setting_toggled", self.setting_toggled)
				self.thing.pack_start(volume)

		self.thing.show()
		self.show_hide_controls()

		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.menu = Menu.Menu('main', [
			Menu.Action(_('Options'), 'show_options', '', gtk.STOCK_PREFERENCES),
			Menu.Action(_('Info'), 'get_info', '', gtk.STOCK_DIALOG_INFO),
			Menu.Action(_('Close'), 'quit', '', gtk.STOCK_CLOSE),
			])
		self.menu.attach(self, self)

		self.connect('delete_event', self.quit)


	def button_press(self, text, event):
		'''Popup menu handler'''
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	def setting_toggled(self, vol, channel, id, val):
		"""Handle checkbox toggles"""
		if id == volumecontrol._MUTE:
			if val: #mute on
				self.mute_mask |= (1<<channel)
				self.set_volume((0, 0), channel)
			else:
				self.mute_mask &= ~(1<<channel)
				self.set_volume(vol.get_level(), channel)
			MASK_MUTE._set(self.mute_mask)

		if id == volumecontrol._LOCK:
			if val:
				self.lock_mask |= (1<<channel)
			else:
				self.lock_mask &= ~(1<<channel)
			MASK_LOCK._set(self.lock_mask)

		# use the OSS api to set/clear these bits/checkboxes.
		if id == volumecontrol._REC:
			if val:
				#when one is checked, the others (typically) are cleared
				#but it may be possible to have more than one checked (I think)
				self.rec_mask = self.mixer.set_recsrc( (1<<channel) )
				for x in self.thing.get_children():
					if isinstance(x, VolumeControl):
						x.set_recsrc(self.rec_mask & (1<<x.channel))
			else:
				self.rec_mask = self.mixer.set_recsrc( ~(1<<channel) )


	def adjust_volume(self, vol, channel, volume1, volume2):
		"""Track changes to the volume controls"""
		self.set_volume((volume1, volume2), channel)

	def set_volume(self, volume, channel):
		"""Set the playback volume"""
		if self.mixer != None:
			self.mixer.set(channel, (volume[0], volume[1]))

	def get_volume(self, channel):
		"""Get the current sound card setting for specified channel"""
		if self.mixer != None:
			vol = self.mixer.get(channel)
			return (vol[0], vol[1])

	def get_options(self):
		"""Used as the notify callback when options change"""
		if SHOW_VALUES.has_changed:
			controls = self.thing.get_children()
			for control in controls:
				control.show_values(bool(SHOW_VALUES.int_value))

		if SHOW_CONTROLS.has_changed:
			self.show_hide_controls()

	def show_hide_controls(self):
		controls = self.thing.get_children()
		for control in controls:
			if (SHOW_CONTROLS.int_value & (1 << control.channel)):
				control.show()
			else:
				control.hide()
		(x, y) = self.thing.size_request()
		self.resize(x, y)

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options(APP_DIR+'/Mixer.xml')

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def quit(self, ev=None, e1=None):
		rox.app_options.save()
		self.destroy()

