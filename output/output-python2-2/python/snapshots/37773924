class CVersionType:
    def __init__(self, id):
       self.id = id 
       self.diagrams = {}
       self.diagramsList = []
    
    def AddRestrictions(self, diagName, diagRestrictions = []):
        if (diagName not in self.diagrams):
            self.diagramsList.append(diagName)
        self.diagrams[diagName] = (diagRestrictions)
        
    def GetRestrictions(self, diagName):
        for diagRestr in self.diagrams[diagName]:
            yield diagRestr
        
    def GetDiagrams(self):
        for diag in self.diagramsList:
            yield diag
            
    def GetAllRestrictions(self):
        for diagName in self.diagramsList:
            for diagRestr in self.diagrams[diagName]:
                yield diagRestr
            
    def GetId(self):
        return self.id
        
    def SetId(self, value):
        self.id = value