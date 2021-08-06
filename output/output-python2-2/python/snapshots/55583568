# -*- coding: utf-8 -*-

"""
@namespace Area

    Tools and events manipulation


Copyright 2007, NATE-LSI-EPUSP

Oficina is developed in Brazil at Escola Politécnica of 
Universidade de São Paulo. NATE is part of LSI (Integrable
Systems Laboratory) and stands for Learning, Work and Entertainment
Research Group. Visit our web page: 
www.lsi.usp.br/nate
Suggestions, bugs and doubts, please email oficina@lsi.usp.br

Oficina is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License 
as published by the Free Software Foundation version 2 of 
the License.

Oficina is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public
License along with Oficina; if not, write to the
Free Software Foundation, Inc., 51 Franklin St, Fifth Floor, 
Boston, MA  02110-1301  USA.
The copy of the GNU General Public License is found in the 
COPYING file included in the source distribution.


Authors:

Joyce Alessandra Saul               (joycealess@gmail.com)
Andre Mossinato                     (andremossinato@gmail.com)
Nathalia Sautchuk Patrício          (nathalia.sautchuk@gmail.com)
Pedro Kayatt                        (pekayatt@gmail.com)
Rafael Barbolo Lopes                (barbolo@gmail.com)
Alexandre A. Gonçalves Martinazzo   (alexandremartinazzo@gmail.com)

Colaborators:
Bruno Gola                          (brunogola@gmail.com)

Group Manager:
Irene Karaguilla Ficheman           (irene@lsi.usp.br)

Cientific Coordinator:
Roseli de Deus Lopes                (roseli@lsi.usp.br)

UI Design (OLPC):
Eben Eliason                        (eben@laptop.org)

Project Coordinator (OLPC):
Manusheel Gupta                     (manu@laptop.org)

Project Advisor (OLPC):
Walter Bender                       (walter@laptop.org)

"""



import gtk, gobject, logging, os, tempfile
import math
import pango
from fill import *
from Desenho import Desenho

##Tools and events manipulation are handle with this class.
class Area(gtk.DrawingArea):


    __gsignals__ = {
        'undo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'redo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'action-saved' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'select' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
    }

    def __init__(self, janela):
        """ Initialize the object from class Area which is derived from gtk.DrawingArea.

            @param  self -- the Area object (GtkDrawingArea)
            @param  janela -- the parent window

        """
        logging.debug('Area.__init__(self, janela)')
        
        gtk.DrawingArea.__init__(self)
        #self.set_size_request(800, 600)
        self.set_size_request(1185, 770)
        
        self.set_events(gtk.gdk.POINTER_MOTION_MASK |
                gtk.gdk.POINTER_MOTION_HINT_MASK |
                gtk.gdk.BUTTON_PRESS_MASK |
                gtk.gdk.BUTTON_RELEASE_MASK|
                gtk.gdk.EXPOSURE_MASK |
                gtk.gdk.KEY_PRESS_MASK |
                gtk.gdk.KEY_RELEASE_MASK) 
                
        self.connect("expose_event",self.expose)
        self.connect("motion_notify_event", self.mousemove)
        self.connect("button_press_event", self.mousedown)
        self.connect("button_release_event", self.mouseup)
        self.connect("key_press_event", self.key_press)
        self.connect("key_release_event", self.key_release)

        self.set_flags(gtk.CAN_FOCUS)
        self.grab_focus()

        self.set_extension_events(gtk.gdk.EXTENSION_EVENTS_CURSOR)
        ## Define which tool is been used. It is now described as a dictionnary,
        ## with the following keys:
        ## - 'name'          : a string
        ## - 'line size'     : a integer
        ## - 'fill color'    : a gtk.gdk.Color object
        ## - 'stroke color'  : a gtk.gdk.Color object
        ## - 'line shape'    : a string - 'circle' or 'square', for now
        ## - 'fill'          : a Boolean value
        ## - 'vertices'      : a integer
        ## All values migth be None, execept in 'name' key.
        self.tool = {
        'name'          : 'pencil',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
        }
        self.desenha = False
        self.selmove = False
        self.sel_get_out = False
        self.connect("configure_event", self.configure_event)
        self.oldx = 0
        self.oldy = 0
        self.polygon_start = True
        self.points = []
        self.gc = None
        self.gc_rainbow = None
        self.gc_line = None
        self.gc_eraser = None
        self.gc_brush = None
        self.gc_selection = None
        self.pixmap = None  
        self.pixmap_temp = None
        self.pixmap_sel = None
        self.desenho = []   
        self.textos = []
        self.estadoTexto = 0
        self.janela = janela    
        self.d = Desenho(self)
        self.line_size = 2
        self.line_shape = 'circle'
        self.last = []
        self.rainbow_counter = 0
                
        self.font = pango.FontDescription('Sans 9')
        self._set_selection_bounds(0,0,0,0)
        
        #start of UNDO and REDO
        ## This flag is used when is the first time you click on Undo
        self.first_undo = True
        ## When you are just clicking on undo or redo and not drawing undo_surf is True
        self.undo_surf = False
        self.undo_times = 0
        self.redo_times = 0
        ##pixmaps list to Undo func
        self.undo_list=[]
        
        ##Number of sides for regular polygon
        self.vertices = 5
        
        ##Shapes will be filled or not?
        self.fill = True


    def configure_event(self, widget, event):
        """Configure the Area object.

        @param self -- the Area object (GtkDrawingArea)
        @param widget -- the Area object (GtkDrawingArea)
        @param event -- GdkEvent

        """
        logging.debug('Area.configure_event(self, widget, event)')
        
        win = widget.window
        width = win.get_geometry()[2]
        height = win.get_geometry()[3]  
        
        ##It is the main pixmap, who is display most of the time.
        self.pixmap = gtk.gdk.Pixmap(win, width, height, -1)
        self.pixmap.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        ##This pixmap is showed when we need show something and not draw it.
        self.pixmap_temp = gtk.gdk.Pixmap(win, width, height, -1)
        self.pixmap_temp.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        
        ##When something is selected this pixmap draw and rectangular box out of the selection
        #self.pixmap_sel = gtk.gdk.Pixmap(win, width, height, -1)
        #self.pixmap_sel.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        
        self.gc = win.new_gc()    
        self.gc_eraser = win.new_gc()
        colormap = self.get_colormap()
        white = colormap.alloc_color('#ffffff', True, True) # white      
        self.gc_eraser.set_foreground(white)
        self.gc_rainbow = win.new_gc()
        
        self.gc_brush = win.new_gc()      
        self.gc_brush.set_foreground(white)
                
        self.gc_line = win.new_gc()   

        self.gc_selection = win.new_gc()  
        self.gc_selection.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        black = colormap.alloc_color('#000000', True, True)  # black
        self.gc_selection.set_foreground(black)
        
        self.gc_selection1 = win.new_gc()  #this make another white line out of the black line
        self.gc_selection1.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        self.gc_selection1.set_foreground(white)
        
        
        self.enableUndo(widget)
        
        # Setting a initial tool
        # If not set here, cursor icon can't be load
        self.set_tool(self.tool)
        
        return True
        
    def configure_line(self, size):
        """Configure the new line's size.

            @param  self -- the Area object (GtkDrawingArea)
            @param  size -- the size of the new line

        """     
        #logging.debug('Area.configure_line(self, size)')
        
        self.line_size = size
        self.gc_line.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)

    def expose(self, widget, event):
        """ This function define which pixmap will be showed to the user.
            Show up the Area object (GtkDrawingArea).

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)
            @param  event -- GdkEvent

        """
        #logging.debug('Area.expose(self, widget, event)')
        area = event.area
        if self.tool['name'] is not 'text':
            self.grab_focus()
        if self.desenha or self.selmove:
            widget.window.draw_drawable(self.gc,self.pixmap_temp,area[0],area[1],area[0],area[1],area[2],area[3])
        else:
            widget.window.draw_drawable(self.gc,self.pixmap,area[0],area[1],area[0],area[1],area[2],area[3])
        return False

    def mousedown(self,widget,event):
        """Make the Area object (GtkDrawingArea) recognize that the mouse button has been pressed.

            @param self -- the Area object (GtkDrawingArea)
            @param widget -- the Area object (GtkDrawingArea)
            @param event -- GdkEvent

        """
        width, height = self.window.get_size()
        coords = int(event.x), int(event.y)
        
        # text
        if self.tool['name'] == 'text':
            self.d.text(widget,event)
            
        # This fixes a bug that made the text viewer get stuck in the canvas
        elif self.estadoTexto is 1:
            try:
            # This works for a gtk.Entry
                text = self.janela.textview.get_text()
            except AttributeError:
            # This works for a gtk.TextView
                buf = self.janela.textview.get_buffer()
                start, end = buf.get_bounds()
                text = buf.get_text(start, end)
                
            if text is not None:
                self.d.text(widget,event)
            self.estadoTexto = 0
            self.janela.textview.hide()
            
        self.oldx, self.oldy = coords
        
        if self.polygon_start is False and self.tool['name'] is not 'polygon':
            self.d.polygon(widget, coords, False, self.tool['fill'],"bug")
        
        x , y, state = event.window.get_pointer()

        if state & gtk.gdk.BUTTON3_MASK:#Handle with the right button click event.
            if self.tool['name'] == 'marquee-rectangular':
                self.sel_get_out = True
        elif state & gtk.gdk.BUTTON1_MASK: #Handle with the left button click event.
            if self.tool['name'] == 'eraser':
                self.last = []
                self.d.eraser(widget, coords, self.last, self.line_size, self.tool['line shape'])
                self.last = coords
            elif self.tool['name'] == 'brush':
                self.last = []
                self.d.brush(widget, coords, self.last, self.line_size, self.tool['line shape'])
                self.last = coords
            elif self.tool['name'] == 'rainbow':
                self.last = []
                self.d.rainbow(widget, coords, self.last, self.rainbow_counter,self.line_size, self.tool['line shape'])
                self.last = coords
            elif self.tool['name'] == 'polygon':
                self.configure_line(self.line_size)
                if self.polygon_start == False:
                    self.desenha = True
            if self.selmove:
                if self.tool['name'] != 'marquee-rectangular': #get out of the func selection
                    self.getout()
                    self.selmove = False
                else:
                    size = self.pixmap_sel.get_size()
                    xi = self.orig_x
                    yi = self.orig_y
                    xf = xi + size[0]
                    yf = yi + size[1]
                    if (coords[0] < xi) or (coords[0] > xf) or (coords[1] < yi) or (coords[1] > yf):
                        self.sel_get_out = True
            else:
                self.pixmap_temp.draw_drawable(self.gc, self.pixmap, 0,0,0,0, width, height)
            self.desenha = True
        widget.queue_draw()


        
    def mousemove(self,widget,event):
        """Make the Area object (GtkDrawingArea) recognize that the mouse is moving.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)
            @param  event -- GdkEvent

        """
        x = event.x
        y = event.y
        state = event.state

        coords = int(x), int(y)
                        
        if state & gtk.gdk.BUTTON1_MASK and self.pixmap != None:
            if self.tool['name'] == 'pencil':
                self.d.brush(widget, coords, self.last, 2, 'circle')
                self.last = coords
            elif self.tool['name'] == 'eraser':
                self.d.eraser(widget, coords, self.last, self.line_size, self.tool['line shape'])
                self.last = coords
            elif self.tool['name'] == 'brush':
                self.d.brush(widget, coords, self.last, self.line_size, self.tool['line shape'])
                self.last = coords
            elif self.tool['name'] == 'rainbow':
                self.d.rainbow(widget, coords, self.last, self.rainbow_counter,self.line_size, self.tool['line shape'])
                self.rainbow_counter += 1
                if self.rainbow_counter > 11:
                    self.rainbow_counter = 0
                self.last = coords
            if self.desenha:
                if self.tool['name'] == 'line':
                    self.configure_line(self.line_size)
                    self.d.line(widget, coords)
                    
                elif self.tool['name'] == 'ellipse':
                    self.configure_line(self.line_size)
                    self.d.circle(widget,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'rectangle':
                    self.configure_line(self.line_size)
                    self.d.square(widget,event,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'marquee-rectangular' and not self.selmove:
                    self.d.selection(widget,coords)
                # selected
                elif self.tool['name'] == 'marquee-rectangular' and self.selmove:
                    if not self.sel_get_out:
                        self.d.moveSelection(widget,coords)
                        
                elif self.tool['name'] == 'polygon':
                    self.configure_line(self.line_size)
                    self.d.polygon(widget,coords,True,self.tool['fill'],"motion")
                    
                elif self.tool['name'] == 'triangle':
                    self.configure_line(self.line_size)
                    self.d.triangle(widget,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'trapezoid':
                    self.configure_line(self.line_size)
                    self.d.trapezoid(widget,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'arrow':
                    self.configure_line(self.line_size)
                    self.d.arrow(widget,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'parallelogram':
                    self.configure_line(self.line_size)
                    self.d.parallelogram(widget,coords,True,self.tool['fill'])
                    
                elif self.tool['name'] == 'star':
                    self.configure_line(self.line_size)
                    self.d.star(widget,coords,self.tool['vertices'],True,self.tool['fill'])
                    
                elif self.tool['name'] == 'polygon_regular':
                    self.configure_line(self.line_size)
                    self.d.polygon_regular(widget,coords,self.tool['vertices'],True,self.tool['fill'])
                    
                elif self.tool['name'] == 'heart':
                    self.configure_line(self.line_size)
                    self.d.heart(widget,coords,True,self.tool['fill'])
        else:
            if self.tool['name'] == 'marquee-rectangular' and self.selmove:
                size = self.pixmap_sel.get_size()
                xi = self.orig_x
                yi = self.orig_y
                xf = xi + size[0]
                yf = yi + size[1]
                if (coords[0] < xi) or (coords[0] > xf) or (coords[1] < yi) or (coords[1] > yf):
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                else:
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR))

            elif self.tool['name'] == 'polygon' and not self.selmove:
                self.desenha = True
                self.configure_line(self.line_size)
                self.d.polygon(widget,coords,True,self.tool['fill'],"moving")

        gtk.gdk.event_request_motions (event)

    def mouseup(self,widget,event): 
        """Make the Area object (GtkDrawingArea) recognize that the mouse was released.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)
            @param  event -- GdkEvent

        """
        coords = int(event.x), int(event.y)
        width, height = self.window.get_size()
        if self.desenha or self.sel_get_out:
            if self.tool['name'] == 'line':
                self.pixmap.draw_line(self.gc_line,self.oldx,self.oldy, int (event.x), int(event.y))                
                widget.queue_draw()
                self.enableUndo(widget)
                
            elif self.tool['name'] == 'ellipse':
                self.d.circle(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)
            
            elif self.tool['name'] == 'rectangle':
                self.d.square(widget,event,coords,False,self.tool['fill'])
                self.enableUndo(widget)

            elif self.tool['name'] == 'marquee-rectangular':
                if self.selmove == False:
                    self.d.selection(widget,coords,False)
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR))
                    self.selmove = True
                    self.sel_get_out = False
                    self.emit('select')
                elif self.sel_get_out: #get out of the func selection
                    self.getout()
                    try:
                        del(self.d.pixbuf_resize)
                    except: pass
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                    widget.queue_draw()
                    self.enableUndo(widget)
                elif self.selmove:
                    self.orig_x = self.orig_x + coords[0] - self.oldx
                    self.orig_y = self.orig_y + coords[1] - self.oldy
                self.emit('select')

            elif self.tool['name'] == 'polygon':
                self.d.polygon(widget, coords, False, self.tool['fill'],"release")

            elif self.tool['name'] == 'bucket':
                width, height = self.window.get_size()
                fill(self.pixmap, self.gc, coords[0], coords[1], width, height, self.gc_line.foreground.pixel)
                widget.queue_draw()
                self.enableUndo(widget)
  
            elif self.tool['name'] == 'triangle':
                self.d.triangle(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)
  
            elif self.tool['name'] == 'trapezoid':
                self.d.trapezoid(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)
 
            elif self.tool['name'] == 'arrow':
                self.d.arrow(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)
 
            elif self.tool['name'] == 'parallelogram':
                self.d.parallelogram(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)

            elif self.tool['name'] == 'star':
                self.d.star(widget,coords,self.tool['vertices'],False,self.tool['fill'])
                self.enableUndo(widget)

            elif self.tool['name'] == 'polygon_regular':
                self.d.polygon_regular(widget,coords,self.tool['vertices'],False,self.tool['fill'])
                self.enableUndo(widget)

            elif self.tool['name'] == 'heart':
                self.d.heart(widget,coords,False,self.tool['fill'])
                self.enableUndo(widget)

        if self.tool['name'] == 'brush' or self.tool['name'] == 'eraser' or self.tool['name'] == 'rainbow' or self.tool['name'] == 'pencil' :
            self.last = []
            widget.queue_draw() 
            self.enableUndo(widget)
        self.desenha = False
        
    def undo(self):
        """Undo the last drawing change.

            @param  self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.undo(self)')
        width, height = self.window.get_size()
        
        #if is the first time you click on UNDO (because undo_list always wait for the NEXT image)
        if self.first_undo:
            self.undo_times -= 1
            
        #print "Undo no.%d" %(self.undo_times)
        if self.undo_times >0 : 
            self.undo_times -= 1
            self.redo_times += 1
            try: #to not try paint someting wrong
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, width, height)
            except:
                logging.debug('Cant draw')
                pass
            self.queue_draw()
        else:   
            self.undo_times = 0
            
        self.first_undo=False
        self.undo_surf = True


        #special case for func polygon
        if self.tool['name'] == 'polygon':        
                self.polygon_start = True #start the polygon again
        

        self.emit('undo')
        
    def redo(self):
        """Redo the last undo operation.

            @param  self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.redo(self)')
        width, height = self.window.get_size()
        
        #print "REDO no.%d" %(self.redo_times)
        if  (self.redo_times>0):
            self.redo_times -= 1
            self.undo_times += 1
            
            try: #to not try paint someting wrong 
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, width, height)
            except:
                logging.debug('Cant draw')
                self.undo_times-=1
        self.queue_draw()
        
        self.emit('redo')
            
    def enableUndo(self,widget):
        """Keep the last change in a list for Undo/Redo commands.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)

        """
        #logging.debug('Area.enableUndo(self,widget)')
        
        width, height = self.window.get_size()
        
        if self.undo_surf:
            self.undo_times += 1
        
        self.undo_list.append(None)#alloc memory
        self.undo_list[self.undo_times] = gtk.gdk.Pixmap(widget.window, width, height, -1) #define type
        self.undo_list[self.undo_times].draw_drawable(self.gc,self.pixmap,0,0,0,0, width, height) #copy workarea
        self.undo_times += 1
        self.redo_times = 0 
        self.first_undo = True
        self.undo_surf = False
        
        #This is the part where we can limit the steps of undo/redo     
        if self.undo_times==12:
           self.undo_list.pop(0)
           self.undo_times-=1
        
        self.emit('action-saved')
           
    def copy(self):
        """ Copy Image.
            When the tool selection is working make the change the copy of selectioned area
        
            @param  self -- the Area object (GtkDrawingArea)
        """
        clipBoard = gtk.Clipboard()
        if 'SUGAR_ACTIVITY_ROOT' in os.environ:
            temp_dir = os.path.join(os.environ.get('SUGAR_ACTIVITY_ROOT'), 'instance')
        else:
            temp_dir = '/tmp'

        f, tempPath = tempfile.mkstemp(suffix='.png', dir=temp_dir)
        del f
        
        if self.selmove:
            size = self.pixmap_sel.get_size()
            pixbuf_copy = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB,True,8,size[0],size[1])
            pixbuf_copy.get_from_drawable(self.pixmap_sel, gtk.gdk.colormap_get_system(),0,0,0,0,size[0],size[1])
            pixbuf_copy.save(tempPath,'png')
            os.chmod(tempPath, 0604)

            clipBoard.set_with_data( [('text/uri-list', 0, 0)], self._copyGetFunc, self._copyClearFunc, tempPath)
        else :
            logging.debug('Area.copy(self): Please select some area first')
            
    def _copyGetFunc(  self, clipboard, selection_data, info, data ):
        """  Determine type data to put in clipboard  
        
            @param  self -- the Area object (GtkDrawingArea)
            @param  clipboard -- a gtk.Clipboard object
            @param  selection_data -- data of selection
            @param  info -- the application assigned integer associated with a target
            @param  data -- user data (tempPath)
        """
        tempPath = data

        if selection_data.target == "text/uri-list":
            selection_data.set_uris(['file://' + tempPath])

    def _copyClearFunc( self, clipboard, data ):
        """ Clear the clipboard
        
            @param  self -- the Area object (GtkDrawingArea)
            @param  clipboard -- a gtk.Clipboard object
            @param  data -- user data (tempPath)
        """
        if (data != None):
            if (os.path.exists(data)):
                os.remove( data )
        data = None
           
    def past(self,widget):
        """ Past image.
        Past image that is in pixmap
        
            @param  self -- the Area object (GtkDrawingArea)
        """
        width, height = self.window.get_size()
        
        tempPath = os.path.join("/tmp", "tempFile")
        tempPath = os.path.abspath(tempPath)
        
        clipBoard = gtk.Clipboard()
        
        if clipBoard.wait_is_image_available():
            self.getout(True,widget)
            pixbuf_sel = clipBoard.wait_for_image()
            size = (int)(pixbuf_sel.get_width()),(int)(pixbuf_sel.get_height())
            self.pixmap_sel = gtk.gdk.Pixmap(self.window,size[0],size[1],-1)
            self.pixmap_sel.draw_pixbuf(self.gc,pixbuf_sel,0,0,0,0,size[0],size[1],dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)

            self.selmove = True
            self.desenha = True
            self.sel_get_out = False
            self.orig_x, self.orig_y = 0,0

            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap_sel,0,0,0,0,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,0,0,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection1,False,-1,-1,size[0]+2,size[1]+2)

            self.tool['name'] = 'marquee-rectangular'
            self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR)) 
            self.emit('select')
        else:
            self.loadImage(tempPath, self, True)
            logging.debug('Area.past(self): Load from clipboard fails, loading from tempPatch')
        
        self.queue_draw()
            
    def set_fill_color(self, color):
        """Set fill color.

            @param  self -- the Area object (GtkDrawingArea)
            @param  color -- a gdk.Color object

        """
        logging.debug('Area._set_fill_color(self, color)')
        
        self.gc.set_foreground(color)
        
 
    def set_stroke_color(self, color):
        """Set stroke color.

            @param  self -- the Area object (GtkDrawingArea)
            @param  color -- a gdk.Color object

        """
        logging.debug('Area._set_stroke_color(self, color)')
        
        self.gc_line.set_foreground(color)
        self.gc_line.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)  
        self.gc_brush.set_foreground(color)

    def grayscale(self,widget):
        """Apply grayscale effect.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)

        """

        logging.debug('Area._set_grayscale(self,widget)')
        width, height = self.window.get_size()
         
        if self.selmove:
            size = self.pixmap_sel.get_size()
            pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB,False,8,size[0],size[1])
            pix.get_from_drawable(self.pixmap_sel,gtk.gdk.colormap_get_system(),0,0,0,0,size[0],size[1])
        else:
            pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB,False,8,width,height)
            pix.get_from_drawable(self.pixmap,gtk.gdk.colormap_get_system(),0,0,0,0,width,height)

        pix.saturate_and_pixelate(pix,0,0)

        try:
            self.d.pixbuf_resize.saturate_and_pixelate(self.d.pixbuf_resize,0,0)
        except: pass

        if self.selmove:
            self.pixmap_sel.draw_pixbuf(self.gc,pix,0,0,0,0,size[0],size[1],dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)

            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap_sel,0,0,self.orig_x,self.orig_y,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,self.orig_x,self.orig_y,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection1,False,self.orig_x-1,self.orig_y-1,size[0]+2,size[1]+2)

        else:
            self.pixmap.draw_pixbuf(self.gc,pix,0,0,0,0,width,height,dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)
        
        self.queue_draw()
        if not self.selmove:
            self.enableUndo(widget)

    def _pixbuf2Image(self, pb):
        """change a pixbuf to RGB image

            @param  self -- the Area object (GtkDrawingArea)
            @param  pb -- the pixbuf object (gtk.gdk.Pixbuf)
            
            @return RGB Image

        """
        width,height = pb.get_width(),pb.get_height()
        return Image.fromstring("RGB",(width,height),pb.get_pixels() )
        
    def _image2pixbuf(self, im):  
        """change a RGB image to a pixbuf

            @param  self -- the Area object (GtkDrawingArea)
            @param  im -- a RGB image
            
            @return pixbuf

        """
        file1 = StringIO.StringIO()  
        im.save(file1, "ppm")  
        contents = file1.getvalue()  
        file1.close()  
        loader = gtk.gdk.PixbufLoader("pnm")  
        loader.write(contents, len(contents))  
        pixbuf = loader.get_pixbuf()  
        loader.close()  
        return pixbuf  
    
    def _rotate_left(self, widget):
        """Rotate the image.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)
            

        """
        logging.debug('Area._rotate_left(self)')
        if self.selmove:
            width, height = self.window.get_size()
            size = self.pixmap_sel.get_size()
            pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False,8,size[0],size[1])
            pix.get_from_drawable(self.pixmap_sel, gtk.gdk.colormap_get_system(),0,0,0,0,size[0],size[1])
            pix = pix.rotate_simple(90)
            try:
                 self.d.pixbuf_resize = self.d.pixbuf_resize.rotate_simple(90)
            except: pass
            try:
                del(self.pixmap_sel)
            except: pass
            self.pixmap_sel = gtk.gdk.Pixmap(widget.window,size[1],size[0],-1)
            self.pixmap_sel.draw_pixbuf(self.gc,pix,0,0,0,0,width=-1,height=-1,dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap_sel,0,0,self.orig_x,self.orig_y,size[1],size[0])
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,self.orig_x,self.orig_y,size[1],size[0])
            self.pixmap_temp.draw_rectangle(self.gc_selection1,False,self.orig_x-1,self.orig_y-1,size[1]+2,size[0]+2)
            self.queue_draw()
                
        else :
            logging.debug('Please select some area first')

    def _rotate_right(self, widget):
        """Rotate the image.

            @param  self -- the Area object (GtkDrawingArea)
            @param  widget -- the Area object (GtkDrawingArea)
            

        """
        logging.debug('Area._rotate_right(self)')
        if self.selmove:
            width, height = self.window.get_size()
            size = self.pixmap_sel.get_size()
            pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False,8,size[0],size[1])
            pix.get_from_drawable(self.pixmap_sel, gtk.gdk.colormap_get_system(),0,0,0,0,size[0],size[1])
            pix = pix.rotate_simple(270)
            try:
                 self.d.pixbuf_resize = self.d.pixbuf_resize.rotate_simple(270)
            except: pass
            try:
                del(self.pixmap_sel)
            except: pass
            self.pixmap_sel = gtk.gdk.Pixmap(widget.window,size[1],size[0],-1)
            self.pixmap_sel.draw_pixbuf(self.gc,pix,0,0,0,0,width=-1,height=-1,dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap_sel,0,0,self.orig_x,self.orig_y,size[1],size[0])
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,self.orig_x,self.orig_y,size[1],size[0])
            self.pixmap_temp.draw_rectangle(self.gc_selection1,False,self.orig_x-1,self.orig_y-1,size[1]+2,size[0]+2)
            self.queue_draw()
                
        else :
            logging.debug('Please select some area first')

    def can_undo(self):
        """
        Indicate if is there some action to undo
        
            @param  self -- the Area object (GtkDrawingArea)
        
        """
#        logging.debug('Area.can_undo(self)')
        
        undo_times = self.undo_times
        
        if self.first_undo:
            undo_times-=1
        
        if undo_times < 1:
            return False
        else:
            return True
            
    def can_redo(self):
        """
        Indicate if is there some action to redo
        
            @param  self -- the Area object (GtkDrawingArea)
            
        """
        #logging.debug('Area.can_redo(self)')
        
        if self.redo_times < 1:
            return False
        else:
            return True
            
    def is_selected(self):
        """
        Return True if there is some thing selected
        
            @param  self -- the Area object (GtkDrawingArea)
            
        """
        
        #logging.debug('Area.is_selected(self)')
        
        if self.selmove:
            return True
        else:
            return False
            
    def _set_selection_bounds(self, x1, y1, x2, y2):
        """
            Set selection bounds   
            
            @param  self -- the Area object (GtkDrawingArea)
            @param  x1,y1,x2,y2 -- the coords of limit points
        """    
        self._selection_corners = (x1, y1, x2, y2)
     
    def get_selection_bounds(self):
        """ 
            Get points of selection
        
            @param  self -- the Area object (GtkDrawingArea)
            
            @return selection_corners
            
        """
        return self._selection_corners[0], self._selection_corners[1], self._selection_corners[2], self._selection_corners[3]

    def loadImage(self, name, widget=None, load_selected=True):
        """Load an image.

            @param  self -- Area instance
            @param  name -- string (image file path)
            @param  widget -- GtkDrawingArea
            @param  load_selected -- False if loading file from Journal

        """
        logging.debug('Area.loadImage')
        logging.debug('Loading file %s', name)
        logging.debug('From Journal? %s', not load_selected)
        
        if not load_selected :
            pixbuf = gtk.gdk.pixbuf_new_from_file(name)
            size = (int)(pixbuf.get_width()), (int)(pixbuf.get_height())
            self.pixmap.draw_pixbuf(self.gc, pixbuf,0,0,0,0,size[0],size[1],dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0, y_dither=0)
            self.undo_times -= 1
            self.enableUndo(widget)
        else :
            pixbuf = gtk.gdk.pixbuf_new_from_file(name)
            size = (int)(pixbuf.get_width()), (int)(pixbuf.get_height())
            #self.getout(True,widget)
            self.getout(True)
            #self.pixmap_sel = gtk.gdk.Pixmap(widget.window,size[0],size[1],-1)
            self.pixmap_sel = gtk.gdk.Pixmap(self.window,size[0],size[1],-1)
            self.pixmap_sel.draw_pixbuf(self.gc,pixbuf,0,0,0,0,size[0],size[1],dither=gtk.gdk.RGB_DITHER_NORMAL,x_dither=0,y_dither=0)

            self.sel_get_out = False
            self.selmove = True
            self.desenha = True
            self.orig_x, self.orig_y = 0,0
            #width, height = widget.window.get_size()
            width, height = self.window.get_size()
            
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap_sel,0,0,0,0,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,0,0,size[0],size[1])
            self.pixmap_temp.draw_rectangle(self.gc_selection1,False,-1,-1,size[0]+2,size[1]+2)
            
            self.tool['name'] = 'marquee-rectangular'
            self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR)) 
            self.emit('select')
        #widget.queue_draw()
        self.queue_draw()
        
    def clear(self):
        """ Clear Canvas
            @param self -- Area instance
        """
        logging.debug('Area.clear')
        self.d.clear(self)
        
        # If something is selected, the action will be saved 
        # after it is unselected
        if not self.is_selected():
            self.enableUndo(self)
        
    # Changing to public methods
    def _set_fill_color(self, color):
        self.set_fill_color(color)
        
    def _set_stroke_color(self, color):
        self.set_stroke_color(color)
        
    def _set_grayscale(self,widget):
        self.grayscale(widget)
    
    def set_tool(self, tool):
        '''
        Method to configure all tools.
        
        @param - tool: a dictionary with the following keys:
                       'name': a string
                       'line size': a integer
                       'fill color': a gtk.gdk.Color object
                       'stroke color': a gtk.gdk.Color object
                       'line shape': a string - 'circle' or 'square', for now
                       'fill': a Boolean value
                       'vertices': a integer
        '''
        logging.debug('Area.set_tool')
        
        #FIXME: self.tool should be a dict too.
        print tool
        
        self.tool = tool
        
        try:
            if self.tool['line size'] is not None:
                self.configure_line(self.tool['line size'])
        
            if self.tool['fill color'] is not None:
                self.set_fill_color(self.tool['fill color'])
            else:
                # use black
                self.set_fill_color( gtk.gdk.Color(0,0,0) )
                
            if self.tool['stroke color'] is not None:
                self.set_stroke_color(self.tool['stroke color'])
            else:
                # use black
                self.set_stroke_color( gtk.gdk.Color(0,0,0) )
        
        except AttributeError:
            pass
        
        if self.tool['line shape'] is not None:
            self.line_shape = self.tool['line shape']
        
        if self.tool['fill'] is not None:
            self.fill = self.tool['fill']
        
        if ( self.tool['name'] is 'polygon_regular' \
           or self.tool['name'] is 'star')\
           and (self.tool['vertices'] is not None):
            self.vertices = self.tool['vertices']
        
        # Setting the cursor
        try:
            pixbuf = gtk.gdk.pixbuf_new_from_file('./images/' + tool['name'] + '.png')
            cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pixbuf, 6, 21)
        except gobject.GError:
            cursor = None
        
        self.window.set_cursor(cursor)
        
    def getout(self,undo=False,widget=None):
        logging.debug('Area.getout')
        
        try:
            self.pixmap_sel
            size = self.pixmap_sel.get_size()
            self.pixmap.draw_drawable(self.gc,self.pixmap_sel,0,0,self.orig_x,self.orig_y,size[0],size[1])
            self.selmove = False
            if undo:
                if widget is not None:
                    self.enableUndo(widget)
                else:
                    self.enableUndo(self)
                    
            del(self.pixmap_sel)
            del(self.pixbuf_resize)
        
        except NameError, message:
            logging.debug(message)
        except Exception, message:
            logging.debug('Unexpected error: %s', message)

    def key_press(self,widget,event):
        if event.keyval == gtk.keysyms.BackSpace:
            if self.selmove:
                self.selmove = False
                try:
                    del(self.pixmap_sel)
                    del(self.pixbuf_resize)
                except: pass
                if self.tool['name'] == 'marquee-rectangular':
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                widget.queue_draw()
                self.enableUndo(widget)
        elif event.keyval == gtk.keysyms.a and gtk.gdk.CONTROL_MASK:
            if self.selmove:
                self.getout()
            width, height = self.window.get_size()
            if self.tool['name'] == 'marquee-rectangular':
                self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR))
            self.pixmap_sel = gtk.gdk.Pixmap(self.window,width,height,-1)
            self.pixmap_sel.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap_temp.draw_drawable(self.gc,self.pixmap,0,0,0,0,width,height)
            self.pixmap.draw_rectangle(self.get_style().white_gc,True,0,0,width,height)
            self.orig_x = 0
            self.orig_y = 0
            self.pixmap_temp.draw_rectangle(self.gc_selection,False,0,0,width-1,height-1)
            self.selmove = True
            self.sel_get_out = False
            self.emit('select')
            widget.queue_draw()
        elif event.keyval == gtk.keysyms.d and gtk.gdk.CONTROL_MASK:
            if self.selmove:
                self.getout(True,widget)
                if self.tool['name'] == 'marquee-rectangular':
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                widget.queue_draw()
        elif event.keyval == gtk.keysyms.Return:
            self.getout(True,widget)
            if self.tool['name'] == 'marquee-rectangular':
                self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
            widget.queue_draw()

    def key_release(self,widget,event):
        pass
