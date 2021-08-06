import os.path
import os

from AbstractStorage import CAbstractStorage

class CDirectory(CAbstractStorage):
    def __init__(self, path):
        self.path = path
    
    def listdir(self, path):
        return os.listdir(os.path.join(self.path, path))
    
    def get_file_path(self, path):
        return os.path.join(self.path, path)
    
    def file(self, path):
        return open(self.get_file_path(path), 'rb')
    
    def read_file(self, path):
        return self.file(path).read()
