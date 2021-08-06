import rox
from rox import g, filer, Menu, app_options, i18n
from rox.options import Option
import os, sys, re, string, threading, time
from threading import *
from random import Random
import pympg123
import pyplaylist

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

#Bitmaps that are changed after initialization.
BMP_PAUSE = APP_PATH+'/pixmaps/media-pause.png'
BMP_PLAY = APP_PATH+'/pixmaps/media-play.png'
		
rox.setup_app_options(APP_NAME)

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', os.path.expanduser("~")+'/Music')

#the name of your mp3 player (include full path if necessary)
MP3_PLAYER = Option('mp3_player', '')

SHUFFLE = Option('shuffle', 0)
REPEAT = Option('repeat', 0)

rox.app_options.notify()


class MusicBox(rox.Window):
	def __init__(self):
		rox.Window.__init__(self)
				
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
		
		#create boxes
		self.vbox = g.VBox()
		self.hbox_Row2 = g.HBox(spacing=2)
		self.add(self.vbox)

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
			(_('/Refresh'), 'update', '<StockItem>', '<Ctrl>O', g.STOCK_REFRESH),
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
					None, self.update, None, 0)

		self.toolbar.insert_space(0)

#TODO: Hook this up, or decide not to support it (OSS vs Alsa vs ???)
		self.volume = g.Adjustment(0.5, 0.0, 1.0, 0.1, 0.1, 0.0)
		self.volume.connect("value_changed", self.adjust_volume)
		volume_control = g.HScale(self.volume)
		volume_control.set_draw_value(False)
		volume_control.set_size_request(100, 20)
		self.toolbar.insert_widget(volume_control, _('Volume'), _('Volume'), 0)

		self.toolbar.insert_space(0)

		image_shuffle = g.Image()
		image_shuffle.set_from_file(APP_PATH+"/pixmaps/media-shuffle.png")
		self.shuffle = self.toolbar.insert_element(g.TOOLBAR_CHILD_TOGGLEBUTTON,
					None, _('Shuffle'), _('Shuffle'),'hiya', 
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
		self.vbox.pack_start(self.toolbar, g.FALSE, g.TRUE, 0)
		
		swin = g.ScrolledWindow()
		self.scroll_window = swin

		swin.set_border_width(4)
		swin.set_policy(g.POLICY_ALWAYS, g.POLICY_ALWAYS)
		swin.set_shadow_type(g.SHADOW_IN)

		self.song_list = g.TreeStore(str, str, str, str, str, str)
		view = g.TreeView(self.song_list)
		self.view = view
		swin.add(view)
		view.set_search_column(COL_TITLE)
		view.set_enable_search(True)
		view.set_reorderable(False)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Artist'), cell, text = COL_ARTIST)
		view.append_column(column)
		column.set_sort_column_id(COL_ARTIST)
		column.set_resizable(True)
		column.set_reorderable(True)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Song'), cell, text = COL_TITLE)
		view.append_column(column)
		column.set_sort_column_id(COL_TITLE)
		column.set_resizable(True)
		column.set_reorderable(True)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Album'), cell, text = COL_ALBUM)
		view.append_column(column)
		column.set_sort_column_id(COL_ALBUM)
		column.set_resizable(True)
		column.set_reorderable(True)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Genre'), cell, text = COL_GENRE)
		view.append_column(column)
		column.set_sort_column_id(COL_GENRE)
		column.set_resizable(True)
		column.set_reorderable(True)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Time'), cell, text = COL_LENGTH)
		view.append_column(column)
		column.set_sort_column_id(COL_LENGTH)
		column.set_resizable(True)
		column.set_reorderable(True)

		self.vbox.pack_start(swin, True, True, 0)

		view.connect('row-activated', self.activate)

		self.selection = view.get_selection()
		
		self.rndm = Random(17) # for shuffle


		####################################################################
		# Statusbar
		####################################################################
		self.status_bar = timedStatusbar() 
		self.vbox.pack_end(self.status_bar, expand=g.FALSE, fill=g.TRUE, padding=0)


		####################################################################
		# Finish
		####################################################################
		self.vbox.show_all()

		#init the player instance and start the monitoring thread
#TODO: add Ogg support
		g.threads_init()
		
		if MP3_PLAYER.value == '':
			self.player = None
			rox.edit_options()
		else:
			self.player = pympg123.Player(MP3_PLAYER.value, self.status_update)
			self.foo = Thread(name='test', target=self.player.handle_audio)
			self.foo.start()
		
		self.current_song = ""
		self.current_artist = ""


	####################################################################
	# (re)load the playlist
	####################################################################
	def update(self, button=None):
		self.set_title(APP_NAME+_(' - Scanning, please wait...'))
		g.gdk.flush()

		self.song_list.clear()

		foo = pyplaylist.playlist()
		foo.compile_masterlist(LIBRARY.value)
		foo.set_playlist_as_list()

		foo.first()
		for t in foo.playlist:
			iter = self.song_list.append(None)
			foo.next()
			self.song_list.set(iter, COL_FILE,   foo.current_file(),
						 COL_ARTIST, foo.artist, 
						 COL_TITLE,  foo.songname, 
						 COL_ALBUM,  foo.album,
						 COL_GENRE,  foo.genre,
						 COL_LENGTH, foo.length)

		#select the first song in the list as a starting point
		self.view.set_cursor((0,))
		self.set_title(APP_NAME)

	
	####################################################################
	# double-click hander, plays the song
	####################################################################
	def activate(self, view, path, column):
		self.play()
	
	####################################################################
	# Play button handler (toggle between play and pause)
	####################################################################
	def play_selected(self, button=None):
		if self.player.state == 'play':
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

		if self.player == None:
			self.player = pympg123.Player(MP3_PLAYER.value, self.status_update)
			self.foo = Thread(name='test', target=self.player.handle_audio)
			self.foo.start()

		self.player.play(self.current_file)

	####################################################################
	# skip to previous song and play it
	####################################################################
	def prev(self, button=None):
		path, column = self.view.get_cursor()
		nth = path[0]-1
		path = (nth,)
		self.view.set_cursor(path)
		self.play()
		self.status_bar.output("Prev...",6000)	
		
	####################################################################
	# skip to next song and play it
	####################################################################
	def next(self, button=None):
		if self.shuffle.get_active():
			nth = self.rndm.randrange(0, len(self.song_list))
		else:
			path, column = self.view.get_cursor()
			nth = path[0]
			if nth >= len(self.song_list)-1:
				if self.repeat.get_active():
					nth = 0
			else:
				nth = nth+1
		path = (nth,)
		self.view.set_cursor(path)
		self.play()
		self.status_bar.output("Next...",6000)	
		
	####################################################################
	# stop the current song (but don't kill the player)
	####################################################################
	def stop(self, button=None):
		self.player.stop()
		self.image_play.set_from_file(BMP_PLAY)
		self.status_bar.output("Stopped...",6000)	

	####################################################################
	# pause playing (toggle)
	####################################################################
	def pause(self, button=None):
		self.player.pause()
		self.image_play.set_from_file(BMP_PLAY)
		self.status_bar.output("Paused...",6000)	

	####################################################################
	# status update
	####################################################################
	def status_update(self, state, remain, progress):
		g.threads_enter()

		if state == 'play':
			duration = int(remain + progress)
			
			hour = string.zfill(str(int(progress)/3600),2)
			min = string.zfill(str(int(progress)%3600/60),2)
			sec = string.zfill(str(int(progress)%3600%60),2)
		
			hourremain = string.zfill(str(remain/3600),2)
			minremain = string.zfill(str(remain%3600/60),2)
			secremain = string.zfill(str(remain%3600%60),2)

			try:
				percent = 100*(float(progress)/duration)
				percent = repr(int(percent)) + "%"
			except ZeroDivisionError:
				pass

			if (self.view_state == VIEW_LARGE):
				self.status_bar.output(_('Playing: ')+self.current_song+\
						_(' by ')+self.current_artist+\
						' ('+hourremain+':'+minremain+':'+secremain+')', 0)
			else:
				self.status_bar.output(_('Playing: ')+self.current_song+\
						' ('+hourremain+':'+minremain+':'+secremain+')', 0)
			
		elif state == 'pause':
			self.status_bar.output(_('Paused... '), 0)
		elif state == 'stop':
			self.status_bar.output(_('Stopped... '), 0)
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
		self.player.stop()
		self.player.shutdown()
		self.destroy()
		
	####################################################################
	# used as the notify callback when options change
	####################################################################
	def get_options(self):
		if SHUFFLE.has_changed:
			self.shuffle.set_active(SHUFFLE.int_value)
			
		if REPEAT.has_changed:
			self.repeat.set_active(REPEAT.int_value)
			
		if MP3_PLAYER.has_changed:
			#TODO: how to update only when OK is pressed?
			#rox.info("This only takes effect when you restart MusicBox")
			print "MP3 Player has changed"
			
		if LIBRARY.has_changed:
			#TODO: how to update only when OK is pressed?
			#self.update()
			print "Library has changed"

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
	# set the playback volume
	####################################################################
	def adjust_volume(self, vol):
		print "Volume."+str(vol.get_value())
		
	
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
        self.pop(1)
        self.push(1,msg)
        self.set_timer(timeout)

    def del_timer(self):
        if self.last_tag:
            g.timeout_remove(self.last_tag)
    
    def set_timer(self, timeout):
        if timeout > 0:
            self.last_tag = g.timeout_add(timeout,self.clear)

    def clear(self):
        self.pop(1)
        self.push(1,"")
        return g.FALSE
