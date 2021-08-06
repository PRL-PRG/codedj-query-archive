
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