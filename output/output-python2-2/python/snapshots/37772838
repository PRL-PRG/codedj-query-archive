
from lib.lib import UMLException

class CDataType:
    
    def __init__(self, id):
        self.id = id
        self.dataTypes = []
    
    def AddDataType(self, type):
        self.dataTypes.append(type)
    
    def GetDataTypes(self):
        for i in self.dataTypes:
            yield i
    
    def UpdateDataType(self, oldType, newType, elementType):
        if elementType == "Class":
            for i in self.dataTypes:
                if i == oldType:
                    self.dataTypes.remove(oldType)
                    self.dataTypes.append(newType)
                    break
            else:
                raise UMLException("Bad data type")
    
    def DeleteDataType(self, type):
        self.dataTypes.remove(type)
    
    def ClearDataType(self):
        self.dataTypes = []
        