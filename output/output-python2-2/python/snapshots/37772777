from CodeObject import CCodeObject
from lib.consts import ICONS_PATH, TMP_IMAGES, STYLE_PATH
import os.path
import glob

class CCopyFile(CCodeObject):
    def __init__(self, what = None, extension="*"):
        CCodeObject.__init__(self)
        self.from_dir = self.__GetDir(what)
        self.extension = extension
    
    def __GetDir(self, key):
        o = {}
        o['icons'] = ICONS_PATH
        o['diagrams'] = TMP_IMAGES
        o['style'] = STYLE_PATH
        return o[key]
    
    def copy_file(self, from_dir, to_dir, filename):
        if os.path.isfile(os.path.join(from_dir,filename)):
            input = open(os.path.join(from_dir,filename), "rb")
            output = open(os.path.join(to_dir,filename), "wb")
            output.write(input.read())

    
    def Generate(self, elementObject, path, file = None):
        aktual_dir = os.getcwd()
        if self.from_dir is not None:
            os.chdir(self.from_dir)
            for i in glob.glob("*"):
                self.copy_file(self.from_dir, path, i)
        os.chdir(aktual_dir)
        return [True,""]