
from CodeContainer import CCodeContainer
from Property import CProperty
from PropertyLoop import CPropertyLoop

class CConnectionLoop(CCodeContainer):
    
    def __init__(self, id, value, separator=""):
        CCodeContainer.__init__(self)
        self.collection = id
        self.separator = separator
        self.value = value
    
    def __GetItemFromParentLoop(self):
        parent = self.GetParent()
        
        while parent.GetParent() is not None:
            if isinstance(parent, CPropertyLoop):
                return parent.GetItem()
            parent = parent.GetParent()
        return None
    
    def Generate(self, elementObject, path, fil = None):
        ret = [True, ""]
        retFlag = False
        separatorFlag = False
        for id, item in enumerate(elementObject.GetConnections()):
            if item.GetProperty(self.collection) == self.value or self.value == "All":
                for i in self.childs:
                    elementObject.__LOOPVARS__ = item.GetProperty()
                    genList = i.Generate(elementObject, path, fil)
                    ret = self.JoinReturnValue(ret, genList)
                    if isinstance(i, CProperty) and genList[0]:
                        retFlag = True
                        separatorFlag = True
                    del elementObject.__LOOPVARS__
                if separatorFlag:
                    ret[1] += self.separator
                separatorFlag = False
        
        if ret[1].find(self.separator, -1) == -1:
            ret[1] = ret[1][:-len(self.separator)]
        
        
        #~ item = self.__GetItemFromParentLoop()
        #~ if item is not None:
            #~ elementObject.__LOOPVARS__ = item 
        
        ret[0] = retFlag
            
        return ret
        
    def GetRules(self):
        yield self.GetSymbol(), []
        sep = self.separator.strip()
        if sep == '\\n':
            sep = 'br'
        if sep:
            sep_nt = 'sep:'+CCodeContainer.GetSymbol(self)
            yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()] + [sep_nt]
            yield sep_nt, []
            yield sep_nt, [sep] + [self.GetSymbol()]
            #~ +[child.GetSymbol() for child in self.childs if child.Parse()] + [sep_nt]
        else:
            yield self.GetSymbol(), [child.GetSymbol() for child in self.childs if child.Parse()] + [self.GetSymbol()]
        for rule in self.GetChildRules():
            yield rule
        
    def GetTokens(self):
        if self.separator:
            terminal = self.separator.strip()
            regexp = self.separator.strip()
            symbols = '\\\'\"{}[]().^$*+?|'
            for sym in symbols:
                regexp = regexp.replace(sym, '\\'+sym)
            yield terminal, regexp, 'text' 
            
        
    def GetAction(self):
        yield self.GetSymbol(), self
    

    def GetSymbol(self):
        return 'connection', CCodeContainer.GetSymbol(self)
        
    def IsLoop(self):
        return True
