from lib.lib import UMLException
from ProjectNode import CProjectNode

class CProjekt(object):
    def __init__(self, file = None):
        self.root = None
      
    def SetRoot(self, value):
        self.root = value
    
    def GetRoot(self):
        return self.root
    
    def GetNode(self, path):
        node = self.root
        
        k = path.split('/')[0]
        i,j = k.split(':')
                
        if i == self.root.GetName() and j == self.root.GetType():
            for i in path.split('/')[1:]:
                j, k  = i.split(':')
                if k == "=DrawingArea=":
                    return node
                else:
                    node = node.GetChild(j, k)
                if node is None:
                    raise UMLException("BadPath")
            return node
        raise UMLException("BadPath3")
    
    
    def Find(self, name):
        stack = [self.root]
        while len(stack) > 0:
            node = stack.pop(0)
            if node.GetName() == name:
                return node
            stack += node.GetChilds()
        return None

    def AddNode(self, node, parent):
        if parent is None:
            self.root = node
        else:
            parent.AddChild(node)
            

    def MoveNode(self, node, newParent):
        node.GetParent(node).RemoveChild(node)
        node.SetParent(newParent)
        newParent.AddChild(node)

    def RemoveNode(self, node):
        node.GetParent(node).RemoveChild(node)

    Root = property(GetRoot, SetRoot)