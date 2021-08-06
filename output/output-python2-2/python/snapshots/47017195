#!/usr/bin/env python

import os
import logging
import re
import pkg_resources

logger = logging.getLogger('dejumble')

ORIGINAL_DIR = '.dejumblefs'

def pathparts(path):
    return path.split('/')[1:]

def flags2mode(flags):
    md = {os.O_RDONLY: 'r', os.O_WRONLY: 'w', os.O_RDWR: 'w+'}
    m = md[flags & (os.O_RDONLY | os.O_WRONLY | os.O_RDWR)]
    if flags | os.O_APPEND:
        m = m.replace('w', 'a', 1)
    return m

_filenameincrease = re.compile('^(.*)\((\d+)\)$')

def increasefilename(path):
    filename, extension = filenameextension(path)
    if not extension == None:
        extension = '.%s' % extension
    else:
        extension = ''

    num = 1
    m = _filenameincrease.match(filename)

    if not m is None:
        num = int(m.group(2)) + 1
        filename = m.group(1)

    return '%s(%i)%s' % (filename, num, extension)

def addtrailingslash(path):
    return '/%s' % path

def ignoretag(filename):
    return not filename == '..' and not filename == '.' and not re.match('\.dejumble', filename)

def extensionregex(extension):
    return re.compile('%s$' % extension);

def filenameextension(path):
    if re.search('\.', path):
        filename, extension = path.rsplit('.', 1)
    else:
        filename = path
        extension = None
    return filename, extension

def getbasefilelist():
    return [ '..', '.' ]

def unique(inlist, keepstr = True):
    typ = type(inlist)
    if not typ == list:
        inlist = list(inlist)
    i = 0
    while i < len(inlist):
        try:
            del inlist[inlist.index(inlist[i], i + 1)]
        except:
            i += 1
    if not typ in (str, unicode):
        inlist = typ(inlist)
    elif keepstr:
        inlist = ''.join(inlist)
    return inlist

_configuration = {}

def readconfig(name):
    if not name in _configuration:
        defaultfilename = pkg_resources.resource_filename('dejumble', 'conf/%s-default.conf' % name)
        userfilename = os.path.expanduser('~/.dejumble/%s.conf' % name)
        currentdirfilename = './.dejumble/%s.conf' % name
        config = {}
        readconfigfile(config, defaultfilename)
        readconfigfile(config, userfilename)
        readconfigfile(config, currentdirfilename)
        _configuration[name] = config

    return _configuration[name]

def readconfigfile(config, path):
    if os.path.isfile(path):
        file = open(path, 'r')
        for line in file.readlines():
            name, value = line.split('=', 1)
            config[name] = value.strip()

    return config


