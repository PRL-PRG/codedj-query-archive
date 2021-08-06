import pygtk
pygtk.require( '2.0' )
import gtk

class ImageButton(gtk.Button):
    
    def __init__(self , image_path, enter_image_path = None):
        gtk.Button.__init__(self)
        self.alloc = None
        
        if enter_image_path != None:
            self.connect('enter',self.on_btn_enter, None)
            self.connect('leave',self.on_btn_leave, None)
        
        self.image_path = image_path
        self.enter_image_path = enter_image_path
        
        self.image = gtk.Image()
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        
        self.connect('expose-event', self.expose)
        self.connect('size-allocate', self.size_allocate)
        
    def size_allocate(self, widget, allocation):
        self.alloc = allocation
        
    def expose(self, widget, event):
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width/2) - self.img_pixbuf.get_width() / 2 , self.alloc.y + (self.alloc.height/2) - (self.img_pixbuf.get_height() / 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
        
    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, wiget, event):
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
class ImageToggleButton(gtk.ToggleButton):
    
    def __init__(self , mainImg_path, altImg_path, enter_image_path = None):
        gtk.ToggleButton.__init__(self)
        self.alloc = None
        
        self.connect('clicked',self.on_btn_click, None)
        if enter_image_path != None:
            self.connect('enter',self.on_btn_enter, None)
            self.connect('leave',self.on_btn_leave, None)
        
        self.mainImg_path = mainImg_path
        self.altImg_path = altImg_path
        self.enter_image_path = enter_image_path
        
        self.image = gtk.Image()
        self.image.set_from_file(self.mainImg_path)
        self.img_pixbuf = self.image.get_pixbuf()
        
        self.connect('expose-event', self.expose)
        self.connect('size-allocate', self.size_allocate)
        
    def size_allocate(self, widget, allocation):
        self.alloc = allocation
        
    def switch(self):
        if not self.get_active():
            self.image.set_from_file(self.mainImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
            self.queue_draw()
        else:
            self.image.set_from_file(self.altImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
            self.queue_draw()
        
    def expose(self, widget, event):
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width/2) - self.img_pixbuf.get_width() / 2 , self.alloc.y + (self.alloc.height/2) - (self.img_pixbuf.get_height() / 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
    
    def on_btn_click(self, widget, event):
        self.switch()

    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, wiget, event):
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()

class ImageRadioButton(gtk.RadioButton):
    
    def __init__(self , group, mainImg_path, altImg_path, enter_image_path = None):
        gtk.RadioButton.__init__(self, group)
        self.alloc = None
        
        self.connect('clicked',self.on_btn_click, None)
        if enter_image_path != None:
            self.connect('enter',self.on_btn_enter, None)
            self.connect('leave',self.on_btn_leave, None)
    
        self.mainImg_path = mainImg_path
        self.altImg_path = altImg_path
        self.enter_image_path = enter_image_path
        
        self.image = gtk.Image()
        self.image.set_from_file(self.mainImg_path)
        self.img_pixbuf = self.image.get_pixbuf()
        
        self.connect('expose-event', self.expose)
        self.connect('size-allocate', self.size_allocate)
        self.switch()
        
    def size_allocate(self, widget, allocation):
        self.alloc = allocation
        
    def switch(self):
        if not self.get_active():
            self.image.set_from_file(self.mainImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
            self.queue_draw()
        else:
            self.image.set_from_file(self.altImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
            self.queue_draw()
        
    def expose(self, widget, event):
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width/2) - self.img_pixbuf.get_width() / 2 , self.alloc.y + (self.alloc.height/2) - (self.img_pixbuf.get_height() / 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
    
    def on_btn_click(self, widget, event):
        self.switch()
            
    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, wiget, event):
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
        