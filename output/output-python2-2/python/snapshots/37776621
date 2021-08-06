import xml.dom.minidom
import consts
import os.path
import os

class CConfig(object):
    """
    Automatic config file manager
    """
    def __init__(self, file):
        """
        Initialize config manager and loads config file
        
        @param file: path to config file
        @type  file: string
        """
        self.file = None
        self.Clear()
        self.Load(file)
        if not os.path.isdir(self['/Paths/UserDir']):
            os.mkdir(self['/Paths/UserDir'])
        self.original = self.cfgs.copy()
        if os.path.isfile(self['/Paths/UserConfig']):
            try:
                self.Load(self['/Paths/UserConfig'])
            except:
                pass
        self.file = str(self['/Paths/UserConfig'])
    
    def __del__(self):
        """
        Automaticaly save config file on object destroy
        """
        self.Save()
    
    def Clear(self):
        """
        Clears the config values
        """
        self.cfgs = {}
        self.original = self.cfgs
        self.revision = 0
    
    def __setitem__(self, path, value):
        """
        Set config value
        
        @param path: path to config value
        @type  path: string
        
        @param value: value to set
        @type  value: atomic
        """
        self.revision += 1
        self.cfgs[path] = value
    
    def __getitem__(self, path):
        """
        Get config value
        
        @param path: path to config value
        @type  path: string
        
        @return: value at given path
        @rtype:  atomic
        """
        return self.cfgs[path]
    
    def __contains__(self, path):
        """
        Determine, if given path exists in config
        
        @param path: path to config value
        @type  path: string
        
        @return: True, if path exists
        @rtype:  boolean
        """
        return path in self.cfgs
    
    def Load(self, root, path = None):
        """
        Load an XML file under given path
        
        @param root: XML element to parse or XML file path
        @type  root: L{Element<xml.dom.minidom.Element>} or string
        
        @param path: path in config to which values has to be inserted,
            or None
        @type  path: string
        """
        if isinstance(root, (str, unicode)):
            root = xml.dom.minidom.parse(root).documentElement
        if path is None:
            path = ''
        else:
            path += '/'+str(root.tagName)
        text = ''
        for i in root.childNodes:
            if i.nodeType == i.TEXT_NODE:
                text += i.data.decode('unicode_escape')
            if i.nodeType not in (i.ELEMENT_NODE, i.DOCUMENT_NODE):
                continue
            if i.tagName == 'Include':
                if i.hasAttribute('what'):
                    if i.getAttribute('what') == 'app_path':
                        text += consts.ROOT_PATH
                        continue
                elif i.hasAttribute('path'):
                    text += self.cfgs[i.getAttribute('path')]
                    continue
            tmp = self.Load(i, path)
            if tmp != '':
                self.cfgs[path+'/'+str(i.tagName)] = tmp
        text = text.strip()
        self.revision += 1
        if root.hasAttribute('type'):
            type = root.getAttribute('type')
            if type == 'int':
                text = int(text)
            elif type == 'float':
                text = float(text)
            elif type == 'bool':
                text = text.lower() in ('0', 'f', 'no', 'false')
            elif type == 'path':
                text = os.path.abspath(os.path.expanduser(text))
                if os.path.isdir(text):
                    text += os.sep
            return text
        else:
            return text
    
    def Save(self):
        """
        Save changes to user config XML file
        """
        def XMLEncode(val):
            ret = repr(val)
            if isinstance(val, str):
                ret = ret[1:-1]
            elif isinstance(val, unicode):
                ret = ret[2:-1]
            return ret.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('<', '&gt;').replace('"', '&quot;')
        
        out = {}
        save = {'Config': out}
        f = file(self.file, 'w')
        
        def save(root = save, level = 0):
            for part, val in root.iteritems():
                if isinstance(val, dict):
                    print>>f, ' '*(level*4)+'<%s>'%part
                    save(val, level+1)
                    print>>f, ' '*(level*4)+'</%s>'%part
                else:
                    print>>f, ' '*(level*4)+'<%s>%s</%s>'%(part, XMLEncode(val), part)
        
        for path, val in self.cfgs.iteritems():
            if val != self.original.get(path, None):
                tmp = out
                path = path.split('/')
                for part in path[1:-1]:
                    tmp2 = tmp.setdefault(part, {})
                    if not isinstance(tmp2, dict):
                        tmp2 = tmp[part] = {}
                    tmp = tmp2
                tmp[path[-1]] = val
        
        print>>f, '<?xml version="1.0" encoding="utf-8"?>'
        save()
    
    def GetRevision(self):
        """
        Get revision number of config object. Revision is initiated to
        zero and incremented after each change
        
        @return: revision number
        @rtype:  integer
        """
        return self.revision

config = CConfig(consts.MAIN_CONFIG_PATH)
