from CodeContainer import CCodeContainer

class CChangeElement(CCodeContainer):
    def __init__(self, id):
        CCodeContainer.__init__(self)
        self.id = id
        
    
    
    def Generate(self, element, path, fil = None):
        newElement, = self.GetVariables(element, 'id')
        ret = [True, ""]
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(newElement, path, fil))
        return ret