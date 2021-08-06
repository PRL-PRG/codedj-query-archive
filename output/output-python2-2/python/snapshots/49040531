class PackagesTree:
	def __init__(self):
		self.binary = False
		self.source = False
		self.size = 0
		self.packages = {}
		self.provides = []

	def set_packages(self, packages):
		self.packages = packages

		# Fill the provides list 
		if self.packages:
			for key in self.packages.keys():
				self.__add_provided_packages__(self.packages[key])
			self.size += self.packages[key]['Size']

	def __add_provided_packages__(self, package):
		if package.has_key('Provides'):
			for package_provided in package['Provides']:
				self.provides.append(package_provided[0][0])

	def __remove_provided_packages__(self,package):
		if package.has_key('Provides'):
			for package_provided in package['Provides']:
				if package_provided[0][0] in self.provides:
					self.provides.remove(package_provided[0][0])

	def has_package(self, query_package):
		if query_package in self.packages.keys():
			return True
		else:
			if query_package in self.provides:
				return True
		return False

	def add_package(self, package_name, package_info):
		self.packages[package_name] = package_info
		self.size += self.packages[package_name]['Size']
		self.__add_provided_packages__(self.packages[package_name])

	def get_size(self):
		return self.size

	def get_sections(self):
		sections = []
		for key in self.packages.keys():
			if not self.packages[key]['Section'] in sections:
				sections.append(self.packages[key]['Section'])
		return sections

	def load_section(self, external_packages, section):
		for key in external_packages.keys():
			if external_packages[key]['Section'] == section:
				self.packages[key] = external_packages[key]
				self.size += self.packages[key]['Size']
				self.__add_provided_packages__(self.packages[key])

	def remove_section(self, section):
		for key in self.packages.keys():
			if self.packages[key]['Section'] == section:
				self.size -= self.packages[key]['Size']
				self.packages.pop(key)
				self.__remove_provided_packages__(self.packages[key])
				
	def show_info(self):
 		print 'Size: %s MB' % (float(self.size) / 2 ** 20)
		print 'Number of packages: %d' % self.packages.__len__()
