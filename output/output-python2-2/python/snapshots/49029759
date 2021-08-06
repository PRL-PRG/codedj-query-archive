#!/usr/bin/env python
# -*- coding: UTF8 -*-

"""
bb-assist - A Broadband Assistant Configurator

Copyright (C) 2005 Junta de Andalucía

Autor/es (Author/s):

- Vicente J. Ruiz Jurado <vjrj@tid.es>

Este fichero es parte de bb-assist.

bb-assist es software libre. Puede redistribuirlo y/o modificarlo
bajo los términos de la Licencia Pública General de GNU según es
publicada por la Free Software Foundation, bien de la versión 2 de dicha
Licencia o bien (según su elección) de cualquier versión posterior.
 
bb-assist se distribuye con la esperanza de que sea útil,
pero SIN NINGUNA GARANTÍA, incluso sin la garantía MERCANTIL
implícita o sin garantizar la CONVENIENCIA PARA UN PROPÓSITO
PARTICULAR. Véase la Licencia Pública General de GNU para más detalles.

Debería haber recibido una copia de la Licencia Pública General
junto con bb-assist. Si no ha sido así, escriba a la Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.

-------------------------------------------------------------------------

This file is part of bb-assist.

bb-assist is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
at your option) any later version.

bb-assist is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Foobar; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
"""
import sys, os, tempfile, time
import socket, struct
import dbus
from xml.dom.ext.reader import PyExpat
from xml.xpath          import Evaluate

# Errors
BBNOERR, BBERRGEN, BBERREOF, BBERRTOUT, BBERRPWD, BBERRDHCP, BBERRLOCK, BBERRCFGDEV, BBERREAGLE, BBERRPPPCONF, BBERRIFACES = (0,1,2,3,4,5,6,7,8,9,10)

# Paths
PATH = '/usr/share/bb-assist/'
#PATH = './'
GLADEDIR = os.path.join(PATH, 'glade')
PIXMAPSDIR = os.path.join(GLADEDIR, 'pixmaps')

def searchPathXml(search_path, xmlfile):
    reader = PyExpat.Reader( )
    xml_file = open(os.path.join(PATH, xmlfile), "r")
    dom = reader.fromStream(xml_file)
    return Evaluate(search_path, dom.documentElement)

def boldme(string):
    return "<b>" + string + "</b>"

class provider:
    def __init__(self, xpath_devnode = None):
        if xpath_devnode == None:
            self.prov_id = ""
            self.prov_name = ""
            self.shortname = ""
            self.dns1 = ""
            self.dns2 = ""
            self.vpi = ""
            self.vci = ""
            self.pppoe = ""
            self.pppoa = ""
            self.encap = ""
        else:
            self.prov_id   = xpath_devnode.getAttribute("id")
            self.prov_name = xpath_devnode.getAttribute("name")
            self.shortname = xpath_devnode.getAttribute("shortname")
            self.dns1 = Evaluate("dns1/text( )", xpath_devnode)[0].nodeValue
            self.dns2 = Evaluate("dns2/text( )", xpath_devnode)[0].nodeValue
            self.vpi = Evaluate("vpi/text( )", xpath_devnode)[0].nodeValue
            self.vci = Evaluate("vci/text( )", xpath_devnode)[0].nodeValue
            self.pppoe = Evaluate("pppoe/text( )", xpath_devnode)[0].nodeValue
            self.pppoa = Evaluate("pppoa/text( )", xpath_devnode)[0].nodeValue
            self.encap = Evaluate("encap/text( )", xpath_devnode)[0].nodeValue
    def __str__(self):      
        return "%s %s" % (self.prov_id, self.prov_name)

class tty_conf:
    def __init__(self, id_tty = ""):
        if id_tty == "":
            self.tty_id = ""
            self.tty_baudrate = ""
            self.tty_bits = ""
            self.tty_parity = ""
            self.tty_stopbits = ""
            self.tty_xonxoff = ""
            self.tty_rtscts = ""
        else:
            path_tty = "tty_conf[@id=" + id_tty + "]"
            devnode = searchPathXml(path_tty, 'ttyconf.xml')
            if (len(devnode) <> 1):
                raise SyntaxError, _("en tty_conf")
            self.tty_id = id_tty
            self.tty_baudrate = devnode[0].getAttribute("baudrate")
            self.tty_bits = devnode[0].getAttribute("bits")
            self.tty_parity = devnode[0].getAttribute("parity")
            self.tty_stopbits = devnode[0].getAttribute("stopbits")
            self.tty_xonxoff = devnode[0].getAttribute("xonxoff")
            self.tty_rtscts = devnode[0].getAttribute("rtscts")
    def __str__(self):      
        return "%s %s" % (self.tty_id, self.tty_baudrate)

class devicetype:
    def __init__(self, id_devtype = ""):
        if id_devtype == "":
            self.dt_id = ""
            self.dt_name = ""
        else:
            self.dt_id = id_devtype
            path_dt = "devicetype[@id=" + id_devtype + "]"
            devnode = searchPathXml(path_dt, 'devicetypes.xml')
            if (len(devnode) <> 1):
                raise SyntaxError, _("en devicetype")
            self.dt_name = devnode[0].getAttribute("name")
    def __str__(self):      
        return "%s %s" % (self.dt_id, self.dt_name)

class bb_device:
    def __init__(self, devnode = None, devprovider = None):
        if devnode == None:
            self.devnode = None
            self.name = None
            self.id = None
            self.console = None
            self.can_be_eth_conf = None
            self.can_be_tty_conf = None
            self.support = None
            self.device_type = None
            self.tty_conf = None
            self.provider = provider()
        else:
            self.devnode = devnode
            self.name = devnode.getAttribute("name")
            self.id = devnode.getAttribute("id")
            self.image = devnode.getAttribute("image")
            self.console = Evaluate("console/text( )", devnode)[0].nodeValue
            self.support = Evaluate("support/text( )", devnode)[0].nodeValue
            self.can_be_eth_conf = int(Evaluate("can_be_eth_conf/text( )",
                                                devnode)[0].nodeValue)
            self.can_be_tty_conf = int(Evaluate("can_be_tty_conf/text( )",
                                                devnode)[0].nodeValue)
            self.device_type = devicetype(Evaluate("device_type",
                                                   devnode)[0].getAttribute("id"))
            if self.device_type.dt_id == '0001':
                #DSL USB
                self.tty_conf = None
                self.linux_driver = Evaluate("usb_conf/linux_driver/text( )",
                                             devnode)[0].nodeValue
                devidnodes = Evaluate("usb_conf/deviceid_list/deviceid",
                                       devnode)
                self.devids = []
                for devidnode in devidnodes:
                    vendor = devidnode.getAttribute("vendor")
                    model = devidnode.getAttribute("model")
                    self.devids += [(vendor, model)]
            else:
                #DSL Router
                self.tty_conf = tty_conf(Evaluate("tty_conf",
                                                  devnode)[0].getAttribute("id"))
                self.eth_conf_port = Evaluate("eth_conf_port/text( )", devnode)[0].nodeValue
            if devprovider != None:
                self.provider = devprovider
                defpass_node = Evaluate("provider_list/provider[@id=" +
                                        devprovider.prov_id +
                                        "]/default_passwd_list/" +
                                        "default_passwd[@id='0001']",
                                        devnode)
                if len(defpass_node) > 0:
                    self.default_passwd = defpass_node[0].getAttribute("passwd")
                else:
                    self.default_passwd = None
            else:
                self.provider = provider()
                self.default_passwd = None
                            
    def __str__(self):
        return "%s %s %s %s %s (%s) (%s)" % \
                (self.name, self.id, self.console, self.support,
                 self.device_type, self.tty_conf, self.provider)

class bb_device_conf(bb_device):
    __bb = bb_device
    def __init__(self, ini_bb_dev = None):
        if ini_bb_dev:
            self.__bb.__init__(self, ini_bb_dev.devnode, ini_bb_dev.provider)
        else:
            self.__bb.__init__(self)
        # Dynamic parameters
        self.param = {}
        
class operation:
    def __init__(self, opernode = None):
        if opernode == None:
            self.opernode = None
            self.id = None
            self.bb_device = None
            self.supported = 0
            self.ui_public = 0
            self.opername = None
            self.firmware = None
            self.initial_func = None
            self.default_timeout = None
            self.send_delay = None
            self.druid_page_list = []
        else:
            self.opernode = opernode
            self.id = opernode.getAttribute("id")
            self.bb_device = opernode.getAttribute("bb_device")
            self.supported = opernode.getAttribute("supported")
            self.ui_public = opernode.getAttribute("ui_public")
            self.opername = {}
            for opername_node in Evaluate("opername_list/opername", opernode):
                self.opername[opername_node.getAttribute("lang")] = opername_node.getAttribute("name")
            self.firmware = Evaluate("firmware/text( )",
                                     opernode)[0].nodeValue
            self.initial_func = Evaluate("initial_func/text( )",
                                         opernode)[0].nodeValue
            self.default_timeout = Evaluate("default_timeout/text( )",
                                            opernode)[0].nodeValue
            self.send_delay = Evaluate("send_delay/text( )",
                                            opernode)[0].nodeValue
            self.druid_page_list = []
            for druid_page_node in Evaluate("druid_page_list/druid_page/text( )",
                                            opernode):
                self.druid_page_list += [druid_page_node.nodeValue]
    def __str__(self):      
        return "%s %s %s %s %s %s %s %s %s" % \
               (self.id, self.bb_device, self.supported, self.ui_public, self.opername,
                self.firmware, self.initial_func, self.default_timeout, self.druid_page_list)

# IP address functions

def ipAddress_atol(ipAddress):
    """
    Obtains a address in long format, from a string
    """
    try: ret = struct.unpack('>L',socket.inet_aton(ipAddress))[0]
    except:
        raise socket.error, _("en modulo bbutils. Dirección IP incorrecta")
    return ret

def ipAddress_ltoa(ipAddress):
    """
    Obtains a address in string format, from a long
    """
    try: ret = socket.inet_ntoa(struct.pack('>L', ipAddress))
    except:
        raise socket.error, _("en modulo bbutils. Dirección IP incorrecta")
    return ret

def ipAddress_incr(ipAddress, increment):
    """
    Calculate: ipAddress + increment
    """
    return ipAddress_ltoa(ipAddress_atol(ipAddress) + increment)
    
def ipMasqIncrem(ipAddress, ipMask, increment):
    """
    Calculate: (ipAddress AND ipMask) + increment
    """
    return (socket.inet_ntoa((struct.pack('>L',
                                          (struct.unpack('>L',socket.inet_aton(ipAddress))[0] & 
                                           struct.unpack('>L',socket.inet_aton(ipMask))[0]) +
                                          increment))))

def ipNet(ipAddress, ipMask):
    """
    Calculate: (ipAddress AND ipMask)
    """
    return (socket.inet_ntoa((struct.pack('>L',
                                          (struct.unpack('>L',socket.inet_aton(ipAddress))[0] & 
                                           struct.unpack('>L',socket.inet_aton(ipMask))[0])))))
def ipBroadcast(ipAddress, ipMask):
    return ipAddress_ltoa(ipAddress_atol(ipAddress) | (ipAddress_atol('255.255.255.255') - ipAddress_atol(ipMask)))
