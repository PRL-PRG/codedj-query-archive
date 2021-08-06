import os
from gettext import gettext as _

import gtk

from sugar.activity import activity

#from Oficina import Oficina
from toolbox import Toolbox
from Area import Area
from Cursors import Cursors

# DRAW_WIDTH  = 1195
# DRAW_HEIGHT = 800

class OficinaActivity(activity.Activity):
    def __init__(self, handle):
        """Initialize the OficinaActivity object.

            Keyword arguments:
            self -- 
            handle --

        """
        activity.Activity.__init__(self, handle)

        os.chdir(activity.get_bundle_path())
        #print activity.get_bundle_path()
        
        toolbox = Toolbox(self)
        self.set_toolbox(toolbox)
        toolbox.show()       
  

        # addind a textview widget
        sw = gtk.ScrolledWindow()
        sw.set_policy(gtk.POLICY_AUTOMATIC, gtk.POLICY_AUTOMATIC)

        self._fixed = gtk.Fixed()	
        self._area = Area(self)	
        color = gtk.gdk.color_parse("white")
        self._fixed.modify_bg(gtk.STATE_NORMAL, color)

	self.bg = gtk.Image()
	self.bg.set_from_file('./icons/bg.svg')
	self._fixed.put(self.bg, 200, 100)
        self.bg.show()

        #FIXME: use a textview instead of a Entry
        #self._textview = gtk.TextView()
        self._textview = gtk.Entry()
        self._area.tool = 2
        self._fixed.put(self._area, 200 , 100)

        sw.add_with_viewport(self._fixed)
        self._area.show()
        self._fixed.show()


        self._fixed.put(self._textview, 0, 0)
        self._textview.hide()
        sw.show()

        # setting scrolledwindow as activity canvas...
        self.set_canvas(sw)


    def read_file(self, file_path):
        '''Read file from Sugar Journal.

        self --
		file_path --

        '''
        print 'read file...'
        print file_path
        #self._area.d.limpatudo()
        #self._area.d.clear()
        self._area.d.loadImage(file_path)


    def write_file(self, file_path):
        '''Save file on Sugar Journal.

		self --
		file_path -- 

        '''
        print file_path
        width, height = self._area.window.get_size()
        pixbuf = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, width, height)
        pixbuf.get_from_drawable(self._area.pixmap, gtk.gdk.colormap_get_system(), 0, 0, 0, 0, -1, -1)
        pixbuf.save(file_path, 'png', {})	

