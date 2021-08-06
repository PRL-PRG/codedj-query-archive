from distutils.core import setup
import glob, os

modname = 'bb-assist'
PATH = '/usr/share/' + modname + '/'

if __name__ == '__main__' :
  dist = setup(name = modname,
    version = '0.1',
    license = 'GPL',
    description = 'A BroadBand Assistant configurator',
    long_description = ' This package is an assistant for help in the configuration of several types of Broadband (DSL, etc) devices.',
    author = 'Vicente J. Ruiz Jurado',
    author_email = 'vjrj@tid.es',
    package_dir = {modname: '.'},
    scripts=[
      'bb-assist',
    ],
    py_modules = ["bbutils", "bbexpbackend", "bbsysbackend"],
    data_files=[
      (os.path.join(PATH, 'glade'), glob.glob('glade/*.glade')),
      (os.path.join(PATH, 'glade/pixmaps'), glob.glob('glade/pixmaps/*')),
      ('/usr/share/gnome/help/bb-assist/es/', glob.glob('help/es/*')),
      ('/usr/share/applications', ['bb-assist.desktop']),
      ('/etc/hotplug/usb', ['extra/speedtch']),
      (PATH, glob.glob('*.xml')),
      (PATH, glob.glob('*.xsl')),
    ]
  )
