import os
import re
import time

from pkg_resources import resource_filename #IGNORE:E0611

ORIGINAL_DIR = '.dejumblefs'


def pathparts(path):
    return path.split('/')[1:]


def flags2mode(flags):
    filemode = {os.O_RDONLY: 'r', os.O_WRONLY: 'w', os.O_RDWR: 'w+'}
    mode = filemode[flags & (os.O_RDONLY | os.O_WRONLY | os.O_RDWR)]
    if flags & os.O_APPEND:
        mode = mode.replace('w', 'a', 1)
    return mode


def addtrailingslash(path):
    if path.startswith(os.sep):
        return path
    else:
        return '%s%s' % (os.sep, path)


def removeroot(realpath, root):
    if realpath.startswith(root):
        return realpath.replace(root, '', 1)
    else:
        raise RuntimeError


def ignoretag(filename):
    return (not filename == '/..' and not filename == '/.'
            and not filename.startswith('/.dejumblefs'))


def extensionregex(extension):
    return re.compile('%s$' % extension)


def getbasefilelist():
    return ['..', '.']


def unique(s):
    return set(s)

############################################
# Cacheable class


class Cacheable:

    def __init__(self):
        self.expiretime = time.time()

    def reset(self):
        self.expirecache()
        self.refreshcache()

    def expirecache(self):
        self.expiretime = time.time()

    def refreshcache(self):
        if self.expiretime < time.time():
            self.expiretime = time.time() + 60
            self.updatecache()

    def updatecache(self):
        pass

    def deletefromcache(self, string): #IGNORE:W0613
        pass

    def addtocache(self, string): #IGNORE:W0613
        pass

############################################
# Configuration functions

_CONFIGURATION = {}


def readconfig(name):
    if not name in _CONFIGURATION:
        defaultfilename = resource_filename('dejumblefs', #IGNORE:E1101
                                            'conf/%s-default.conf' % name)
        userfilename = os.path.expanduser('~/.dejumblefs/%s.conf' % name)
        currentdirfilename = './.dejumblefs/%s.conf' % name
        config = {}
        readconfigfile(config, defaultfilename)
        readconfigfile(config, userfilename)
        readconfigfile(config, currentdirfilename)
        _CONFIGURATION[name] = config

    return _CONFIGURATION[name]


def readconfigfile(config, path):
    if os.path.isfile(path):
        ofile = open(path, 'r')
        for line in ofile.readlines():
            name, value = line.split('=', 1)
            _CONFIGURATION[name] = value.strip()

    return config
