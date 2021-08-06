import rox
from rox import g, filer, Menu, app_options, i18n, loading
from rox.options import Option
import os, sys, re, string, threading, time, stat
from threading import *
from random import Random
import player
from ID3 import *

try:
	import ogg.vorbis
	HAVE_OGG = True
except:
	HAVE_OGG = False
	print 'No OGG support!'

try:
	import mad
	HAVE_MAD = True
except:
	HAVE_MAD = False
	print 'No MP3 support!'

if not HAVE_MAD and not HAVE_OGG:
	raise ValueError, "You must have at least one of the Ogg Vorbis or MAD libraries installed."

_ = rox.i18n.translation(os.path.join(rox.app_dir, 'Messages'))

#Who am I and how did I get here?
APP_NAME = 'MusicBox'
APP_PATH = os.path.split(os.path.abspath(sys.argv[0]))[0]


#View options
VIEW_DEFAULT_SIZE = (600, 400)
VIEW_MINI_SIZE = (430, 56) #trial and error here.  Is there a way to get this programmatically?
VIEW_MINI = 0
VIEW_LARGE = 1
VIEW_LARGE_SIZE = (500, 200)

#Column indicies
COL_FILE = 0
COL_ARTIST = 1
COL_TITLE = 2
COL_ALBUM = 3
COL_GENRE = 4
COL_LENGTH = 5

COLUMNS = [
	(_('Artist'), COL_ARTIST),
	(_('Title'), COL_TITLE),
	(_('Album'), COL_ALBUM),
	(_('Genre'), COL_GENRE),
#	(_('Length'), COL_LENGTH),
]


#Bitmaps that are changed after initialization.
BMP_PAUSE = APP_PATH+'/pixmaps/media-pause.png'
BMP_PLAY = APP_PATH+'/pixmaps/media-play.png'

rox.setup_app_options(APP_NAME)

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', os.path.expanduser("~")+'/Music')

#the ao driver type you want to use (esd, oss, alsa, alsa09, ...)
DRIVER_ID = Option('driver_id', 'esd')

SHUFFLE = Option('shuffle', 0)
REPEAT = Option('repeat', 0)

#Don't replay any of the last n songs in shuffle mode
SHUFFLE_CACHE_SIZE = Option('shuffle_cache', 10)


rox.app_options.notify()

DND_TYPES = ['audio/mp3' 'audio/ogg' 'application/x-ogg' 'inode/directory']


class MusicBox(rox.Window, loading.XDSLoader):
	def __init__(self):
		rox.Window.__init__(self)
		loading.XDSLoader.__init__(self, DND_TYPES)

		self.set_title(APP_NAME)
		self.set_border_width(1)
		self.set_default_size(VIEW_DEFAULT_SIZE[0], VIEW_DEFAULT_SIZE[1])
		self.set_position(g.WIN_POS_CENTER)

		#update things when options change
		rox.app_options.add_notify(self.get_options)

		#capture wm delete event
		self.connect("delete_event", self.delete_event)

		#start in normal view mode (so you can see the songs!)
		self.view_state = VIEW_LARGE

		self.replace_library = True

		self.shuffle_cache = []


		####################################################################
		# Menu
		####################################################################
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			(_('/Play\/Pause'), 'play_selected', '', '<Ctrl>P', 0),
			(_('/Stop'), 'stop', '', '<Ctrl>S', 0),
			('/','','<Separator>','', 0),
			(_('/Back'), 'prev', '', '<Ctrl>B', 0),
			(_('/Next'), 'next', '', '<Ctrl>N', 0),
			('/','','<Separator>','', 0),
			(_('/Toggle View'),  'toggle_view', '', '<Ctrl>T', 0),
			(_('/Options'), 'show_options', '<StockItem>', '<Ctrl>O', g.STOCK_PREFERENCES),
			(_('/Refresh'), 'update_thd', '<StockItem>', '<Ctrl>O', g.STOCK_REFRESH),
			('/','','<Separator>','', 0),
			(_('/Quit'), 'close', '<StockItem>', '<Ctrl>Q', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


		####################################################################
		# Toolbar
		####################################################################

		self.toolbar = g.Toolbar()
		self.toolbar.set_style(g.TOOLBAR_ICONS)

		self.toolbar.insert_stock(g.STOCK_PREFERENCES, _('Options'),
					None, self.show_options, None, 0)
		self.toolbar.insert_stock(g.STOCK_REFRESH, _('Refresh'),
					None, self.update_thd, None, 0)

		self.toolbar.insert_space(0)

		self.we_did_it = False
		self.seek_bar = g.Adjustment(0.5, 0.0, 1.0, 0.1, 0.1, 0.0)
		self.seek_bar.connect("value_changed", self.adjust_seek_bar)
		seek_bar_control = g.HScale(self.seek_bar)
		seek_bar_control.set_draw_value(False)
		seek_bar_control.set_size_request(100, 20)
		#seek_bar_control.set_update_policy(g.UPDATE_DISCONTINUOUS)
		self.toolbar.insert_widget(seek_bar_control, _('Seek'), _('Seek'), 0)

		self.toolbar.insert_space(0)

		image_shuffle = g.Image()
		image_shuffle.set_from_file(APP_PATH+"/pixmaps/media-shuffle.png")
		self.shuffle = self.toolbar.insert_element(g.TOOLBAR_CHILD_TOGGLEBUTTON,
					None, _('Shuffle'), _('Shuffle'),None,
					image_shuffle, None, None, 0)
		self.shuffle.set_active(SHUFFLE.int_value)

		image_repeat = g.Image()
		image_repeat.set_from_file(APP_PATH+"/pixmaps/media-repeat.png")
		self.repeat = self.toolbar.insert_element(g.TOOLBAR_CHILD_TOGGLEBUTTON,
					None, _('Repeat'), _('Repeat'), None,
					image_repeat, None, None, 0)
		self.repeat.set_active(REPEAT.int_value)

		self.toolbar.insert_space(0)

		image_next = g.Image()
		image_next.set_from_file(APP_PATH+"/pixmaps/media-next.png")
		self.toolbar.insert_item(_('Next'), _('Next'),
					None, image_next, self.next, None, 0)

		image_stop = g.Image()
		image_stop.set_from_file(APP_PATH+"/pixmaps/media-stop.png")
		self.toolbar.insert_item(_('Stop'), _('Stop'),
					None, image_stop, self.stop, None, 0, )

		image_play = g.Image()
		self.image_play = image_play
		image_play.set_from_file(BMP_PLAY)
		self.toolbar.insert_item(_('Play/Pause'), _('Play/Pause'),
					None, image_play, self.play_selected, None, 0)

		image_prev = g.Image()
		image_prev.set_from_file(APP_PATH+"/pixmaps/media-prev.png")
		self.toolbar.insert_item(_('Prev'), _('Prev'),
					None, image_prev, self.prev, None, 0)

		self.toolbar.insert_space(0)
		self.toolbar.insert_stock(g.STOCK_CLOSE, _('Close'),
					None, self.close, None, 0)


		####################################################################
		# Song Playlist
		####################################################################

		swin = g.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(4)
		swin.set_policy(g.POLICY_ALWAYS, g.POLICY_ALWAYS)
		swin.set_shadow_type(g.SHADOW_IN)

		self.song_list = g.TreeStore(str, str, str, str, str, str)
		view = g.TreeView(self.song_list)
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)

#TODO: Drag and Drop from MusicBox...
#		view.drag_source_set(g.gdk.BUTTON1_MASK,
#			[('XdndDirectSave0', 0, 0),
#			('application/octet-stream', 0, 1)],
#			g.gdk.ACTION_COPY)
#		view.connect('drag_data_get', self.on_drag_data_get)

#TODO: A little icon showing the current song playing...
#		cell = g.CellRendererPixbuf()
#		column = g.TreeViewColumn('', cell)
#		view.append_column(column)
#		column.set_resizable(False)
#		column.set_reorderable(False)

		for n in range(len(COLUMNS)):
			cell = g.CellRendererText()
			column = g.TreeViewColumn(COLUMNS[n][0], cell, text = COLUMNS[n][1])
			view.append_column(column)
			column.set_sort_column_id(COLUMNS[n][1])
			column.set_resizable(True)
			column.set_reorderable(True)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()

#TODO: Multiple Selections
#		self.selection.set_mode(g.SELECTION_MULTIPLE)

		####################################################################
		# Statusbar
		####################################################################
		self.status_bar = timedStatusbar()


		####################################################################
		# Create layout, pack and show widgets
		####################################################################
		self.vbox = g.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.toolbar, False, True, 0)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.pack_end(self.status_bar, False, True, 0)
		self.vbox.show_all()

		self.show()
		self.update_thd()

		self.player = None
		self.current_song = ""
		self.current_artist = ""
		self.rndm = Random(time.time()) # for shuffle


#TODO: Drag and Drop from MusicBox...
	####################################################################
	# Tell the drop target what the filename(s) is(are)
	####################################################################
	def on_drag_data_get(self, treeview, context, selection, info, time):
		#print treeview, context, selection, info

		model, iter = self.selection.get_selected()
		self.current_file = model.get_value(iter, COL_FILE)
		payload = model.get_value(iter, COL_FILE)

		print 'Source: %s' % payload

		selection.set(selection.target, 8, payload)
		model.remove(iter)


	####################################################################
	# run a background thread to update the tag info after loading
	####################################################################
	def update_thd(self, button=None):
		thd_update = Thread(name='update', target=self.update)
		thd_update.start()

	####################################################################
	# Get all the tag info from the songs in the list
	####################################################################
	def get_tag_info(self):
		g.threads_enter()
		model = self.view.get_model()
		iter = model.get_iter_root()
		g.threads_leave()

		while iter:
			g.threads_enter()
			artist = album = title = genre = ''
			filename = model.get_value(iter, COL_FILE)
			(root, ext) = os.path.splitext(filename)
			found = False
			if (ext == '.mp3' and HAVE_MAD):
				song = ID3(filename)
				try:
					artist = song['ARTIST']
					album = song['ALBUM']
					title = song['TITLE']
					genre = song['GENRE']
					found = True
				except:
					pass

			elif (ext == '.ogg' and HAVE_OGG):
				song = ogg.vorbis.VorbisFile(filename).comment().as_dict()
				try:
					artist = song['ARTIST'][0]
					album = song['ALBUM'][0]
					title = song['TITLE'][0]
					genre = song['GENRE'][0]
					found = True
				except:
					pass
			if found:
				self.song_list.set(iter,
					COL_FILE,   filename,
					COL_ARTIST, artist,
					COL_TITLE,  title,
					COL_ALBUM,  album,
					COL_GENRE,  genre,
					)
			iter = model.iter_next(iter)
			g.threads_leave()
			time.sleep(0.1) #this is a background process, be nice


	####################################################################
	# (re)load the playlist (no tags!)
	####################################################################
	def update(self, button=None):
		g.threads_enter()
		self.set_title(APP_NAME+_(' - Scanning, please wait...'))
		self.song_list.clear()
		g.threads_leave()

		#LIBRARY can be a ':' separated path
		library_path = LIBRARY.value.split(":")

		def add_song(filename):
			(root, ext) = os.path.splitext(filename)
			ext.lower()
			artist = album = title = genre = ''
			(artist, album, title, genre) = guess(filename)
			found = False
			if ext == '.mp3' or ext == '.ogg':
				g.threads_enter()
				iter = self.song_list.append(None)
				self.song_list.set(iter,
						COL_FILE,   filename,
						COL_ARTIST, artist,
						COL_TITLE,  title,
						COL_ALBUM,  album,
						COL_GENRE,  genre,
						)
				g.threads_leave()


		def process_pls(pls_file):
			pls = open(pls_file, 'r')
			if (pls):
				for line in pls.xreadlines():
					filename = re.match('^File[0-9]+=(.*)', line)
					if filename:
						add_song(filename.group(1))

		def process_m3u(m3u_file):
			m3u = open(m3u_file, 'r')
			if (m3u):
				for line in m3u.xreadlines():
					filename = re.match('(^/.*)', line)
					if filename:
						add_song(filename.group(1))

		def guess(filename):
			path = filename.split("/")
			title = path[-1]
			(title, ext) = os.path.splitext(title)
			album = path[-2]
			artist = path[-3]
			parts = title.split("-")
			try:
				title = parts[1]
			except IndexError:
				title = parts[0]
			genre = _('unknown')
			comment = ""
			return (artist, album, title, genre)

		def visit(self, dirname, names):
			for filename in names:
				add_song(dirname+'/'+filename)

		for library_element in library_path:
			if os.access(library_element, os.R_OK):
				#check if the element is a folder
				if stat.S_ISDIR(os.stat(library_element)[stat.ST_MODE]):
					os.path.walk(library_element, visit, self)
				else:
					#check for playlist files...
					(root, ext) = os.path.splitext(library_element)
					if ext == '.pls':
						process_pls(library_element)
					elif ext == '.m3u':
						process_m3u(library_element)

					else:
						#assume the element is a song file...
						visit(self, '', (library_element,))

		#select the first song in the list as a starting point
		g.threads_enter()
		self.view.set_cursor((0,))
		self.set_title(APP_NAME)
		self.status_bar.output(str(len(self.song_list))+' songs.', 0)
		g.threads_leave()

		self.get_tag_info()

	####################################################################
	# double-click handler, plays the song
	####################################################################
	def activate(self, view, path, column):
		self.play()

	####################################################################
	# Play button handler (toggle between play and pause)
	####################################################################
	def play_selected(self, button=None):
		if (self.player) and ((self.player.state == 'play') or
			(self.player.state == 'pause')):
			self.pause()
		else:
			self.play()

	####################################################################
	# play the current song
	####################################################################
	def play(self):
		model, iter = self.selection.get_selected()
		self.current_file = model.get_value(iter, COL_FILE)
		self.current_artist = model.get_value(iter, COL_ARTIST)
		self.current_song = model.get_value(iter, COL_TITLE)
		self.image_play.set_from_file(BMP_PAUSE)

		if self.player and self.player.state != 'stop':
			self.player.stop()

		self.player = None
		self.foo = None
		self.player = player.AOPlayer(self.current_file, self.status_update, DRIVER_ID.value)
		self.foo = Thread(name='player', target=self.player.play)
		self.foo.start()

	####################################################################
	# skip to previous song and play it
	####################################################################
	def prev(self, button=None):
		path, column = self.view.get_cursor()
		n = max(0, path[0]-1)
		path = (n,)
		self.view.set_cursor(path)
		self.play()
		self.status_bar.output(_('Prev'),6000)

	####################################################################
	# skip to next song and play it
	####################################################################
	def next(self, button=None):
		num_songs = len(self.song_list)
		if self.shuffle.get_active():
			while True:
				n = self.rndm.randrange(0, num_songs)
				if SHUFFLE_CACHE_SIZE.value >= num_songs:
					break
				if n not in self.shuffle_cache:
					self.shuffle_cache.append(n)
					if len(self.shuffle_cache) > SHUFFLE_CACHE_SIZE.value:
						self.shuffle_cache.pop(0)
					break
		else:
			path, column = self.view.get_cursor()
			n = path[0]
			if n >= num_songs-1:
				if self.repeat.get_active():
					n = 0
			else:
				n = n+1
		path = (n,)
		self.view.set_cursor(path)
		self.play()
		self.status_bar.output(_('Next'),6000)

	####################################################################
	# stop the current song (but don't kill the player)
	####################################################################
	def stop(self, button=None):
		if (self.player) and (self.player.state != 'stop'):
			self.player.stop()
			self.player = None
			self.foo = None
		self.image_play.set_from_file(BMP_PLAY)
		self.status_bar.output(_('Stop'),6000)

	####################################################################
	# pause playing (toggle)
	####################################################################
	def pause(self, button=None):
		self.player.pause()
		self.image_play.set_from_file(BMP_PLAY)
		self.status_bar.output(_('Pause'),6000)

	####################################################################
	# status update
	####################################################################
	def status_update(self, state, remain, progress):
		g.threads_enter()

		if state == 'play':
			duration = int(remain + progress)

#			hour = string.zfill(str(int(progress)/3600),2)
			min = string.zfill(str(int(progress)%3600/60),2)
			sec = string.zfill(str(int(progress)%3600%60),2)

#			hourremain = string.zfill(str(remain/3600),2)
			minremain = string.zfill(str(remain%3600/60),2)
			secremain = string.zfill(str(remain%3600%60),2)

			try:
				percent = 100*(float(progress)/duration)
				percent = repr(int(percent)) + "%"
			except ZeroDivisionError:
				pass

			if (self.view_state == VIEW_LARGE):
#				self.status_bar.output(_('Playing: ')+self.current_song+\
#						_(' by ')+self.current_artist+\
#						' ('+minremain+':'+secremain+')', 0)
				self.set_title(APP_NAME+' - '+self.current_song+\
						_(' by ')+self.current_artist+\
						' ('+minremain+':'+secremain+')')
			else:
#				self.status_bar.output(_('Playing: ')+self.current_song+\
#						' ('+minremain+':'+secremain+')', 0)
				self.set_title(APP_NAME+' - '+self.current_song+\
						' ('+minremain+':'+secremain+')')

			self.we_did_it = True
			self.seek_bar.set_value(float(progress)/duration)


		elif state == 'pause':
			self.status_bar.output(_('Pause'), 0)
		elif state == 'stop':
			self.status_bar.output(_('Stop'), 0)
		elif state == 'eof':
			self.next()

		g.threads_leave()


	####################################################################
	# same as close, but called from the window manager
	####################################################################
	def delete_event(self, ev, e1):
		self.close()

	####################################################################
	# stop playing, kill the player and exit
	####################################################################
	def close(self, button = None):
		if (self.player):
			self.stop()
		self.destroy()

	####################################################################
	# used as the notify callback when options change
	####################################################################
	def get_options(self):
		if SHUFFLE.has_changed:
			self.shuffle.set_active(SHUFFLE.int_value)

		if REPEAT.has_changed:
			self.repeat.set_active(REPEAT.int_value)

		if LIBRARY.has_changed:
			pass
			#TODO: how to update only when OK is pressed?
			#self.update()
			#print "Library has changed"

	####################################################################
	# options edit dialog
	####################################################################
	def show_options(self, button=None):
		rox.edit_options()

	####################################################################
	# popup menu handler
	####################################################################
	def button_press(self, text, event):
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	####################################################################
	# set the playback position (seek)
	####################################################################
	def adjust_seek_bar(self, vol):
		if self.we_did_it:
			#ignore updates caused by playback progress
			self.we_did_it = False
		else:
			#process those caused by dragging the slider
			if self.player:
				self.player.seek(vol.get_value())

	####################################################################
	# toggle between normal size and only toolbar+statusbar
	####################################################################
	def toggle_view(self):
		if self.view_state == VIEW_LARGE:
			self.scroll_window.hide()
			self.set_size_request(VIEW_MINI_SIZE[0], VIEW_MINI_SIZE[1])
			(self.old_width, self.old_height) = self.get_size()
			self.resize(VIEW_MINI_SIZE[0], VIEW_MINI_SIZE[1])
			self.set_resizable(False)
			self.view_state = VIEW_MINI
		else:
			self.scroll_window.show()
			self.set_size_request(VIEW_LARGE_SIZE[0], VIEW_LARGE_SIZE[1])
			self.resize(self.old_width, self.old_height)
			self.view_state = VIEW_LARGE
			self.set_resizable(True)


	####################################################################
	# Check if the Shift key is pressed or not when Dropping files
	####################################################################
	def xds_drag_drop(self, widget, context, data, info, time):
		if context.actions & g.gdk.ACTION_MOVE:
			pass
		if context.actions & g.gdk.ACTION_COPY:
			self.replace_library = True
		else:
			self.replace_library = False
		return loading.XDSLoader.xds_drag_drop(self, widget, context, data, info, time)


	####################################################################
	# Accept files and folders dropped on us as new Library
	####################################################################
	def xds_load_uris(self, uris):
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

		rox.app_options.save()
		self.update_thd()


####################################################################
# utlity class(es)
#	copied from Songer
####################################################################
class timedStatusbar(g.Statusbar):
	_o = None
	def __init__(self):
		g.Statusbar.__init__(self)
		self._initialize()

	def _initialize(self):
		self.last_tag = None

	def output(self, msg,timeout):
		self.del_timer()
		if timeout == 0:
			self.pop(1)
			self.push(1,msg)
			self.set_timer(timeout)

	def del_timer(self):
		if self.last_tag:
			g.timeout_remove(self.last_tag)
			self.pop(1)

	def set_timer(self, timeout):
		if timeout > 0:
			self.last_tag = g.timeout_add(timeout,self.clear)

	def clear(self):
		self.pop(1)
		self.last_tag = None
		return False


####################################################################
# find the location of an executable on your path
# How come this isn't part of the standard library?
####################################################################
def which(filename):
	if (filename == None) or (filename == ''):
		return None

	env_path = os.getenv('PATH').split(':')
	for p in env_path:
		if os.access(p+'/'+filename, os.X_OK):
			return p+'/'+filename
	return None
