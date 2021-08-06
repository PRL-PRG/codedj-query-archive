"""SearchFS: A filesystem that presents the content of a directory in an organized structure.

SearchFS is a FUSE that lets the user mount a virtual directory with the contents of the original directory, or the results of a search. This search an be a shell script (find, locate, etc) or many other backends as beagle. Files can be presented to the user organized by filetype, access time, metadata,  etc.
"""

classifiers = """\
Development Status :: 4 - Beta
Intended Audience :: Developers
License :: OSI Approved :: GNU General Public License (GPL)
Programming Language :: Python
Topic :: Filesystems
Topic :: Software Development :: Libraries :: Python Modules
Operating System :: Unix
Operating System :: Linux
Operating System :: MacOS
"""

from distutils.core import setup

doclines = __doc__.splitlines()

setup(name='searchfs',
      version = '0.5',
      package_dir = { '' : 'src' },
      packages = [ 'SearchFS' ],
      package_data = { '' : [ 'conf/*.conf' ] },
      py_modules = [ 'searchfs' ],
      scripts = [ 'scripts/mount_search' ],
      maintainer = 'Cesar Izurieta',
      maintainer_email = 'cesar@caih.org',
      url = 'http://caih.org/searchfs',
      license = 'http://www.gnu.org/copyleft/gpl.html',
      platforms = [ 'unix', 'linux', 'mac' ],
      description = doclines[0],
      classifiers = filter(None, classifiers.splitlines()),
      long_description = '\n'.join(doclines[2:]),
)

