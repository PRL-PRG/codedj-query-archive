
from CodeContainer import CCodeContainer

class COptional(CCodeContainer):
    
    def __init__(self, condition = None):
        CCodeContainer.__init__(self)
        self.condition = condition
        
    
    def Generate(self, elementObject, path, fil = None):
        ret = [True, ""]
        for i in self.childs:
            genList = i.Generate(elementObject, path, fil)
            ret = self.JoinReturnValue(ret, genList)

        if ret[0]:
            return ret
        else:
            return [True, ""]
            
    def GetSymbol(self):
        return 'opt-' + CCodeContainer.GetSymbol(self)
            
    def GetRules(self):
        yield self.GetSymbol(), []
        yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()]
        for rule in self.GetChildRules():
            yield rule
