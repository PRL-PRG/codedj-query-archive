#-*- coding: utf-8 -*-
from __future__ import with_statement, division, absolute_import
import gtk, gobject, sys
#from gobject.propertyhelper import property as gprop
from usefulgprop import property as gprop

__all__ = 'Box',

class Box(gobject.GObject):
	"""
	Similar to gtk.gdk.Rectangle, but also has a color associated with it.
	"""
	rect = gprop(
		type=gtk.gdk.Rectangle,
		nick='the space taken',
		blurb='A gtk.gdk.Rectangle of the same size and position.',
		flags=gobject.PARAM_READWRITE,
#		default=gtk.gdk.Rectangle(),
		)
	color = gprop(
		type=gtk.gdk.Color,
		nick='the color',
		blurb='The gtk.gdk.Color for the color.',
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE,
#		default=gtk.gdk.Color(),
		)
	# Shortcuts
	width = gprop(
		type=gobject.TYPE_INT,
		getter=(lambda self: self.rect.width),
		setter=(lambda self, value: setattr(self.rect,'width', value) if self.rect else None),
		nick='width of the box',
		blurb='the width of the box in pixels',
		default=0,
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	height = gprop(
		type=gobject.TYPE_INT,
		getter=(lambda self: self.rect.height),
		setter=(lambda self, value: setattr(self.rect,'height', value) if self.rect else None),
		nick='height of the box',
		blurb='the height of the box in pixels',
		default=0,
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	x = gprop(
		type=gobject.TYPE_INT,
		getter=(lambda self: self.rect.x),
		setter=(lambda self, value: setattr(self.rect,'x', value) if self.rect else None),
		nick='',
		blurb='',
		default=0,
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	y = gprop(
		type=gobject.TYPE_INT,
		getter=(lambda self: self.rect.y),
		setter=(lambda self, value: setattr(self.rect,'y', value) if self.rect else None),
		nick='',
		blurb='',
		default=0,
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	pixel = gprop(
		type=gobject.TYPE_ULONG,
		getter=(lambda self: self.color.pixel),
		setter=(lambda self, value: setattr(self.color,'pixel', value) if self.color else None),
		nick='',
		blurb='',
		minimum=0,
		maximum=0xFFFFFF,
		default=0,
		flags=gobject.PARAM_READWRITE|gobject.PARAM_CONSTRUCT
		)
	red = gprop(
		type=gobject.TYPE_UINT,
		getter=(lambda self: self.color.red),
		setter=(lambda self, value: setattr(self.color,'red', value) if self.color else None),
		nick='',
		blurb='',
		minimum=0,
		maximum=0xFFFF,
		default=0,
		flags=gobject.PARAM_READWRITE|gobject.PARAM_CONSTRUCT
		)
	green = gprop(
		type=gobject.TYPE_UINT,
		getter=(lambda self: self.color.green),
		setter=(lambda self, value: setattr(self.color,'green', value) if self.color else None),
		nick='',
		blurb='',
		minimum=0,
		maximum=0xFFFF,
		default=0,
		flags=gobject.PARAM_READWRITE|gobject.PARAM_CONSTRUCT
		)
	blue = gprop(
		type=gobject.TYPE_UINT,
		getter=(lambda self: self.color.blue),
		setter=(lambda self, value: setattr(self.color,'blue', value) if self.color else None),
		nick='',
		blurb='',
		minimum=0,
		maximum=0xFFFF,
		default=0,
		flags=gobject.PARAM_READWRITE|gobject.PARAM_CONSTRUCT
		)
	
	def __init__(self,rect=None,color=None):
		gobject.GObject.__init__(self)
		self.rect = rect if rect is not None else gtk.gdk.Rectangle()
		self.color = color if color is not None else gtk.gdk.Color()
	
	def _rect_notify(self,obj,prop):
		if obj is self.rect and prop.name in ('x','y','width','height'):
			self.emit('notify::'+prop.name, prop)
	def _connect_rect(self, rect):
#		self.rect_nid = rect.connect('notify', self._rect_notify)
		pass
	def _disconnect_rect(self, rect):
#		rect.disconnect(self._rect_nid)
#		del self._rect_nid
		pass
	
	def _color_notify(self,obj,prop):
		if obj is self._color and prop.name in ('pixel','red','green','blue'):
			self.emit('notify::'+prop.name, prop)
	def _connect_color(self, color):
#		self._color_nid = color.connect('notify', self._color_notify)
		pass
	def _disconnect_color(self, color):
#		color.disconnect(self._color_nid)
#		del self._color_nid
		pass
	
	def __repr__(self):
		return "<%s rect=(%i,%i, %i,%i) color=(0x%X, 0x%X, 0x%X)>" % (
			type(self).__name__, 
			self.rect.x, self.rect.y, self.rect.width, self.rect.height,
			self.color.red, self.color.green, self.color.blue
			)
	
	def dimensions_text(self):
		r = self.rect
		return u'%i,%i\N{RIGHTWARDS ARROW}%i,%i (%i\N{VECTOR OR CROSS PRODUCT}%i)' % \
			(r.x, r.y, r.x+r.width, r.y+r.height, r.width, r.height)

#print dir(Box)
#print Box.props, list(Box.props)
#print Box.__gtype__, dir(Box.__gtype__), Box.__gtype__.fundamental
