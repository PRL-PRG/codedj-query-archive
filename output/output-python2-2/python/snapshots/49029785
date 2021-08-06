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

import os, sys, time
import pexpect                          # Depends: python-pexpect
import serial                           #   python-serial
import re
from bbutils            import *
from serial.serialutil  import *
from xml.dom.ext.reader import PyExpat  #   python-xml
from xml.xpath          import Evaluate

CMDENCODING='iso-8859-1'

def sendline_delayed(child, string, delay=0):
    """This sends a string to the expect child
    and returns the number of bytes written.
    """
    s = string
    total_act = len(s)
    total_bytes = 0
    while total_act > 0:
        n = child.send(s[0])
        if n >= 1:
            s = s[1:]
            total_act -= n
        elif n == 0:
            pass
        total_bytes += n
        time.sleep(float(delay)/1000.0)
    os.write(child.child_fd, os.linesep)
    return total_bytes

def func_parse(dom, child, func, default_timeout = -1, sdelay = 0):
    """ Parse a func in the xml and process every cmd in
    the function.
    """
    path_cmd = "cmd_func[@id='"+ func +"']/cmd"
    cmds = Evaluate(path_cmd, dom.documentElement)
    if (len(cmds) < 1):
        raise SyntaxError, _("en modulo de expect. Función call") + \
              " '" + func + "' " + _("desconocida")
    return (cmd_parse(dom, child, cmds, default_timeout, sdelay))

def cmd_parse(dom, child, cmds, default_timeout = -1, sdelay = 0):
    """ Process a command. There are several types of commands:
    - send: send a command to a tty/telnet
    - return: return a result
    - call: calls to another function
    """
    for actcmd in cmds:
        # regexp, with escaped chars
        send_cmd = actcmd.getAttribute('send').decode('string_escape')
        act_exp_ok = actcmd.getAttribute('exp_ok').decode('string_escape')
        act_except = actcmd.getAttribute('on_except') # on error call func
        err = actcmd.getAttribute('err').decode('string_escape')
        ret_cmd = actcmd.getAttribute('return') # returns a value
        call_cmd = actcmd.getAttribute('call')  # call a function
        
        if ret_cmd: type_cmd = 'return'
        elif call_cmd: type_cmd = 'call'
        else: type_cmd = 'sendline'

        if type_cmd == 'sendline':
            if (sdelay != 0):
                sendline_delayed(child, send_cmd, sdelay)
            else:
                child.sendline(send_cmd)
            expect_list = []
            cmdid_list = []
            if err != '':
                expect_list += [re.compile(err.encode(CMDENCODING))]
                if act_except != '':
                    cmdid_list += [act_except]
                else:
                    cmdid_list += ['__exit__']
            expect_list += [re.compile(act_exp_ok.encode(CMDENCODING))]
            cmdid_list += ['__plus1__']
            
            expects = Evaluate("expect_list/expect", actcmd)
            
            for expectact in expects:
                expect_list += [re.compile(expectact.getAttribute('out').encode(CMDENCODING))]
                inner_cmds = Evaluate('cmd', expectact)
                #print inner_cmds
                cmdid_list += [inner_cmds]

            cmd_timeout = actcmd.getAttribute('timeout')
            if cmd_timeout != '': act_timeout = int(cmd_timeout)
            else: 
                act_timeout = int(default_timeout)

            if act_except != '':
                expect_list += [pexpect.EOF, pexpect.TIMEOUT]
                cmdid_list += [act_except, act_except]
            else:
                expect_list += [pexpect.EOF, pexpect.TIMEOUT]
                cmdid_list += ['__eof__', '__timeout__']

            expopt = child.expect_list(expect_list, timeout=act_timeout)

            if cmdid_list[expopt] == '__plus1__':
                sub_ret = 0
            elif cmdid_list[expopt] == act_except:
                #print "EXCEPTION" # FIXME only for fast debug
                sub_ret = func_parse(dom, child, act_except,
                                     default_timeout, sdelay)
            elif cmdid_list[expopt] == '__exit__':
                sub_ret = func_parse(dom, child, '__exit__',
                                     default_timeout, sdelay)
            elif cmdid_list[expopt] == '__eof__':
                sub_ret = func_parse(dom, child, '__eof__',
                                     default_timeout, sdelay)
            elif cmdid_list[expopt] == '__timeout__':
                sub_ret = func_parse(dom, child, '__timeout__',
                                     default_timeout, sdelay)
            elif type(cmdid_list[expopt]) == list:
                sub_ret = cmd_parse(dom, child, cmdid_list[expopt],
                                    default_timeout, sdelay)
        elif type_cmd == 'call':
            sub_ret = func_parse(dom, child, call_cmd,
                                 default_timeout, sdelay)
        elif type_cmd == 'return':
            return(int(ret_cmd))
        if sub_ret != 0:
            # if return != 0 raise the error
            return sub_ret
    # All commands ok, return 0
    return 0

def processOper(fin, fout):
    """ A operation consist in a initial function a serial/ethernet
    over the operation is executed, a default timeout for commands.
    """
    reader = PyExpat.Reader( )
    if (fin != sys.stdin):
      dom = reader.fromStream(open(fin.name, "r"))
    else:
      dom = reader.fromStream(sys.stdin)
    cmd_ini         = Evaluate("initial_func/text( )",
                               dom.documentElement)[0].nodeValue
    default_timeout = Evaluate("default_timeout/text( )",
                               dom.documentElement)[0].nodeValue
    sdelay          = int(Evaluate("send_delay/text( )",
                               dom.documentElement)[0].nodeValue)

    ethnode = Evaluate("eth_params", dom.documentElement)
    eth_dev = ethnode[0].getAttribute('dev')
    if len(eth_dev) != 0:
        by_serial = False
        ip = ethnode[0].getAttribute('ip')
        port = ethnode[0].getAttribute('port')
    else:
        by_serial = True
        tty_read        = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('tty')
        baudrate_read   = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('baudrate')
        bits_read       = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('bits')
        parity_read     = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('parity')
        stopbits_read   = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('stopbits')
        xonxoff_read    = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('xonxoff')
        rtscts_read     = Evaluate("serial_params",
                                   dom.documentElement)[0].getAttribute('rtscts')
    if by_serial:
        if   (parity_read == "N") : parity_cte = PARITY_NONE
        elif (parity_read == "E") : parity_cte = PARITY_EVEN
        elif (parity_read == "O") : parity_cte = PARITY_ODD

        ser = serial.Serial(
            port     = int(tty_read),
            baudrate = int(baudrate_read),
            bytesize = int(bits_read),
            
            parity   = parity_cte,
            stopbits = int(stopbits_read),
            timeout  = None,               # set a timeout value, None to wait forever
            xonxoff  = int(xonxoff_read),  # enable software flow control
            rtscts   = int(rtscts_read),   # enable RTS/CTS flow control
            writeTimeout = None,           # set a timeout for writes
            )
        fd = ser.fd
        if os.path.exists("/var/lock/LCK..ttyS"+tty_read):
            return BBERRLOCK
        lock = open("/var/lock/LCK..ttyS" + tty_read, "w")
        lock.write(str(os.getpid()))
        child = pexpect.spawn(fd)
    else:
        telnet_cmd = 'telnet ' + ip + " " + port
        child = pexpect.spawn(telnet_cmd)
    child.setlog(fout)
    cmd_act = cmd_ini
    try: ret_val = func_parse(dom, child, cmd_act, default_timeout, sdelay)
    finally:
        if by_serial:
            ser.close()
            os.remove("/var/lock/LCK..ttyS" + tty_read)
        return ret_val

def main(fin, fout):
    ret = processOper(fin, fout)
    sys.exit(ret)

if __name__ == "__main__":
    if len(sys.argv) == 3:
        # argv[1] = inputfile, argv[2] = errorfile
        fin  = open(sys.argv[1], "r")
        fout = open(sys.argv[2], "w")
        main(fin, fout)
    else:
        main(sys.stdin, sys.stdout)
