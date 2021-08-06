

class CDocumentation:
    
    def __init__(self, name, project, rootNode):
        self.name = name
        self.project = project
        self.rootNode = rootNode
    
    def GetProperty(self, key):
        out = {}
        out['name'] = self.name
        val = None
        
        if key == 'treeView':
            v = []
            o = {}
            o['name'] = self.rootNode.GetName()
            o['icon'] = self.rootNode.GetObject().GetType().GetIcon()
            o['indent']  = "0"
            o['path'] = self.rootNode.GetPath().replace('/','-').replace(':','+')
            o['isdiagram'] = False
            o['element'] = self.rootNode.GetObject()
            v.append(o)
            def Rekurzia(root, indent):
                for i in root.GetDrawingAreas():
                    o = {}
                    o['name'] = i.GetName()
                    o['icon'] = i.GetType().GetIcon()
                    o['indent'] = str(indent * 20)
                    o['path'] = i.GetPath().replace('/','-').replace(':','+')
                    o['isdiagram'] = True
                    o['element'] = i
                    v.append(o)
                
                for i in root.GetChilds():
                    o = {}
                    o['name'] = i.GetName()
                    o['icon'] = i.GetObject().GetType().GetIcon()
                    o['indent'] = str(indent * 20)
                    o['path'] = i.GetPath().replace('/','-').replace(':','+')
                    o['isdiagram'] = False
                    o['element'] = i.GetObject()
                    v.append(o)
                    Rekurzia(i, indent + 1)
            Rekurzia(self.rootNode, 1)
            val = v
        
        if val is None:
            val = out[key]
        return val
