
import os.path
from CodeObject import CCodeObject

class CText(CCodeObject):
    
    def __init__(self, text):
        CCodeObject.__init__(self)
        self.text = text
        
    def __repr__(self, indent = 0):
        return '%s<%s "%s">\n'%('  '*indent, self.__class__.__name__,  self.text)
    
    def Generate(self, element, path, file = None):
        return [True, self.GetRoot().GetNewLineIndent() + self.text]
        
    def GetSymbol(self):
        return self.text
            
        
    def GetTokens(self):
        terminal = self.text
        regexp = self.text
        symbols = '\\\'\"{}[]().^$*+?|'
        for sym in symbols:
            regexp = regexp.replace(sym, '\\'+sym)
        yield terminal, regexp, 'text'