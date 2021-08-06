import pygtk
pygtk.require('2.0')
import gtk

from Framework.CSound.CSoundClient import CSoundClient

class MicRecordingWindow( gtk.Window ):
    def __init__( self, handleCloseWindowCallback ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.setupWindow( handleCloseWindowCallback )

    def setupWindow( self, handleCloseWindowCallback ):
        self.set_position(gtk.WIN_POS_CENTER_ON_PARENT)
        self.set_title("Mic Recording Window")
        self.set_border_width(10)
        self.bbox = gtk.VBox(False, 5)
        self.add(self.bbox)
        
        self.bufferOne = self.initButton( " buf 1 ", self.recBufferOne )
        self.bufferTwo = self.initButton( " buf 2 ", self.recBufferTwo )
        self.bufferThree = self.initButton( " buf 3 ", self.recBufferThree )
        self.bufferFour = self.initButton( " buf 4 ", self.recBufferFour )
        self.closeButton = self.initButton(" close ", handleCloseWindowCallback )
        
        self.connect( "delete_event", handleCloseWindowCallback )

    def recBufferOne( self, data=None ):
        CSoundClient.micRecording( 31 )

    def recBufferTwo( self, data=None ):
        CSoundClient.micRecording( 32 )

    def recBufferThree( self, data=None ):
        CSoundClient.micRecording( 33 )

    def recBufferFour( self, data=None ):
        CSoundClient.micRecording( 34 )

    def initButton(self, label, buttonFunction):
        button = gtk.Button(label)
        button.connect("clicked", buttonFunction)
        self.bbox.pack_start(button)
