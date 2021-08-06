
from CodeObject import CCodeObject

class Cbr(CCodeObject):
    
    def __init__(self, required = "False", count = 1):
        CCodeObject.__init__(self)
        self.required = required
        self.count = int(count)
        self.text = '\n'
    
    def Generate(self, element, path, fil = None):
        self.GetRoot().SetNewLine(True)
        return [True ,self.text*self.count]
