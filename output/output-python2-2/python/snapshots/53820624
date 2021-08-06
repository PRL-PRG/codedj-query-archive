# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
This handles the raw communication with the Nexuiz server.
"""
from __future__ import division, absolute_import, with_statement
from twisted.internet.protocol import DatagramProtocol
import sys
from .utils import quote
__all__ = 'Rcon',

SEND_FORMAT = '\xFF\xFF\xFF\xFF%s\0'
RECV_PREFIX = '\xFF\xFF\xFF\xFFn'

class Rcon(DatagramProtocol):
	__password = None
	_host = _port = None
	def __init__(self,host,port, password):
		super(Rcon, self).__init__()
		self._host = host
		self._port = port or 26000
		self.__password = password
	
	def startProtocol(self):
		self.transport.connect(self._host, self._port)
		print >> sys.stderr, "we can only send to %s now" % str((host, port))
		#self.transport.write("hello") # no need for address
		
	# Possibly invoked if there is no server listening on the
	# address to which we are sending.
	def connectionRefused(self):
		print >> sys.stderr, "No one listening"
	
	def datagramReceived(self, data, (host, port)):
		if data.startswith(RECV_PREFIX):
			data = data[len(RECV_PREFIX):]
		else:
			return
		if data[-1] == '\0':
			data = data[:-1]
		print >> sys.stderr, "received %r from %s:%d" % (data, host, port)
		self.textReceived(data)
	
	def textReceived(self, data):
		print data
	
	def send_raw(self, text):
		return self.transport.write(SEND_FORMAT % text)
	
	def format_rcon(self, cmd):
		"""r.format_rcon(str) -> str
		Turns the command into what should be passed to send_raw()
		"""
		return "rcon %s %s" % (self.__password, cmd)
	
	def _send(self, cmd):
		"""r._send(str) -> None
		Wraps a command in rcon to be sent.
		"""
		return self.send_raw("rcon %s %s" % (self.__password, cmd))
	
	def _sends(self, cmds):
		return self.send_raw('\0'.join("rcon %s %s" % (self.__password, cmd) for cmd in cmds))
	
	def format_cmd(self, cmd, *parts):
		return cmd+' '.join(quote(a for a in pargs))
	
	def cmd(self, cmd, *pargs):
		"""r.cmd(str, ...) -> None
		Sends the command cmd with the arguments pargs.
		"""
		return self._send(self.format_cmd(cmd, *pargs))
	
	def cmds(self, *cmds):
		"""r.cmds((str, ...), ...) -> None
		Sends a single packet with all the commands.
		"""
		return self._sends((self.format_cmd(*cmd) for cmd in cmds))
	
	def start_streaming(self):
		"""
		Sets up streaming console.
		"""
		self.cmd("log_dest_udp", "%s:%i" % (self.transport.getHost(), self._realPortNumber)) # Yeah, yeah. Using private data of superclass.

if __name__ == '__main__':
	from twisted.internet import reactor
	proto = Rcon()
	reactor.listenUDP(0, proto)
	proto.start_streaming()
	reactor.run()

