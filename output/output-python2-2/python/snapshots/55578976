#!/usr/bin/python

import pygtk
pygtk.require( '2.0' )
import gtk

from GUI.Core.MainWindow import MainWindow
from Framework.Constants import Constants
from Framework.CSound.CSoundServer import CsoundServerMult

#----------------------------------------------------------------------
# Startup an instance of TamTam
#----------------------------------------------------------------------
if __name__ == "__main__": 
    # TODO this should get started outside of TamTam (perhaps by Sugar?)
    # start the CSoundServer
    #server = CsoundServerMult( ( Constants.CSOUND_SERVER_ADDRESS, Constants.CSOUND_SERVER_PORT ) )
    #server.interpret()
    
    #create the main TamTam window
    tamTam = MainWindow()
    
    #start the gtk event loop
    gtk.main()