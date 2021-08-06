from setuptools import setup, find_packages
import sys, os

version = '0.1'

setup(name='PasteDjangoTemplate',
      version=version,
      description="Paste-driven template for Django",
      long_description="""\
Template for PasteScript for making Django projectless application.
Generated application is a regular Django app which is not Paste-enabled.
""",
      classifiers=[], # Get strings from http://pypi.python.org/pypi?%3Aaction=list_classifiers
      keywords='',
      author='Yury Yurevich',
      author_email='the.pythy@gmail.com',
      url='',
      license='',
      packages=find_packages(),
      include_package_data=True,
      zip_safe=False,
      install_requires=[
          "PasteScript>=1.3"
      ],
      entry_points="""
      # -*- Entry points: -*-
      [paste.paster_create_template]
      django = pastedjangotemplate:DjangoTemplate
      """,
      )
