# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Patches over http://bugzilla.gnome.org/show_bug.cgi?id=543330
"""
from __future__ import division, absolute_import, with_statement
import gobject
__all__ = 'property',

def _version_compare(l,r):
	"""_version_compare(tuple, tuple) -> number
	Takes two version tuples and compares them. Uses the same semantics as 
	cmp().
	"""
	if l == r: return 0
	for a,b in zip(l,r):
		c = cmp(a,b)
		if c: return c
	else:
		c = cmp(len(l), len(r))
		if c:
			return c
		else:
			return 0

if _version_compare(gobject.glib_version, (9,0,0)) >= 0:
	# Bug fixed
	from gobject.propertyhelper import property as intermediateprop
else:
	from gobject.propertyhelper import property as gprop
	from gobject.constants import \
		 TYPE_NONE, TYPE_INTERFACE, TYPE_CHAR, TYPE_UCHAR, \
		 TYPE_BOOLEAN, TYPE_INT, TYPE_UINT, TYPE_LONG, \
		 TYPE_ULONG, TYPE_INT64, TYPE_UINT64, TYPE_ENUM, \
		 TYPE_FLAGS, TYPE_FLOAT, TYPE_DOUBLE, TYPE_STRING, \
		 TYPE_POINTER, TYPE_BOXED, TYPE_PARAM, TYPE_OBJECT, \
		 TYPE_PYOBJECT
	class intermediateprop(gprop):
		def _type_from_python(self, type): # Yes, this is private
			try:
				return super(property, self)._type_from_python(type)
			except TypeError:
				# For these, the actual type can (or must) be passed to __gproperties__
				if issubclass(type, gobject.GInterface):
					return type
				elif issubclass(type, gobject.GEnum):
					return TYPE_ENUM
				elif issubclass(type, gobject.GFlags):
					return TYPE_FLAGS
				elif issubclass(type, gobject.GPointer):
					return type
				elif issubclass(type, gobject.GBoxed):
					return type
				# Nothing for TYPE_PARAM
				elif issubclass(type, gobject.GObject) or isinstance(type, gobject.GType):
					return type
				elif issubclass(type, object) or isinstance(type, __builtins__.type):
					return type
				else:
					raise
	
		def _check_default(self):
			if self.type in (TYPE_BOXED, TYPE_INTERFACE, TYPE_PARAM, TYPE_OBJECT, TYPE_POINTER):
				if self.default is not None:
					raise TypeError("object types does not have default values")
			super(property, self)._check_default()
		
		def get_pspec_args(self):
			ptype = self.type
			if hasattr(ptype, '__gtype__'):
				ptype = ptype.__gtype__.fundamental
			if ptype == TYPE_ENUM or ptype == TYPE_FLAGS:
				args = (self.default,)
			elif ptype in (TYPE_BOXED, TYPE_INTERFACE, TYPE_PARAM, TYPE_OBJECT, TYPE_POINTER):
				args = ()
			else:
				return super(property, self).get_pspec_args()
			return (self.type, self.nick, self.blurb) + args + (self.flags,)

# Adds a feature that if you give Ellipsis as the getter or setter, it uses the default
class property(intermediateprop):
	def __init__(self, *p, **kw):
		super(property, self).__init__(*p, **kw)
		if self.getter is Ellipsis:
			self.getter = self._default_getter
		if self.setter is Ellipsis:
			self.setter = self._default_setter

