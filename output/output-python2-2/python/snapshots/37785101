from StringToken import CStringToken
from string import join

class CDedentToken(CStringToken):
    def __init__(self):
        CStringToken.__init__(self, 'dedent', '\\n[\\s\\n]*')
        
    def match(self, text, pos, indents = None):
        m = self.regexp.match(text, pos)
        if indents is not None and m and len(m.group(0)) > 0:
            str = m.group(0)
            lastspaces = str.split('\n')[-1]
            if len(lastspaces) < len(join(indents, '')):
                if len(lastspaces) == len(join(indents[0:-1], '')):
                    newpos = len(str)
                else:
                    newpos = 0
                return '\n'+join(indents[0:-1], ''), newpos, indents[0:-1]
        return None, 0, indents
