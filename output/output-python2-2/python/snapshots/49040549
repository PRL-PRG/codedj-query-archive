import sys
import os
from util.configobj.configobj import ConfigObj, ConfigObjError, flatten_errors
from util.configobj.validate import Validator, ValidateError

class Config:
	""" This class parse and load the config of debian-images-generator """
	def __init__(self, config_file, spec_file):
		""" Load config, chech for errors and set the variables to be
		accesibles """
		self.config = self.__load_config__(config_file, spec_file)

		if not self.config:
			print >> sys.stderr, "\n  You have to solve this problems before going on\n"
			sys.exit(1)

		self.__set_variables__()

	def __load_config__(self, config_file, spec_file):
		""" Load config from the config_file var.
		spec_file is used to chec that the config is valid.
		It have been added an extra check that configobj doesn't have: directory. """

		#TODO: It seems that configbobj has not function to raise an error when a value that it's not present at the spec is writen in the config file.
		try:
			local_config = ConfigObj(config_file, {'file_error': True, 'configspec':spec_file})
			fdict = {
				'directory': self.__check_directory__,
				'arch': self.__check_arch__,
				'arch_list': self.__check_arch_list__
			}
			config_validator = local_config.validate(Validator(fdict), preserve_errors = True)
			errors = False
			# If there are entries of incorrect type, show all the errors
			for entry in flatten_errors(local_config, config_validator):
				errors = True
				section_list, key, error = entry
				if key is not None:
					section_list.append(key)
				else:
					section_list.append('[missing section]')
				section_string = ','.join(section_list)
				if not error:
					error = 'Missing value or section'
				print >> sys.stderr, section_string, '=', error
		except ConfigObjError, e:
			for error in e.errors:
				print >> sys.stderr, error.message, '\n  "', error.line, '"'
			return None
		except IOError:
			# TODO: Point what you should do in this case
			print >> sys.stderr, "The config file", config_file, "doesn't exist!"
			return None
		except:
			print >> sys.stderr, "Unexpected error:", sys.exc_info()[0]
			return None

		if errors:
			return None
		else:
			return local_config

	def __set_variables__(self):
		""" Make accesible this variables to the rest of the world """

		if not self.config['Project']:
			self.config['Project'] = 'debian'

		if not self.config['DebianInstallerCodeName']:
			self.config['DebianInstallerCodeName'] = self.config['CodeName']

		# Check that the last character of the uri is a '/', if not, add it.
		lastchar = self.config['Mirror'].__len__() - 1
		if self.config['Mirror'][lastchar] != '/':
			self.config['Mirror'] += '/'

		if not self.config['Out']:
			self.config['Out'] = self.config['TDir'] + os.sep + 'out'
			if not os.path.isdir(self.config['Out']) and os.path.isdir(self.config['TDir']):
				os.mkdir(self.config['Out'])

		if not self.config['AptTmp']:
			self.config['AptTmp'] = self.config['TDir'] + os.sep + 'apt'
			if not os.path.isdir(self.config['AptTmp']) and os.path.isdir(self.config['TDir']):
				os.mkdir(self.config['AptTmp'])

		if not self.config['Arch']:
			self.config['Arch'] = os.popen('dpkg --print-architecture').read().strip()

		if self.config['MultiArch']:
			self.config['Arch'] = 'multi'

		if not self.config['DiskInfo']:
			self.config['DiskInfo'] = 'DiskInfo'

		if not self.config['KernelParams']:
			self.config['KernelParams'] = ''

		# FIXME
#		if not self.config['UdebInclude']:
#			self.config['UdebInclude'] = 'data' + os.sep + 'distro' + os.sep + self.config['CodeName'] + os.sep + self.config['Arch'] + '_udeb_include'

		# TODO: UdebExclude, BaseInclude, BaseExclude

	def __check_directory__(self, value):
		""" Check that the value passed is a directory, if not raise a
		VdtMissingDir exception """

		if not os.path.isdir(value):
			raise VdtMissingDir(value)
		return value

	def __check_arch__(self, value):
		""" Check that the value passed is a valid architecture, if not
		raise a VdtInvalidArch """

		if value not in ['i386', 'alpha', 'arm', 'hppa', 'ia64', 
			'm68k', 'mips', 'mipsel', 'powerpc', 's390', 'sparc',
			'amd64']:
			raise VdtInvalidArch(value)
		return value

	def __check_arch_list__(self, value):
		""" Check that the list passed has at least two elements and
		that the elements represents a valid architecture """

		if type(value) != type([]):
			raise VdtTooShortArchList(value)

		for arch in value:
			self.__check_arch__(arch)
		return value

	def get_config(self):
		""" Return config dictionary """
		return self.config

class VdtMissingDir(ValidateError):
	""" Extra Validator exception that it's raised when a directory doesn't
	exists """

	def __init__(self, value):
		ValidateError.__init__(self,'Directory "%s" doesn\'t exist ' % value)

class VdtInvalidArch(ValidateError):
	""" Extra Validator exception that it's raised when we haven't
	recognised the architecture """

	def __init__(self, value):
		ValidateError.__init__(self,'Invalid architecture: "%s"' % value)

class VdtTooShortArchList(ValidateError):
	""" Extra Validator exception that it's raised when the list of
	arches is empty or have only one element """

	def __init__(self, value):
		ValidateError.__init__(self,
		'At least, it have to contain two arches. Only contains "%s"' % value)
