from StringToken import CStringToken

class CEpsilonToken(CStringToken):
    def __init__(self):
        CStringToken.__init__(self, 'epsilon', '')
        
    def match(self, text, pos, indents = None):
        return '', 0, indents
