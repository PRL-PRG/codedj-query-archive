from CodeObject import CCodeObject

class CToken(CCodeObject):
    
    def __init__(self, value, default, type="text"):
        CCodeObject.__init__(self)
        self.value = value
        self.default = default
        self.type = type
        if type == "":
            self.type = 'text'
    
    
    
    def Generate(self, elementObject, path, fil = None):
        return [True, self.GetRoot().GetNewLineIndent() + self.default]