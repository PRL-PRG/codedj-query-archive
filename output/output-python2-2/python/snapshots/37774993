import re
from Object import CDomainObject
from lib.Exceptions import DomainTypeError

class CDomainType(object):
    
    ATOMIC = 'int', 'float', 'str', 'text', 'bool', 'enum', 'list'
    
    def __init__(self, name, factory):
        '''
        @param name: Domain identifier
        @type name: str

        @param factory: domain factory that already loaded all the domains
        @type factory: L{CDomainFactory<Factory.CDomainFactory>}
        '''
        self.name = name
        self.imports = []
        self.attributes = {}
        self.factory = factory
        self.parsers = []
        self.attributeorder = []
    
    def GetFactory(self):
        '''
        @retrun: Current domain factory
        @rtype: L{CDomainFactory<Factory.CDomainFactory>}
        '''
        return self.factory
    
    def AppendAttribute(self, id, name, type):
        '''
        Add attribute the domain
        
        @param id: identifier of attribute.
        @type id: str
        
        @param name: Name of attribute. Displayed to user
        @type name: str
        
        @param type: id of domain. Must be atomic domain or one of imported
        @type type: str
        
        @raise DomainTypeError: if type is not atomic or one of imported domains
        '''
        
        if not type in self.ATOMIC and not type in self.imports:
            raise DomainTypeError('Used type %s is not imported'%(type, ))
        
        self.attributes[id] = {'name': name, 'type':type}
        self.attributeorder.append(id)
    
    def HasAttribute(self, id):
        '''
        @return: True if id is defined attribute
        @rtype: bool
        '''
        return id in self.attributes

    def AppendImport(self, id):
        '''
        Add name of domain that current uses
        
        Only imported domains are allowed to be used
        
        @param id: identifier of imported domain
        @type id: str
        '''
        assert isinstance(id, (str, unicode))
        self.imports.append(id)
    
    def AppendEnumValue(self, id, value):
        '''
        Add next valid value for enum type.
        
        @param id: identifier of attribute
        @type id: str
        
        @param value: enum value
        @type value: str
        '''
        
        assert isinstance(value, (str, unicode))
        if not id in self.attributes:
            raise DomainTypeError('Unknown identifier %s'%(id, ))
            
        if value in self.attributes.get('enum',[]):
            raise DomainTypeError('the same enum value already defined')
        
        self.attributes[id].setdefault('enum',[]).append(value)
    
    def SetList(self, id, type, parser=None):
        '''
        Set information about items in list
        
        @param id: attribute identifier
        @type id: str
        
        @param type: domain of item in list, must be either atomic or imported
        @type type: str
        
        @param separator: character (substring) by which are items of list
        separated from each other when in list in string representation
        @type separator: str
        '''
        if not id in self.attributes:
            raise DomainTypeError('Unknown identifier %s'%(id, ))
        
        self.attributes[id]['list'] = {'type':type, 'parser':parser}
    
    def AppendParser(self, parser):
        '''
        Add parameters for new parser.
        
        @param regexp: Regular expression, compiled in verbose mode
        @type regexp: str
        '''
        self.parsers.append(parser)


    def __InnerImportLoop(self, name):
        '''
        Inner recursive loop of self.HasImportLoop
        
        @param name: id of domain that is searched in import tree
        @type name: str
        
        @return: string representing part of the loop or False
        @rtype: bool / str
        '''
        for imported in self.imports:
            if name == imported: 
                return self.name + ' - ' + name
            else:
                result = self.factory.GetDomain(imported).__InnerImportLoop(name)
                if result:
                    return self.name + ' - ' + result
        return False

    def HasImportLoop(self):
        '''
        @return: False if no loop detected, string with loop description otherwise
        @rtype: bool / str
        '''
        return self.__InnerImportLoop(self.name)
    
    def UndefinedImports(self):
        '''
        @return: list of the domain names that are imported but not recognized
        @rtype: list
        '''
        return ([name for name in self.imports if not self.factory.HasDomain(name)])
    
    def CheckMissingInfo(self):
        '''
        Search trough self.attributes for missing information such as:
            - enum part of attributes with "enum" domain
            - list part of attributes with "list" domain and all its 
        '''
        for id, info in self.attributes.iteritems():
            if info['type'] == 'enum' and len(info.get('enum',[])) == 0:
                raise DomainTypeError('In domain "&s" is attribute "&s" of enum '
                    'domain, but has no "enum" values defined'&(self.name, id))
            elif info['type'] == 'list' and 'list' not in info:
                raise DomainTypeError('In domain "&s" is attribute "&s" of list '
                    'domain, but has no "list" definition'&(self.name, id))
    
    def GetAttribute(self, id):
        '''
        get item information as an dictionary
        
        keys of the dictionary: 'name', 'type', 'options', 'itemtype'
        
        @param id: identifier of item
        @type id: str
        
        @return: information about item
        @rtype dict
        '''
        
        return self.attributes[id]
    
    def IterAttributeIDs(self):
        '''
        Iterator over ID of items
        
        @rtype: str
        '''
        for id in self.attributeorder:
            yield id
    
    def IterParsers(self):
        '''
        Iterator over parsers
        
        @rtype: re._pattern_type
        '''
        for parser in self.parsers:
            yield parser
    
    def GetName(self):
        '''
        @return: name of current domain
        @rtype: str
        '''
        return self.name
    
    def GetImports(self):
        '''
        @return: list of imported domains
        @rtype: list
        '''
        return self.imports
    
    def GetDefaultValue(self, id=None, domain=None):
        '''
        @return: default value of item defined by id
        @rtype: various
        
        @raise DomainTypeError: when id is not valid item or domain is unknown
        or none of parameters are set
        
        @param id: item identifier
        @type id: str
        '''
        
        if id is not None:
            if not id in self.attributes:
                raise DomainTypeError('Unknown identifier %s'%(id, ))
            type = self.attributes[id]['type']
        
        elif domain is not None and (self.IsAtomic(domain=domain) or self.factory.HasDomain(domain)):
            type = domain
        
        else:
            raise DomainTypeError('None of the parameters are valid\n'
                'id = "%s" domain = "%s"'%(id, domain))
        
        if self.IsAtomic(domain=type):
            if type == 'int': 
                return 0
            elif type == 'float':
                return 0.0
            elif type == 'bool':
                return False
            elif type in ('str', 'text'):
                return ''
            elif type == 'list':
                return []
            elif type == 'enum':
                return self.attributes[id]['enum'][0]
        else:
            return CDomainObject(self.factory.GetDomain(type))
    
    def IsAtomic(self, id=None, domain=None):
        '''
        Test on atomicity of domain
        
        Domain can be specified by parameter domain or by attribute of current
        domain, then its domain is tested.
        
        @return: True if domain is atomic
        @rtype: bool
        
        @param id: identifier of attribute which domain is asked
        @type id: str
        
        @param domain: name of domain
        @type domain: str
        
        @raise DomainTypeError: if id is not valid item identifier
        '''
        if id is not None:
            if not id in self.attributes:
                raise DomainTypeError('Unknown identifier "%s"'%(id, ))
            return self.IsAtomic(domain = self.attributes[id]['type'])
        elif domain is not None:
            return domain in self.ATOMIC
        else:
            raise DomainTypeError("Invalid input parameters")
    
    def TransformValue(self, value, id = None, domain = None):
        '''
        @return: value transformed to domain that is defined for defined attribute
        
        @param id: identifier of the attribute
        @type id: str
        
        @param value: value to be transformed
        
        @raise DomainTypeError: 
            - if id is not recoginzed
            - if value is incopatible with attribute domain
        '''
        
        if domain is None:
            if not id in self.attributes:
                raise DomainTypeError('Unknown identifier %s'%(id, ))
            type = self.attributes[id]['type']
        else:
            type = domain
        
        if type in self.ATOMIC:
            if type == 'int':
                return self.__GetInt(value)
            elif type == 'float':
                return self.__GetFloat(value)
            elif type in ('str', 'text'):
                return self.__GetStr(value)
            elif type == 'bool':
                return self.__GetBool(value)
            elif type == 'enum':
                try:
                    return self.__GetEnum(value, self.attributes[id]['enum'])
                except KeyError:
                    raise DomainTypeError(
                        'In domain "%s" is attribute "%s" of type "enum", '
                        'but has no defined values'%(self.name, id))
            elif type == 'list':
                try:
                    return self.__GetList(value, **self.attributes[id]['list'])
                except KeyError:
                    raise DomainTypeError(
                        'In domain "%s" is attribute "%s" of type "list", '
                        'but has no list definition'%(self.name, id))
        else:
            return self.__GetNonAtomic(value, type)
    
    def __GetInt(self, value):
        if isinstance(value, (int, long)):
            return value
        elif isinstance(value, (str, unicode)):
            try:
                return int(value)
            except:
                raise DomainTypeError('Cannot convert value to int')
        else:
            raise DomainTypeError('Invalid value type')
    
    def __GetFloat(self, value):
        if isinstance(value, (float, int, long, str, unicode)):
            try:
                return float(value)
            except:
                raise DomainTypeError('Cannot convert value to float')
        else:
            raise DomainTypeError('Invalid value type')
    
    def __GetStr(self, value):
        if isinstance(value, (str, unicode)):
            return value
        else:
            return str(value)
    
    def __GetBool(self, value):
        if isinstance(value, bool):
            return value
        elif isinstance(value, (int, float, long)):
            return bool(value)
        elif isinstance(value, (str, unicode)):
            if value.lower() in ('true', '1', 'yes'):
                return True
            elif value.lower() in ('false', '0', 'no'):
                return False
            else:
                raise DomainTypeError('Invalid string to be converted to bool')
        else:
            raise DomainTypeError('Invalid value type')
    
    def __GetEnum(self, value, enum):
        if isinstance(value, (str, unicode)):
            if enum.count(value) > 0:
                return value
            else:
                raise DomainTypeError('value is not member of enumeration')
        elif isinstance(value, (int, long)):
            if 0 <= value < len(enum):
                return enum[value]
            else:
                raise DomainTypeError('value points to the index out of range')
        else:
            raise DomainTypeError('value cannot be converted to enumeration item')
    
    def __GetList(self, value, type, parser=None):
        if isinstance(value, (str, unicode)) and parser is not None : 
            domain = self.factory.GetDomain(type)
            atempt = [False]
            
            for itemparser in domain.IterParsers():
                attempt = [itemparser.CreateObject(part, domain) for part in parser.Split(value)]
                if all(attempt):
                    break
            
            if not all(attempt):
                raise DomainTypeError('No parser can parse all the items in the list')
            return attempt
            
        elif isinstance(value, (list, tuple)):
            return [self.__GetNonAtomic(item, type) for item in value]
            
        else:
            raise DomainTypeError('value cannot be converted to list')

    
    def __GetNonAtomic(self, value, type):
        domain = self.factory.GetDomain(type)
        if isinstance(value, CDomainObject):
            if value.GetType().GetName() == type:
                return value
            else:
                raise DomainTypeError('Type mismatch')
        elif isinstance(value, (str, unicode)):
            attempt = None
            for parser in domain.IterParsers():
                attempt = parser.CreateObject(value, domain)
                if attempt:
                    break
            if not attempt:
                raise DomainTypeError('No parser can parse value')
            return attempt
        elif isinstance(value, dict):
            result = CDomainObject(domain)
            result.SetSaveInfo(value)
            return result
        else:
            raise DomainTypeError('Invalid value type')
    
    def PackValue(self, id, value):
        '''
        '''
        
        if not id in self.attributes:
            raise DomainTypeError('Unknown identifier %s'%(id, ))
        
        if self.IsAtomic(id = id):
            if self.attributes[id]['type'] == 'list':
                return [(unicode(item)
                    if self.IsAtomic(domain = self.attributes[id]['list']['type'])
                    else item.GetSaveInfo()) for item in value]
            else:
                return unicode(value)
        else:
            return value.GetSaveInfo()
