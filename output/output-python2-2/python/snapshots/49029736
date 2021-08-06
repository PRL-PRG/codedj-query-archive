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

import os, sys, tempfile
import socket
import shutil
import re
import subprocess
from bbutils            import *
from xml.dom.ext.reader import PyExpat  #   python-xml
from xml.xpath          import Evaluate
from Ft.Xml             import MarkupWriter

PPPPEERCONF="/etc/ppp/peers/dsl-bb-assist"
PPPPEERCONFNAME="dsl-bb-assist"

BB_STR_INI = "# Begin: Maintained by bb-assist (please don't remove)"
BB_STR_END = "# End: Maintained by bb-assist (please don't remove)"

def bb_enable_iface_with_config(devcf):
    """
    Configuration of eth interfaces via system-tools-backends.
        
    * Example of static interface configuration via system-tools-backends

    Function call:
    (sysret, xmlcfg, xmlcfgout) = bb_enable_iface_with_config('eth1', 'none', '10.0.0.6',
                                                              '10.255.255.255', '10.0.0.1',
                                                              '255.255.255.0', '10.0.0.0')
    xml passed to system-tools-backends:
    
    <?xml version='1.0' encoding='UTF-8' standalone='yes'?>
    <interface type='ethernet'>
      <configuration>
        <address>10.0.0.8</address>
        <auto>1</auto>
        <bootproto>none</bootproto>
        <broadcast>10.0.0.255</broadcast>
        <file>eth1</file>
        <gateway>10.0.0.1</gateway>
        <netmask>255.255.255.0</netmask>
        <network>10.0.0.0</network>
      </configuration>
      <dev>eth1</dev>
      <enabled>1</enabled>
    </interface>
    <!-- GST: end of request -->
    
    * Example of dhcp interface configuration via system-tools-backend

    Function call:
    (sysret, xmlcfg, xmlcfgout) = bb_enable_iface_with_config('eth1', 'dhcp')

    xml passed to system-tools-backends:
    
    <interface type='ethernet'>
       <configuration>
         <auto>1</auto>
         <bootproto>dhcp</bootproto>
         <file>eth0</file>
       </configuration>
       <dev>eth0</dev>
       <enabled>1</enabled>
    </interface>
    <!-- GST: end of request -->
    
    """

    netcfg_xmlfile = tempfile.NamedTemporaryFile()
    netcfgout_xmlfile = tempfile.NamedTemporaryFile()
    writer = MarkupWriter(netcfg_xmlfile, encoding='UTF-8', indent=u"yes", 
                          standalone="yes")
    writer.startDocument()
    writer.startElement(u'interface')
    writer.attribute(u'type', u'ethernet')
    writer.startElement(u'configuration')
    if devcf.param['dhcp'] == 'True':
        conf = { 'auto' : '1', \
               'bootproto' : 'dhcp', \
               'file': devcf.param['eth_to_conf'] }
    else:
        broadcast = ipBroadcast(devcf.param['ip_computer'],
                                devcf.param['mask_computer'])
        conf = { 'address' : devcf.param['ip_computer'], \
                 'auto' : '1', \
                 'bootproto' :'none', \
                 'broadcast' : broadcast, \
                 'file': devcf.param['eth_to_conf'], \
                 'gateway' : devcf.param['gw_computer'], \
                 'netmask' : devcf.param['mask_computer'], \
                 'network' : devcf.param['net_computer']}
    for confparam in conf.keys():
        writer.startElement(unicode(confparam))
        writer.text(unicode(conf[confparam]))
        writer.endElement(unicode(confparam))
    writer.endElement(u'configuration')
    writer.startElement(u'dev')
    writer.text(unicode(devcf.param['eth_to_conf']))
    writer.endElement(u'dev')
    writer.startElement(u'enabled')
    writer.text(u'1')
    writer.endElement(u'enable')
    writer.endElement(u'interface')
    writer.text(u'\n')
    writer.comment(u' GST: end of request ')
    writer.endDocument()
    netcfg_xmlfile.flush()

    xmlcfg = open(netcfg_xmlfile.name).read( )
    net_cfg_cmd = "cat " + netcfg_xmlfile.name + " | /usr/share/setup-tool-backends/scripts/network-conf -d enable_iface_with_config | grep -vE \"^$\" > " + netcfgout_xmlfile.name
    os.system(net_cfg_cmd)
    xmlcfgout = open(netcfgout_xmlfile.name).read( )
    reader = PyExpat.Reader( )
    dom = reader.fromStream(open(netcfgout_xmlfile.name, "r"))
    successCfg = Evaluate("success/text( )",
                         dom.documentElement)[0].nodeValue
    if successCfg == '0':
        sysret = BBERRCFGDEV # Error configuration dev
    else:
        sysret = BBNOERR # Ok
    return (sysret, xmlcfg, xmlcfgout)

def bb_update_resolvconf(dns1, dns2, sys_output_file):
    """
    Update of dns configuration.
    system-tools-backend don't have an option to only update de resolv.conf and
    ubuntu I think is not using resolvconf package by default. Then I update
    directly /etc/resolv.conf
    """

    # Check first if dns1 and dns2 are IP address for security reasons
    # if not exception
    socket.inet_aton(dns1)
    socket.inet_aton(dns2)

    fileresolv = open("/etc/resolv.conf", "w")
    fileresolv.write("nameserver " + dns1 + "\n")
    fileresolv.write("nameserver " + dns2 + "\n")
    fileresolv.close()
    sys_output_file.write("* /etc/resolv.conf: " +
                          _("Configurado correctamente.") + "\n")

def bb_papchap_conf(PPPuser, PPPpasswd, sys_output_file):
    """
    This configures PPPuser, PPPpasswd in /etc/ppp/{pap-secrets,chap-secrets}
    if the user/passwd exists already updates it with the new info
    """
    # Check the vars
    
    filechap = open('/etc/ppp/pap-secrets', 'a+')
    filepap  = open('/etc/ppp/chap-secrets', 'a+')

    # Erase PPPuser from {pap,chap}-secrets

    usertofind = "\"" + PPPuser + "\""
    
    chapnewtmp = tempfile.NamedTemporaryFile()
    papnewtmp  = tempfile.NamedTemporaryFile()

    for line in filechap:
        if (line.find(usertofind) == -1):
            chapnewtmp.write(line)
        
    for line in filepap:
        if (line.find(usertofind) == -1):
            papnewtmp.write(line)

    # Add PPPuser to {pap,chap}-secrets

    auth = "\"" + PPPuser + "\" * \"" + PPPpasswd + "\"\n"

    chapnewtmp.write(auth)
    papnewtmp.write(auth)
    chapnewtmp.flush()
    papnewtmp.flush()
        
    filechap = open("/etc/ppp/pap-secrets",  "w")
    filepap  = open("/etc/ppp/chap-secrets", "w")
    newchaps = open(chapnewtmp.name,  "r").read()
    newpaps  = open(papnewtmp.name, "r").read()

    for line in newchaps:
        filechap.write(line)
    for line in newpaps:
        filepap.write(line)

    filepap.close()
    filechap.close()
    
    os.chmod("/etc/ppp/pap-secrets", 0600)
    os.chmod("/etc/ppp/chap-secrets", 0600)
    subprocess.call(["chown", "root:root", "/etc/ppp/pap-secrets"],
                    stdout=sys_output_file, stderr=sys_output_file)
    subprocess.call(["chown", "root:root", "/etc/ppp/chap-secrets"],
                    stdout=sys_output_file, stderr=sys_output_file)
    sys_output_file.write("* /etc/ppp/*-secrets: " +
                          _("Configurados correctamente.") + "\n")

def bb_deb_conf_net_interfaces(str_to_add, sys_output_file, hotplug_int_tomap = '', ifaceToConf = ''):
    """
    clears and conf /etc/network/interfaces in debian/ubuntu system
    from previous configurations of the assistant

    Comments the iface 'ifaceToConf' and add to the map section the
    interface 'hotplug_int_to_map'
    """
    
    fileint = open('/etc/network/interfaces', 'r')
        
    try: shutil.copyfile('/etc/network/interfaces.seg.2', '/etc/network/interfaces.seg.3')
    except: pass
    try: shutil.copyfile('/etc/network/interfaces.seg.1', '/etc/network/interfaces.seg.2')
    except: pass
    shutil.copyfile('/etc/network/interfaces', '/etc/network/interfaces.seg.1')
    
    str_ini_tofind = "Begin: Maintained by bb-assist"
    str_end_tofind = "End: Maintained by bb-assist"
    
    interfacestmp = tempfile.NamedTemporaryFile()

    in_configuration = False
    for line in fileint:
        if (line.find(str_ini_tofind) == -1 and \
            in_configuration == False):
            interfacestmp.write(line)
        elif (line.find(str_end_tofind) != -1):
            in_configuration = False
        else: # I found a configuration
            in_configuration = True
        
    interfacestmp.flush()
        
    newinterfaces_uncomment = open(interfacestmp.name, "r").readlines()

    # Commment old ifaces of the ifaceToConf
    newinterfaces = []
    in_iface = False
    for line in newinterfaces_uncomment:
        if (re.search("^[\s]*auto[\s]+%s[\s]*$" % ifaceToConf, line) != None):
            newinterfaces += ["#" + line]
            continue
        # Match "auto lo ethToConf" for example
        auto_re = "^[\s]*auto[\s]+([a-z0-9:]+[\s]+)*%s[\s]*$" % ifaceToConf
        if (re.search(auto_re, line) != None):
            newinterfaces += re.sub("[\s]%s$" % ifaceToConf, "", line)
            continue
        # Match "auto lo ethToConf eth1" for example
        auto_re = "^[\s]*auto[\s]+([a-z0-9:]+[\s]+)*%s[^\d][\s]*" % ifaceToConf
        if (re.search(auto_re, line) != None):
            # remove auto of the interface to conf
            newinterfaces += re.sub("[\s]%s[\s]+" % ifaceToConf, " ", line)
            continue
        if (re.search("^[\s]*iface[\s]+%s(:[\d]+)?[\s]+(inet|ipx|inet6)[\s]+" % ifaceToConf, line) != None):
            newinterfaces += ["#" + line]
            in_iface = True
            continue
        if (in_iface and re.search("^[\s]*(address|netmask|gateway|network|broadcast|metric|media|pointopoint|hwaddress|mtu|pre-up|up|post-up|provider|down|pre-down|post-down)[\s]", line) != None):
            newinterfaces += ["#" + line]
            continue
        if (in_iface and re.search("^[\s]*#", line) != None):
            newinterfaces += [line]
            continue
        newinterfaces += [line]
        in_iface = False

    hotplug_section = False
    fileint = open('/etc/network/interfaces', 'w')
    for line in newinterfaces:
        if len(hotplug_int_tomap) != 0:
            # We are trying to do this:
            # mapping hotplug
            #    script grep
            # # Mark of bb-assist
            #    map $hotplug_int_tomap
            # # Mark of bb-assist
            #    map eth0
            if line.find("mapping hotplug") != -1:
                hotplug_section = True
            if hotplug_section and line.find("script grep") != -1:
                fileint.write(line)
                hotplug_section = False
                fileint.write(BB_STR_INI + "\n")
                fileint.write("\tmap " + hotplug_int_tomap + "\n")
                fileint.write(BB_STR_END + "\n")
                continue
        fileint.write(line)

    # I add the new part (if exist)
    for line in str_to_add:
        fileint.write(line + "\n")
    fileint.close()
        
    os.chmod("/etc/network/interfaces", 0644)
    retsys = subprocess.call(["chown", "root:root", "/etc/network/interfaces"],
                             stdout=sys_output_file, stderr=sys_output_file)
    if (retsys == 0):
        sys_output_file.write("* /etc/network/interfaces: " +
                              _("Configurado correctamente.") + "\n")
        return BBNOERR
    else:
        return BBERRIFACES        

def bb_create_ppp_peer_conf(devcf, sys_output_file):
    """
    conf ppp peer pppoe/pppoa configuration
    """
    
    peerfile = open(PPPPEERCONF, 'w')
    
    if (devcf.param['ppp_noipdefault']):
        peerfile.write('noipdefault\n')
    if devcf.param['ppp_defaultroute']:
        peerfile.write('defaultroute\n')
    if devcf.param['ppp_replacedefaultroute']:
        peerfile.write('replacedefaultroute\n')
    if devcf.param['ppp_usepeerdns']:
        peerfile.write('usepeerdns\n')
    if devcf.param['ppp_noauth']:
        peerfile.write('noauth\n')
    if devcf.param['ppp_updetach']:
        peerfile.write('updetach\n')
    peerfile.write("user \'" + devcf.param['PPPuser'] + "'\n")
    if devcf.param['ppp_proto'] == 'PPPoE':
        peerfile.write('plugin rp-pppoe.so')
        if devcf.device_type.dt_id == '0001': # modems
            if (devcf.id == '0003') or (devcf.id == '0006'):
                # Alcatel Speedtouch and Vitelcom
                peerfile.write('\nnas0\n') # an extra newline
        if devcf.device_type.dt_id == '0002': # routers
            peerfile.write(" " + devcf.param['eth_to_conf'] + '\n')
    if devcf.param['ppp_proto'] == 'PPPoA':
        peerfile.write('plugin pppoatm.so\n')
        peerfile.write(devcf.param['vpi'] + "." +
                       devcf.param['vci'] + '\n')
    if devcf.param['ppp_persist']:
        peerfile.write('persist\n')
    if devcf.param['ppp_novjccomp']:
        peerfile.write('novjccomp\n')
    if devcf.param['ppp_noaccomp']:
        peerfile.write('noaccomp\n')
    if devcf.param['ppp_nobsdcomp']:
        peerfile.write('nobsdcomp\n')
    if devcf.param['ppp_nodeflate']:
        peerfile.write('nodeflate\n')
    if devcf.param['ppp_nopcomp']:
        peerfile.write('nopcomp\n')
    if devcf.param['ppp_noccp']:
        peerfile.write('noccp\n')
    if devcf.param['ppp_novj']:
        peerfile.write('novj\n')
    peerfile.write('maxfail ' + devcf.param['ppp_maxfail'] + '\n')
    peerfile.write('mru ' + devcf.param['ppp_mru'] + '\n')
    peerfile.write('mtu ' + devcf.param['ppp_mtu'] + '\n')
    if (devcf.param['ppp_lcpechofailure'] != '0'):
        peerfile.write('lcp-echo-failure ' +
                       devcf.param['ppp_lcpechofailure'] + '\n')
    if (devcf.param['ppp_lcpechointerval'] != '0'):
        peerfile.write('lcp-echo-interval ' +
                       devcf.param['ppp_lcpechointerval'] + '\n')
    if devcf.param['ppp_debug']:
        peerfile.write('debug\n')
    peerfile.close()
    os.chmod(PPPPEERCONF, 0640)
    retsys = subprocess.call(["chown", "root:dip", PPPPEERCONF],
                             stdout=sys_output_file, stderr=sys_output_file)
    sys_output_file.write("* " + PPPPEERCONF + ": " +
                          _("Configurado correctamente.") + "\n")
    if (retsys == 0):
        return BBNOERR
    else:
        return BBERRPPPCONF

def bb_eagleconf(devcf, sys_output_file):
    if devcf.param['mod_conf'] == "monodinamic":
        isp = 'ES01'
        enc = 1
        login = devcf.param['PPPuser']
        passwd = devcf.param['PPPpasswd']
        staticIP = 'none'
        updatedns = int(devcf.param['ppp_usepeerdns'])
        startonboot = int(devcf.param['ppp_startonboot'])
    else:
        isp = 'ES02'
        enc = 3
        login = 'none'
        passwd = 'none'
        staticIP = devcf.param['ip_computer']
        updatedns = 0
        startonboot = 1
    pwd_enc = '1' # FIXME: test this
    param = " '--params=LINETYPE=00000001|VPI=%07x|VCI=%08x|ENC=%08d|ISP=%s|ISP_LOGIN=%s|ISP_PWD=%s|PWD_ENCRYPT=%s|STATIC_IP=%s|UPDATE_DNS=%d|START_ON_BOOT=%d|USE_TESTCONNEC=0|EU_LANG=|FORCE_IF=auto|CMVEI=WO|CMVEP=WO'" % (int(devcf.param['vpi']), int(devcf.param['vci']), enc, isp, login, passwd, pwd_enc, staticIP, updatedns, startonboot)
    retsys = subprocess.call("/usr/sbin/eagleconfig" + param, shell=True,
                             stdout=sys_output_file, stderr=sys_output_file)
    if (retsys == 0):
        if os.path.exists("/etc/eagle-usb/eagle-usb_must_be_configured"):
            os.remove("/etc/eagle-usb/eagle-usb_must_be_configured")
        return BBNOERR
    else:
        return BBERREAGLE
    
def bb_ppp_conf(devcf, sys_output_file):
    """
    Configures a pppoe/pppoa in a simple way depending also on the provider
    selected in the GUI
    Eagle modems use eagleconf.
    """
    
    if (devcf.id == '0002'):
        retsys = bb_eagleconf(devcf, sys_output_file)
    else:
        bb_papchap_conf(devcf.param['PPPuser'], devcf.param['PPPpasswd'], sys_output_file)
        retsys = bb_create_ppp_peer_conf(devcf, sys_output_file)
    return retsys

def ipOverAtmIntStr(devcf):
    broad = ipBroadcast(devcf.param['ip_computer'],
                        devcf.param['mask_computer'])
    gw = devcf.param['gw_computer']

    # FIXME: In the future is better to let the user choose in the UI between atm interfaces
    # already configured
    atmint = 'atm0'
    iface  = [BB_STR_INI]
    iface += ['auto %s' % atmint] # Fixme: obtain a free atm interface
    iface += ['iface %s inet static' % atmint]
    iface += ['  address %s' % devcf.param['ip_computer']]
    iface += ['  netmask %s' % devcf.param['mask_computer']]
    iface += ['  broadcast %s' % broad]
    iface += ['  gateway %s' % gw]
    iface += ['  pre-up route del default || exit 0']
    iface += ['  pre-up pgrep atmarpd 2>&1 && /etc/init.d/atm start 2>&1 | /usr/bin/logger -t bb-assist']
    iface += ['  pre-up ifconfig %s 2>&1 || atmarp -c %s 2>&1 | /usr/bin/logger -t bb-assist' % (atmint, atmint)]
    iface += ['  up atmarp -s %s 0.%s.%s || exit 0 | /usr/bin/logger -t bb-assist' % (gw, devcf.provider.vpi, devcf.provider.vci)]
    iface += ['  up echo Interface $IFACE going up | /usr/bin/logger -t bb-assist']
    iface += ['  down echo Interface $IFACE going down | /usr/bin/logger -t bb-assist']
    iface += [BB_STR_END]
    return iface

def ethIntStr(devcf):
    ethint = devcf.param['eth_to_conf']

    iface  = [BB_STR_INI]
    iface += ['auto %s' % ethint]

    if devcf.param['dhcp'] == 'False':
        broad = ipBroadcast(devcf.param['ip_computer'],
                            devcf.param['mask_computer'])    
        iface += ['iface %s inet static' % ethint]
        iface += ['  address %s' % devcf.param['ip_computer']]
        iface += ['  netmask %s' % devcf.param['mask_computer']]
        iface += ['  network %s' % devcf.param['net_computer']]
        iface += ['  broadcast %s' % broad]
        iface += ['  gateway %s' % devcf.param['gw_computer']]
    else:
        iface += ['iface %s inet dhcp' % ethint]
    iface += ['  up echo Interface $IFACE going up | /usr/bin/logger -t bb-assist']
    iface += ['  down echo Interface $IFACE going down | /usr/bin/logger -t bb-assist']
    iface += [BB_STR_END]
    return iface

def pppIntStr(devcf):
    # Finxe, put the right encoding 
    iface  = [BB_STR_INI]
    iface += ['iface %s inet ppp' % PPPPEERCONFNAME]
    if devcf.param['ppp_proto'] == 'PPPoE':
        iface += ['  pre-up modprobe -q pppoe']
    if devcf.param['ppp_proto'] == 'PPPoA':
        iface += ['  pre-up modprobe -q pppoatm']
    if devcf.param['ppp_proto'] == 'PPPoE' and devcf.device_type.dt_id == '0001':
        iface += ['  pre-up modprobe -q br2684'] # PPPoE - USB
        iface += ['  pre-up pgrep br2684ctl >/dev/null 2>&1 ; if [[ $? -eq 0 ]] ; then pkill br2684ctl; fi']
        # FIXME Encap depends on provider
        iface += ['  pre-up br2684ctl -b -c 0 -a 0.%s.%s -e 1 2>&1 | /usr/bin/logger -t bb-assist' % 
                  (devcf.provider.vpi, devcf.provider.vci)]
        iface += ['  pre-up /sbin/ifconfig nas0 up 2>&1 | /usr/bin/logger -t bb-assist']
    if devcf.param['ppp_proto'] == 'PPPoE' and devcf.device_type.dt_id == '0002':
        iface += ['  pre-up (/sbin/ifconfig %s up 2>&1 || exit 0) | /usr/bin/logger -t bb-assist' \
                  % devcf.param['eth_to_conf']]
    iface += ['  provider %s' % PPPPEERCONFNAME]
    if devcf.param['ppp_proto'] == 'PPPoE' and devcf.device_type.dt_id == '0001':
        iface += ['  post-down pgrep br2684ctl >/dev/null 2>&1 ; if [[ $? -eq 0 ]] ; then pkill br2684ctl; fi'] # PPPoE - USB
        iface += ['  pre-down /sbin/ifconfig nas0 down 2>&1 | /usr/bin/logger -t bb-assist']
    iface += [BB_STR_END]
    return iface

def create_default_hotplug_usb_conf(device, net_if, pppd_peer, sys_output_file):
    file_to_conf = "/etc/default/%s" % device

    conf  = ['# %s kernel module options with hotplug' % device]
    conf += ['']
    conf += ['# IP over ATM']
    conf += ['#']
    conf += ['# NET_IF="atm0"']
    conf += ['# You also need a correct /etc/network/interfaces defining atm0']
    conf += ['# See /usr/share/doc/cxacru/ for examples']
    conf += ['']
    conf += ['NET_IF="%s"' % net_if]
    conf += ['']
    conf += ['# PPPoA/PPPoE configuration']
    conf += ['# PPPD_PEER="/etc/ppp/peer/dsl-example']
    conf += ['']
    conf += ['PPPD_PEER="%s"' % pppd_peer]

    filedef = open(file_to_conf, "w")
    for line in conf:
        filedef.write(line + "\n")
    filedef.close()
        
    os.chmod(file_to_conf, 0644)
    subprocess.call(["chown", "root:root", file_to_conf],
                    stdout=sys_output_file, stderr=sys_output_file)
    sys_output_file.write("* " + file_to_conf + ": " +
                          _("Configurado correctamente.") + "\n")

def bb_ifup(dev, sys_output_file):
    retsys = subprocess.call(["ifup", dev], stdout=sys_output_file,
                             stderr=sys_output_file)
    if (retsys == 0):
        sys_output_file.write("* Interface " + dev + " " +
                              _("levantado correctamente.") + "\n")
        return BBNOERR
    else:
        return BBERRIFACES

def bb_ifdown(dev, sys_output_file):
    retsys = subprocess.call(["ifdown", dev], stdout=sys_output_file,
                             stderr=sys_output_file)
    if (retsys == 0):
        sys_output_file.write("* Interface " + dev + " " +
                              _("bajado correctamente.") + "\n")
        return BBNOERR
    else:
        return BBERRIFACES
    
def conf_pc_lan(devcf, sys_output_file):
    retsys = BBNOERR
    if devcf.device_type.dt_id == '0001': # USB
        if devcf.param['mod_conf'] == "monostatic":
            # IP over ATM (RFC 1483 routed)
            if devcf.id == '0003' or devcf.id == '0006':
                create_default_hotplug_usb_conf(devcf.linux_driver, 'atm0', '', sys_output_file)
                retsys = bb_deb_conf_net_interfaces(ipOverAtmIntStr(devcf), sys_output_file, 'atm0', 'atm0')
                bb_update_resolvconf(devcf.param['dns1'], devcf.param['dns2'], sys_output_file)
                if (retsys == BBNOERR):
                    bb_ifdown('atm0',  sys_output_file)
                    retsys = bb_ifup('atm0',  sys_output_file)
            if devcf.id == '0002':
                retsys = bb_eagleconf(devcf, sys_output_file)
                bb_update_resolvconf(devcf.param['dns1'], devcf.param['dns2'], sys_output_file)
        elif devcf.param['mod_conf'] == "monodinamic":
            # PPPoE/PPPoA
            retsys = bb_ppp_conf(devcf, sys_output_file)
            if retsys == BBNOERR and (devcf.id == '0003' or devcf.id == '0006'):
                # eagle-usb uses eagleconfig
                create_default_hotplug_usb_conf(devcf.linux_driver, '', PPPPEERCONFNAME,
                                                sys_output_file)
                retsys = bb_deb_conf_net_interfaces(pppIntStr(devcf), sys_output_file,
                                                    PPPPEERCONFNAME, PPPPEERCONFNAME)
                if (retsys == BBNOERR):
                    bb_ifdown(PPPPEERCONFNAME, sys_output_file)
                    retsys = bb_ifup(PPPPEERCONFNAME, sys_output_file)
    if devcf.device_type.dt_id == '0002': # routers    
        if devcf.param['mod_conf'] == "monostatic":
            # DHCP (the server in the router has a one address pool)
            retsys = bb_deb_conf_net_interfaces(ethIntStr(devcf), sys_output_file, '',
                                                devcf.param['eth_to_conf'])
            if (retsys == BBNOERR):
                retsys = bb_ifdown(devcf.param['eth_to_conf'], sys_output_file)
            if (retsys == BBNOERR):
                retsys = bb_ifup(devcf.param['eth_to_conf'], sys_output_file)
        elif devcf.param['mod_conf'] == "monodinamic":
            # Conf PPPoE/PPPoA interface and ppp/peer
            retsys = bb_deb_conf_net_interfaces(ethIntStr(devcf) + pppIntStr(devcf),
                                                sys_output_file, '', devcf.param['eth_to_conf'])
            if (retsys == BBNOERR):
                retsys = bb_ppp_conf(devcf, sys_output_file)
            # The ethernet of the PC
            if (retsys == BBNOERR):
                bb_ifdown(devcf.param['eth_to_conf'],  sys_output_file)
                retsys = bb_ifup(devcf.param['eth_to_conf'],  sys_output_file)
                if (retsys == BBNOERR):
                    retsys = bb_ifup(PPPPEERCONFNAME, sys_output_file)
        elif devcf.param['mod_conf'] == "multistatic" or \
                 devcf.param['mod_conf'] == "multidinamic":
            if devcf.param['dhcp'] == 'True':
                # Configure Ethernet with DHCP
                retsys = bb_deb_conf_net_interfaces(ethIntStr(devcf), sys_output_file, '',
                                                    devcf.param['eth_to_conf'])
            else:
                # Configure Ethernet Computer (also the DNS)
                retsys = bb_deb_conf_net_interfaces(ethIntStr(devcf), sys_output_file, '',
                                                    devcf.param['eth_to_conf'])
                bb_update_resolvconf(devcf.param['dns1'], devcf.param['dns2'], sys_output_file)
            bb_ifdown(devcf.param['eth_to_conf'],  sys_output_file)
            if (retsys == BBNOERR):
                retsys = bb_ifup(devcf.param['eth_to_conf'],  sys_output_file)
    return retsys

if __name__ == "__main__":
    pass
