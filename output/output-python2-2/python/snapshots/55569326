import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.GUIConstants import GUIConstants
from GUI.Core.TunePageView import TunePageView

def swap(l,i,j):
    e = l[i]
    l[i] = l[j]
    l[j] = e

class TuneView( gtk.ScrolledWindow ):

    NO_PAGE = -1

    def _page_width(self):
        return self.pageContainer.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS

    def __init__( self, selectPageCallback ):
        gtk.ScrolledWindow.__init__( self )
        
        #selectPageCallback(): currently connected to pagePlayer.setPlayTune, which skips to a given page of the tune.
        self.selectPageCallback = selectPageCallback
        self.selectedPageIndex = self.NO_PAGE

        self.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_AUTOMATIC )
        self.set_placement( gtk.CORNER_TOP_LEFT )

        #self.pageViews: list of our custom PageView widgets
        self.pageViews = [] 
        self.pageContainer = gtk.HBox( False )
        self.add_with_viewport( self.pageContainer )

        #the old part
        self.pageContainer.drag_dest_set( gtk.DEST_DEFAULT_ALL,
                                          [ ( "bank page", gtk.TARGET_SAME_APP, 10 ), 
                                              ( "tune page", gtk.TARGET_SAME_APP, 11 )],
                                          gtk.gdk.ACTION_COPY|gtk.gdk.ACTION_MOVE )

        self.pageContainer.connect( "drag_data_received", self.dragDataReceived )

    #private method: called by gtk when pages get dragged onto the tune-view
    def dragDataReceived( self, widget, context, x, y, selectionData, info, time ):
        print 'dragDataReceived: ', selectionData.data,  info, selectionData.data 
        recv = selectionData.data.split()
        if recv[0] == 'p':
            pageId = int( recv[1] )
            self.addPage( pageId, min( x / self._page_width(), len( self.pageViews )) )
        elif recv[0] == 't':
            self.moveSelectedPage( min( x / self._page_width(), len( self.pageViews ) -1))
        else:
            raise 'ERROR'

    #public method: called by MainWindow on file load
    def syncFromPagePlayer(self):
        raise 'never call this'
        map( lambda pv:pv.destroy(), self.pageViews )
        self.pageViews = []
        tunePages = self.tunePagesCallback()
        for i in range( len(tunePages)):
            self.addPage( tunePages[i], i, False)


    def addPage( self, pageID, position ):
        #create a new widget
        pageView = TunePageView( pageID, position, self.selectPage )
        self.pageViews.insert( position, pageView )
        self.pageContainer.pack_start( pageView, False )
        self.pageContainer.reorder_child( pageView, position )
        
        pageView.set_size_request( self.pageContainer.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, 
                                   GUIConstants.PAGE_HEIGHT )
        pageView.show()

        for i in range( len(self.pageViews)) :
            self.pageViews[i].tuneIndex = i
            self.pageViews[i].setSelected( i == position)
        self.selectPageCallback( pageID, position )
        pageView.drag_source_set( 
            gtk.gdk.BUTTON1_MASK, 
            [   ( "tune page", gtk.TARGET_SAME_APP, 11 ) ],
            gtk.gdk.ACTION_COPY|gtk.gdk.ACTION_MOVE )

    def moveSelectedPage( self, position):
        self.pageContainer.reorder_child( self.pageViews[self.selectedPageIndex], position )
        swap( self.pageViews, self.selectedPageIndex, position )
        self.selectedPageIndex = position
        for i in range( len(self.pageViews) ) :
            self.pageViews[i].tuneIndex = i
            self.pageViews[i].setSelected( i == position)

    def removePage( self, position ):
        pv = self.pageViews[position]
        self.pageViews[position:position+1] = []
        if self.selectedPageIndex >= position : self.selectedPageIndex -= 1
        for i in range( len(self.pageViews)) :
            self.pageViews[i].tuneIndex = i
            self.pageViews[i].setSelected( i == position)
        self.pageContainer.remove(pv)
        del pv
        
    def selectPage( self, selectedPageIndex, invokeCallback = True ):
        if selectedPageIndex >= len( self.pageViews ): selectedPageIndex = self.NO_PAGE
        self.selectedPageIndex = selectedPageIndex
        if selectedPageIndex == self.NO_PAGE:
            for pv in self.pageViews: pv.setSelected(False)
            if invokeCallback: self.selectPageCallback( -1, -1 )
        else:
            if not self.pageViews[ selectedPageIndex ].selected:
                map( lambda pv: pv.setSelected( pv.tuneIndex == selectedPageIndex), self.pageViews)
                if invokeCallback: self.selectPageCallback( self.pageViews[selectedPageIndex].pageID, selectedPageIndex )

    def set_size_request( self, width, height ):
        gtk.ScrolledWindow.set_size_request( self, width, height )
        map( lambda pv: pv.set_size_request( width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS, GUIConstants.PAGE_HEIGHT ), self.pageViews)

    def getPageId( self, idx):
        return self.pageViews[idx].pageID

    def getTune( self ):
        return [ p.pageID for p in self.pageViews ]

