import re
from string import join

class CToken:
    def __init__(self, terminal, regexp = None):
        self.terminal = terminal
        if regexp is not None:
            self.regexp = re.compile(regexp)
        else:
            self.regexp = re.compile(terminal)
            
    def match(self, text, pos):
        m = self.regexp.match(text, pos)
        if m and len(m.group(0)) > 0:
            str = m.group(0)
            return str, len(str)
        return None, 0
        
    def GetTerminal(self):
        return self.terminal
        
    def __repr__(self):
        return self.terminal
