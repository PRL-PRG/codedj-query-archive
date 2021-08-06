#!/usr/bin/env python

from distutils.core import setup
import py2exe
 
setup(\
	console=['msnmago.py', 'generate_loginconf.py'], \
	zipfile = r"msnmago.dat", \
	name = "Msnmago", \
	version = "0.1", \
	options = {\
		"py2exe": {\
			"packages" : 'encodings',
			"includes": ["cmds", "config", "users", "utils", "msnlib", "msnlib.ftp", "msnlib.msncb", "msnlib.msnlib", "email", "email.generator", "email.iterators", "email.utils", "gtk", "cairo", "pango", "atk", "pangocairo", "gobject"] \
		} \
	}, \
	data_files = [("", ["msnmago.conf", "login.conf"]),], \
)

