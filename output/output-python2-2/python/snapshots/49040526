#!/usr/bin/python2.4

from libdig.config import *
from libdig.sweeper import *
from libdig.aptlogic import *
from libdig.packagestree import *
from libdig.media import *
from libdig.mediatype import *
from libdig.tasks import *
from libdig.auxiliar import *

if __name__ == '__main__':

#	dig_path = '/usr/share/debian-images-generator/'
	dig_path = '/home/carlospc/dig/deb-imgs-gen/trunk'
	tasks_path = os.sep.join([dig_path, 'data', 'tasks'])

	# TODO: Set config_file correctly
	config_file = os.getcwd() + os.sep + "dig.conf"
	# TODO: Set spec_file correctly
	spec_file = os.getcwd() + os.sep + "libdig/dig.spec"

	# Parse and load the config
	announce("Loading config...")	
	conf = Config(config_file, spec_file).get_config()
	
	# Clean and create dirs if needed
	announce("Cleaning temporal dirs...")
	setup_dirs(conf)

	# Apt Logic
	announce("Loading apt info...")
	apt = AptLogic(conf)
	available_packages = apt.get_packages_info()

	# devel info:
	if conf['Devel']:
		announce("devel info")
		print_vars(conf)

	# Let's play
	announce('Generating a set...')
	type = MediaType('cd')
	media_set = MediaSet(type, available_packages, conf)
	# Select the main task
	if conf['InstallerCd'] == 0:
		main_task = os.sep.join([tasks_path, 'Debian_' + conf['CodeName']]) 
	elif conf['InstallerCd'] == 1:
		main_task = os.sep.join([tasks_path, 'debian-installer-' + conf['CodeName']])
	elif conf['InstallerCd'] == 2:
		main_task = os.sep.join([tasks_path, 'debian-installer+kernel-' + conf['CodeName']]) 
	else:
		print '\n ERROR. InstallerCd has to be 0, 1 or 2' 
		import sys
		sys.exit(1)
	media_set.add_task(main_task)
	media_set.show_info()
	media_set.solidify()
	media_set.generate_images()
