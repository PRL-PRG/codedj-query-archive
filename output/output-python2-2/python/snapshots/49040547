# Kind of tasks:
#  o ARCH depend
#  o ARCH independent
#  o include other tasks
#  o exlcude orther tasks

from packagestree import *
import re
import os

class Task:
	def __init__(self, filename_task, forcenonuscd1):
		self.__announce_task__(filename_task)

		if not os.path.isfile(filename_task):
			raise TaskFileNotFound, filename_task

		self.task_keys = ['i386', 'alpha', 'arm', 'hppa', 'ia64', 'm68k',
			'mips', 'mipsel', 'powerpc', 's390', 'sparc', 'amd64', 'all']
		self.file_task = open(filename_task)
		self.dirname = os.path.dirname(os.path.abspath(filename_task))
		self.forcenonuscd1 = forcenonuscd1
		self.include = {}
		self.exclude = {}
		for key in self.task_keys:
			self.include[key] = []
			self.exclude[key] = []

		task_without_comments = self.__delete_comments__(self.file_task.read())
		self.file_task.close()

		self.__import_include__(task_without_comments)
		self.__import_exclude__(task_without_comments)
		self.__import_archs_packages__(task_without_comments)
		self.__import_indep_packages__(task_without_comments)

		if forcenonuscd1:
			self.__import_nonus__(task_without_comments)

	def __announce_task__(self, task):
		print '  Including %s task...' % task

	def __import_nonus__(self, text):
		nonus_re = re.compile('#if\s*\(\s*FORCENONUSONCD1\s*==\s*1\s*\)\s*\n\s*#include\s*"(.*?)".*#endif', re.DOTALL)
		for task_to_include in nonus_re.findall(text):
			# Create a task
			temporal_task = Task(self.dirname + os.sep + task_to_include, self.forcenonuscd1)
			# Add the info of the task into this one
			self.load(temporal_task, 'include')

	def __import_indep_packages__(self, text):
		dep_re = re.compile('#ifdef .*?#endif', re.DOTALL)
		shell_comments_re = re.compile('#.*')
		indep_packages = shell_comments_re.sub('', dep_re.sub('', text))
		for package in indep_packages.strip().split('\n'):
			if package:
				if not package in self.include['all']:
					self.include['all'].append(package)

	def __delete_comments__(self, text):
		comments_re = re.compile('\/\*.*?\*\/', re.DOTALL)
		return comments_re.sub('', text)

	def __import_include__(self, text):
		include_re = re.compile('#include <(.*?)>')
		tasks_to_include = include_re.findall(text)
		for task_to_include in tasks_to_include:
			# Create a task
			task_fn = self.dirname + os.sep + task_to_include
			temporal_task = Task(task_fn, self.forcenonuscd1)
			# Add the info of the task into this one
			self.load(temporal_task, 'include')

	def __import_exclude__(self, text):
		exclude_re = re.compile('#exclude <(.*?)>')
		tasks_to_exclude = exclude_re.findall(text)
		for task_to_exclude in tasks_to_exclude:
			# Create a task
			temporal_task = Task(self.dirname + os.sep + task_to_exclude, self.forcenonuscd1)
			# Add the info of the task into this one
			self.load(temporal_task, 'exclude')

	def __import_archs_packages__(self, text):
		arch_deps_re = re.compile('#ifdef ARCH_(\w*).*?\n(.*?)#endif', re.DOTALL)
		arch_deps = arch_deps_re.findall(text)
		for arch, packages in arch_deps:
			for package in packages.strip().split('\n'):
				if package and not package in self.include[arch]:
					self.include[arch].append(package)
	def load(self, task, op):
		ext_task_include = task.get_include()
		ext_task_exclude = task.get_exclude()
	
		if op == 'include':
			# for all the arches and 'all' do...
			for key in self.task_keys:
				# if we aren't in a empty list
				if ext_task_include[key]:
					# for each element
					for element in ext_task_include[key]:
						# if we haven't it already, append it
						if not element in self.include[key]:
							self.include[key].append(element)
						# TODO think if it worth to add an extra check here. Check if the element is in include and exclude at the same time.
				if ext_task_exclude[key]:
					for element in ext_task_exclude[key]:
						if not element in self.exclude[key]:
							self.exclude[key].append(element)
		elif op == 'exclude':
			# for all the arches and 'all' do...
			for key in self.task_keys:
				# if we aren't in a empty list
				if ext_task_include[key]:
					# for each element included on the task.
					for element in ext_task_include[key]:
						# if the element is not included in the current exclude list, append it.
						if not element in self.exclude[key]:
							self.exclude[key].append(element)
				if ext_task_exclude[key]:
					# for each element excluded on the task
					for element in ext_task_exclude[key]:
						# exclude it also in this task.
						if not element in self.exclude[key]:
							self.exclude[key].append(element)
		else:
			raise TaskIllegalArgument, op

	def get_include(self):
		return self.include

	def get_exclude(self):
		return self.exclude

	def get_unknown_packages(self, packages, key_task):
		"""
		Only packages from include
		"""
		unknown = []
		for package in self.include[key_task]:
			if not packages.has_package(package):
				unknown.append(package)
		return unknown

	def remove_unknown_elements(self, packages, key_task):
		"""
		Only packages from include
		"""
		include_branch = []
		unknown = []

		for package in self.include[key_task]:
			if packages.has_package(package):
				include_branch.append(package)
			else:
				unknown.append(package)

		self.include[key_task] = include_branch
		return unknown

	def show_info(self, key_task=None):
		if key_task:
			print "=== INCLUDE ==="
			print "# ", key_task
			for package in self.include[key_task]:
				print "  ", package
			print "=== EXCLUDE ==="
			print "# ", key_task
			for package in self.exclude[key_task]:
				print "  ", package
		else:
			print "=== INCLUDE ==="
			for key in self.task_keys:
				print "# ", key
				for package in  self.include[key]:
					print "  ", package
	
			print "=== EXCLUDE ==="
			for key in self.task_keys:
				print "# ", key
				for package in self.exclude[key]:
					print "  ", package

#		log = open('/tmp/dig.log', 'w')
#
#		print >> log, "=== INCLUDE ==="
#		for key in self.task_keys:
#			print >> log, "# ", key
#			for package in  self.include[key]:
#				print >> log, "  ", package
#
#		print >> log,  "=== EXCLUDE ==="
#		for key in self.task_keys:
#			print >> log, "# ", key
#			for package in self.exclude[key]:
#				print >> log, "  ", package

class TaskError(Exception):
	pass

class TaskIllegalArgument(TaskError):
	def __init__(self, value):
		TaskError.__init__(self, 'Illegal argument: "%s"' % value)

class TaskFileNotFound(TaskError):
	def __init__(self, value):
		TaskError.__init__(self, 'Tasks file "%s" doesn\'t exist' % value)
