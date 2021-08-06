
from CodeContainer import CCodeContainer
from Indent import CIndent

class CCodeElement(CCodeContainer):
    
    def __init__(self, id):
        CCodeContainer.__init__(self)
        self.id = id


    def Generate(self, templates, element, path, file = None):
        self.templates = templates
        ret = [True, ""]
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(element, path, file))
        return ret

    def GetTemplate(self, element):
        if self.templates.has_key(element):
            return self.templates[element]
        return None 
    
    def GetTemplates(self):
        return self.templates
        
    def GetType(self):
        return self.id
        
    def GetSymbol(self):
        return ('element', self.id)
        
    def GetRules(self):
        yield '', [self.GetSymbol(), '']
        yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()]
        for rule in self.GetChildRules():
            yield rule
        
    def GetAction(self):
        yield  self.GetSymbol(), self