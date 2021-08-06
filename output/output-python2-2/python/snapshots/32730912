# Copyright (C) Scott Walker 2007 <iswalker at gmail dot com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2, or (at your option)
# any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with this program.  If not, write to:
# 	The Free Software Foundation, Inc.,
# 	51 Franklin Street, Fifth Floor
# 	Boston, MA  02110-1301, USA.
#

import os, socket
import struct, hashlib
from btpdwebui.btpd.bencode import bencode, bdecode


class _Enum(object):
    """Class to represent the various enumerations in the btpd code."""
    
    # Child classes can override this to add their
    # preferred string representations of a numeric value.
    _strings = []
    
    @classmethod
    def enumstr(cls, value):
        """Translate the enum value to its string."""
        try:
            return cls._strings[value]
        except IndexError:
            return 'unknown'

    @classmethod
    def enumval(cls, string):
        """Translate the enum string to its value."""
        value = 0
        for s in cls._strings:
            if s == string: return value
            value += 1
        return -1
        

class TActivity(_Enum):
    """Torrent activity (enum ipc_twc)"""    
    ALL, ACTIVE, INACTIVE = range(3)

    _strings = ['all',
                'active',
                'inactive']


class TStates(_Enum):
    """Torrent state (enum ipc_tstate)"""    
    INACTIVE, START, STOP, LEECH, SEED = range(5)

    _strings = ['inactive',
                'start',
                'stop',
                'leech',
                'seed']


class TAttrs(_Enum):
    """Torrent attribute (enum ipc_tval)"""   
    CGOT, CSIZE, DIR, NAME, NUM, IHASH, PCGOT, PCOUNT, PCCOUNT, PCSEEN, \
    RATEDWN, RATEUP, SESSDWN, SESSUP, STATE, TOTDWN, TOTUP, TRERR = range(18)


class TAttrTypes(_Enum):
    """Torrent attribute types (enum ipc_type)"""    
    ERR, BIN, NUM, STR = range(4)


class TErrors(_Enum):
    """Possible errors returned by btpd (enum ipc_err)"""
    OK, COMMERR, BADCDIR, BADT, BADTENT, BADTRACKER, CREATECDIR, NOKEY, \
    NOTENT, SHUTDOWN, TACTIVE, TENTEXIST, TINACTIVE = range(13)

    _strings = ['no error',
                'communication error',
                'bad content directory',
                'bad torrent',
                'bad torrent entry',
                'bad tracker',
                'could not create content directory',
                'no such key',
                'no such torrent entry',
                'btpd is shutting down',
                'torrent is active',
                'torrent entry exists',
                'torrent is inactive']


class Error(Exception):
    """Exception for general btpd module errors"""
    pass


class RequestError(Exception):
    """Exception for errors returned by the btpd daemon"""
    
    def __init__(self, code):
        self.code = code

    def __str__(self):
        return TErrors.enumstr(self.code)


def find_btpd_dir():
    """Find and return the btpd home directory"""
    if os.environ.has_key('BTPD_HOME'):
        btpd_dir = os.environ['BTPD_HOME']
    else:
        btpd_dir = os.path.expanduser('~')
        btpd_dir = os.path.join(btpd_dir, '.btpd')
    if not os.path.isdir(btpd_dir):
        raise Error('cannot find btpd directory')
    return btpd_dir

def torrent_name(metainfo):
    """Returns the torrent name given its metainfo"""
    try:
        mi = bdecode(metainfo)
        return mi['info']['name']
    except:
        raise Error('no torrent name')    


class Btpd(object):
    """Interface to the btpd daemon"""

    def __init__(self, timeout=5):
        self.btpd_dir = find_btpd_dir()
        self.timeout = timeout
        self._sock = None
    
    def _connect(self):
        try:
            sock_path = os.path.join(self.btpd_dir, 'sock')
            self._sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            self._sock.settimeout(self.timeout)        
            self._sock.connect(sock_path)
        except socket.error:
            raise Error('failed to connect to Btpd')

    def _disconnect(self):
        if self._sock is not None:
            self._sock.close()
            self._sock = None

    def _sendmsg(self, data):
        # btpd messages consists of a 32-bit length
        # header followed by a bendoded string.
        size = len(data)
        request = struct.pack('I%ds' % size, size, data)
        self._sock.sendall(request)        

    def _recvmsg(self):
        # get the 32-bit length header
        # and unpack it into an integer.
        data = self._sock.recv(4)
        if len(data) != 4: return ''
        size = struct.unpack("I", data)[0]
        if size == 0: return ''
        # return the bencoded message body
        return self._sock.recv(size)          

    def _req_result(self, data):
        self._connect()
        try:
            self._sendmsg(bencode(data))
            res = bdecode(self._recvmsg())
        finally:
            self._disconnect()
        if type(res) is not dict:
            raise Error('unknown response type')
        return res

    def _req_code(self, data):
        res = self._req_result(data)
        if not res.has_key('code'):
            raise Error('missing response code')
        if res['code'] != TErrors.OK:
            raise RequestError(res['code'])
        return res

    def delete(self, torrent):
        """Delete the specified torrent from btpd"""
        self._req_code(['del', torrent])

    def start(self, torrent):
        """Start the specified torrent"""
        self._req_code(['start', torrent])

    def stop(self, torrent):
        """Stop the specified torrent"""
        self._req_code(['stop', torrent])

    def get(self, activity, keys, callback):
        """Retrieve information about the current torrents.
        activity => TActivity enum
        keys     => TAttrs enum of the fields we want
        callback => Callback function called for each torrent
        """
        args = {'from' : activity,
                'keys' : keys}
        res = self._req_code(['tget', args])
        if not res.has_key('result'):
            raise Error('missing result')
        for tinfo in res['result']:
            if type(tinfo) is int:                
                callback(tinfo, None)
            else:
                tor = {}
                for i in range(len(keys)):
                    tor[keys[i]] = tinfo[i*2+1]
                callback(TErrors.OK, tor)

    def add(self, metainfo, content, topdir=False, name=None):
        """Add a new torrent to btpd.
        metainfo => raw torrent file data
        content  => top-level content directory
        topdir   => if true, torrent name is appended to 'content'
        name     => desired torrent name (overrides name in metainfo)
        """
        if topdir:
            try:
                # Append the name to content directory.  
                # If torrent_name throws (there is no name), then
                # just ignore it (don't add a top directory).
                tdir = name or torrent_name(metainfo)
                content = os.path.join(content, tdir)                        
            except: pass
        args = {'content' : content,
                'torrent' : metainfo}
        if name is not None and len(name) > 0:
            args['name'] = name        
        res = self._req_code(['add', args])
        return res['num']

