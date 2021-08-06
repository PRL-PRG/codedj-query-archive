class kk:
    i = 0
    @classmethod
    def get(self):
        self.i += 1
        return str(self.i)
    
    
class CCodeObject(object):
    def __init__(self):
        self.parent = None
        #~ self.symbol = str(uuid.uuid1())
        self.symbol = kk.get()
        self.indent = ""
        self.newLine = False
        self.recursive = []
        self.text = ["",""]
        
    def __repr__(self, indent = 0):
        return '%s<%s "%s">\n'%('  '*indent, self.__class__.__name__, self.GetSymbol())
    
    def JoinReturnValue(self, list1, list2):
        return [list1[0] & list2[0], list1[1] + list2[1]]
    
    def Generate(self, element, path, file = None):
        return [True, ""]
    
    def GetParent(self):
        return self.parent
    
    def SetParent(self, parent):
        self.parent = parent
       
    def ParseVariables(self, elementObject, *vals):
        for val in vals:
            if not isinstance(val, (str, unicode)):
                yield val
            elif val[0] == '#':
                yield elementObject.GetProperty(val[1:])
            elif val[0] == '@':
                yield elementObject.__LOOPVARS__[val[1:]]
            elif val[0] == '{':
                yield elementObject.GetProperty(val.split('}')[0][1:]) + val.split('}')[1]
            else:
                yield val
    
    def GetVariables(self, element, *names):
        return self.ParseVariables(element, *(getattr(self, name) for name in names))
    
    def GetRoot(self):
        if self.parent is None:
            return self
        parent = self.GetParent()
        while parent.GetParent() is not None:
            parent = parent.GetParent()
        return parent
    
    def SetIndent(self, text):
        self.indent = text
    
    def GetIndent(self):
        return self.indent
       
    def AddIndent(self, text):
        self.indent += text
    
    def RemoveIndent(self, text):
        self.indent = self.indent[:-len(text)]
        
    def GetNewLine(self):
        return self.newLine
    
    def SetNewLine(self, value):
        self.newLine = value
    
    def GetNewLineIndent(self):
        if self.newLine:
            self.newLine = False
            return self.indent
        return ""
    
    def AppendRecursive(self, item):
        self.recursive.append(item)
    
    def PopRecursive(self):
        return self.recursive.pop(len(self.recursive) - 1)
    
    def GetFirstRecursive(self):
        return self.recursive[0]
        
    def InRecursive(self):
        if len(self.recursive) == 0:
            return False
        return True
        
    
    def GetRules(self):
        if False:
            yield None
    
    def GetSymbol(self):
        return self.symbol
        
    def GetTokens(self):
        if False:
            yield None
            
    def Walk(self):
        yield self
        
    def Parse(self):
        return True
        
    def GetAction(self):
        if False:
            yield None
    
    def IsLoop(self):
        return False
        
    def CreateNew(self):
        return True