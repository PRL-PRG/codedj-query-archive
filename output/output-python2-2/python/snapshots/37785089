from CodeContainer import CCodeContainer

class CLoop(CCodeContainer):
    def __init__(self, separator=''):
        CCodeContainer.__init__(self)
        self.separator = separator
        self.text = ""
        
    def Generate(self, elementObject, path, fil = None):
        ret = [False, ""]
        _sep = '';
 
        for i in self.childs:
            genList = i.Generate(elementObject, path, fil)
            if isinstance(i, CCodeContainer):
                ret = [ret[0] | genList[0], ret[1] + _sep + genList[1]]
            else:
                ret = [ret[0], ret[1] + _sep + genList[1]]
            _sep = self.separator
        
        return ret
        
    def GetSymbol(self):
        return 'loop-' + CCodeContainer.GetSymbol(self)
        
    def GetRules(self):
        yield self.GetSymbol(), []
        sep = self.separator.strip()
        if sep == '\\n':
            sep = 'br'
        if sep:
            sep_nt = 'sep:'+self.GetSymbol()
            yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()] + [sep_nt]
            yield sep_nt, [sep, self.GetSymbol()]
            yield sep_nt, []
        else:
            yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()] + [self.GetSymbol()]
        for rule in self.GetChildRules():
            yield rule
                    
    def GetTokens(self):
        terminal = self.separator.strip()
        if terminal and terminal != '\n':
            regexp = self.separator.strip()
            symbols = '\\\'\"{}[]().^$*+?|'
            for sym in symbols:
                regexp = regexp.replace(sym, '\\'+sym)
            yield terminal, regexp, 'text'
