class a1(object):
    
    def q(self):
        print 'z'
        
class a2(object):
    def q(self):
        print 'zzz'

class b1(a1):
    pass
    
class b2(a2, b1):
    
    def __init__(self):
        a2.__init__(self)
        b1.__init__(self)
        
    
    
q = b2()
q.q()
    