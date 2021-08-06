import pygtk
pygtk.require( '2.0' )
import gtk

class TuneView( gtk.ScrolledWindow ):
    def __init__( self, selectPageCallback ):
        gtk.ScrolledWindow.__init__( self )
        
        self.selectPageCallback = selectPageCallback
        
        self.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_AUTOMATIC )
        self.set_placement( gtk.CORNER_TOP_LEFT )

        self.pageViews = {}        
        self.pageContainer = gtk.HBox( False )
        self.add_with_viewport( self.pageContainer )

    def addPageView( self, pageView ):
        self.pageViews[ pageView.pageID ] = pageView
        self.pageContainer.pack_start( pageView, False )
        
    def selectPage( self, selectedPageID ):
        if not self.pageViews[ selectedPageID ].selected:
            for pageID in self.pageViews.keys():
                self.pageViews[ pageID ].setSelected( pageID == selectedPageID )
            
            self.selectPageCallback( selectedPageID )