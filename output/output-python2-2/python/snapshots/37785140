from CodeObject import CCodeObject

class CEpsilon(CCodeObject):
    
    def __init__(self):
        CCodeObject.__init__(self)
        
    def __repr__(self, indent = 0):
        return '%s<%s "%s">\n'%('  '*indent, self.__class__.__name__,  'Epsilon')
    
    def Generate(self, element, path, file = None):
        return [True, self.GetRoot().GetNewLineIndent()]
        
    def GetSymbol(self):
        return 'epsilon'
            
        
    def GetTokens(self):
        yield 'epsilon', '', ''