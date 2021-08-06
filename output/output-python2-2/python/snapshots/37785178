import os
import os.path
from Documentation import CDocumentation

class CGenerator:
    
    def __init__(self, type, path = None):
        self.type = type
        self.path = path
    
    def GetPath(self):
        return self.path
    
    def SetPath(self, path):
        self.path = path
    
    def GenerateElement(self, elementObj):
        template = self.type.GetElement(elementObj.GetType().GetId())
        template.Generate(self.type.GetElements(), elementObj, self.path)
    
    def GenerateDocumentation(self, name, project):
        element = CDocumentation(name, project, project.GetRoot())
        template = self.type.GetElement("documentation")
        template.Generate(template, element, self.path)
        del element