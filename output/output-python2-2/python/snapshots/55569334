import pygtk
pygtk.require( '2.0' )
import gtk

import pango

from GUI.GUIConstants import GUIConstants

class PageView( gtk.DrawingArea ):
    def __init__( self, pageID, selectPageCallback, selected = False ):
        gtk.DrawingArea.__init__( self )
        
        self.pageID = pageID
        self.selectPageCallback = selectPageCallback
        self.selected = selected
        
        self.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        
        self.connect( "expose-event", self.handleExposeEvent )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "drag_data_get", self.getData )

    def handleButtonPress( self, widget, event ):
        if event.button == 1 or event.button == 3:
            self.selectPageCallback( self.pageID, event.button == 1 )
                    
    def getData( self, widget, context, selection, targetType, eventTime ):
        return selection.set( gtk.gdk.SELECTION_PRIMARY, 32, "p %d" % self.pageID )
            
    def toggleSelected( self ):
        self.selected = not self.selected
        self.queue_draw()
        
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
        layout = self.create_pango_layout( "%d" % ( self.pageID + 1 ) )
        layout.set_font_description( pango.FontDescription( 'Sans 10' ) )
        self.window.draw_layout( self.window.new_gc(), 5, 5, layout )
