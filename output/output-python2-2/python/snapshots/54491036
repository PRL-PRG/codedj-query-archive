from _mp3 import *
from _flac import *
from _ogg import *
from _wav import *


TYPE_OGG = ['application/ogg', 'audio/x-vorbis+ogg']
TYPE_MP3 = ['audio/mpeg']
TYPE_FLAC = ['audio/x-flac']
TYPE_WAV = ['audio/x-wav']
TYPE_LIST = TYPE_OGG + TYPE_MP3 + TYPE_FLAC + TYPE_WAV


def get_info(song):
	if song.type in TYPE_MP3 and HAVE_MP3:
		_mp3.get_info(song)
	elif song.type in TYPE_OGG and HAVE_OGG:
		_ogg.get_info(song)
	elif song.type in TYPE_FLAC and HAVE_FLAC:
		_flac.get_info(song)
	elif song.type in TYPE_WAV and HAVE_WAV:
		_wav.get_info(song)
	else:
		raise ValueError, 'Unsupported file %s (type: %s).' % (song.filename, song.type)
		
		
def get_decoder(name, type, buffersize):
	if (type in TYPE_OGG and HAVE_OGG):
		return OGGDecoder(name, buffersize)
	elif (type in TYPE_MP3 and HAVE_MP3):
		return MP3Decoder(name, buffersize)
	elif (type in TYPE_FLAC and HAVE_FLAC):
		return FLACDecoder(name, buffersize)
	elif (type in TYPE_WAV):
		return WAVDecoder(name, buffersize)
	else:
		raise ValueError, 'Unsupported file %s (type: %s).' % (name, type)

