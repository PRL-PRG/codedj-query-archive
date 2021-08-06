import subprocess
from setuptools import setup, find_packages, Extension

def libraries():
    p = subprocess.Popen(["estconfig", "--libs"], stdout=subprocess.PIPE)
    return [i[2:] for i in p.stdout.read().split() if i[:2] == '-l']

def library_dirs():
    p = subprocess.Popen(["estconfig", "--ldflags"], stdout=subprocess.PIPE)
    return [i[2:] for i in p.stdout.read().split() if i[:2] == '-L']

def include_dirs():
    p = subprocess.Popen(["estconfig", "--cflags"], stdout=subprocess.PIPE)
    return [i[2:] for i in p.stdout.read().split() if i[:2] == '-I']

ext = Extension('hype._hype',
                ['hype/_hype.pyx'],
                libraries=libraries(),
                library_dirs=library_dirs(),
                include_dirs=include_dirs(),
                )

setup(
    name='hype',
    version='0.1.2',
    packages=find_packages(),
    ext_modules=[ext,],
    package_data={}
    )

