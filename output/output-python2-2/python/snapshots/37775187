from lib.config import config

class CVisualObject:
    def __init__(self):
        self.parent = None
    
    def __GetAttrs(self, value, names):
        for name in names:
            value = getattr(value, name)
        return value
    
    def ParseVariables(self, context, *vals):
        for val in vals:
            if not isinstance(val, (str, unicode)):
                yield val
            elif val[0] == '#':
                names = val[1:].split('.')
                yield self.__GetAttrs(context.GetAttribute(names[0]), names[1:])
            elif val[0] == '@':
                names = val[1:].split('.')
                yield self.__GetAttrs(context['item'][names[0]], names[1:])
            elif val[0] == '/':
                yield config[val]
            else:
                yield val
    
    def GetVariables(self, context, *names):
        return self.ParseVariables(context, *(getattr(self, name) for name in names))
    
    def GetResizable(self):
        return False, False
    
    def ComputeSize(self, context):
        return 0, 0
    
    def GetSize(self, context):
        size = context.GetCachedSize(self)
        if size is not None:
            return size
        size = self.ComputeSize(context)
        return context.CacheSize(self, size)

    def GetParent(self):
        return self.parent

    def Paint(self, context):
        pass

    def SetParent(self, parent):
        self.parent = parent
