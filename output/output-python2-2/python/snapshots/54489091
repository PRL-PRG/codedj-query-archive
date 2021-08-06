"""
	ripper.py
		GUI front-end to cdda2wav and lame.

	Copyright 2004-2006 Kenneth Hayber <ken@hayber.us>
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

import gtk, os, sys, signal, re, string, socket, time, popen2, Queue
from random import Random

import rox
from rox import i18n, app_options, Menu, filer, InfoWin, tasks
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
APP_PATH = rox.app_dir


#Options.xml processing
from rox import choices
choices.migrate(APP_NAME, 'hayber.us')
rox.setup_app_options(APP_NAME, site='hayber.us')
Menu.set_save_name(APP_NAME, site='hayber.us')

#assume that everyone puts their music in ~/Music
LIBRARY = Option('library', '~/Music')

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

DEBUG = 0
def dbg(*args):
	if DEBUG:
		import sys
		print >>sys.stderr, args


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
		self.set_position(gtk.WIN_POS_MOUSE)

		#capture wm delete event
		self.connect("delete_event", self.delete_event)

		# Update things when options change
		rox.app_options.add_notify(self.get_options)

		self.build_gui()
		self.build_toolbar()
		self.build_menu()

		# Create layout, pack and show widgets
		self.vbox = gtk.VBox()
		self.add(self.vbox)
		self.vbox.pack_start(self.toolbar, False, True, 0)
		self.vbox.pack_start(self.table, False, True, 0)
		self.vbox.pack_start(self.scroll_window, True, True, 0)
		self.vbox.show_all()

		# Defaults and Misc
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

		tasks.Task(self.update_gui())


	def build_menu(self):
		self.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.connect('button-press-event', self.button_press)
		self.view.add_events(gtk.gdk.BUTTON_PRESS_MASK)
		self.view.connect('button-press-event', self.button_press)

		self.menu = Menu.Menu('main', [
			Menu.Action(_('Rip & Encode'), 'rip_n_encode', '', gtk.STOCK_EXECUTE),
			Menu.Action(_('Reload CD'), 'do_get_tracks', '', gtk.STOCK_REFRESH),
			Menu.Action(_('Stop'), 'stop', '', gtk.STOCK_STOP),
			Menu.Separator(),
			Menu.Action(_('Options'), 'show_options', '', gtk.STOCK_PREFERENCES),
			Menu.Action(_('Info'),	'get_info', '', gtk.STOCK_DIALOG_INFO),
			Menu.Action(_("Quit"), 'close', '', gtk.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


	def build_toolbar(self):
		self.toolbar = gtk.Toolbar()
		self.toolbar.set_style(gtk.TOOLBAR_ICONS)
		self.toolbar.insert_stock(gtk.STOCK_PREFERENCES, _('Settings'), None, self.show_options, None, 0)
		self.stop_btn = self.toolbar.insert_stock(gtk.STOCK_STOP, _('Stop'), None, self.stop, None, 0)
		self.rip_btn = self.toolbar.insert_stock(gtk.STOCK_EXECUTE, _('Rip & Encode'), None, self.rip_n_encode, None, 0)
		self.refresh_btn = self.toolbar.insert_stock(gtk.STOCK_REFRESH, _('Reload CD'), None, self.do_get_tracks, None, 0)
		self.toolbar.insert_stock(gtk.STOCK_GO_UP, _('Show destination dir'), None, self.show_dir, None, 0)
		self.toolbar.insert_stock(gtk.STOCK_CLOSE, _('Close'), None, self.close, None, 0)


	def build_gui(self):
		swin = gtk.ScrolledWindow()
		self.scroll_window = swin
		swin.set_policy(gtk.POLICY_AUTOMATIC, gtk.POLICY_AUTOMATIC)
		swin.set_shadow_type(gtk.SHADOW_IN)

		self.store = gtk.ListStore(int, str, str, str)
		view = gtk.TreeView(self.store)
		self.view = view
		swin.add(view)
		view.set_rules_hint(True)

		cell = gtk.CellRendererToggle()
		cell.connect('toggled', self.toggle_check)
		column = gtk.TreeViewColumn('', cell, active=COL_ENABLE)
		view.append_column(column)
		column.set_resizable(False)
		column.set_reorderable(False)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Track'), cell, text = COL_TRACK)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Time'), cell, text = COL_TIME)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Status'), cell, text = COL_STATUS)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)

		self.table = gtk.Table(5, 2, False)
		x_pad = 2
		y_pad = 1

		self.artist_entry = gtk.Entry(max=255)
		self.artist_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Artist')), 0, 1, 2, 3, 0, 0, 4, y_pad)
		self.table.attach(self.artist_entry, 1, 2, 2, 3, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		self.album_entry = gtk.Entry(max=255)
		self.album_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Album')),	0, 1, 3, 4, 0, 0, 4, y_pad)
		self.table.attach(self.album_entry,	1, 2, 3, 4, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		genres.genre_list.sort()
		self.genre_combo = gtk.Combo()
		self.genre_combo.set_popdown_strings(genres.genre_list)
		self.genre_combo.entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Genre')),	0, 1, 4, 5, 0, 0, 4, y_pad)
		self.table.attach(self.genre_combo,	1, 2, 4, 5, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		self.year_entry = gtk.Entry(max=4)
		self.year_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Year')),	0, 1, 5, 6, 0, 0, 4, y_pad)
		self.table.attach(self.year_entry,	1, 2, 5, 6, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)


	def update_gui(self):
		'''Update UI based on current state'''
		while True:
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
	
			yield tasks.TimeoutBlocker(1)
	

	def stuff_changed(self, button=None):
		'''Get new text from edit boxes and save it'''
		self.genre = self.genre_combo.entry.get_text()
		self.artist = self.artist_entry.get_text()
		self.album = self.album_entry.get_text()
		self.year = self.year_entry.get_text()


	def stop(self, it):
		'''Stop current rip/encode process'''
		self.stop_request = True


	def show_dir(self, *dummy):
		''' Pops up a filer window. '''
		temp = os.path.join(os.path.expanduser(LIBRARY.value), self.artist, self.album)
		filer.show_file(temp)


	def do_get_tracks(self, button=None):
		'''Get the track info (cddb and cd) in a thread'''
		if self.is_ripping:
			return
		tasks.Task(self.get_tracks())


	def no_disc(self):
		'''Clear all info and display <no disc>'''
		dbg("no disc in tray?")
		self.store.clear()
		self.artist_entry.set_text(_('<no disc>'))
		self.album_entry.set_text('')
		self.genre_combo.entry.set_text('')
		self.year_entry.set_text('')
		self.view.columns_autosize()


	def get_tracks(self):
		'''Get the track info (cddb and cd)'''
		self.is_cddbing = True
		stuff = self.get_cddb()
		(count, artist, album, genre, year, tracklist) = stuff

		yield None

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
			dbg(track)
			iter = self.store.append(None)
			self.store.set(iter, COL_TRACK, track[0])
			self.store.set(iter, COL_TIME, track[1])
			self.store.set(iter, COL_ENABLE, True)
			yield None

		self.view.columns_autosize()
		self.is_cddbing = False


	def get_cddb(self):
		'''Query cddb for track and cd info'''
		dlg = gtk.MessageDialog(buttons=gtk.BUTTONS_CANCEL, message_format="Getting Track Info.")
		dlg.show()
		while gtk.events_pending():
			gtk.main_iteration()

		count = artist = genre = album = year = ''
		tracklist = []
		tracktime = []

		#Note: all the nested try|except|pass statements are to ensure that as much
		#info is processed as possible.  One exception should not stop
		#the whole thing and return nothing.

		try:
			count = cd_logic.total_tracks()
			cddb_id = cd_logic.get_cddb_id()

			#PyCDDB wants a string delimited by spaces, go figure.
			cddb_id_string = ''
			for n in cddb_id:
				cddb_id_string += str(n)+' '
			#dbg(disc_id, cddb_id, cddb_id_string)

			for i in range(count):
				tracktime = cd_logic.get_track_time_total(i+1)
				track_time = time.strftime('%M:%S', time.gmtime(tracktime))
				tracklist.append((_('Track')+`i`,track_time))

			try:
				db = PyCDDB.PyCDDB(CDDB_SERVER.value)
				query_info = db.query(cddb_id_string)
				#dbg(query_info)

				dlg.set_title(_('Got Disc Info'))
				while gtk.events_pending():
					gtk.main_iteration()
				
				#make sure we didn't get an error, then query CDDB
				if len(query_info) > 0:
					rndm = Random()
					index = rndm.randrange(0, len(query_info))
					read_info = db.read(query_info[index])
					#dbg(read_info)
					dlg.set_title(_('Got Track Info'))
					while gtk.events_pending():
						gtk.main_iteration()

					try:
						(artist, album) = query_info[index]['title'].split('/')
						artist = artist.strip().decode('latin-1', 'replace')
						album = album.strip().decode('latin-1', 'replace')
						genre = query_info[index]['category']
						if genre in ['misc', 'data']:
							genre = 'Other'

						#dbg(query_info['year'])
						#dbg(read_info['EXTD'])
						#dbg(read_info['YEARD'])

						#x = re.match(r'.*YEAR: (.+).*',read_info['EXTD'])
						#if x:
						#	print x.group(1)
						#	year = x.group(1)
					except:
						pass

					if len(read_info['TTITLE']) > 0:
						for i in range(count):
							try:
								track_name = read_info['TTITLE'][i].decode('latin-1', 'replace')
								track_time = tracklist[i][1]
								#dbg(i, track_name, track_time)
								tracklist[i] = (track_name, track_time)
							except:
								pass
			except:
				pass

		except:
			pass

		dlg.destroy()
		return count, artist, album, genre, year, tracklist


	def run_cdda2wav(self, tracknum, track):
		'''Run cdda2wav to rip a track from the CD'''
		cdda2wav_cmd = RIPPER.value
		cdda2wav_dev = RIPPER_DEV.value
		cdda2wav_lun = RIPPER_LUN.value
		cdda2wav_args = '-g -D%s -A%s -t %d "%s"' % (
						cdda2wav_lun, cdda2wav_dev, tracknum+1, strip_illegal(track))
		cdda2wav_opts =  RIPPER_OPTS.value

		thing = popen2.Popen4(cdda2wav_cmd+' '+cdda2wav_opts+' '+cdda2wav_args )
		return thing


	def run_lame(self, tracknum, track, artist, genre, album, year):
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

		#dbg(lame_opts, lame_tags, lame_args)
		thing = popen2.Popen4(lame_cmd+' '+lame_opts+' '+lame_tags+' '+lame_args )
		return thing


	def run_ogg(self, tracknum, track, artist, genre, album, year):
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

		#dbg(ogg_opts, ogg_tags, ogg_args)
		thing = popen2.Popen4(ogg_cmd+' '+ogg_opts+' '+ogg_tags+' '+ogg_args )
		return thing


	def rip_n_encode(self, button=None):
		'''Process all selected tracks (rip and encode)'''
		try:
			os.chdir(os.path.expanduser(LIBRARY.value))
		except:
			try:
				os.mkdir(os.path.expanduser(LIBRARY.value))
				os.chdir(os.path.expanduser(LIBRARY.value))
			except:
				rox.alert("Failed to find or create Library dir")

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

		tasks.Task(self.ripit())
		tasks.Task(self.encodeit())


	def ripit(self):
		'''Thread to rip all selected tracks'''
		self.is_ripping = True
		for tracknum in range(self.count):
			if self.stop_request:
				break;

			if self.store[tracknum][COL_ENABLE]:
				track = self.store[tracknum][COL_TRACK]
				thing = self.run_cdda2wav(tracknum, track)
				outfile = thing.fromchild
				while True:
					blocker = tasks.InputBlocker(outfile)
					yield blocker

					if self.stop_request:
						os.kill(thing.pid, signal.SIGKILL)
						break

					line = myreadline(outfile)
					if line:
						x = re.match(".*([ 0-9][0-9])%", line)
						if x:
							percent = int(x.group(1))
							self.status_update(tracknum, 'rip', percent)
					else:
						break

				status = thing.wait()
				self.status_update(tracknum, 'rip', 100)

				if status <> 0:
					#dbg('cdda2wav died %d' % status)
					self.status_update(tracknum, 'rip_error', 0)
				else:
					#push this track on the queue for the encoder
					if self.wavqueue:
						self.wavqueue.put((track, tracknum))

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

			try:
				(track, tracknum) = self.wavqueue.get(False)
			except Queue.Empty:
				yield tasks.TimeoutBlocker(1)
				continue

			if track == None:
				break

			if ENCODER.value == 'MP3':
				thing = self.run_lame(tracknum, track, self.artist, self.genre, self.album, self.year)
			else:
				thing = self.run_ogg(tracknum, track, self.artist, self.genre, self.album, self.year)

			outfile = thing.fromchild
			while True:
				blocker = tasks.InputBlocker(outfile)
				yield blocker

				if self.stop_request:
					os.kill(thing.pid, signal.SIGKILL)
					break

				line = myreadline(outfile)
				#dbg(line)
				if line:
					if ENCODER.value == 'MP3':
						x = re.match(r"^[\s]+([0-9]+)/([0-9]+)", line)
						if x:
							percent = int(100 * (float(x.group(1)) / float(x.group(2))))
							self.status_update(tracknum, 'enc', percent)
					else: #OGG
						x = re.match('^.*\[[\s]*([.0-9]+)%\]', line)
						if x:
							percent = float(x.group(1))
							self.status_update(tracknum, 'enc', percent)
				else:
					break

			status = thing.wait()
			self.status_update(tracknum, 'enc', 100)

			if status <> 0:
				#dbg('encoder died %d' % status)
				self.status_update(tracknum, 'enc_error', 0)

			try: os.unlink(strip_illegal(track)+".wav")
			except:	pass
			try: os.unlink(strip_illegal(track)+".inf")
			except:	pass

		self.is_encoding = False


	def status_update(self, row, state, percent):
		'''Callback from rip/encode tasks to update display'''
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


	def activate(self, view, path, column):
		'''Edit a track name'''
		model, iter = self.view.get_selection().get_selected()
		if iter:
			track = model.get_value(iter, COL_TRACK)
			dlg = gtk.Dialog(APP_NAME)
			dlg.set_default_size(350, 100)
			dlg.show()

			entry = gtk.Entry()
			entry.set_text(track)
			entry.show()
			entry.set_activates_default(True)
			dlg.vbox.pack_start(entry)

			dlg.add_button(gtk.STOCK_OK, gtk.RESPONSE_OK)
			dlg.add_button(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL)
			dlg.set_default_response(gtk.RESPONSE_OK)
			response = dlg.run()

			if response == gtk.RESPONSE_OK:
				track = entry.get_text()
				#dbg(track)
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

	def get_info(self):
		InfoWin.infowin(APP_NAME)

	def delete_event(self, ev, e1):
		'''Bye-bye'''
		self.close()

	def close(self, button = None):
		'''We're outta here!'''
		self.destroy()


