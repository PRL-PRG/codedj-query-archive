
from CodeObject import CCodeObject

class CCodeContainer(CCodeObject):
    
    def __init__(self):
        CCodeObject.__init__(self)
        self.childs = []
        
    def __repr__(self, indent = 0):
        result = CCodeObject.__repr__(self, indent)
        #~ for child in self.childs:
            #~ result += child.__repr__(indent+1)
        return result
    
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
        
    def GetChildRules(self):
        for child in self.childs:
            if child.Parse():
                for x in child.GetRules():
                    yield x
        
    def GetRules(self):
        yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()]
        for rule in self.GetChildRules():
            yield rule

    def Walk(self):
        yield self
        for child in self.childs:
            for x in child.Walk():
                yield x