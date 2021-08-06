
from CodeContainer import CCodeContainer

class CBlock(CCodeContainer):
    def __init__(self):
        CCodeContainer.__init__(self)
        self.text = ""
        
    def Generate(self, elementObject, path, fil = None):
        ret = [True,""]
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(elementObject, path, fil))
              
        self.text += ret[1]
        if self.GetRoot().InRecursive():
            return [ret[0],""]
        else:
            ret[1] = self.text
            self.text = ""
            return ret
        