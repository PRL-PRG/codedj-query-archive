"""
	musicbox.py (play either ogg or mp3 files)

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

from __future__ import generators

import os, sys, re, string, threading
from threading import *
from SimpleXMLRPCServer import SimpleXMLRPCServer

import rox
from rox import g, Menu, app_options, loading, saving, InfoWin
from rox.options import Option

import player, playlist, playlistui


#Who am I and how did I get here?
APP_NAME = "MusicBox"
APP_DIR = rox.app_dir


#View options
VIEW_DEFAULT_SIZE = (100, 200)


#Bitmaps that are changed after initialization.
BMP_PAUSE = APP_DIR+'/pixmaps/media-pause.png'
BMP_PLAY = APP_DIR+'/pixmaps/media-play.png'


#Options.xml processing
rox.setup_app_options(APP_NAME)

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', os.path.expanduser('~')+'/Music')

#how to parse each library leaf to get artist, album, title...
LIBRARY_RE = Option('library_re', '^.*/(?P<artist>.*)/(?P<album>.*)/(?P<title>.*)')

#the ao driver type you want to use (esd, oss, alsa, alsa09, ...)
DRIVER_ID = Option('driver_id', 'esd')

SHUFFLE = Option('shuffle', 0)
REPEAT = Option('repeat', 0)

#Don't replay any of the last n songs in shuffle mode
SHUFFLE_CACHE_SIZE = Option('shuffle_cache', 10)

#buffer size used by audio device read/write
AUDIO_BUFFER_SIZE = Option('audio_buffer', 4096)

rox.app_options.notify()


DND_TYPES = ['audio/x-mp3' 'application/ogg' 'inode/directory']

class MusicBox(rox.Window, loading.XDSLoader):
	"""A Music Player for mp3 and ogg - main class"""

	def __init__(self):
		"""Constructor for MusicBox"""
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, DND_TYPES)

		# Main window settings
		self.set_title(APP_NAME)
		self.set_border_width(0)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_MOUSE)
		#self.set_resizable(False)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)

		self.connect('delete_event', self.delete_event)
		self.connect('window-state-event', self.window_state_event)
		self.connect('drag-motion', self.xds_drag_motion)

		# Set some defaults
		self.replace_library = False
		self.library = LIBRARY.value.split(':')

		self.playlist = None
		self.current_song = None

		# Menu
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			Menu.Action(_("Play")+'\/'+_("Pause"), 'play_pause'),
			Menu.Action(_("Stop"), 'stop'),
			Menu.Separator(),
			Menu.Action(_("Back"), 'prev'),
			Menu.Action(_("Next"), 'next'),
			Menu.Separator(),
			Menu.Action(_("Save"), 'save', '', g.STOCK_SAVE),
			Menu.Action(_("Refresh"), 'refresh', '', g.STOCK_REFRESH),
			Menu.Separator(),
			Menu.Action(_("Options"), 'show_options', '', g.STOCK_PREFERENCES),
			Menu.Action(_('Info'),	'get_info', '', g.STOCK_DIALOG_INFO),
			Menu.Separator(),
			Menu.Action(_("Quit"), 'close', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)

		# Toolbar
		self.toolbar = g.Toolbar()
		self.toolbar.set_style(g.TOOLBAR_ICONS)

		self.toolbar.insert_stock(g.STOCK_PREFERENCES, _("Options"),
					None, self.show_options, None, 0)
		self.list_btn = self.toolbar.insert_stock(g.STOCK_INDEX, _("Playlist"),
					None, self.show_playlist, None, 0)
		self.list_btn.set_sensitive(False)

		self.toolbar.insert_space(0)

		image_shuffle = g.Image()
		image_shuffle.set_from_file(APP_DIR+'/pixmaps/media-shuffle.png')
		self.shuffle = self.toolbar.insert_element(g.TOOLBAR_CHILD_TOGGLEBUTTON,
					None, _("Shuffle"), _("Shuffle"),None,
					image_shuffle, None, None, 0)
		self.shuffle.set_active(SHUFFLE.int_value)

		image_repeat = g.Image()
		image_repeat.set_from_file(APP_DIR+'/pixmaps/media-repeat.png')
		self.repeat = self.toolbar.insert_element(g.TOOLBAR_CHILD_TOGGLEBUTTON,
					None, _("Repeat"), _("Repeat"), None,
					image_repeat, None, None, 0)
		self.repeat.set_active(REPEAT.int_value)

		self.toolbar.insert_space(0)

		image_next = g.Image()
		image_next.set_from_file(APP_DIR+'/pixmaps/media-next.png')
		self.next_btn = self.toolbar.insert_item(_("Next"), _("Next"),
					None, image_next, self.next, None, 0)
		self.next_btn.set_sensitive(False)

		image_stop = g.Image()
		image_stop.set_from_file(APP_DIR+'/pixmaps/media-stop.png')
		self.stop_btn = self.toolbar.insert_item(_("Stop"), _("Stop"),
					None, image_stop, self.stop, None, 0, )
		self.stop_btn.set_sensitive(False)

		image_play = g.Image()
		self.image_play = image_play
		image_play.set_from_file(BMP_PLAY)
		self.play_btn = self.toolbar.insert_item(_("Play")+'/'+_("Pause"), _("Play")+'/'+_("Pause"),
					None, image_play, self.play_pause, None, 0)
		self.play_btn.set_sensitive(False)

		image_prev = g.Image()
		image_prev.set_from_file(APP_DIR+'/pixmaps/media-prev.png')
		self.prev_btn = self.toolbar.insert_item(_("Prev"), _("Prev"),
					None, image_prev, self.prev, None, 0)
		self.prev_btn.set_sensitive(False)

		# Create layout, and text display(s)
		self.display = g.Layout()
		self.display.set_size_request(250, 140)
		self.display.modify_bg(g.STATE_NORMAL, g.gdk.color_parse('#A6A699'))

		self.display_song = g.Label()
		self.display_song.set_line_wrap(True)
		self.display_song.set_size_request(250, 90)
		self.display.put(self.display_song, 10, 0)
		self.display_song.set_alignment(0.0, 0.1)

		self.display_status = g.Label()
		self.display.put(self.display_status, 10, 100)

		self.display_time = g.Label()
		self.display.put(self.display_time, 10, 120)

		self.volume = g.Adjustment(50.0, 0.0, 100.0, 1.0, 10.0, 0.0)
		self.volume.connect('value_changed', self.adjust_volume)
		self.volume_control = g.VScale(self.volume)
		self.volume_control.set_draw_value(False)
		self.volume_control.set_inverted(True)
		self.volume_control.set_size_request(30, 100)

		self.we_did_it = False
		self.seek_bar = g.Adjustment(0.0, 0.0, 1.0, 0.01, 0.1, 0.0)
		self.seek_bar.connect('value_changed', self.adjust_seek_bar)
		seek_bar_control = g.HScale(self.seek_bar)
		seek_bar_control.set_update_policy(g.UPDATE_DELAYED)
		seek_bar_control.set_draw_value(False)
		seek_bar_control.set_size_request(100, 30)

		# Pack and show widgets
		self.vbox = g.VBox()
		self.hbox = g.HBox()
		self.add(self.vbox)
		self.vbox.add(self.hbox)

		self.hbox.pack_start(self.display, True, True, 0)
		self.hbox.pack_end(self.volume_control, False, True, 0)
		self.vbox.pack_end(self.toolbar, False, True, 0)
		self.vbox.pack_end(seek_bar_control, False, True, 0)
		self.vbox.show_all()

		# Kick things off...
		self.show()

		self.playlist = playlist.Playlist(SHUFFLE_CACHE_SIZE.int_value, LIBRARY_RE.value)
		self.playlistUI = None
		self.player = player.Player(self.status_update,
								DRIVER_ID.value,
								AUDIO_BUFFER_SIZE.int_value)
		self.foo = Thread(name='player', target=self.player.run)
		self.foo.setDaemon(True)
		self.foo.start()
		self.volume.set_value(self.player.get_volume())

		if len(sys.argv) > 1:
			self.load_args(sys.argv[1:], True)
		else:
			self.load_args([], False)

		#start xmlrpc server to listen for remote commands
		thd_load = Thread(name='xmlrpc', target=self.server)
		thd_load.setDaemon(True)
		thd_load.start()

	def set_sensitive(self, state):
		self.list_btn.set_sensitive(state)
		self.play_btn.set_sensitive(state)
		self.next_btn.set_sensitive(state)
		self.stop_btn.set_sensitive(state)

	def server(self):
		"""Run an XMLRPC server to process external/remote commands"""
		server = SimpleXMLRPCServer(('localhost', 8989))
		server.register_function(self.add_songs)
		server.register_function(self.load_songs)
		server.register_function(self.play)
		server.register_function(self.prev)
		server.register_function(self.next)
		server.register_function(self.stop)
		server.register_function(self.pause)
		server.serve_forever()

	def update_thd(self, button=None):
		"""Thread to load songs from source dirs"""
		thd_load = Thread(name='load', target=self.load)
		thd_load.setDaemon(True)
		thd_load.start()

	def refresh(self):
		self.library = [LIBRARY.value]
		self.update_thd()

	def loading(self):
		pass
		g.threads_enter()
		self.display_status.set_text(_("Loading")+': '+str(len(self.playlist)))
		if len(self.playlist):
			self.set_sensitive(True)
		else:
			self.set_sensitive(False)
		g.threads_leave()

	def load(self):
		"""Load the playlist either from a saved xml file, or from source dirs"""
		g.threads_enter()
		self.display_status.set_text(_("Loading songs, please wait..."))
		if self.playlistUI:
			self.playlistUI.view.set_model(None)
		g.threads_leave()

		self.playlist.get_songs(self.library, self.loading, self.replace_library)

		g.threads_enter()
		self.display_status.set_text(_("Ready")+': '+_("loaded ")+str(len(self.playlist))+_(" songs"))

		if len(self.playlist):
			self.set_sensitive(True)
		else:
			self.set_sensitive(False)

		if self.playlistUI:
			self.playlistUI.view.set_model(self.playlist.song_list)

		if self.replace_library and len(self.playlist):
			self.play()

		g.threads_leave()

	def save(self):
		"""Save the current list"""
		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
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
		return True #needed for xmlrpc

	def load_songs(self, args):
		self.load_args(args, True)
		return True #needed for xmlrpc

	def play(self):
		"""Play the current song"""
		try:
			self.player.stop()
			self.current_song = self.playlist.get()
			self.player.play(self.current_song.filename, self.current_song.type)
			self.image_play.set_from_file(BMP_PAUSE)
			self.prev_btn.set_sensitive(self.playlist.get_previous())
			self.display_song.set_text(self.current_song.title+"\n"+ \
										self.current_song.artist+"\n"+ \
										self.current_song.album)
		except TypeError, detail:
			rox.info(str(detail))
			return False #needed for xmlrpc
		except:
			rox.info(_("Failed to start playing %s") % self.current_song.filename)
			return False #needed for xmlrpc

		if self.playlistUI:
			self.playlistUI.sync()
		return True #needed for xmlrpc

	def play_pause(self, button=None):
		"""Play button handler (toggle between play and pause)"""
		if (self.player.state == 'play') or (self.player.state == 'pause'):
			self.pause()
		else:
			self.play()
		return True #needed for xmlrpc

	def prev(self, button=None):
		"""Skip to previous song and play it"""
		self.current_song = self.playlist.prev()
		self.play()
		return True #needed for xmlrpc

	def next(self, button=None):
		"""Skip to next song and play it (with shuffle and repeat)"""
		if self.shuffle.get_active():
			self.playlist.shuffle()
			self.current_song = self.playlist.get()
		else:
			try:
				self.current_song = self.playlist.next()
			except StopIteration:
				if self.repeat.get_active():
					self.current_song = self.playlist.first()
		self.play()
		return True #needed for xmlrpc

	def stop(self, button=None):
		"""Stop playing"""
		self.player.stop()
		self.foo = None
		self.image_play.set_from_file(BMP_PLAY)
		return True #needed for xmlrpc

	def pause(self, button=None):
		"""Pause playing (toggle)"""
		self.player.pause()
		if (self.player.state == 'play'):
			self.image_play.set_from_file(BMP_PAUSE)
		else:
			self.image_play.set_from_file(BMP_PLAY)
		return True #needed for xmlrpc

	def status_update(self, state, remain, progress):
		"""Status update (elapsed time, end of song, etc."""
		g.threads_enter()
		self.volume.set_value(self.player.get_volume())

		if state == 'play':
			duration = int(remain + progress)

			min = string.zfill(str(int(progress)%3600/60),2)
			sec = string.zfill(str(int(progress)%3600%60),2)

			minremain = string.zfill(str(remain%3600/60),2)
			secremain = string.zfill(str(remain%3600%60),2)

			self.display_status.set_text(_("Playing"))
			self.display_time.set_text(_("Time Remaining")+': '+minremain+':'+secremain)

			if (self.window_state & g.gdk.WINDOW_STATE_ICONIFIED):
				self.set_title(self.current_song.title+' - '+minremain+':'+secremain)

			self.we_did_it = True
			self.seek_bar.set_value(float(progress)/duration)

		elif state == 'pause':
			self.display_status.set_text(_("Paused"))
		elif state == 'stop':
			self.display_status.set_text(_("Stopped"))
		elif state == 'eof':
			self.next()
		g.threads_leave()

	def delete_event(self, ev, e1):
		"""Same as close, but called from the window manager"""
		self.close()

	def window_state_event(self, window, event):
		"""Track changes in window state and such..."""
		self.my_gdk_window = event.window
		self.window_state = event.new_window_state
		if not (self.window_state & g.gdk.WINDOW_STATE_ICONIFIED):
			self.set_title(APP_NAME)

	def close(self, button = None):
		"""Stop playing, kill the player and exit"""
		self.stop()
		if self.playlistUI:
			self.playlistUI.close()
		self.destroy()

	def get_options(self):
		"""Used as the notify callback when options change"""
		if SHUFFLE.has_changed:
			self.shuffle.set_active(SHUFFLE.int_value)

		if REPEAT.has_changed:
			self.repeat.set_active(REPEAT.int_value)

	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()

	def show_playlist(self, button=None):
		"""Display the playlist window"""
		if not self.playlistUI:
			self.playlistUI = playlistui.PlaylistUI(self.playlist)
			self.playlistUI.connect('destroy', self.playlist_close)
			self.list_btn.set_sensitive(False)

	def playlist_close(self, item=None):
		"""Notice when the playlistUI goes away (so we don't crash)"""
		self.playlistUI = None
		self.list_btn.set_sensitive(True)

	def button_press(self, text, event):
		"""Popup menu handler"""
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def adjust_seek_bar(self, pos):
		"""Set the playback position (seek)"""
		if self.we_did_it:
			#ignore updates caused by playback progress
			self.we_did_it = False
		else:
			#process those caused by dragging the slider
			self.player.seek(pos.get_value())

	def adjust_volume(self, vol):
		"""Set the playback volume"""
		self.player.set_volume(vol.get_value())

	def xds_drag_motion(self, widget, context, x, y, timestamp):
		pass

	def xds_drag_drop(self, widget, context, data, info, time):
		"""Check if the Shift key is pressed or not when Dropping files"""
		if context.actions & g.gdk.ACTION_MOVE:
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
		thd_load = Thread(name='load', target=self.load)
		thd_load.setDaemon(True)
		thd_load.start()

