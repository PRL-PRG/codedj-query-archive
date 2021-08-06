"""
	playlistui.py
		Playlist UI for MusicBox application.

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

import rox, gobject
from rox import g, Menu, saving, loading, mime

import os, sys, re, threading
from threading import *

import playlist
from playlist import COL_FILE, COL_TITLE, COL_TRACK, COL_ALBUM, COL_ARTIST, COL_GENRE, COL_LENGTH, COL_TYPE


#Who am I and how did I get here?
APP_NAME = "MusicBox"
APP_DIR = rox.app_dir

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

DND_TYPES = ['audio/x-mp3' 'application/ogg' 'inode/directory']

class PlaylistUI(rox.Window, loading.XDSLoader):
	"""the playlist UI for MusicBox"""

	def __init__(self, the_playlist):
		"""Constructor"""
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, DND_TYPES)

		self.playlist = the_playlist  #this is a reference to the main playlist
		self.library = []
		self.replace_library = True

		self.set_title(APP_NAME+' - '+_("Playlist"))
		self.set_border_width(0)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_NONE)

		#capture wm delete event
		self.connect('delete_event', self.delete_event)

		# Menu
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
#			Menu.SubMenu(_('Filter'), [
#				Menu.Action(_("All songs"), 'filter_none', ''),
#				Menu.Action(_("This Artist"), 'filter_artist', ''),
#				Menu.Action(_("This Album"), 'filter_album', ''),
#				Menu.Action(_("This Genre"), 'filter_genre', ''),
#				Menu.Action(_("New Filter..."), 'filter_new', '')
#				]),
#			Menu.Separator(),
			Menu.Action(_("Save"), 'save', '', g.STOCK_SAVE),
			Menu.Separator(),
			Menu.Action(_("Close"), 'close', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)

		# Playlist
		swin = g.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(0)
		swin.set_policy(g.POLICY_AUTOMATIC, g.POLICY_AUTOMATIC)

#		view = g.TreeView(self.playlist.song_list.filter_new())
		view = g.TreeView(self.playlist.song_list)
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)
		self.view.set_reorderable(True)
		self.view.set_search_column(COL_TITLE)

		self.view.drag_source_set(g.gdk.BUTTON_PRESS_MASK, [('text/uri-list', 0, 0)], g.gdk.ACTION_COPY)
		self.view.connect('drag_data_get', self.drag_data_get)

		self.view.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.view.connect('button-press-event', self.button_press)

		#TODO: A little icon showing the current song playing...
		#cell = g.CellRendererPixbuf()
		#column = g.TreeViewColumn('', cell)
		#view.append_column(column)
		#column.set_resizable(False)
		#column.set_reorderable(False)

		for n in range(len(COLUMNS)):
			cell = g.CellRendererText()
			column = g.TreeViewColumn(COLUMNS[n][0], cell, text = COLUMNS[n][1])
			view.append_column(column)
			column.set_sort_column_id(COLUMNS[n][1])
			column.set_resizable(True)
			column.set_reorderable(True)
			column.set_sizing(g.TREE_VIEW_COLUMN_FIXED)
			column.set_fixed_width(COLUMNS[n][3])
			column.connect('clicked', self.col_activate)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)
		self.view.set_search_column(COL_ARTIST)

		#TODO: Multiple Selections
		#self.selection.set_mode(g.SELECTION_MULTIPLE)

		# Create layout, pack and show widgets
		self.vbox = g.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.show_all()

		self.show()
		self.sync()

	def drag_data_get(self, widget, context, selection, targetType, eventTime):
		print >>sys.stderr, selection.target, selection.format, selection.data
		if selection.target == 'text/uri-list':
			selection.set(selection.target, 8, 'test.fil\n')

	def load(self):
		"""Load the playlist either from a saved xml file, or from source dirs"""
		try:
			import xmlrpclib
			client = xmlrpclib.Server("http://localhost:8989", None, False)
			if self.replace_library:
				client.load_songs(self.library)
			else:
				client.add_songs(self.library)
		except:
			rox.report_exception()

	def save(self):
		"""Save the current list"""
		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
		box.show()

	def sync(self):
		"""Scroll the playlistUI to the currently selected song"""
		try:
			index = self.playlist.get_index()
			self.view.set_cursor((index,))
			self.view.scroll_to_cell((index,))
		except:
			pass

	def play(self):
		"""Play the current song"""
		try:
			import xmlrpclib
			client = xmlrpclib.Server("http://localhost:8989", None, False)
			client.play()
		except:
			rox.report_exception()

	def activate(self, view, path, column):
		"""Double-click handler, plays the song"""
		self.playlist.set(path[0])
		self.play()

	def set_selection(self, selection):
		"""Tell the playlist what we currently have selected"""
		#print selection
		pass

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

	def col_activate(self, column):
		"""Set the selected column as the search <Ctrl-S> column"""
		self.view.set_search_column(column.get_sort_column_id())

	def xds_drag_drop(self, widget, context, data, info, time):
		"""Check if the Shift key is pressed or not when Dropping files"""
		if context.actions & g.gdk.ACTION_COPY:
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

