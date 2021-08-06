# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
doc string
"""
from __future__ import division, absolute_import, with_statement
import gtk, os, sys
from functools import wraps
__all__ = 'Actor', 'action'

_iconfactory = None

#TODO: Use gtk.Action
def action(label=None, stock=None, tooltip=None, group=None, accel=None, 
		shortlabel=None, radiogroup=None, toggle=None, active=None, 
		enabled=None, **kw):
	"""action(...) -> callable(method) -> method
	Flags the method as an action. Decorator.
	
	>>> @action(stock='gtk-add')
	... def foo(self):
	... 	pass
	"""
	global _iconfactory
	if 'theme' in kw or 'image' in kw:
		# Add this to the stock list.
		
		# Some argument parsing
		assert stock is not None
		keymod,keyval = 0,0
		if accel:
			keymod,keyval = gtk.accelerator_parse(accel)
		print "Accel parse: %r %r %r" % (accel, keymod, keyval)
		
		if 'theme' in kw:
			# Lookup the image from the icon sets
			icon = kw['theme']
			iconset = gtk.icon_factory_lookup_default(icon)
			if iconset is None:
				# We have to load the icon from the theme and convert it into factory work
				theme = gtk.icon_theme_get_default()
				sizes = theme.get_icon_sizes(icon)
				if len(sizes) != 0:
					iconset = gtk.IconSet()
					src = gtk.IconSource()
					src.set_icon_name(icon)
					iconset.add_source(src)

		elif 'image' in kw:
			img = kw['image']
			if img is None:
				iconset = NotImplemented
			else:
				if isinstance(img, basestring):
					# File name
					img = gtk.gdk.pixbuf_new_from_file(img)
				iconset = gtk.IconSet(img)
		else:
			assert False, "Logic shouldn't get here"
		
		# Create the icon factory, if needed
		if _iconfactory is None:
			_iconfactory = gtk.IconFactory()
			_iconfactory.add_default()
		
		# Add the icon to the factory
		if iconset is None:
			from warnings import warn
			warn("iconset wasn't set.")
		elif iconset is NotImplemented:
			pass # No Image
		else:
			_iconfactory.add(stock, iconset)
		
		# Add the stock
		gtk.stock_add([(stock, label, keymod, keyval, '')])
		
		# Clean out the args so that it pulls them from stock
		label = None
		#accel = None
	
	def _(method):
		method.act_name = method.__name__
		method.act_label = label
		method.act_stock = stock
		method.act_tooltip = tooltip
		method.act_group = group
		method.act_accel = accel
		method.act_shortlabel = shortlabel
		method.act_radiogroup = radiogroup
		method.act_toggle = toggle
		method.act_active = active
		method.act_enabled = enabled
		return method
	return _

class ActionHandler(object):
	__slots__ = '_method',
	def __init__(self, m):
		self._method = m
	def __call__(self, action):
		self._method()

class Actor(object):
	_mergeid = None
	def __new__(cls, *p, **kw):
		print 'Actor.__new__()'
		self = super(Actor, cls).__new__(cls, *p, **kw)
		self.actiongroups = {}
		self.actions = {}
		radiogroups = {}
		for a in dir(self):
			if not hasattr(self, a): continue
			v = getattr(self,a)
			func = v
			if hasattr(v, 'im_func'):
				v = v.im_func
			if hasattr(v, 'act_stock'):
				if v.act_group is None:
					v.act_group = cls.__name__
				
				ga_cls = gtk.Action
				extraparams = ()
				if v.act_radiogroup:
					ga_cls = gtk.RadioAction
					grp = radiogroups.setdefault(v.act_radiogroup, [])
					extraparams = len(grp),
				elif v.act_toggle:
					ga_cls = gtk.ToggleAction
				
				act = ga_cls(v.act_name, v.act_label, v.act_tooltip, v.act_stock, *extraparams)
				
				if v.act_shortlabel:
					act.short_label = v.act_shortlabel
				if v.act_radiogroup:
					grp = radiogroups[v.act_radiogroup]
					if len(grp) != 0:
						act.set_group(grp[0])
					grp.append(act)
				if v.act_active is not None:
					act.set_active(v.act_active)
				if v.act_enabled is not None:
					act.set_sensitive(v.act_enabled)
				group = self.actiongroups.setdefault(v.act_group, gtk.ActionGroup(v.act_group))
				group.add_action_with_accel(act, v.act_accel)
				
				act.connect('activate', ActionHandler(func))
				
				v.action = act
				#func.action = act # So that methods have it as well, except methods don't allow user-defined properties
				self.actions[act.get_name()] = act
		self.manager = gtk.UIManager()
		self.manager.connect('add-widget', self.add_widget)
		if __debug__: 
			self.manager.get_accel_group().connect('accel-changed', lambda grp, key, mods, closure: sys.stdout.write("Accel changed: %r %r (%r) %r %r\n" % (grp, key, chr(key), mods, closure)))
		for grp in self.actiongroups.itervalues():
			self.manager.insert_action_group(grp, -1)
		if hasattr(self, '__ui__'):
			self._mergeid = self.manager.add_ui_from_string(self.__ui__)
#		if hasattr(self, 'window'):
#			self.window.connect('show', self.preshow)
		return self
	
	def preshow(self, *_):
		"""
		Call this before showing the window.
		"""
		self.manager.ensure_update()
		self.window.add_accel_group(self.manager.get_accel_group())
		#return True
	
	def add_menubar(self, menubar):
		raise NotImplementedError
	
	def add_toolbar(self, toolbar):
		raise NotImplementedError
	
	def add_widget(self, manager, widget):
		if isinstance(widget, gtk.MenuBar):
			self.add_menubar(widget)
		elif isinstance(widget, gtk.Toolbar):
			self.add_toolbar(widget)

