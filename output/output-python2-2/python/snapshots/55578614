import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.GUIConstants import GUIConstants
from GUI.Core.TunePageView import TunePageView

class TuneView( gtk.ScrolledWindow ):
    def _page_width(self):
        return self.pageContainer.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS

    def __init__( self, selectPageCallback, tunePagesCallback ):
        gtk.ScrolledWindow.__init__( self )
        
        #selectPageCallback(): currently connected to pagePlayer.setPlayTune, which skips to a given page of the tune.
        self.selectPageCallback = selectPageCallback

        #tunePagesCallback(): returns pagePlayer's list of pageIDs
        self.tunePagesCallback = tunePagesCallback

        self.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_AUTOMATIC )
        self.set_placement( gtk.CORNER_TOP_LEFT )

        #self.pageViews: list of our custom PageView widgets, which should always correspond to tunePagesCallback
        self.pageViews = [] 
        self.pageContainer = gtk.HBox( False )
        self.add_with_viewport( self.pageContainer )
        
        self.pageContainer.drag_dest_set( gtk.DEST_DEFAULT_ALL,
                                          [ ( "page_bank_drag", gtk.TARGET_SAME_APP, 10 ) ],
                                          gtk.gdk.ACTION_COPY )
        
        self.pageContainer.connect( "drag_data_received", self.receivedData )

    #private method: called by gtk when pages get dragged onto the tune-view
    def receivedData( self, widget, context, x, y, selectionData, info, time ):
        self.addPage( int( selectionData.data ), min( x / self._page_width(), len( self.tunePagesCallback() )), True )

    #public method: called by MainWindow on file load
    def syncFromPagePlayer(self):
        map( lambda pv:pv.destroy(), self.pageViews )
        self.pageViews = []
        tunePages = self.tunePagesCallback()
        for i in range( len(tunePages)):
            self.addPage( tunePages[i], i, False)


    def addPage( self, pageID, position, mess_with_tunePages = True ):
        #NOTE: sneaky to manipulate pagePlayer's data struct this way.
        if mess_with_tunePages : self.tunePagesCallback().insert( position, pageID )
        
        #create a new widget
        pageView = TunePageView( pageID, position, self.selectPage )
        self.pageViews.insert( position, pageView )
        self.pageContainer.pack_start( pageView, False )
        self.pageContainer.reorder_child( pageView, position )
        
        pageView.set_size_request( self.pageContainer.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, 
                                   GUIConstants.PAGE_HEIGHT )
        pageView.show()

        for i in range( len(self.pageViews)) :
            self.pageViews[i].pageIndex = i
            self.pageViews[i].setSelected( i == position)

        self.selectPageCallback( position )

    #public method: who calls this? what does it do?
    #               also, does the caller of this method look after re-ordering pagePlayer's tunePages list?
    def movePage( self, pageView, pageIndex ):
        self.pageContainer.reorder_child( pageView, pageIndex )
        pageView.pageIndex = pageIndex
        
    def selectPage( self, selectedPageIndex, invokeCallback = True ):
        #print 'TuneView::selectPage: selectedPageIndex ', selectedPageIndex
        #print 'TuneView::selectPage ', self.tunePagesCallback()
        if not self.pageViews[ selectedPageIndex ].selected:
            map( lambda pv: pv.setSelected( pv.pageIndex == selectedPageIndex), self.pageViews)

            #print 'TuneView::selectPage ', self.tunePagesCallback()

            if invokeCallback:
                self.selectPageCallback( selectedPageIndex )
            
    def deselectAll( self ):
        # Try a little FP on for size
        map( lambda pv:pv.setSelected(False), self.pageViews )
            
    def set_size_request( self, width, height ):
        gtk.ScrolledWindow.set_size_request( self, width, height )
        map( lambda pv: pv.set_size_request( width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, GUIConstants.PAGE_HEIGHT ), self.pageViews)

