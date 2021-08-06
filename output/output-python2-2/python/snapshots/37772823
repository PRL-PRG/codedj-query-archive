
from CodeContainer import CCodeContainer
from Property import CProperty
from CodeCondition import CCodeCondition

class CPropertyLoop(CCodeContainer):
    
    def __init__(self, id, separator = "", parse = None):
        CCodeContainer.__init__(self)
        self.collection = id
        self.separator = separator
        self.parse = parse
        self.item = None
    
    def GetItem(self):
        return self.item
    
    def __GetItemFromParentLoop(self):
        parent = self.GetParent()
        
        while parent.GetParent() is not None:
            if isinstance(parent, CPropertyLoop):
                return parent.GetItem()
            parent = parent.GetParent()
        return None
    
    def Generate(self, elementObject, path, fil = None):        
        root = self.GetRoot()
        ret = [True, ""]
        retFlag = False
        separatorFlag = False
        if self.parse is not None:
            if self.collection == "@params":
                txtParams, = self.GetVariables(elementObject, 'collection')
                params = elementObject.ParseParams(txtParams)
                if params is None:
                    return [False, ""]
                for id in xrange(len(params.values()[0])):
                    part = {}
                    for k,v in params.items():
                        part[k] = v[id]
                    for ch in self.childs:
                        elementObject.__LOOPVARS__ = part
                        genList = ch.Generate(elementObject, path, fil)
                        ret = self.JoinReturnValue(ret, genList)
                        if isinstance(ch, (CCodeContainer)) and genList[0]:
                            retFlag = True
                    if id < len(params.values()[0]) - 1:
                        ret[1] += self.separator
        else:
            if elementObject.GetProperty(self.collection) is None or len(elementObject.GetProperty(self.collection)) == 0:
                return [False,""]
            for item in elementObject.GetProperty(self.collection):
                self.item = item
                for i in self.childs:
                    elementObject.__LOOPVARS__ = item
                    genList = i.Generate(elementObject, path, fil)
                    ret = self.JoinReturnValue(ret, genList)
                    if isinstance(i, (CCodeContainer)) and genList[0]:
                        retFlag = True
                        separatorFlag = True
                    #~ del elementObject.__LOOPVARS__
                if separatorFlag:
                    ret[1] += '\n'
                    root.SetNewLine(True)
                    separatorFlag = False
        
        item = self.__GetItemFromParentLoop()
        if item is not None:
            elementObject.__LOOPVARS__ = item 
        
        if retFlag:
            ret[0] = True
            
        return ret