
from CodeContainer import CCodeContainer

class CIndent(CCodeContainer):
    
    def __init__(self, required = "False", defsize = 4):
        CCodeContainer.__init__(self)
        self.required = required
        self.defsize = int(defsize)
        self.text = ' '
    
    def GetText(self):
        return self.text * self.defsize
        
    def Generate(self, element, path, fil = None):
        root = self.GetRoot()
        if self.defsize > 0:
            root.AddIndent(self.GetText())
        else:
            ind = root.GetIndent()
            root.SetIndent("")
        ret = [True, ""]
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(element, path, fil))
        if self.defsize > 0:
            root.RemoveIndent(self.GetText())
        else:
            root.SetIndent(ind)
        
        return ret
