from StringToken import CStringToken
from string import join

class CBrToken(CStringToken):
    def __init__(self, indents = False):
        self.indents = indents
        CStringToken.__init__(self, 'br', '\\n[\\s\\n]*')
        
    def match(self, text, pos, indents = None):
        m = self.regexp.match(text, pos)
        if m and len(m.group(0)) > 0:
            str = m.group(0)
            if self.indents is None:
                return str, len(str), None
            if len(m.group(0)) + pos == len(text):
                return None, 0, indents
            lastspaces = str.split('\n')[-1]
            if len(lastspaces) == len(join(indents, '')):
                return str, len(str), indents
        return None, 0, indents
        
