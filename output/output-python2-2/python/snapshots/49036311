#-*- coding: utf8 -*-

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

import os.path

from deviceactor import DeviceActor

VOLUMEICON = os.path.abspath('actors/img/volume.png') 

class Actor (DeviceActor):

    __required__ = {'info.category': 'volume'}

    #def on_added(self):
    #    self.msg_render.show_info("Dispositivo de volumen conectado")

    def on_modified(self, key):
        if key == 'volume.is_mounted':
            try:
                if self.properties['volume.is_mounted']:
                    mount_point = self.properties['volume.mount_point']

                    def open_volume():
                        os.system('nautilus ' + mount_point) 

                    self.message_render.show("Almacenamiento", 
                        "Dispositivo montado en", VOLUMEICON,
                        actions = {mount_point: open_volume})
                else:
                    self.message_render.show("Almacenamiento", 
                            "Dispositivo desmontado", VOLUMEICON) 

            except Exception, e:
                self.logger.error("Error: " + str(e))

