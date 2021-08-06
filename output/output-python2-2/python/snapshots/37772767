
from CodeContainer import CCodeContainer
from lib.lib import ToBool

class CCodeCondition(CCodeContainer):
    
    def __init__(self, id, value, negate = 0):
        CCodeContainer.__init__(self)
        self.id = id
        self.value = value
        self.negate = ToBool(negate)
    
    def __IsTrue(self, elementObject):
        ret = False
        if self.id == 'Recursive':
            if self.GetRoot().InRecursive() is ToBool(self.value):
                return True
            return False
            
        condition, = self.GetVariables(elementObject, 'id')
        list = self.value.split("|")
        for i in list:
            if str(condition) == i.strip(' '):
                ret =  True
        
        if self.negate:
            ret = not ret
        
        return ret
    
    def Generate(self, element, path, fil = None):
        ret = [True, ""]
        retFlag = False
        if self.__IsTrue(element):
            for i in self.childs:
                genList = i.Generate(element, path, fil)
                ret = self.JoinReturnValue(ret, genList)
                if isinstance(i, CCodeContainer) and genList[0]:
                    retFlag = True
        else:
            return [False, ""]
        
        if retFlag:
            ret[0] = True
        
        return ret