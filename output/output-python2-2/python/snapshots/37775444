from lib.lib import XMLEncode, Indent
from lib.Exceptions.UserException import *
import os.path
from lib.config import config
import datetime
from lib.consts import RECENTFILES_NAMESPACE

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

                    
#if lxml.etree is imported successfully, we use xml validation with xsd schema
if HAVE_LXML:
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "recentfiles.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)


class CRecentFiles(object):
    
    def __init__(self):
        self.filename = os.path.expanduser(config["/Paths/RecentFiles"])
        
        self.files = []
        self.LoadRecentFiles()

    
    def GetRecentFiles(self):
        for i in self.files:
            yield i
    
    def RemoveFile(self, file):
        for id, (f,d) in enumerate(self.files):
            if f == file:
                del self.files[id]
                return
    
    def AddFile(self, file):
        file = os.path.abspath(file)
        self.RemoveFile(file)
        self.files.insert(0,(file,datetime.datetime.now().strftime("%d.%m.%Y %H:%M:%S")))
    
    
    def LoadRecentFiles(self):
        try:
            tree = etree.parse(self.filename)     #try to open and parse recentfile.xml file
        except IOError, e:
            return
        
        root = tree.getroot()
        #xml (recentfiles.xml) file is validate with xsd schema (recentfile.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise XMLError(xmlschema.error_log.last_error)

        for file in root.getchildren():
            if os.path.exists(file.get("name")):
                self.files.append((file.get("name"),file.get("date")))

    def SaveRecentFiles(self):
        root = etree.XML('<RecentFiles xmlns="http://umlfri.kst.fri.uniza.sk/xmlschema/recentfiles.xsd"></RecentFiles>')
        
        for name, date in self.files:
            root.append(etree.Element(RECENTFILES_NAMESPACE+"File", name=name, date=date))  #namespace {xxx} is required

        #xml tree is validate with xsd schema (recentfile.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise XMLError(xmlschema.error_log.last_error)
        
        #make human-friendly tree
        Indent(root)
        
        #save Recent File Tree
        f = open(self.filename, 'w')
        f.write('<?xml version="1.0" encoding="utf-8"?>\n'+etree.tostring(root, encoding='utf-8'))
        f.close()
        