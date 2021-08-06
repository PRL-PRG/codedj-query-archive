#!/usr/bin/python
# -*- coding: utf-8 -*-

import os

import syck

from generators.file import ControlGenerator
from config import config


class Builder(object):
    """ The responsability of this class is to call all the ScriptGenerator.activate methods in the right way.
    """

    def __init__(self, path):
        config['source_path'] = path


    def make_package(self):
        """ Make the package. Use ScriptGenerator objects for this propouse.
        """
        try:
            os.mkdir(config['source_path'] + '/debian')
        except:
            pass

        ControlGenerator().activate()

