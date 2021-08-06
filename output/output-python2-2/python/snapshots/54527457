import os, string, time, re, stat

class Journal:
	entry_dir = '/home/www/htdocs/entries/'
	date_format = '%Y-%m-%d'
	time_format = '%H%M'
	datetime_format = '%A %Y-%m-%d %H:%M:%S %Z'
	id_format = date_format + '-' + time_format

	def __init__(s, req, form, user_cookie, session_cookie):
		s.req = req
		s.form = form
		s.user = user_cookie.value
		#s.session = session_cookie.value

	def write(s, data):
		s.req.write(data)

	def dispatch(s):
		s.write('<div id="content">')

		s.write('<div id="content_title">')
		s.write('<h2>Journal</h2>')
		s.write('</div><!--id="content-title"-->')

		# parse CGI params, defaults to entry list
		op = s.form.getfirst('op', 'recent')

		if op =='recent':
		# entry list: whole entries; defaults to most recent list
			starter = s.form.getfirst('start', 0)
			ender = s.form.getfirst('end', 15)
			s.display_list(int(starter), int(ender))
		elif op == 'index':
		# entry index: title and byline only; defaults to all entries
			starter = s.form.getfirst('start', 0)
			ender = s.form.getfirst('end', -1)
			s.display_list(int(starter), int(ender), as_index=True)
		elif op == 'entry':
		# single entry; defaults to most recent entry
			if s.form.has_key('id'):
				s.display_entry(s.form.getfirst('id'))
			else:
				s.display_entry(time.strftime(s.entry_id_format))
		elif op == 'edit':
		# entry edit; must give entry entry_id to edit
			if s.form.has_key('id'):
				s.display_edit(s.form.getfirst('id'))
			else:
				s.print_menu()
				s.print_error('cannot edit unspecified entry')
		elif op == 'preview':
		# preview edited entry; uses only form data, nothing is read from storage
			(entry_id, headers, data) = s.parse_form()
			s.display_preview(entry_id, headers, data)
		elif op == 'update':
		# actually update the edited entry; using form data, write it to storage
		# creates a new entry when needed
			(entry_id, headers, data) = s.parse_form()
			s.display_update(entry_id, headers, data)
		elif op == 'add':
		# add a new entry
			s.display_add(time.localtime())
		else:
		# unknown operation requested
			s.print_menu()
			s.print_error('unknown request: ' + os.environ['QUERY_STRING'])

		s.write('</div><!--id="content"-->')

		return


	def parse_form(s):
		"""parse form data for use in previews and updates"""

		# get the current time
		now = time.localtime()

		data = []
		headers = {}

		# get the entry entry_id
		entry_id = s.form.getfirst('id')

		# get the entry data, split on newline
		data = s.form.getfirst('data')
		data = data.split('\n')

		# note the size of the entry data
		headers['data_size'] = str(len(s.form.getfirst('data')))
		for x in ('file_size', 'data_offset'):
			headers[x] = ''

		# get various headers that might be emtpy
		headers['title'] = s.form.getfirst('title', '')
		headers['author'] = s.user

		headers['date'] = s.form.getfirst('date')
		headers['time'] = s.form.getfirst('time')

		return (entry_id, headers, data)


	# journal utility functions

	def get_file_list(s, start, end):
		"""return ASCII-descending list of entry filenames from start to end
		if end is less than zero, return from start to the end of the list"""

		# regexp for entry filenames
		entry_regexp = re.compile('^\d{4}-\d{2}-\d{2}-\d{4}$')

		file_list = []

		# get a list of filenames
		files = os.listdir(s.entry_dir)
		# make sure they match the entry filename format
		files = filter(entry_regexp.match, files)
		# sort them
		files.sort()
		# reverse them so the most recent is first
		files.reverse()

		# make sure no one is being funny with negative numbers
		if end < 0:
			end = None;

		# just grab the filenames that were requested
		for name in files[start:end]:
			file_list.append(name)

		return file_list


	def get_headers(s, entry):
		"""return dictionary of headers from file object"""

		headers = {}

		# remember the file size
		headers['file_size'] = os.stat(entry.name)[stat.ST_SIZE]

		# make sure we're at the beginning of the file
		entry.seek(0)

		# get the first line
		line = entry.readline()
		# parse headers until a blank line is found
		while (line != '\n'):# and (line != '\r\n'):
			k, v = string.split(line, ':')
			v = string.strip(v)
			# only save the header if a value is given
			if v != '':
				headers[k] = v
			# get the next line
			line = entry.readline()

		# remember where we are in the file, for get_data()
		headers['data_offset'] = entry.tell()

		# compute size of entry data
		headers['data_size'] = headers['file_size'] - headers['data_offset']

		return headers


	def get_data(s, entry, data_offset):
		"""return list of lines from file object after get_headers() hes been called"""

		data = []

		# skip the headers
		entry.seek(data_offset);

		# read the entry data
		data = entry.readlines()

		return data


	def open_entry(s, entry_id):
		"""attempts to open the file named by entry_id, returning a file object if successful"""
		try:
		# try to open specified entry file
			entry = file(s.entry_dir + entry_id, 'r')
		except IOError:
		# fail miserably if unopenable
			s.print_error('Unable to open entry file ' + entry_id + '.')
			return None

		return entry


	# print_* functions: called from other functions to display items

	def print_error(s, message):
		s.write('<div class="error">')
		s.write('<p>')
		s.write('ERROR: ' + message)
		s.write('</p>')
		s.write('</div><!--id="error"-->')
		return

	def print_alert(s, message):
		s.write('<div class="alert">')
		s.write('<p>')
		s.write('ALERT: ' + message)
		s.write('</p>')
		s.write('</div><!--id="alert"-->')
		return

	def print_menu(s, page = None):
		s.write('<div id="journal-menu">')
		print	'| '
		s.write('<a href="?p=journal;op=add" ')
		if page == 'add':  s.write('style="font-weight: bold;" ')
		s.write('title="Add a new entry">Add</a> |')
		s.write('<a href="?p=journal;op=recent" ')
		if page == 'recent':  s.write('style="font-weight: bold;" ')
		s.write('title="Display most recent entries">Recent</a> |')
		s.write('<a href="?p=journal;op=index" ')
		if page == 'index':  s.write('style="font-weight: bold;" ')
		s.write('title="Display and index of entries">Index</a> |')
		s.write('</div><!--id="journal-menu"-->')
		return

	def print_entry(s, entry_id, headers, data, options = True, show_meta = False, edit = False, preview=False):
		if not headers.has_key('title'):
			headers['title'] = entry_id
		if not headers.has_key('author'):
			headers['author'] = 'unknown'

		s.write('<div class="entry">')

		s.write('<h3 class="title">')
		s.write('<a href="?p=journal;op=entry;id=' + entry_id + '" title="Entry ID: ' + entry_id + '">' + headers['title'] + '</a>')
		s.write('</h3>')

		s.write('<p class="byline">')
		s.write('<span class="byline">')
		s.write('posted <span class="datestamp">' + headers['date'] + ' @ ' + headers['time'] + '</span>')
		s.write(' by <span class="author">' + headers['author'] + '</span>')
		s.write('</span>')
		if options or show_meta:
			s.write('<span class="entry-options">')
			if show_meta:
				s.write('| file size: ' + str(headers['file_size']) + ' bytes | data offset: ' + str(headers['data_offset']) + ' | data size: ')
				s.write(str(headers['data_size']) + ' bytes ')
			if options:
				s.write('| <a ')
				if edit: s.write('style="font-weight: bold;" ')
				s.write('href="?p=journal;op=edit;id=' + entry_id + '" title="Edit this entry">' + 'Edit</a>')
			s.write(' |</span>')
		s.write('</p>')

		# is this for a full listing or an index?
		if data != None:
			s.write('<div class="entry-data">')
			for line in data:
				s.write(line)
			s.write('</div><!--id="entry_data"-->')
		if not preview and not edit:
			s.write('<div id="journal_mtime">')
			s.write('<p>')
			s.write('Last updated ')
			s.write(time.strftime(s.datetime_format, time.localtime(os.stat(s.entry_dir + entry_id)[8])))
			s.write('</p>')
			s.write('</div>')

		s.write('</div><!--id="entry"-->')

		return

	def print_edit_form(s, entry_id, headers, data, previewed = False):
		s.write('<form action="index.py" method="get">')
		s.write('<div id="edit-form">')
		s.write('<input type="hidden" name="p" value="journal"/>')
		s.write('<input type="hidden" name="id" value="' + entry_id + '" />')
		s.write('<input type="hidden" name="date" value="' + headers['date'] + '" />')
		s.write('<input type="hidden" name="time" value="' + headers['time'] + '" />')
		s.write('<input type="hidden" name="file_size" value="' + str(headers['file_size']) + '" />')
		s.write('<input type="hidden" name="data_size" value="' + str(headers['data_size']) + '" />')
		s.write('<input type="hidden" name="data_offset" value="' + str(headers['data_offset']) + '" />')
		s.write('<div class="title-box"><p>')
		s.write('<input type="text" name="title" size="80" value="' + headers['title'] + '" />')
		s.write('</p></div>')
		s.write('<div class="byline"><p>')
		s.write('posted <span class="datestamp">' + headers['date'] + ' @ ' + headers['time'] + '</span>')
		s.write(' by <strong>%s</strong>' % s.user)
		#s.write('<input type="hidden" name="author" value="%s" />' % (s.user))
		s.write('</p></div>')
		s.write('<div class="data-box"><p>')
		s.write('<textarea name="data" rows="20" cols="80">')
		for l in data: s.write(l)
		s.write('</textarea>')
		s.write('</p></div>')
		s.write('<div class="buttons"><p>')
		s.write('<input type="submit" name="op" value="preview" />')
		s.write('&nbsp;&nbsp;')
		s.write('<input type="reset" value="reset" />')
		if previewed:
			s.write('&nbsp;&nbsp;')
			s.write('<input type="submit" name="op" value="update" />')
		s.write('</p></div>')
		s.write('</div><!--id="edit-form"-->')
		s.write('</form>')
		return


	# display_* functions: called from dispatch() to display page content
	def display_list(s, start, end, as_index = False):
		entries = {}

		if as_index:
			s.print_menu(page = 'index')
		else:
			s.print_menu(page = 'recent')

		files = s.get_file_list(start, end)
		# try to s.write(each file in the list
		for entry_id in files:
			entry = s.open_entry(entry_id)
			if entry == None: return
			headers = s.get_headers(entry)
			if as_index:
				data = None
			else:
				data = s.get_data(entry, headers['data_offset'])

			s.print_entry(entry_id, headers, data, options=False, show_meta=as_index);

		return

	def display_entry(s, entry_id):
		s.print_menu()
		entry = s.open_entry(entry_id)
		if entry == None: return
		headers = s.get_headers(entry)
		data = s.get_data(entry, headers['data_offset'])

		s.print_entry(entry_id, headers, data, options = True)

		return

	def display_edit(s, entry_id):
		s.print_menu()
		if entry_id:
			entry = s.open_entry(entry_id)
		if entry == None:
			return
		headers = s.get_headers(entry)
		data = s.get_data(entry, headers['data_offset'])
		s.print_alert('Editing entry ' + entry_id)
		s.print_entry(entry_id, headers, data, options=True, show_meta=True, edit = True)
		s.print_edit_form(entry_id, headers, data)
		return

	def display_preview(s, entry_id, headers, data):
		s.print_menu()
		s.print_alert('Previewing entry ' + entry_id)
		s.print_entry(entry_id, headers, data, options=True, show_meta=True, edit = True, preview=True)
		s.print_edit_form(entry_id, headers, data, previewed = True)
		return

	def display_update(s, entry_id, headers, data):
		s.print_menu()

		# do we have a good password
		pass_file = file(s.entry_dir + '.htpasswd', 'r')
		for line in pass_file.readlines():
			user, password = string.split(line, ':')
			password = string.strip(password)
			if user == s.user_cookie.value:
				break
		if password != headers['password']:
			s.print_error('bad password! no cheating!')
			s.print_alert('Previewing entry ' + entry_id)
			s.print_entry(entry_id, headers, data, options=True, show_meta=True, edit = True)
			s.print_edit_form(entry_id, headers, data, previewed = True)
			return

		# check for existing files and notify what's going to happen
		try:
			file(s.entry_dir + entry_id, 'r')
		except IOError:
			s.print_alert('Created new entry ' + entry_id)
		else:
			s.print_alert('Updated existing entry ' + entry_id)

		# create the file, overwriting the existing if needed
		entry = file(s.entry_dir + entry_id, 'w')

		# write the headers out
		for key in ['date', 'time', 'title', 'author']:
			entry.write(key + ': ' + headers[key] + '\n')
		#for (key, value) in headers.items():
		#	if key not in ['file_size', 'data_size', 'data_offset', 'password']:
		#		entry.write(key + ': ' + value + '\n')

		# seperator
		entry.write('\n')
		# write out data
		for line in data:
			entry.write(line)

		# close the new file to save the changes
		entry.close()

		# reread the new data, just to check
		entry = s.open_entry(entry_id)
		if entry == None:
			s.print_error("can't read new/updated entry")
			return
		headers = s.get_headers(entry)
		data = s.get_data(entry, headers['data_offset'])

		# show the updated/new entry
		s.print_entry(entry_id, headers, data)
		return

	def display_add(s, when):
		headers = {}
		data = []

		entry_id = time.strftime(s.id_format, when)
		headers['date'] = time.strftime(s.date_format, when)
		headers['time'] = time.strftime(s.time_format, when)
		for x in ['file_size', 'data_size', 'data_offset', 'title', 'author']:
			headers[x] = ''

		s.print_menu('add')

		s.print_alert('New Entry ' + entry_id)

		s.print_edit_form(entry_id, headers, data)
