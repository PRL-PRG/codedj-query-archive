#!/usr/bin/env python

# Released under the GPL v3 by Jackson Yee (jackson@gotpossum.com)
# Copyright 2008
#
# Project site: http://code.google.com/p/python-video4linux2/

import pyv4l2

import sys
import datetime
import os
import sys

# =====================================================================
def Run():
	global numpictures

	parser = OptionParser()
	parser.add_option("-d", "--device", dest="device",
			  help="video device", default="/dev/video0" )
	parser.add_option("-i", "--input", dest="input",
			  help="Input number: typically 0-8", default="0" )
	parser.add_option("-p", "--pixelformat", dest="pixelformat",
			  help="Format codes", default="RGB4" )
	parser.add_option("-x", "--width", dest="width",
			  help="Capture width", default="640" )
	parser.add_option("-y", "--height", dest="height",
			  help="Capture height", default="480" )
	parser.add_option("-o", "--outputdir", dest="outputdir",
			  help="Directory to save files into", default="pics" )

	(options, args) = parser.parse_args()
		
	d = pyv4l2.Device(options.device)
	
	d.SetInput( int(options.input) )
	
	d.GetFormat()
	d.SetStandard( d.standards['NTSC'] )
	d.SetField( d.fields['Interlaced'] )
	d.SetPixelFormat(options.pixelformat)
	d.SetResolution( int(options.width), int(options.height) )
	
	i = 0
	starttime = datetime.datetime.now()	
	
	try:
		print 'Trying to create directory', outputdir
		os.mkdir(outputdir)
	except Exception, e:
		print 'Could not create directory', e
	
	print 'Recording %s:%s with format %s at (%s, %s)' % (options.device,
		options.input,
		d.format.pixelformat,
		d.format.width,
		d.format.height,
		)
	
	try:
		while True:
			d.Read()
			filename = '%s/%09i.jpg' % (options.outputdir, i)
			d.SaveJPEG(filename, 70)
			sys.stdout.write('.')
			sys.stdout.flush()
			i += 1
	except KeyboardInterrupt:
		print '\nKeyboard interrupt caught. Quitting...'
	
	endtime = datetime.datetime.now()
	elapsedtime	=	endtime - starttime
	seconds = elapsedtime.seconds + (float(elapsedtime.microseconds) / 1000000)
	
	print 'Saved %i pictures in %.2f seconds: %f fps' % (i, 
		seconds, 
		i / seconds)
	

if __name__ == '__main__':
	Run()
	
	

