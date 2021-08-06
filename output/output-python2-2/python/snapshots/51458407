#-*- coding: utf-8 -*-
"""
The ImageSpace control. Handles the image and the selection boxes on top.

Ignore the errant red outline when not running in optimized mode, it's just our 
change-rect debugger.
"""
from __future__ import with_statement, division, absolute_import
import gtk, gobject, sys, cairo
from glade import CustomWidget
from rectutils import *
from box import Box
#from gobject.propertyhelper import property as gprop
from usefulgprop import property as gprop

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

def treeview_rubber_band(widget, rect, c):
	"""
	From gtk_tree_view_paint_rubber_band()
	"""
	cr = widget.window.cairo_create()
	cr.set_line_width(1.0)
	
	cr.set_source_rgba(
			widget.style.fg[gtk.STATE_NORMAL].red / 65535,
			widget.style.fg[gtk.STATE_NORMAL].green / 65535,
			widget.style.fg[gtk.STATE_NORMAL].blue / 65535,
			.25)
	
	cr.rectangle(rect)
	cr.clip()
	cr.paint()
	
	cr.set_source_rgb(
			widget.style.fg[gtk.STATE_NORMAL].red / 65535,
			widget.style.fg[gtk.STATE_NORMAL].green / 65535,
			widget.style.fg[gtk.STATE_NORMAL].blue / 65535)
	
	cr.rectangle(
			rect.x + 0.5, rect.y + 0.5,
			rect.width - 1, rect.height - 1)
	cr.stroke();
	
	del cr

def iconview_rubber_band(widget, rect, cr):
	"""
	From gtk_icon_view_paint_rubberband()
	"""
	# Style properties
	fill_color_gdk = widget.style_get_property("selection-box-color")
	fill_color_alpha = widget.style_get_property("selection-box-alpha")

	if not fill_color_gdk:
		fill_color_gdk = widget.style.base[gtk.STATE_SELECTED].copy()

	cr.set_source_rgba(
			fill_color_gdk.red / 65535,
			fill_color_gdk.green / 65535,
			fill_color_gdk.blue / 65535,
			fill_color_alpha / 255)

	cr.save()
	cr.rectangle(rect)
	cr.clip()
	cr.paint()

	# Draw the border without alpha
	cr.set_source_rgb(
			fill_color_gdk.red / 65535,
			fill_color_gdk.green / 65535,
			fill_color_gdk.blue / 65535)
	cr.rectangle(
			rect.x + 0.5, rect.y + 0.5,
			rect.width - 1, rect.height - 1)
	cr.stroke()
	cr.restore()

	del fill_color_gdk

#class ImageSpaceModes(gobject.GEnum):
#	__enum_values__ = {
#		# int : GEnum
#		0: 'SELECT',
#		1: 'INSERT',
#		}
#	#def __new__(cls, value); value must be int
#	# Note that this requires calling some C-level functions and allocating a new GType

class ImageSpace(gtk.Widget):
	"""
	Displays an image and allows the user to draw boxes over it.
	
	TODO: Add Special Modes
	"""
	# Constants
	SELECT, INSERT = MODES = range(2) #map(ImageSpaceModes, xrange(2))
	__gsignals__ = {
		'box-added'    : (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (Box,)),
		'insert-box-changed': (gobject.SIGNAL_RUN_LAST|gobject.SIGNAL_ACTION, gobject.TYPE_NONE, (Box,)),
		'realize'      : 'override',
		'expose-event' : 'override',
		'size-allocate': 'override',
		'size-request' : 'override',
#		'query-tooltip': 'override',
		}
	zoom = gprop(
		type=gobject.TYPE_DOUBLE,
		nick='view zoom',
		blurb='the amount of zoom. 1.0 is normal',
		minimum=0.0,
		maximum=10.0, # A really big number
		default=1.0,
		flags=gobject.PARAM_READWRITE
		)
	image = gprop(
		type=gtk.gdk.Pixbuf,
		nick='the image to draw',
		blurb='the background image',
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
#	overlap = gprop(
#		type=gobject.TYPE_BOOLEAN,
#		nick='allow overlapping boxes',
#		blurb='Should boxes be allowed to overlap?',
#		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
#		)
	mode = gprop(
		type=gobject.TYPE_UINT,
		nick='current mode',
		blurb='The current user interaction mode. either selecting or inserting.',
		minimum=min(MODES),
		maximum=max(MODES),
		default=SELECT,
		flags=gobject.PARAM_READWRITE
		)
	# This is a style property
	alpha = gprop(
		type=gobject.TYPE_UINT,
		nick='alpha',
		blurb='The alpha used when drawing the interrior of boxes.',
		minimum=0,
		maximum=255,
		default=64,
		flags=gobject.PARAM_READWRITE
		)
	def _set_model(self, value):
			if self.model is not None:
				self._disconnect_model(self.model)
			type(self).model._default_setter(self, value)
			if value is not None:
				self._connect_model(value)
			if self.flags() & gtk.REALIZED:
				self.queue_draw()
	model = gprop(
		type=gobject.TYPE_OBJECT, #gtk.TreeModel,
		getter=Ellipsis,
		setter=_set_model,
		nick='data model',
		blurb='The model where boxes are pulled from.',
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	def _set_selection(self, value):
			if self.selection is not None:
				self._disconnect_selection(self.selection)
			type(self).selection._default_setter(self, value)
			if value is not None:
				self._connect_selection(value)
			if self.flags() & gtk.REALIZED:
				self.queue_draw()
	selection = gprop(
		type=gobject.TYPE_OBJECT, #gtk.TreeSelection,
		getter=Ellipsis,
		setter=_set_selection,
		nick='tree selection',
		blurb='Selection from a TreeView. If None, manage selection ourselves.',
		flags=gobject.PARAM_READWRITE
		)
	box_col = gprop(
		type=gobject.TYPE_UINT,
		nick='color column',
		blurb='The column to pull colors from.',
#		minimum=0,
#		maximum=sys.maxint,
		default=1,
		flags=gobject.PARAM_CONSTRUCT|gobject.PARAM_READWRITE
		)
	next_color = gprop(
		type=gtk.gdk.Color,
		nick='next color',
		blurb='The color the next box will be. I suggest setting this in the box-added signal.',
		flags=gobject.PARAM_READWRITE
		)
	
	_insert_start_coords = None
	_temporary_box = None # Used for adding boxes
	_current_box = None # The box we're hovering over, possibly chosen arbitrarily
	_model_listeners = None
	_selection_listeners = None
	_pbl_handlers = None # signal handles for PixbufLoader
	
	def __init__(self, image=None, model=None, box=1):
#		print "__init__", self, image, model, color, rect
		gtk.Widget.__init__(self)
		# Properties
		self.image = image
		self.model = model
		self.box_col = box
		self.next_color = gtk.gdk.color_parse('#0f0')
		# Other attributes
		self.cr = None
		# other stuff
		self._update()
		self.connect('notify::zoom', 
			lambda self, prop: self.queue_draw_area(*self.allocation) if self.flags() & gtk.REALIZED else None)
	
	def _connect_model(self, model):
		self._model_listeners = (
			model.connect('row-changed', self._model_changed),
			model.connect('row-deleted', self._model_changed),
			model.connect('row-inserted', self._model_changed),
			)
	def _disconnect_model(self, model):
		for l in self._model_listeners:
			model.disconnect(l)
		self._model_listeners = ()
	
	def _connect_selection(self, sel):
		self._selection_listeners = (
			sel.connect('changed', self._selection_changed),
			)
	def _disconnect_selection(self, sel):
		for l in self._selection_listeners:
			selection.disconnect(l)
		self._selection_listeners = ()
	
	def loadfrompixbuf(self, pbloader):
		"""is.loadfrompixbuf(PixbufLoader) -> None
		Prepares the ImageSpace to load an image from PixbufLoader. The caller 
		is expected to create the PixbufLoader and call its write() method.
		"""
		self.image = None
		self._pbl_handlers = (
			pbloader.connect('area-prepared', self.pbl_do_prepared),
			pbloader.connect('area-updated', self.pbl_do_updated),
			pbloader.connect('closed', self.pbl_do_closed),
			)
	
	def pbl_do_prepared(self, pbloader):
		self.image = pbloader.get_pixbuf()
	
	def pbl_do_updated(self, pbloader, x, y, width, height):
		if self.flags() & gtk.REALIZED:
			redraw = self.rect2widget(frect(x,y-1,width,height+1)) # Go back one row
			self.queue_draw_area(*redraw)
	
	def pbl_do_closed(self, pbloader):
		for h in self._pbl_handlers:
			pbloader.disconnect(h)
		self._pbl_handlers = None
	
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
			event_mask=self.get_events() | gtk.gdk.EXPOSURE_MASK
			         | gtk.gdk.BUTTON1_MOTION_MASK
			         | gtk.gdk.BUTTON_PRESS_MASK | gtk.gdk.BUTTON_RELEASE_MASK
			         | gtk.gdk.POINTER_MOTION_MASK
			         | gtk.gdk.POINTER_MOTION_HINT_MASK)
		
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
		#self.connect("motion_notify_event", self.do_motion_notify_event)
#		self.connect("query-tooltip", self.do_query_tooltip_event)
		try:
			self.cr = self.window.cairo_create()
		except AttributeError:
			self.cr = None
		#self.set_tooltip_text('spam&eggs')
	
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
		if self.image is not None:
			requisition.width = self._image.get_width() * self.zoom
			requisition.height = self._image.get_height() * self.zoom
	
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
		if self.image is None or self.allocation is None:
			return
		self.zoom = min(
			self.allocation.width/self.image.get_width(),
			self.allocation.height/self.image.get_height()
			)
		if self.flags() & gtk.REALIZED:
			self.queue_draw_area(*self.allocation)

	
	def do_expose_event(self, event):
		# The do_expose_event is called when the widget is asked to draw itself
		# Remember that this will be called a lot of times, so it's usually
		# a good idea to write this code as optimized as it can be, don't
		# create any resources in here.
		
		if self.cr is None:
			return self._expose_gdk(event)
		else:
			# For w/e reason, this has to be created every time
			self.cr = self.window.cairo_create()
			return self._expose_cairo(event)
	
	def _expose_gdk(self, event):
		z = self.zoom
		# Draw image
		if self.image is not None:
			# Center
			dx = (self.allocation.width - self.image.get_width()*z) // 2
			dy = 0
			self.window.draw_pixbuf(self.gc, self._scaled,
				0,0,
				dx,dy)
		
		# Draw boxes on top of it
		gc = self.window.new_gc()
		gc.copy(self.gc)
		def draw_box(model, path, row, self):
			box = model.get(row, int(self.box_col))
			rect = box.rect
			draw_alpha_rectangle_gdk(self.window, gc, box.color, True, 
				rect.x*z, rect.y*z, (rect.width+1)*z, (rect.height+1)*z, self.alpha)
		if self.model is not None:
			self.model.foreach(draw_box, self)
	
	SELECTSIZE = 2.0
	TEMP_IS_SELECTED = False
	def _expose_cairo(self, event):
		cr = self.cr
		alloc = self.allocation
		img = self.image
		z = self.zoom
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
		def draw_box_border(self, c, r, s):
			if s:
				cr.set_line_width(linewidth*self.SELECTSIZE)
			else:
				cr.set_line_width(linewidth)
			# draw border
			cr.set_source_rgba(c.red/0xFFFF, c.green/0xFFFF, c.blue/0xFFFF, 1.0)
			cr.rectangle(r)
			cr.stroke()
		def draw_box_fill(self, c, r, s):
			# draw fill
			if self.alpha > 0:
				cr.set_source_rgba(c.red/0xFFFF, c.green/0xFFFF, c.blue/0xFFFF, self.alpha/0xFF)
				cr.rectangle(r)
				cr.fill()
		
		boxes = []
		def draw_box_row(model, path, row, self):
			box, = model.get(row, int(self.box_col))
			c = box.color
			r = box.rect
			r = gtk.gdk.Rectangle(*r)
			r.width += 1
			r.height += 1
			s = False
			if self.selection is not None:
				s = self.selection.iter_is_selected(row)
			boxes.append((c,r,s))
		if self.model is not None:
			self.model.foreach(draw_box_row, self)
			for c,r,s in boxes:
				draw_box_fill(self,c,r,s)
		if self._temporary_box is not None:
			draw_box_fill(self, self._temporary_box.color, self._temporary_box.rect, self.TEMP_IS_SELECTED)
		for c,r,s in boxes:
			draw_box_border(self,c,r,s)
		if self._temporary_box is not None:
			draw_box_border(self, self._temporary_box.color, self._temporary_box.rect, self.TEMP_IS_SELECTED)
		if __debug__:
			if self._changed_rect is not None:
				draw_box_border(self, gtk.gdk.color_parse('#F00'), self._changed_rect, False)
	
	def _update(self):
		# Called when zoom or image changes
		if self.image is not None and not hasattr(self.window, 'cairo_create'):
			if self.zoom == 1.0:
				self._scaled = self.image
			else:
				self._scaled = self.image.scale_simple(
					int(self.image.get_width() * self.zoom),
					int(self.image.get_height() * self.zoom),
					gtk.gdk.INTERP_HYPER)
		else:
			self._scaled = None
		if self.flags() & gtk.REALIZED:
			self.window.invalidate_rect(self.allocation, True)
	
	def img2widgetcoords(self, x,y):
		"""is.img2widgetcoords(num,num) -> (num, num)
		Converts the given (x,y) from image coordinates to widget coordinates 
		(suitable for using with GTK).
		
		Inverse of widget2imgcoords().
		"""
		# Change to centered-origin
		if self.image is not None:
			x -= self.image.get_width() * self.zoom / 2
			y -= self.image.get_height() * self.zoom / 2
		# Scale
		x *= self.zoom
		y *= self.zoom
		# Change to Widget's origin
		x += self.allocation.width / 2
		y += self.allocation.height / 2
		return x,y
	
	def widget2imgcoords(self, x,y):
		"""is.img2widgetcoords(num,num) -> (num, num)
		Converts the given (x,y) from widget coordinates (suitable for using 
		with GTK) to image coordinates.
		
		Inverse of img2widgetcoords().
		"""
		# Change to centered-origin
		x -= self.allocation.width / 2
		y -= self.allocation.height / 2
		# Scale
		x *= self.zoom
		y *= self.zoom
		# Change to Image's origin
		if self.image is not None:
			x += self.image.get_width() * self.zoom / 2
			y += self.image.get_height() * self.zoom / 2
		return x,y
	
	def rect2img(self, rect):
		x,y = self.widget2imgcoords(rect.x, rect.y)
		w = rect.width * self.zoom
		h = rect.height * self.zoom
		return frect(x,y,w,h)
	
	def rect2widget(self, rect):
		x,y = self.img2widgetcoords(rect.x, rect.y)
		w = rect.width / self.zoom
		h = rect.height / self.zoom
		return frect(x,y,w,h)
	
	def alloc2img(self):
		"""is.alloc2img() -> Rectangle
		Translates allocation to the images coordinates.
		"""
		return self.rect2img(self.allocation)
	
	def find_boxes_under_coord(self,x,y):
		"""is.find_boxes_under_coord(num,num) -> [Box]
		Returns all of the boxes underneath image location (x,y).
		"""
		return tuple(r[self.box_col] for r in self.model if rect_contains(r[self.box_col].rect,x,y))
	
	RESIZE_RANGE = 5
	def find_boxes_coord_near(self, x,y, range=None):
		"""is.find_boxes_coord_near(num,num) -> (Box, dir), ...
		Returns all of the boxes which:
		* are underneath image location (x,y)
		* have an edge near image location (x,y)
		
		If given, range is how close to the edge we need to be (in widget 
		pixels).
		
		dir is intern()'d 'N'orth, 'S'outh, 'E'ast, 'W'est, 'NE', 'NW', 'SW', 
		'SE'.
		
		If the location given is within range of two opposite sides (ie skinny 
		boxes), then the closer of the two is returned
		"""
		if range is None: range = self.RESIZE_RANGE
		range /= self.zoom
		
		
		for box in self.find_boxes_under_coord(x,y):
			dir = ''
			if box.height < range*2:
				# Skinny!
				dir += 'N' if y - box.y < box.y+box.height - y else 'S'
			elif y - box.y <= range:
				dir += 'N'
			elif box.y+box.height - y <= range:
				dir += 'S'
			if box.width < range*2:
				# Skinny!
				dir = 'W' if x - box.x < box.x+box.width - x else 'E'
			elif x - box.x <= range:
				dir = 'W'
			elif box.x+box.width - x <= range:
				dir = 'E'
#			print "find_boxes_coord_near: box, dir: (%r,%r) %r, %r    \r" % (x,y,box, dir),
			sys.stdout.flush()
			if len(dir):
				yield box, intern(dir)
	
	def _model_changed(self, model, path, iter=None):
		self._changed_rect = None
		if not self.flags() & gtk.REALIZED: return
		if iter is not None:
			self.queue_draw_area(*self.rect2widget(self.model.get(iter, int(self.box_col))[0].rect))
		else:
			self.queue_draw_area(*self.allocation)
	
	def _selection_changed(self, selection):
		if not self.flags() & gtk.REALIZED: return
		self.queue_draw_area(*self.allocation)
	
	_boxes_under_cursor = None
	
	def get_tooltip_text(self, boxes):
		if len(boxes) == 0:
			return None
		return '\n'.join(b.dimensions_text() for b in boxes)
	
	def do_query_tooltip(self, x,y, keyboard_mode, tooltip, _=None):
		# If widget wasn't passed as self
		if _ is not None: x,y, keyboard_mode, tooltip = y, keyboard_mode, tooltip, _
		print 'do_query_tooltip',self, x,y, keyboard_mode, tooltip
		ix,iy = self.widget2imgcoords(x,y)
		boxes = self.find_boxes_under_coord(ix,iy)
		if len(boxes) == 0:
			return False
		tooltip.set_text(self.get_tooltip_text(boxes))
		return True
	
	_changed_rect = None
	
	def _update_boxes(self, x,y):
		"""
		Handles the fairly complex algorithm used to cache-and-calculate the 
		boxes that are underneath the cursor.
		
		Current Caching: A rectangle for which the current state is true.
		"""
		alloc = self.alloc2img()
		
		if not rect_contains(alloc, x,y):
			# The mouse has left the widget
			self._changed_rect = None
			self._boxes_under_cursor = []
			return True
		
		if self._changed_rect is None or not rect_contains(self._changed_rect, x, y):
			if len(self.model) == 0: return False
			# The mouse left the common area
#			print '(%i,%i)' % (x,y),
			
#			print "Old rect:", tuple(self._changed_rect) if self._changed_rect is not None else self._changed_rect,
			self._changed_rect = None
				
			
			# Calculate new boxes
			newboxes = self.find_boxes_under_coord(x,y)
			self._boxes_under_cursor = newboxes
#			print "newboxes:", newboxes,
			
			# Update the caching rectangle
			if len(newboxes):
				changed = newboxes[0].rect
			else: # Outside of any boxes, use allocation
				changed = alloc
			for b in newboxes[1:]:
				changed = changed.intersect(b.rect)
			for r in self.model:
				b = r[self.box_col]
				if b not in newboxes:
					changed = rect_diff(changed, b.rect, (x,y))
			if changed == alloc: # This is so extrodinarily BAD that we should test for it.
				from warnings import warn
				warn("The chosen change rect was the allocation. THIS SHOULD'T HAPPEN.")
				changed = None
#			print "Change rect:", tuple(changed)
			self._changed_rect = changed
			if __debug__: # If debugging, redraw every time the box changes
				self.queue_draw_area(*self.allocation)
			assert changed is None or rect_contains(changed, x,y)
			return True
		else:
			return False
	
	def get_boxes_under_cursor(self,x=None,y=None):
		"""is.get_boxes_under_cursor() -> (Box, ...)
		Return the list of boxes currently under the cursor, in some order
		"""
		if not self._boxes_under_cursor or not self._changed_rect or (x is not None and y is not None):
			# It doesn't matter if these are way off: if the mouse is outside 
			# the cache box, it'll be recalculated.
			if x is None or y is None:
				x,y,_ = self.window.get_pointer()
			x,y = self.widget2imgcoords(x,y)
			self._update_boxes(x,y)
		return self._boxes_under_cursor[:]
	
	RESIZE_CURSORS = {
		'N' :gtk.gdk.TOP_SIDE,
		'S' :gtk.gdk.BOTTOM_SIDE,
		'E' :gtk.gdk.RIGHT_SIDE,
		'W' :gtk.gdk.LEFT_SIDE,
		'NE':gtk.gdk.TOP_RIGHT_CORNER,
		'NW':gtk.gdk.TOP_LEFT_CORNER,
		'SE':gtk.gdk.BOTTOM_RIGHT_CORNER,
		'SW':gtk.gdk.BOTTOM_LEFT_CORNER,
		}
	
	_rubber_band_start = None
	_box_may_resize = None
	_box_are_resizing = None
	_box_may_resize_dir = None
	_box_are_resizing_dir = None
	def do_motion_notify_event(self, event):
		"""
		Handles updating all the cached info dealing with the mouse (eg, boxes, 
		tooltips).
		"""
		# if this is a hint, then let's get all the necessary 
		# information, if not it's all we need.
		if event.is_hint:
			x, y, state = event.window.get_pointer()
		else:
			x = event.x
			y = event.y
			state = event.state
		
		# Update box underneath cursor, for tooltip
		ix, iy = icoords = self.widget2imgcoords(x,y)
		if self._update_boxes(*icoords):
			self.set_tooltip_text(self.get_tooltip_text(self._boxes_under_cursor))
			self.trigger_tooltip_query()
		
		if self.mode == self.INSERT and self._insert_start_coords is not None and state & gtk.gdk.BUTTON1_MASK:
			# Adjust temporary box
			nr = pt2rect(icoords, self._insert_start_coords)
			redraw = nr.union(self._temporary_box.rect)
			self._temporary_box.rect = nr
			#self.queue_draw_area(*redraw)
			self.queue_draw()
			self.emit('insert-box-changed', self._temporary_box)
		elif self._box_are_resizing is not None and state & gtk.gdk.BUTTON1_MASK:
			d = self._box_are_resizing_dir
			b = self._box_are_resizing
			obox = frect(*b.rect)
			if 'W' in d:
				b.x, b.width = round(ix), round(b.x + b.width - ix)
			elif 'E' in d:
				b.width = round(ix - b.x)
			if 'N' in d:
				b.y, b.height = round(iy), round(b.y + b.height - iy)
			elif 'S' in d:
				b.height = round(iy - b.y)
			print "Resizing: %r (%r,%r) (%r,%r) %r->%r" % (d, x,y, ix,iy, list(obox), list(b.rect))
			self.queue_draw_area(*self.rect2widget(union(obox, b.rect)))
		elif not state & (gtk.gdk.BUTTON1_MASK | gtk.gdk.BUTTON2_MASK | 
				gtk.gdk.BUTTON3_MASK | gtk.gdk.BUTTON4_MASK | 
				gtk.gdk.BUTTON5_MASK): # Hover
			boxes = tuple(self.find_boxes_coord_near(*icoords)) #FIXME: Use cache
			if len(boxes):
				#print "Nearby Boxes: %r" % (boxes,)
				box, dir = boxes[0]
				self._box_may_resize = box
				self._box_may_resize_dir = dir
				self.window.set_cursor(gtk.gdk.Cursor(self.window.get_display(), self.RESIZE_CURSORS[dir]))
			else:
				self._box_may_resize = self._box_may_resize_dir = None
				self.window.set_cursor(None)
	
	def do_button_press_event(self, event):
		# make sure it was the first button
		if event.button == 1:
			if self._box_may_resize is not None:
				print "Start resize"
				# FIXME: Calculate offset
				self._box_are_resizing = self._box_may_resize
				self._box_are_resizing_dir = self._box_may_resize_dir
			elif self.mode == self.INSERT:
				# Begin new box
				self._insert_start_coords = self.widget2imgcoords(event.x, event.y)
				self._temporary_box = Box(frect(*self._insert_start_coords+(0,0)), self.next_color.copy())
				self.emit('insert-box-changed', self._temporary_box)
			elif self.mode == self.SELECT:
				# Change selection
				# TODO: Rubber banding
				if self.selection is None: return True
				boxes = self.get_boxes_under_cursor(event.x, event.y)
				rows = []
			return True
	
	def do_button_release_event(self, event):
		# make sure it was the first button
		if event.button == 1:
			if self._box_are_resizing is not None:
				print "Stop resize"
				self._box_are_resizing = self._box_may_resize = None
				self._box_are_resizing_dir = self._box_may_resize_dir = None
			elif self.mode == self.INSERT:
				# End new box
				nb = self._temporary_box
				self._insert_start_coords = self._temporary_box = None
				if nb.rect.width == 0 or nb.rect.height == 0: 
					return
				redraw = self.rect2widget(nb.rect)
				redraw.x -= 1
				redraw.y -= 1
				redraw.width += 2
				redraw.height += 2
				self.queue_draw_area(*redraw)
				self.emit('box-added', nb)
				self._changed_rect = None
			elif self.mode == self.SELECT:
				# Change selection
				# TODO: Rubber banding
				if self.selection is None: return True
				rows = []
				boxes = self.get_boxes_under_cursor(event.x, event.y)
				def check(model, path, iter, ua):
					boxes, rows = ua
					rbox, = model.get(iter, int(self.box_col))
					if rbox in boxes:
						rows.append(iter)
				if event.state & gtk.gdk.SHIFT_MASK:
					self.model.foreach(check, (boxes, rows))
					assert len(boxes) == len(rows)
				else:
					self.model.foreach(check, (boxes[0:1], rows))
				print "Boxes: %r" % (boxes,)
				print "Rows: %r" % rows
				selection = self.selection
				if event.state & gtk.gdk.CONTROL_MASK:
					for r in rows: 
						if selection.iter_is_selected(r):
							selection.unselect_iter(r)
						else:
							selection.select_iter(r)
				else:
					selection.unselect_all()
					for r in rows: 
						selection.select_iter(r)
				
	
	def do_box_added(self, box):
		print "box-added", self, box
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
	bls.append(['', Box(gtk.gdk.Rectangle(50,100,200,300), gtk.gdk.color_parse('#AF0'))])
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

