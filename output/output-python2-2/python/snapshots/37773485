import xml.dom.minidom
import consts
import os.path

class CConfig:
    def __init__(self, file, userconfig = None):
        self.clear()
        self.load(file)
        if userconfig is not None:
            self.original = self.cfgs.copy()
            if os.path.isfile(userconfig):
                self.load(userconfig)
            self.file = userconfig
        else:
            self.file = file
    
    def clear(self):
        self.cfgs = {}
        self.original = self.cfgs
        self.__getitem__ = self.cfgs.__getitem__
        self.__setitem__ = self.cfgs.__setitem__
        self.__contains__ = self.cfgs.__contains__
    
    def load(self, root, path = ''):
        if isinstance(root, (str, unicode)):
            root = xml.dom.minidom.parse(root).documentElement
        path += '/'+root.tagName
        text = ''
        for i in root.childNodes:
            if i.nodeType == i.TEXT_NODE:
                text += i.data
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
            self.cfgs[path+'/'+i.tagName] = self.load(i, path)
        if root.hasAttribute('type'):
            type = root.getAttribute('type')
            if type == 'int':
                text = int(text)
            elif type == 'float':
                text = float(text)
            elif type == 'bool':
                text = text.lower() in ('0', 'f', 'no', 'false')
            return text
        else:
            return text

config = CConfig(consts.MAIN_CONFIG_PATH, consts.USER_CONFIG_PATH)
