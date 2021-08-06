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
from rox import i18n, app_options, Menu, filer, InfoWin, tasks, loading, fileutils
from rox.options import Option

import PyCDDB, cd_logic, CDROM, genres, support

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
LIBRARY = Option('library', '~/Music/$A/$L/$S')

#RIPPER options
DEVICE = Option('device', '/dev/cdrom')
RIPPER = Option('ripper', 'nice cdda2wav -g -x -H -D$D -A$D -t $T "$F"')
ENCODER = Option('encoder', 'nice lame --ta "$A" --tt "$S" --tl "$L" --tg "$G" --tn $T --ty $Y "$F.wav" "$F.mp3"')

EJECT_AFTER_RIP = Option('eject_after_rip', '0')
RIP_ONLY = Option('rip_only', '0')
KEEP_WAV = Option('keep_wav', '0')

CDDB_SERVER = Option('cddb_server', 'http://freedb.freedb.org/~cddb/cddb.cgi')

rox.app_options.notify()

COVER_SIZE = 96

#Column indicies
COL_ENABLE = 0
COL_TRACK = 1
COL_TIME = 2
COL_STATUS = 3
COL_PERCENT = 4

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


class Ripper(rox.Window, rox.loading.XDSLoader):
	'''Rip and Encode a CD'''
	def __init__(self):
		rox.Window.__init__(self)

		# Support dropping Album Cover Art on our Window.
		# Get types supported by gdk-pixbuf
		mtypes = []
		for fmt in gtk.gdk.pixbuf_get_formats():
			mtypes += fmt['mime_types']
		rox.loading.XDSLoader.__init__(self, mtypes)

		self.set_title(APP_NAME)
		self.set_default_size(450, 500)
		self.set_position(gtk.WIN_POS_MOUSE)

		# Capture wm delete event
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
		self.closing = False
		self.artwork_saved = False

		cd_logic.set_dev(DEVICE.value)
		self.cd_status = None
		self.disc_id = None
		self.cd_status_changed = False

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
			Menu.Action(_('Help'),	'show_help', '', gtk.STOCK_HELP),
			Menu.Action(_("Quit"), 'close', '', gtk.STOCK_CLOSE),
			])
		self.menu.attach(self,self)


	def build_toolbar(self):
		self.toolbar = gtk.Toolbar()
		self.toolbar.set_style(gtk.TOOLBAR_ICONS)
		self.toolbar.insert_stock(gtk.STOCK_HELP, _('Help'), None, self.show_help, None, 0)
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

		self.store = gtk.ListStore(int, str, str, str, int)
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
		column = gtk.TreeViewColumn(_('Track'), cell, text=COL_TRACK)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Time'), cell, text=COL_TIME)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		cell = gtk.CellRendererProgress()
		column = gtk.TreeViewColumn(_('Status'), cell, text=COL_STATUS, value=COL_PERCENT)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(False)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.handler = self.selection.connect('changed', self.set_selection)

		self.table = gtk.Table(5, 3, False)
		x_pad = 2
		y_pad = 1

		self.artwork = gtk.Image()
		self.table.attach(self.artwork, 2, 3, 0, 5, 0, 0, x_pad, y_pad)

		self.artist_entry = gtk.Entry(max=255)
		self.artist_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Artist')), 0, 1, 2, 3, 0, 0, 4, y_pad)
		self.table.attach(self.artist_entry, 1, 2, 2, 3, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		self.album_entry = gtk.Entry(max=255)
		self.album_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Album')),	0, 1, 3, 4, 0, 0, 4, y_pad)
		self.table.attach(self.album_entry,	1, 2, 3, 4, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		self.genre_entry = gtk.Entry(max=255)
		self.genre_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Genre')),	0, 1, 4, 5, 0, 0, 4, y_pad)
		self.table.attach(self.genre_entry,	1, 2, 4, 5, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		self.year_entry = gtk.Entry(max=4)
		self.year_entry.connect('changed', self.stuff_changed)
		self.table.attach(gtk.Label(str=_('Year')),	0, 1, 5, 6, 0, 0, 4, y_pad)
		self.table.attach(self.year_entry,	1, 2, 5, 6, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		hbox1 = gtk.HBox()
		
		ck = gtk.CheckButton(label=_('Eject after Rip'))
		ck.set_active(bool(EJECT_AFTER_RIP.int_value))
		ck.connect('toggled', self.toggled, EJECT_AFTER_RIP)
		hbox1.pack_start(ck, False, False, 5)
		
		ck = gtk.CheckButton(label=_('Keep WAV files'))
		ck.set_active(bool(KEEP_WAV.int_value))
		ck.connect('toggled', self.toggled, KEEP_WAV)
		hbox1.pack_start(ck, False, False, 5)
		
		ck = gtk.CheckButton(label=_('Rip Only'))
		ck.set_active(bool(RIP_ONLY.int_value))
		ck.connect('toggled', self.toggled, RIP_ONLY)
		hbox1.pack_start(ck, False, False, 5)

		self.table.attach(hbox1, 0, 3, 6, 7, True, True, x_pad, y_pad)


	def toggled(self, button, param):
		param.int_value = button.get_active()


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
				self.rip_btn.set_sensitive(bool(self.disc_id is not None))
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
	
			if self.closing:
				break

			yield tasks.TimeoutBlocker(1)

	
	def scale_cover(self, pixbuf):
		w = pixbuf.get_width()
		h = pixbuf.get_height()
		if w < COVER_SIZE and h < COVER_SIZE:
			return pixbuf
		scale = 1.0
		if w < COVER_SIZE*2 and h < COVER_SIZE*2:
			scale = 0.5
		else:
			if w > h:
				scale = float(COVER_SIZE)/float(w)
			else:
				scale = float(COVER_SIZE)/float(h)
	
		return pixbuf.scale_simple(int(scale*w), int(scale*h), gtk.gdk.INTERP_BILINEAR)
	

	def xds_load_from_stream(self, name, type, stream):
		loader = gtk.gdk.PixbufLoader()
		buf = stream.read()
		loader.write(buf)
		pbuf = loader.get_pixbuf()
		self.artwork_saved = False
		self.set_artwork(pbuf)


	def set_artwork(self, pbuf):
		if pbuf:
			self.artwork.set_from_pixbuf(self.scale_cover(pbuf))
			self.set_size_request(COVER_SIZE, COVER_SIZE)
			self.save_artwork()


	def save_artwork(self, force=False):
		pbuf = self.artwork.get_pixbuf()
		if not pbuf: return

		# We're not sure about the target dir yet, so skip it.
		if not force and not self.is_ripping and not self.is_encoding:
			return

		# OK, we're have the dir, check for existing file and ask to overwrite
		filename = os.path.join(self.library, '.DirIcon')
		if os.path.exists(filename):
			# Don't keep asking
			if self.artwork_saved:
				return
			if not rox.confirm(_("Overwrite current Artwork?"), gtk.STOCK_SAVE):
				return

		# Save the damn thing already, will ya?
		pbuf.save(filename, 'png')
		self.artwork_saved = True


	def stuff_changed(self, button=None):
		'''Get new text from edit boxes and save it'''
		self.genre = self.genre_entry.get_text()
		self.artist = self.artist_entry.get_text()
		self.album = self.album_entry.get_text()
		self.year = self.year_entry.get_text()
		self.parse_library()


	def stop(self, *it):
		'''Stop current rip/encode process'''
		self.stop_request = True


	def parse_library(self):
		library = os.path.expanduser(LIBRARY.value)
		filespec = ""
		if "$" in library:
			(library, filespec) = os.path.split(library)
			library = string.replace(library, '$A', strip_illegal(self.artist))
			library = string.replace(library, '$L', strip_illegal(self.album))
		self.library = library
		self.filespec = filespec  #still may contain $ parameters, we parse these later


	def build_filename(self, track, tracknum):
		fn = self.filespec
		fn = string.replace(fn, '$A', strip_illegal(self.artist)) 
		fn = string.replace(fn, '$L', strip_illegal(self.album))
		fn = string.replace(fn, '$S', strip_illegal(track))
		fn = string.replace(fn, '$T', "%02d" % (tracknum+1)) 
		return fn


	def show_dir(self, *dummy):
		''' Pops up a filer window. '''
		filer.show_file(self.library)


	def do_get_tracks(self, button=None):
		'''Get the track info (cddb and cd) in a thread'''
		if self.is_ripping:
			return
		tasks.Task(self.get_tracks())


	def no_disc(self):
		'''Clear all info and display <no disc>'''
		#dbg("no disc in tray?")
		self.store.clear()
		self.artist_entry.set_text(_('<no disc>'))
		self.album_entry.set_text('')
		self.genre_entry.set_text('')
		self.year_entry.set_text('')
		self.view.columns_autosize()
		self.stuff_changed()


	def get_tracks(self):
		'''Get the track info (cddb and cd)'''
		self.is_cddbing = True
		stuff = self.do_cddb()
		(count, artist, album, genre, year, tracklist) = stuff

		yield None

		self.count = count
		self.tracklist = tracklist

		if artist: self.artist_entry.set_text(artist)
		if album: self.album_entry.set_text(album)
		if genre: self.genre_entry.set_text(genre)
		if year: self.year_entry.set_text(year)
		self.stuff_changed()

		self.store.clear()
		for track in tracklist:
			#dbg(track)
			iter = self.store.append(None)
			self.store.set(iter, COL_TRACK, track[0])
			self.store.set(iter, COL_TIME, track[1])
			self.store.set(iter, COL_ENABLE, True)
			self.store.set(iter, COL_STATUS, None)
			self.store.set(iter, COL_STATUS, "")
			yield None

		self.view.columns_autosize()
		self.is_cddbing = False


	def do_cddb(self):
		'''Query cddb for track and cd info'''
		dlg = gtk.MessageDialog(buttons=gtk.BUTTONS_CANCEL, message_format="Getting Track Info.")
		dlg.show()
		dlg.set_transient_for(self)
		dlg.set_modal(True)
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

						dbg(query_info['year'])
						dbg(read_info['EXTD'])
						dbg(read_info['YEARD'])

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


	def run_ripper(self, tracknum, track, filename):
		'''Rip selected tracks from the CD'''
		cmd = RIPPER.value
		cmd = string.replace(cmd, '$D', DEVICE.value)
		cmd = string.replace(cmd, '$T', str(tracknum+1))
		cmd = string.replace(cmd, '$F', filename)
		thing = popen2.Popen4(cmd)
		return thing


	def run_encoder(self, tracknum, track, filename, artist, genre, album, year):
		'''Encode each WAV file'''
		if year is None or not len(year):
			year = "1900"

		cmd = ENCODER.value
		cmd = string.replace(cmd, '$A', artist)
		cmd = string.replace(cmd, '$L', album)
		cmd = string.replace(cmd, '$S', track)
		cmd = string.replace(cmd, '$G', genre)
		cmd = string.replace(cmd, '$T', str(tracknum+1))
		cmd = string.replace(cmd, '$Y', year)
		cmd = string.replace(cmd, '$F', filename)
		thing = popen2.Popen4(cmd)
		return thing


	def rip_n_encode(self, button=None):
		'''Process all selected tracks (rip and encode)'''
		try:
			rox.fileutils.makedirs(self.library)
			os.chdir(self.library)
		except:
			rox.alert(_("Failed to find or create Library dir.  Cannot save files."))
			return

		self.save_artwork(True) #force

		self.stop_request = False
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
				filename = self.build_filename(track, tracknum)
				thing = self.run_ripper(tracknum, track, filename)
				outfile = thing.fromchild
				percent = 0
				last_percent = -1
				handler = support.get_handler(RIPPER.value)
				while True:
					blocker = tasks.InputBlocker(outfile)
					yield blocker

					if self.stop_request:
						os.kill(thing.pid, signal.SIGKILL)
						break

					line = myreadline(outfile)
					if line:
						percent = handler.get_percent(line)
						if percent <> last_percent:
							self.status_update(tracknum, 'rip', percent)
							last_percent = percent
					else:
						break
				status = thing.wait()
				self.status_update(tracknum, 'rip', 100)

				if status <> 0:
					#dbg('ripper died %d' % status)
					self.status_update(tracknum, 'rip_error', 0)
				else:
					#push this track on the queue for the encoder
					if self.wavqueue and not RIP_ONLY.int_value:
						self.wavqueue.put((track, tracknum, filename))

		#push None object to tell encoder we're done
		if self.wavqueue:
			self.wavqueue.put((None, None, None))

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
				(track, tracknum, filename) = self.wavqueue.get(False)
			except Queue.Empty:
				yield tasks.TimeoutBlocker(1)
				continue

			if track == None:
				break

			thing = self.run_encoder(tracknum, track, filename, self.artist, self.genre, self.album, self.year)
			outfile = thing.fromchild

			handler = support.get_handler(ENCODER.value)
			while True:
				blocker = tasks.InputBlocker(outfile)
				yield blocker

				if self.stop_request:
					os.kill(thing.pid, signal.SIGKILL)
					break

				line = myreadline(outfile)
				#dbg(line)
				if line:
					percent = handler.get_percent(line)
					self.status_update(tracknum, 'enc', percent)
				else:
					break

			status = thing.wait()
			self.status_update(tracknum, 'enc', 100)

			if status <> 0:
				dbg('encoder died %d' % status)
				self.status_update(tracknum, 'enc_error', 0)

			if not KEEP_WAV.int_value:
				try: os.unlink(filename+".wav")
				except:	pass

		self.is_encoding = False


	def status_update(self, row, state, percent):
		'''Callback from rip/encode tasks to update display'''
		if 'rip' in state:
			msg1 = _('Ripping')
		elif 'enc' in state:
			msg1 = _('Encoding')

		if 'error' in state:
			msg2 = ': %s' % _('error')
#		elif percent == -1:
#			pulse #how to handle unknown processes?
		elif percent < 100:
			msg2 = ': %d%%' % percent
		else:
			msg2 = ': %s' % _('done')

		iter = self.store.get_iter((row,))
		if iter:
			self.store.set_value(iter, COL_STATUS, msg1+msg2)
			self.store.set_value(iter, COL_PERCENT, percent)


	def activate(self, view, path, column):
		'''Edit a track name'''
		model, iter = self.view.get_selection().get_selected()
		if iter:
			track = model.get_value(iter, COL_TRACK)
			dlg = gtk.Dialog(APP_NAME)
			dlg.set_default_size(350, 100)
			dlg.set_transient_for(self)
			dlg.set_modal(True)
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
		# Not needed, just here for future reference.
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
		#TODO Update toggle buttons
		self.stuff_changed()

	def show_help(self, button=None):
		rox.filer.open_dir(os.path.join(rox.app_dir, 'Help'))

	def delete_event(self, ev, e1):
		'''Bye-bye'''
		self.close()

	def close(self, button = None):
		'''We are outta here!'''
		self.stop()
		self.closing = True
		if self.is_ripping or self.is_encoding:
			dlg = gtk.MessageDialog(self, gtk.DIALOG_MODAL|gtk.DIALOG_DESTROY_WITH_PARENT)
			dlg.set_title(_("Please Wait"))
			dlg.set_markup(_("Stopping current operation..."))
			dlg.show()
			while self.is_ripping or self.is_encoding:
				while gtk.events_pending():
					gtk.main_iteration()
			dlg.destroy()
		self.destroy()


