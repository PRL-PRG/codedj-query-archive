#!/usr/bin/python2.5
#
# Copyright (c) 2008 JT Olds
# http://www.jtolds.com/
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.

from __future__ import with_statement
import socket, struct, json, threading, errno
import gflags as flags


FLAGS = flags.FLAGS
flags.DEFINE_float("object_receive_time", 1.0, "The time before we give up "
    "trying to receive a serialized network 'object' after receiving its size "
    "information.")
flags.DEFINE_boolean("output_network_traffic", False, "Whether or not to output"
    " network 'object' traffic being sent or received by a FancySocket.")


class Error_(Exception): pass
class NetworkError(Error_): pass
class SocketClosed(NetworkError): pass
class UnexpectedSocketClose(SocketClosed): pass

class FancySocket(object):
  
  def __init__(self, sock=None):
    if sock is None:
      self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    else:
      assert type(sock) != FancySocket
      self.sock = sock
    self.sending_lock = threading.RLock()
    self.receiving_lock = threading.RLock()
  
  def sendObject(self, obj, short_int = True):
    if short_int:
      pack_string = "!H"
    else:
      pack_string = "!I"
    if FLAGS.output_network_traffic: print "sending:", obj
    obj = json.write(obj)
    size = len(obj)
    if short_int:
      assert size < 65536 # number of values a short int can hold
    else:
      assert size < 4294967296 # number of values a normal int can hold
    size = struct.pack(pack_string, size)
    with self.sending_lock:
      self.sendall(size)
      self.sendall(obj)
  
  def getObject(self, short_int = True, timeout=None):
    with self.receiving_lock:
      self.sock.settimeout(timeout)
      try:
        if short_int:
          msg_size = struct.unpack("!H", self.recvall(2))[0]
        else:
          msg_size = struct.unpack("!I", self.recvall(4))[0]
      except socket.timeout, e:
        return None
      except socket.error, e:
        if e[0] == errno.EAGAIN:
          return None
        else:
          raise e
      self.sock.settimeout(FLAGS.object_receive_time)
      obj = json.read(self.recvall(msg_size))
      self.sock.settimeout(None)
    if FLAGS.output_network_traffic: print "receiving:", obj
    return obj
  
  def sendall(self, *args, **kwargs):
    with self.sending_lock:
      return self.sock.sendall(*args, **kwargs)
  
  def recvall(self, size):
    """continue to try and receive size bytes of network data. returns all or
    raises an exception"""
    data = ""
    received = True
    with self.receiving_lock:
      while len(data) < size and received:
        received = self.sock.recv(size - len(data))
        data += received
    if received: return data
    raise UnexpectedSocketClose, "could not read %d bytes" % size
  
  def close(self, *args, **kwargs):
    return self.sock.close(*args, **kwargs)
    
  def getpeername(self, *args, **kwargs):
    return self.sock.getpeername(*args, **kwargs)
  
  def connect(self, *args, **kwargs):
    return self.sock.connect(*args, **kwargs)
    
  def readline(self):
    data = ""
    byte = None
    while byte != "\n":
      byte = self.recvall(1)
      data += byte
    return data
