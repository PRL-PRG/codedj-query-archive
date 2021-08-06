import sys
import gtk
import pango

class DebugAttribute(object):
    """
    Prints debuging information on each attribute access
    
    Usage:
        >>> import lib.debug
        >>> class test(object):
        ...     attr = lib.debug.DebugAttribute('attr')
        ...     
        ...     def __init__(self):
        ...         self.attr = 5
        ...             
        ...     def get(self):
        ...         return self.attr
        ...
        >>> a = test()
        ========================
        File: "<stdin>", line: 5
        Set test.attr to 5
        ========================
        >>> a.get()
        ========================
        File: "<stdin>", line: 8
        Get test.attr (5)
        ========================
        5
    """
    def __init__(self, name, val = None):
        """
        Initialize attribute debuging object
        
        @param name: name of attribute
        @type  name: string
        
        @param val: initial value of attribute
        @type  val: anything
        """
        self.defval = val
        self.val = {}
        self.name = name
    
    def __set__(self, instance, value):
        self.val[id(instance)] = value
        f = sys._getframe(1)
        print '========================'
        print 'File: "%s", line: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Set %s.%s to %r'%(instance.__class__.__name__, self.name, value)
        print '========================'
    
    def __get__(self, instance, owner):
        f = sys._getframe(1)
        print '========================'
        print 'File: "%s", line: %d'%(f.f_code.co_filename, f.f_lineno)
        print 'Get %s.%s (%r)'%(instance.__class__.__name__, self.name, self.val)
        print '========================'
        return self.val.get(id(instance), self.defval)
