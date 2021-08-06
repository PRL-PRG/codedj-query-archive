#!/usr/bin/env python
# F. Pierfederici (fpierfed@lsst.org)
from numpy.distutils.core import setup
from numpy.distutils.core import Extension


_ephem = Extension(name = 'ephem._ephem', 
                   sources=['ephemwrap.f90', ],
                   include_dirs=['/usr/local/orbfit/include',
                                 '/usr/local/orbfit/src/suit',
                                 '/usr/local/orbfit/src/propag',
                                 '/usr/local/orbfit/src/fitobs'],
                   library_dirs=['/usr/local/orbfit/lib/', ],
                   libraries=['ephem', 'suit', 'gauss'],
                   f2py_options=[])


if __name__ == "__main__": 
    setup(name = 'ephem', 
          description = "Ephemerides module", 
          author = "Francesco Pierfederici", 
          author_email = "fpierfed@lsst.org",
          version='1.0a1',
          package_dir={'ephem': '.'},
          packages=['ephem', ],
          ext_modules = [_ephem, ]
) 
