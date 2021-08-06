##############################################################################
#
# Copyright (c) 2006-2008 Zope Corporation and Contributors.
# All Rights Reserved.
#
# This software is subject to the provisions of the Zope Public License,
# Version 2.1 (ZPL).  A copy of the ZPL should accompany this distribution.
# THIS SOFTWARE IS PROVIDED "AS IS" AND ANY AND ALL EXPRESS OR IMPLIED
# WARRANTIES ARE DISCLAIMED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF TITLE, MERCHANTABILITY, AGAINST INFRINGEMENT, AND FITNESS
# FOR A PARTICULAR PURPOSE.
#
##############################################################################
import os
from setuptools import setup, find_packages

# generic helpers primarily for the long_description
try:
    import docutils
except ImportError:
    docutils = None
    def validateReST(text):
        return ''
else:
    import docutils.utils
    import docutils.parsers.rst
    import StringIO
    def validateReST(text):
        doc = docutils.utils.new_document('validator')
        # our desired settings
        doc.reporter.halt_level = 5
        doc.reporter.report_level = 1
        stream = doc.reporter.stream = StringIO.StringIO()
        # docutils buglets (?)
        doc.settings.tab_width = 2
        doc.settings.pep_references = doc.settings.rfc_references = False
        doc.settings.trim_footnote_reference_space = None
        # and we're off...
        parser = docutils.parsers.rst.Parser()
        parser.parse(text, doc)
        return stream.getvalue()

def text(*args, **kwargs):
    # note: distutils explicitly disallows unicode for setup values :-/
    # http://docs.python.org/dist/meta-data.html
    tmp = []
    for a in args:
        if a.endswith('.txt'):
            f = open(os.path.join(*a.split('/')))
            tmp.append(f.read())
            f.close()
            tmp.append('\n\n')
        else:
            tmp.append(a)
    if len(tmp) == 1:
        res = tmp[0]
    else:
        res = ''.join(tmp)
    out = kwargs.get('out')
    if out is True:
        out = 'TEST_THIS_REST_BEFORE_REGISTERING.txt'
    if out:
        f = open(out, 'w')
        f.write(res)
        f.close()
        report = validateReST(res)
        if report:
            print report
            raise ValueError('ReST validation error')
    return res
# end helpers; below this line should be code custom to this package

setup(
    name='zc.async',
    version='1.5.0',
    namespace_packages=['zc'],
    packages=find_packages('src'),
    package_dir={'':'src'},
    package_data = {'': ['*.txt', '*.zcml']},
    zip_safe=False,
    author='Gary Poster',
    author_email='gary.poster@canonical.com',
    description=
        'Schedule durable tasks across multiple processes and machines.',
    long_description=text(
        'README.txt',
        'src/zc/async/CHANGES.txt',
        out=(docutils is not None)),
    license='ZPL',
    install_requires=[
        'ZODB3',
        'pytz',
        'rwproperty',
        'setuptools',
        'uuid',
        'zc.queue',
        'zc.dict>=1.2.1',
        'zc.twist>=1.3',
        'Twisted>=8.0.1', # 8.0 was setuptools compatible, 8.0.1 had bugfixes.
        # note that Twisted builds with warnings with py2.4.  It works fine,
        # and is considered a supported Python version on the Twisted site.
        'zope.bforest>=1.2',
        'zope.component',
        'zope.event',
        'zope.i18nmessageid',
        'zope.interface',
        'zope.minmax',
        'zope.testing',
        'rwproperty',
        ],
    extras_require={
        'monitor': [
            'zc.monitor',
            'simplejson',
            ],
        'z3':[
            'zc.z3monitor',
            'zope.security',
            'zope.app.security',
            'zope.app.component',
            'simplejson',
            ]},
    url='http://packages.python.org/zc.async',
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Zope Public License",
        "Operating System :: OS Independent",
        "Programming Language :: Python",
        "Framework :: ZODB"],
    )
