class UMLException(Exception):
    def __init__(self, name, params = ()):
        self.name = name
        self.params = params
    
    def GetName(self):
        return self.name
    
    def GetParam(self, index):
        if isinstance(self.params, (list, tuple)):
            return self.params[index]
        else:
            if index == 0:
                return self.params
            else:
                return None
    
    def __str__(self):
        return self.name+" "+str(self.params)
    
    def __repr__(self):
        return self.__class__.__name__


def ToBool(val):
    if type(val) in (str, unicode):
        return val.lower() in ('1', 'yes', 'true')
    else:
        return val == True

def XMLEncode(val):
    ret = repr(val)
    if isinstance(val, str):
        ret = ret[1:-1]
    elif isinstance(val, unicode):
        ret = ret[2:-1]
    return ret.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('<', '&gt;').replace('"', '&quot;').encode('utf8')

class IDGenerator:
    def __init__(self):
        self.ids = {}
    
    def id(self, obj):
        return self.ids.setdefault(id(obj), len(self.ids))
    
    __call__ = id

