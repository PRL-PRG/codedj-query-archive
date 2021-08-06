
import os.path
from CodeObject import CCodeObject

class CText(CCodeObject):
    
    def __init__(self, text):
        CCodeObject.__init__(self)
        self.text = text
    
    def Generate(self, element, path, file = None):
        return [True, self.GetRoot().GetNewLineIndent() + self.text]
            