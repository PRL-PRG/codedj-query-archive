from StringToken import CStringToken

class CEofToken(CStringToken):
    def __init__(self):
        CStringToken.__init__(self, 'eof', '[\\n ]*')
        
    def match(self, text, pos, indents = None):
        m = self.regexp.match(text, pos)
        if m and len(m.group(0)) + pos == len(text) and not indents:
            return '', 0, indents
        else:
            return None, 0, indents
