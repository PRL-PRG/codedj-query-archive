from Token import CToken

class CStringToken(CToken):
    def match(self, text, pos, indents = None):
        return CToken.match(self, text, pos) + (indents, )