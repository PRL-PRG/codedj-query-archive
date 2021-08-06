import  pygtk
pygtk.require('2.0')
import gtk
from gtk import gdk

class Cursors:
	def __init__(self, archive):
		"""Initialize Cursors object.

		Keyword arguments:
		self -- Cursors.Cursors instance
		archive -- 

		"""			
		color = gtk.gdk.Color()
		pix = gtk.gdk.pixbuf_new_from_file("./images/" + archive)		
		self._cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
	
	def cursor(self):
		"""Return self._cursor.

		Keyword arguments:
		self -- Cursors.Cursors instance

		"""
		return self._cursor
		
		
