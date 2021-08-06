#-*- coding: utf-8 -*-
"""
doc string
"""
from __future__ import with_statement, division, absolute_import
import gtk, gobject, sys, cairo
from glade import CustomWidget

__all__ = 'ImageSpace',

def RGBA(*p):
	rv = 0L
	for n in p:
		print map(hex,p),hex(n),hex(rv)
		rv = (rv << 8) | (n & 0xFF)
	return rv

# A "method" for Drawable
def draw_alpha_rectangle_gdk(self, gc, color, filled, x, y, width, height, alpha):
	"""
	Takes care of the tedious task drawing a rectangle with a semi-transparent 
	filling.
	
	Nearly identical to draw_rectangle, except for:
	* alpha - the alpha that should be used, 0-255
	
	Uses non-filled sizing rules.
	"""
	# I feel like there should be a more efficient way to do this
	if filled or alpha == 0:
		pb = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, True, 8, width+1, height+1)
		c = color
		print c, c.to_string(), map(hex, [c.red, c.green, c.blue])
		print hex(RGBA(c.red//256, c.green//256, c.blue//256, alpha))
		pb.fill(RGBA(c.red//256, c.green//256, c.blue//256, alpha))
		self.draw_pixbuf(gc, pb, 0,0, x,y, -1,-1)
		del pb
	gc 
	self.draw_rectangle(gc, False, x, y, width, height)

class ImageSpace(gtk.Widget):
	"""
	Displays an image and allows the user to draw boxes over it.
	
	TODO: Add Special Modes
	"""
	# Constants
	SELECT, INSERT = MODES = range(2)
	__gsignals__ = { 'realize': 'override',
	                 'expose-event' : 'override',
	                 'size-allocate': 'override',
	                 'size-request' : 'override',}
	__gproperties__ = {
		'zoom' : (gobject.TYPE_DOUBLE,
		           'view zoom',
		           'the amount of zoom. 1.0 is normal',
		           0.0,
		           10.0, # A really big number
		           1.0,
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'image' : (gtk.gdk.Pixbuf,
		           'the image to draw',
		           'the background image',
		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
#		'overlap' : (gobject.TYPE_BOOLEAN,
#		           'allow overlapping boxes',
#		           'Should boxes be allowed to overlap?',
#		           gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE),
		'mode' : (gobject.TYPE_UINT,
		           'current mode',
		           'The current user interaction mode. either selecting or inserting.',
		           min(MODES),
		           max(MODES),
		           SELECT,
		           gobject.PARAM_READWRITE),
		'alpha' : (gobject.TYPE_UINT,
		           'current mode',
		           'The current user interaction mode. either selecting or inserting.',
		           0,
		           255,
		           127,
		           gobject.PARAM_READWRITE),
		'model' : (gobject.TYPE_OBJECT, #gtk.TreeModel,
		           'data model',
		           'The model where boxes are pulled from.',
		           gobject.PARAM_READWRITE),
		'color_col' : (gobject.TYPE_UINT,
		           'color column',
		           'The column to pull colors from.',
		           0,
		           sys.maxint,
		           8,
		           gobject.PARAM_READWRITE),
		'rect_col' : (gobject.TYPE_UINT,
		           'box column',
		           'The column to pull rectangles from.',
		           0,
		           sys.maxint,
		           7,
		           gobject.PARAM_READWRITE),
		}
	
	prop = lambda name: property((lambda s: s.get_property(name)), (lambda s,v: s.set_property(name,v)))
	
	zoom = prop('zoom')
	image = prop('image')
	overlap = prop('overlap')
	mode = prop('mode')
	alpha = prop('alpha')
	model = prop('model')
	color_col = prop('alpha')
	rect_col = prop('model')
	
	del prop
	
	_image = _zoom = _mode = _alpha = _model = _color_col = _rect_col = None
	
	def __init__(self, image=None, model=None, color=8, rect=7):
#		print "__init__", self, image, model, color, rect
		gtk.Widget.__init__(self)
		self._image = image
		self._zoom = 1.0
		self._mode = self.SELECT
		self._alpha = 127
		self._model = model
		self._color_col = color
		self._rect_col = rect
		self.cr = None
		self._update()
	
	def do_get_property(self, property):
		if hasattr(self, '_'+property.name):
			return getattr(self, '_'+property.name)
		else:
			raise AttributeError, 'unknown property %s' % property.name
	
	def do_set_property(self, property, value):
		if property.name == 'mode':
			if value in self.MODES:
				self._mode = value
			else:
				raise ValueError, 'mode must be one of %s' % self.MODES
		elif hasattr(self, '_'+property.name):
			setattr(self, '_'+property.name, value)
			self._update()
		else:
			raise AttributeError, 'unknown property %s' % property.name		
	
	def do_realize(self):
		# The do_realize method is responsible for creating GDK (windowing system)
		# resources. In this example we will create a new gdk.Window which we
		# then draw on
		
		# First set an internal flag telling that we're realized
		self.set_flags(self.flags() | gtk.REALIZED)
		
		# Create a new gdk.Window which we can draw on.
		# Also say that we want to receive exposure events by setting
		# the event_mask
		self.window = gtk.gdk.Window(
			self.get_parent_window(),
			width=self.allocation.width,
			height=self.allocation.height,
			window_type=gtk.gdk.WINDOW_CHILD,
			wclass=gtk.gdk.INPUT_OUTPUT,
			event_mask=self.get_events() | gtk.gdk.EXPOSURE_MASK)
#			         | gtk.gdk.BUTTON1_MOTION_MASK | gtk.gdk.BUTTON_PRESS_MASK
#			         | gtk.gdk.POINTER_MOTION_MASK
#			         | gtk.gdk.POINTER_MOTION_HINT_MASK)
		
		# Associate the gdk.Window with ourselves, Gtk+ needs a reference
		# between the widget and the gdk window
		self.window.set_user_data(self)
		
		# Attach the style to the gdk.Window, a style contains colors and
		# GC contextes used for drawing
		self.style.attach(self.window)

		# The default color of the background should be what
		# the style (theme engine) tells us.
		self.style.set_background(self.window, gtk.STATE_NORMAL)
		self.window.move_resize(*self.allocation)
		
		# Some extra stuff
		self.gc = self.style.fg_gc[gtk.STATE_NORMAL]
		try:
			self.cr = self.window.cairo_create()
		except AttributeError:
			self.cr = None

	
	def do_unrealize(self):
        # The do_unrealized method is responsible for freeing the GDK resources

        # De-associate the window we created in do_realize with ourselves
		self.window.set_user_data(None)
		#self.window.destroy()
	
	def do_size_request(self, requisition):
		# The do_size_request method Gtk+ is calling on a widget to ask
		# it the widget how large it wishes to be. It's not guaranteed
		# that gtk+ will actually give this size to the widget

		# In this case, we say that we want to be as big as the
		# text is, plus a little border around it.
		if self._image is not None:
			requisition.width = self._image.get_width() * self._zoom
			requisition.height = self._image.get_height() * self._zoom
	
	def do_size_allocate(self, allocation):
		# The do_size_allocate is called by when the actual size is known
		# and the widget is told how much space could actually be allocated

		#Save the allocated space
		self.allocation = allocation
		# If we're realized, move and resize the window to the
		# requested coordinates/positions
		if self.flags() & gtk.REALIZED:
			self.window.move_resize(*allocation)
	
	def zoom_to_size(self, *p):
		"""is.zoome_to_size() -> None
		Adjusts the zoom so the image fills the allocation.
		"""
		if self._image is None or self.allocation is None:
			return
		self.zoom = min(
			self.allocation.width/self._image.get_width(),
			self.allocation.height/self._image.get_height()
			)
	
	def do_expose_event(self, event):
		# The do_expose_event is called when the widget is asked to draw itself
		# Remember that this will be called a lot of times, so it's usually
		# a good idea to write this code as optimized as it can be, don't
		# create any resources in here.
		
		if self.cr is None:
			print "Using GDK"
			return self._expose_gdk(event)
		else:
			print "Using Cairo"
			# For w/e reason, this has to be created every time
			self.cr = self.window.cairo_create()
			return self._expose_cairo(event)
	
	def _expose_gdk(self, event):
		z = self._zoom
		# Draw image
		if self._image is not None:
			# Center
			dx = (self.allocation.width - self._image.get_width()*z) // 2
			dy = 0
			self.window.draw_pixbuf(self.gc, self._scaled,
				0,0,
				dx,dy)
		
		# Draw boxes on top of it
		gc = self.window.new_gc()
		gc.copy(self.gc)
		def draw_box(model, path, row, self):
			color = model.get(row, self.color_col)
			rect = model.get(row, self.rect_col)
			draw_alpha_rectangle_gdk(self.window, gc, color, True, 
				rect.x*z, rect.y*z, (rect.width+1)*z, (rect.height+1)*z, self._alpha)
		if self._model is not None:
			self._model.foreach(draw_box, self)
	
	def _expose_cairo(self, event):
		cr = self.cr
		alloc = self.allocation
		img = self._image
		z = self._zoom
		# Do some translation
		if img is not None:
			# Center
			dx = (alloc.width/z - img.get_width()) / 2
			dy = (alloc.height/z - img.get_height()) / 2
		else:
			dx = alloc.width / 2
			dy = alloc.height / 2
		cr.translate(dx,dy)
		cr.scale(z, z)
		linewidth = 1.0/z
		
		# Draw image
		if img is not None:
			cr.set_source_pixbuf(img, 0, 0) # set_source_surface()
			cr.rectangle((0,0,img.get_width(), img.get_height()))
			cr.fill()
		
		# Draw boxes on top of it
		# This all works
		cr.set_line_width(linewidth)
		cr.set_line_join(cairo.LINE_JOIN_MITER)
		def draw_box(model, path, row, self):
			c,r = model.get(row, self._color_col, self._rect_col)
			r = gtk.gdk.Rectangle(*r)
			r.width += 1
			r.height += 1
			# draw border
			cr.set_source_rgba(c.red/0xFFFF, c.green/0xFFFF, c.blue/0xFFFF, 1.0)
			cr.rectangle(r)
			cr.stroke()
			# draw fill
			if self._alpha > 0:
				cr.set_source_rgba(c.red/0xFFFF, c.green/0xFFFF, c.blue/0xFFFF, self._alpha/0xFF)
				cr.rectangle(r)
				cr.fill()
		
		if self._model is not None:
			self._model.foreach(draw_box, self)
	
	def _update(self):
		# Called when zoom or image changes
		if self._image is not None and not hasattr(self.window, 'cairo_create'):
			if self._zoom == 1.0:
				self._scaled = self._image
			else:
				self._scaled = self._image.scale_simple(
					int(self._image.get_width() * self._zoom),
					int(self._image.get_height() * self._zoom),
					gtk.gdk.INTERP_HYPER)
		else:
			self._scaled = None
		if self.flags() & gtk.REALIZED:
			self.window.invalidate_rect(self.allocation, True)
CustomWidget(ImageSpace)

if __name__ == "__main__":
	from box import Box
	from boxmodel import BoxListStore
	win = gtk.Window()
	win.set_border_width(5)
	win.set_title('Widget test')
	win.connect('delete-event', gtk.main_quit)
	win.connect('size-allocate', lambda *p: w.zoom_to_size())
	
	frame = gtk.Frame("Example frame")
	win.add(frame)
	
	bls = BoxListStore()
	bls.append(['', Box(gtk.gdk.Rectangle(124,191,248,383), gtk.gdk.color_parse('#F0A'))])
	print "Model data:", map(tuple, bls)
	w = ImageSpace(model=bls)
	w.alpha = 0x55
	w.zoom = 0.5
	w.image=gtk.gdk.pixbuf_new_from_file('test.gif')
	frame.add(w)
	
	win.show_all()
#	w.zoom_to_size()
	print 'Window:', w.window
	gtk.main()

