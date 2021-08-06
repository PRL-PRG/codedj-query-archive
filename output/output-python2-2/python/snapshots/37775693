from lib.lib import UMLException, XMLEncode
import os.path
from lib.config import config
import xml.dom.minidom
import datetime



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
            dom = xml.dom.minidom.parse(self.filename)
        except IOError, e:
            return
        root = dom.documentElement
        if root.tagName != 'RecentFiles':
            raise UMLException("XMLError", root.tagName)
        
        for file in root.childNodes:
            if file.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            if file.tagName == 'File':
                if os.path.exists(file.getAttribute("name").decode('unicode_escape')):
                    self.files.append((file.getAttribute("name").decode('unicode_escape'),file.getAttribute("date").decode('unicode_escape')))

    def SaveRecentFiles(self):
        f = file(self.filename,"w")
        print>>f, '<?xml version="1.0"?>'
        print>>f, '<RecentFiles>'
        for name, date in self.files:
            print>>f, '  <File name="%s" date="%s" />'%(XMLEncode(name),XMLEncode(date))
        print>>f,'</RecentFiles>'
        f.close()
        
