import re

class generic:
	def get_percent(self, line):
		return -1

class cdda2wav:
	def __init__(self):
		self.re_progress = re.compile(".*([ 0-9][0-9])%")
		self.percent = 0

	def get_percent(self, line):
		x = self.re_progress.match(line)
		if x:
			self.percent = int(x.group(1))
		return self.percent

class cdparanoia:
	def __init__(self):
		self.start_sec = None
		self.end_sec = None
		self.re_start_sec = re.compile(".*from sector +([0-9]+)")
		self.re_end_sec = re.compile(".*to sector +([0-9]+)")
		self.re_progress = re.compile("##: -2 \[wrote\] @ ([0-9]+)")
		self.percent = 0

	def get_percent(self, line):
		if self.start_sec is None:
			x = self.re_start_sec.match(line)
			if x:
				self.start_sec = int(x.group(1))
		if self.end_sec is None:
			x = self.re_end_sec.match(line)
			if x:
				self.end_sec = int(x.group(1))
		if self.start_sec is not None and self.end_sec is not None:
			x = self.re_progress.match(line)
			if x:
				self.percent = 100 * (int(x.group(1))/1176-self.start_sec) / (self.end_sec-self.start_sec)
		return self.percent

class lame:
	def __init__(self):
		self.re_progress = re.compile(r"^[\s]+([0-9]+)/([0-9]+)")
		self.percent = 0

	def get_percent(self, line):
		x = self.re_progress.match(line)
		if x:
			self.percent = int(100 * (float(x.group(1)) / float(x.group(2))))
		return self.percent

class oggenc:
	def __init__(self):
		self.re_progress = re.compile('^.*\[[\s]*([.0-9]+)%\]')
		self.percent = 0

	def get_percent(self, line):
		x = self.re_progress.match(line)
		if x:
			self.percent = int(float(x.group(1)))
		return self.percent

codecs = {
	'cdda2wav': cdda2wav,
	'cdparanoia': cdparanoia,
	'lame': lame,
	'oggenc': oggenc,
	'generic': generic
}

def get_handler(text):
	for codec in codecs:
		if codec in text:
			return codecs[codec]()
	return codecs['generic']()
