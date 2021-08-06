class SyntaxError(Exception):
    """When we run into an unexpected token, this is the exception to use"""
    def __init__(self, pos=-1, msg="Bad Token"):
        Exception.__init__(self)
        self.pos = pos
        self.msg = msg
    def __repr__(self):
        if self.pos < 0:
            return "#<syntax-error>"
        else: 
            return "SyntaxError[@ char %s: %s]" % (repr(self.pos), self.msg)

class NoMoreTokens(Exception):
    """Another exception object, for when we run out of tokens"""
    pass

class CScanner:
    def __init__(self, patterns, ignore):
        """Patterns is [(terminal,regex)...]
        Ignore is [terminal,...];
        """
        self.tokens = []
        self.restrictions = []
        self.ignore = ignore
        self.patterns = patterns
        
    def token(self, i, restrict=0):
        """Get the i'th token, and if i is one past the end, then scan
        for another token; restrict is a list of tokens that
        are allowed, or 0 for any token."""
        if i == -1:
            i = len(self.tokens)
        if i == len(self.tokens): 
            self.scan(restrict)
        if i < len(self.tokens):
            # Make sure the restriction is more restricted
            if restrict and self.restrictions[i]:
                for r in restrict:
                    if r not in self.restrictions[i]:
                        raise NotImplementedError("Unimplemented: restriction set changed")
            return self.tokens[i]
        raise NoMoreTokens()

    def scan(self, restrict):
        pass
