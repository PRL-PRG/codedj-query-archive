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
	import alsaaudio
except:
	rox.croak(_("You need to install the pyalsaaudio module"))

APP_NAME = 'MixerX'
APP_DIR = rox.app_dir
APP_SIZE = [20, 100]

#Options.xml processing
rox.setup_app_options('VolumeX', 'Mixer.xml', site='hayber.us')
Menu.set_save_name('VolumeX', site='hayber.us')

MIXER_DEVICE = Option('mixer_device', 'default')
SHOW_VALUES = Option('show_values', False)
SHOW_CONTROLS = Option('controls', -1)

MASK_LOCK = Option('lock_mask', -1)
MASK_MUTE = Option('mute_mask', 0)


try:
	ALSA_CHANNELS = []
	for channel in alsaaudio.mixers(MIXER_DEVICE.value):
		id = 0
		while (channel,id) in ALSA_CHANNELS:
			id += 1
		mixer = alsaaudio.Mixer(channel, id, MIXER_DEVICE.value)
		if len(mixer.volumecap()):
			ALSA_CHANNELS.append((channel,id))
except:
	pass	

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

	n = 0
	for channel, id in ALSA_CHANNELS:
		mixer = alsaaudio.Mixer(channel, id, MIXER_DEVICE.value)
		if not len(mixer.volumecap()):
			continue
		checkbox = controls[n] = gtk.CheckButton(label=channel)
		if option.int_value & (1 << n):
			checkbox.set_active(True)
		checkbox.connect('toggled', lambda e: box.check_widget(option))
		vbox.pack_start(checkbox)
		n += 1
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


class Mixer(rox.Window):
	"""A sound mixer class"""
	def __init__(self):
		rox.Window.__init__(self)

		self.thing = gtk.HBox()
		self.add(self.thing)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)

		self.lock_mask = MASK_LOCK.int_value

		n = 0
		for channel, id in ALSA_CHANNELS:
			#if the mixer supports a channel add it
			mixer = alsaaudio.Mixer(channel, id, MIXER_DEVICE.value)
			option_mask = option_value = 0

			if not len(mixer.volumecap()):
				continue
				
			if len(mixer.getvolume()) > 1:
				option_mask |= volumecontrol._STEREO
				option_mask |= volumecontrol._LOCK

			if self.lock_mask & (1 << n):
				option_value |= volumecontrol._LOCK

			try:
				if mixer.getrec():
					option_mask |= volumecontrol._REC
				if mixer.getrec()[0]:
					option_value |= volumecontrol._REC
			except:
				pass

			try:
				if mixer.getmute():
					option_mask |= volumecontrol._MUTE
				if mixer.getmute()[0]:
					option_value |= volumecontrol._MUTE
			except:
				pass
			
			volume = VolumeControl(n, option_mask, option_value,
								SHOW_VALUES.int_value, channel)
			volume.set_level(self.get_volume(n))
			volume.connect("volume_changed", self.adjust_volume)
			volume.connect("volume_setting_toggled", self.setting_toggled)
			self.thing.pack_start(volume)
			n += 1

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

	def setting_toggled(self, vol, channel, button, val):
		"""Handle checkbox toggles"""
		ch, id = ALSA_CHANNELS[channel]
		mixer = alsaaudio.Mixer(ch, id, MIXER_DEVICE.value)
		
		if button == volumecontrol._MUTE:
			mixer.setmute(val)

		if button == volumecontrol._LOCK:
			if val:
				self.lock_mask |= (1<<channel)
			else:
				self.lock_mask &= ~(1<<channel)
			MASK_LOCK._set(self.lock_mask)

		if button == volumecontrol._REC:
			mixer.setrec(val)


	def adjust_volume(self, vol, channel, volume1, volume2):
		"""Track changes to the volume controls"""
		self.set_volume((volume1, volume2), channel)

	def set_volume(self, volume, channel):
		"""Set the playback volume"""
		ch, id = ALSA_CHANNELS[channel]
		mixer = alsaaudio.Mixer(ch, id, MIXER_DEVICE.value)
		
		try:
			mixer.setvolume(volume[0], 0)
			mixer.setvolume(volume[1], 1)
		except:
			pass

	def get_volume(self, channel):
		"""Get the current sound card setting for specified channel"""
		ch, id = ALSA_CHANNELS[channel]
		mixer = alsaaudio.Mixer(ch, id, MIXER_DEVICE.value)
		vol = mixer.getvolume()
		if len(vol) == 1:
			return (vol[0], vol[0])
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

