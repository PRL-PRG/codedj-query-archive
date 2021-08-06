import pygtk
pygtk.require( '2.0' )
import gtk 

class ImageHScale( gtk.HScale ):
    def __init__( self, image_name, adjustment = None, slider_border = 0, insensitive_name = None, trough_color = "#333", snap = False ):
        gtk.HScale.__init__( self, adjustment )
        
        if snap: self.snap = 1/snap
        else: self.snap = False
        
        colormap = self.get_colormap()
        self.troughcolor = colormap.alloc_color( trough_color, True, True )
        
        img = gtk.Image()
        img.set_from_file( image_name )
        self.sliderPixbuf = img.get_pixbuf()

        if insensitive_name == None:
            self.insensitivePixbuf = None
        else:
            img = gtk.Image()
            img.set_from_file( insensitive_name )
            self.insensitivePixbuf = img.get_pixbuf()
        
        name = image_name + "ImageHScale"
        self.set_name(name)        
        
        rc_str = """
style "scale_style" {
    GtkRange::slider_width = %d
    GtkScale::slider_length = %d
}
widget "*%s*" style "scale_style" 
        """ % ( self.sliderPixbuf.get_width(), self.sliderPixbuf.get_height(), name)
        gtk.rc_parse_string( rc_str )
        
        self.pixbufWidth = self.sliderPixbuf.get_width()
        self.pixbufHeight = self.sliderPixbuf.get_height()
        self.sliderBorder = slider_border
        self.sliderBorderMUL2 = self.sliderBorder*2
        
        self.set_draw_value(False)
        
        self.connect( "expose-event", self.expose )
        self.connect( "size-allocate", self.size_allocate )
        self.connect( "button-release-event", self.button_release )
        
    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
        self.sliderY = self.alloc.height//2 - self.pixbufHeight//2
        return False

    def set_snap( self, snap ):
	if snap: self.snap = 1/snap
	else: self.snap = False
	self.expose(self, event=None)
        
    def expose( self, widget, event ):
        
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        
        gc.foreground = self.troughcolor
                
        self.window.draw_rectangle( gc, True, self.alloc.x + self.sliderBorder, self.alloc.y + self.alloc.height//2 - 1, self.alloc.width - self.sliderBorderMUL2, 3 )
        
        val = self.get_value()
        if self.snap:
            val = round(self.snap*val)/self.snap
        adj = self.get_adjustment()
        if self.get_inverted(): 
            sliderX = int((self.alloc.width - self.pixbufWidth)*(adj.upper-val)/(adj.upper - adj.lower))
        else:
            sliderX = int((self.alloc.width - self.pixbufWidth)*(val-adj.lower)/(adj.upper - adj.lower))
        
        if self.insensitivePixbuf != None and self.state == gtk.STATE_INSENSITIVE:
            self.window.draw_pixbuf( gc, self.insensitivePixbuf, 0, 0, self.alloc.x + sliderX, self.alloc.y + self.sliderY, self.pixbufWidth, self.pixbufHeight, gtk.gdk.RGB_DITHER_NORMAL, 0, 0 )
        else:
            self.window.draw_pixbuf( gc, self.sliderPixbuf, 0, 0, self.alloc.x + sliderX, self.alloc.y + self.sliderY, self.pixbufWidth, self.pixbufHeight, gtk.gdk.RGB_DITHER_NORMAL, 0, 0 )
        
        return True
        
    def button_release( self, widget, event ):
        
        if self.snap:
            self.set_value( round(self.snap*self.get_value())/self.snap )

class ImageVScale( gtk.VScale ):
    def __init__( self, image_name, adjustment = None, slider_border = 0, insensitive_name = None, trough_color = "#333", snap = False ):
        gtk.VScale.__init__( self, adjustment )
        
        if snap: self.snap = 1/snap
        else: self.snap = False
        
        colormap = self.get_colormap()
        self.troughcolor = colormap.alloc_color( trough_color, True, True )
        
        img = gtk.Image()
        img.set_from_file( image_name )
        self.sliderPixbuf = img.get_pixbuf()
        
        if insensitive_name == None:
            self.insensitivePixbuf = None
        else:
            img = gtk.Image()
            img.set_from_file( insensitive_name )
            self.insensitivePixbuf = img.get_pixbuf()
        
        name = image_name + "ImageVScale"
        self.set_name(name)        
        
        rc_str = """
style "scale_style" {
    GtkRange::slider_width = %d
    GtkScale::slider_length = %d
}
widget "*%s*" style "scale_style" 
        """ % ( self.sliderPixbuf.get_width(), self.sliderPixbuf.get_height(), name)
        gtk.rc_parse_string( rc_str )
        
        self.pixbufWidth = self.sliderPixbuf.get_width()
        self.pixbufHeight = self.sliderPixbuf.get_height()
        self.sliderBorder = slider_border
        self.sliderBorderMUL2 = self.sliderBorder*2
        
        self.set_draw_value(False)
        
        self.connect( "expose-event", self.expose )
        self.connect( "size-allocate", self.size_allocate )
        self.connect( "button-release-event", self.button_release )
    
    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
        self.sliderX = self.alloc.width//2 - self.pixbufWidth//2
        return False

    def set_snap( self, snap ):
	if snap: self.snap = 1/snap
	else: self.snap = False
	self.expose(self, event=None)
    
    def expose( self, widget, event ):
        
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        
        gc.foreground = self.troughcolor
                
        self.window.draw_rectangle( gc, True, self.alloc.x + self.alloc.width//2 - 1, self.alloc.y + self.sliderBorder, 3, self.alloc.height - self.sliderBorderMUL2 )
        
        val = self.get_value()
        if self.snap:
            val = round(self.snap*val)/self.snap
        adj = self.get_adjustment()
        if self.get_inverted(): 
            sliderY = int((self.alloc.height - self.pixbufHeight)*(adj.upper-val)/(adj.upper - adj.lower))
        else:
            sliderY = int((self.alloc.height - self.pixbufHeight)*(val-adj.lower)/(adj.upper - adj.lower))
        
        if self.insensitivePixbuf != None and self.state == gtk.STATE_INSENSITIVE:
            self.window.draw_pixbuf( gc, self.insensitivePixbuf, 0, 0, self.alloc.x + self.sliderX, self.alloc.y + sliderY, self.pixbufWidth, self.pixbufHeight, gtk.gdk.RGB_DITHER_NORMAL, 0, 0 )
        else:
            self.window.draw_pixbuf( gc, self.sliderPixbuf, 0, 0, self.alloc.x + self.sliderX, self.alloc.y + sliderY, self.pixbufWidth, self.pixbufHeight, gtk.gdk.RGB_DITHER_NORMAL, 0, 0 )
        
        return True
        
    def button_release( self, widget, event ):
        
        if self.snap:
            self.set_value( round(self.snap*self.get_value())/self.snap )

class RoundHBox( gtk.HBox ):
    def __init__( self, radius = 5, fillcolor = "#000", bordercolor = "#FFF", homogeneous = False, spacing = 0 ):
        gtk.HBox.__init__( self, homogeneous, spacing )
        self.alloc = None
        
        self.radius = radius
        
        colormap = self.get_colormap()
        self.fillcolor = colormap.alloc_color(fillcolor,True,True)
        self.bordercolor = colormap.alloc_color(bordercolor,True,True)
        
        self.connect( "expose-event", self.expose )
        self.connect( "size-allocate", self.size_allocate )
        
    def update_constants( self ):
        
        if self.alloc == None: return
    
        self.borderW = self.get_border_width()
        self.borderWMUL2 = self.borderW*2
        self.corner = self.radius + self.borderW
        self.cornerMUL2 = self.corner*2
        self.cornerMINborderW = self.corner - self.borderW
                
        self.xPLUborderW = self.alloc.x + self.borderW
        self.xPLUcorner = self.alloc.x + self.corner
        self.xPLUwidthMINborderW = self.alloc.x + self.alloc.width - self.borderW
        self.xPLUwidthMINcorner = self.alloc.x + self.alloc.width - self.corner
        self.yPLUborderW = self.alloc.y + self.borderW
        self.yPLUcorner = self.alloc.y + self.corner
        self.yPLUheightMINborderW = self.alloc.y + self.alloc.height - self.borderW
        self.yPLUheightMINcorner = self.alloc.y + self.alloc.height - self.corner
        self.widthMINborderW = self.alloc.width - self.borderW
        self.widthMINcorner = self.alloc.width - self.corner
        self.widthMINcornerMUL2 = self.alloc.width - self.cornerMUL2
        self.heightMINborderW = self.alloc.height - self.borderW
        self.heightMINcorner = self.alloc.height - self.corner
        self.heightMINborderWMUL2 = self.alloc.height - self.borderWMUL2
        self.heightMINcornerMUL2 = self.alloc.height - self.cornerMUL2
        
        self.roundX1 = self.alloc.x + self.borderW - 1
        self.roundX2 = self.alloc.x + self.alloc.width - self.corner - self.radius - 1
        self.roundY1 = self.alloc.y + self.borderW - 1
        self.roundY2 = self.alloc.y + self.alloc.height - self.corner - self.radius - 1
        self.roundD = self.radius*2 + 1
        self.rightAngle = 90*64
        
    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
        self.update_constants()
        return False
        
    def set_border_width( self, width ):
        gtk.HBox.set_border_width( self, width )
        self.update_constants()
        
    def set_radius( self, radius ):
        self.radius = radius
        self.update_constants()
        
    def set_fill_color( self, color ):
        colormap = self.get_colormap()
        self.fillcolor = colormap.alloc_color(color,True,True)
        
    def set_border_color( self, color ):
        colormap = self.get_colormap()
        self.bordercolor = colormap.alloc_color(color,True,True)
    
    def expose( self, widget, event ):
        
        if self.alloc == None: return
        
        #TP.ProfileBegin( "Round*Box::expose" )
        
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        
        startX = event.area.x - self.alloc.x
        startY = event.area.y - self.alloc.y
        stopX = startX + event.area.width
        stopY = startY + event.area.height
        
        saveForeground = gc.foreground
        
        # Note: could maybe do some optimization to fill only areas that are within the dirty rect, but drawing
        # seems to be quite fast compared to python code, so just leave it at clipping by each geometry feature
        
        gc.foreground = self.bordercolor
        if self.borderW:
            if stopY > self.corner and startY < self.heightMINcorner:
                if startX < self.borderW:         # draw left border 
                    self.window.draw_rectangle( gc, True, self.alloc.x, self.yPLUcorner, self.borderW, self.heightMINcornerMUL2 )
                if stopX > self.widthMINborderW:  # draw right border 
                    self.window.draw_rectangle( gc, True, self.xPLUwidthMINborderW, self.yPLUcorner, self.borderW, self.heightMINcornerMUL2 )
            
            if stopX > self.corner and startX < self.widthMINcorner:
                if startY < self.borderW:         # draw top border
                    self.window.draw_rectangle( gc, True, self.xPLUcorner, self.alloc.y, self.widthMINcornerMUL2, self.borderW )
                if stopY > self.heightMINborderW: # draw bottom border
                    self.window.draw_rectangle( gc, True, self.xPLUcorner, self.yPLUheightMINborderW, self.widthMINcornerMUL2, self.borderW )
        
        if startX < self.corner:
            if startY < self.corner:              # draw top left corner
                self.window.draw_rectangle( gc, True, self.alloc.x, self.alloc.y, self.corner, self.corner )
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX1, self.roundY1, self.roundD, self.roundD, self.rightAngle, self.rightAngle )
                gc.foreground = self.bordercolor
            if stopY > self.heightMINcorner:      # draw bottom left corner
                self.window.draw_rectangle( gc, True, self.alloc.x, self.yPLUheightMINcorner, self.corner, self.corner )
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX1, self.roundY2, self.roundD, self.roundD, -self.rightAngle, -self.rightAngle )
                gc.foreground = self.bordercolor
        if stopX > self.widthMINcorner:
            if startY < self.corner:              # draw top right corner
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.alloc.y, self.corner, self.corner )                
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX2, self.roundY1, self.roundD, self.roundD, 0, self.rightAngle )
                gc.foreground = self.bordercolor
            if stopY > self.heightMINcorner:      # draw bottom right corner
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.yPLUheightMINcorner, self.corner, self.corner )                                
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX2, self.roundY2, self.roundD, self.roundD, 0, -self.rightAngle )
                gc.foreground = self.bordercolor
        
        gc.foreground = self.fillcolor
        if startX < self.widthMINcorner and stopX > self.corner:
            if startY < self.heightMINborderW and stopY > self.borderW: # draw centre fill
                self.window.draw_rectangle( gc, True, self.xPLUcorner, self.yPLUborderW, self.widthMINcornerMUL2, self.heightMINborderWMUL2 )
        if startX < self.corner and stopX > self.borderW:
            if startY < self.heightMINcorner and stopY > self.corner:   # draw left fill
                self.window.draw_rectangle( gc, True, self.xPLUborderW, self.yPLUcorner, self.cornerMINborderW, self.heightMINcornerMUL2 )
        if startX < self.widthMINborderW and stopX > self.widthMINcorner:
            if startY < self.heightMINcorner and stopY > self.corner:   # draw right fill
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.yPLUcorner, self.cornerMINborderW, self.heightMINcornerMUL2 )
                
        gc.foreground = saveForeground
        
        #TP.ProfileEnd( "Round*Box::expose" )
        
        return False
        
        
class RoundVBox( gtk.VBox ):
    def __init__( self, radius = 5, fillcolor = "#000", bordercolor = "#FFF", homogeneous = False, spacing = 0 ):
        gtk.VBox.__init__( self, homogeneous, spacing )
        self.alloc = None
        
        self.radius = radius
        
        colormap = self.get_colormap()
        self.fillcolor = colormap.alloc_color(fillcolor,True,True)
        self.bordercolor = colormap.alloc_color(bordercolor,True,True)
        
        self.connect( "expose-event", self.expose )
        self.connect( "size-allocate", self.size_allocate )
        
    def update_constants( self ):
        
        if self.alloc == None: return
    
        self.borderW = self.get_border_width()
        self.borderWMUL2 = self.borderW*2
        self.corner = self.radius + self.borderW
        self.cornerMUL2 = self.corner*2
        self.cornerMINborderW = self.corner - self.borderW
                
        self.xPLUborderW = self.alloc.x + self.borderW
        self.xPLUcorner = self.alloc.x + self.corner
        self.xPLUwidthMINborderW = self.alloc.x + self.alloc.width - self.borderW
        self.xPLUwidthMINcorner = self.alloc.x + self.alloc.width - self.corner
        self.yPLUborderW = self.alloc.y + self.borderW
        self.yPLUcorner = self.alloc.y + self.corner
        self.yPLUheightMINborderW = self.alloc.y + self.alloc.height - self.borderW
        self.yPLUheightMINcorner = self.alloc.y + self.alloc.height - self.corner
        self.widthMINborderW = self.alloc.width - self.borderW
        self.widthMINcorner = self.alloc.width - self.corner
        self.widthMINcornerMUL2 = self.alloc.width - self.cornerMUL2
        self.heightMINborderW = self.alloc.height - self.borderW
        self.heightMINcorner = self.alloc.height - self.corner
        self.heightMINborderWMUL2 = self.alloc.height - self.borderWMUL2
        self.heightMINcornerMUL2 = self.alloc.height - self.cornerMUL2
        
        self.roundX1 = self.alloc.x + self.borderW - 1
        self.roundX2 = self.alloc.x + self.alloc.width - self.corner - self.radius - 1
        self.roundY1 = self.alloc.y + self.borderW - 1
        self.roundY2 = self.alloc.y + self.alloc.height - self.corner - self.radius - 1
        self.roundD = self.radius*2 + 1
        self.rightAngle = 90*64
        
    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
        self.update_constants()
        return False
        
    def set_border_width( self, width ):
        gtk.VBox.set_border_width( self, width )
        self.update_constants()
        
    def set_radius( self, radius ):
        self.radius = radius
        self.update_constants()
        
    def set_fill_color( self, color ):
        colormap = self.get_colormap()
        self.fillcolor = colormap.alloc_color(color,True,True)
        
    def set_border_color( self, color ):
        colormap = self.get_colormap()
        self.bordercolor = colormap.alloc_color(color,True,True)
    
    def expose( self, widget, event ):
        
        if self.alloc == None: return
        
        #TP.ProfileBegin( "Round*Box::expose" )
        
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        
        startX = event.area.x - self.alloc.x
        startY = event.area.y - self.alloc.y
        stopX = startX + event.area.width
        stopY = startY + event.area.height
        
        saveForeground = gc.foreground
        
        # Note: could maybe do some optimization to fill only areas that are within the dirty rect, but drawing
        # seems to be quite fast compared to python code, so just leave it at clipping by each geometry feature
        
        gc.foreground = self.bordercolor
        if self.borderW:
            if stopY > self.corner and startY < self.heightMINcorner:
                if startX < self.borderW:         # draw left border 
                    self.window.draw_rectangle( gc, True, self.alloc.x, self.yPLUcorner, self.borderW, self.heightMINcornerMUL2 )
                if stopX > self.widthMINborderW:  # draw right border 
                    self.window.draw_rectangle( gc, True, self.xPLUwidthMINborderW, self.yPLUcorner, self.borderW, self.heightMINcornerMUL2 )
            
            if stopX > self.corner and startX < self.widthMINcorner:
                if startY < self.borderW:         # draw top border
                    self.window.draw_rectangle( gc, True, self.xPLUcorner, self.alloc.y, self.widthMINcornerMUL2, self.borderW )
                if stopY > self.heightMINborderW: # draw bottom border
                    self.window.draw_rectangle( gc, True, self.xPLUcorner, self.yPLUheightMINborderW, self.widthMINcornerMUL2, self.borderW )
        
        if startX < self.corner:
            if startY < self.corner:              # draw top left corner
                self.window.draw_rectangle( gc, True, self.alloc.x, self.alloc.y, self.corner, self.corner )
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX1, self.roundY1, self.roundD, self.roundD, self.rightAngle, self.rightAngle )
                gc.foreground = self.bordercolor
            if stopY > self.heightMINcorner:      # draw bottom left corner
                self.window.draw_rectangle( gc, True, self.alloc.x, self.yPLUheightMINcorner, self.corner, self.corner )
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX1, self.roundY2, self.roundD, self.roundD, -self.rightAngle, -self.rightAngle )
                gc.foreground = self.bordercolor
        if stopX > self.widthMINcorner:
            if startY < self.corner:              # draw top right corner
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.alloc.y, self.corner, self.corner )                
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX2, self.roundY1, self.roundD, self.roundD, 0, self.rightAngle )
                gc.foreground = self.bordercolor
            if stopY > self.heightMINcorner:      # draw bottom right corner
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.yPLUheightMINcorner, self.corner, self.corner )                                
                gc.foreground = self.fillcolor
                self.window.draw_arc( gc, True, self.roundX2, self.roundY2, self.roundD, self.roundD, 0, -self.rightAngle )
                gc.foreground = self.bordercolor
        
        gc.foreground = self.fillcolor
        if startX < self.widthMINcorner and stopX > self.corner:
            if startY < self.heightMINborderW and stopY > self.borderW: # draw centre fill
                self.window.draw_rectangle( gc, True, self.xPLUcorner, self.yPLUborderW, self.widthMINcornerMUL2, self.heightMINborderWMUL2 )
        if startX < self.corner and stopX > self.borderW:
            if startY < self.heightMINcorner and stopY > self.corner:   # draw left fill
                self.window.draw_rectangle( gc, True, self.xPLUborderW, self.yPLUcorner, self.cornerMINborderW, self.heightMINcornerMUL2 )
        if startX < self.widthMINborderW and stopX > self.widthMINcorner:
            if startY < self.heightMINcorner and stopY > self.corner:   # draw right fill
                self.window.draw_rectangle( gc, True, self.xPLUwidthMINcorner, self.yPLUcorner, self.cornerMINborderW, self.heightMINcornerMUL2 )
                
        gc.foreground = saveForeground
        
        #TP.ProfileEnd( "Round*Box::expose" )
        
        return False
    
class ImageButton(gtk.Button):
    
    def __init__(self , image_path, enter_image_path = None, click_image_path = None):
        gtk.Button.__init__(self)
        self.alloc = None
        
        if enter_image_path != None:
            self.connect('enter',self.on_btn_enter, None)
            self.connect('leave',self.on_btn_leave, None)
            
        if click_image_path != None:
            self.connect('pressed',self.on_btn_press, None)
            self.connect('released',self.on_btn_release, None)
        
        self.image_path = image_path
        self.enter_image_path = enter_image_path
        self.click_image_path = click_image_path
        
        self.image = gtk.Image()
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.set_size_request(self.img_pixbuf.get_width(),self.img_pixbuf.get_height())
        
        self.connect('expose-event', self.expose)
        self.connect('size-allocate', self.size_allocate)
        
    def size_allocate(self, widget, allocation):
        self.alloc = allocation
        
    def expose(self, widget, event):
        style = self.get_style()
        gc = style.fg_gc[gtk.STATE_NORMAL]
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width//2) - self.img_pixbuf.get_width() // 2 , self.alloc.y + (self.alloc.height//2) - (self.img_pixbuf.get_height() // 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
        
    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, widget, event):
        self.image.set_from_file(self.image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
        
    def on_btn_press(self, widget, event):
        self.image.set_from_file(self.click_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
        
    def on_btn_release(self, widget, event):
        if self.enter_image_path != None:
            self.image.set_from_file(self.enter_image_path)
        else:
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
        self.set_size_request(self.img_pixbuf.get_width(),self.img_pixbuf.get_height())
        
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
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width//2) - self.img_pixbuf.get_width() // 2 , self.alloc.y + (self.alloc.height//2) - (self.img_pixbuf.get_height() // 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
    
    def on_btn_click(self, widget, event):
        self.switch()

    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, widget, event):
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
        self.set_size_request(self.img_pixbuf.get_width(),self.img_pixbuf.get_height())
        
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
        self.window.draw_pixbuf(gc, self.img_pixbuf, 0, 0, self.alloc.x + (self.alloc.width//2) - self.img_pixbuf.get_width() // 2 , self.alloc.y + (self.alloc.height//2) - (self.img_pixbuf.get_height() // 2), width=self.img_pixbuf.get_width() , height=self.img_pixbuf.get_height(), dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        return True
    
    def on_btn_click(self, widget, event):
        self.switch()
            
    def on_btn_enter(self, widget, event):
        self.image.set_from_file(self.enter_image_path)
        self.img_pixbuf = self.image.get_pixbuf()
        self.queue_draw()
    
    def on_btn_leave(self, widget, event):
        if self.get_active() == True:
            self.image.set_from_file(self.altImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
        else:
            self.image.set_from_file(self.mainImg_path)
            self.img_pixbuf = self.image.get_pixbuf()
            self.queue_draw()
        
        
