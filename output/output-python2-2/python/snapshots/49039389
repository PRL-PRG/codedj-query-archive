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
import urllib2
import bz2
import gzip

__revision__ = '0.01'

def get_content(uri):
    """ get_content(uri) -> string

    Get from the URI uri the content of the file which is pointed with 
    that URI.

    For doing that thing some functions are to be called. This is an 
    abstraction of those funtions who are going to decompress the
    file gotten from the URI.

    """

    path, extension = os.path.splitext(uri)
    try:
        file_desc = urllib2.urlopen(uri)
    except OSError, error:
        print >> sys.stderr, "Error: %s" % error.strerror
        print >> sys.stderr, "%s was not found" % uri
        return None
    except urllib2.URLError:
        print >> sys.stderr, "Error: \n%s was not found" % uri
        return None
    orig_file_content = file_desc.read()
    file_desc.close()
    
    if extension == '.gz':
        content = decompress_gz(orig_file_content)
    elif extension == '.bz2':
        content = decompress_bz2(orig_file_content)
    elif extension == '':
        content = orig_file_content
    else:
        print >> sys.stderr, "Error: Extension %s not supported" % extension
        return None

    return content


def decompress_gz(file_content):
    """ decompress_gz(file_content) -> string

    Decompress and extract the content of file_content.

    This function is used for gzip compressed files.

    """

    tmp_file_name = os.tempnam('/tmp')
    temp_file_gz = open(tmp_file_name, 'w')
    temp_file_gz.write(file_content)
    temp_file_gz.close()
    content = gzip.open(tmp_file_name).read()

    return content


def decompress_bz2(file_content):
    """ decompress_bz2(file_content) -> string

    Decompress and extract the content of file_content.

    This function is used for bzip2 compressed files.

    """

    content = bz2.decompress(file_content)

    return content

