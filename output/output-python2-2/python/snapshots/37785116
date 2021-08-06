
from CodeObject import CCodeObject

class CWhitespace(CCodeObject):
    
    def __init__(self, required = "False", count = 1):
        CCodeObject.__init__(self)
        self.text = ' '
        self.required = required
        self.count = int(count)
        self.symbol = 'whitespace'
    
    def Generate(self, element, path, file = None):
        return [True, self.text * self.count]
        
    def Parse(self):
        return False