import pygtk
pygtk.require( '2.0' )
import gtk

import pango

from GUI.GUIConstants import GUIConstants

class PageView( gtk.DrawingArea ):
    def __init__( self, pageID ):
        gtk.DrawingArea.__init__( self )
        
        self.pageID = pageID
        self.selected = False
        
        self.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        
        self.connect( "expose-event", self.handleExposeEvent )
        self.connect( "button-press-event", self.handleButtonPress )
        self.set_size_request( GUIConstants.PAGE_WIDTH, GUIConstants.PAGE_HEIGHT )

    def handleButtonPress( self, widget, event ):
        if not self.selected:
            #TODO: um.... yeah.
            self.parent.parent.parent.selectPage( self.pageID )
            
    def setSelected( self, selected ):
        if self.selected != selected:
            self.selected = selected
            self.queue_draw()

    # TODO: this is temporary: replace with a visual representation of the page
    def handleExposeEvent( self, drawingArea, event ):
        size = self.get_allocation()
        context = self.window.cairo_create()
        
        if self.selected:
            context.set_line_width( GUIConstants.PAGE_SELECTED_BORDER_SIZE )
        else:
            context.set_line_width( GUIConstants.PAGE_BORDER_SIZE )
        context.move_to( 0, 0 )
        context.rel_line_to( size.width, 0 )
        context.rel_line_to( 0, size.height )
        context.rel_line_to( -size.width, 0 )
        context.close_path()
            
        #blue background
        context.set_source_rgb( 0.75, 0.75, 0.75 )
        context.fill_preserve()
            
        #black border
        context.set_source_rgb( 0, 0, 0 )
        context.stroke()
        
        #text
        layout = self.create_pango_layout( "%d" % self.pageID )
        layout.set_font_description( pango.FontDescription( 'Sans 14' ) )
        self.window.draw_layout( self.window.new_gc(), 42, 15, layout )