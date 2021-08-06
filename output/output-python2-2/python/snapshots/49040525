class Boot:
	def __init__(self, conf, dig_dir, mirror_path, root_dir, isolinux_dir, install_dir, distro_dir):
		"""
		Boot relationed stuff
		"""
		import os

		self.conf = conf
		self.dig_dir = dig_dir
		self.mirror_path = mirror_path
		self.root_dir = root_dir
		self.isolinux_dir = isolinux_dir
		self.install_dir = install_dir
		self.distro_dir = distro_dir

		self.images_dir = os.sep.join([self.mirror_path, 'dists', self.conf['CodeName'], 'main', 'installer-' + self.conf['Arch'], 'current', 'images'])

                self.boot_images = ['cdrom/boot.img','cdrom/initrd.gz','cdrom/vmlinuz','cdrom/debian-cd_info.tar.gz', 'floppy/cd-drivers.img', 'floppy/boot.img', 'floppy/root.img']

	# TODO
	def get_size(self):
		"""
		Return the size of the isolinux/, install/, .disk/
		"""

		return 0

	def include_images(self):
		"""
		Include the images and floppies. This method is called for any media number.
		"""

		import os
		import shutil

                for image in self.boot_images:
                        if not os.path.isdir(os.sep.join([self.root_dir, os.path.dirname(image)])):
                                os.makedirs(os.sep.join([self.root_dir, os.path.dirname(image)]))

                        shutil.copyfile(os.sep.join([self.images_dir, image]), os.sep.join([self.root_dir, image]))
                        # TODO consider extra disk images

	def make_bootable(self, media_number):
		"""
		Make a media bootable. Only called for the first media.
		"""

		import os
		import shutil
		import tarfile
		import re

		print 'Using ISOLINUX boot-disk image on CD' + media_number
		print self.isolinux_dir
		os.makedirs(self.isolinux_dir)
		os.makedirs(self.install_dir)
		
		shutil.copyfile(os.sep.join([self.dig_dir, 'data', 'distro', self.conf['CodeName'], 'isolinux.bin']), os.sep.join([self.isolinux_dir, 'isolinux.bin']))
		shutil.copyfile(os.sep.join([self.images_dir, 'cdrom', 'vmlinuz']), os.sep.join([self.install_dir, 'vmlinuz']))
		shutil.copyfile(os.sep.join([self.images_dir, 'cdrom', 'initrd.gz']), os.sep.join([self.install_dir, 'initrd.gz']))
		
		for image in self.boot_images:
			# TODO: check if the image has execute permission.
			dir_aux = os.sep.join([self.install_dir, os.path.dirname(image)])
			if not os.path.isdir(dir_aux):
				os.makedirs(dir_aux)
			shutil.copyfile(os.sep.join([self.images_dir, image]), os.sep.join([dir_aux, os.path.basename(image)]))

		# Include Smart Boot Manager
		self.__include_smb__()
		
		tar = tarfile.open(os.sep.join([self.root_dir, 'cdrom', 'debian-cd_info.tar.gz']), 'r:gz')
		
		for tarinfo in tar:
			tar.extract(tarinfo, self.isolinux_dir)
		tar.close()
		
		shutil.move(os.sep.join([self.isolinux_dir, 'syslinux.txt']), os.sep.join([self.isolinux_dir, 'isolinux.txt']))
		
		if os.path.isfile(os.sep.join([self.isolinux_dir, 'f3.txt.with26'])):
			shutil.move(os.sep.join([self.isolinux_dir, 'f3.txt.with26']), os.sep.join([self.isolinux_dir, 'f3.txt']))
			boot_26_images = ['cdrom/2.6/initrd.gz','cdrom/2.6/vmlinuz']
			for image in boot_26_images:
				if not os.path.isdir(os.sep.join([self.root_dir, os.path.dirname(image)])):
					os.makedirs(os.sep.join([self.root_dir, os.path.dirname(image)]))
				shutil.copyfile(os.sep.join([self.images_dir, image]), os.sep.join([self.root_dir, image]))
			
			shutil.copytree(os.sep.join([self.images_dir, 'cdrom/2.6']), os.sep.join([self.install_dir, '2.6']))
			
			# A little bit of isolinux.cfg
			kernel_params_re = re.compile('\$KERNEL_PARAMS')
			isolinux26_f = open(os.sep.join([self.distro_dir, 'isolinux26.cfg']), 'r')
			isolinux26_txt = kernel_params_re.sub(self.conf['KernelParams'], isolinux26_f.read())
			isolinux26_f.close()
			
			isolinux_f = open(os.sep.join([self.isolinux_dir, 'isolinux.cfg']), 'w')
			isolinux_f.write(isolinux26_txt)
			isolinux_f.close()
		else:
			shutil.copyfile(os.sep.join([self.distro_dir, 'isolinux.cfg']), os.sep.join([self.isolinux_dir, 'isolinux.cfg']))
	
	def __include_smb__(self):
		"""
		Include Smart Boot Manager image for people where isolinux fails
		"""

		import os
		import gzip
		import shutil

		gzipped_fn = os.sep.join([self.dig_dir, 'data', 'distro', self.conf['CodeName'], 'sbm.bin.gz'])
		ungzipped_fn = os.sep.join([self.install_dir, 'sbm.bin'])
		gzipped_file = gzip.GzipFile(filename=gzipped_fn)
		f = open(ungzipped_fn, 'w')
		for line in gzipped_file:
			print >>f, line,
		f.close()
		gzipped_file.close()
		
		# Keep the original file timestamp
		os.popen('touch -r %s %s' % (gzipped_fn, ungzipped_fn))
		shutil.copyfile(os.sep.join([self.dig_dir, 'data', 'distro', self.conf['CodeName'], 'README.sbm']), os.sep.join([self.install_dir, 'README.sbm']))

