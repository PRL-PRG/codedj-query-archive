"""
	playlistui.py
		Playlist UI for MusicBox application.

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

import gtk, gobject, os, sys, re

import rox
from rox import Menu, saving, loading, mime, filer

import playlist
from playlist import COL_FILE, COL_TITLE, COL_TRACK, COL_ALBUM, COL_ARTIST
from playlist import COL_GENRE, COL_LENGTH, COL_TYPE, COL_ICON

import plugins

#Who am I and how did I get here?
APP_NAME = "MusicBox"
APP_DIR = rox.app_dir
APP_DOMAIN = 'hayber.us'

#View options
VIEW_DEFAULT_SIZE = (760, 400)


COLUMNS = [
#	(_("Filename"), COL_FILE, str, 100),
	(_("Artist"), COL_ARTIST, str, 200),
	(_("Album"), COL_ALBUM, str, 200),
	(_("Title"), COL_TITLE, str, 200),
	(_("Track"), COL_TRACK, int, 50),
	(_("Genre"), COL_GENRE, str, 80),
#	(_("Length"), COL_LENGTH, int, 60),
#	(_("Type"), COL_TYPE, str, 60),
]


class PlaylistUI(rox.Window, loading.XDSLoader):
	"""the playlist UI for MusicBox"""

	def __init__(self, the_playlist, musicbox):
		"""Constructor"""
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, plugins.TYPE_LIST)

		self.playlist = the_playlist  #this is a reference to the main playlist
		self.library = []
		self.replace_library = True
		self.musicbox = musicbox

		self.set_title(APP_NAME+' - '+_("Playlist"))
		self.set_role("PlayList")
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])

		#capture wm delete event
		self.connect('delete_event', self.delete_event)

		# Menu
		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.connect('popup-menu', self.menukey_press)

		self.menu = Menu.Menu('main', [
			Menu.Action(_("Play"), 'play'),
			Menu.Action(_("Delete"), 'delete'),
#			Menu.Action(_("Sync"), 'sync'),
			Menu.SubMenu(_('Filter'), [
				Menu.Action(_("All songs"), 'filter'),
				Menu.Action(_("This Artist"), 'filter', None, None, (COL_ARTIST,)),
				Menu.Action(_("This Album"), 'filter', None, None, (COL_ALBUM,)),
				Menu.Action(_("This Genre"), 'filter', None, None, (COL_GENRE,)),
#				Menu.Action(_("New Filter..."), 'filter_new')
				]),
			Menu.Separator(),
			Menu.Action(_("Save"), 'save', '', gtk.STOCK_SAVE),
			Menu.Action(_("Open location"), 'show_dir', '', gtk.STOCK_GO_UP),
			Menu.Separator(),
			Menu.Action(_("Close"), 'close', '', gtk.STOCK_CLOSE),
			])
		self.menu.attach(self,self)

		# Playlist
		swin = gtk.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(0)
		swin.set_policy(gtk.POLICY_AUTOMATIC, gtk.POLICY_AUTOMATIC)

		view = gtk.TreeView(self.playlist.get_model())
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)
		self.view.set_reorderable(True)
		self.view.set_search_column(COL_TITLE)

#enable for drag from playlist to other apps (doesn't work yet)
#		self.view.drag_source_set(gtk.gdk.BUTTON_PRESS_MASK, [('text/uri-list', 0, 0)], gtk.gdk.ACTION_COPY)
#		self.view.connect('drag_data_get', self.drag_data_get)

		self.view.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.view.connect('button-press-event', self.button_press)

		#icon showing the current song...
		cell = gtk.CellRendererPixbuf()
		column = gtk.TreeViewColumn('', cell, stock_id=COL_ICON)
		view.append_column(column)
		column.set_resizable(False)
		column.set_reorderable(False)

		for n in range(len(COLUMNS)):
			cell = gtk.CellRendererText()
			column = gtk.TreeViewColumn(COLUMNS[n][0], cell, text = COLUMNS[n][1])
			view.append_column(column)
			column.set_sort_column_id(COLUMNS[n][1])
			column.set_resizable(True)
			column.set_reorderable(True)
			column.set_sizing(gtk.TREE_VIEW_COLUMN_FIXED)
			column.set_fixed_width(COLUMNS[n][3])
			column.connect('clicked', self.col_activate)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)
		self.view.set_search_column(COL_ARTIST)

		#TODO: Multiple Selections
		#self.selection.set_mode(gtk.SELECTION_MULTIPLE)

		# Create layout, pack and show widgets
		self.vbox = gtk.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.show_all()

		self.show()
		self.sync()

	def filter(self, col=None):
		if col:
			try:
				iter = self.playlist.get_model().get_iter((self.curr_index,))
				data = self.playlist.get_model().get_value(iter, col)
			except:
				rox.info(_("Please select a song first."))
				return
		else:
			data = None
		self.playlist.set_filter(col, data)

	def drag_data_get(self, widget, context, selection, targetType, eventTime):
		print >>sys.stderr, selection.target, selection.format, selection.data
		if selection.target == 'text/uri-list':
			selection.set(selection.target, 8, 'test.fil\n')

	def load(self):
		"""Load the playlist either from a saved xml file, or from source dirs"""
		try:
			if self.replace_library:
				self.musicbox.load_songs(self.library)
			else:
				self.musicbox.add_songs(self.library)
		except:
			rox.report_exception()

	def save(self):
		"""Save the current list"""
#		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
#		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'MyMusic.music'), 'application/x-music-playlist')
		file = 'MyMusic.music'
		path = os.path.join(rox.basedir.save_config_path(APP_NAME, APP_DOMAIN), file)
		box = saving.SaveBox(self.playlist, path, 'application/x-music-playlist')
		box.show()

	def sync(self):
		"""Scroll the playlistUI to the currently selected song"""
		try:
			index = self.playlist.get_index()
			self.view.set_cursor((index,))
			self.view.scroll_to_cell((index,))
			self.curr_index = index
		except:
			pass

	def delete(self, *dummy):
		try:
			song = self.playlist.delete(self.curr_index)
		except:
			rox.alert(_("No track selected."))

	def play(self, *dummy):
		"""Play the current song"""
		try:
			self.playlist.set(self.curr_index)
			self.musicbox.play()
		except:
			rox.report_exception()

	def activate(self, view, path, column):
		"""Double-click handler, plays the song"""
		self.play()

	def set_selection(self, selection):
		"""Tell the playlist what we currently have selected"""
		(cursor, thing) = self.view.get_cursor()
		self.curr_index = cursor[0]

	def show_dir(self, *dummy):
		''' Pops up a filer window. '''
		try:
			song = self.playlist.get(self.curr_index)
			filer.show_file(song.filename)
		except:
			rox.alert(_("No track selected."))

	def delete_event(self, ev, e1):
		"""Same as close, but called from the window manager"""
		self.close()

	def close(self, button = None):
		"""Destroy ourselves and all our children"""
		self.destroy()

	def button_press(self, text, event):
		"""Popup menu handler"""
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	def menukey_press(self, widget):
		self.menu.popup(self, None)

	def col_activate(self, column):
		"""Set the selected column as the search <Ctrl-S> column"""
		self.view.set_search_column(column.get_sort_column_id())

	def xds_drag_drop(self, widget, context, data, info, time):
		"""Check if the Shift key is pressed or not when Dropping files"""
		if context.actions & gtk.gdk.ACTION_COPY:
			self.replace_library = False
		else:
			self.replace_library = True
		return loading.XDSLoader.xds_drag_drop(self, widget, context, data, info, time)

	def xds_load_uris(self, uris):
		"""Accept files and folders dropped on us as new Library"""
		path = []
		#strip off the 'file://' part and concatenate them
		for s in uris:
			path.append(rox.get_local_path(s))
		self.library = path
		self.load()

