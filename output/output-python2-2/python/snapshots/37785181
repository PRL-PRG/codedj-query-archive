class COptionalRule:
    
    def GetFirst(self, caller):
        if caller is None:
            caller = self
        else:
            if caller is self:
                raise 'circullar stuff'
        if len(self.childs) > 0
            for first in self.childs[0].GetFirst(caller):
                yield first
        for next in self.parent.GetNext(self):
            yield next