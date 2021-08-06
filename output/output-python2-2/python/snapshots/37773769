class CDiagramType:
    def __init__(self, id):
        self.icon = None
        self.id = id
        self.elements = []
        self.connections = []
        self.swimlines = False
        self.lifelines = False
    
    def AppendElement(self, value):
        self.elements.append( value )
        
    def AppendConnection(self, value):
        self.connections.append( value )
        
    def GetConnections(self):
        return self.connections
        
    def GetElements(self):
        return self.elements
    
    def GetIcon(self):
        return self.icon
    
    def GetId(self):
        return self.id
        
    def SetSpecial(self, swimlines, lifelines):
        self.swimlines = swimlines
        self.lifelines = lifelines
        
    def AllowSwimlines(self):
        return self.swimlines
        
    def AllowLifelines(self):
        return self.lifelines
    
    def SetIcon(self, pixbuf):
        self.icon = pixbuf
    
    def SetId(self, id):
        self.id = id
