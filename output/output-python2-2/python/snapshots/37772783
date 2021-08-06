import os.path

from CodeContainer import CCodeContainer

class CFile(CCodeContainer):
    
    def __init__(self, name, value, prefix = "", sufix = ""):
        CCodeContainer.__init__(self)
        self.name = name
        self.value = value
        self.count = 0
        self.prefix = prefix
        self.sufix = sufix
          
    def Generate(self, element, path, fil = None):
        name, = self.GetVariables(element, 'name')
        ret = [True, ""]
        if self.count == 0:
            if not self.GetRoot().InRecursive():
                name = self.prefix + name + self.sufix
                fil = file(os.path.join(path, name),"w")
                
        self.count += 1
        for i in self.childs:
            ret = self.JoinReturnValue(ret, i.Generate(element, path, fil))
        self.count -= 1
        
        if self.count == 0:
            if not self.GetRoot().InRecursive():
                fil.write(ret[1])
                if self.GetRoot().text[0] == fil.name:
                    fil.write(self.GetRoot().text[1])
                    self.GetRoot().text = ["",""]
                fil.close()
            else:
                name, = self.GetVariables(self.GetRoot().GetFirstRecursive()[1], 'name')
                name = self.prefix + name + self.sufix
                self.GetRoot().text[0] = os.path.join(path, name)
                self.GetRoot().text[1] += ret[1]
            return [True, ""]

        return ret