import consts
import os.path
import os
import colors
# TODO: recursive imports
# from lib.Exceptions.DevException import *

#try to import necessary lybraries for XML parsing
try:
    from lxml import etree
    HAVE_LXML = True
except ImportError:
    HAVE_LXML = False
    try:
        # Python 2.5
        import xml.etree.cElementTree as etree
    except ImportError:
        try:
            # Python 2.5
            import xml.etree.ElementTree as etree
        except ImportError:
            try:
                # normal cElementTree install
                import cElementTree as etree
            except ImportError:
                # normal ElementTree install
                import elementtree.ElementTree as etree

def path_type(val):
    val = val.replace(u'\xFF', consts.ROOT_PATH)
    val = os.path.abspath(os.path.expanduser(val))
    if os.path.isdir(val):
        val += os.sep
    return val

def color_type(val):
    return colors.colors.get(val, val)

types = {
    "/Styles/Element/LineColor": color_type,
    "/Styles/Element/FillColor": color_type,
    "/Styles/Element/Fill2Color": color_type,
    "/Styles/Element/Fill3Color": color_type,
    "/Styles/Element/ShadowColor": color_type,
    "/Styles/Element/NameTextColor": color_type,
    "/Styles/Element/TextColor": color_type,
    "/Styles/Connection/ArrowAngleSteps": int,
    "/Styles/Connection/MinimalAngle": float,
    "/Styles/Connection/LineColor": color_type,
    "/Styles/Connection/ArrowColor": color_type,
    "/Styles/Connection/ArrowFillColor": color_type,
    "/Styles/Connection/NameTextColor": color_type,
    "/Styles/Connection/TextColor": color_type,
    "/Styles/Connection/TextFill": color_type,
    "/Styles/Selection/PointsSize": int,
    "/Styles/Selection/RectangleWidth": int,
    "/Styles/Selection/PointsColor": color_type,
    "/Styles/Selection/RectangleColor": color_type,
    "/Styles/Drag/RectangleWidth": int,
    "/Styles/Drag/RectangleColor": color_type,
    "/Paths/Root": path_type,
    "/Paths/Templates": path_type,
    "/Paths/Images": path_type,
    "/Paths/Gui": path_type,
    "/Paths/Locales": path_type,
    "/Paths/Schema": path_type,
    "/Paths/UserDir": path_type,
    "/Paths/UserConfig": path_type,
    "/Paths/RecentFiles": path_type,
    "/Page/Width": int,
    "/Page/Height": int,
}

class CConfig(object):
    """
    Automatic config file manager
    """
    
    CONFIG_NAMESPACE = 'http://umlfri.kst.fri.uniza.sk/xmlschema/config.xsd'
    
    def __init__(self, file):
        """
        Initialize config manager and loads config file
        
        @param file: path to config file
        @type  file: string
        """
        self.file = None
        self.original = {}
        self.Clear()
        
        tree = etree.XML(open(file).read().replace('&apppath;', '&#xFF;'))
        if HAVE_LXML:
            xmlschema_path = path_type(tree.find('./{'+self.CONFIG_NAMESPACE+'}Paths/{'+self.CONFIG_NAMESPACE+'}Schema').text)
            if xmlschema_path:
                xmlschema_doc = etree.parse(os.path.join(xmlschema_path, "config.xsd"))
                self.xmlschema = etree.XMLSchema(xmlschema_doc)
                if not self.xmlschema.validate(tree):
                    raise Exception, ("XMLError", self.xmlschema.error_log.last_error)
            else:
                raise Exception, ("XMLError", "Schema path is not found in config file")
        
        self.original = self.__Load(tree)
        self.cfgs = self.original.copy()
        
        k = self.original.keys()
        k.sort()
        
        if not os.path.isdir(self['/Paths/UserDir']):
            os.mkdir(self['/Paths/UserDir'])
        
        self.file = self['/Paths/UserConfig']
        if os.path.isfile(self.file):
            tree = etree.XML(open(self.file).read().replace('&apppath;', '&#xFF;'))
            # TODO: User config validation
            self.cfgs.update(self.__Load(tree))
    
    def __del__(self):
        """
        Automaticaly save config file on object destroy
        """
        self.__Save()
    
    def Clear(self):
        """
        Clears the config values
        """
        self.cfgs = {}
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
    
    def __Load(self, root):
        """
        Load an XML file under given path
        
        @param root: XML element to parse
        @type  root: L{Element<xml.etree.ElementTree.Element>}
        """
        
        ret = {}
        def recursive(root, path):
            for child in root:
                name = path+child.tag.split('}')[1]
                if len(child):
                    recursive(child, name+'/')
                elif child.text is None:
                    ret[name] = types.get(name, unicode)('')
                else:
                    ret[name] = types.get(name, unicode)(child.text)
        
        recursive(root, '/')
        
        return ret
    
    def __Save(self):
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
                    if level == 0:
                        print>>f, ' '*(level*4)+'<%s xmlns="%s">'%(part, self.CONFIG_NAMESPACE)
                    else:
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
