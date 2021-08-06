#!/usr/bin/env python
#-*- coding: utf8 -*-

# Módulo ffsectool- Modulo de utilidad que hace varias operaciones sobre la
# base de datos de seguridad de Firefox
#
#Copyright (C) 2005 Junta de Andalucía
#
#Autor/es (Author/s):
#
#- Lorenzo Gil Sanchez <lgs@yaco.es>
#
#Este fichero es parte del módulo E-Admin de Guadalinex 2006
#
#E-Admin de Guadalinex 2006 es software libre. Puede redistribuirlo y/o modificarlo
#bajo los términos de la Licencia Pública General de GNU según es
#publicada por la Free Software Foundation, bien de la versión 2 de dicha
#Licencia o bien (según su elección) de cualquier versión posterior.
#
#Detección de Hardware de Guadalinex 2005  se distribuye con la esperanza de que sea útil,
#pero SIN NINGUNA GARANTÍA, incluso sin la garantía MERCANTIL
#implícita o sin garantizar la CONVENIENCIA PARA UN PROPÓSITO
#PARTICULAR. Véase la Licencia Pública General de GNU para más detalles.
#
#Debería haber recibido una copia de la Licencia Pública General
#junto con Detección de Hardware de Guadalinex 2005 . Si no ha sido así, escriba a la Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
#
#-------------------------------------------------------------------------
#
#This file is part of E-Admin de Guadalinex 2006 .
#
#E-Admin de Guadalinex 2006  is free software; you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation; either version 2 of the License, or
#at your option) any later version.
#
#E-Admin Guadalinex 2006  is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.
#
#You should have received a copy of the GNU General Public License
#along with Foobar; if not, write to the Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import commands
import subprocess
import os

FIREFOX_CMD  = '/usr/bin/firefox'
CERTUTIL_CMD = '/usr/bin/certutil'
MODUTIL_CMD  = '/usr/bin/modutil'

class FireFoxSecurityUtils(object):
    def is_firefox_running(self):
        cmd = '%s -remote "ping()"' % FIREFOX_CMD
        status, output = commands.getstatusoutput(cmd)
        return status == 0

    def create_default_profile(self):
        cmd = '%s -CreateProfile "default"'
        status, output = commands.getstatusoutput(cmd)
        return status == 0

    def get_default_profile_dir(self):
        user_dir = os.path.expanduser('~')
        ff_dir = os.path.join(user_dir, '.mozilla', 'firefox')
        for name in os.listdir(ff_dir):
            full_name = os.path.join(ff_dir, name)
            if os.path.isdir(full_name) and name.endswith('.default'):
                return full_name

    def has_security_method(self, security_method):
        profile = self.get_default_profile_dir()
        if not profile:
            return False

        cmd = '%s -list -dbdir "%s"' % (MODUTIL_CMD, profile)
        status, output = commands.getstatusoutput(cmd)
        return security_method in output

    def add_security_method(self, name, library, mechanisms="FRIENDLY"):
        profile = self.get_default_profile_dir()
        if not profile:
            return False

        # we need to use the subprocess module since this command request
        # interactive input
        cmd = [MODUTIL_CMD, '-add', "%s" % name, '-libfile', library,
               '-mechanisms', "%s" % mechanisms, '-dbdir', "%s" % profile]
        process = subprocess.Popen(cmd,
                                   stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE)
        process.stdin.write('\n')
        process.wait()
        return process.returncode == 0

    def has_root_ca_certificate(self, ca_name):
        profile = self.get_default_profile_dir()
        if not profile:
            return False

        cmd = '%s -L -n "%s" -d "%s"' % (CERTUTIL_CMD, ca_name, profile)
        status, output = commands.getstatusoutput(cmd)
        return status == 0

    def add_root_ca_certificate(self, ca_name, certificate_file):
        profile = self.get_default_profile_dir()
        if not profile:
            return False

        if self.has_root_ca_certificate(ca_name):
            return False

        cmd = '%s -A -a -d "%s" -i %s -n "%s" -t "TCu,Cu,Cu"'
        cmd = cmd % (CERTUTIL_CMD, profile, certificate_file, ca_name)
        status, output = commands.getstatusoutput(cmd)
        return status == 0
