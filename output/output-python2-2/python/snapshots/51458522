#-*- coding: utf-8 -*-
"""
Defines a class which allows for easy use of Glade. Based on SubIM.
"""
## (c) 2007 James Bliss <http://www.astro73.com/>
## This code is freely usable and modifiable for any purpose with these 
## conditions:
##  1. This notice remains here
##  2. You send me a note that you're using it. I just like to know.
__all__ = 'GladeWindow', 'CustomWidget', 'resource'

# Stores the global typedict, updated by the decorator
_glade_typedict = {}

def CustomWidget(widget):
	"""CustomWidget(thing) -> thing
	Registers a class as a custom widget. Usable as a function decorator or a 
	metaclass.
	Example:
	  @CustomWidget
	  def SpamWidget(...):
	      ...
	
	If a string is given, it returns a callable, ie:
	  @CustomWidget("Spam")
	  def SpamWidget(...):
	      ...
	"""
	def _(w):
		global _glade_typedict
		_glade_typedict[name] = w
		return w
	if isinstance(widget, basestring):
		name = widget
		return _
	else:
		name = widget.__name__
		_(widget)
		return widget

class GladeWindow(object):
	"""
	Inherit from this in order to get some cool Glade automagick.
	
	Set the __roots__ class property to a list of names of the root widgets to 
	load. If unset, all widgets are loaded
	
	For all controls in the XML, a property matching their ID is assigned to it.
	ie, if you have a control with id="MyButton", then 
	MyApp("myui.glade").MyButton would be that button.
	
	Properties:
	 * _xml - The gtk.glade.XML instance from which our widgets were loaded.
	
	This class is not multiple-inheritance safe:
	 * super() is used
	 * __new__() is callable only with the arguments defined in this class
	 * super().__new__() is called with no arguments
	"""
	
	__roots__ = ()
	__slots__ = '_xml','__dict__','__weakref__'
	def __listWidgets(self, root):
		"""
		Grabs the names of all the widgets and sets a property to that name.
		"""
		import gtk.glade
		if not hasattr(root, 'get_children'): return
		for c in root.get_children():
			n = gtk.glade.get_widget_name(c)
			if n is None: 
				continue # Wasn't an XML child
			setattr(self, n, c)
			self.__listWidgets(c)
	
	def __new__(cls, fname, root="", domain="", typedict={}):
		"""GladeWindow(fname, root="", domain="", typedict={}) -> GladeWindow
		The arguments are passed directly to gtk.glade.XML. The documentation 
		(from <http://pygtk.org/docs/pygtk/class-gladexml.html#constructor-gladexml>):
		 * fname : the XML file name
		 * root : the widget node in fname to start building from (or "")
		 * domain : the translation domain for the XML file (or "" for default)
		 * typedict : A dictionary used to lookup types (or {} for default)
		
		Note that when possible, root is filled in automatically (ie, when 
		__roots__ is 1 item.)
		
		If you need to perform initialization, define an __init__() method. It
		will be called after the XML is loaded, events connected, etc.
		"""
		import gtk.glade
		
		self = super(GladeWindow,cls).__new__(cls)
		
		# Get the typedict
		td = _glade_typedict.copy()
		td.update(typedict)
		
		# Get the XML
		if len(cls.__roots__) == 1 and root == "":
			root = cls.__roots__[0]
		self._xml = gtk.glade.XML(fname,root,romain,td)
		
		# Connect events
		self._xml.signal_autoconnect(self)
		
		# Set widget properties
		roots = cls.__roots__
		if len(roots) > 0:
			for r in roots:
				w = self._xml.get_widget(r)
				setattr(self, r, w)
				self.__listWidgets(w) # Initialize all the props
		else:
			for w in self._xml.get_widget_prefix(''):
				n = gtk.glade.get_widget_name(w)
				if n is None: 
					continue # Wasn't an XML child (???)
				setattr(self, n, w)
				# Don't call __listWidgets() because get_widget_prefix() will
				# return everything
		
		# Do client init
		if hasattr(self, '__init__'):
			self.__init__(fname, root, domain, typedict)
		
		return self

class ResourceNotFoundWarning(Warning):
	"""
	Indicates that a resource could not be located.
	"""
	__slots__ = 'filename',
	def __init__(self, fn):
		super(ResourceNotFoundWarning, self).__init__()
		self.filename = fn
	def __unicode__(self):
		return u"Couldn't find resource %r" % fn
	def __str__(self):
		return str(unicode(self))

def resource(fn,sec="share", appname=None):
	"""resource(filename. section="share", appname=None) -> string
	Attempts to locate a file given a section in a cross-platform manner.
	
	It searches a list of common linux prefix paths, the same dir as the 
	started script (sys.argv[0]), etc.
	
	appname defaults to the script name.
	
	The list of searched directories is as follows:
	* <script dir>
	* <sys.prefix>/<section>/<appname>
	* /usr/local/<section>/<appname>
	* /usr/<section>/<appname>
	* /opt/<section>/<appname>
	* /<section>/<appname>
	* <script dir>/../<section>/<appname>
	* <script dir>/<section>/<appname>
	* ../<section>/<appname>
	
	When everything fails, just returns the passed-in file name (ie the current 
	directory) and raises a warning.
	
	Some special sections:
	* $var - Adds /var/lib to the mix (dollar sign stripped before using the 
	  normal list)
	* doc - Adss the whole list using share/doc as the section
	
	This function is geared towards files that are installed with the script, 
	not files that are created by the app (eg in /var/run, /var/log, /tmp)
	"""
	import sys, os
	script = sys.argv[0]
	if appname is None:
		appname = os.path.basename(script)
	def _resource_paths(fn,sec,appname):
		import sys, os
		if sec[0] == '$':
			sec=sec[1:]
		return [
			os.path.dirname(script), # For development and Win32
			os.path.join(sys.prefix, sec, appname), # Assuming a single, global prefix
			# And now for some common prefix's
			'/'+os.path.join('usr', 'local', sec, appname),
			'/'+os.path.join('usr', sec, appname),
			'/'+os.path.join('opt', sec, appname),
			'/'+os.path.join(sec, appname),
			# Relative to the script
			os.path.join(os.path.dirname(script), os.pardir, sec, appname),
			os.path.join(os.path.dirname(script), sec, appname),
			# Now we're getting desparate
			os.path.join(os.pardir, sec, appname),
			]
	paths = _resource_paths(fn,sec,appname)
	if sec == '$var':
		paths.append('/'+os.path.join('var', 'lib', appname))
	elif sec == 'doc':
		paths += _resource_paths(fn, 'share/doc', appname)
	for path in paths:
		try:
			f = os.path.join(path, fn)
			if os.path.exists(f):
				return f
		except: pass
	else:
		import warnings
		warnings.warn(ResourceNotFoundWarning(fn))
		return fn #Can't actually find it

