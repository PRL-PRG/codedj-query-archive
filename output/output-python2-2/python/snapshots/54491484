#!/usr/bin/env python
#
# Name: pyplaylist
# Desc: library for managing playlists
# Date: 11/28/2001
# Vers: 2.0
#
# Copyright (C) 2001 Ben Wilson
#  
#
# This library is free software; you #can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
#
#
#	http://mpy3.sourceforge.net
#	Contact: ben@thelocust.org / thelocust@users.sourceforge.net
#
#
# init the thing like this:
# 	foo = pyplaylist.playlist()
#	foo.compile_masterlist("/path/to/mp3s")
#
#	this will find all mp3 files under that directory.  be forewarned,
#	a lot of mp3s will take a while to list, though on my Pentium-100,	
#	2500 MP3s takes about 10 seconds.
#
#	foo.list will be the master list of songs
#	foo.playlist is the list that is used.  if not randomized, then this is a just a copy
#			of foo.list.  If randomized, this is a randomized copy foo.list
#
# methods:
#
#	clear() - clear the list
#	next() - bump up the index one. also, if you attempt to go past the
#			end of the list, it returns to the first spot.
#	prev() - previous index in list. also, if you attempt to go to the song
#		before the beginning of the list, it goes to the end.
#       first() - set the current index to 0.
#	
#	randomize() - randomizes the playlist.  
#	get_current_tag() - you shouldn't need this, as it is all handled internally,
#			though it just gets the current taginfo from the current file
#
#	current_filename() - returns the filename of the MP3 that is the current index.
#	current_index() - returns the current index of the file in the master list
#
#       add_index_to_templist(index) - add this index to the templist
#       add_filename_to_templist(filename) - add this filename, which might be a directory, to the templist.  
#
#       set_templist_as_playlist() - set the templist to the playlist
#
# properties:
#	list 	- array of MP3s (paths)
#       playlist - array of indexes that reference list
#	size	- number of mp3s in LIST
#	songname - uh.. the songname of the current file.  
#	artist 	- uh.. the artist of the current file.
#	album 	- uh.. the album of the current file.
#	comment - uh.. the comment of the current file.



import os
import string
import random
import mp3infor


class playlist:
	debug_on = 0
	list = []
	templist = []
	playlist = []
	index = 0 

	def __init__(self):
		self.clear() 		# initialize the playlist to nil

	def debug(self, text):
		if (self.debug_on == 1):
			print "pyplaylist: " + text + "\r"

	def clear(self):
		self.debug("Clearing Playlist")
		self.list = []		# clear the playlist
		self.playlist = []
		self.index = 0
		self.israndom = 0

	def findfiles(self, list, directory):
		self.debug("Finding Files")
		filenames = os.listdir(directory)

		for filename in filenames:		# recursive search for MP3s

			fullpath = directory + "/" + filename	# assemble full path name from directory and 
								# filename separated by "/"

			if os.path.isfile(fullpath):
				if string. upper(fullpath[-3:]) == "MP3":	# compare the last three letter in the 
										# fullpath string to MP3, and if it is MP3
					list.append(fullpath)   			# then append it to the playlist

			if os.path.isdir(fullpath):		# if the pathname is a directory
				self.findfiles(list,fullpath)	# then call another instance of yourself
											# and descend deeper!
			
	def compile_masterlist(self,path):
		self.findfiles(self.list, path)
#		self.list.sort()


	def set_playlist_as_list(self):
		self.debug("found files --> " +str(len(self.list)))

		for i in range(len(self.list)):
			self.playlist.append(i)

		self.debug("plist files --> " +str(len(self.playlist)))
		self.get_current_tag()	
		


	def current_index(self):
		# return the current index number
		if (len(self.playlist) > self.index):
			return self.playlist[self.index]
		else:
			return -1

	def current_file(self):
		# return the current filename!
		self.debug("lenlist-->" + str(len(self.list)))
		self.debug("index-->" + str(self.index))
		if (len(self.playlist) > self.index):
			return self.list[self.playlist[self.index]]
		else:
			return ""

	def next(self):
		# move to the next song in the playlist

		# if we are random, then move ahead RANDOMLY!
		if (self.israndom == 1):
			self.debug("moving NEXT (randomly)")
			random.seed()
			self.index = random.randint(0,len(self.playlist)-1)
		else:
			self.debug("moving NEXT")
			if (self.index==len(self.playlist)-1):		
				self.index=0
			else:
				self.index = self.index + 1

		# get the current_tag
		self.get_current_tag()

	def prev(self):
		# move to the previous song

		# if we are random, then move previously RANDOM!
		if (self.israndom == 1):
			self.debug("moving PREVIOUS (randomly)")
			random.seed()
			self.index = random.randint(0,len(self.playlist)-1)
		else:
			self.debug("moving PREVIOUS")
			if (self.index == 0):		
				self.index = len(self.playlist) - 1
			else:
				self.index = self.index - 1

		self.get_current_tag()

	def first(self):
		# move to the first song in the list, and get tag
		
		self.index = 0
		self.get_current_tag()

	def randomize(self):
		# set the israndom bit to true or false (it's a toggle!)
		if (self.israndom == 1):				
			self.debug("NON-RANDOM set")
			self.israndom = 0			
		else:
			self.debug("RANDOM set")
			self.israndom = 1		


	def get_current_tag(self):
		# get the tag for the current file

		self.debug("Getting Current Tag for MP3")

		self.songname, self.artist, self.album, self.genre, self.comment, self.length = "","","","","",""

		if (len(self.current_file()) > 0 ):
			mymp3 = mp3infor.open_mp3(self.current_file())	
			try:
				mymp3.read_tag()
#				mymp3.sync_read_header()

				self.songname = ""
				self.artist = ""
				self.album = ""
				self.genre = ""
				self.comment = ""
				self.length = ""

				for i in range(len(mymp3.songname)):
					if mymp3.songname[i] in string.printable:
						self.songname = self.songname + mymp3.songname[i]
				self.songname = self.songname.strip()

				for i in range(len(mymp3.artist)):
					if mymp3.artist[i] in string.printable:
						self.artist = self.artist + mymp3.artist[i]
				self.artist = self.artist.strip()

				for i in range(len(mymp3.album)):
					if mymp3.album[i] in string.printable:
						self.album = self.album + mymp3.album[i]
				self.album = self.album.strip()

#				self.length = string.zfill(int(mymp3.get_length()[1]), 2) + ':' + string.zfill(int(mymp3.get_length()[2]), 2)
					
				for i in range(len(mymp3.get_genre())):
					if mymp3.get_genre()[i] in string.printable:
						self.genre = self.genre + mymp3.get_genre()[i]

				for i in range(len(mymp3.comment)):
					if mymp3.comment[i] in string.printable:
						self.comment = self.comment + mymp3.comment[i]
				self.comment = self.comment.strip()
		


			
			except mp3infor.NoTagError:
				current = self.current_file().split("/")
				filename = current[-1]
				filename = filename.split(".mp3")[0]
				album = current[-2]
				artist = current[-3]
				parts = filename.split("-")
				try:
					filename = parts[1]
				except IndexError:
					filename = parts[0]
				
				self.songname = filename
				self.artist = artist
				self.album = album
				self.genre = "unknown"
				self.length = "unknown"
				self.comment = "Info from file"


	def add_index_to_templist(self, index):
		# attempt to a an index to the temp playlist.  if it is already there, remove it.
		# other wise, add it.
		
		self.debug("adding " + str(index) + " to templist")

		try:
			if self.templist.index(index) > -1 :
				self.templist.remove(index)
				self.debug("add_to_templist: already found in list! - REMOVING")
		except ValueError:
			self.debug("add_index_to_templist: not found in list -- adding")
			self.templist.append(index)
				

	def add_file_to_templist(self,filename):
		# atttempt to add a filename to the temp playlist.
		# you may send directories as the filename, and it will add ALL under that directory.
		#
		# keep in mind that this must search the master "list" for the filename, and then get the
		# index of that file to add to the templist, since playlists are merely arrays of indexes
		
		self.debug("adding " + str(filename) + " to templist")

		
		if os.path.isdir(filename):
			zlist = []
			self.findfiles(zlist, filename)
			for i in range(len(zlist)):
				try:
					x =  self.list.index(zlist[i])
					if (x > -1):
						self.add_index_to_templist(x)
				except ValueError:
					self.debug("add_filename_to_templist: filename--> " + filename + " not in masterlist - not adding")

		else:
			try:
				x =  self.list.index(filename)
				if (x > -1):
					self.add_index_to_templist(x)
			except ValueError:
				self.debug("add_filename_to_templist: filename--> " + filename + " not in masterlist - not adding")
			
		

	def clear_templist(self):
		# kill the templist
		self.templist = []

	def set_templist_as_list(self):
		# uh. put all indexes from "list" into "templist"  used for "tag all"
		for i in range(len(self.list)):
			self.templist.append(i)

	def set_templist_as_playlist(self):
		# after we are done compiling our templist, then use this
		# function to set it as the playlist!
		self.playlist = self.templist
		
