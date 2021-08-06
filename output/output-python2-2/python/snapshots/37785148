from CodeObject import CCodeObject
from CodeContainer import CCodeContainer

class CAllowElement(CCodeContainer):
    
    def __init__(self, id):
        CCodeContainer.__init__(self)
        self.id = id
    
    def Generate(self, elementObject, path, fil = None):
        ret = [True, ""]
        id = 0
        for id, i in enumerate(elementObject.GetNode().GetNodeSpecifyElements(None, self.id, False)):
            root = self.GetRoot()
            template = root.GetTemplate(i.GetObject().GetType().GetId())
            if  template is not None:
                id += 1
                ret = self.JoinReturnValue(ret, template.Generate(root.GetTemplates(), i.GetObject(), path))
        
        if id == 0:
            return [False, ""]
            
        return ret
        
    def GetSymbol(self):
        return ('element', self.id)
        
    