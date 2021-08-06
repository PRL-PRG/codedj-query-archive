# Copyright (C) 2002 by Monty Taylor
# Translate ply files into LaTeX.

#     This program is free software; you can redistribute it and/or modify
#     it under the terms of the GNU General Public License as published by
#     the Free Software Foundation; either version 2 of the License, or
#     (at your option) any later version.

#     This program is distributed in the hope that it will be useful,
#     but WITHOUT ANY WARRANTY; without even the implied warranty of
#     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#     GNU General Public License for more details.

#     You should have received a copy of the GNU General Public License
#     along with this program; if not, write to the Free Software
#     Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

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
  
