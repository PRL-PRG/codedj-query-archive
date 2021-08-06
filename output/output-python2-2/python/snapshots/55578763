import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.GUIConstants import GUIConstants
from GUI.Core.TunePageView import TunePageView

class TuneView( gtk.ScrolledWindow ):
    def __init__( self, selectPageCallback, tunePages ):
        gtk.ScrolledWindow.__init__( self )
        
        self.selectPageCallback = selectPageCallback
        self.tunePages = tunePages
        
        self.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_AUTOMATIC )
        self.set_placement( gtk.CORNER_TOP_LEFT )

        self.pageViews = []       
        self.pageContainer = gtk.HBox( False )
        self.add_with_viewport( self.pageContainer )
        
        self.pageContainer.drag_dest_set( gtk.DEST_DEFAULT_ALL,
                                          [ ( "page_bank_drag", gtk.TARGET_SAME_APP, 10 ) ],
                                          gtk.gdk.ACTION_COPY )
        
        self.pageContainer.connect( "drag_data_received", self.receivedData )

    def receivedData( self, widget, context, x, y, selectionData, info, time ):
        self.addPage( int( selectionData.data ), len( self.tunePages ) )

    def addPage( self, pageID, pageIndex ):
        self.tunePages.insert( pageIndex, pageID )
        
        pageView = TunePageView( pageID, pageIndex, self.selectPage )
        self.pageViews.insert( pageIndex, pageView )
        self.pageContainer.pack_start( pageView, False )
        self.pageContainer.reorder_child( pageView, pageIndex )
        
        pageView.set_size_request( self.pageContainer.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, 
                                   GUIConstants.PAGE_HEIGHT )
        pageView.show()
        self.selectPage( pageIndex )

    def movePage( self, pageView, pageIndex ):
        self.pageContainer.reorder_child( pageView, pageIndex )
        pageView.pageIndex = pageIndex
        
    def selectPage( self, selectedPageIndex, invokeCallback = True ):
        if not self.pageViews[ selectedPageIndex ].selected:
            for pageIndex in range( len( self.pageViews ) ):
                self.pageViews[ pageIndex ].setSelected( pageIndex == selectedPageIndex )
                
            if invokeCallback:
                self.selectPageCallback( selectedPageIndex )
            
    def deselectAll( self ):
        for pageIndex in range( len( self.pageViews ) ):
            self.pageViews[ pageIndex ].setSelected( False )
            
    def set_size_request( self, width, height ):
        gtk.ScrolledWindow.set_size_request( self, width, height )
        
        for pageIndex in range( len( self.pageViews ) ):
            self.pageViews[ pageIndex ].set_size_request( width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, GUIConstants.PAGE_HEIGHT )