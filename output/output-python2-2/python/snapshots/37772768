
from CodeContainer import CCodeContainer

class CRecursive(CCodeContainer):
    
    def __init__(self):
        CCodeContainer.__init__(self)
        
    def Generate(self, element, path, fil = None):
        ret = [True, ""]
        for i in self.childs:
            self.GetRoot().AppendRecursive((i,element))
            ret = self.JoinReturnValue(ret, i.Generate(element, path, fil))
            self.GetRoot().PopRecursive()            
        return ret