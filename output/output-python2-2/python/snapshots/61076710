from ez_setup import use_setuptools
use_setuptools()

from setuptools import setup, find_packages

version = '0.9'

setup(
    name='django-maintenancemode',
    version=version,
    description='Django-maintenancemode allows you to temporary shutdown your site for maintenance work',
    long_description='Django maintenance mode is a middleware that allows you to temporary shutdown your site for non staff users and show them a page that the site is down for maintenance. Logged in users having staff credentials can still fully use the site.',
    author='Remco Wendt',
    author_email='remco@maykinmedia.nl',
    license = "BSD",
    platforms = ["any"],
    url='http://code.google.com/p/django-maintenancemode/',
    download_url='',
    package_dir = {'': 'src'},
    packages=find_packages('src'),
    include_package_data=True,
    classifiers=[
            'Development Status :: 4 - Beta',
            'Environment :: Web Environment',
            'Framework :: Django',
            'Intended Audience :: Developers',
            'License :: OSI Approved :: BSD License',
            'Operating System :: OS Independent',
            'Programming Language :: Python',
            'Topic :: Utilities',
    ],
    zip_safe=False,
)