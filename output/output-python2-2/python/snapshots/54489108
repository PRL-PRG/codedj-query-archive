"""
	ripper.py
		GUI front-end to cdda2wav and lame.

	Copyright 2004 Kenneth Hayber <khayber@socal.rr.com>
		All rights reserved.

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License.

	This program is distributed in the hope that it will be useful
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program; if not, write to the Free Software
	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
"""

import os, sys, signal, re, string, socket, time, popen2, threading, Queue
from random import Random
from threading import *

import rox
from rox import g, i18n, app_options, Menu
from rox.options import Option

import PyCDDB, cd_logic, CDROM, genres

try:
	import xattr
	HAVE_XATTR = True
except:
	HAVE_XATTR = False

_ = rox.i18n.translation(os.path.join(rox.app_dir, 'Messages'))

#Who am I and how did I get here?
APP_NAME = 'Ripper'  #I could call it Mr. Giles, but that would be gay.
APP_PATH = os.path.split(os.path.abspath(sys.argv[0]))[0]


#Options.xml processing
rox.setup_app_options(APP_NAME)

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', os.path.expanduser('~')+'/MyMusic')

#RIPPER options
RIPPER = Option('ripper', 'cdda2wav')
RIPPER_DEV = Option('ripper_dev', '/dev/cdrom')
RIPPER_LUN = Option('ripper_lun', 'ATAPI:0,1,0')
RIPPER_OPTS = Option('ripper_opts', '-x -H')

EJECT_AFTER_RIP = Option('eject_after_rip', '0')

#ENCODER options
ENCODER = Option('encoder', 'MP3')

MP3_ENCODER = Option('mp3_encoder', 'lame')
MP3_ENCODER_OPTS = Option('mp3_encoder_opts', '--vbr-new -b160 --nohist --add-id3v2')

OGG_ENCODER = Option('ogg_encoder', 'oggenc')
OGG_ENCODER_OPTS = Option('ogg_encoder_opts', '-q5')

#CDDB Server and Options
CDDB_SERVER = Option('cddb_server', 'http://freedb.freedb.org/~cddb/cddb.cgi')

rox.app_options.notify()


#Column indicies
COL_ENABLE = 0
COL_TRACK = 1
COL_TIME = 2
COL_STATUS = 3


# My gentoo python doesn't have universal line ending support compiled in
# and these guys (cdda2wav and lame) use CR line endings to pretty-up their output.
def myreadline(file):
	'''Return a line of input using \r or \n as terminators'''
	line = ''
	while '\n' not in line and '\r' not in line:
		char = file.read(1)
		if char == '': return line
		line += char
	return line


def which(filename):
	'''Return the full path of an executable if found on the path'''
	if (filename == None) or (filename == ''):
		return None

	env_path = os.getenv('PATH').split(':')
	for p in env_path:
		if os.access(p+'/'+filename, os.X_OK):
			return p+'/'+filename
	return None


def strip_illegal(instr):
	'''remove illegal (filename) characters from string'''
	str = instr
	str = str.strip()
	str = string.translate(str, string.maketrans(r'/+{}*.?', r'--()___'))
	return str


class Ripper(rox.Window):
	'''Rip and Encode a CD'''
	def __init__(self):
		rox.Window.__init__(self)

		self.set_title(APP_NAME)
		self.set_default_size(450, 500)
		self.set_position(g.WIN_POS_MOUSE)

		#capture wm delete event
		self.connect("delete_event", self.delete_event)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)


		#song list
		#######################################
		swin = g.ScrolledWindow()
		self.scroll_window = swin
		swin.set_policy(g.POLICY_AUTOMATIC, g.POLICY_AUTOMATIC)
		swin.set_shadow_type(g.SHADOW_IN)

		self.store = g.ListStore(int, str, str, str)
		view = g.TreeView(self.store)
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)

		cell = g.CellRendererToggle()
		cell.connect('toggled', self.toggle_check)
		column = g.TreeViewColumn('', cell, active=COL_ENABLE)
		view.append_column(column)
		column.set_resizable(False)
		column.set_reorderable(False)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Track'), cell, text = COL_TRACK)
		view.append_column(column)
		column.set_resizable(True)
		#column.set_sizing(g.TREE_VIEW_COLUMN_AUTOSIZE)
		column.set_reorderable(False)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Time'), cell, text = COL_TIME)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		cell = g.CellRendererText()
		column = g.TreeViewColumn(_('Status'), cell, text = COL_STATUS)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)


		self.toolbar = g.Toolbar()
		self.toolbar.set_style(g.TOOLBAR_ICONS)
		self.toolbar.insert_stock(g.STOCK_PREFERENCES,
							_('Settings'), None, self.show_options, None, 0)
		self.stop_btn = self.toolbar.insert_stock(g.STOCK_STOP,
							_('Stop'), None, self.stop, None, 0)
		self.rip_btn = self.toolbar.insert_stock(g.STOCK_EXECUTE,
							_('Rip & Encode'), None, self.rip_n_encode, None, 0)
		self.refresh_btn = self.toolbar.insert_stock(g.STOCK_REFRESH,
							_('Reload CD'), None, self.do_get_tracks, None, 0)


		self.table = g.Table(5, 2, False)
		x_pad = 2
		y_pad = 1

		self.artist_entry = g.Entry(max=255)
		self.artist_entry.connect('changed', self.stuff_changed)
		self.table.attach(g.Label(str=_('Artist')), 0, 1, 2, 3, 0, 0, 4, y_pad)
		self.table.attach(self.artist_entry, 1, 2, 2, 3, g.EXPAND|g.FILL, 0, x_pad, y_pad)

		self.album_entry = g.Entry(max=255)
		self.album_entry.connect('changed', self.stuff_changed)
		self.table.attach(g.Label(str=_('Album')),	0, 1, 3, 4, 0, 0, 4, y_pad)
		self.table.attach(self.album_entry,	1, 2, 3, 4, g.EXPAND|g.FILL, 0, x_pad, y_pad)

		genres.genre_list.sort()
		self.genre_combo = g.Combo()
		self.genre_combo.set_popdown_strings(genres.genre_list)
		self.genre_combo.entry.connect('changed', self.stuff_changed)
		self.table.attach(g.Label(str=_('Genre')),	0, 1, 4, 5, 0, 0, 4, y_pad)
		self.table.attach(self.genre_combo,	1, 2, 4, 5, g.EXPAND|g.FILL, 0, x_pad, y_pad)

		self.year_entry = g.Entry(max=4)
		self.year_entry.connect('changed', self.stuff_changed)
		self.table.attach(g.Label(str=_('Year')),	0, 1, 5, 6, 0, 0, 4, y_pad)
		self.table.attach(self.year_entry,	1, 2, 5, 6, g.EXPAND|g.FILL, 0, x_pad, y_pad)


		# Create layout, pack and show widgets
		self.vbox = g.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.toolbar, False, True, 0)
		self.vbox.pack_start(self.table, False, True, 0)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.show_all()

		# Menu
		self.add_events(g.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		view.add_events(g.gdk.BUTTON_PRESS_MASK)
		view.connect('button-press-event', self.button_press)

		Menu.set_save_name(APP_NAME)
		self.menu = Menu.Menu('main', [
			Menu.Action(_('Rip & Encode'), 'rip_n_encode', '', g.STOCK_EXECUTE),
			Menu.Action(_('Reload CD'), 'do_get_tracks', '', g.STOCK_REFRESH),
			Menu.Action(_('Stop'), 'stop', '', g.STOCK_STOP),
			Menu.Separator(),
			Menu.Action(_('Settings'), 'show_options', '', g.STOCK_PREFERENCES),
			Menu.Action(_("Quit"), 'close', '', g.STOCK_CLOSE),
			])
		self.menu.attach(self,self)

		# Defaults and Misc
		self.cddb_thd = None
		self.ripper_thd = None
		self.encoder_thd = None
		self.is_ripping = False
		self.is_encoding = False
		self.is_cddbing = False
		self.stop_request = False

		cd_logic.set_dev(RIPPER_DEV.value)
		self.cd_status = cd_logic.check_dev()
		self.disc_id = None
		self.cd_status_changed = False

		if self.cd_status in [CDROM.CDS_TRAY_OPEN, CDROM.CDS_NO_DISC]:
			self.no_disc()
		else:
			self.do_get_tracks()
		g.timeout_add(1000, self.update_gui)


	def update_gui(self):
		'''Update button status based on current state'''
		cd_status = cd_logic.check_dev()
		if self.cd_status != cd_status:
			self.cd_status = cd_status
			self.cd_status_changed = True

		if self.is_ripping or self.is_encoding:
			self.stop_btn.set_sensitive(True)
			self.rip_btn.set_sensitive(False)
			self.refresh_btn.set_sensitive(False)

		if not self.is_ripping and not self.is_encoding and not self.is_cddbing:
			self.stop_btn.set_sensitive(False)
			self.rip_btn.set_sensitive(True)
			self.refresh_btn.set_sensitive(True)

			#get tracks if cd changed and not doing other things
			if self.cd_status_changed:
				if self.cd_status in [CDROM.CDS_TRAY_OPEN, CDROM.CDS_NO_DISC]:
					self.no_disc()
					self.disc_id = None
				else:
					disc_id = cd_logic.get_disc_id()
					if self.disc_id <> disc_id:
						self.disc_id = disc_id
						self.do_get_tracks()
				self.cd_status_changed = False

		#need this to keep the timer running(?)
		return True


	def stuff_changed(self, button=None):
		'''Get new text from edit boxes and save it'''
		self.genre = self.genre_combo.entry.get_text()
		self.artist = self.artist_entry.get_text()
		self.album = self.album_entry.get_text()
		self.year = self.year_entry.get_text()


	def runit(self, it):
		'''Run a function in a thread'''
		thd_it = Thread(name='mythread', target=it)
		thd_it.setDaemon(True)
		thd_it.start()
		return thd_it


	def stop(self, it):
		'''Stop current rip/encode process'''
		self.stop_request = True


	def do_get_tracks(self, button=None):
		'''Get the track info (cddb and cd) in a thread'''
		if self.is_ripping:
			return
		self.cddb_thd = self.runit(self.get_tracks)


	def no_disc(self):
		'''Clear all info and display <no disc>'''
		#print "no disc in tray?"
		g.threads_enter()
		self.store.clear()
		self.artist_entry.set_text(_('<no disc>'))
		self.album_entry.set_text('')
		self.genre_combo.entry.set_text('')
		self.year_entry.set_text('')
		self.view.columns_autosize()
		g.threads_leave()


	def get_tracks(self):
		'''Get the track info (cddb and cd)'''
		self.is_cddbing = True
		stuff = self.get_cddb()
		g.threads_enter()
		(count, artist, album, genre, year, tracklist) = stuff
		#print count, artist, album, genre, year, tracklist

		self.artist = artist
		self.count = count
		self.album = album
		self.genre = genre
		self.year = year
		self.tracklist = tracklist

		if artist: self.artist_entry.set_text(artist)
		if album: self.album_entry.set_text(album)
		if genre: self.genre_combo.entry.set_text(genre)
		if year: self.year_entry.set_text(year)

		self.store.clear()
		for track in tracklist:
			#print song
			iter = self.store.append(None)
			self.store.set(iter, COL_TRACK, track[0])
			self.store.set(iter, COL_TIME, track[1])
			self.store.set(iter, COL_ENABLE, True)

		self.view.columns_autosize()
		g.threads_leave()
		self.is_cddbing = False


	def get_cddb(self):
		'''Query cddb for track and cd info'''
		g.threads_enter()
		dlg = g.MessageDialog(buttons=g.BUTTONS_CANCEL, message_format="Getting Track Info.")
		dlg.set_position(g.WIN_POS_NONE)
		(a, b) = dlg.get_size()
		(x, y) = self.get_position()
		(dx, dy) = self.get_size()
		dlg.move(x+dx/2-a/2, y+dy/2-b/2)
		dlg.show()
		g.threads_leave()

		count = artist = genre = album = year = ''
		tracklist = []
		tracktime = []

		#Note: all the nested try statements are to ensure that as much
		#info is processed as possible.  One exception should not stop
		#the whole thing and return nothing.

		try:
			count = cd_logic.total_tracks()
			cddb_id = cd_logic.get_cddb_id()

			#PyCDDB wants a string delimited by spaces, go figure.
			cddb_id_string = ''
			for n in cddb_id:
				cddb_id_string += str(n)+' '
			#print disc_id
			#print cddb_id, cddb_id_string

			for i in range(count):
				tracktime = cd_logic.get_track_time_total(i+1)
				track_time = time.strftime('%M:%S', time.gmtime(tracktime))
				tracklist.append((_('Track')+`i`,track_time))

			try:
				db = PyCDDB.PyCDDB(CDDB_SERVER.value)
				query_info = db.query(cddb_id_string)
				#print query_info

				g.threads_enter()
				dlg.set_title(_('Got Disc Info'))
				g.threads_leave()

				#make sure we didn't get an error, then query CDDB
				if len(query_info) > 0:
					rndm = Random()
					index = rndm.randrange(0, len(query_info))
					read_info = db.read(query_info[index])
					#print read_info
					g.threads_enter()
					dlg.set_title(_('Got Track Info'))
					g.threads_leave()

					try:
						(artist, album) = query_info[index]['title'].split('/')
						artist = artist.strip()
						album = album.strip()
						genre = query_info[index]['category']
						if genre in ['misc', 'data']:
							genre = 'Other'

						print query_info['year']
						print read_info['EXTD']
						print read_info['YEARD']

						#x = re.match(r'.*YEAR: (.+).*',read_info['EXTD'])
						#if x:
						#	print x.group(1)
						#	year = x.group(1)
					except:
						pass

					if len(read_info['TTITLE']) > 0:
						for i in range(count):
							try:
								track_name = read_info['TTITLE'][i]
								track_time = tracklist[i][1]
								#print i, track_name, track_time
								tracklist[i] = (track_name, track_time)
							except:
								pass
			except:
				pass

		except:
			pass

		g.threads_enter()
		dlg.destroy()
		g.threads_leave()
		return count, artist, album, genre, year, tracklist


	def get_cdda2wav(self, tracknum, track):
		'''Run cdda2wav to rip a track from the CD'''
		cdda2wav_cmd = RIPPER.value
		cdda2wav_dev = RIPPER_DEV.value
		cdda2wav_lun = RIPPER_LUN.value
		cdda2wav_args = '-D%s -A%s -t %d "%s"' % (
						cdda2wav_lun, cdda2wav_dev, tracknum+1, strip_illegal(track))
		cdda2wav_opts =  RIPPER_OPTS.value
		#print cdda2wav_opts, cdda2wav_args

		thing = popen2.Popen4(cdda2wav_cmd+' '+cdda2wav_opts+' '+cdda2wav_args )
		outfile = thing.fromchild

		while True:
			line = myreadline(outfile)
			if line:
				x = re.match('([\s0-9]+)%', line)
				if x:
					percent = int(x.group(1))
					self.status_update(tracknum, 'rip', percent)
			else:
				break
			if self.stop_request:
				break

		if self.stop_request:
			os.kill(thing.pid, signal.SIGKILL)

		code = thing.wait()
		self.status_update(tracknum, 'rip', 100)
		#print code
		return code


	def get_lame(self, tracknum, track, artist, genre, album, year):
		'''Run lame to encode a wav file to mp3'''
		try:
			int_year = int(year)
		except:
			int_year = 1

		lame_cmd = MP3_ENCODER.value
		lame_opts = MP3_ENCODER_OPTS.value
		lame_tags = '--ta "%s" --tt "%s" --tl "%s" --tg "%s" --tn %d --ty %d' % (
					artist, track, album, genre, tracknum+1, int_year)
		lame_args = '"%s" "%s"' % (strip_illegal(track)+'.wav', strip_illegal(track)+'.mp3')

		#print lame_opts, lame_tags, lame_args

		thing = popen2.Popen4(lame_cmd+' '+lame_opts+' '+lame_tags+' '+lame_args )
		outfile = thing.fromchild

		while True:
			line = myreadline(outfile)
			if line:
				#print line
				#for some reason getting this right for lame was a royal pain.
				x = re.match(r"^[\s]+([0-9]+)/([0-9]+)", line)
				if x:
					percent = int(100 * (float(x.group(1)) / float(x.group(2))))
					self.status_update(tracknum, 'enc', percent)
			else:
				break
			if self.stop_request:
				break

		if self.stop_request:
			os.kill(thing.pid, signal.SIGKILL)
		elif HAVE_XATTR:
			try:
				filename = strip_illegal(track)+'.mp3'
				xattr.setxattr(filename, 'user.Title', track)
				xattr.setxattr(filename, 'user.Artist', artist)
				xattr.setxattr(filename, 'user.Album', album)
				xattr.setxattr(filename, 'user.Genre', genre)
				xattr.setxattr(filename, 'user.Track', '%d' % tracknum)
				xattr.setxattr(filename, 'user.Year', year)
			except:
				pass

		code = thing.wait()
		self.status_update(tracknum, 'enc', 100)
		#print code
		return code


	def get_ogg(self, tracknum, track, artist, genre, album, year):
		'''Run oggenc to encode a wav file to ogg'''
		try:
			int_year = int(year)
		except:
			int_year = 1

		ogg_cmd = OGG_ENCODER.value
		ogg_opts = OGG_ENCODER_OPTS.value
		ogg_tags = '-a "%s" -t "%s" -l "%s" -G "%s" -N %d -d %d' % (
					artist, track, album, genre, tracknum+1, int_year)
		ogg_args = '"%s"' % (strip_illegal(track)+'.wav')

		#print ogg_opts, ogg_tags, ogg_args

		thing = popen2.Popen4(ogg_cmd+' '+ogg_opts+' '+ogg_tags+' '+ogg_args )
		outfile = thing.fromchild

		while True:
			line = myreadline(outfile)
			if line:
				#print line
				#for some reason getting this right for ogg was a royal pain.
				x = re.match('^.*\[[\s]*([.0-9]+)%\]', line)
				if x:
					percent = float(x.group(1))
					self.status_update(tracknum, 'enc', percent)
			else:
				break
			if self.stop_request:
				break

		if self.stop_request:
			os.kill(thing.pid, signal.SIGKILL)
		elif HAVE_XATTR:
			try:
				filename = strip_illegal(track)+'.ogg'
				xattr.setxattr(filename, 'user.Title', track)
				xattr.setxattr(filename, 'user.Artist', artist)
				xattr.setxattr(filename, 'user.Album', album)
				xattr.setxattr(filename, 'user.Genre', genre)
				xattr.setxattr(filename, 'user.Track', '%d' % tracknum)
				xattr.setxattr(filename, 'user.Year', year)
			except:
				pass

		code = thing.wait()
		self.status_update(tracknum, 'enc', 100)
		#print code
		return code


	def rip_n_encode(self, button=None):
		'''Process all selected tracks (rip and encode)'''
		try: os.chdir(os.path.expanduser('~'))
		except: pass
		try: os.mkdir(LIBRARY.value)
		except: pass
		try: os.chdir(LIBRARY.value)
		except: pass

		if self.count and self.artist and self.album:
			try: os.mkdir(self.artist)
			except: pass

			try: os.mkdir(self.artist+'/'+self.album)
			except: pass

			try: os.chdir(self.artist+'/'+self.album)
			except: pass

		self.stop_request = False

		#the queue to feed tracks from ripper to encoder
		self.wavqueue = Queue.Queue(1000)

		self.ripper_thd = self.runit(self.ripit)
		self.encoder_thd = self.runit(self.encodeit)


	def ripit(self):
		'''Thread to rip all selected tracks'''
		self.is_ripping = True
		for i in range(self.count):
			if self.stop_request:
				break;

			if self.store[i][COL_ENABLE]:
				track = self.store[i][COL_TRACK]
				#print i, track
				status = self.get_cdda2wav(i, track)
				if status <> 0:
					print 'cdda2wav died %d' % status
					self.status_update(i, 'rip_error', 0)
				else:
					#push this track on the queue for the encoder
					if self.wavqueue:
						self.wavqueue.put((track, i))

		#push None object to tell encoder we're done
		if self.wavqueue:
			self.wavqueue.put((None, None))

		self.is_ripping = False
		cd_logic.stop()
		if EJECT_AFTER_RIP.int_value:
			cd_logic.eject()


	def encodeit(self):
		'''Thread to encode all tracks from the wavqueue'''
		self.is_encoding = True
		while True:
			if self.stop_request:
				break
			(track, tracknum) = self.wavqueue.get(True)
			if track == None:
				break

			if ENCODER.value == 'MP3':
				status = self.get_lame(tracknum, track, self.artist, self.genre, self.album, self.year)
			else:
				status = self.get_ogg(tracknum, track, self.artist, self.genre, self.album, self.year)

			if status <> 0:
				print 'encoder died %d' % status
				self.status_update(tracknum, 'enc_error', 0)
			try: os.unlink(strip_illegal(track)+".wav")
			except:	pass
			try: os.unlink(strip_illegal(track)+".inf")
			except:	pass

		self.is_encoding = False
		del self.wavqueue


	def status_update(self, row, state, percent):
		'''Callback from rip/encode threads to update display'''
		g.threads_enter()

		iter = self.store.get_iter((row,))
		if not iter: return

		if state == 'rip':
			if percent < 100:
				self.store.set_value(iter, COL_STATUS, _('Ripping')+': %d%%' % percent)
			else:
				self.store.set_value(iter, COL_STATUS, _('Ripping')+': '+_('done'))

		if state == 'enc':
			if percent < 100:
				self.store.set_value(iter, COL_STATUS, _('Encoding')+': %d%%' % percent)
			else:
				self.store.set_value(iter, COL_STATUS, _('Encoding')+': '+_('done'))

		if state == 'rip_error':
			self.store.set_value(iter, COL_STATUS, _('Ripping')+': '+_('error'))

		if state == 'enc_error':
			self.store.set_value(iter, COL_STATUS, _('Encoding')+': '+_('error'))

		g.threads_leave()


	def activate(self, view, path, column):
		'''Edit a track name'''
		model, iter = self.view.get_selection().get_selected()
		if iter:
			track = model.get_value(iter, COL_TRACK)
			dlg = g.Dialog(APP_NAME)
			dlg.set_position(g.WIN_POS_NONE)
			dlg.set_default_size(350, 100)
			(a, b) = dlg.get_size()
			(x, y) = self.get_position()
			(dx, dy) = self.get_size()
			dlg.move(x+dx/2-a/2, y+dy/2-b/2)
			dlg.show()

			entry = g.Entry()
			entry.set_text(track)
			dlg.set_position(g.WIN_POS_MOUSE)
			entry.show()
			entry.set_activates_default(True)
			dlg.vbox.pack_start(entry)

			dlg.add_button(g.STOCK_OK, g.RESPONSE_OK)
			dlg.add_button(g.STOCK_CANCEL, g.RESPONSE_CANCEL)
			dlg.set_default_response(g.RESPONSE_OK)
			response = dlg.run()

			if response == g.RESPONSE_OK:
				track = entry.get_text()
				#print track
				model.set_value(iter, COL_TRACK, track)
				self.view.columns_autosize()

			dlg.destroy()


	def toggle_check(self, cell, rownum):
		'''Toggle state for each song'''
		row = self.store[rownum]
		row[COL_ENABLE] = not row[COL_ENABLE]
		self.store.row_changed(rownum, row.iter)


	def set_selection(self, thing):
		'''Get current selection'''
		#model, iter = self.view.get_selection().get_selected()
		#if iter:
		#	track = model.get_value(iter, COL_TRACK)

	def button_press(self, text, event):
		'''Popup menu handler'''
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1

	def show_options(self, button=None):
		'''Show Options dialog'''
		rox.edit_options()

	def get_options(self):
		'''Get changed Options'''
		pass

	def delete_event(self, ev, e1):
		'''Bye-bye'''
		self.close()

	def close(self, button = None):
		'''We're outta here!'''
		self.destroy()


