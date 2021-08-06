from __future__ import generators

import rox
from rox import g, Menu, app_options, loading, saving, mime, InfoWin
from rox.options import Option

import os, sys, re, string, threading
from threading import *

import player, playlist, playlistui

from SimpleXMLRPCServer import SimpleXMLRPCServer


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
		#######################################
		self.set_title(APP_NAME)
		self.set_border_width(1)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_MOUSE)
		#self.set_resizable(False)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)

		self.connect('delete_event', self.delete_event)

		# Set some defaults
		#######################################
		self.replace_library = True
		self.player = None
		self.playlist = None
		self.current_song = None

		self.playlist = playlist.Playlist(self.status_update)
		self.playlistUI = None


		# Menu
		#######################################
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			('/'+_("Play")+'\/'+_("Pause"), 'play_pause', '', '', 0),
			('/'+_("Stop"), 'stop', '', '', 0),
			('/','','<Separator>','', 0),
			('/'+_("Back"), 'prev', '', '', 0),
			('/'+_("Next"), 'next', '', '', 0),
			('/','','<Separator>','', 0),
			('/'+_("Save"), 'save', '<StockItem>', '', g.STOCK_SAVE),
			('/'+_("Refresh"), 'update_thd', '<StockItem>', '', g.STOCK_REFRESH),
			('/','','<Separator>','', 0),
			('/'+_("Options"), 'show_options', '<StockItem>', '', g.STOCK_PREFERENCES),
			('/'+_('Info'),	'get_info',    '<StockItem>', '', g.STOCK_DIALOG_INFO),
			('/','','<Separator>','', 0),
			('/'+_("Quit"), 'close', '<StockItem>', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


		# Toolbar
		#######################################
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
		#######################################
		self.display = g.Layout()
		self.display.modify_bg(g.STATE_NORMAL, g.gdk.color_parse('#A6A699'))

		self.display_song = g.Label()
		self.display_song.set_line_wrap(True)
		self.display.put(self.display_song, 10, 10)

		self.display_status = g.Label()
		self.display.put(self.display_status, 10, 90)

		self.display_time = g.Label()
		self.display.put(self.display_time, 10, 110)

		self.volume = g.Adjustment(0.5, 0.0, 1.0, 0.1, 0.1, 0.0)
		self.volume.connect('value_changed', self.adjust_volume)
		self.volume_control = g.VScale(self.volume)
		self.volume_control.set_draw_value(False)
		self.volume_control.set_inverted(True)
		self.volume_control.set_size_request(30, 100)

		self.we_did_it = False
		self.seek_bar = g.Adjustment(0.5, 0.0, 1.0, 0.1, 0.1, 0.0)
		self.seek_bar.connect('value_changed', self.adjust_seek_bar)
		seek_bar_control = g.HScale(self.seek_bar)
		seek_bar_control.set_update_policy(g.UPDATE_DELAYED)
		seek_bar_control.set_draw_value(False)
		seek_bar_control.set_size_request(100, 30)

		# Pack and show widgets
		#######################################
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
		#######################################
		self.show()

		self.load_args(sys.argv[1:])

		#start xmlrpc server to listen for remote commands
		thd_load = Thread(name='xmlrpc', target=self.server)
		thd_load.setDaemon(True)
		thd_load.start()


	####################################################################
	def set_sensitive(self, state):
		self.list_btn.set_sensitive(state)
		self.play_btn.set_sensitive(state)
		self.prev_btn.set_sensitive(state)
		self.next_btn.set_sensitive(state)
		self.stop_btn.set_sensitive(state)


	####################################################################
	def server(self):
		"""Run an XMLRPC server to process external/remote commands"""
		server = SimpleXMLRPCServer(('localhost', 8989))
#		server.register_introspection_functions()
		server.register_function(self.load_args)
		server.register_function(self.play)
		server.register_function(self.prev)
		server.register_function(self.next)
		server.register_function(self.stop)
		server.register_function(self.pause)
		server.serve_forever()


	####################################################################
	def update_thd(self, button=None):
		"""Thread to load songs from source dirs"""
		thd_load = Thread(name='load', target=self.load)
		thd_load.setDaemon(True)
		thd_load.start()


	####################################################################
	def load(self):
		"""Load the playlist either from a saved xml file, or from source dirs"""

		g.threads_enter()
		self.display_status.set_text(_("Loading songs, please wait..."))
		g.threads_leave()

		self.playlist.get_songs(os.path.expanduser(LIBRARY.value), LIBRARY_RE.value)

		g.threads_enter()
		self.display_status.set_text(_("Ready")+': '+_("loaded ")+str(len(self.playlist))+_(" songs"))

		if len(self.playlist):
			self.set_sensitive(True)
		else:
			self.set_sensitive(False)

		if LIBRARY.has_changed and len(self.playlist):
			self.play()
		g.threads_leave()

		LIBRARY.has_changed = False


	####################################################################
	def save(self):
		"""Save the current list"""
		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
		box.show()


	####################################################################
	def load_args(self, args):
		"""Accept files and folders from the command line (or dropped on our icon)"""
		if len(args):
			path = ''
			#concatenate them all together with ':', like a PATH.
			for s in args:
				if path == '':
					path = s
				else:
					path = path+':'+s

			#Shift key is down or not?  Add vs Replace
			if self.replace_library:
				LIBRARY.value = path
			else:
				LIBRARY.value += ':'+path
			LIBRARY.has_changed = True

		self.update_thd()

		return True


	####################################################################
	def play(self):
		"""Play the current song"""
		self.image_play.set_from_file(BMP_PAUSE)

		self.current_song = self.playlist.get()

		if self.player and self.player.state != 'stop':
			self.player.stop()

		self.player = None
		self.foo = None

		try:
			self.player = player.Player(self.current_song.filename,
								str(mime.get_type(self.current_song.filename)),
								self.status_update,
								DRIVER_ID.value,
								AUDIO_BUFFER_SIZE.int_value)

			self.volume.set_value(self.player.get_volume())
			self.foo = Thread(name='player', target=self.player.play)
			self.foo.setDaemon(True)
			self.foo.start()

		except TypeError, detail:
			rox.info(str(detail))
			return False

		except:
			rox.info(_("Failed to start playing %s") % self.current_song.filename)
			return False

		if self.playlistUI:
			self.playlistUI.sync()

		return True


	####################################################################
	def play_pause(self, button=None):
		"""Play button handler (toggle between play and pause)"""
		if (self.player) and ((self.player.state == 'play') or
			(self.player.state == 'pause')):
			self.pause()
		else:
			self.play()
		return True


	####################################################################
	def prev(self, button=None):
		"""Skip to previous song and play it"""
		try:
			self.current_song = self.playlist.prev()
		except StopIteration:
			if self.repeat.get_active():
				self.current_song = self.playlist.last()

		self.play()

		return True


	####################################################################
	def next(self, button=None):
		"""Skip to next song and play it (with shuffle and repeat)"""

		if self.shuffle.get_active():
			self.playlist.shuffle()

		try:
			self.current_song = self.playlist.next()
		except StopIteration:
			if self.repeat.get_active():
				self.current_song = self.playlist.first()

		self.play()

		return True


	####################################################################
	def stop(self, button=None):
		"""Stop playing"""
		if (self.player) and (self.player.state != 'stop'):
			self.player.stop()
			self.player = None
			self.foo = None

		self.display_status.set_text(_("Stopped"))
		self.display_song.set_text('')
		self.display_time.set_text('')
		self.image_play.set_from_file(BMP_PLAY)
		return True


	####################################################################
	def pause(self, button=None):
		"""Pause playing (toggle)"""
		self.player.pause()
		if (self.player) and (self.player.state == 'play'):
			self.image_play.set_from_file(BMP_PAUSE)
		else:
			self.image_play.set_from_file(BMP_PLAY)
		return True


	####################################################################
	def status_update(self, state, remain, progress):
		"""Status update (elapsed time, end of song, etc."""
		g.threads_enter()

		song_string = str(self.playlist.get_index()+1)+_(" of ")+str(len(self.playlist))

		if state == 'play':
			duration = int(remain + progress)

			min = string.zfill(str(int(progress)%3600/60),2)
			sec = string.zfill(str(int(progress)%3600%60),2)

			minremain = string.zfill(str(remain%3600/60),2)
			secremain = string.zfill(str(remain%3600%60),2)

			try:
				percent = 100*(float(progress)/duration)
				percent = repr(int(percent)) + '%'
			except ZeroDivisionError:
				pass

			self.display_status.set_text(_("Playing")+': '+song_string)
			self.display_song.set_text(self.current_song.title+"\n"+ \
										self.current_song.artist+"\n"+ \
										self.current_song.album)
			self.display_time.set_text(_("Time Remaining")+': '+minremain+':'+secremain)

			self.we_did_it = True
			self.seek_bar.set_value(float(progress)/duration)

		elif state == 'loading':
			self.display_status.set_text(_("Loading")+': '+str(len(self.playlist)))
			if len(self.playlist):
				self.set_sensitive(True)
			else:
				self.set_sensitive(False)
		elif state == 'pause':
			self.display_status.set_text(_("Paused")+': '+song_string)
		elif state == 'stop':
			self.display_status.set_text(_("Stopped"))
			self.display_song.set_text('')
			self.display_time.set_text('')
		elif state == 'eof':
			self.next()

		g.threads_leave()


	####################################################################
	def delete_event(self, ev, e1):
		"""Same as close, but called from the window manager"""
		self.close()


	####################################################################
	def close(self, button = None):
		"""Stop playing, kill the player and exit"""

		if self.player:
			self.stop()

		if self.playlistUI:
			self.playlistUI.close()

		self.destroy()


	####################################################################
	def get_options(self):
		"""Used as the notify callback when options change"""
		if SHUFFLE.has_changed:
			self.shuffle.set_active(SHUFFLE.int_value)

		if REPEAT.has_changed:
			self.repeat.set_active(REPEAT.int_value)


	####################################################################
	def show_options(self, button=None):
		"""Options edit dialog"""
		rox.edit_options()


	####################################################################
	def show_playlist(self, button=None):
		"""Display the playlist window"""
		self.playlistUI = playlistui.PlaylistUI(self.playlist, self.play)
		self.playlistUI.connect('destroy', self.playlist_close)


	def playlist_close(self, item=None):
		"""Notice when the playlistUI goes away (so we don't crash)"""
		self.playlistUI = None


	####################################################################
	def button_press(self, text, event):
		"""Popup menu handler"""
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	####################################################################
	def get_info(self):
		InfoWin.infowin(APP_NAME)


	####################################################################
	def adjust_seek_bar(self, pos):
		"""Set the playback position (seek)"""
		if self.we_did_it:
			#ignore updates caused by playback progress
			self.we_did_it = False
		else:
			#process those caused by dragging the slider
			if self.player:
				self.player.seek(pos.get_value())


	####################################################################
	def adjust_volume(self, vol):
		"""Set the playback volume"""
		if self.player:
			self.player.set_volume(vol.get_value())


	####################################################################
	def xds_drag_drop(self, widget, context, data, info, time):
		"""Check if the Shift key is pressed or not when Dropping files"""
		if context.actions & g.gdk.ACTION_MOVE:
			pass
		if context.actions & g.gdk.ACTION_COPY:
			self.replace_library = True
		else:
			self.replace_library = False
		return loading.XDSLoader.xds_drag_drop(self, widget, context, data, info, time)


	####################################################################
	def xds_load_uris(self, uris):
		"""Accept files and folders dropped on us as new Library"""
		path = ''
		#strip off the 'file://' part and concatenate them all
		#together with ':', like a PATH.
		for s in uris:
			x = re.match('^file://(.*)', s)
			if x:
				if path == '':
					path = x.group(1)
				else:
					path = path+':'+x.group(1)

		#Shift key is down or not?  Add vs Replace
		if self.replace_library:
			LIBRARY.value = path
		else:
			LIBRARY.value += ':'+path

		#rox.app_options.save()
		LIBRARY.has_changed = True

		thd_load = Thread(name='load', target=self.load)
		thd_load.setDaemon(True)
		thd_load.start()



