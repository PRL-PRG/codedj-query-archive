import os
import sys

if len(sys.argv) > 1:
	path = sys.argv[1]
	l = os.listdir(path)

	for elem in l:
		os.rename(path + elem , path + os.path.splitext(elem)[0])
else:
	print 'Please input a path'
