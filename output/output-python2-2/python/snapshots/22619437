import os
from setuptools import setup, find_packages

def read(*rnames):
    return open(os.path.join(os.path.dirname(__file__), *rnames)).read()

name = "zc.authorizedotnet"
setup(
    name = name,
    version = "1.3",
    author = "Zope Corporation",
    author_email = "zope3-dev@zope.org",
    description = "A simple interface to Authorize.Net's AIM API",
    long_description=read('src', 'zc', 'authorizedotnet', 'README.txt'),
    license = "ZPL 2.1",
    keywords = "credit card authorize.net CC AIM",
    url='http://svn.zope.org/zc.authorizedotnet',

    packages = ['zc', 'zc.authorizedotnet'],
    package_dir = {'': 'src'},
    namespace_packages = ['zc'],
    install_requires = ['setuptools', 'zc.ssl', 'zc.creditcard'],
    include_package_data = True,
    tests_require = ['zope.testing', 'zope.testbrowser'],
    test_suite = name+'.tests.test_suite',
    classifiers = [
       'Intended Audience :: Developers',
       'License :: OSI Approved :: Zope Public License',
       'Topic :: Software Development :: Libraries :: Python Modules',
       ],
    )
