#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import logging

APTCONFPATH='/usr/share/gsd/apt.conf'
SOURCESFILE='/tmp/gsd/sources.list'
DISTRONAME='flamenco'

class SupplementCustomizer(object):

    def __init__(self, mountpoint):
        self.logger = logging.getLogger()
        self.mountpoint = mountpoint


    def customize(self):
        """
        This method install suppletory.
        """
        print "<#> Customizing"
        self.__prepare_system() 

        #Update apt system
        cmd = 'APT_CONFIG=' + APTCONFPATH 
        cmd += ' sudo synaptic --hide-main-window' 
        cmd += ' --update-at-startup --non-interactive'
        os.system(cmd)


    def get_diskdefines(self ):
        filepath = self.mountpoint + \
                '/README.diskdefines'

        try:
            fileobject = open(filepath)
        except Exception, e:
            self.logger.error(str(e))
            return {}

        result = {}
        for line in fileobject.readlines():
            items = line.split(None, 2)
            try:
                result[items[1]] = items[2]
            except IndexError, e:
                result[items[1]] = ''

        return result


    def __create_sources_list(self):

        self.logger.debug('Creating sources.list')
        diskdefines = self.get_diskdefines()
        fileobj = open(SOURCESFILE, 'w')

        #Create entries for the supplement's URIs
        keys = diskdefines.keys()
        keys.sort()
        for key in keys:
            if key.startswith('URI'):
                self.__process_uri(diskdefines[key], fileobj)
        fileobj.close()


    def __process_uri(self, value, fileobj):
        
        self.logger.debug('Processing uri: ' + value)
        if value.startswith('http://') or \
                value.startswith('fto://'):
            fileobj.write('deb ' + str(value) + '\n')

        else:
            fileobj.write('deb file:' + self.mountpoint + value + \
                    ' '+ DISTRONAME +' main \n')


    def __prepare_system(self):
        #Try for password. Three times.
        res = 768 
        attemps = 0

        # Errno 768: Bad password
        while res == 768 and attemps < 3:
            res = os.system('gksudo -m "Introduzca contraseña" /bin/true')
            # Errno 512: User press cancel
            if res == 512:
                self.logger.debug("User press cancel")
                return
            attemps += 1

        if res == 768:
            self.logger.debug("Three attemps for password")
            return

        #Prepare apt system
        os.system('cp -a /usr/share/gsd /tmp')

        #Generate sources.list
        self.__create_sources_list()
         
