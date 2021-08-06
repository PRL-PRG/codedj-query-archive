import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.GUIConstants import GUIConstants
from GUI.Core.PageView import PageView

class PageBankView( gtk.Frame ):

    NO_PAGE = -1

    def __init__( self, selectPageCallback, pageDropCallback ):
        gtk.Frame.__init__( self )
        self.table = gtk.Table( 1, GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS )
        self.add( self.table )
        self.drag_dest_set( gtk.DEST_DEFAULT_ALL, [ ( "tune page", gtk.TARGET_SAME_APP, 11 )], gtk.gdk.ACTION_COPY|gtk.gdk.ACTION_MOVE )
        self.connect( "drag_data_received", self.dragDataReceived )
        self.selectPageCallback = selectPageCallback
        self.pageDropCallback = pageDropCallback
        self.selectedPageIds = set([])
        self.pageIndexDictionary = {}
        self.pageViews = {}

    def dragDataReceived( self, widget, context, x, y, selectionData, info, time):
        self.pageDropCallback( selectionData.data )
        
    def addPage( self, pageId, invokeCallback = True ):
        pageIndex = len( self.pageViews.keys() )
        self.pageIndexDictionary[ pageIndex ] = pageId
        
        #TODO: resize table to suit number of pages?
        #if pageIndex > ( self.table.n-rows * self.table.n_columns ):
        #    self.table.resize( self.table.n_rows + 1, self.table.n_columns )
        
        pageView = PageView( pageIndex, self.selectPage, True )
        self.pageViews[ pageIndex ] = pageView
            
        columnIndex = pageIndex % GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS
        rowIndex = int( pageIndex / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS )
        self.table.attach( pageView, columnIndex, columnIndex + 1, rowIndex, rowIndex + 1, gtk.SHRINK, gtk.SHRINK )
        
        self.updateSize( pageView )
        
        pageView.drag_source_set( gtk.gdk.BUTTON1_MASK, 
                                  [ ( "bank page", gtk.TARGET_SAME_APP, 10 ) ],
                                  gtk.gdk.ACTION_COPY )
        
        self.selectPage( pageId, True, invokeCallback )
        
        pageView.show()
            
    def set_size_request( self, width, height ):
        gtk.Frame.set_size_request( self, width, height )
        self.table.set_size_request( width, height )
        for pageId in self.pageViews.keys():
            self.updateSize( self.pageViews[ pageId ] )
            
    def updateSize( self, pageView ):
        pageView.set_size_request( self.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS,
                                   GUIConstants.PAGE_HEIGHT - 1 )
    
    def selectPage( self, selectedPageId, invokeCallback = True, deselectOthers = True ):
        if deselectOthers:
            for pageId in self.pageViews.keys():
                self.pageViews[ pageId ].setSelected( pageId == selectedPageId )
                if pageId != selectedPageId:
                    self.selectedPageIds.discard( pageId )
                else:
                    self.selectedPageIds.add( pageId )
                #nb: pageId might be NO_PAGE, and selectedPageIds can be empty here
            
        else:
            self.pageViews[ selectedPageId ].toggleSelected()
            if self.pageViews[ selectedPageId ].selected:
                self.selectedPageIds.add( selectedPageId )
            else:
                self.selectedPageIds.discard( selectedPageId )
            
        if invokeCallback:
            self.selectPageCallback( selectedPageId )
            
    def getSelectedPageIds( self ):
        rval =  filter( lambda id: self.pageViews[id].selected == True, self.pageViews.keys())
        return rval

