from setuptools import setup, find_packages
import sys, os

version = '0.1'

setup(name='PasteDeploySimplestExample',
      version=version,
      description="The simple example of Paste-enabled app",
      long_description="""\
The PasteDeploy-enabled example app for beginners""",
      classifiers=[], # Get strings from http://pypi.python.org/pypi?%3Aaction=list_classifiers
      keywords='',
      author='Yury Yurevich',
      author_email='the.pythy@gmail.com',
      url='',
      license='GNU GPL2',
      packages=find_packages(exclude=['ez_setup', 'examples', 'tests']),
      include_package_data=True,
      zip_safe=False,
      install_requires=[
          # -*- Extra requirements: -*-
      ],
      entry_points="""
      # -*- Entry points: -*-
      
      [paste.app_factory]
      main = pastedeploysimplestexample:demo_app_factory
      
      [paste.server_factory]
      main = pastedeploysimplestexample:paste_demo_server_factory
      wsgiref = pastedeploysimplestexample:wsgiref_demo_server_factory
      """,
      )
