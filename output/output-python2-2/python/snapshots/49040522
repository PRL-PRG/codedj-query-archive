from auxiliar import say
import shutil

class Doc:
	def __init__(self, conf):
		"""
		The purpose of this class is to fill all the documentation
		that has to have a Debian media.
		"""

		self.conf = conf

	# TODO
	def get_size(self):
		"""
		Return the size of all the files that are documentation.
		Mainly the doc/ dir and the 'README'´s files..
		"""

		return 0

	def __include_faq__(self):
		"""
		Look for html.tar.gz faq and uncompress it under the 
		self.media_doc_dir
		"""

		import os
		import tarfile

		# Vars
		faq_dir = os.sep.join([self.media_doc_dir, 'FAQ'])
		html_faq_dir = os.sep.join([faq_dir, 'html'])

		# Make the html directory and uncompress the faq if exists
		os.mkdir(html_faq_dir)
		if os.path.isfile(os.sep.join([faq_dir, 'debian-faq.en.html.tar.gz'])):
			tar = tarfile.open(os.sep.join([faq_dir, 'debian-faq.en.html.tar.gz']), 'r:gz')
			for tarinfo in tar:
				tar.extract(tarinfo, html_faq_dir)
			tar.close()
		elif os.path.isfile(os.sep.join([faq_dir, 'debian-faq.html.tar.gz'])):
			tar = tarfile.open(os.sep.join([faq_dir, 'debian-faq.html.tar.gz']), 'r:gz')
			for tarinfo in tar:
				tar.extract(tarinfo, html_faq_dir)
			tar.close()
		else:
			warn('no html compressed doc!')

	def __include_doc_dir__(self):
		"""
		Copy the doc/ directory from the mirror removing the unnecessary data.
		"""
		import os
		import re

		# If the doc/ dir exists, retreive it to the cd directory.
		if os.path.isdir(self.mirror_doc_dir):
			shutil.copytree(self.mirror_doc_dir, self.media_doc_dir)


			debian_keyring = os.sep.join([self.media_doc_dir, 'debian-keyring.tar.gz'])
			if os.path.isfile(debian_keyring):
				os.unlink(debian_keyring)

			# Remove all the dedication-* files that aren't from the current debian version 
			dedication_re = re.compile('^dedication-.*')
			debversion_re = re.compile('.*' + self.conf['DebVersion'].__str__() + '.*')
			for curdir, dirs, files in os.walk(self.media_doc_dir):
				for file in files:
					if dedication_re.match(file):
						if not debversion_re.match(file):
							os.unlink(os.sep.join([curdir, file]))

			# FAQ stuff
			self.__include_faq__()

		else:
			raise DocNoResourcesFound(self.mirror_doc_dir)

	def __include_readmes__(self):
		"""
		Include all the README.* files
		"""

		import os
		import re

		# Copy all the README.* files under self.mirror_path
		readme_re = re.compile('README.*')
		for element in os.listdir(self.mirror_path):
			if readme_re.match(element):
				shutil.copyfile(os.sep.join([self.mirror_path, element]), \
						os.sep.join([self.media_dir, element]))

		main_version_re = re.compile('[ _r].*')
		main_version = main_version_re.sub('', self.conf['DebVersion'].__str__())

		# If dedication-VERSION.txt exists, move it into dedication.txt
		dedication_with_version = os.sep.join([self.media_doc_dir, 'dedication-' + main_version + '.txt'])
		if os.path.isfile(dedication_with_version):
			shutil.move(dedication_with_version, \
				    os.sep.join([self.media_dir, 'dedication.txt']))
			os.symlink(os.sep.join(['..', 'dedication.txt']), \
				   dedication_with_version)

		readmes = [os.sep.join([self.media_dir, 'README']),
				os.sep.join([self.media_dir, 'README.1ST']),
				os.sep.join([self.media_dir, 'README.CD-manufacture']),
				os.sep.join([self.media_dir, 'README.multicd']),
				os.sep.join([self.media_dir, 'README.pgp']),
				os.sep.join([self.media_dir, 'README.non-US'])]
		
		# Remove this readmes
		for readme in readmes:
			if os.path.isfile(readme):
				os.unlink(readme)

		readme_html_in_fn = os.sep.join([self.codename_data_dir, 'README.html.in'])
		if os.path.isfile(readme_html_in_fn):
#			cpp_cmd = ['cpp', '-traditional', '-undef', '-P', '-C', \
#				   '-Wall', '-nostdinc', '-I' + self.media_dir, \
#				   '-D', 'OMIT_MANUAL="0"', '-D', \
#				   'OMIT_RELEASE_NOTES="0"', '-D', 'OUTPUTtext',
#				   readme_html_in_fn, '|', 'sed', '-e', '\'s/%%.//g\'']
#			cpp_stdout_f = open(os.sep.join([self.conf['TDir'], \
#							'cpp_stdout']), 'w')
#			cpp_stderr_f = open(os.sep.join([self.conf['TDir'], \
#							'cpp_stderr']), 'w')
#			cpp_retval = subprocess.call(cpp_cmd, 0, None, None, \
#						cpp_stdout_f, cpp_stderr_f)


			# FIXME Problems, it's waiting README.diskdefines
			cpp_cmd = ['cpp', '-traditional', '-undef', '-P', '-C', \
				   '-Wall', '-nostdinc', '-I' + self.media_dir, \
				   '-D', 'OMIT_MANUAL="0"', '-D', \
				   'OMIT_RELEASE_NOTES="0"', '-D', 'OUTPUTtext',
				   readme_html_in_fn, '|', 'sed', '-e', \
				   '\'s/%%.//g\'', '>', \
				   os.sep.join([self.media_dir, 'README.html'])]
			os.popen(' '.join(cpp_cmd))

			lynx_cmd = ['lynx', '-dump', '-force_html', \
				    os.sep.join([self.media_dir, 'README.html']), '|', 
				    'todos', '>', \
				    os.sep.join([self.media_dir, 'README.txt'])]
			os.popen(' '.join(lynx_cmd))

			cpp_cmd[13] = 'OUTPUThtml'
			os.popen(' '.join(cpp_cmd))

			shutil.copytree(os.sep.join([self.dig_dir, 'data', 'pics']), os.sep.join([self.media_dir, 'pics']))

			if os.path.isfile(os.sep.join([self.mirror_path, 'dists', self.conf['CodeName'], 'main', 'Release-Notes'])):
				shutil.copyfile(os.sep.join([self.mirror_path, \
					        	'dists', \
							self.conf['CodeName'], \
							'main', 'Release-Notes']), \
						os.sep.join([self.media_dir, \
							'Release-Notes']))

		if self.conf['InstallerCd'] == 0:
			contents_arch_gz = os.sep.join([self.mirror_path, 'dists', self.conf['CodeName'], 'Contents-' + self.conf['Arch'] + '.gz'])
			if os.sep.isfile(contents_arch_gz):
				shutil.copyfile(contents_arch_gz, os.sep.join([self.media_dir, 'dists', self.conf['CodeName'], 'Contents-' + self.conf['Arch'] + '.gz']))
			else:
				print 'WARNING: there\'s no Contents-%s.gz file for %s !' % (self.conf['Arch'], self.conf['CodeName'])


		readme_arch_fn = 'README.' + self.conf['Arch']
		readme_arch = os.sep.join([self.codename_data_dir, readme_arch_fn])
		if os.path.isfile(readme_arch):
			shutil.copyfile(readme_arch, os.sep.join([self.media_dir, readme_arch_fn]))

		readme_first_fn = 'README.1ST'
		readme_first_cd = os.sep.join([self.media_dir, readme_first_fn])

		readme_first_arch_fn = readme_first_fn + '.' + self.conf['Arch']
		readme_first_arch = os.sep.join([self.codename_data_dir, readme_first_arch_fn])
		if os.path.isfile(readme_first_arch):
			shutil.copyfile(readme_first_arch, readme_first_cd)
			todos_cmd = 'todos ' + readme_first_cd
			os.popen(todos_cmd)

		readme_multicd_fn = 'README.multicd'
		readme_multicd = os.sep.join([self.codename_data_dir, readme_multicd_fn])
		readme_multicd_cd = os.sep.join([self.media_dir, readme_multicd_fn])
		if os.path.isfile(readme_multicd):
			shutil.copyfile(readme_multicd, readme_multicd_cd)

	def __include_contents_arch__(self):
		"""
		Include Content-Arch.gz file 
		"""
		pass

		# TODO Translate it!
		#        if [ -n "$NONUS" ]
		#        then
		#            if [ -e $NONUS/dists/$CODENAME/non-US/Contents-$ARCH.gz ]; then
		#                 cp -pf $NONUS/dists/$CODENAME/non-US/Contents-$ARCH.gz \
		#                    $dir/dists/$CODENAME/non-US/
		#            else                 echo "WARNING: there's no Content-$ARCH.gz file for $CODENAME/non-US !"
		#            fi
		#        fi

	def include_doc(self, media_dir, mirror_path, dig_dir):
		"""
		Include documentation. Call all the methods needed.
		"""
		
		import os

		# Save the params
		self.media_dir = media_dir
		self.mirror_path = mirror_path
		self.dig_dir = dig_dir

		# Helpful vars
		self.codename_data_dir = os.sep.join([self.dig_dir, 'data', 'distro', self.conf['CodeName']])
		self.mirror_doc_dir = os.sep.join([self.mirror_path, 'doc'])
		self.media_doc_dir = os.sep.join([self.media_dir, 'doc'])

		say('Adding the documentation (bin)...')
		self.__include_doc_dir__()
		self.__include_readmes__()

class DocError(Exception):
        pass

class DocNoResourcesFound(DocError):
        def __init__(self, value):
                MediaError.__init__(self, '\n\n  Missing \'doc\/\' directory:\n    %s\n' % value)
