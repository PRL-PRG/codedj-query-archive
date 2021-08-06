import os

def clean_dir(directory):
	# TODO: do it in a more python way
	os.popen('rm -rf ' + directory)
	#for dirpath, dirnames, filenames in os.walk(directory):
	#       for file in filenames:
	#               os.remove(dirpath + file)
	#       os.removedirs(dirpath)

def setup_dirs(conf):
        binary_dir = conf['TDir'] + os.sep + conf['CodeName'] + '-' + conf['Arch']
        source_dir = conf['TDir'] + os.sep + conf['CodeName'] + '-src'
        apt_dir = conf['AptTmp'] + os.sep + conf['CodeName'] + '-' + conf['Arch']

        if os.path.isdir(binary_dir):
                clean_dir(binary_dir)
        if os.path.isdir(source_dir):
                clean_dir(source_dir)
        if os.path.isdir(apt_dir):
                clean_dir(apt_dir)
