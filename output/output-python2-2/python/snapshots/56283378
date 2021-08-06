import os

from setuptools import setup, find_packages

long_description = (
    open('src/zc/async/README.txt').read() + "\n" +
    open('src/zc/async/README_2.txt').read() + "\n" +
    open('src/zc/async/README_3.txt').read() +
    "\n\n=======\nChanges\n=======\n\n" +
    open('src/zc/async/CHANGES.txt').read() + "\n")

f = open('TEST_THIS_REST_BEFORE_REGISTERING.txt', 'w')
f.write(long_description)
f.close()

setup(
    name='zc.async',
    version='1.0',
    packages=find_packages('src'),
    package_dir={'':'src'},
    zip_safe=False,
    author='Zope Project',
    author_email='zope-dev@zope.org',
    description='Perform durable tasks asynchronously',
    long_description=long_description,
    license='ZPL',
    install_requires=[
        'ZODB3',
        'pytz',
        'rwproperty',
        'uuid',
        'zc.queue',
        'zc.dict>=1.2.1',
        'zc.twist>=1.2',
        'Twisted>=8.0.1', # 8.0 was setuptools compatible, 8.0.1 had bugfixes.
        # note that Twisted builds with warnings, at least with py2.4.  It
        # seems to still build ok.
        'zope.bforest>=1.1.1',
        'zope.component',
        'zope.i18nmessageid',
        'zope.interface',
        'zope.testing',
        'rwproperty',
        ],
    extras_require={
        'z3':[
            'zc.z3monitor',
            'simplejson',
            ]},
    include_package_data=True,
    )
