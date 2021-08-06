#-*- coding: utf8 -*-

#MÃ³dulo volumeactor- MÃ³dulo que implementa el "actor hardware" para los
#dispositivos de volumen (dispositivos que se montan como unidades de disco) 
#
#Copyright (C) 2005 Junta de AndalucÃ­a
#
#Autor/es (Author/s):
#
#- Gumersindo Coronel PÃ©rez <gcoronel@emergya.info>
#
#Este fichero es parte de DetecciÃ³n de Hardware de Guadalinex 2005 
#
#DetecciÃ³n de Hardware de Guadalinex 2005  es software libre. Puede redistribuirlo y/o modificarlo 
#bajo los tÃ©rminos de la Licencia PÃºblica General de GNU segÃºn es 
#publicada por la Free Software Foundation, bien de la versiÃ³n 2 de dicha
#Licencia o bien (segÃºn su elecciÃ³n) de cualquier versiÃ³n posterior. 
#
#DetecciÃ³n de Hardware de Guadalinex 2005  se distribuye con la esperanza de que sea Ãºtil, 
#pero SIN NINGUNA GARANTÃA, incluso sin la garantÃ­a MERCANTIL 
#implÃ­cita o sin garantizar la CONVENIENCIA PARA UN PROPÃSITO 
#PARTICULAR. VÃ©ase la Licencia PÃºblica General de GNU para mÃ¡s detalles. 
#
#DeberÃ­a haber recibido una copia de la Licencia PÃºblica General 
#junto con DetecciÃ³n de Hardware de Guadalinex 2005 . Si no ha sido asÃ­, escriba a la Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
#
#-------------------------------------------------------------------------
#
#This file is part of DetecciÃ³n de Hardware de Guadalinex 2005 .
#
#DetecciÃ³n de Hardware de Guadalinex 2005  is free software; you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation; either version 2 of the License, or
#at your option) any later version.
#
#DetecciÃ³n de Hardware de Guadalinex 2005  is distributed in the hope that it will be useful,
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
    #    self.msg_render.show_info(_("Volume device attached"))

    def on_modified(self, key):
        if key == 'volume.is_mounted':
            try:
                if self.properties['volume.is_mounted']:
                    mount_point = self.properties['volume.mount_point']

                    def open_volume():
                        os.system('nautilus ' + mount_point) 

                    self.message_render.show(_("Storage"), 
                        _("Device mounted on"), VOLUMEICON,
                        actions = {mount_point: open_volume})
                else:
                    self.message_render.show(_("Storage"), 
                            _("Device unmounted"), VOLUMEICON) 

            except Exception, e:
                self.logger.error(_("Error:") + " " + str(e))

