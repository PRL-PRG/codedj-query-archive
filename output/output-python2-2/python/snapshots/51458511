# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
doc string
"""
from __future__ import division, absolute_import, with_statement
import gtk, os, sys
__all__ = 'Actor', 'action'

_iconfactory = None

#TODO: Use gtk.Action
def action(label=None, stock=None, tooltip=None, group=None, accel=None, shortlabel=None, **kw):
	"""action(...) -> callable(method) -> method
	Registers the method as an action.
	"""
	global _iconfactory
	if 'theme' in kw or 'image' in kw:
		# Add this to the stock list.
		
		# Some argument parsing
		assert stock is not None
		keymod,keyval = 0,0
		if accel:
			keymod,keyval = gtk.accelerator_parse(accel)
		
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
		print iconset
		_iconfactory.add(stock, iconset)
		
		# Add the stock
		gtk.stock_add([(stock, label, keymod, keyval, '')])
		
		# Clean out the args so that it pulls them from stock
		label = None
		accel = None
	
	def _(method):
		method.act_name = method.__name__
		method.act_label = label
		method.act_stock = stock
		method.act_tooltip = tooltip
		method.act_group = group
		method.act_accel = accel
		method.act_shortlabel = shortlabel
		return method
	return _

class Actor(object):
	def __new__(cls, *p, **kw):
		print 'Actor.__new__()'
		self = super(Actor, cls).__new__(cls, *p, **kw)
		self.actiongroups = {}
		self.actions = {}
		for a in dir(self):
			if not hasattr(self, a): continue
			v = getattr(self,a)
			func = v
			if hasattr(v, 'im_func'):
				v = v.im_func
			if hasattr(v, 'act_stock'):
				if v.act_group is None:
					v.act_group = cls.__name__
				act = gtk.Action(v.act_name, v.act_label, v.act_tooltip, v.act_stock)
				if v.act_shortlabel:
					act.short_label = v.act_shortlabel
				
				group = self.actiongroups.get(v.act_group, gtk.ActionGroup(v.act_group))
				group.add_action_with_accel(act, v.act_accel)
				
				act.connect('activate', lambda action: func())
				
				v.action = act
				self.actions[act.get_name()] = act
		return self

