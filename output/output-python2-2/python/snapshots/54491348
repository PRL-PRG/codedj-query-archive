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

import rox
from rox import g, Menu, saving, mime

import os, sys, re, threading
from threading import *

import playlist


#Who am I and how did I get here?
APP_NAME = "MusicBox"
APP_DIR = rox.app_dir

#View options
VIEW_DEFAULT_SIZE = (600, 400)

#Column indicies
COL_FILE = 0
COL_ARTIST = 1
COL_TITLE = 2
COL_ALBUM = 3
COL_TRACK = 4
COL_GENRE = 5
COL_LENGTH = 6
COL_TYPE = 7
COL_INDEX = 8

COLUMNS = [
	(_("Artist"), COL_ARTIST),
	(_("Title"), COL_TITLE),
	(_("Album"), COL_ALBUM),
	(_("Track"), COL_TRACK),
	(_("Genre"), COL_GENRE),
	(_("Length"), COL_LENGTH),
]


class PlaylistUI(rox.Window):
	"""the playlist UI for MusicBox"""

	def __init__(self, the_playlist, callback):
		"""Constructor"""
		rox.Window.__init__(self)

		self.playlist = the_playlist  #this is a reference to the main playlist
		self.callback = callback #to pass back play() commands to the main window

		self.set_title(APP_NAME+' - '+_("Playlist"))
		self.set_border_width(0)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_NONE)

		#capture wm delete event
		self.connect('delete_event', self.delete_event)


		# Menu
		#######################################
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			('/'+_("Filter")+'/'+_("All songs"), 'filter_none', '', '', 0),
			('/'+_("Filter")+'/'+_("This Artist"), 'filter_artist', '', '', 0),
			('/'+_("Filter")+'/'+_("This Album"), 'filter_album', '', '', 0),
			('/'+_("Filter")+'/'+_("This Genre"), 'filter_genre', '', '', 0),
			('/'+_("Filter")+'/'+_("New Filter..."), 'filter_new', '', '', 0),
			('/','','<Separator>','', 0),
			('/'+_("Save"), 'save', '<StockItem>', '', g.STOCK_SAVE),
			('/'+_("Refresh"), 'update_thd', '<StockItem>', '', g.STOCK_REFRESH),
			('/','','<Separator>','', 0),
			('/'+_("Quit"), 'close', '<StockItem>', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


		# Playlist
		#######################################
		swin = g.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(0)
		swin.set_policy(g.POLICY_AUTOMATIC, g.POLICY_AUTOMATIC)

		self.store = g.ListStore(str, str, str, str, str, str, str, str, int)
		view = g.TreeView(self.store)
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)

		self.view.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.view.connect('button-press-event', self.button_press)

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

		self.store.set_sort_func(COL_TRACK, self.comparemethod, COL_TRACK)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)
		self.view.set_search_column(COL_ARTIST)

		#TODO: Multiple Selections
		#self.selection.set_mode(g.SELECTION_MULTIPLE)


		# Create layout, pack and show widgets
		#######################################
		self.vbox = g.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.show_all()

		self.update_thd()
		self.show()


	####################################################################
	def save(self):
		"""Save the current list"""
		box = saving.SaveBox(self.playlist, rox.choices.save(APP_NAME, 'Library.xml'), 'text/xml')
		box.show()


	####################################################################
	def update_thd(self, button=None):
		"""Thread to load songs from source dirs"""
		thd_update = Thread(name='update', target=self.refresh)
		thd_update.setDaemon(False)
		thd_update.start()


	####################################################################
	def set_song_info(self, iter, song, index):
		"""Copy the song info (tags) into the TreeModel iter"""
		if song.filename: self.store.set(iter, COL_FILE, song.filename)
		if song.title: self.store.set(iter, COL_TITLE, song.title)
		if song.track: self.store.set(iter, COL_TRACK, song.track)
		if song.album: self.store.set(iter, COL_ALBUM,  song.album)
		if song.artist: self.store.set(iter, COL_ARTIST, song.artist)
		if song.genre: self.store.set(iter, COL_GENRE,  song.genre)
		if song.length: self.store.set(iter, COL_LENGTH, song.length)
		if song.type: self.store.set(iter, COL_TYPE,  song.type)
		self.store.set(iter, COL_INDEX, index)


	####################################################################
	def refresh(self):
		"""Re-display the playlist (don't get any new info)"""
		g.threads_enter()

		#don't process changed signals while updating
		self.selection.handler_block(self.handler)

		#save where we were, iterating destroys this info
		save_index = self.playlist.get_index()

		self.store.clear()
		self.scroll_window.hide()
		g.threads_leave()

		for song in self.playlist:
			if song != None:
				g.threads_enter()
				iter = self.store.append(None)
				self.set_song_info(iter, song, len(self.store)-1)
				g.threads_leave()

		g.threads_enter()
		self.scroll_window.show()
		self.playlist.set(save_index)
		self.sync()
		self.selection.handler_unblock(self.handler)
		g.threads_leave()


	####################################################################
	def sync(self):
		"""Scroll the playlistUI to the currently selected song"""
		#realizing that the index and the current view's sort order will not
		#always match...
		song = self.playlist.get()
		iter = self.store.get_iter_root()
		index = 0
		while iter != None:
			if self.store.get_value(iter, COL_FILE) == song.filename:
				break
			index +=1
			iter = self.store.iter_next(iter)

		self.view.set_cursor((index,))
		self.view.scroll_to_cell((index,))


	####################################################################
	def play(self):
		"""Play the current song"""
		if self.callback:
			self.callback()


	####################################################################
	def activate(self, view, path, column):
		"""Double-click handler, plays the song"""
		self.play()


	####################################################################
	def set_selection(self, thing):
		"""Tell the playlist what we currently have selected"""
		model, iter = self.view.get_selection().get_selected()
		if iter:
			n = model.get_value(iter, COL_INDEX)
			self.playlist.set(n)


	####################################################################
	def delete_event(self, ev, e1):
		"""Same as close, but called from the window manager"""
		self.close()


	####################################################################
	def close(self, button = None):
		"""Destroy ourselves and all our children"""
		self.destroy()


	####################################################################
	def button_press(self, text, event):
		"""Popup menu handler"""
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1


	####################################################################
	def col_activate(self, column):
		"""Set the selected column as the search <Ctrl-S> column"""
		self.view.set_search_column(column.get_sort_column_id())


	####################################################################
	def comparemethod(self, model, iter1, iter2, user_data):
		"""Method to sort by Track and others"""
		try:
			if user_data == COL_TRACK:
				item1 = model.get_value(iter1, COL_ALBUM)+model.get_value(iter1, COL_TRACK)
				item2 = model.get_value(iter2, COL_ALBUM)+model.get_value(iter2, COL_TRACK)
				#print item1, item2

			if item1 < item2:
				return -1
			elif item1 > item2:
				return 1
			else:
				return 0
		except:
			return 0


	####################################################################
	def filter_none(self):
		"""Clear any filter and show all songs"""
		self.playlist.the_filter = {}
		self.update_thd()


	####################################################################
	def filter_album(self):
		"""Filter by the currently selected album"""
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			album = model.get_value(iter, COL_ALBUM)
			self.playlist.the_filter = {'album':[album]}
			self.update_thd()


	####################################################################
	def filter_artist(self):
		"""Filter by the currently selected artist"""
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			artist = model.get_value(iter, COL_ARTIST)
			self.playlist.the_filter = {'artist':[artist]}
			self.update_thd()


	####################################################################
	def filter_genre(self):
		"""Filter by the currently selected genre"""
		model, iter = self.selection.get_selected()
		if not model or not iter:
			rox.info(_('Please make a selection first.'))
		else:
			genre = model.get_value(iter, COL_GENRE)
			self.playlist.the_filter = {'genre':[genre]}
			self.update_thd()


	####################################################################
	def filter_new(self):
		"""Create a new filter via a dialog"""
		rox.info("Not implemented yet")
		pass

