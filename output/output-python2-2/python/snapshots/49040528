class MediaType:
	def __init__(self, desc):
		self.size = 0
		self.desc = 'Undefined'

		self.set_size(desc)

	def set_size(self, desc, size=None):
		if (desc == 'cd'):
			# 660602880 = 630 x 2^20
			self.size = 660602880
			self.desc = desc
			if size:
				raise MediaTypeBadArguments
		elif (desc == 'cd700'):
			# 713031680 = 680 x 2^20
			self.size = 713031680
			self.desc = desc
			if size:
				raise MediaTypeBadArguments
		elif (desc == 'cd800'):
			# 817889280 = 780 x 2^20
			self.size = 817889280
			self.desc = desc
			if size:
				raise MediaTypeBadArguments
		elif (desc == 'dvd'):
			# 4823449600 = 4600 x 2^20
			self.size = 4823449600
			self.desc = desc
			if size:
				raise MediaTypeBadArguments
		elif (desc == 'custom'):
			# If size is not defined or isn't a number, raise
			# an exception
			if not size or type(size) != type(0):
				raise MediaTypeBadArguments
			# We expect to receive the size in MB TODO: doc it!
			self.size = size * (2 ** 20)
			self.desc = desc
		else:
			raise MediaTypeUnknow(desc)

	def get_size(self):
		return self.size

	def set_custom_size(self, customsize):
		self.size = customsize
class MediaTypeError(Exception):
	pass

class MediaTypeUnknow(MediaTypeError):
	def __init__(self, value):
		MediaTypeError.__init__(
			self,
			'unknow media type "%s".' % value)

class MediaTypeBadArguments(MediaTypeError):
	def __init__(self):
		MediaTypeError.__init__(
			self,
			'bad arguments for media type')
