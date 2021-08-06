from setuptools import setup, find_packages, Extension

ext = Extension('hype._hype',
                ['hype/_hype.pyx'],
                libraries=['estraier'],
                library_dirs=['/usr/local/lib', '/opt/local/lib'],
                include_dirs=['/usr/local/include', '/opt/local/include']
                )

setup(
    name='hype',
    version='0.1',
    packages=find_packages(),
    ext_modules=[ext,],
    package_data={}
    )

