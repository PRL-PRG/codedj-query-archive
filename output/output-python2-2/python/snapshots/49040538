from tasks import *
from packagestree import *
from doc import *
from boot import *
from auxiliar import * 
import os
import subprocess
import shutil

class MediaSet:
	def __init__(self, type, available_packages, conf):
		self.type = type
		self.available_packages = available_packages
		self.conf = conf
		self.pieces = []
		self.unknown_packages = []

	def add_task(self, task):
		"""
		Add all the packages into the MediaElements.
		Return a list with the unkown packages.
		"""
		t = Task(task, self.conf['ForceNonUsOnCD1'])
		unknown_packages = []
		unknown_packages = t.remove_unknown_elements(self.available_packages, self.conf['Arch'])
		unknown_packages.extend(t.remove_unknown_elements(self.available_packages, 'all'))

		for unknown_package in unknown_packages:
			if not unknown_package in self.unknown_packages:
				self.unknown_packages.append(unknown_package)

		if self.unknown_packages:
			if not self.conf['Force']:
				raise MediaUnknownPackages, self.unknown_packages
			else:
				print "\n  There is unknown packages but Force = True, so continue...\n"

		whole_task = []
		whole_task.extend(t.include['all'])
		whole_task.extend(t.include[self.conf['Arch']])

		for package in whole_task:
			self.add_package(package)
	
	def add_package(self, package):
		"""
		Add the package in the first MediaElement that there is 
		enough space, if there isn't any, add a new MediaElement.
		"""
		archived = False
		i = 0

		if not self.pieces:
			self.add_element()

		while not archived:
			if self.available_packages.packages.has_key(package):
				if self.available_packages.packages[package]['Size'] < self.pieces[i].get_free_space():
					self.pieces[i].add_package(package, self.available_packages.packages[package])
					archived = True
				else:
					if self.pieces.__len__() <= i + 1:
						self.add_element()
					i += 1
			else:
				# We call "provider" each available package...
				for provider in self.available_packages.packages.keys():
					# For each package provided, if is the 
					# package we are looking for, set 
					# "package" with "provider" and break!
					if self.available_packages.packages[provider].has_key('Provides'): 
						for package_provided in self.available_packages.packages[provider]['Provides']:
							if package == package_provided[0][0]:
								package = provider
								break
						# If the package is equal to the provider
						# it means that we have found the real provider
						# so break to archive the package.
						if package == provider:
							break

	def add_element(self):
		media_element = MediaElement(self.type, self.pieces.__len__() + 1, self.conf)
		self.pieces.append(media_element)

	def solidify(self):
		for piece in self.pieces:
			piece.solidify()

	def generate_images(self):
		for piece in self.pieces:
			piece.generate_image()

	def show_info(self):

		print 'Number of elements: ', self.pieces.__len__()
		print '======================='

		i = 1
		for piece in self.pieces:
			print 'Piece ', i
			print '========'
			piece.show_info()
			i += 1 

class MediaElement:
	def __init__(self, type, media_number, conf):
		self.packages = PackagesTree()
		self.boot = None 
		self.doc = Doc(conf)
		self.extra = None
		self.type = type
		self.media_number = media_number.__str__()
		self.conf = conf

	def get_free_space(self):
		return self.type.get_size() - self.get_used_space() 

	def get_used_space(self):
		total = 0

		if self.packages:
			total += self.packages.get_size()

		if self.boot:
			total += self.boot.get_size()

		if self.doc:
			total += self.doc.get_size()

#		if self.extra:
#			total += self.extra.get_size()

		return total

	def add_package(self, package_name, package_info):
		self.packages.add_package(package_name, package_info)

	def __update_packages__(self, package, category, dists_dir):
		# Packages filename	
		packages_fn = os.sep.join([dists_dir, self.conf['CodeName'], category, 'binary-' + self.conf['Arch'], 'Packages'])

		# Create the directory if doesn't exists yet
		if not os.path.isdir(os.path.dirname(packages_fn)):
			os.makedirs(os.path.dirname(packages_fn))

		# Open the Packages file in append mode
		packages_f = open(packages_fn, 'a')

		# Order of the fields
		fields_priority = ['Version', 'Priority', 'Section', 'Essential', 'Maintainer', 'Pre-Depends', 'Depends', 'Recommends', 'Suggests', 'Conflicts', 'Provides', 'Replaces', 'Architecture', 'Filename', 'Size', 'MD5sum', 'Description', 'Enhances']

		# Let's go
		# 1. Start with the 'Packages' field
		print >> packages_f, 'Package: ' + package

		# 2. Continue with all the fields that we know
		for field in fields_priority:
			if self.packages.packages[package].has_key(field):
				# If 'field' is equal to one of the relationship fields call flatten_depends
				if ['Depends', 'Recommends', 'Suggests', 'Enhances', 'Pre-Depends', 'Conflicts', 'Provides'].count(field):
					print >> packages_f, field + ': ' + flatten_depends(self.packages.packages[package][field])
				# Else print the field as is.
				else:
					print >> packages_f, field + ': ', self.packages.packages[package][field]

		# 3. Add all the fields that we don't know about
		for field in self.packages.packages[package].keys():
			if not fields_priority.count(field):
				print >> packages_f, field + ': ', self.packages.packages[package][field]
		# 4. Add an extra line
		print >> packages_f, ''

		packages_f.close()

	def __get_category__(self, package):
		# Regular expresion to find a '/' in the 'Section' field
		category_re = re.compile('(\w+)/.*')
		# Let's get the category of the package having a look the 'Section' field
		category_tmp = category_re.findall(self.packages.packages[package]['Section'])
		if category_tmp:
			return category_tmp[0]
		else:
			return 'main'

	def __get_path_under_pool__(self, package, category):
		# Have a look for the Source field
		source = None
		if self.packages.packages[package].has_key('Source'):
			source = self.packages.packages[package]['Source']
		# Fill real_source variable with the real source package name.
		if source:
			real_source = source
		else:
			real_source = package
		# Regular expresion to check if a package start with 'lib' string
		lib_re = re.compile('^lib(\w).+')
		# Check if we are working with a lib?*
		char_tmp = lib_re.findall(real_source)
		if char_tmp:
			char = 'lib' + char_tmp[0]
		else:
			char = real_source[0]
		return os.sep.join([category, char, real_source])

	def solidify(self):
		print 'Copying packages of the media %s...' % self.media_number

		# Directories
		root_dir = os.sep.join([self.conf['TDir'], self.conf['CodeName'] + '-' + self.conf['Arch']])
		media_dir = os.sep.join([root_dir, 'CD' + self.media_number])
		pool_dir = os.sep.join([media_dir, 'pool'])
		dists_dir = os.sep.join([media_dir, 'dists'])
		disk_dir = os.sep.join([media_dir, '.disk'])
		boot_dir = os.sep.join([root_dir, 'boot' + self.media_number])
		isolinux_dir = os.sep.join([boot_dir, 'isolinux'])
		install_dir = os.sep.join([media_dir, 'install'])

		# FIXME this path shouldn't be absolute
		dig_dir = '/home/carlospc/dig/deb-imgs-gen/trunk'
		distro_dir = os.sep.join([dig_dir, 'data', 'distro', 'sarge'])

		# Regular expresions
		fn_re = re.compile('.*/(.*\.u?deb)')
		file_re = re.compile('^file\:')

		# Create dirs
		if not os.path.isdir(pool_dir):
			os.makedirs(pool_dir)
		if not os.path.isdir(dists_dir):
			os.makedirs(dists_dir)
		if not os.path.isdir(disk_dir):
			os.mkdir(disk_dir)

		# Check that all packages required by debootstrap are included
		# and create .disk/base_installable if yes
		# Also create .disk/base_components
		if self.media_number == '1':
			debootstrap_cmd = ['debootstrap', '--arch', \
					   self.conf['Arch'], \
					   '--print-debs', \
					   self.conf['CodeName'], \
					   os.sep.join([self.conf['TDir'], \
							'debootstrap.tmp']), \
					   self.conf['Mirror']]

			debootstrap_stdout_f = open(os.path.join(self.conf['TDir'], 'debootstrap_stdout'), 'w')
			debootstrap_stderr_f = open(os.path.join(self.conf['TDir'], 'debootstrap_stderr'), 'w')
			debootstrap_retval = subprocess.call(debootstrap_cmd, \
							     0, None, None, \
							     debootstrap_stdout_f, \
							     debootstrap_stderr_f)
			debootstrap_stdout_f.close()
			debootstrap_stderr_f.close()

			# Success
			if not debootstrap_retval:
				missing_packages = []
				debootstrap_stdout_f = open(os.path.join(self.conf['TDir'], 'debootstrap_stdout'), 'r')
				for package in debootstrap_stdout_f.read().strip().split(' '):
					if not self.packages.has_package(package):
						missing_packages.append(package)
				debootstrap_stdout_f.close()
				if missing_packages:
					print 'Missing debootstrap-required packages:'
					print ' '.join(missing_packages)
				else:
					# Touch .disk/base_installable
					base_installable_f = open(os.sep.join([media_dir, '.disk', 'base_installable']),'w')
					base_installable_f.close()
		

			base_components_f = open(os.sep.join([media_dir, '.disk', 'base_components']), 'w')

			base_components_f.write('main')
			base_components_f.close()

			if self.conf['UdebInclude']:
				if os.path.isfile(self.conf['UdebInclude']):
					shutil.copyfile(self.conf['UdebInclude'], os.sep.join([media_dir, '.disk', 'udeb_include']))
				else:
					raise MediaMissingUdebInclude(self.conf['UdebInclude'])

			if self.conf['UdebExclude']:
				if os.path.isfile(self.conf['UdebExclude']):
					shutil.copyfile(self.conf['UdebExclude'], os.sep.join([media_dir, '.disk', 'udeb_exclude']))
				else:
					raise MediaMissingUdebInclude(self.conf['UdebExclude'])

			if self.conf['BaseInclude']:
				if os.path.isfile(os.sep.join([dig_dir, 'data', 'distro', self.conf['CodeName'], 'base_include'])):
					shutil.copyfile(os.sep.join([dig_dir, 'data', 'distro', self.conf['CodeName'], 'base_include']), os.sep.join([media_dir, '.disk', 'base_include']))
				else:
					raise MediaMissingBaseInclude(self.conf['BaseInclude'])

			if self.conf['BaseExclude']:
				if os.path.isfile(os.sep.join([dig_dir, 'data', 'distro', self.conf['CodeName'], 'base_exclude'])):
					shutil.copyfile(os.sep.join([dig_dir, 'data', 'distro', self.conf['CodeName'], 'base_exclude']), os.sep.join([media_dir, '.disk', 'base_include']))
				else:
					raise MediaMissingBaseExclude(self.conf['BaseExclude'])
			

		if file_re.findall(self.conf['Mirror']):
			mirror_path = self.conf['Mirror'][7:]
		else:
			raise MediaUnsupportedUrl(self.conf['Mirror'])

		for package in self.packages.packages.keys():
			# Get the category of this package 
			category = self.__get_category__(package)
			# Get the path under the pool directory
			path_under_pool = self.__get_path_under_pool__(package, category)
			# The absolute path
			dest_dir = os.sep.join([pool_dir, path_under_pool])
			if not os.path.isdir(dest_dir):
				os.makedirs(dest_dir)

			# TODO: it could be .diff.gz, .tar.gz, .dsc ...

			fn = fn_re.findall(self.packages.packages[package]['Filename'])[0]
			src =  os.sep.join([mirror_path, self.packages.packages[package]['Filename']])
			dst = os.sep.join([dest_dir, fn])
			
			if not os.path.isfile(src):
				raise MediaMissingPackage(src)
		
			# Do a hard link	
			os.link(src, dst)

			# Update the Packages file
			self.__update_packages__(package, category, dists_dir)

		self.__generate_dists__(media_dir)

		# Create a symbolic link with the name of the project
		os.symlink('.', os.sep.join([media_dir, self.conf['Project']]))

		if not os.path.isdir(os.sep.join([media_dir, '.disk'])):
			os.mkdir(os.sep.join([media_dir, '.disk']))

		# fill .disk/info file
		info_f = open(os.sep.join([media_dir, '.disk', 'info']), 'w')
		info_f.write(self.conf['DiskInfo'])
		info_f.close()

		# Create Boot object
		self.boot = Boot(self.conf, dig_dir, mirror_path, root_dir, isolinux_dir, install_dir, distro_dir)

		# Include boot images
		self.boot.include_images()
		
		if self.media_number == '1':
			# Make bootable
			self.boot.make_bootable(self.media_number)

			# TODO autorun.bat
		
			self.doc.include_doc(media_dir, mirror_path, dig_dir)

	def __generate_dists__(self, media_dir):

		print "Scaning packages... "

		command = 'dpkg-scanpackages -a ' + self.conf['Arch'] + ' ' + media_dir + ' /dev/null' 
		std = os.popen3(command)
		stdout = std[1]
		stderr = std[2]

	def generate_image(self):
		arch = self.conf['Arch']
		# from debian-cd: set_mkisofs_opts
		if arch == 'i386' or arch == 'amd64' or arch == 'alpha':
			opts=' -J '
		else:
			opts=''

		volumeid = 'Testing'
		output = os.sep.join([self.conf['Out'], 'image' + self.media_number  + '.iso'])

		#args = '-r -V "' + volumeid +'" -o ' + output + ' -cache-inodes '+ opts + ' -l -joliet-long -hide-rr-moved'
		args = '-r -V "' + volumeid +'" -o ' + output + ' -cache-inodes -J -b isolinux/isolinux.bin -c isolinux/boot.cat -no-emul-boot -boot-load-size 4 -boot-info-table ' + os.sep.join([self.conf['TDir'], self.conf['CodeName'] + '-' + self.conf['Arch'], 'boot' + self.media_number])
#'-cache-inodes -J -b isolinux/isolinux.bin -c isolinux/boot.cat -no-emul-boot -boot-load-size 4 -boot-info-table ' + self.conf['TDir'] os.sep + self.conf['CodeName'] + '-' + self.conf['Arch'] os.sep + 'boot' + self.media_number
		target = os.sep.join([self.conf['TDir'], self.conf['CodeName'] + '-' + self.conf['Arch'], 'CD' + self.media_number])

		command = 'mkisofs %s %s' % (args, target)
		print 'Executing %s' % command
		os.popen(command)

	def set_packages(self, packages):
		self.packages = packages

	def get_packages(self):
		return self.packages

	def set_boot(self, boot):
		self.boot = boot

	def get_boot(self):
		return self.boot

	def set_doc(self, doc):
		self.doc = doc

	def get_doc(self):
		return self.doc

	def set_extra(self, extra):
		self.extra = extra

	def get_extra(self):
		return self.extra

	def show_info(self):
		self.packages.show_info()


class MediaError(Exception):
	pass

class MediaUnknownPackages(MediaError):
	def __init__(self, value):
		list_unknown = ''
		for unknown in value:
			list_unknown += '  ' + unknown + '\n'
		MediaError.__init__(self, '\n\n  Unknown packages:\n    %s\n\nTry to fix it or add "Force = True" at the config file\nif you know what you are doing\n' % list_unknown)

class MediaUnsupportedUrl(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Unsupported url:\n    %s\n' % value)

class MediaMissingPackage(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Missing package:\n    %s\n' % value)

class MediaMissingUdebInclude(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Missing UdebInclude file:\n    %s\n' % value)

class MediaMissingUdebExclude(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Missing UdebExclude file:\n    %s\n' % value)

class MediaMissingBaseInclude(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Missing BaseInclude file:\n    %s\n' % value)

class MediaMissingBaseExclude(MediaError):
	def __init__(self, value):
		MediaError.__init__(self, '\n\n  Missing BaseExclude file:\n    %s\n' % value)
