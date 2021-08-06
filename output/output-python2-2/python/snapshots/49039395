#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date:  $ 
#     $Author:  $
#

""" Decompress is a module for decompressing bz2 or gz compressed files.

For a compresed or not compressed file return its content.

"""

import sys
import os.path
import os
import bz2
import urllib2


def get_content(uri):

    path, extension = os.path.splitext(uri)
    try:
        file_desc = urllib2.urlopen(uri)
    except Error, e:
        print >> sys.std.err, "Error: %s" % e.value
        return None
    file = file_desc.read()
    file_desc.close()
    
    if extension == '.gz':
        content = decompress_gz(file)
    elif extension == '.bz2':
        content = decompress_bz2(file)
    elif extension == '':
        content = file
    else:
        print >> stderr, "Error: Extension %s not supported" % extension
        return None

    return content


def decompress_gz(file):

    tmp_file_name = os.tempnam('/tmp')
    temp_file_gz = open(tmp_file_name, 'w')
    temp_file_gz.write(file)
    temp_file_gz.close()
    content = gzip.open(tmp_file_name).read()

    return content


def decompress_bz2(file):

    content = bz2.decompress(file)

    return content

