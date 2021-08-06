#!/usr/bin/env python

import distutils.sysconfig
from distutils.core import setup, Extension
from distutils.command.install_headers import install_headers
import os, sys
from glob import glob



#infer library directory (osx linux cygwin)
libstub='lib';
os_libs=[];
os_libdirs=[];
e_link_args=[];
if sys.platform=='cygwin':
    libstub='lib_cygwin';
    os_libs=['winmm','stdc++'];
    os_libdirs=['c:\\mingw\\lib'];
if sys.platform=='win32':
    libstub='lib_cygwin';
    os_libs=['winmm','stdc++'];
    os_libdirs=['c:\\mingw\\lib'];
elif sys.platform[0:5]=='linux' :
    libstub='lib_linux'
elif sys.platform=='darwin':
    libstub='lib_osx'
    #e_link_args=['-framework CoreMIDI -framework Carbon'];


share_base=os.path.join('.','..','..','..','..','share');
share_includedir=os.path.join(share_base,'include')
share_libdir=os.path.join(share_base,libstub)

libdse_include=os.path.join('.','..','libdse')



libdirs=[share_libdir];
libdirs.extend(os_libdirs);
#sharedlibs= ['rete','dse','midi','cblas','atlas','netcdf','supc++','stdc++']
sharedlibs= ['rete','dse','midi','netcdf','supc++','stdc++']
libs=sharedlibs;
libs.extend(os_libs);
coderoot_include=os.path.join('.','..')

ext_modules = [Extension('rete',
                         ['retemodule.cpp'],
                         include_dirs=[share_includedir],
                         library_dirs=libdirs,
                         extra_link_args=e_link_args,
                         libraries = libs)]

packages = ['dse']
class modified_install_headers(install_headers):
    def finalize_options(self):
        print 'Installing in '+install_dir
        install_headers.finalize_options(self)
        self.install_dir = \
                os.path.join(os.path.split(self.install_dir)[0], 'rete')
headers = os.path.join("..","*.h")
setup (name = "dse",
       version = "0.0.1",
       description = "Python Interfaces",
       long_description =  """Python Interfaces""",
       author = "Douglas Eck",
       author_email = "eckdoug@iro.umontreal.ca",
       url = "http://www.idsia.ch/~doug",
       #licence = "GPL",
       packages = packages,
       #headers = headers,
       ext_package = 'dse',
       ext_modules = ext_modules,
       cmdclass = {'install_headers': modified_install_headers},
       )




