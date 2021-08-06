
class CGrid:
    def __init__(self, size, visible=False):
        self.size = size
        self.visible = visible
    
    def GetSize(self):
        return self.size
    
    def SetSize(self, size):
        self.size = size
    
    def GetVisible(self):
        return self.visible
    
    def SetVisible(self, visible):
        self.visible = visible
    
    def Paint(self, canvas, sizeDiagram):
        if self.visible:
            for i in range(0, sizeDiagram[0], self.size):
                for j in range(0, sizeDiagram[1], self.size):
                    canvas.DrawPoint(i,j)
