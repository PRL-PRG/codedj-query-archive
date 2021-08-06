
from CodeContainer import CCodeContainer

class COptional(CCodeContainer):
    
    def __init__(self, default = ""):
        CCodeContainer.__init__(self)
        self.default = default
        
    
    def Generate(self, elementObject, path, fil = None):
        ret = [True, ""]
        for i in self.childs:
            genList = i.Generate(elementObject, path, fil)
            ret = self.JoinReturnValue(ret, genList)

        if ret[0]:
            return ret
        else:
            return [True, self.default]
            