from lib.Exceptions import DomainObjectError
import re

class CDomainObject(object):
    '''
    representation of logical element attribute - its value
    '''
    
    def __init__(self, type):
        '''
        create new instance
        
        all the inner values are set to default values defined by type.
        Non-atomic inner values are set to another CDomainObject objects
        creating tree-like structure.
        
        @param type: domain type of current object - definition
        @type type: L{CDomainType<Type.CDomainType>}
        '''
        
        if isinstance(type, (str, unicode)):
            raise DomainObjectError('string cannot be used as domain reference')
        self.type = type
        self.values = {}
        for id in self.type.IterAttributeIDs():
            self.values[id] = self.type.GetDefaultValue(id)
    
    def GetType(self, id=''):
        '''
        @return: DomainType of attribute
        @rtype: L{CDomainType<lib.Domains.Type.CDomainType>}
        
        @param id: path to the attribute
        @type id: str
        '''
        return self._TracePath(id, 'gettype')
    
    def GetDomainName(self, id):
        '''
        @return: name of DomainType of attribute
        @rtype: str
        
        @param id: path to the attribute
        @type id: str
        '''
        return self._TracePath(id, 'getdomainname')
    
    def GetValue(self, id):
        '''
        @return: value of attribute
        @rtype: various
        
        @param id: path to the attribute
        @type id: str
        '''
        return self._TracePath(id, 'getvalue')
    
    def SetValue(self, id, value):
        '''
        Set new value to the attribute
        
        @param id: path to the attribute
        @type id: str
        
        @param value: new value to be set
        @type value: various
        '''
        self._TracePath(id, 'setvalue', value)
    
    def AppendItem(self, id):
        '''
        Append next object to the attribute with type list
        
        @param id: path to the attribute
        @type id: str
        
        @param id: path to the attribute
        @type id: str
        '''
        self._TracePath(id, 'append')
    
    def RemoveItem(self, id):
        '''
        Remove object from attribute with type list
        
        @param id: path to the attribute
        @type id: str
        '''
        self._TracePath(id, 'remove')
    
    def HasVisualAttribute(self, id):
        '''
        @return: True if attribute is being displayed
        '''
        return self.GetDomainName(id) != 'text'
        #return self._TracePath(id, 'visual')
    
    def _TracePath(self, id, action, value = None):
        '''
        Find attribute defined by id and perform action
        
        @param id: 
        '''
        
        path = re.split(r'(\[|\.)', id, 1)
        
        if len(path) == 1: #work with current attribute
            if action == 'setvalue':
                self.values[path[0]] = self.type.TransformValue(value, id = path[0])
                return
            elif action == 'getvalue':
                return self.values[path[0]]
            elif action == 'gettype':
                if path[0] == '':
                    return self.type
                else:
                    return self.type.GetFactory().GetDomain(self.type.GetAttribute(path[0])['type'])
            elif action == 'getdomainname':
                if path[0] == '':
                    return self.type.GetName()
                else:
                    return self.type.GetAttribute(path[0])['type']
            elif action == 'append':
                if self.type.GetAttribute(path[0])['type'] == 'list':
                    self.values[path[0]].append(self.type.GetDefaultValue(domain = self.type.GetAttribute(path[0])['list']['type']))
                else:
                    raise DomainObjectError('Attribute %s of domain %s is not of type "list"'%\
                    (path[0], self.type.GetName()))
            elif action == 'remove':
                raise DomainObjectError('RemoveItem is allowed on item of a list only')
            elif action == 'visual':
                return self.type.HasVisualAttribute(path[0])
        
        elif path[1] == '.': #nested call
            if self.type.IsAtomic(id = path[0]): #atomic element doesn't have items
                raise DomainObjectError('Attribute %s of domain %s is atomic'%\
                    (path[0], self.type.GetName()))
            return self.values[path[0]]._TracePath(path[2], call, value)
        
        elif path[1] == '[': #index of list
            
            if self.type.GetAttribute(path[0])['type'] <> 'list':
                raise CDomainObjectError('Attribute %s of domain %s cannot be indexed'%\
                    (path[0], self.type.GetName()))
            
            idx, rest = path[2].split(']', 1)
            idx = int(idx)
            if self.type.IsAtomic(domain = self.type.GetAttribute(path[0])['list']['type']) or rest == '':
                if rest:
                    raise DomainObjectError('Nothing was expected after "]"')
                
                if action == 'setvalue':
                    self.values[path[0]][idx] = self.type.TransformValue(value, domain = self.type.GetAttribute(path[0])['list']['type'])
                    return
                elif action == 'getvalue':
                    return self.values[path[0]][idx]
                elif action == 'gettype':
                    return self.type.GetFactory().GetDomain(self.type.GetAttribute(path[0])['list']['type'])
                elif action == 'getdomainname':
                    return self.type.GetAttribute(path[0])['list']['type']
                elif action == 'append':
                    if self.type.GetAttribute(path[0])['list']['type'] == 'list':
                        self.values[path[0]][idx].append(self.type.GetDefaultValue(domain = self.type.GetAttribute(path[0])['list']['type']))
                    else:
                        raise DomainObjectError('Type of items in list %s of domain %s are not of type "list"'%\
                        (path[0], self.type.GetName()))
                elif action == 'remove':
                    self.values[path[0]].pop(idx)
                elif action == 'visual':
                    return self.type.HasVisualAttribute(path[0])
                
            if rest.startswith('.'):
                return self.values[path[0]][idx]._TracePath(rest[1:], action, value)
            
    def GetSaveInfo(self):
        '''
        @return: structured dictionary containing all the necessary data for .frip file
        @rtype: dict
        '''
        return dict([(id, self.type.PackValue(id, value)) for id, value in self.values.iteritems()])
    
    def SetSaveInfo(self, data):
        '''
        Restore all the attribute values from dictionary loaded from .frip file
        
        @param data: structured dictionary as returned from 
        L{self.GetSaveInfo<self.GetSaveInfo>}
        @type data: dict
        '''
        for key, value in data.iteritems():
            if isinstance(value, dict):
                self.GetValue(key).SetSaveInfo(value)
            elif isinstance(value, (list, str, unicode)):
                self.SetValue(key, value)
