#!/usr/bin/python
# -*- coding: utf-8 -*-

import syck
import os
import os.path

from config import config

class FileGenerator(object):

    def __init__(self):
        self.template_content = ''

    
    def activate(self):
        raise NotImplementedError


    def set_template_content(self, template_name):
        """ Set template content from  template_name (using config dictionary)

        @param template_name: Key of the config dictionary for template.
        @type template_name: string
        """
        try:
            template_file = open(config[template_name])
            self.template_content = template_file.read()
            template_file.close()
        except KeyError:
            print "Don't find template '%s'" % template_name
        except:
            print "Can't create template content. Template: %s" % template_name


    
    def _write_file(self, path):
        real_file = open(config['source_path'] + '/' + path, 'w')
        real_file.write(self.template_content)
        real_file.close()


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
        self.set_template_content('control_template')
        self.info = syck.load(open(config['source_path'] + '/info').read())

        self.__set_name()
        self.__set_author()
        #self.__set_descriptions()
        self.__set_depends()

        self._write_file('debian/control')


    def __set_name(self):
        newcontent = self.template_content.replace('<NAME>', self.info['name'])
        self.template_content = newcontent


    def __set_author(self):
        author = self.info['author']
        newcontent = self.template_content.replace('<MANTAINER>', author)
        self.template_content = newcontent


    def __set_depends(self):
        depends = self.__parse_deps('/depends')
        newcontent = self.template_content.replace('<DEPENDS>', depends)
        self.template_content = newcontent


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
    """ Generates debian/rules.

    Generates debian/rules file based on "newfiles_skel" directory 
    and "newfies" file.
    """

    def __init__(self):
        self.dhinstall_list = []
        self.dirs = []
        FileGenerator.__init__(self)


    def activate(self):
        self.set_template_content('rules_template')

        self.__process_newfiles()
        self.__process_newfiles_skel()
        self.__write_rules_file()


    def __process_newfiles(self):
        """ Process "newfiles" file looking for files to install.
        """
        newfiles_lines = open(config['source_path'] + '/newfiles').readlines()

        for line in newfiles_lines:
            line = line.strip()
            line_tuple = line.split()
            if (len(line_tuple) != 2) or line.startswith('#'):
                continue

            self.__add_dhinstall(*line_tuple)


    def __process_newfiles_skel(self):
        """ Process "newfiles_skel" directory recursively.

        Process "newfiles_skel" directory recursively 
        looking for files to install.
        """ 
        orig_stuff_len = len(config['source_path'] + '/')
        dest_stuff_len = len(config['source_path'] + '/newfiles_skel/')


        def set_dhinstalls(arg, dirname, file_names):
            self.dirs.append(dirname[dest_stuff_len - 1:])    

            for fname in file_names:
                base_path = dirname + os.sep + fname
                orig_path = base_path[orig_stuff_len:]
                dest_path = base_path[dest_stuff_len:]

                if not '/.svn' in orig_path: 
                    self.__add_dhinstall(orig_path, dest_path)

        os.path.walk(config['source_path'] + '/newfiles_skel', 
                set_dhinstalls, None)



    def __write_rules_file(self):
        dhinstall_content = '\n'.join(self.dhinstall_list)
        newcontent = self.template_content.replace('<DHINSTALL_SLOT>', 
                dhinstall_content)
        self.template_content = newcontent

        self._write_file('debian/rules')

        # write debian/dirs file
        dirs_file = open(config['source_path'] + '/debian/dirs', 'w')
        dirs_file.write('\n'.join(self.dirs))
        dirs_file.close()


    def __add_dhinstall(self, orig_path, dest_path):
        command = "\tdh_install %s\t%s" % (orig_path, dest_path)
        self.dhinstall_list.append(command)







class PostInstallGenerator(FileGenerator):
    pass


class PreRemoveGenerator(FileGenerator):
    pass
