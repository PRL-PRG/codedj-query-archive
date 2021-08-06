from CodeObject import CCodeObject

class CToken(CCodeObject):
    
    def __init__(self, value, default):
        CCodeObject.__init__(self)
        self.value = value
        self.default = default
    
    
    def Generate(self, elementObject, path, fil = None):
        return [True, self.GetRoot().GetNewLineIndent() + self.default]