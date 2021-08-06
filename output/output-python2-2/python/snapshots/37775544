from lib.Exceptions.UserException import *
from Container import CContainer

class CSimpleContainer(CContainer):
    def AppendChild(self, child):
        if len(self.GetChilds()) > 0:
            raise DrawingError("SCChildCount")
        CContainer.AppendChild(self, child)

    def SetChild(self, child):
        if len(self.GetChilds()) > 0:
            self.RemoveChild(self.GetChild())
        self.AppendChild(child)
    
    def GetChild(self):
        return CContainer.GetChild(self, 0)
