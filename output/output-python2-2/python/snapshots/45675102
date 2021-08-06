#!/usr/bin/python2.2
#
# Translate ply files into LaTeX.

import sys

from wxPython.wx import *
from plywood import plywood

class WxPly(wxApp):
    def OnInit(self):
      top = wxFrame(NULL, -1, "In August Play Typesetting System")

      frame = wxFileDialog(top, "Select input file", '', '', '', wxOPEN, (0,0))
      frame.ShowModal()
      filename = frame.GetPath()
      ply=plywood(filename)
      ply.process()
      ply.close()
      ply.makedvi()
      ply.makepdf()
      sys.exit(0)

    
if __name__=="__main__":
    app=WxPly(0)
    app.MainLoop()
  
