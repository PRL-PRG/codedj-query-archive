"""

	Copyright 2005 Kenneth Hayber <ken@hayber.us>, All rights reserved.

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

try:
	import flac
	import flac.metadata as metadata
	import flac.decoder as fldecoder
	HAVE_FLAC = True
except:
	HAVE_FLAC = False
	print "No FLAC support!"

def get_info(song):
	#TODO
	pass

class FLACDecoder:
	"""
	A decoder to process FLAC sound files.  Utilizes the standard pyflac module
	Called from the Player class to read and decode FLAC files.
	"""
	def __init__(self, filename, buffersize):
		"""Initialize the decoder"""
		self.buffersize = buffersize
		self.filename = filename
		self.last_decode = 0

		chain = metadata.Chain()
		chain.read(filename)
		it = metadata.Iterator()
		it.init(chain)
		while 1:
			block = it.get_block()
			if block.type == metadata.STREAMINFO:
				streaminfo = block.data.stream_info
				self.total_samples = streaminfo.total_samples
				self.sample_rate = streaminfo.sample_rate
				self.nchannels = streaminfo.channels

			if not it.next():
				break

	def open(self):
		"""Open the file and prepare for decoding"""

		def metadata_callback(dec, block):	pass
		def error_callback(dec, status):	pass
		def write_callback(dec, buff, size):
			if self.buff == '':
				self.buff = buff[:-1]
			else:
				self.buff += buff[:-1]
			return fldecoder.FLAC__FILE_DECODER_OK

		# create a new file decoder
		mydec = fldecoder.FileDecoder()

		# set some properties
		mydec.set_md5_checking(False);
		mydec.set_filename(self.filename)
		mydec.set_metadata_respond_all()

		# set the callbacks
		mydec.set_write_callback(write_callback)
		mydec.set_error_callback(error_callback)
		mydec.set_metadata_callback(metadata_callback)

		# initialise, process metadata
		mydec.init()
		mydec.process_until_end_of_metadata()

		self.mydec = mydec

	def close(self):
		"""Close the file and do any needed cleanup"""
		self.mydec.finish()

	def length(self):
		"""Return the length of the file in seconds as an integer"""
		return self.total_samples / self.sample_rate

	def samplerate(self):
		"""Return the sample rate of the file in samples per second"""
		return self.sample_rate

	def channels(self):
		"""Return the number of channels in the file"""
		return self.nchannels

	def read(self):
		"""
		Read data from the file and decode to PCM data. Return a buffer
		of data and length, or (None, 0) at EOF
		"""
		self.buff = ''
		self.mydec.process_single()
		bytes = len(self.buff)
		if bytes:
			return (self.buff, bytes)
		else:
			return (None, 0)

	def tell(self):
		"""Return the current playback position in seconds"""
		curr_decode = self.mydec.get_decode_position()
		return int(curr_decode / (self.sample_rate * self.channels()))

	def seek(self, pos):
		"""Jump to pos as a percentage of the total length of the file"""
		pass #TODO

	def info(self):
		"""Display some FLAC information"""
		# create a chain
		chain = metadata.Chain()
		chain.read(self.filename)

		# get iterator, initialise
		it = metadata.Iterator()
		it.init(chain)

		cur_block = 0;
		while 1:
			block = it.get_block()
			# print some common fields
			print 'METADATA BLOCK #%d' % cur_block
			print '  type: %d (%s)' % (block.type, metadata.TypeString(block.type))
			print '  is_last: %d' % block.is_last
			print '  length: %d' % block.length
			cur_block += 1

			if block.type == metadata.STREAMINFO:
				# print STREAMINFO fields
				streaminfo = block.data.stream_info
				print '  minimum blocksize: %d' % streaminfo.min_blocksize
				print '  maximum blocksize: %d' % streaminfo.max_blocksize
				print '  minimum framesize: %d' % streaminfo.min_framesize
				print '  maximum framesize: %d' % streaminfo.max_framesize
				print '  sample_rate: %d' % streaminfo.sample_rate
				print '  channels: %d' % streaminfo.channels
				print '  bits-per-sample: %d' % streaminfo.bits_per_sample
				print '  total samples: %d' % streaminfo.total_samples
				#print '  md5sum: %s' % streaminfo.md5sum

			elif block.type == metadata.SEEKTABLE:
				# print SEEKTABLE fields
				seektable = block.data.seek_table
				print '  seek points: %d' % seektable.num_points
				for i in range(seektable.num_points):
					pt = seektable.points[i]
					print '    point %d: sample_number=%d, stream_offset=%d, frame_samples=%d' % (i, pt.sample_number, pt.stream_offset, pt.frame_samples)

			elif block.type == metadata.CUESHEET:
				# print CUESHEET
				cuesheet = block.data.cue_sheet
				print '  media catalog number: %s' % cuesheet.media_catalog_number
				print '  lead-in: %d' % cuesheet.lead_in
				print '  is CD: %d' % cuesheet.is_cd
				print '  number of tracks: %d' % cuesheet.num_tracks
				for i in range(cuesheet.num_tracks):
					tr = cuesheet.tracks[i]
					print '    track[%d]' % i
					print '      offset: %d' % tr.offset
					print '      number: %d' % ord(tr.number)
					print '      ISRC: %s' % tr.isrc
					if tr.type == 0:
						print '      type: AUDIO'
					else:
						print '      type: NON-AUDIO'
					if tr.pre_emphasis == 1:
						print '      pre-emphasis: true'
					else:
						print '      pre-emphasis: false'
					print '      number of index points: %d' % ord(tr.num_indices)
					for j in range(ord(tr.num_indices)):
						print '        index[%d]' % j
						print '          offset: %d' % tr.indices[j].offset
						print '          number: %d' % ord(tr.indices[j].number)

			elif block.type == metadata.VORBIS_COMMENT:
				# print vorbis tags
				comment = block.data.vorbis_comment
				print '  vendor string: %s' % comment.vendor_string
				print '  comments: %d' % comment.num_comments
				for i in range(comment.num_comments):
					print '    comment[%d]: %s' % (i, comment.comments[i])

			if not it.next():
				break

	
	