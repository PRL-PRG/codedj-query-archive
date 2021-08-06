#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import os.path
import shutil

import syck

from config import config
from generators.file import ControlGenerator
from generators.file import RulesGenerator 
from generators.file import ChangelogGenerator 
from generators.file import CompatGenerator 
from generators.file import PostInstGenerator
from generators.file import PostRmGenerator


class Builder(object):
    """ The responsability of this class is to call all the ScriptGenerator.activate methods in the right way.
    """

    def __init__(self, path):
        config['source_path'] = path
        config['info'] = syck.load(open(path + '/gcs/info').read())


    def make_package(self):
        """ Make the package. Use ScriptGenerator objects for this propouse.
        """
        try:
            os.mkdir(config['source_path'] + '/debian')
        except:
            pass

        self.__prepare_conffiles()
        self.__set_compat()

        ControlGenerator().activate()
        RulesGenerator().activate()
        ChangelogGenerator().activate()
        CompatGenerator().activate()
        PostInstGenerator().activate()
        PostRmGenerator().activate()

        os.system('debuild -us -uc')


    def __set_compat(self):
        fcompat = open(config['source_path'] + \
                '/debian/compat', 'w')
        fcompat.write('4')
        fcompat.close()


    def __prepare_conffiles(self):
        """ Add .gv4 extension at all conffiles (making a copy)
        """
        def copy_file(arg, dirname, file_names):

            for fname in file_names:
                abs_path = dirname + os.sep + fname

                if (not '/.svn' in abs_path) and \
                        (not abs_path.endswith(config['config_extension']))\
                        and (os.path.isfile(abs_path)):
                    shutil.copy(abs_path, abs_path + '.gv4')

        os.path.walk(config['source_path'] + '/gcs/conffiles_skel/',
                copy_file, None)

