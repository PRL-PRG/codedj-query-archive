#!/usr/bin/python2.4

import pygtk
pygtk.require( '2.0' )
import gtk

from Framework.Constants import Constants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundServer import CsoundServerMult
from Framework.CSound.CSoundConstants import CSoundConstants

#----------------------------------------------------------------------
# Startup an instance of TamTam!
#----------------------------------------------------------------------
if __name__ == "__main__": 
    # TODO this should get started outside of TamTam (perhaps by Sugar?)
    # start the CSoundServer
    server = CsoundServerMult( ( CSoundConstants.SERVER_ADDRESS, CSoundConstants.SERVER_PORT ) )
    server.interpret()
    
