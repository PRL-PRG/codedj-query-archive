from Container import CContainer
from lib.lib import UMLException

class CSimpleContainer(CContainer):
    def AppendChild(self, child):
        if len(self.GetChilds()) > 0:
            raise UMLException("SCChildCount")
        CContainer.AppendChild(self, child)

    def SetChild(self, child):
        if len(self.GetChilds()) > 0:
            self.RemoveChild(self.GetChild(0))
        self.AppendChild(child)
