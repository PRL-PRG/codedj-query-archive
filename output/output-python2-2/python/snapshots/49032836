#!/usr/bin/python
# -*- coding: utf-8 -*-


#Módulo usbprinter - Módulo que implementa el "actor hardware" para las
#impresoras usb
#
#Copyright (C) 2005 Junta de Andalucía
#
#Autor/es (Author/s):
#
#- Gumersindo Coronel Pérez <gcoronel@emergya.info>
#
#Este fichero es parte de Detección de Hardware de Guadalinex 2005 
#
#Detección de Hardware de Guadalinex 2005  es software libre. Puede redistribuirlo y/o modificarlo 
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
#This file is part of Detección de Hardware de Guadalinex 2005 .
#
#Detección de Hardware de Guadalinex 2005  is free software; you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation; either version 2 of the License, or
#at your option) any later version.
#
#Detección de Hardware de Guadalinex 2005  is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.
#
#You should have received a copy of the GNU General Public License
#along with Foobar; if not, write to the Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA


import os.path
import time

from utils.synaptic import Synaptic
from utils import get_sudo
from deviceactor import DeviceActor


PRINTERICON = os.path.abspath('actors/img/printer.png')
PRINTERICONOFF = os.path.abspath('actors/img/printeroff.png')

class Actor(DeviceActor):

    __required__ = {
        'info.bus': 'usb',
        'info.linux.driver': 'usblp'
    }

    def on_added(self):
        s = Synaptic()
        packages = ['gnome-cups-manager']

        def install_packages():
            if get_sudo():
                s.install(packages)
                open_printer_dialog()

        def open_printer_dialog():
            if get_sudo():
                os.system('sudo /etc/init.d/cupsys restart')
                time.sleep(1)
                os.system('sudo gnome-cups-add &')

        if s.check(packages):
            actions = {"Añadir": open_printer_dialog}
        else:
            actions = {"Instalar los paquetes necesarios": install_packages}

        self.msg_render.show("IMPRESORA", "Impresora detectada",
             PRINTERICON, actions = actions)


    def on_removed(self):
        self.msg_render.show("IMPRESORA", "Impresora desconectada", PRINTERICONOFF)
