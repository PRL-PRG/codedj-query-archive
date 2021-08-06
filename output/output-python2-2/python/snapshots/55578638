import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.GUIConstants import GUIConstants
from GUI.Core.PageView import PageView

class PageBankView( gtk.Frame ):
    def __init__( self, selectPageCallback, selectedPageIDsCallback ):
        gtk.Frame.__init__( self )
        
        self.table = gtk.Table( 1, GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS )
        self.add( self.table )
        
        self.selectPageCallback = selectPageCallback
        self.selectedPageIDsCallback = selectedPageIDsCallback
        
        self.pageIndexDictionary = {}
        self.pageViews = {}
        
    def addPage( self, pageID, invokeCallback = True ):
        pageIndex = len( self.pageViews.keys() )
        self.pageIndexDictionary[ pageIndex ] = pageID
        
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
                                  [ ( "page_bank_drag", gtk.TARGET_SAME_APP, 10 ) ],
                                  gtk.gdk.ACTION_COPY )
        
        self.selectPage( pageID, True, invokeCallback )
        
        pageView.show()
            
    def set_size_request( self, width, height ):
        gtk.Frame.set_size_request( self, width, height )
        self.table.set_size_request( width, height )
        for pageID in self.pageViews.keys():
            self.updateSize( self.pageViews[ pageID ] )
            
    def updateSize( self, pageView ):
        pageView.set_size_request( self.get_allocation().width / GUIConstants.NUMBER_OF_PAGE_BANK_COLUMNS,
                                   GUIConstants.PAGE_HEIGHT - 1 )
    
    def selectPage( self, selectedPageID, deselectOthers = True, invokeCallback = True ):
        if deselectOthers:
            for pageID in self.pageViews.keys():
                self.pageViews[ pageID ].setSelected( pageID == selectedPageID )
                if pageID != selectedPageID:
                    self.selectedPageIDsCallback().discard( pageID )
                else:
                    self.selectedPageIDsCallback().add( pageID )
            
        else:
            self.pageViews[ selectedPageID ].toggleSelected()
            if self.pageViews[ selectedPageID ].selected:
                self.selectedPageIDsCallback().add( selectedPageID )
            else:
                self.selectedPageIDsCallback().discard( selectedPageID )
            
        if invokeCallback:
            self.selectPageCallback( selectedPageID )
            
    def deselectAll( self ):
        for pageID in self.pageViews.keys():
            self.pageViews[ pageID ].setSelected( False )
            
        self.selectedPageIDsCallback().clear()
