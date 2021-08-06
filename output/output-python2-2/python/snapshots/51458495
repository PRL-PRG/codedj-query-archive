# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Some more generic TreeModel mixins.
"""
from __future__ import division, absolute_import, with_statement
__all__ = 'GenericTreeStore', 'GenericMovableModel'

class GenericTreeStore:
	# def on_set_value(self, rowref, column, value)
	# def on_remove(self, rowref)
	# def on_clear(self)
	# def on_prepend(self, row=None) -> user data
	# def on_append(self, row=None) -> user data
	# def on_insert(self, position, row=None) -> user data
	# def on_insert_before(self, sibling, row=None) -> user data
	# def on_insert_after(self, sibling, row=None) -> user data
	
	def set_value(self, iter, column, value):
		rowref = self.get_user_data(iter)
		self.on_set_value(rowref, column, value)
		path = () if iter is None else self.on_get_path(ref)
		self.row_changed(path, iter)
	
	def set(self, iter, *pargs):
		ref = self.get_user_data(iter)
		for col, data in (pargs[i:i+2] for i in xrange(0, len(pargs), 2)):
			self.on_set_value(ref, col, data)
		path = () if iter is None else self.on_get_path(ref)
		self.row_changed(path, iter)
	
	def remove(self, iter):
		rowref = self.get_user_data(iter)
		path = () if iter is None else self.on_get_path(rowref)
		rv = self.on_remove(rowref)
		self.row_deleted(path)
		return rv
	
	def on_clear(self): # A default
		self.foreach(lambda self, path, iter, ud: self.on_remove(self.get_user_data(iter)), None)
	
	def clear(self):
		paths = []
		self.foreach(lambda self, path, iter, ud: path.append(path), None) # It's horribly, horribly inefficient, but that's how it is
		self.on_clear()
		for path in paths: self.row_deleted(path)
	
	def prepend(self, row=None):
		ref = self.on_prepend(row)
		path = self.on_get_path(ref)
		iter = self.create_tree_iter(ref)
		self.row_inserted(path, iter)
		return iter
	
	def append(self, row=None):
		ref = self.on_append(row)
		path = self.on_get_path(ref)
		iter = self.create_tree_iter(ref)
		self.row_inserted(path, iter)
		return iter
	
	def insert(self, position, row=None):
		ref = self.on_insert(position, row)
		path = self.on_get_path(ref)
		iter = self.create_tree_iter(ref)
		self.row_inserted(path, iter)
		return iter
	
	def insert_before(self, sibling, row=None):
		if sibling is None:
			rv = self.on_append(row)
		else:
			rowref = self.get_user_data(sibling)
			rv = self.on_insert_before(rowref, row)
		path = self.on_get_path(rv)
		iter = self.create_tree_iter(rv)
		self.row_inserted(path, iter)
		return iter
	
	def insert_after(self, sibling, row=None):
		if sibling is None:
			rv = self.on_prepend(row)
		else:
			rowref = self.get_user_data(sibling)
			rv = self.on_insert_after(rowref, row)
		path = self.on_get_path(rv)
		iter = self.create_tree_iter(rv)
		self.row_inserted(path, iter)
		return iter
	
class GenericMovableModel:
	# This requires a list
	
	# def on_reorder(self, new_order) -> None
	def reorder(self, new_order):
		self.on_reorder(new_order)
		
		# Is there a better way to do this?
		inversed = [None]*len(new_order)
		for i,v in enumerate(new_order):
			inversed[v] = i
			
		self.rows_reordered((), None, inversed)
	
	# def on_swap(self, a, b) -> None
	def swap(self, a, b):
		a,b = map(self.get_user_data, (a,b))
		pa,pb = map(self.on_get_path, (a,b))
		pa = pa[0]
		pb = pb[0]
		self.on_swap(a,b)
		l = self.on_get_n_children(None)
		new_order = range(l)
		new_order[pa], new_order[pb] = pb, pa
		self.rows_reordered((), None, new_order)
	
	# def on_move_after(self, rowref, position) -> None
	def move_after(self, iter, position):
		rowref = self.get_user_data(iter)
		oldpos = self.on_get_path(rowref)[0]
		pos = None
		newpos = 0
		if position is not None:
			pos = self.get_user_data(position)
			newpos = self.on_get_path(pos)
		
		self.on_move_after(rowref, pos)
					
		l = self.on_get_n_children(None)
		new_order = range(l)
		new_order[oldpos] = newpos
		self.rows_reordered((), None, new_order)
	
	# def on_move_before(self, rowref, position) -> None
	def move_before(self, iter, position):
		rowref = self.get_user_data(iter)
		oldpos = self.on_get_path(rowref)[0]
		l = self.on_get_n_children(None)
		pos = None
		newpos = l-1
		if position is not None:
			pos = self.get_user_data(position)
			newpos = self.on_get_path(pos)
		
		self.on_move_before(rowref, pos)
					
		new_order = range(l)
		new_order[oldpos] = newpos
		self.rows_reordered((), None, new_order)

