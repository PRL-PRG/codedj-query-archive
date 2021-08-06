from lib.lib import UMLException

class CProjectNode(object):
    def __init__(self, parent = None, object = None, path = None):
        self.parent = parent
        self.childs = []
        self.drawingareas = []
        self.object = object
        if path is not None:
            self.object.SetPath(path)
        object.Assign(self)

    def Change(self):
        if self.parent is not None:
            parentPath = self.parent.GetPath()+ "/"
        else:
            parentPath = ""


        self.SetPath(parentPath + self.GetName() + ":" + self.GetType())
        for i in self.drawingareas:
            i.SetPath(self.GetPath() + "/" +  i.GetName() + ":=DrawingArea=")
            
        for i in self.childs:
            i.Change()


    def GetAppears(self):
        return self.GetObject().GetAppears()

    def AddAppears(self, drawingArea):
        self.GetObject().AddAppears(drawingArea)

    def RemoveAppears(self, drawingArea):
        self.GetObject().RemoveAppears(drawingArea)

    def GetDrawingAreas(self):
        return self.drawingareas

    def HasDrawingArea(self):
        return len(self.drawingareas) > 0

    def GetPath(self):
        return self.object.GetPath()

    def SetPath(self, path):
        self.object.SetPath(path)

    def GetObject(self):
        return self.object

    def GetName(self):
        return self.object.GetName()

    def GetType(self):
        return self.object.GetType().GetId()

    def AddChild(self, child):
        if child not in self.childs:
            self.childs.append(child)
            child.parent = self
        else:
            raise UMLException("ExistsChild")


    def AddDrawingArea(self, area):
        if area not in self.drawingareas:
            area.Assign(self) # vygenerovanie nazvu
            self.drawingareas.append(area)
    
    def MoveDrawingAreaToNewNode(self, newNode, area):
        self.RemoveDrawingArea(area)
        #newNode.AddDrawingArea(area)
        newNode.drawingareas.append(area)
    
    def MoveNode(self, parentNode):
        self.parent.RemoveChild(self)
        self.parent = parentNode
        parentNode.AddChild(self)
        self.SetPath(parentNode.GetPath() + "/" + self.GetPath().split('/')[-1])
    
    def FindDrawingArea(self, name):
        for i in self.drawingareas:
            if i.GetName() == name:
                return i
        return None


    def GetChild(self, name, type):
        for i in self.childs:
            if i.GetName() == name and i.GetType() == type:
                return i
        else:
            return None

    def GetIndexChild(self, index):
        if index <= len(self.childs) - 1:
            return self.childs[index]
        else:
            raise UMLException("NodeNotExists")

    def GetChilds(self):
        for i in self.childs:
            yield i

    def HasChild(self):
        return len(self.childs) > 0

    def GetParent(self):
        return self.parent

    def RemoveChild(self, child):
        if child in self.childs:
            self.childs.remove(child)
        else:
            raise UMLException("ChildNotExists")

    def RemoveDrawingArea(self, area):
        if area in self.drawingareas:
            self.drawingareas.remove(area)
        else:
            raise UMLException("AreaNotExists")

    def SetParent(self, parent):
        self.parent = parent

    Parent = property(GetParent,SetParent)