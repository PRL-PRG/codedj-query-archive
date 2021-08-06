from distutils.core import setup

"""SearchFS: mounts a directory with the results of a query.

SearchFS is a FUSE that lets the user mount a virtual directory with the result of a query. This query can be a shell script (find, locate, etc) or many other backends as beagle.
"""

classifiers = """\
Development Status :: 2 - Beta
Intended Audience :: Developers
License :: OSI Approved :: Zope Public License
Programming Language :: Python
Topic :: Filesystems
Topic :: Software Development :: Libraries :: Python Modules
Operating System :: Unix
"""

doclines = __doc__.splitlines()

setup(name="searchfs",
      version="0.1",
      maintainer="Cesar Izurieta",
      maintainer_email="cesar@caih.org",
      url="http://caih.org/searchfs"
      license="http://www.gnu.org/copyleft/gpl.html",
      platforms=["unix"],
      description = doclines[0],
      classifiers = filter(None, classifiers.splitlines()),
      long_description = "\n".join(doclines[2:])
)


