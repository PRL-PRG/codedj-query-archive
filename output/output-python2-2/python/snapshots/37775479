from lib.config import config
from lib.Drawing import CConnection
from CacheableObject import CCacheableObject
from SelectableObject import CSelectableObject

class CVisibleObject(CCacheableObject, CSelectableObject):
    '''
    Ancestor for CElement and CConLabel    
    '''
    
    def __init__(self):
        '''
        Common initialization of the visible object
        '''
        self.position = (0,0)
        self.deltaSize = (0,0)
        super(CVisibleObject,self).__init__()

    def AreYouAtPosition(self, canvas, pos):
        x, y = pos
        width, height = self.GetSize(canvas)
        return self.position[0] <= x <= self.position[0] + width and \
            self.position[1] <= y <= self.position[1] + height
    
    def AreYouInRange(self, canvas, topleft, bottomright, all = False):
        (x1, y1), (x2, y2) = topleft, bottomright
        width, height = self.GetSize(canvas)
        
        if all:
            return (x1 <= self.position[0] <= self.position[0] + width <= x2) and (y1 <= self.position[1] <= self.position[1] + height <= y2)
        else:
            return ((x1 <= self.position[0] <= x2) and (y1 <= self.position[1] <= y2)) or \
                   ((x1 <= self.position[0] + width <= x2) and (y1 <= self.position[1] + height <= y2)) or \
                   ((self.position[0] <= x1 <= self.position[0] + width) and (self.position[1] <= y1 <= self.position[1] + height))
    
    def GetObject(self):
        return self.object
    
    def GetPosition(self):
        return self.position
        
    def GetCenter(self, canvas):
        w, h = self.GetSize(canvas)
        return w / 2 + self.position[0], h / 2 + self.position[1]
    
    def GetSize(self, canvas):
        w, h = self.object.GetSize(canvas, self)
        w, h = w + self.deltaSize[0], h + self.deltaSize[1]
        return w, h 
        
    def GetMinimalSize(self, canvas):
        w, h = self.object.GetSize(canvas, self)
        return w, h
        
    def GetSquare(self, canvas):
        x, y = self.GetPosition()
        w, h = self.GetSize(canvas)
        return ((x, y), (x + w, y + h))
    
    def SetPosition(self, pos):
        self.position = pos
        
    def GetDiagram(self):
        return self.diagram
        

