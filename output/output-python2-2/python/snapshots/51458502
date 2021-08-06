#-*- coding: utf-8 -*-
"""
doc string
"""
from __future__ import with_statement, division, absolute_import
import gtk, gobject, sys

__all__ = 'Box',

class Box(gobject.GObject):
	"""
	Nearly identical to gtk.gdk.Rectangle, but also has a color associated with 
	it.
	"""
	__gtype_name__ = 'Box'
	__gproperties__ = {
		'rect' : (gtk.gdk.Rectangle,
		           'the space taken',
		           'A gtk.gdk.Rectangle of the same size and position.',
		           gobject.PARAM_READWRITE),
		'color' : (gtk.gdk.Color,
		           'the color',
		           'The gtk.gdk.Color for the color.',
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		# Shortcuts
		'width' : (gobject.TYPE_INT,
		           'width of the box',
		           'the width of the box in pixels',
		           -sys.maxint - 1,
		           sys.maxint,
		           0,
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'height' : (gobject.TYPE_INT,
		           'height of the box',
		           'the height of the box in pixels',
		           -sys.maxint - 1,
		           sys.maxint,
		           0,
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'x' : (gobject.TYPE_INT,
		           '',
		           '',
		           -sys.maxint - 1,
		           sys.maxint,
		           0,
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'y' : (gobject.TYPE_INT,
		           '',
		           '',
		           -sys.maxint - 1,
		           sys.maxint,
		           0,
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'pixel' : (gobject.TYPE_ULONG,
		           '',
		           '',
		           0,
		           0xFFFFFF,
		           0,
		           gobject.PARAM_READWRITE),
		'red' : (gobject.TYPE_UINT,
		           '',
		           '',
		           0,
		           0xFFFF,
		           0,
		           gobject.PARAM_READWRITE),
		'green' : (gobject.TYPE_UINT,
		           '',
		           '',
		           0,
		           0xFFFF,
		           0,
		           gobject.PARAM_READWRITE),
		'blue' : (gobject.TYPE_UINT,
		           '',
		           '',
		           0,
		           0xFFFF,
		           0,
		           gobject.PARAM_READWRITE),
		}
	
	prop = lambda name: property((lambda s: s.get_property(name)), (lambda s,v: s.set_property(name,v)))
	
	rect = prop('rect')
	color = prop('color')
	x = prop('x')
	y = prop('y')
	width = prop('width')
	height = prop('height')
	pixel = prop('pixel')
	red = prop('red')
	green = prop('green')
	blue = prop('blue')
	
	del prop
	
	_rect = gtk.gdk.Rectangle()
	_color = gtk.gdk.Color()
	def __init__(self,rect=None,color=None):
		gobject.GObject.__init__(self)
		self._rect = gtk.gdk.Rectangle() 
		if rect is not None: self._rect = rect
		self._color = gtk.gdk.Color() 
		if color is not None: self._color = color
	
	def _rect_notify(self,obj,prop):
		if obj is self._rect and prop.name in ('x','y','width','height'):
			self.emit('notify::'+prop.name, prop)
	def _connect_rect(self, rect):
		#self._rect_nid = rect.connect('notify', self._rect_notify)
		pass
	def _disconnect_rect(self, rect):
		#rect.disconnect(self._rect_nid)
		#del self._rect_nid
		pass
	
	def dimensions_text(self):
		r = self._rect
		return u'%i,%i\u2192%i,%i (%i\u2A2F%i)' % (r.x, r.y, r.x+r.width, r.y+r.height, r.width, r.height)
	
#	def _color_notify(self,obj,prop):
#		if obj is self._color and prop.name in ('pixel','red','green','blue'):
#			self.emit('notify::'+prop.name, prop)
#	def _connect_color(self, color):
#		self._color_nid = color.connect('notify', self._color_notify)
#	def _disconnect_color(self, color):
#		color.disconnect(self._color_nid)
#		del self._color_nid
	
	def do_get_property(self, property):
		if property.name == 'color':
			return self._color
		elif property.name == 'rect':
			return self._rect
		elif property.name in ('x','y','width','height'):
			return getattr(self._rect, property.name)
		elif property.name in ('pixel','red','green','blue'):
			return getattr(self._color, property.name)
		else:
			raise AttributeError, 'unknown property %s' % property.name
	
	def do_set_property(self, property, value):
		if property.name == 'color':
			if self._color is not value:
#				self._disconnect_color(self._color)
				self._color = value
#				self._connect_color(self._color)
		elif property.name == 'rect':
			if self._rect is not value:
				self._disconnect_rect(self._rect)
				self._rect = value
				self._connect_rect(self._rect)
		elif property.name in ('x','y','width','height'):
			setattr(self._rect, property.name, value)
		elif property.name in ('pixel','red','green','blue'):
			setattr(self._color, property.name, value)
		else:
			raise AttributeError, 'unknown property %s' % property.name
gobject.type_register(Box)
