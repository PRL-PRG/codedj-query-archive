from lib.config import config
from lib.Drawing import CConnection

class CElement:
    def __init__(self, diagram, obj, isLoad = False):
        self.isLoad = isLoad
        self.objct = obj
        self.position = (0,0)
        self.deltaSize = (0,0)
        self.selected = False
        self.squares = []
        self.diagram = diagram
        self.diagram.AddElement(self)
        self.objct.AddAppears(diagram)
        self.__AddExistingConnections()
        self.ClearSizeCache()
        self.revision = 0
        self.cfgrevision = 0
    
    def ClearSizeCache(self):
        self.__sizecache = {}
    
    def CacheSize(self, obj, size):
        line = getattr(self, '__LOOPVARS__', {}).get('line')
        self.__sizecache[(id(obj), line)] = size
        return size
    
    def GetCachedSize(self, obj):
        if self.revision < self.objct.GetRevision() or self.cfgrevision < config.GetRevision():
            self.ClearSizeCache()
            self.revision = self.objct.GetRevision()
            self.cfgrevision = config.GetRevision()
            return None
        line = getattr(self, '__LOOPVARS__', {}).get('line')
        return self.__sizecache.get((id(obj), line))
    
    def __AddExistingConnections(self):
        if not self.isLoad:
            for i in self.objct.GetConnections():
                if i.GetSource() is not self.objct:
                    if self.diagram.HasElementObject(i.GetSource()) is not None:
                        CConnection(self.diagram,i,self.diagram.HasElementObject(i.GetSource()),self)
                elif i.GetDestination() is not self.objct:
                    if self.diagram.HasElementObject(i.GetDestination()) is not None:
                        CConnection(self.diagram,i,self,self.diagram.HasElementObject(i.GetDestination()))
                    
    def __AddSquare(self, index, x, y, posx, posy):
        size = config['/Styles/Selection/PointsSize']
        if posx == 0:
            x = x - size // 2
            x1 = x + size
        else:
            x1 = x + posx * size
        if posy == 0:
            y = y - size // 2
            y1 = y + size
        else:
            y1 = y + posy * size
        if x1 < x:
            x1, x = x, x1
        if y1 < y:
            y1, y = y, y1
        self.squares.append(((-posx, -posy), (x, y), (x1 - x, y1 - y)))

    def AreYouAtPosition(self, canvas, pos):
        x, y = pos
        width, height = self.GetSize(canvas)
        
        if  (self.position[0] <= x <= self.position[0] + width) and (self.position[1] <= y <= self.position[1] + height):
            return True
        else:
            return False
    
    def AreYouInRange(self, canvas, topleft, bottomright, all = False):
        (x1, y1), (x2, y2) = topleft, bottomright
        width, height = self.GetSize(canvas)
        
        if all:
            return (x1 <= self.position[0] <= self.position[0] + width <= x2) and (y1 <= self.position[1] <= self.position[1] + height <= y2)
        else:
            return ((x1 <= self.position[0] <= x2) and (y1 <= self.position[1] <= y2)) or \
                   ((x1 <= self.position[0] + width <= x2) and (y1 <= self.position[1] + height <= y2)) or \
                   ((self.position[0] <= x1 <= self.position[0] + width) and (self.position[1] <= y1 <= self.position[1] + height))
    
    def Select(self):
        self.selected = True
    
    def Deselect(self):
        self.selected = False
        self.squares = []
    
    def GetSelected(self):
        return self.selected
        
    def GetObject(self):
        return self.objct

    def GetPosition(self):
        return self.position
        
    def GetCenter(self, canvas):
        w, h = self.GetSize(canvas)
        return w / 2 + self.position[0], h / 2 + self.position[1]
    
    def GetSize(self, canvas):
        w, h = self.objct.GetSize(canvas, self)
        w, h = w + self.deltaSize[0], h + self.deltaSize[1]
        return w, h 
        
    def GetMinimalSize(self, canvas):
        w, h = self.objct.GetSize(canvas, self)
        return w, h
        
    def GetSquare(self, canvas):
        x, y = self.GetPosition()
        w, h = self.GetSize(canvas)
        return ((x, y), (x + w, y + h))

    def Paint(self, canvas, delta = (0, 0)):
        self.objct.Paint(canvas, self, delta)
        if self.selected:
            x, y = self.position
            w, h = self.GetSize(canvas)
            rx, ry = self.objct.GetType().GetResizable()
            
            self.squares = []
            
            if rx and ry:
                self.__AddSquare(0, x       , y       ,  1,  1)
                self.__AddSquare(2, x + w   , y       , -1,  1)
                self.__AddSquare(5, x       , y + h   ,  1, -1)
                self.__AddSquare(7, x + w   , y + h   , -1, -1)
            if ry:
                self.__AddSquare(1, x + w//2, y       ,  0,  1)
                self.__AddSquare(6, x + w//2, y + h   ,  0, -1)
            if rx:
                self.__AddSquare(3, x       , y + h//2,  1,  0)
                self.__AddSquare(4, x + w   , y + h//2, -1,  0)
            
            dx, dy = delta
            for i in self.squares:
                canvas.DrawRectangle((i[1][0] + dx, i[1][1] + dy), i[2], None, config['/Styles/Selection/PointsColor'])
            
            canvas.DrawRectangle((x + dx, y + dy), (w, h), fg = config['/Styles/Selection/RectangleColor'], line_width = config['/Styles/Selection/RectangleWidth'])

    def SetPosition(self, pos):
        self.position = pos
        
    def GetDiagram(self):
        return self.diagram
        
    def GetConnections(self):
        for c1 in self.GetObject().GetConnections(): #ConnectionObject
            for c2 in self.diagram.GetConnections(): # Connection
                if c2.GetObject() is c1:
                    yield c2

    
    # Vrati poziciu obvodoveho(resizing) stvorceka na pozicii
    def GetSquareAtPosition(self, pos):
        x, y = pos
        for sq in self.squares:
            sqbx = sq[1][0]
            sqby = sq[1][1]
            sqex = sqbx + sq[2][0]
            sqey = sqby + sq[2][1]
            if (x >= sqbx and x <= sqex and y >= sqby and y <= sqey):
                return sq[0]
    
    # Zmena velkosti(pripadne pozicie) elementu
    def Resize(self, canvas, delta, selSquareIdx):
        resRect = self.GetResizedRect(canvas, delta, selSquareIdx)
        minSize = self.GetMinimalSize(canvas)
        self.position = resRect[0]
        self.deltaSize = (max(0, resRect[1][0]-minSize[0]), max(0, resRect[1][1]-minSize[1]))
            
    def GetSizeRelative(self):
        return self.deltaSize
        
    def SetSizeRelative(self, relatSize):
        self.deltaSize = relatSize
    
    # Zistenie novej polohy a velkosti pri resizingu
        # delta = relativna zmena velkosti
        # selSquareIdx = index uchytavacieho-resizovacieho bodu
    def GetResizedRect(self, canvas, delta, mult):
        pos = list(self.GetPosition())
        size = list(self.GetSize(canvas))
        minsize = self.GetMinimalSize(canvas)
        
        for i in (0, 1):
            if mult[i] < 0:
                if delta[i] > size[i] - minsize[i]:
                    pos[i] += size[i] - minsize[i]
                    size[i] = minsize[i]
                else:
                    pos[i] += delta[i]
                    size[i] -= delta[i]
            else:
                size[i] = max(minsize[i], size[i] + mult[i] * delta[i])
                
        return pos, size
        
    def CopyFromElement(self, element):
        self.deltaSize = element.deltaSize
        self.position = element.position