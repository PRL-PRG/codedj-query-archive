class CAlternateRule(object):

    def GetFirst(self, caller = None):
        if caller is None:
            caller = self
        else:
            if caller is self:
                raise 'circullar stuff'
        for child in self.childs:
            for first in child.GetFirst(caller):
                yield first