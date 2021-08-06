#!/usr/bin/python
# -*- coding: utf-8 -*-

import os.path

from deviceactor import DeviceActor

NETWORKICON = os.path.abspath('actors/img/network.png')
NETWORKICONOFF = os.path.abspath('actors/img/networkoff.png')

class Actor (DeviceActor):

    __required__ = {
    "linux.subsystem":"net"
    }

    def on_added(self):
        interface = self.properties['net.interface']
        self.msg_render.show("Red", 
                "Interfaz de red %s conectada" % interface,
                NETWORKICON)

    def on_removed(self):
        interface = self.properties['net.interface']
        self.msg_render.show("Red", 
                "Interfaz de red %s desconectada" % interface,
                NETWORKICONOFF)

