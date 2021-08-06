#!/usr/bin/env python

# Released under the GPL v3 by Jackson Yee (jackson@gotpossum.com)
# Copyright 2008
#
# Project site: http://code.google.com/p/python-video4linux2/

"""
Sample application to test pyv4l2 read functionality
"""

# =====================================================================

import pyv4l2

import datetime
import os
import sys

from optparse import OptionParser


def Run():

	parser = OptionParser()
	parser.add_option("-d", "--device",
		default="/dev/video0", 
		help="video device [%default]", )
	parser.add_option("-i", "--input", 
		type="int", default=0,
		help="Input number: typically 0-8 [%default]" )
	parser.add_option("-p", "--pixelformat", 
		default="RGB4", 
		help="Format code [%default]", )
	parser.add_option("-x", "--width", 
		type="int", default="800",
		help="Capture width [%default]", )
	parser.add_option("-y", "--height", 
		type="int", default="600",
		help="Capture height [%default]", )
	parser.add_option("-o", "--outputdir", 
		default="test",
		help="Directory to save files into. [%default]", )

	(options, args) = parser.parse_args()
		
	d = pyv4l2.Device(options.device)
	
	d.SetInput( options.input )
	
	d.GetFormat()
	d.SetStandard( d.standards['NTSC'] )
	d.SetField( d.fields['Interlaced'] )
	d.SetPixelFormat(options.pixelformat)
	d.SetResolution( options.width, options.height )
	
	i = 0
	starttime = datetime.datetime.now()	
	
	try:
		print 'Trying to create directory', options.outputdir
		os.mkdir(options.outputdir)
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
			print 'Saved', filename
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
	
	

