class UMLException(Exception):
    def __init__(self, name, params = ()):
        self.name = name
        self.params = params
    
    def __str__(self):
        return self.name+" "+str(self.params)
    
    def __repr__(self):
        return self.__class__.__name__

def ToBool(val):
    if type(val) in (str, unicode):
        return val.lower() in ('1', 'yes', 'true')
    else:
        return val is True
