
class CLanguageType:
    
    def __init__(self, language, indents = False):
        self.language = language
        self.elements = {}
        self.tokens = {}
        self.rules = {}
        self.indents = False
    
    def __repr__(self):
        result = 'Language: %s (Indents: %s)\n'%(self.language, `self.indents`)
        result += 'Tokens:'
        for id, value in self.tokens.items():
            result += '"\n  \'%s\': "%s'%(id, value)
        result += '\n\nRules:'
        for id, value in self.GetRules():
            result += '\n\'%s\':%s'%(id, repr(value))
        result += '\nElements:'
        for id, value in self.elements.items():
            result += '\n\'%s\':\n%s'%(id, repr(value))
        return result
        
    
    def GetLanguage(self):
        return self.language
    
    def SetLanguage(self, language):
        self.language = language
        
    def GetIndents(self):
        return self.indents
        
    def SetIndents(self, value):
        self.indents = value
        
    def AddElement(self, id, element):
        self.elements[id] = element
    
    def GetElement(self, id):
        return self.elements[id]
    
    def GetElements(self):
        return self.elements
        
    def AddToken(self, id, value, type):
        if type == "":
            type = 'text'
        self.tokens[id] = (value, type)
        
    def GetTokens(self):
        for id, (value, type) in self.tokens.items():
            yield id, value, type

    def AddRule(self, id, rule):
        self.rules[id] = rule
        
    def GetRules(self):
        yield '', ['eod']
        for id,elem in self.GetElements().items():
            for rule in elem.GetRules():
                yield rule
        for id, rule in self.rules.items():
            for rule in rule.GetRules():
                yield rule

    def GetWalk(self):
        for id,elem in self.GetElements().items():
            for walk in elem.Walk():
                yield walk
