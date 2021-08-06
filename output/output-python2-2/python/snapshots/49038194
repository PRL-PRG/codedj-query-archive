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
        self.__set_descriptions()
        self.__set_depends()

        self.__write_control_file()


    def __set_name(self):
        newcontent = self.control_content.replace('<NAME>', self.info['name'])
        self.control_content = newcontent


    def __set_author(self):
        author = self.info['author']
        newcontent = self.control_content.replace('<MANTAINER>', author)
        self.control_content = newcontent


    def __set_depends(self):
        depends = self.__parse_deps('/depends')
        newcontent = self.control_content.replace('<DEPENDS>', depends)
        self.control_content = newcontent


    def __write_control_file(self):
        control_file = open(config['source_path'] + '/debian/control', 'w')
        control_file.write(self.control_content)
        control_file.close()


    def __parse_deps(self, file):
        depends_list = open(config['source_path'] + file).readlines()
    
        new_depends = []
        for depend in depends_list:
            depend = depend.strip()
            if not depend or depend.startswith('#'):
                continue
            name_and_version = depend.split()

            depend_string = name_and_version[0]
            if len(name_and_version) == 2:
                depend_string += " (%s)" %(name_and_version[1])

            new_depends.append(depend_string)

        depends = ', '.join(new_depends)
        return depends



class RulesGenerator(FileGenerator):
    pass


class PostInstallGenerator(FileGenerator):
    pass


class PreRemoveGenerator(FileGenerator):
    pass
