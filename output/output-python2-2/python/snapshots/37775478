from lib.config import config

class CCacheableObject(object):
    
    """Size cache of visible object"""
    
    def __init__(self):
        super(CCacheableObject, self).__init__()
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
        if self.revision < self.object.GetRevision() or self.cfgrevision < config.GetRevision():
            self.ClearSizeCache()
            self.revision = self.object.GetRevision()
            self.cfgrevision = config.GetRevision()
            return None
        line = getattr(self, '__LOOPVARS__', {}).get('line')
        return self.__sizecache.get((id(obj), line))
    
