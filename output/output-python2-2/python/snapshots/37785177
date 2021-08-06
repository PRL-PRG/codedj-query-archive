import os
import stat

from Scanner import CScanner, NoMoreTokens, SyntaxError
from TextScanner import CTextScanner
from lib.ReverseEngineering.Tokens import *

class CDirectoryScanner(CScanner):
    def __init__(self, patterns, ignore, root):
        CScanner.__init__(self, patterns, ignore)
        self.path = (root, )
        self.files = []
        self.dirs = []
        self.text_scanner = None
        
    def get_file(self):
        if self.path:
            for name in os.listdir(os.path.join(*self.path)):
                try:
                    st = os.lstat(os.path.join(*self.path + (name, )))
                except os.error:
                    continue
                if stat.S_ISREG(st.st_mode):
                    result = self.path + (name, )
                    if result not in self.files:
                        print result
                        return result
        return None
        
    def get_dir(self):
        if self.path:
            for name in os.listdir(os.path.join(*self.path)):
                try:
                    st = os.lstat(os.path.join(*self.path + (name, )))
                except os.error:
                    continue
                if stat.S_ISDIR(st.st_mode):
                    result = self.path + (name, )
                    if result not in self.dirs:
                        print result
                        return result
        return None
        
    def open_file(self, path):
        f = file(os.path.join(*path), 'r')
        self.files.append(path)
        self.text_scanner = CTextScanner(self.patterns, self.ignore, f.read())
        
    def open_dir(self, path):
        self.dirs.append(path)
        self.path = path
        
    def scan(self, restrict):
        while True:
            best_pat = None
            if self.text_scanner is not None:
                best_pat = self.text_scanner.token(-1, restrict)
                if best_pat[0] == 'eof':
                    self.text_scanner = None
            else:
                best_pat = None
                best_match = -1
                for token in self.patterns:
                    p = token.terminal
                    if restrict and p not in restrict and p not in self.ignore:
                        continue
                    if isinstance(token, CFileToken):
                        path = self.get_file()
                        if path is None:
                            continue
                        _str, _len = token.match(path[-1])
                        if _str is None or p in self.ignore:
                            continue
                        self.open_file(path)
                        best_pat = p, _str, None
                        break
                    elif isinstance(token, CDirToken):
                        path = self.get_dir()
                        if path is None:
                            continue
                        _str, _len = token.match(path[-1])
                        if _str is None or p in self.ignore:
                            continue
                        self.open_dir(path)
                        best_pat = p, _str, None
                        break
                    elif restrict and 'epsilon' in restrict:
                        best_pat = 'epsilon', '', None
                    else:
                        continue
            if best_pat is None :
                if len(self.path) > 0:
                    best_pat = 'eod', '', None
                    self.path = self.path[0:-1]
                else:
                    return
            if best_pat is not None:
                if best_pat[0] not in self.ignore:
                    self.tokens.append(best_pat)
                    self.restrictions.append(restrict)
                return
