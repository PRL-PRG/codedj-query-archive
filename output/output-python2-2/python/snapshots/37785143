from Scanner import CScanner, NoMoreTokens, SyntaxError
from lib.ReverseEngineering.Tokens import *
from string import join

class CTextScanner(CScanner):
    def __init__(self, patterns, ignore, input):
        CScanner.__init__(self, patterns, ignore)
        self.input = input
        self.pos = 0
        self.indents = []
                
    def scan(self, restrict):
        while 1:
            best_pat = None
            best_match = -1
            for token in self.patterns:
                p = token.terminal
                if not isinstance(token, CStringToken):
                    continue
                if restrict and p not in restrict and p not in self.ignore:
                    continue
                _str, _len, _ind = token.match(self.input, self.pos, self.indents)
                if _str is not None and len(_str) > best_match:
                    best_pat = (p, _str, _len, _ind)
                    best_match = len(_str)
                    
            # If we didn't find anything, raise an error
            if best_pat is None and best_match < 0:
                msg = "Bad Token"
                if restrict:
                    msg = "Trying to find one of "+join(restrict,", ")
                raise SyntaxError(self.pos, msg)
            # If we found something that isn't to be ignored, return it
            if best_pat[0] not in self.ignore:
                # Create a token with this data
                token = (best_pat[0], best_pat[1], self.pos)
                self.pos = self.pos + best_pat[2]
                self.indents = best_pat[3]
                
                # Only add this token if it's not in the list
                # (to prevent looping)
                if token[0] == 'epsilon' or not self.tokens or token != self.tokens[-1]:
                    self.tokens.append(token)
                    self.restrictions.append(restrict)
                return
            else:
                # This token should be ignored ..
                self.pos = self.pos + best_pat[2]
