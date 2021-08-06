from lib.lib import UMLException, XMLEncode
import os.path
from lib.config import config
import datetime

#try to import necessary lybraries for XML parsing
try:
    from lxml import etree
    HAVE_LXML = True
    #print("running with lxml.etree")
except ImportError:
    HAVE_LXML = False
    try:
        # Python 2.5
        import xml.etree.cElementTree as etree
        #print("running with cElementTree on Python 2.5+")
    except ImportError:
        try:
            # Python 2.5
            import xml.etree.ElementTree as etree
            #print("running with ElementTree on Python 2.5+")
        except ImportError:
            try:
                # normal cElementTree install
                import cElementTree as etree
                #print("running with cElementTree")
            except ImportError:
                # normal ElementTree install
                import elementtree.ElementTree as etree
                #print("running with ElementTree")

                    
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
        #xml (version) file is validate with xsd schema (recentfile.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise UMLException("XMLError", xmlschema.error_log.last_error)

        for file in root.getchildren():
            if os.path.exists(file.get("name").decode('unicode_escape')):
                self.files.append((file.get("name").decode('unicode_escape'),file.get("date").decode('unicode_escape')))

    def SaveRecentFiles(self):
        f = file(self.filename,"w")
        print>>f, '<?xml version="1.0"?>'
        print>>f, '<RecentFiles xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://umlfri.kst.fri.uniza.sk/xmlschema/recentfiles.xsd ..\share\schema\recentfiles.xsd" xmlns="http://umlfri.kst.fri.uniza.sk/xmlschema/recentfiles.xsd">'
        for name, date in self.files:
            print>>f, '  <File name="%s" date="%s" />'%(XMLEncode(name),XMLEncode(date))
        print>>f,'</RecentFiles>'
        f.close()
        
