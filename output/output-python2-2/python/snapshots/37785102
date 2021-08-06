from StringToken import CStringToken
from string import join

class CIndentToken(CStringToken):
    def __init__(self):
        CStringToken.__init__(self, 'indent', '\\n[\\s\\n]*')
    
    def match(self, text, pos, indents = None):
        m = self.regexp.match(text, pos)
        if indents is not None and m and len(m.group(0)) > 0 and len(m.group(0)) + pos < len(text):
            str = m.group(0)
            lastspaces = str.split('\n')[-1]
            if len(lastspaces) > len(join(indents, '')):
                inde = list(indents)
                inde.append(' ' * (len(lastspaces) - len(join(indents, ''))))
                return '\n'+lastspaces, len(str), inde
        return None, 0, indents
        
