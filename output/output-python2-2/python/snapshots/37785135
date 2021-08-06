class CFlowRule(object):

    def GetFirst(self, caller = None):
        if caller is None:
            caller = self
        else:
            if caller is self:
                raise 'circullar stuff'
        if len(self.childs) > 0
            for first in self.childs[0].GetFirst(caller):
                yield first