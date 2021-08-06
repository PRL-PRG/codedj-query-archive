#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date$ 
#     $Author$
#

""" Utils is a bunch os utils functions

List of functons:
 - get_path() -> string

"""


import os.path
import os

__revision__ = '0.01'


def get_path(binary):
    """get_path(binary) -> bool

    Check if the binary is installed in the path of the user.
    If there it is, returns the path, if not, returns None

    """

    # Get the PATH of the current user
    full_path = os.environ['PATH'].split(':')

    for path in full_path:
        bin = os.path.join(path, binary)
        if os.path.isfile(bin):
            return bin

    return None
    


# vim:ai:et:sts=4:tw=80:sw=4:
