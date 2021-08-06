import pygtk
pygtk.require( '2.0' )
import gtk

from PageView import PageView

class TunePageView( PageView ):
    def __init__( self, pageID, pageIndex, selectPageCallback, selected = False ):
        PageView.__init__( self, pageID, selectPageCallback, selected )
        
        self.pageIndex = pageIndex
        
    def handleButtonPress( self, widget, data ):
        self.selectPageCallback( self.pageIndex )