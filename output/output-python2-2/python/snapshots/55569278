import pygtk
pygtk.require( '2.0' )
import gtk

from PageView import PageView

class TunePageView( PageView ):
    def __init__( self, pageID, tuneIndex, selectPageCallback, selected = False ):
        PageView.__init__( self, pageID, selectPageCallback, selected )
        
        self.pageIndex = tuneIndex
        
    def handleButtonPress( self, widget, data ):
        self.selectPageCallback( self.tuneIndex )

    def getData( self, widget, context, selection, targetType, eventTime ):
        return selection.set( gtk.gdk.SELECTION_PRIMARY, 32, "t %d %d" % (self.pageID,self.pageIndex) )
