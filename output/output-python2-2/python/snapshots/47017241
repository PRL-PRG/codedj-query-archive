#!/usr/bin/env python

import os
import logging
import re

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

def increasefilename(path):
    if re.search('\.', path):
        filename, extension = path.rsplit('.', 1)
        extension = '.' + extension
    else:
        filename = path
        extension = ''

    num = 1
    m = re.match('^(.*)\((\d+)\)$', filename)

    if not m is None:
        num = int(m.group(2)) + 1
        filename = m.group(1)

    return filename + '(' + str(num) + ')' + extension

def addtrailingslash(path):
    return '/' + path

def getbasefilelist():
    return [ '..', '.' ]

def extract(tuple):
    if tuple == None:
        return None
    else:
        return str(tuple[0])

