
from CodeObject import CCodeObject

class CCodeContainer(CCodeObject):
    
    def __init__(self):
        CCodeObject.__init__(self)
        self.childs = []
    
    def AppendChild(self, child):
        self.childs.append(child)
        child.SetParent(self)

    def GetChild(self, index):
        return self.childs[index]

    def GetChilds(self):
        return self.childs
    
    def Generate(self, element, path, file = None):
        ret = [True, ""]
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(element, path))
        
        return ret