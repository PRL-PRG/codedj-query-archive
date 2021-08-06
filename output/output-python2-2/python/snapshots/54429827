"""Virtual hosting implementation for Aspen.
"""
import httplib
import os
import socket
import subprocess
from os.path import exists, isdir, join

import proxy


# Monkey-patch HTTPConnection to bind to AF_UNIX.
# ===============================================
# This is a monkey-patch so we don't have to touch proxy.

def _set_hostport(self, host, port):
    """Extended to support AF_UNIX.
    """
    if host[0] in ['.', '/']: # AF_UNIX
        host = os.path.realpath(host)
        assert exists(host), "AF_UNIX socket %s not found" % host
        self.host = host
        self.port = port
    else:
        httplib.HTTPConnection._set_hostport(self, host, port)


def connect(self):
    """Extended to support AF_UNIX.
    """
    msg = "getaddrinfo returns an empty list"
    
    if self.port is None: # AF_UNIX
        self.sock = socket.socket(socket.AF_UNIX)
        self.sock.connect(self.host)
        return
        
    for res in socket.getaddrinfo(self.host, self.port, 0,
                                  socket.SOCK_STREAM):
        af, socktype, proto, canonname, sa = res
        try:
            self.sock = socket.socket(af, socktype, proto)
            if self.debuglevel > 0:
                print "connect: (%s, %s)" % (self.host, self.port)
            self.sock.connect(sa)
        except socket.error, msg:
            if self.debuglevel > 0:
                print 'connect fail:', (self.host, self.port)
            if self.sock:
                self.sock.close()
            self.sock = None
            continue
        break
    if not self.sock:
        raise socket.error, msg


httplib.HTTPConnection._set_hostport = _set_hostport
httplib.HTTPConnection.connect = connect


# Define the vhosting WSGI app.
# =============================

class VHost:
    
    def __init__(self, config):
        """Spawn the child processes here.
        """
        root = config.paths.root
        __ = config.paths.__
        self.hosts = {}
        
        sockdir = join(__, 'var', 'sock')
        if not isdir(sockdir):
            os.makedirs(sockdir) # set mode to 1666
        
        for hostname in os.listdir(root):
            if hostname == '__' or hostname.startswith('.'):
                continue
            hostroot = join(root, hostname)
            if not isdir(hostroot):
                continue
            hostsock = join(sockdir, hostname)
            proc = subprocess.Popen(['aspen', '-r'+hostroot, '-a'+hostsock])
            prox = proxy.TransparentProxy(force_host=hostsock)
            self.hosts[hostname] = (proc, prox)
        

    def __call__(self, environ, start_response):
        """Proxy to the child processes here.
        """
        host = environ.get('HTTP_HOST')
        if host is None:
            raise ValueError('No HTTP host provided.')
        if host not in self.hosts:
            raise ValueError("Don't know about %s" % host)
        proc, prox = self.hosts[host]
        return prox(environ, start_response)
        