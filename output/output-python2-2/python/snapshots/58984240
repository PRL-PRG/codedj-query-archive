from distutils.core import setup
from distutils.core import Extension as _Extension

try:
    from Pyrex.Distutils.build_ext import build_ext
except ImportError:
    from distutils.command.build_ext import build_ext
    # Pyrex isn't around, so fix up the sources
    class Extension(_Extension):
        """Extension that uses '.c' files in place of '.pyx' files"""
        def __init__(self,*args,**kw):
            _Extension.__init__(self,*args,**kw)
            sources = []
            for s in self.sources:
                if s.endswith('.pyx'):
                    sources.append(s[:-3]+'c')
                else:
                    sources.append(s)
            self.sources = sources

else:
    # Pyrex is here, just use regular extension type
    Extension = _Extension

ext = Extension('hype._hype',
                ['hype/_hype.pyx'],
                libraries=['estraier'],
                library_dirs=['/usr/lib/', '/usr/local/lib', '/opt/local/lib'],
                include_dirs=['/usr/include/', '/usr/local/include', '/opt/local/include',
                # Debian non-standard locations
                '/usr/include/qdbm',
                '/usr/include/estraier'],
                )

setup(
    name='hype',
    version='0.1',
    packages=['hype'],
    ext_modules=[ext,],
    package_data={},
    cmdclass = {'build_ext' : build_ext}
    )

