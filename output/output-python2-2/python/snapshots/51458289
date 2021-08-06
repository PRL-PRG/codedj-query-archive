# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Some utilities for dealing with rectangles
"""
from __future__ import division, absolute_import, with_statement
import gtk
__all__ = ('rect_contains', 'slice_diff', 'slice_len', 'slice_contains', 
	'slice_diff_max', 'rect_diff', 'pt2rect', 'frect')

def frect(x,y,w,h):
	_ = lambda n: int(round(n))
	return gtk.gdk.Rectangle(_(x), _(y), _(w), _(h))

def rect_contains(rect, x,y):
	"""rect_contains(gtk.gdk.Rectangle, number, number) -> bool
	Determines if the given coordinates are contained in the rectangle 
	inclusively. That is, if the width is 5, then there are 5 valid integers 
	for x.
	"""
	return (rect.x <= x < rect.x+rect.width) and \
	       (rect.y <= y < rect.y+rect.height)

def slice_diff(s1, s2):
	"""slice_diff(slice, slice) -> slice|None, slice|None
	Returns the slice(s) of the numbers contained in s1 but not in s2.
	
	Two slices are necessary because s2 may be entirely contained in s1.
	
	Return values:
	 * Slice, None: There's one slice. It contains the portion of s1 not in s2
	 * Slice, Slice: s2 subset of s1, 2 slices. Each one contains a portion of 
	 	s1 not in s2
	* None, None: s1 subset of s2, null set.
	
	Note: Both slices need to have starts and stops. Steps are ignored.
	"""
	if s2.stop <= s1.start or s1.stop <= s2.start:
		# non-intersecting
		return s1, None
	elif s2.start <= s1.start and s1.stop <= s2.stop:
		# s1 entirely within s2, no results
		return None, None
	elif s1.start <= s2.start and s2.stop <= s1.stop:
		# s2 entirely within s1, two slices
		return slice(s1.start, s2.start), slice(s2.stop, s1.stop)
	elif s1.start <= s2.start and s1.stop <= s2.stop:
		# Partial intersection: s1 is lower
		return slice(s1.start, s2.start), None
	else: 
		assert s2.start <= s1.start and s2.stop <= s1.stop
		return slice(s2.stop, s1.stop), None

slice_len = lambda s: s.stop - s.start
slice_contains = lambda s, n: s.start <= n < s.stop

def slice_diff_max(s1, s2, p=None):
	d, d2 = slice_diff(s1, s2)
	
#	print 'slice_diff_max: %r - %r (%r)= %r, %r' % (s1, s2, p, d, d2),
	
	if d is None: # s1 subset of s2
#		print None
		return None
	
	# If there's 2, choose one
	if d2 is not None:
		if p is None:
			if slice_len(d2) > slice_len(d): 
				d = d2
		else:
			if not slice_contains(d, p):
				d = d2
#	print '#2' if d == d2 else '#1'
	return d


def rect_diff(rect1, rect2, preferred=None):
	"""rect_diff(gtk.gdk.Rectangle, gtk.gdk.Rectangle, [(x,y)]) -> gtk.gdk.Rectangle
	Returns the largest gtk.gdk.Rectangle that is entirely within rect1 but has 
	nothing in rect2.
	
	If rect2 is entirely within rect1, returns a rectangle. If preferred is 
	set, it influences the selection of the rectangle. Tries to find the 
	largest rectangle (or at least a larger rectangle).
	
	Note: Currently uses a per-axis computation. Meaning that it performs this 
	algorithm on each axis and then merges results. The upshot is that it 
	doesn't find the absolutely largest rectangle, just one of the possible 
	ones.
	"""
	px,py = None,None
	if preferred is not None:
		px,py = preferred
	# The X-Axis
	r1_x = slice(rect1.x, rect1.x+rect1.width)
	r2_x = slice(rect2.x, rect2.x+rect2.width)
#	print 'X:',
	dx = slice_diff_max(r1_x, r2_x, px)
	if px is not None and dx is not None:
		if not slice_contains(dx, px):
			dx = r1_x
	
	# The Y-Axis
	r1_y = slice(rect1.y, rect1.y+rect1.height)
	r2_y = slice(rect2.y, rect2.y+rect2.height)
#	print 'Y:',
	dy = slice_diff_max(r1_y, r2_y, py)
	if py is not None and dy is not None:
		if not slice_contains(dy, py):
			dy = r1_y
	
	# Take the results (in dx and dy) and make a new rectangle
	if dx is None and dy is None:
		# rect1 subset of rect2
		rv = None
	elif dx is None:
		rv = frect(rect1.x, dy.start, rect1.width, dy.stop - dy.start)
	elif dy is None:
		rv = frect(dx.start, rect1.y, dx.stop - dx.start, rect1.height)
	else:
		rv = frect(dx.start, dy.start, dx.stop - dx.start, dy.stop - dy.start)
#	print 'rect_diff: %r - %r (%r,%r)= %r' % (tuple(rect1), tuple(rect2), px,py, 'None' if rv is None else tuple(rv))
	return rv

def pt2rect(*pargs):
	"""
	Takes a series of 2-tuples and creates a rect from them.
	"""
	# zip() utterly fails
	x = [c[0] for c in pargs]
	y = [c[1] for c in pargs]
	sx = min(*x)
	sy = min(*y)
	ex = max(*x)
	ey = max(*y)
	return frect(sx, sy, ex-sx, ey-sy)
