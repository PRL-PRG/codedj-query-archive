import sys

class DebugAttribute(object):
    def __init__(self, name, val = None):
        self.val = val
        self.name = name
    
    def __set__(self, instance, value):
        self.val = value
        f = sys._getframe(1)
        print '========================'
        print 'Subor: "%s", riadok: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Set %s.%s to %r'%(instance.__class__.__name__, self.name, value)
        print '========================'
    
    def __get__(self, instance, owner):
        f = sys._getframe(1)
        print '========================'
        print 'Subor: "%s", riadok: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Get %s.%s (%r)'%(instance.__class__.__name__, self.name, self.val)
        print '========================'
        return self.val
