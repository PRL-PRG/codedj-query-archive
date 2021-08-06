import os
import re
import stat
import string
import sys
import time

class Journal:
	config_dir = "/home/www/config"
	entry_dir = os.path.join(os.path.dirname(__file__), "..", "entries")
	# regexp for entry filenames
	entry_regexp = re.compile("^\d{4}-\d{2}-\d{2}-\d{4}$")
	date_format = "%Y-%m-%d"
	time_format = "%H%M"
	datetime_format = "%A %Y-%m-%d %H:%M:%S %Z"
	id_format = date_format + "-" + time_format

	def __init__(s, form, user_cookie, session_cookie, loader):
		s.form = form
		s.user = ""
		s.session = ""
		if user_cookie:
			s.user = user_cookie.value
		if session_cookie:
			s.session = session_cookie.value
		s.template = loader.load("journal.xml")
		s.output = []

	def dispatch(s):
		jop = s.form.getfirst("jop", "recent").lower()
		s.errors = []
		s.alerts = []
		s.page = jop
		s.edit = False
		s.new_entry = False
		s.preview = False
		s.entries = []
		s.show_mod_time = False

		if jop =="recent":
			# entry list: whole entries; defaults to most recent list
			starter = int(s.form.getfirst("start", 0))
			ender = int(s.form.getfirst("end", 10))
			s.display_list(starter, ender)
		elif jop == "index":
			# entry index: title and byline only; defaults to all entries
			starter = int(s.form.getfirst("start", 0))
			ender = int(s.form.getfirst("end", 0))
			s.display_list(starter, ender, fetch_data=False)
		elif jop == "entry":
			# single entry
			if "id" in s.form:
				entry_id = s.form.getfirst("id")
				headers, data = s.open_entry(entry_id)
				if headers:
					s.entries.append({
						"id": entry_id,
						"headers": headers,
						"data": data
					})
				else:
					s.errors.append("Could not open specified entry.")
					s.display_list()
			else:
				s.errors.append("No entry specified.")
				s.display_list()
		elif jop == "edit":
			# entry edit; must give entry entry_id to edit
			if "id" in s.form:
				entry_id = s.form.getfirst("id")
				headers, data = s.open_entry(entry_id)
				if headers:
					s.alerts.append("Editing entry " + entry_id)
					s.edit = True
					s.entries.append({
						"id": entry_id,
						"headers": headers,
						"data": data
					})
				else:
					s.errors.append("Could not open specified entry.")
					s.display_list()
			else:
				s.errors.append("No entry specified.")
				s.display_list()
		elif jop == "preview":
			# preview edited entry; uses only form data, nothing is read from storage
			entry_id, headers, data = s.parse_form()
			s.alerts.append("Previewing entry: %s" % entry_id)
			s.edit = True
			s.preview = True
			s.entries.append({
				"id": entry_id,
				"headers": headers,
				"data": data
			})
		elif jop == "update":
			# actually update the edited entry; using form data, write it to storage
			# creates a new entry when needed
			entry_id, headers, data = s.parse_form()
			s.display_update(entry_id, headers, data)
		elif jop == "add":
			headers = {}
			data = []
			now = time.localtime()
			entry_id = time.strftime(s.id_format, now)
			headers["date"] = time.strftime(s.date_format, now)
			headers["time"] = time.strftime(s.time_format, now)
			s.alerts.append("New Entry: %s" % entry_id)
			s.edit = True
			s.new_entry = True
			s.entries.append({
				"id": entry_id,
				"headers": headers,
				"data": data
			})
		else:
			s.errors.append("Unknown request: %s" % os.environ["QUERY_STRING"])

		stream = s.template.generate(
			errors=s.errors,
			alerts=s.alerts,
			page=s.page,
			edit=s.edit,
			new_entry=s.new_entry,
			preview=s.preview,
			user=s.user,
			session=s.session,
			entries = s.entries,
		)
		return stream.render().splitlines()

	def parse_form(s):
		"""parse form data for use in previews and updates"""
		headers = {}
		data = []
		# get the entry entry_id
		entry_id = s.form.getfirst("id")
		# get various headers that might be emtpy
		headers["title"] = s.form.getfirst("title", "")
		headers["author"] = s.user
		headers["date"] = s.form.getfirst("date")
		headers["time"] = s.form.getfirst("time")
		# get the entry data, split on newlines
		# NOTE: splitlines() seemed to eat all newlines. is FieldStorage multi-line aware?
		data = s.form.getfirst("data", "").split("\n")
		return entry_id, headers, data

	def open_entry(s, entry_id, fetch_data = True):
		"""attempts to open the file named by entry_id, returning a file object if successful"""
		try:
			# try to open specified entry file
			entryName = os.path.join(s.entry_dir, entry_id)
			entry = file(entryName, "r")
		except IOError:
			# fail miserably if unopenable
			#s.errors.append("Unable to open entry file " + entry_id + ".")
			return None, None
		headers = {}
		# get the first line
		line = entry.readline()
		# parse headers until a blank line is found
		while (line != "\n"):# and (line != "\r\n"):
			k, v = string.split(line, ":")
			v = string.strip(v)
			# only save the header if a value is given
			if v != "":
				headers[k] = v
			# get the next line
			line = entry.readline()
		data = []
		if fetch_data:
			data = entry.readlines()
		return headers, data

	def display_list(s, start=0, end=10, fetch_data = True):
		# get a list of filenames
		files = os.listdir(s.entry_dir)
		# make sure they match the entry filename format
		files = filter(s.entry_regexp.match, files)
		# sort & reverse them so most recent is first
		files.sort()
		files.reverse()
		if end == 0:
			end = None
		for entry_id in files[start:end]:
			headers, data = s.open_entry(entry_id, fetch_data)
			if headers:
				s.entries.append({
					"id": entry_id,
					"headers": headers,
					"data": data
				})
			else:
				s.errors.append("Could not open entry %s." % entry_id)

	def display_update(s, entry_id, headers, data):
		if s.session:
			# check for existing files and notify what's going to happen
			try:
				file(os.path.join(s.entry_dir, entry_id), "r")
			except IOError:
				s.alerts.append("Created new entry: %s" % entry_id)
			else:
				s.alerts.append("Updated existing entry: %s" % entry_id)
			# create the file, overwriting the existing if needed
			entry = file(os.path.join(s.entry_dir, entry_id), "w")
			# write the headers out
			for key, value in headers.items():
				entry.write("%s: %s\n" % (key, value))
			# seperator
			entry.write("\n")
			# write out data
			for line in data:
				entry.write(line)
			# close the new file to save the changes
			entry.close()
			# reread the new data, just to check
			new_headers, new_data = s.open_entry(entry_id)
			if not new_headers:
				s.errors.append("Can't read new/updated entry from storage: %s" % entry_id)
				s.edit = True
				s.preview = True
		else:
			s.errors.append("You must be logged in to Add or Update entries.")
			s.alerts.append("Previewing entry: %s" % entry_id)
			s.edit = True
			s.preview = True
		s.entries.append({
			"id": entry_id,
			"headers": headers,
			"data": data
		})
