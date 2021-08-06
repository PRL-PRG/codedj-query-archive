#!/usr/bin/python2.4

from packagestree import *
from auxiliar import *
import apt_pkg
import urllib
import gzip
import os
import re

class AptLogic:
	def __init__(self, conf):
		self.conf = conf
		self.packages = {}
		self.sources = {}

		packages_file = self.__get_packages__()
		sources_file = self.__get_sources__()
		self.__parsePackages__(packages_file)
#		self.__parsePackages__(sources_file)

		# Devel info
		if self.conf['Devel']:
			f = open(self.conf['AptTmp'] + os.sep + 'python_vision', 'w')
			for pkg in self.packages:
				print >> f, "Package: ", pkg
				for field in self.packages[pkg].keys():
					print >> f, field + ": ", self.packages[pkg][field]
				print >> f, ""
			f.close()

	# TODO: consider download .bz2 or plain text Packages
	def __get_packages_file__(self, mirror, filename, component, ftppath):

		os.chdir(self.conf['AptTmp'])
		file_re = re.compile('^file\:')

		# Look if the package is local or external
		if not file_re.findall(mirror):
			raise AptLogicUnsupported('Unsupported operation: mirror is %s.\n  Currently, there is only support for local mirrors like:\n    file:///some/path/to/your/mirror' % mirror)

		url = mirror + "dists/" + self.conf['CodeName'] + "/" + component + "/" + ftppath
		gzip_fn = None

		filename = filename.replace('/','_')

		say('Downloading file into %s.gz file...' % filename)
		try:
			gzip_fn, info = urllib.urlretrieve(url, filename + '.gz')
			content_html_re = re.compile('.*Content-Type: text/html;.*', re.DOTALL)
			if content_html_re.match(info.__str__()):
				# Arrr, we haven't get the archive
				warn('\n  Missing %s\n' % url)


		except IOError, (errno, strerror):
			raise AptLogicIOError(strerror, url)

		os.path.isfile
		try:
			say('Decompressing %s file ...' % filename)
			gzipped_file = gzip.GzipFile(filename=gzip_fn)
			f = open(filename, 'w')
			for line in gzipped_file:
			    print >>f, line,
			f.close()
			gzipped_file.close()
		finally:
			if gzip_fn is not None:
				os.unlink(gzip_fn)
		return open(filename, 'r')

	def __get_sources__(self):
		"""
		Get a file called
			self.conf['CodeName'] + '_Sources'
		It contains all the info about source packages of all his 
		components (main, contrib, non-free, ...)
		"""

                # Let's go to the Apt temporal dir.
                os.chdir(self.conf['AptTmp'])

                # Define a global Source file, all the *_Sources files are going to be in this file.
                global_sources_file = open(self.conf['CodeName'] + '_Sources', 'w')

		# The main/debian-installer is in main, so remove it.
		components = self.conf['Components']
		if 'main/debian-installer' in components:
			components.remove('main/debian-installer')

                # For every component defined...
                for component in components:
                        # Download the Packages.gz file
                        file = self.__get_packages_file__(self.conf["Mirror"], \
                                        "%s_%s_Sources" % (self.conf['CodeName'], component), \
                                        component, "source" + "/Sources.gz")

                        # "cat" it into the global_packages_file
                        for line in file:
                                print >>global_sources_file, line,
                        file.close()

		global_sources_file.close()
		return open(self.conf['CodeName'] + '_Sources', 'r')
	

	def __get_packages__(self):
		"""
		Get a file called:
			 self.conf['CodeName'] + '_Packages'
		It contains all the info about packages of all his components
		(main, contrib, ...)
		"""

		# Let's go to the Apt temporal dir.
		os.chdir(self.conf['AptTmp'])

		# Define a global Package file, all the *_Packages files are going to be in this file.
		global_packages_file = open(self.conf['CodeName'] + '_Packages', 'w')

		# For every component defined...
		for component in self.conf['Components']:
			# Download the Packages.gz file
			file = self.__get_packages_file__(self.conf["Mirror"], \
					"%s_%s_Packages" % (self.conf['CodeName'], component), \
					component, "binary-" + self.conf['Arch'] + "/Packages.gz")

			# "cat" it into the global_packages_file
			for line in file:
				print >>global_packages_file, line,
			file.close()
	
		# Do the same with the ExtraMirrors	
		if self.conf['ExtraMirrors']:
			counter = 1
			for extra in self.conf['ExtraMirrors']:
				extra_packages_file = open(self.conf['CodeName'] + '_extra' + counter.__str__() + '_Packages', 'w')
				for component in self.conf['Components']:
					file = self.__get_packages_file__(extra, "%s_%s_extra%d_Packages" % (self.conf['CodeName'], component, counter), component, "binary-" + self.conf['Arch'] + "/Packages.gz")
					for line in file:
						print >>global_packages_file, line,
					file.close()
				counter += 1
				
		
		global_packages_file.close()
		return open(self.conf['CodeName'] + '_Packages', 'r')
	
	def __parsePackages__(self, f):
		"""
		Most info are raw info. This method parses the needed fields.
		"""

		p = apt_pkg.ParseTagFile(f)

		# Just load into memory the fields that are going to be useful
		while p.Step() == 1:
			pkg = p.Section['Package']

			self.packages[pkg] = {}

			for field in p.Section.keys():
				if field == 'Package':
					pass 
				elif ['Depends', 'Recommends', 'Suggests', 'Enhances', 'Pre-Depends', 'Conflicts', 'Provides'].count(field):
					value = p.Section.get(field, "")
					self.packages[pkg][field] = apt_pkg.ParseDepends(value)
				elif ['Size', 'Installed-Size'].count(field):
			    		value = p.Section.get(field, "0")
			    		self.packages[pkg][field] = int(value)
				elif field == 'Source':
					src = p.Section.get(field, pkg)
					idx = src.find('(')
					if idx != -1:
						src = src[:idx].strip()
					self.packages[pkg][field] = src
				elif field == 'Provides':
					self.packages[pkg]["Provides"] = apt_pkg.ParseDepends(p.Section.get("Provides", ""))
				else:
					self.packages[pkg][field] = p.Section.get(field, '')

	        f.close()

	def get_packages_info(self):
		pt = PackagesTree()
		pt.set_packages(self.packages)
		return pt 

class AptLogicIOError(Exception):
	"""
	This error indicates that the apt logic failed.
	"""
	def __init__(self, strerror, url):
		print '\n  ERROR. %s: %s\n' % (strerror[1], url)
		import sys
		sys.exit(1)

class AptLogicUnsupported(Exception):
	def __init__(self, value):
		print '\n  ERROR. %s\n' % value
		import sys
		sys.exit(1)	
