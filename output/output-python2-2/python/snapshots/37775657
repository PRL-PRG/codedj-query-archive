class CVersionType:
    def __init__(self, id):
       self.id = id 
       self.diagramsList = []

    def AddDiagram(self, diagName):
        if (diagName not in self.diagramsList):
            self.diagramsList.append(diagName)
        
    def GetDiagrams(self):
        for diag in self.diagramsList:
            yield diag
            
    def GetId(self):
        return self.id
        
    def SetId(self, value):
        self.id = value