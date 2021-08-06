# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
doc string
"""
from __future__ import division, absolute_import, with_statement
import os
try:
	import gnomevfs
except ImportError:
	gnomevfs = None


def make_absolute(fn):
	print "make_absolute:", fn,
	if gnomevfs:
		rv = gnomevfs.URI(fn)
		if rv.is_local and not fn.startswith('file:'):
			rv = gnomevfs.URI(os.path.abspath(fn))
		rv = str(rv)
	else:
		rv = os.path.abspath(fn)
	print rv
	return rv
