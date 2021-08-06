#!/usr/bin/env python

# Released under the GPL v3 by Jackson Yee (jackson@gotpossum.com)
# Copyright 2008

import pyv4l2

import sys
import datetime
import os

numpictures	=	0.0

def StreamCallback(d, b, p):
	global numpictures
	#print 'Got frame %s of size %s at %s' % (b.sequence, b.bytesused, int(p))
	filename = '%s/%09i.jpg' % (sys.argv[6], b.sequence)
	d.SaveJPEG(filename, 70, p)
	print 'Saved', filename
	numpictures += 1
	return True

def Run():
	global numpictures
	
	if len(sys.argv) < 7:
		print """recordpics.py device input pixelformat width height outputdir
		
Sample application to test pyv4l2 mmap functionality
	
	device		Video device: e.g. /dev/video0
	input		Input number: typically 0-8
	pixelformat	Format codes: e.g. RGB4
	width		Capture width
	height		Capture height
	outputdir	Directory to save files into
	"""
		return
		
	d = pyv4l2.Device(sys.argv[1])
	
	d.SetInput( int(sys.argv[2]) )
	
	d.GetFormat()
	d.SetStandard( d.standards['NTSC'] )
	d.SetField( d.fields['Interlaced'] )
	d.SetPixelFormat(sys.argv[3])
	d.SetResolution( int(sys.argv[4]), int(sys.argv[5])	 )
	
	i = 0
	starttime = datetime.datetime.now()	
	
	try:
		print 'Trying to create directory', sys.argv[6]
		os.mkdir(sys.argv[6])
	except Exception, e:
		print 'Could not create directory', e
	
	print 'Recording %s:%s with format %s at (%s, %s)' % (sys.argv[1],
		sys.argv[2],
		d.format.pixelformat,
		d.format.width,
		d.format.height,
		)
	
	try:
		d.SetupStreaming(5, StreamCallback)
	except KeyboardInterrupt:
		d.StreamOff()
		d.UnmapBuffers()
		print '\nKeyboard interrupt caught. Quitting...'
	
	endtime = datetime.datetime.now()
	elapsedtime	=	endtime - starttime
	seconds = elapsedtime.seconds + (float(elapsedtime.microseconds) / 1000000)
	
	print 'Saved %i pictures in %.2f seconds: %.2f fps' % (numpictures, 
		seconds, 
		numpictures / seconds)
	

if __name__ == '__main__':
	Run()
	
	

