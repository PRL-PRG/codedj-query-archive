# -*- coding: utf8 -*-

#MÃ³dulo usbdeviceactor - MÃ³dulo que implementa el "actor hardware" para los
#dispositivos de usb device. 
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

USBICON = os.path.abspath('actors/img/usb.png')
USBICONOFF = os.path.abspath('actors/img/usboff.png')

class Actor(DeviceActor):

    __required__ = {'info.bus' : 'usb_device'}

    def on_added(self):
        try:
            product = self.properties['usb_device.product']
            vendor = self.properties['info.vendor']
            vendor = vendor and vendor + ', ' or ''
            self.msg_render.show(_("USB"), _("USB device detected:\n") +\
                    vendor + product, USBICON)
            self.vendorproduct = vendor + product

        except:
            self.msg_render.show(_("USB"), _("USB device detected"), USBICON)


    def on_removed(self):
        try:
            self.msg_render.show(_("USB"), _("Usb device disconnected:\n") + \
                    self.vendorproduct, USBICONOFF)
        except:
            self.msg_render.show(_("USB"), _("USB device disconnected"),
                    USBICONOFF)
