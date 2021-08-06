"""
	filefind.py
		A Find in Files Utility for the ROX Desktop.

	Copyright 2005 Kenneth Hayber <ken@hayber.us>
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

import rox, os, sys, popen2, signal
from rox import filer, Menu, tasks, basedir, mime

import gtk, os, sys, signal, re, string, socket, time, popen2, Queue, pango, gobject


APP_NAME = 'Find'
APP_SITE = 'hayber.us'
APP_PATH = rox.app_dir

MAX_HISTORY = 10

#Options.xml processing
from rox.options import Option
rox.setup_app_options(APP_NAME, site=APP_SITE)
Menu.set_save_name(APP_NAME, site=APP_SITE)

OPT_FIND_CMD = Option('find_cmd', 'find "$P" $R -name "$F" -exec grep -Hn $C $B "$T" "{}" \;')
OPT_EDIT_CMD = Option('edit_cmd', None)

OPT_MATCH_CASE = Option('match_case', True)
OPT_MATCH_WORDS = Option('match_words', False)
OPT_RECURSE_DIRS = Option('recurse_dirs', True)
OPT_IGNORE_BIN = Option('ignore_binary', True)

OPT_MATCH_CASE_ON = Option('match_case_on', '')
OPT_MATCH_CASE_OFF = Option('match_case_off', '-i')

OPT_MATCH_WORDS_ON = Option('match_words_on', '-w')
OPT_MATCH_WORDS_OFF = Option('match_words_off', '')

OPT_RECURSE_DIRS_ON = Option('recurse_dirs_on', '')
OPT_RECURSE_DIRS_OFF = Option('recurse_dirs_off', '-maxdepth 1')

OPT_IGNORE_BIN_ON = Option('ignore_binary_on', '-I')
OPT_IGNORE_BIN_OFF = Option('ignore_binary_off', '')


rox.app_options.notify()

#if you don't like the ComboBox, but want history support
#you can hardcode this to False (requires gtk 2.4+)
use_combo_box = hasattr(gtk, 'ComboBox')

if use_combo_box:
	_entryClass = gtk.ComboBoxEntry
else:
	_entryClass = gtk.Entry

class EntryThing(_entryClass):
	'''This class does two things.
		1) it wraps gtk.Entry | gtk.ComboBoxEntry for backwards compatibility
		2) it adds history support via the ComboBox or EntryCompletion
	''' 
	def __init__(self, history=None):
		self.history = history
		self.liststore = gtk.ListStore(gobject.TYPE_STRING)
		if use_combo_box:
  			_entryClass.__init__(self, self.liststore, 0)
			self.entry = self.child
  		else:
  			_entryClass.__init__(self)
			self.entry = self
			try: #for gtk < 2.4 compatibility and in case nothing is saved yet
				completion = gtk.EntryCompletion()
				self.set_completion(completion)
				completion.set_model(self.liststore)
				completion.set_text_column(0)
			except:
				pass
				
		self.load()
		
	def __del__(self):
		'''This is supposed to write the history out upon exit,
		   but it never gets called!!!
		'''
		self.write()
  			
  	def get_text(self):
  		if use_combo_box:
  			return self.child.get_text()
  		else:
  			return _entryClass.get_text(self)
  	
  	def set_text(self, text):
		self.add_text(text)
  		if use_combo_box:
  			index = self.find_text(text)
  			if index != None:
  				self.set_active(index)
  		else:
  			_entryClass.set_text(self, text)
   	
   	def find_text(self, text):
   		'''Check history to see if text already exists, return index or None'''
   		if not text: return
   		try:
			item = self.liststore.get_iter_first()
			index = 0
			while item:
				old_item = self.liststore.get_value(item, 0)
				if old_item == text:
					return index 
				item = self.liststore.iter_next(item)
				index += 1
			return None
		except:
			return None
	   		
  	def add_text(self, text):
  		'''Add item to history (if not duplicate)'''
  		if not text: return
  		if self.find_text(text) != None: return
  		try:
			n = len(self.liststore)
			if n >= MAX_HISTORY:
				self.liststore.remove(self.liststore.get_iter((n-1,)))
			self.liststore.insert(0, [text])
		except:
			pass
  					
  	def write(self):
  		'''Write history to file in config directory'''
		if not self.history:
			return

		def func(model, path, item, history_file):
			history_file.write(model.get_value(item, 0)+'\n')

		try:
			save_dir = basedir.save_config_path(APP_SITE, APP_NAME)
			history_file = file(os.path.join(save_dir, self.history), 'w')
			self.liststore.foreach(func, history_file)
		except:
			pass
  		
  	def load(self):
  		'''Read history from file and add to liststore'''
		if not self.history:
			return

		try:
			save_dir = basedir.save_config_path(APP_SITE, APP_NAME)
			history_file = file(os.path.join(save_dir, self.history), 'r')
			lines = history_file.readlines()
			for x in lines:
				self.liststore.append([x[:-1]]) #remove trailing newline
			history_file.close()
		except:
			pass  
			
RESPONSE_FIND = 1
RESPONSE_CLEAR = 2
RESPONSE_PREFS = 3


class FindWindow(rox.Dialog):
	'''A Find in Files Utility:
	   Calls external search (e.g. find | grep) tool and parses output.
	   Found files and the matching text are displayed in a list.
	   Activating items in the list opens a Text Editor, optionally jumping to 
	   the specific line of text.
	'''
	find_process = None
	updating_button_state = False	# Ignore events
	cancel = False
	tree_node = None

	def __init__(self, in_path=None, in_text=None, in_files=None):
		rox.Dialog.__init__(self)
		self.set_has_separator(False)
		self.set_title(APP_NAME)
		self.set_default_size(550, 500)
		
		self.selected = False
		self.path = ''
		self.what = ''
		self.where = ''
		
		self.add_button(gtk.STOCK_CLEAR, RESPONSE_CLEAR)
		self.add_button(gtk.STOCK_CLOSE, gtk.RESPONSE_CANCEL)	# Must be Cancel for Escape to work
		self.add_button(gtk.STOCK_HELP, gtk.RESPONSE_HELP)
		self.find_btn = gtk.ToggleButton(gtk.STOCK_FIND)
		self.find_btn.set_use_stock(True)
		self.find_btn.set_flags(gtk.CAN_DEFAULT)
		self.add_action_widget(self.find_btn, RESPONSE_FIND)

		self.set_default_response(RESPONSE_FIND)
		def resp(box, action):
			if self.updating_button_state: return

			if action == gtk.RESPONSE_CANCEL:
				self.close()
			elif action == RESPONSE_CLEAR:
				self.clear()
			elif action == RESPONSE_FIND:
				if self.find_process is None:
					self.start_find()
				else:
					self.cancel_find()
			elif action == gtk.RESPONSE_HELP:
				filer.open_dir(os.path.join(rox.app_dir, 'Help'))
		self.connect('response', resp)
		self.connect('delete_event', self.close)

		self.set_response_sensitive(RESPONSE_CLEAR, False)

		# Create layout, pack and show widgets
		table = gtk.Table(5, 2, False)
		x_pad = 2
		y_pad = 1

		path = EntryThing('path')
		table.attach(gtk.Label(_('Path')), 0, 1, 2, 3, 0, 0, 4, y_pad)
		table.attach(path, 1, 2, 2, 3, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)
		if hasattr(gtk, 'FileChooserDialog'):
			self.browse = gtk.Button(label='...')
			self.browse.connect('clicked', self.browser, path)
			table.attach(self.browse, 2, 3, 2, 3, 0, 0, x_pad, y_pad)

		what = EntryThing('pattern')
		path.entry.connect('activate', lambda p: what.grab_focus())
		table.attach(gtk.Label(_('Pattern')),	0, 1, 3, 4, 0, 0, 4, y_pad)
		table.attach(what, 1, 2, 3, 4, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)

		where = EntryThing('files')
		what.entry.connect('activate', lambda p: where.grab_focus())
		where.entry.set_activates_default(True)
		table.attach(gtk.Label(_('Files')),	0, 1, 4, 5, 0, 0, 4, y_pad)
		table.attach(where, 1, 2, 4, 5, gtk.EXPAND|gtk.FILL, 0, x_pad, y_pad)
		
		hbox1 = gtk.HBox()
		
		self.match_case = gtk.CheckButton(label=_('Match case'))
		self.match_case.set_active(bool(OPT_MATCH_CASE.int_value))
		hbox1.pack_start(self.match_case,False, False, 5)
		
		self.match_words = gtk.CheckButton(label=_('Match whole words'))
		self.match_words.set_active(bool(OPT_MATCH_WORDS.int_value))
		hbox1.pack_start(self.match_words,False, False, 5)
		
		self.ignore_binary = gtk.CheckButton(label=_('Ignore binary files'))
		self.ignore_binary.set_active(bool(OPT_IGNORE_BIN.int_value))
		hbox1.pack_start(self.ignore_binary, False, False, 5)
		
		hbox2 = gtk.HBox()
		
		self.recurse_dirs = gtk.CheckButton(label=_('Search subdirectories'))
		self.recurse_dirs.set_active(bool(OPT_RECURSE_DIRS.int_value))
		hbox2.pack_start(self.recurse_dirs,False, False, 5)

		prefs_btn = gtk.Button(stock = gtk.STOCK_PREFERENCES)
		hbox2.pack_start(prefs_btn,False, False, 5)
		prefs_btn.connect('clicked', self.edit_options)

		swin = gtk.ScrolledWindow()
		swin.set_policy(gtk.POLICY_AUTOMATIC, gtk.POLICY_AUTOMATIC)
		swin.set_shadow_type(gtk.SHADOW_IN)

		self.store = gtk.TreeStore(str, int, str)
		view = gtk.TreeView(self.store)
		self.view = view
		swin.add(view)
		view.set_rules_hint(False)
		
#		def func(model, iter, data):
#			if len(model.get_path(iter)) < 2:
#				return True
#			else:
#				return False
#		view.set_row_separator_func(func, None)
		
		cell = gtk.CellRendererText()
		try: #for pre gtk 2.6.0 support
			cell.set_property('ellipsize_set', True)
			cell.set_property('ellipsize', pango.ELLIPSIZE_START)
		except: pass
		self.filename_column = gtk.TreeViewColumn(_('Filename'), cell, text = 0)
		view.append_column(self.filename_column)
		self.filename_column.set_resizable(True)
		self.filename_column.set_reorderable(True)
		self.filename_column.set_min_width(130)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Line'), cell, text = 1)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(True)

		cell = gtk.CellRendererText()
		column = gtk.TreeViewColumn(_('Text'), cell, text = 2)
		view.append_column(column)
		column.set_resizable(True)
		column.set_reorderable(True)

		view.connect('row-activated', self.activate)
		self.selection = view.get_selection()
		self.selection.connect('changed', self.set_selection)

		vbox = self.vbox
		vbox.set_spacing(5)
		vbox.pack_start(table, False, False)
		vbox.pack_start(hbox1, False, False)
		vbox.pack_start(hbox2, False, False)
		vbox.pack_start(swin, True, True)
		vbox.show_all()

		what.connect('changed', self.entry_changed)
		where.connect('changed', self.entry_changed)
		path.connect('changed', self.entry_changed)
		
		self.path_entry = path
		self.what_entry = what
		self.where_entry = where
		
		if in_path: path.set_text(in_path)
		if in_text: what.set_text(in_text)
		if in_files: where.set_text(in_files)
			
		self.set_sensitives()


	def start_find(self, *args):
		'''Execute the find command after applying optional paramters'''
		self.view.grab_focus()	# Take focus from Find button!
		self.cancel = False
		
		self.path_entry.add_text(self.path)
		self.what_entry.add_text(self.what)
		self.where_entry.add_text(self.where)
		
		self.tree_node = self.store.append(None)
		self.store.set(self.tree_node, 
						0, os.path.join(self.path, self.where),
						2, self.what)
		self.view.set_cursor(self.store.get_path(self.tree_node))
		
		path = os.path.expanduser(self.path)
				
		cmd = OPT_FIND_CMD.value
		#long options (deprecated)
		cmd = string.replace(cmd, '$Path', path)
		cmd = string.replace(cmd, '$Files', self.where)
		cmd = string.replace(cmd, '$Text', self.what)
		#short options
		cmd = string.replace(cmd, '$P', path)
		cmd = string.replace(cmd, '$F', self.where)
		cmd = string.replace(cmd, '$T', self.what)
		
		cmd = string.replace(cmd, '$C', [OPT_MATCH_CASE_OFF.value,
						OPT_MATCH_CASE_ON.value]
						[self.match_case.get_active()])
										
		cmd = string.replace(cmd, '$W', [OPT_MATCH_WORDS_OFF.value, 
						OPT_MATCH_WORDS_ON.value]
						[self.match_words.get_active()])
										
		cmd = string.replace(cmd, '$B', [OPT_IGNORE_BIN_OFF.value, 
						OPT_IGNORE_BIN_ON.value]
						[self.ignore_binary.get_active()])
										
		cmd = string.replace(cmd, '$R', [OPT_RECURSE_DIRS_OFF.value, 
						OPT_RECURSE_DIRS_ON.value]
						[self.recurse_dirs.get_active()])
		
		self.find_process = popen2.Popen3(cmd)
		tasks.Task(self.get_status(self.find_process))
		self.set_sensitives()
		
		
	def cancel_find(self, *args):
		self.cancel = True
		os.kill(self.find_process.pid, signal.SIGKILL)

				
	def clear(self, *args):
		self.store.clear()
		self.selected = False
		self.set_sensitives()


	def close(self, *args):
		if self.find_process is not None:
			self.cancel_find()
		self.path_entry.write()
		self.what_entry.write()
		self.where_entry.write()
		self.destroy()


	def get_status(self, thing):
		'''Parse the ouput of the find command and fill the listbox.'''
		outfile = thing.fromchild
		results = None
		while True:
			blocker = tasks.InputBlocker(outfile)
			yield blocker
			if self.cancel:
				self.cancel = False
				self.find_process = None
				return
			line = outfile.readline()
			if line:
				results = True
				self.set_sensitives()
				iter = self.store.append(self.tree_node)
				self.view.expand_row(self.store.get_path(self.tree_node), False)
				try:
					(filename, lineno, text) = string.split(line, ':', 2)
					self.store.set(iter, 0, filename, 1, int(lineno), 2, text[:-1])
				except:
					self.store.set(iter, 2, line[:-1])
			else:
				code = thing.wait()
				self.find_process = None
				if code:
					results = False
					rox.info(_('There was a problem with this search'))
				break

		self.set_sensitives()
		if results is None:
			rox.info(_('Your search returned no results'))
		

	def entry_changed(self, button):
		self.path = self.path_entry.get_text()
		self.what = self.what_entry.get_text()
		self.where = self.where_entry.get_text()
		self.set_sensitives()
		
				
	def set_sensitives(self):
		if self.find_process or (len(self.what) and len(self.where) and len(self.path)):
			self.find_btn.set_sensitive(True)
		else:
			self.find_btn.set_sensitive(False)

		self.set_response_sensitive(RESPONSE_CLEAR, bool(len(self.store)))

		self.updating_button_state = True
		self.find_btn.set_active(self.find_process is not None)
		self.updating_button_state = False
					

	def browser(self, button, path_widget):
		browser = gtk.FileChooserDialog(title=_('Select folder'), buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL, gtk.STOCK_OPEN, gtk.RESPONSE_ACCEPT))
		if not len(self.path):
			self.path = os.path.expanduser('~')
		browser.set_current_folder(self.path)
		browser.set_action(gtk.FILE_CHOOSER_ACTION_SELECT_FOLDER)
		if browser.run() != gtk.RESPONSE_CANCEL:
			try:
				self.path = browser.get_filename()
				path_widget.set_text(self.path)
			except:
				rox.report_exception()
		browser.hide()
		
		
	def set_selection(self, *args):
		self.selected = True
		self.set_sensitives()
		

	def activate(self, view, path, column):
		'''Launch Editor for selected file/text'''
		
		def get_type_handler(mime_type, handler_type = 'MIME-types'):
			"""Lookup the ROX-defined run action for a given mime type.
			mime_type is an object returned by lookup().
			handler_type is a config directory leaf (e.g.'MIME-types')."""
		
			handler = basedir.load_first_config('rox.sourceforge.net', handler_type,
						 mime_type.media + '_' + mime_type.subtype)
			if not handler:
				# Fall back to the base handler if no subtype handler exists
				handler = basedir.load_first_config('rox.sourceforge.net', handler_type,
							 mime_type.media)
			return handler
					
		if not path: return True
		
		# Expand/Collapse search section rows
		if len(path) < 2:
			if self.view.row_expanded(path):
				self.view.collapse_row(path)
			else:
				self.view.expand_row(path, False)
			return True

		model = view.get_model()
		filename = model[path][0]

		event = gtk.get_current_event()

		if column is self.filename_column and event and event.type != gtk.gdk.KEY_PRESS:
			filer.show_file(filename)
		else:
			line = model[path][1]
			
			if len(OPT_EDIT_CMD.value):
				cmd = OPT_EDIT_CMD.value
				cmd = string.replace(cmd, '$File', filename)
				cmd = string.replace(cmd, '$Line', str(line))
				popen2.Popen4(cmd)
			else: #use the ROX defined text handler
				mime_type = rox.mime.lookup('text/plain')
				handler = get_type_handler(mime_type)
				handler_appdir = os.path.join(handler, 'AppRun')
				if os.path.isdir(handler) and os.path.isfile(handler_appdir):
					handler = handler_appdir
				if handler:
					popen2.Popen4('%s "%s"' % (handler, filename))
				else:
					rox.info(_('There is no run action defined for text files!'))
		return True


	def button_press(self, text, event):
		'''Popup menu handler'''
		if event.button != 3:
			return 0
		self.menu.popup(self, event)
		return 1


	def edit_options(self, *args):
		'''Show Options dialog'''
		rox.edit_options()


	def get_options(self):
		'''Get changed Options'''
		pass

