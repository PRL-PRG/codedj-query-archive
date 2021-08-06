def print_vars(conf):

	for field in conf.keys():
		print field, ' = ', conf[field]

def announce(text):
	size = (60 - text.__len__() - 2)
	size_ini = size / 2
	if (size % 2):
	        size_ini +=  1
	size_end = size / 2
	print "=" * size_ini , text, "=" * size_end
	
def say(text):
	print '  ', text

def warn(text):
	print ''
	print '  WARNING: ', text
	print ''

def flatten_depends(depends):
	flat_depends = []
	for dependency in depends:
		if dependency[0][1]:
			if not dependency[0][2]:
				print "ERROR"
			flat_depends.append(dependency[0][0] + ' (' + dependency[0][2] + ' ' + dependency[0][1] + ')')
		else:
			flat_depends.append(dependency[0][0])
	return ', '.join(flat_depends)
