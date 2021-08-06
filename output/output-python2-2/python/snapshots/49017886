#!/usr/bin/python
# -*- coding: utf-8 -*-

#Módulo volumeactor- Módulo que implementa el "actor hardware" para los
#dispositivos de volumen (dispositivos que se montan como unidades de disco) 
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

import os
import os.path

from deviceactor import DeviceActor
from volume import Actor as VolumeActor
from gettext import gettext as _

VOLUMEICON = os.path.abspath('actors/img/volume.png') 

class Actor(DeviceActor):

    __required__ = {'linux.hotplug_type': 3,
                    'volume.fstype': 'ntfs-3g',
                    'volume.is_disc': False
                    }


    def __init__(self, msgr, devprop):
        DeviceActor.__init__(self, msgr, devprop)

        self.volume_actor = VolumeActor(self.msg_render,
                self.properties)


    def on_added(self):
        self.logger.debug('<#>ntfs on_added</#>')
        block_device = self.properties['block.device']
        os.system('pmount %s' % block_device)

        mount_point = '/media/'
        mount_point += block_device.split('/')[-1]
        self.mount_point = mount_point

        self.properties['volume.mount_point'] = mount_point
        self.properties['volume.is_mounted'] = True

        self.volume_actor.properties = self.properties

        self.volume_actor.on_added()


    def on_removed(self):
        self.logger.debug('<#>ntfs on_removed</#>')
        self.volume_actor.properties = self.properties
        self.volume_actor.on_removed()


    def on_modified(self, key):
        self.logger.debug('<#>ntfs on_modified</#>')
        self.logger.debug('<#> %s </#>' % self.properties['volume.is_mounted'])
        self.volume_actor.properties = self.properties
        self.volume_actor.on_modified(key)



class OldActor (DeviceActor):

    __required__ = {'linux.hotplug_type': 3,
                    'volume.fstype': 'ntfs-3g',
                    'volume.is_disc': False
                    }


    def on_added(self):
        block_device = self.properties['block.device']
        os.system('pmount %s' % block_device)

        def open_volume():
            os.system('nautilus "%s"' % mount_point) 

        mount_point = '/media/'
        mount_point += self.properties['block_device'].split('/')[-1]
        self.message_render.show(_("Storage"), 
             _("Device mounted on"), VOLUMEICON,
             actions = {mount_point: open_volume})


    def on_modified(self, key):
        if key == 'volume.is_mounted' and \
            (not self.properties['volume.is_mounted']):
                self.message_render.show(_("Storage"),
                        _('Device unmounted'), VOLUMEICON)
                





