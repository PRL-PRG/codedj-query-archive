from __future__ import generators

import rox
from rox import g, Menu, app_options, loading, mime
from rox.options import Option

import os, sys, re, string, threading, time, stat
from threading import *

import playlist


#Who am I and how did I get here?
APP_NAME = 'MusicBox'
APP_DIR = rox.app_dir

#View options
VIEW_DEFAULT_SIZE = (600, 400)

#Column indicies
COL_FILE = 0
COL_ARTIST = 1
COL_TITLE = 2
COL_TRACK = 3
COL_ALBUM = 4
COL_GENRE = 5
COL_LENGTH = 6
COL_INFO = 7
COL_TYPE = 8

COLUMNS = [
	(_('Artist'), COL_ARTIST),
	(_('Title'), COL_TITLE),
	(_('Album'), COL_ALBUM),
	(_('Track'), COL_TRACK),
	(_('Genre'), COL_GENRE),
	(_('Length'), COL_LENGTH),
	(_('Type'), COL_TYPE),
	(_('Comment'), COL_INFO),
]

DND_TYPES = ['audio/x-mp3' 'application/ogg' 'inode/directory']


class PlaylistUI(rox.Window, loading.XDSLoader):
	"the playlist UI for MusicBox"
	def __init__(self, the_playlist):
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, DND_TYPES)

		self.playlist = the_playlist

		self.set_title(APP_NAME+' - Playlist')
		self.set_border_width(1)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_NONE)


		#capture wm delete event
		self.connect("delete_event", self.delete_event)

		self.replace_library = True


		# Menu
		#######################################
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			(_('/Play\/Pause'), 'play_selected', '', '', 0),
			(_('/Stop'), 'stop', '', '', 0),
			('/','','<Separator>','', 0),
			(_('/Back'), 'prev', '', '', 0),
			(_('/Next'), 'next', '', '', 0),
			('/','','<Separator>','', 0),
			(_('/Filter/All songs'), 'filter_none', '', '', 0),
			(_('/Filter/This Artist'), 'filter_artist', '', '', 0),
			(_('/Filter/This Album'), 'filter_album', '', '', 0),
			(_('/Filter/This Genre'), 'filter_genre', '', '', 0),
			(_('/Filter/New Filter...'), 'filter_new', '', '', 0),
			('/','','<Separator>','', 0),
			(_('/Save'), 'save', '<StockItem>', '', g.STOCK_SAVE),
			(_('/Refresh'), 'update_thd', '<StockItem>', '', g.STOCK_REFRESH),
			('/','','<Separator>','', 0),
			(_('/Quit'), 'close', '<StockItem>', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


		# Toolbar
		#######################################
		self.toolbar = g.Toolbar()
		self.toolbar.set_style(g.TOOLBAR_ICONS)

		self.toolbar.insert_stock(g.STOCK_REFRESH, _('Refresh'),
					None, self.update_thd, None, 0)

		self.toolbar.insert_space(0)

		image_next = g.Image()
		image_next.set_from_file(APP_DIR+"/pixmaps/media-next.png")
		self.toolbar.insert_item(_('Next'), _('Next'),
					None, image_next, self.next, None, 0)

		image_stop = g.Image()
		image_stop.set_from_file(APP_DIR+"/pixmaps/media-stop.png")
		self.toolbar.insert_item(_('Stop'), _('Stop'),
					None, image_stop, self.stop, None, 0, )

		image_play = g.Image()
		self.image_play = image_play
		image_play.set_from_file(APP_DIR+"/pixmaps/media-play.png")
		self.toolbar.insert_item(_('Play/Pause'), _('Play/Pause'),
					None, image_play, self.play_selected, None, 0)

		image_prev = g.Image()
		image_prev.set_from_file(APP_DIR+"/pixmaps/media-prev.png")
		self.toolbar.insert_item(_('Prev'), _('Prev'),
					None, image_prev, self.prev, None, 0)

		self.toolbar.insert_space(0)
		self.toolbar.insert_stock(g.STOCK_CLOSE, _('Close'),
					None, self.close, None, 0)

		# Playlist
		#######################################
		swin = g.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(4)
		swin.set_policy(g.POLICY_AUTOMATIC, g.POLICY_AUTOMATIC)
		swin.set_shadow_type(g.SHADOW_IN)

		self.song_list = g.ListStore(str, str, str, str, str, str, str, str, str)
		view = g.TreeView(self.song_list)
		self.view = view
		swin.add(view)
		#view.set_rules_hint(True)

		self.view.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.view.connect('button-press-event', self.button_press)


		#TODO: Drag and Drop from MusicBox...
		view.drag_source_set(g.gdk.BUTTON1_MASK,
			[('XdndDirectSave0', 0, 0),
			('application/octet-stream', 0, 1)],
			g.gdk.ACTION_COPY)
		view.connect('drag_data_get', self.on_drag_data_get)

		#TODO: A little icon showing the current song playing...
		cell = g.CellRendererPixbuf()
		column = g.TreeViewColumn('', cell)
		view.append_column(column)
		column.set_resizable(False)
		column.set_reorderable(False)

		for n in range(len(COLUMNS)):
			cell = g.CellRendererText()
			column = g.TreeViewColumn(COLUMNS[n][0], cell, text = COLUMNS[n][1])
			view.append_column(column)
			column.set_sort_column_id(COLUMNS[n][1])
			column.set_resizable(True)
			column.set_reorderable(True)
			column.connect('clicked', self.col_activate)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.view.set_search_column(COL_ARTIST)
#		self.view.get_column(COL_ARTIST-1).clicked()

		#TODO: Multiple Selections
		#self.selection.set_mode(g.SELECTION_MULTIPLE)



		# Create layout, pack and show widgets
		#######################################
		self.vbox = g.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.pack_end(self.toolbar, False, True, 0)
		self.vbox.show_all()

		self.show()

		self.current_song = None

		self.update_thd()


	####################################################################
	def load(self):
		pass


	####################################################################
	def save(self):
		"Save the current list"
		self.playlist.save(rox.choices.save(APP_NAME, 'Library.xml'))


	####################################################################
	def update_thd(self, button=None):
		"Thread to load songs from source dirs"
		thd_update = Thread(name='update', target=self.refresh)
		thd_update.setDaemon(True)
		thd_update.start()


	####################################################################
	def set_song_info(self, iter, song):
		"Copy the song info (tags) into the TreeModel iter"
		if song.filename: self.song_list.set(iter, COL_FILE, song.filename)
		if song.title: self.song_list.set(iter, COL_TITLE, song.title)
		if song.track: self.song_list.set(iter, COL_TRACK, song.track)
		if song.album: self.song_list.set(iter, COL_ALBUM,  song.album)
		if song.artist: self.song_list.set(iter, COL_ARTIST, song.artist)
		if song.genre: self.song_list.set(iter, COL_GENRE,  song.genre)
		if song.comment: self.song_list.set(iter, COL_INFO, song.comment)
		if song.length: self.song_list.set(iter, COL_LENGTH, song.length)
		if song.type: self.song_list.set(iter, COL_TYPE,  song.type)


	####################################################################
	def refresh(self):
		"Re-display the playlist (don't get any new info)"
		iter = self.song_list.get_iter_root()
		(c, dummy) = self.view.get_cursor()
		if c == None:
			c = (0,)

		g.threads_enter()
		self.song_list.clear()
		g.threads_leave()

		for song in self.playlist:
			if song != None:
				g.threads_enter()
				iter = self.song_list.append(None)
				self.set_song_info(iter, song)
				g.threads_leave()

		g.threads_enter()
		self.view.set_cursor(c)
		g.threads_leave()


	####################################################################
	def play(self):
		"Play the current song"
		pass


	####################################################################
	def activate(self, view, path, column):
		"Double-click handler, plays the song"
		self.play()


	####################################################################
	def play_selected(self, button=None):
		"Play button handler (toggle between play and pause)"
		pass


	####################################################################
	def prev(self, button=None):
		"Skip to previous song and play it"
		path, column = self.view.get_cursor()
		n = max(0, path[0]-1)
		path = (n,)
		self.view.set_cursor(path)
		self.play()


	####################################################################
	def next(self, button=None):
		"Skip to next song and play it (with shuffle and repeat)"
		path, column = self.view.get_cursor()
		n = max(0, path[0]+1)
		path = (n,)
		self.view.set_cursor(path)
		self.play()


	####################################################################
	def stop(self, button=None):
		"Stop playing"
		pass


	####################################################################
	def pause(self, button=None):
		"Pause playing (toggle)"
		pass


	####################################################################
	def delete_event(self, ev, e1):
		"Same as close, but called from the window manager"
		self.close()


	####################################################################
	def close(self, button = None):
#		if LIBRARY.has_changed:
#			if rox.confirm(_('Save library changes and playlist?'), g.STOCK_SAVE):
#				rox.app_options.save()
#				self.save()

		self.destroy()


	####################################################################
	def button_press(self, text, event):
		"Popup menu handler"
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1


	####################################################################
	def col_activate(self, column):
		"Set the selected column as the search <Ctrl-S> column"
		self.view.set_search_column(column.get_sort_column_id())


	####################################################################
	def xds_drag_drop(self, widget, context, data, info, time):
		"Check if the Shift key is pressed or not when Dropping files"
		if context.actions & g.gdk.ACTION_MOVE:
			pass
		if context.actions & g.gdk.ACTION_COPY:
			self.replace_library = True
		else:
			self.replace_library = False
		return loading.XDSLoader.xds_drag_drop(self, widget, context, data, info, time)


	####################################################################
	def xds_load_uris(self, uris):
		"Accept files and folders dropped on us as new Library"
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
#		if self.replace_library:
#			LIBRARY.value = path
#		else:
#			LIBRARY.value += ':'+path

		#rox.app_options.save()
#		LIBRARY.has_changed = True
		self.update_thd()


	#TODO: Drag and Drop from MusicBox...
	####################################################################
	def on_drag_data_get(self, treeview, context, selection, info, time):
		"Tell the drop target what the filename(s) is(are)"
		#print treeview, context, selection, info

		model, iter = self.selection.get_selected()
		self.current_file = model.get_value(iter, COL_FILE)
		payload = model.get_value(iter, COL_FILE)

		print 'Source: %s' % payload

		selection.set(selection.target, 8, payload)
		model.remove(iter)


	####################################################################
	def filter_none(self):
		"Clear any filter and show all songs"
		self.playlist.the_filter = {}
		self.update_thd()


	####################################################################
	def filter_album(self):
		"Filter by the currently selected album"
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			album = model.get_value(iter, COL_ALBUM)
			self.playlist.the_filter = {'album':[album]}
			self.update_thd()


	####################################################################
	def filter_artist(self):
		"Filter by the currently selected artist"
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			artist = model.get_value(iter, COL_ARTIST)
			self.playlist.the_filter = {'artist':[artist]}
			self.update_thd()


	####################################################################
	def filter_genre(self):
		"Filter by the currently selected genre"
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			genre = model.get_value(iter, COL_GENRE)
			self.playlist.the_filter = {'genre':[genre]}
			self.update_thd()


	####################################################################
	def filter_new(self):
		"Create a new filter via a dialog"
		rox.info("Not implemented yet")
		pass

