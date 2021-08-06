import re
from Object import CDomainObject
from lib.Exceptions import DomainParserError

class CDomainParser(object):
    '''
    Parser supporting various types of parsing methods.
    '''
    
    def __init__(self, separator=None, regexp=None):
        '''
        most of the parameters can be left out set to None
        
        @param separator: used to split by
        '''
        
        self.separator = separator
        self.regexp = (re.compile(regexp, re.X) if regexp else None)
    
    def CreateObject(self, text, domain):
        '''
        @return: Domain object of defined domain constructed from text using regexp
        @rtype: L{CDomainObject<CDomainObject>}
        
        @param text: text string to construct object from
        @type text: str
        
        @param domain: Domain of newly created object
        @type domain: L{CDomainType<Type.CDomainType>}
        '''
        obj = CDomainObject(domain)
        if self.regexp is not None:
            attempt = self.regexp.match(text)
            if attempt:
                for key, value in attempt.groupdict().iteritems():
                    if value is not None:
                        obj.SetValue(key, value)
                return obj
        
        return None
    
    def Split(self, text, maxsplit=None):
        '''
        Split text to the list of strings using separator preferably before
        regexp.
        
        @return: list of strings
        @rtype: list
        
        @param text: text to be separated
        @type text: str
        
        @param maxsplit: maximal number of splits, None for unlimited splits
        @type maxsplit: int, NoneType
        '''
        assert isinstance(text, (str, unicode))
        
        if self.separator:
            return text.split(self.separator, (maxsplit if maxsplit is not None else -1))
        elif self.regexp:
            return self.regexp.split(text, (maxsplit or 0))[::2]
        
        
    def __str__(self):
        result = ['<CDomainParser']
        if self.separator: 
            result.append(' separator="%s"'%(self.separator,))
        if self.regexp:
            result.append(' regexp')
        result.append('>')
        return ''.join(result)
    
    __repr__ = __str__
