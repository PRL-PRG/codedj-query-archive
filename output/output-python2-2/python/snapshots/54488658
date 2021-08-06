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
from rox import g, app_options, Menu, InfoWin, OptionsBox
from rox.options import Option
import gobject, volumecontrol
from volumecontrol import VolumeControl

try:
	import ossaudiodev
except:
	rox.croak(_("You need python 2.3 for ossaudiodev support"))

APP_NAME = 'Mixer'
APP_DIR = rox.app_dir
APP_SIZE = [20, 100]


rox.setup_app_options('Volume', 'Mixer.xml')

MIXER_DEVICE = Option('mixer_device', '/dev/mixer')
SHOW_VALUES = Option('show_values', False)
SHOW_CONTROLS = Option('controls', -1)


def build_mixer_controls(box, node, label, option):
	"""Custom Option widget to allow hide/display of each mixer control"""
	frame = g.ScrolledWindow()
	frame.set_policy(g.POLICY_NEVER, g.POLICY_ALWAYS)
	frame.set_size_request(100, 100)
	vbox = g.VBox()
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
	for control in OSS_CONTROLS:
		if mixer.controls() & (1 << control[0]):
			checkbox = controls[control[0]] = g.CheckButton(label=control[1])
			if option.int_value & (1 << control[0]):
				checkbox.set_active(True)
			checkbox.connect('toggled', lambda e: box.check_widget(option))
			vbox.pack_start(checkbox)
	mixer.close()
	box.may_add_tip(frame, node)
	return [frame]
OptionsBox.widget_registry['mixer_controls'] = build_mixer_controls

rox.app_options.notify()


OSS_CONTROLS = [
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

		self.thing = g.HBox()
		self.add(self.thing)

		mixer = ossaudiodev.openmixer(MIXER_DEVICE.value)
		self.mixer = mixer
		for control in OSS_CONTROLS:
			#if the mixer supports a channel and we haven't masked it out, add it
			if ((mixer.controls() & (1 << control[0])) and
				(SHOW_CONTROLS.int_value & (1 << control[0]))):
				option_mask = option_value = 0
				if mixer.stereocontrols() & (1 << control[0]):
					option_mask |= volumecontrol._STEREO
					option_mask |= volumecontrol._LOCK
					option_value |= volumecontrol._LOCK
				if mixer.reccontrols() & (1 << control[0]):
					option_mask |= volumecontrol._REC
				if mixer.get_recsrc() & (1 << control[0]):
					option_value |= volumecontrol._REC

				option_mask |= volumecontrol._MUTE

				volume = VolumeControl(control[0], option_mask, option_value,
									SHOW_VALUES.int_value, control[1])
				volume.set_level(self.get_volume(control[0]))
				volume.connect("volume_changed", self.adjust_volume)
				volume.connect("volume_setting_toggled", self.setting_toggled)
				self.thing.pack_start(volume)

		self.thing.show_all()
		self.thing.show()

		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		Menu.set_save_name('Volume')
		self.menu = Menu.Menu('main', [
			Menu.Action(_('Options'), 'show_options', '', g.STOCK_PREFERENCES),
			Menu.Action(_('Info'), 'get_info', '', g.STOCK_DIALOG_INFO),
			Menu.Action(_('Close'), 'quit', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self, self)


	def button_press(self, text, event):
		'''Popup menu handler'''
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	def setting_toggled(self, vol, control, id, val):
		"""Handle checkbox toggles"""
		if id == volumecontrol._MUTE:
			if val: #mute on
				self.set_volume((0, 0), control)
			else:
				self.set_volume(vol.get_level(), control)
		if id == volumecontrol._LOCK:
			pass
		if id == volumecontrol._REC:
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
		rox.edit_options(APP_DIR+'/Mixer.xml')

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def quit(self):
		self.destroy()

