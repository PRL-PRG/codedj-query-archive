#!/usr/bin/python
# -*- coding: utf-8 -*-

import syck
import os

from config import config

class FileGenerator(object):
    
    def activate(self):
        raise NotImplementedError


class ControlGenerator(FileGenerator):
    """ Generate debian/control file from ./info file.
    """
    def activate(self):
        """ Generate debian/control file

        Steps:

        1) Obtain control template
        2) Set template properties (using tags) from ./info file.
        3) Write debian/control file
        """
        self.control_content = open(config['control_template']).read()
        self.info = syck.load(open(config['source_path'] + '/info').read())

        self.__set_name()
        self.__set_author()
        self.__set_depends()

        self.__write_control_file()


    def __set_name(self):
        newcontent = self.control_content.replace('<NAME>', self.info['name'])
        self.control_content = newcontent


    def __set_author(self):
        pass


    def __set_depends(self):
        pass


    def __write_control_file(self):
        control_file = open(config['source_path'] + '/debian/control', 'w')
        control_file.write(self.control_content)
        control_file.close()



class RulesGenerator(FileGenerator):
    pass


class PostInstallGenerator(FileGenerator):
    pass


class PreRemoveGenerator(FileGenerator):
    pass
