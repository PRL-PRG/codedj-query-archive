"""
	musicbox.py

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

from __future__ import generators

import os, sys, re, string, threading, pango, gtk, gobject
from threading import *

import rox
from rox import Menu, app_options, loading, saving, InfoWin, OptionsBox, filer
from rox.options import Option


try:
	import player, playlist, playlistui, xsoap, mbtypes
except:
	rox.report_exception()


#Who am I and how did I get here?
APP_NAME = "MusicBox"
APP_DIR = rox.app_dir
APP_DOMAIN = 'hayber.us'

ALBUM_COVER_SIZE = 90

#Toolbar button indexes
BTN_CLOSE = 0
BTN_PREV = 1
BTN_PLAY = 2
BTN_STOP = 3
BTN_NEXT = 4
BTN_REPEAT = 5
BTN_SHUFFLE = 6
BTN_PLAYLIST = 7
BTN_OPTIONS = 8


#Bitmaps (potentially) used in this application
factory = gtk.IconFactory()
for name in [
			'media-next',	'media-pause',	'media-play',	'media-prev',
			'media-repeat',	'media-shuffle','media-stop',	'media-track',
#			'media-eject',	'media-ffwd',	'media-rewind',
#			'volume-max',	'volume-medium','volume-min',	'volume-mute',
#			'volume-zero',	'media-record',
		]:
	path = os.path.join(rox.app_dir, "images", name + ".png")
	pixbuf = gtk.gdk.pixbuf_new_from_file(path)
	if not pixbuf:
		print >>sys.stderr, "Can't load stock icon '%s'" % name
	gtk.stock_add([(name, name, 0, 0, "")])
	factory.add(name, gtk.IconSet(pixbuf = pixbuf))
factory.add_default()


#Options.xml processing
from rox import choices
choices.migrate(APP_NAME, APP_DOMAIN)
rox.setup_app_options(APP_NAME, site=APP_DOMAIN)
Menu.set_save_name(APP_NAME, site=APP_DOMAIN)

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', os.path.expanduser('~')+'/Music')

#how to parse each library leaf to get artist, album, title...
LIBRARY_RE = Option('library_re', '^.*/(?P<artist>.*)/(?P<album>.*)/(?P<title>.*)')

#the ao driver type you want to use (esd, oss, alsa, alsa09, ...)
DRIVER_ID = Option('driver_id', 'esd')
MIXER_DEVICE = Option('mixer_device', '/dev/mixer')

SHUFFLE = Option('shuffle', 0)
REPEAT = Option('repeat', 0)

#Don't replay any of the last n songs in shuffle mode
SHUFFLE_CACHE_SIZE = Option('shuffle_cache', 10)

#Buffer size used by audio device read/write
AUDIO_BUFFER_SIZE = Option('audio_buffer', 4096)

#Eye candy
SONG_FONT = Option('song_font', None)
BASE_FONT = Option('base_font', None)
BG_COLOR = Option('bg_color', '#A6A699')
FG_COLOR = Option('fg_color', '#000000')

#Show/Hide settings
SH_TOOLBAR = Option('toolbar', True)
SH_VOLUME = Option('volume', True)
SH_SEEKBAR = Option('seekbar', True)

#Other GUI details
WORDWRAP = Option('word_wrap', False)
MINITOOLS = Option('mini_toolbar', False)
TIMEDISPLAY = Option('time_display', 0)
ALBUM_ART = Option('album_art', 0)

tooltips = gtk.Tooltips()


def build_tool_options(box, node, label, option):
	"""Custom Option widget to allow show/hide each toolbar button"""

	toolbar = gtk.Toolbar()
	toolbar.set_style(gtk.TOOLBAR_ICONS)

	done = False
	def item_changed(thing):
		if done:  #supress widget updates until later
			box.check_widget(option)

	type1 = gtk.TOOLBAR_CHILD_TOGGLEBUTTON
	buttons = [
		#(type, text, icon, callback)
		(type1, _("Close"), gtk.STOCK_CLOSE, item_changed),
		(type1, _("Prev"), 'media-prev', item_changed),
		(type1, _("Play"), 'media-play', item_changed),
		(type1, _("Stop"), 'media-stop', item_changed),
		(type1, _("Next"), 'media-next', item_changed),
		(type1, _("Repeat"), 'media-repeat', item_changed),
		(type1, _("Shuffle"), 'media-shuffle', item_changed),
		(type1, _("Playlist"), gtk.STOCK_INDEX, item_changed),
		(type1, _("Options"), gtk.STOCK_PREFERENCES, item_changed),
	]

	controls = []
	for (type, text, icon, callback) in buttons:
		image = gtk.image_new_from_stock(icon, gtk.ICON_SIZE_SMALL_TOOLBAR)
		controls.append(toolbar.insert_element(type, None, text, text,
											None, image, callback, None, -1))

	#build a bitmask of which buttons are active
	def get_values():
		value = 0
		for i in range(len(controls)):
			if controls[i].get_active():
				value |= (1 << i)
		return value
	def set_values(): pass
	box.handlers[option] = (get_values, set_values)

	#initialize the button states
	for i in range(len(controls)):
		if option.int_value & (1 << i):
				controls[i].set_active(True)
	done = True #OK, updates activated.

	box.may_add_tip(toolbar, node)
	return [toolbar]

OptionsBox.widget_registry['tool_options'] = build_tool_options
TOOLOPTIONS = Option('toolbar_disable', -1)

rox.app_options.notify()


class MusicBox(rox.Window, loading.XDSLoader):
	"""A Music Player for mp3 and ogg - main class"""
	time_string = ''
	_shuffle = False
	_repeat = False


	def __init__(self):
		"""Constructor for MusicBox"""
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, mbtypes.TYPE_LIST)

		# Main window settings
		self.set_title(APP_NAME)
		self.set_role("MainWindow")

		# Notifications
		rox.app_options.add_notify(self.get_options)
		self.connect('delete_event', self.delete_event)
		self.connect('window-state-event', self.window_state_event)
		self.connect('drag-motion', self.xds_drag_motion)

		# Set some defaults
		self.replace_library = False
		self.library = LIBRARY.value.split(':')
		self.playlist = None
		self.playlistUI = None
		self.current_song = None

		self.shuffle = bool(SHUFFLE.int_value)
		self.repeat = bool(REPEAT.int_value)

		# Build and Init everything
#GTK2.4		self.uimanager = gtk.UIManager()
#GTK2.4		self.uimanager.insert_action_group(self.build_actions(), 0)
#GTK2.4		self.uimanager.add_ui_from_file('ui.xml')
#GTK2.4		self.menu = self.uimanager.get_widget('/ui/popup')
#GTK2.4		self.toolbar = self.uimanager.get_widget('/ui/toolbar')
#GTK2.4		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
#GTK2.4		self.connect('button-press-event', self.button_press)
#GTK2.4		self.connect('popup-menu', self.menukey_press)

		self.build_menu()
		self.build_toolbar()
		self.build_labels()
		self.set_line_wrap()
		self.build_misc()
		self.set_fonts()
		self.set_colors()

		# Pack and show widgets
		self.vbox = gtk.VBox()
		self.hbox = gtk.HBox()
		self.add(self.vbox)
		self.vbox.add(self.hbox)

		self.hbox.pack_start(self.display, True, True, 0)
		self.hbox.pack_end(self.volume_control, False, True, 0)
		self.vbox.pack_end(self.toolbar, False, True, 0)
		self.vbox.pack_end(self.seek_bar_control, False, True, 0)

		self.vbox.show_all()
		self.show_hide_controls()
		self.show_hide_buttons()
		if not ALBUM_ART.int_value:
			self.album_img.hide()
		self.show()

		#Start xmlrpc server
		self.server()

		self.playlist = playlist.Playlist(SHUFFLE_CACHE_SIZE.int_value, LIBRARY_RE.value)
		self.player = player.Player(DRIVER_ID.value, AUDIO_BUFFER_SIZE.int_value)
		self.foo = Thread(name='player', target=self.player.run)
		self.foo.setDaemon(True)
		self.foo.start()
		self.volume.set_value(self.player.get_volume(MIXER_DEVICE.value))

		if len(sys.argv) > 1:
			self.load_args(sys.argv[1:], True)
		else:
			self.load_args([], False)

		gobject.timeout_add(500, self.display_update)

#GTK2.4	def build_actions(self):
#GTK2.4		actions = gtk.ActionGroup('main')
#GTK2.4		actions.add_action(gtk.Action('quit', _("Quit"), _("Quit the application"), gtk.STOCK_QUIT))
#GTK2.4		actions.add_action(gtk.Action('close', _("Close"), _("Close this window"), gtk.STOCK_CLOSE))
#GTK2.4
#GTK2.4		actions.add_action(gtk.Action('options', _("Options"), _("Edit Options"), gtk.STOCK_PREFERENCES))
#GTK2.4		actions.add_action(gtk.Action('info', _("Info"), _("Show program info"), gtk.STOCK_DIALOG_INFO))
#GTK2.4
#GTK2.4		actions.add_action(gtk.Action('play', _("Play"), _("Play"), 'media-play'))
#GTK2.4		actions.add_action(gtk.Action('stop', _("Stop"), _("Stop"), 'media-stop'))
#GTK2.4		actions.add_action(gtk.Action('prev', _("Prev"), _("Prev"), 'media-prev'))
#GTK2.4		actions.add_action(gtk.Action('next', _("Next"), _("Next"), 'media-next'))
#GTK2.4
#GTK2.4		actions.add_action(gtk.Action('playlist', _("Playlist"), _("Show Playlist"), gtk.STOCK_INDEX))
#GTK2.4		actions.add_action(gtk.Action('open', _("Open"), _("Open Location"), gtk.STOCK_GO_UP))
#GTK2.4		actions.add_action(gtk.Action('save', _("Save"), _("Save Playlist"), gtk.STOCK_SAVE))
#GTK2.4
#GTK2.4		actions.add_action(gtk.Action('shuffle', _("Shuffle"), _("Shuffle"), 'media-shuffle'))
#GTK2.4		actions.add_action(gtk.Action('repeat', _("Repeat"), _("Repeat"), 'media-repeat'))
#GTK2.4
#GTK2.4		actions.add_action(gtk.Action('none', None, None, 0))
#GTK2.4		return actions


	def build_menu(self):
		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.connect('popup-menu', self.menukey_press)
		self.menu = Menu.Menu('main', [
			Menu.Action(_("Play")+'\/'+_("Pause"), 'play_pause', '', 'media-play'),
			Menu.Action(_("Stop"), 'stop', '', 'media-stop'),
			Menu.Separator(),

			Menu.Action(_("Back"), 'prev', '', 'media-prev'),
			Menu.Action(_("Next"), 'next', '', 'media-next'),
			Menu.Separator(),

			Menu.SubMenu(_('Playlist'), [
				Menu.Action(_("Show"), 'show_playlist', '', gtk.STOCK_INDEX),
				Menu.Action(_("Open location"), 'show_dir', '', gtk.STOCK_GO_UP),
				Menu.Action(_("Save"), 'save', '', gtk.STOCK_SAVE),
				Menu.ToggleItem(_("Shuffle"), 'shuffle'),
				Menu.ToggleItem(_("Repeat"), 'repeat'),
			]),

			Menu.Separator(),
			Menu.Action(_("Options"), 'show_options', '', gtk.STOCK_PREFERENCES),
			Menu.Action(_('Info'),	'get_info', '', gtk.STOCK_DIALOG_INFO),
			Menu.Separator(),

			Menu.Action(_("Quit"), 'close', '', gtk.STOCK_CLOSE),
		])
		self.menu.attach(self,self)


	def build_toolbar(self):
		self.toolbar = gtk.Toolbar()
		self.toolbar.set_style(gtk.TOOLBAR_ICONS)
		if bool(MINITOOLS.int_value):
			self.toolbar.set_icon_size(gtk.ICON_SIZE_MENU)

		type1 = gtk.TOOLBAR_CHILD_BUTTON
		type2 = gtk.TOOLBAR_CHILD_TOGGLEBUTTON

		items = [
			#(type, text, icon, callback)
			(type1, _("Close"), gtk.STOCK_CLOSE, self.close),
			(type1, _("Prev"), 'media-prev', self.prev),
			(type1, _("Play"), 'media-play', self.play_pause),
			(type1, _("Stop"), 'media-stop', self.stop),
			(type1, _("Next"), 'media-next', self.next),
			(type2, _("Repeat"), 'media-repeat', lambda b: self.set_repeat(b.get_active())),
			(type2, _("Shuffle"), 'media-shuffle', lambda b: self.set_shuffle(b.get_active())),
			(type1, _("Playlist"), gtk.STOCK_INDEX, self.show_playlist),
			(type1, _("Options"), gtk.STOCK_PREFERENCES, self.show_options),
		]

		buttons = []
		for (type, text, icon, callback) in items:
			image = gtk.image_new_from_stock(icon, gtk.ICON_SIZE_SMALL_TOOLBAR)
			buttons.append(self.toolbar.insert_element(type, None, text, text,
											None, image, callback, None, -1))
			if text == _("Play"): #this one changes later, so we have to save it
				self.image_play = image

		self.buttons = buttons

		buttons[BTN_REPEAT].set_active(self.repeat)
		buttons[BTN_SHUFFLE].set_active(self.shuffle)
		buttons[BTN_PLAYLIST].set_sensitive(False)


	def build_labels(self):
		self.display = gtk.Layout()
		self.display_size = (0, 0)
		self.display.connect('size-allocate', self.resize)

		self.album_img = gtk.Image()
		self.album_img.set_alignment(0.0, 0.0)

		hbox = gtk.HBox(False, 5)
		vbox = gtk.VBox()
		hbox.pack_start(self.album_img, False, False, 0)
		hbox.pack_end(vbox, True, True, 0)
		self.display.put(hbox, 0, 0)
		self.display_box = vbox

		self.display_song = gtk.Label()
		self.display_song.set_alignment(0.0, 0.0)

		self.display_album = gtk.Label()
		self.display_album.set_alignment(0.0, 0.0)

		self.display_artist = gtk.Label()
		self.display_artist.set_alignment(0.0, 0.0)

		self.display_status = gtk.Label()
		self.display_status.set_alignment(0.0, 0.0)

		vbox.pack_start(self.display_song, False, True, 0)
		vbox.pack_start(self.display_album, False, True, 0)
		vbox.pack_start(self.display_artist, False, True, 0)
		vbox.pack_start(self.display_status, False, True, 0)


	def set_line_wrap(self):
		self.display_song.set_line_wrap(bool(WORDWRAP.int_value))
		self.display_album.set_line_wrap(bool(WORDWRAP.int_value))
		self.display_artist.set_line_wrap(bool(WORDWRAP.int_value))
		self.display_status.set_line_wrap(bool(WORDWRAP.int_value))


	def build_misc(self):
		self.volume = gtk.Adjustment(50.0, 0.0, 100.0, 1.0, 10.0, 0.0)
		self.volume.connect('value_changed', self.adjust_volume)
		self.volume_control = gtk.VScale(self.volume)
		self.volume_control.set_draw_value(False)
		self.volume_control.set_inverted(True)
		self.volume_control.set_size_request(-1, 90)

		self.seek_bar = gtk.Adjustment(0.0, 0.0, 1.0, 0.01, 0.1, 0.0)
		self.seek_id = self.seek_bar.connect('value_changed', self.adjust_seek_bar)
		self.seek_bar_control = gtk.HScale(self.seek_bar)
		self.seek_bar_control.set_update_policy(gtk.UPDATE_DELAYED)
		self.seek_bar_control.set_draw_value(False)
		self.seek_bar_control.set_size_request(100, -1)


	def set_fonts(self):
		song_font = pango.FontDescription(SONG_FONT.value)
		base_font = pango.FontDescription(BASE_FONT.value)
		self.display_song.modify_font(song_font)
		self.display_album.modify_font(base_font)
		self.display_artist.modify_font(base_font)
		self.display_status.modify_font(base_font)


	def set_colors(self):
		fg_color = gtk.gdk.color_parse(FG_COLOR.value)
		bg_color = gtk.gdk.color_parse(BG_COLOR.value)
		self.display.modify_bg(gtk.STATE_NORMAL, bg_color)
		self.display_song.modify_fg(gtk.STATE_NORMAL, fg_color)
		self.display_album.modify_fg(gtk.STATE_NORMAL, fg_color)
		self.display_artist.modify_fg(gtk.STATE_NORMAL, fg_color)
		self.display_status.modify_fg(gtk.STATE_NORMAL, fg_color)


	def show_hide_controls(self):
		for (option, control) in [
				(SH_VOLUME, self.volume_control), (SH_TOOLBAR, self.toolbar),
				(SH_SEEKBAR, self.seek_bar_control)]:
			if bool(option.int_value):
				control.show()
			else:
				control.hide()


	def show_hide_buttons(self):
		for i in range(len(self.buttons)):
			if TOOLOPTIONS.int_value & (1 << i):
				self.buttons[i].show()
			else:
				self.buttons[i].hide()


	def set_shuffle(self, value):
		try:
			self._shuffle = value
			self.buttons[BTN_SHUFFLE].set_active(self._shuffle)
		except:
			pass #this gets called before everything is built
	shuffle = property(lambda self: self._shuffle, set_shuffle)


	def set_repeat(self, value):
		try:
			self._repeat = value
			self.buttons[BTN_REPEAT].set_active(self._repeat)
		except:
			pass #this gets called before everything is built
	repeat = property(lambda self: self._repeat, set_repeat)


	def resize(self, widget, rectangle):
		"""Called when the window resizes."""
		width = rectangle[2]
		height = rectangle[3]
		try:
			awidth = width
			self.album_img.get_image() #raises an exception if not valid
		except:
			if ALBUM_ART.int_value:
				awidth = width - ALBUM_COVER_SIZE

		width = max(width, -1)
		awidth = max(awidth, -1)

		if self.display_size != (width, height):
			self.display_size = (width, height)
			self.display_box.set_size_request(awidth, -1)
			self.display_song.set_size_request(awidth, -1)
			self.display_album.set_size_request(awidth, -1)
			self.display_artist.set_size_request(awidth, -1)
			self.display_status.set_size_request(awidth, -1)


	def set_sensitive(self, state):
		self.buttons[BTN_PLAYLIST].set_sensitive(state)
		self.buttons[BTN_PLAY].set_sensitive(state)
		self.buttons[BTN_NEXT].set_sensitive(state)
		self.buttons[BTN_STOP].set_sensitive(state)


	def server(self):
		"""process external/remote commands"""
		def callback(window, cmd, args):
			if cmd == 'play':
				self.play()
			elif cmd == 'stop':
				self.stop()
			elif cmd == 'pause':
				self.pause()
			elif cmd == 'next':
				self.next()
			elif cmd == 'prev':
				self.prev()
			elif cmd == 'add_songs':
				self.add_songs(args)
			elif cmd == 'load_songs':
				self.load_songs(args)
			else:
				rox.info("Bad rpc message")

		xsoap.register_server("_ROX_MUSICBOX")
		xsoap.register_callback(callback)


	def update_thd(self, button=None):
		"""load songs from source dirs"""
		self.load()


	def refresh(self):
		self.library = [LIBRARY.value]
		self.update_thd()


	def loading(self):
		self.display_status.set_text(_("Loading")+': '+str(len(self.playlist)))
		if len(self.playlist):
			self.set_sensitive(True)
		else:
			self.set_sensitive(False)


	def load(self):
		"""Load the playlist either from a saved xml file, or from source dirs"""
		self.display_status.set_text(_("Loading songs, please wait..."))
		self.playlist.get_songs(self.library, self.loading, self.replace_library)
		self.display_status.set_text(_("Ready")+': '+_("loaded ")+str(len(self.playlist))+_(" songs"))

		if len(self.playlist):
			self.set_sensitive(True)
		else:
			self.set_sensitive(False)

		if self.replace_library and len(self.playlist):
			if self.shuffle:
				self.next()
			else:
				self.play()


	def save(self):
		"""Save the current list"""
#		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
		file = 'MyMusic.music'
		path = os.path.join(rox.basedir.save_config_path(APP_NAME, APP_DOMAIN), file)
		box = saving.SaveBox(self.playlist, path, 'application/x-music-playlist')
		box.show()


	def load_args(self, args, replace=True):
		"""Accept files and folders from the command line (or dropped on our icon)"""
		self.replace_library = replace
		if len(args):
			path = []
			for s in args:
				path.append(s)
			self.library = path
		self.update_thd()


	def add_songs(self, args):
		self.load_args(args, False)


	def load_songs(self, args):
		self.load_args(args, True)


	def play(self):
		"""Play the current song"""
		size = gtk.ICON_SIZE_SMALL_TOOLBAR

		try:
			self.player.stop()
			self.current_song = self.playlist.get()
			self.player.play(self.current_song.filename, self.current_song.type)
			self.image_play.set_from_stock('media-pause', size)
			self.buttons[BTN_PREV].set_sensitive(self.playlist.get_previous())
			self.display_song.set_text(self.current_song.title)
			self.display_artist.set_text(self.current_song.artist)
			self.display_album.set_text(self.current_song.album)

			tooltips.set_tip(self.buttons[BTN_PLAY], 'Play ['+self.current_song.title+']', tip_private=None)

		except TypeError, detail:
			rox.alert(str(detail))
		except:
			rox.alert(_("Failed to start playing %s") % self.current_song.filename)

		if self.playlistUI:
			self.playlistUI.sync()

		try:
			folder = os.path.dirname(self.current_song.filename)
			pixbuf = None
			for filename in ['.DirIcon',
				'Folder.jpg', 'folder.jpg', '.folder.jpg',
				'Folder.png', 'folder.png', '.folder.png',
				'Album.jpg', 'album.jpg', '.album.jpg',
				'Album.png', 'album.png', '.album.png',
				'Cover.jpg', 'cover.jpg', '.cover.jpg',
				'Cover.png', 'cover.png', '.cover.png',
				]:
				image = os.path.join(folder, filename)
				if os.access(image, os.R_OK):
					pixbuf = gtk.gdk.pixbuf_new_from_file_at_size(image, ALBUM_COVER_SIZE, ALBUM_COVER_SIZE)
					break
			self.album_img.set_from_pixbuf(pixbuf)
		except:
			pass

		#force a resize because the labels may have changed
		self.resize(None, [0, 0, 0, 0])


	def play_pause(self, button=None):
		"""Play button handler (toggle between play and pause)"""
		if (self.player.state == 'play') or (self.player.state == 'pause'):
			self.pause()
		else:
			self.play()


	def prev(self, button=None):
		"""Skip to previous song and play it"""
		self.current_song = self.playlist.prev()
		self.play()


	def next(self, button=None):
		"""Skip to next song and play it (with shuffle and repeat)"""
		if self.shuffle:
			self.playlist.shuffle()
			self.current_song = self.playlist.get()
		else:
			try:
				self.current_song = self.playlist.next()
			except StopIteration:
				if self.repeat:
					self.current_song = self.playlist.first()
				else:
					self.stop()
					return True
		self.play()


	def stop(self, button=None):
		"""Stop playing"""
		size = gtk.ICON_SIZE_SMALL_TOOLBAR
		self.player.stop()
		self.current_song = None
		self.image_play.set_from_stock('media-play', size)
		self.seek_bar.set_value(0.0)


	def pause(self, button=None):
		"""Pause playing (toggle)"""
		size = gtk.ICON_SIZE_SMALL_TOOLBAR
		self.player.pause()
		if (self.player.state == 'play'):
			self.image_play.set_from_stock('media-pause', size)
		else:
			self.image_play.set_from_stock('media-play', size)


	def display_update(self):
		duration = int(self.player.remain + self.player.elapse)
		if duration:
			progress = float(self.player.elapse)/duration
		else:
			progress = 0

		min = string.zfill(str(int(duration)%3600/60),2)
		sec = string.zfill(str(int(duration)%3600%60),2)
		total = min+':'+sec

		minremain = string.zfill(str(self.player.remain%3600/60),2)
		secremain = string.zfill(str(self.player.remain%3600%60),2)
		remain = minremain+':'+secremain

		minelapse = string.zfill(str(self.player.elapse%3600/60),2)
		secelapse = string.zfill(str(self.player.elapse%3600%60),2)
		elapse = minelapse+':'+secelapse

		show_remain = bool(TIMEDISPLAY.int_value)
		if show_remain:
			self.time_string = remain+' / '+total
		else:
			self.time_string = elapse+' / '+total

		state_string = ''
		if self.player.state == 'play':
			self.display_status.set_text(_("Playing")+': '+self.time_string)
			self.seek_bar.handler_block(self.seek_id)
			self.seek_bar.set_value(progress)
			self.seek_bar.handler_unblock(self.seek_id)
		elif self.player.state == 'pause':
			self.display_status.set_text(_("Paused")+': '+self.time_string)
		elif self.player.state == 'stop':
			self.display_status.set_text(_("Stopped"))
		elif self.player.state == 'eof':
			self.display_status.set_text("")
			self.next()

		if (self.window_state & gtk.gdk.WINDOW_STATE_ICONIFIED):
			self.set_title(self.current_song.title+' - '+self.time_string)
		else:
			tooltips.set_tip(self.seek_bar_control, self.time_string, tip_private=None)


		#update the volume control if something other than us changed it
		self.volume.set_value(self.player.get_volume(MIXER_DEVICE.value))

		return True #keep running


	def delete_event(self, ev, e1):
		"""Same as close, but called from the window manager"""
		self.close()


	def window_state_event(self, window, event):
		"""Track changes in window state and such..."""
		self.my_gdk_window = event.window
		self.window_state = event.new_window_state
		if not (self.window_state & gtk.gdk.WINDOW_STATE_ICONIFIED):
			self.set_title(APP_NAME)


	def close(self, button = None):
		"""Stop playing, kill the player and exit"""
		self.stop()
		if self.playlistUI:
			self.playlistUI.close()

		xsoap.unregister_server("_ROX_MUSICBOX")
		self.destroy()


	def get_options(self):
		"""Used as the notify callback when options change"""
		if SHUFFLE.has_changed:
			self.shuffle = SHUFFLE.int_value

		if REPEAT.has_changed:
			self.repeat = REPEAT.int_value

		if SONG_FONT.has_changed or BASE_FONT.has_changed:
			self.set_fonts()

		if FG_COLOR.has_changed or BG_COLOR.has_changed:
			self.set_colors()

		if TOOLOPTIONS.has_changed:
			self.show_hide_buttons()

		if SH_TOOLBAR.has_changed or SH_VOLUME.has_changed or SH_SEEKBAR.has_changed:
			self.show_hide_controls()

		if MINITOOLS.has_changed:
			if bool(MINITOOLS.int_value):
				self.toolbar.set_icon_size(gtk.ICON_SIZE_MENU)
			else:
				self.toolbar.set_icon_size(gtk.ICON_SIZE_SMALL_TOOLBAR)

		if WORDWRAP.has_changed:
			self.set_line_wrap()

		if ALBUM_ART.has_changed:
			if ALBUM_ART.int_value:
				self.album_img.show()
			else:
				self.album_img.hide()
#			self.resize(None, [0, 0, 0, 0])


	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()


	def show_playlist(self, button=None):
		"""Display the playlist window"""
		if not self.playlistUI:
			self.playlistUI = playlistui.PlaylistUI(self.playlist, self)
			self.playlistUI.connect('destroy', self.playlist_close)
			self.buttons[BTN_PLAYLIST].set_sensitive(False)


	def show_dir(self, *dummy):
		''' Pops up a filer window containing the current song, or the library location '''
		if self.current_song:
			filer.show_file(self.playlist.get().filename)
		else:
			filer.show_file(os.path.expanduser(LIBRARY.value))


	def playlist_close(self, item=None):
		"""Notice when the playlistUI goes away (so we don't crash)"""
		self.playlistUI = None
		self.buttons[BTN_PLAYLIST].set_sensitive(True)


	def button_press(self, text, event):
		"""Popup menu handler"""
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
#GTK2.4		self.menu.popup(None, None, None, event.button, 0)
		return 1

	def menukey_press(self, widget):
		''' Called when the user hits the menu key on their keyboard. '''
		self.menu.popup(self, None)

	def get_info(self):
		InfoWin.infowin(APP_NAME)


	def adjust_seek_bar(self, pos):
		"""Set the playback position (seek)"""
		self.player.seek(pos.get_value())


	def adjust_volume(self, vol):
		"""Set the playback volume"""
		self.player.set_volume(vol.get_value(), MIXER_DEVICE.value)


	def xds_drag_motion(self, widget, context, x, y, timestamp):
		pass


	def xds_drag_drop(self, widget, context, data, info, time):
		"""Check if the Shift key is pressed or not when Dropping files"""
		if context.actions & gtk.gdk.ACTION_MOVE:
			self.replace_library = True
		else:
			self.replace_library = False
		return loading.XDSLoader.xds_drag_drop(self, widget, context, data, info, time)


	def xds_load_uris(self, uris):
		"""Accept files and folders dropped on us as new Library"""
		path = []
		#strip off the 'file://' part and concatenate them
		for s in uris:
			path.append(rox.get_local_path(s))
		self.library = path
		self.update_thd()
