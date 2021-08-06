# -*- coding: utf-8 -*-
import  pygtk
pygtk.require('2.0')
import gtk
from gtk import gdk

class Cursores:
	def __init__(self, archive):
		"""Initialize Cursores object.

		Keyword arguments:
		self -- Cursores.Cursores instance
		archive -- 

		"""		
		color = gtk.gdk.Color()
		pix = gtk.gdk.pixbuf_new_from_file("./images/" + archive)		
		self.cursor_ = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
	
	def cursor(self):
		"""Return self.cursor_.

		Keyword arguments:
		self -- Cursores.Cursores instance

		"""
		return self.cursor_
		
		
