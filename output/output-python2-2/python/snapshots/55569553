
#===========================================================================
# Networking Module
#
# - to force host mode create an empty file named "FORCE_HOST" in the base
#   TamTam directory
# - to force peer mode create a file named "FORCE_PEER" with a single line
#   containing the IP of the host to connect to
#
# !! the host must be running before the peers start up !!
#---------------------------------------------------------------------------

import os
import socket
import select
import threading
import xdrlib
import random

import time
import gtk
import gobject
import common.Config as Config

PORT = 24460
LISTENER_PORT = PORT-1
WAIT_PORT = PORT-2

BACKLOG = 5 # allow a backlog of N new connections
MAX_SIZE = 1024 # max message size to receive in one go

MD_OFFLINE = 0
MD_HOST = 1
MD_PEER = 2
MD_WAIT = 3

# enumerate message types
# format: ("NAME", <message size>)
# <message size> specified in bytes
# special:
#    -1 == dynamic, first byte of data containes size
#    -2 == dynamic, first uint32 of data contains size
message_enum = [
("HT_LATENCY_REPLY",        4),  # reply to latency test
("HT_SYNC_REPLY",           8),  # reply to sync test
("HT_TEMPO_UPDATE",         4),  # reply to sync test

("PR_LATENCY_QUERY",        4),  # test latency
("PR_SYNC_QUERY",           4),  # test sync
("PR_TEMPO_QUERY",          0),  # test sync
("PR_REQUEST_TEMPO_CHANGE", 4),  # request tempo change 

("MAX_MSG_ID",              0)
]

# Initialize message ids and MSG_NAME/MSG_SIZE arrays
MSG_NAME = [""]
MSG_SIZE = [0]
i = 1
for m in message_enum:
    exec "%s = %d" % (m[0],i)
    MSG_NAME.append(m[0])
    MSG_SIZE.append(m[1])
    i += 1
del message_enum # clear memory
if MAX_MSG_ID > 256:
    print "Network:: MAX_MSG_ID exeeds limit of 256!"


class Listener( threading.Thread ):
    
    def __init__( self, owner, listenerSocket, inputSockets, outputSockets, exceptSockets ):
        threading.Thread.__init__(self)
        self.owner = owner
        self.listenerSocket = listenerSocket
        self.inputSockets = inputSockets    # note that these are array pointers that match
        self.outputSockets = outputSockets  # those of the Network and should not be reset
        self.exceptSockets = exceptSockets  #

    def run(self):
        while 1:  # rely on the owner to kill us when necessary
            try:
                inputReady, outputReady, exceptReady = select.select( self.inputSockets, self.outputSockets, self.exceptSockets, 0.5 )
                if not len( inputReady ): # timeout
                    continue
                if self.listenerSocket in inputReady:
                    data, s = self.listenerSocket.recvfrom(MAX_SIZE)
                    if data == "REFRESH":
                        continue
                    if data == "CLEAR":
                        self.owner._clearSockets()
                        continue
                    else:
                        break # exit thread
                gtk.gdk.threads_enter()
                self.owner._processSockets( inputReady )
                gtk.gdk.threads_leave()
            except socket.error, (value, message):
                print "Listener:: socket error: " + message
                gtk.gdk.threads_leave()
                break

class Connection:
    
    def __init__( self, sock, address ):
        self.socket = sock
        self.address = address
        self.recvBuf = ""
        self.waitingForData = 0
        self.message = 0

class Network:

    def __init__( self, mode = MD_OFFLINE, hostaddress = None ):

        # check for forced networking
        if os.path.isfile("FORCE_HOST"):
            mode = MD_HOST
        elif os.path.isfile("FORCE_PEER"):
            f = open("FORCE_PEER")
            l = f.read(16)
            print l
            f.close()
            mode = MD_PEER
            hostaddress = (l,PORT)

        # prepare message handlers
        self.processMessage = {}
        for i in range(1,MAX_MSG_ID):
            self.processMessage[i] = []

        self.statusWatcher = []

        # data packing classes
        self.packer = xdrlib.Packer()
        self.unpacker = xdrlib.Unpacker("")

        self.mode = -1
        self.listener = None
        self._fromListener = False
        try:
            self.listenerSocket = socket.socket( socket.AF_INET, socket.SOCK_DGRAM )
            self.listenerSocket.bind( ("localhost", LISTENER_PORT) )
        except socket.error, (value,message):
            print "Network:: FAILED to open listenerSocket: " + message
            mode = MD_OFFLINE
        
        self.inputSockets = [ self.listenerSocket ] # NOTE that these array pointers are passed into
        self.outputSockets = []                     # the Listener and should not be reset
        self.exceptSockets = []                     #
        self.connection = {}      # dict of connections indexed by socket

        self.latencyQueryHandler = {}
        self.latencyQueryStart = {}

        self.connectMessage( HT_LATENCY_REPLY, self.processHT_LATENCY_REPLY )
        self.connectMessage( PR_LATENCY_QUERY, self.processPR_LATENCY_QUERY )

        self.setMode( mode, hostaddress )

    def shutdown( self ):
        if Config.DEBUG > 1: print "Network:: shutting down!"
        
        if self.listener:
            self.listenerSocket.sendto( "EXIT", ("localhost",LISTENER_PORT) )
            time.sleep(0.01) # get off the cpu so the listerer thread has a chance to clear.. IS THERE A BETTER WAY TO DO THIS?
            self.listener = None

        if self.mode == MD_HOST:
            for s in self.inputSockets:
                s.close()
        elif self.mode == MD_PEER:
            self.socket.close()
            self.hostAddress = None

    def setMode( self, mode, hostaddress = None ):

        # cleanup old mode
        if Config.DEBUG > 1: print "Network:: cleaning up old connections"

        if self._fromListener:
            self._clearSockets()
        elif self.listener: # make the listener wake so sockets can close properly
            self.listenerSocket.sendto( "CLEAR", ("localhost",LISTENER_PORT) ) 
            time.sleep(0.01) # get off the cpu so the listerer thread has a chance to clear.. IS THERE A BETTER WAY TO DO THIS?

        self.hostAddress = None

        # initialize new mode
        self.mode = mode 
        if self.mode == MD_HOST:
            if Config.DEBUG > 1: print "Network:: initializing network, host mode"
            try:
                self.socket = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
                address = ("",PORT)
                self.connection[self.socket] = Connection( self.socket, address )
                self.socket.bind(address)
                self.socket.listen(BACKLOG)
                self.inputSockets.append(self.socket)
                if not self._fromListener and self.listener:
                    self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
                elif not self.listener:
                    self.listener = Listener( self, self.listenerSocket, self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listener.start()
            except socket.error, (value, message):
                if self.socket:
                    self.socket.close()
                    self.connection.pop(self.socket)
                print "Network:: FAILED to open socket: " + message
                self.mode = MD_OFFLINE
                if self.listener:
                    self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                    self.listener = None

        elif self.mode == MD_PEER:
            if Config.DEBUG > 1: print "Network:: initializing network, client mode: " + hostaddress[0]
            self.hostAddress = hostaddress
            try:
                self.socket = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
                self.connection[self.socket] = Connection( self.socket, self.hostAddress )
                self.socket.connect(self.hostAddress)
                self.inputSockets.append(self.socket)
                if not self._fromListener and self.listener:
                    self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
                elif not self.listener:
                    self.listener = Listener( self, self.listenerSocket, self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listener.start()
            except socket.error, (value, message):
                if self.socket:
                    self.socket.close()
                    self.connection.pop(self.socket)
                print "Network:: FAILED to open socket: "  + message
                self.mode = MD_OFFLINE
                self.hostAddress = None
                if self.listener:
                    self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                    self.listener = None

        elif self.mode == MD_WAIT:
            if Config.DEBUG > 1: print "Network:: initializing network, wait mode" 
            try:
                self.socket = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
                address = ("",WAIT_PORT)
                self.connection[self.socket] = Connection( self.socket, address )
                self.socket.bind(address)
                self.socket.listen(BACKLOG)
                self.inputSockets.append(self.socket)
                if not self._fromListener and self.listener:
                    self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
                elif not self.listener:
                    self.listener = Listener( self, self.listenerSocket, self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listener.start()
            except socket.error, (value, message):
                if self.socket:
                    self.socket.close()
                    self.connection.pop(self.socket)
                print "Network:: FAILED to open socket: " + message
                self.mode = MD_OFFLINE
                if self.listener:
                    self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                    self.listener = None
                    
        else:
            if Config.DEBUG > 1: print "Network:: offline"
            if self.listener:
                self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                self.listener = None

        for watcher in self.statusWatcher:
            watcher( self.mode ) 

    def _clearSockets( self ):
        for s in self.inputSockets:
            if s != self.listenerSocket:
                self.inputSockets.remove(s)
                self.connection.pop(s)
                s.close()
        for s in self.outputSockets:
            self.outputSockets.remove(s)
            s.close()
        for s in self.exceptSockets:
            self.exceptSockets.remove(s)
            s.close()


    def introducePeer( self, ip ):
        if Config.DEBUG > 1: print "Network:: introducing self to peer " + ip
        try: 
            poke = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
            poke.setblocking(0)
        except socket.error, (value, message):
            print "Network::introducePeer:: FAILED to open socket: " + message
            return
        if poke.connect_ex( (ip, WAIT_PORT) ): # failed to connect
            gobject.timeout_add( 500, self._pokePeer, poke, ip, 0 )
        else: # connected
            if Config.DEBUG > 1: print "Netwtork:: introduction succeeded"
            poke.close()

    def _pokePeer( self, poke, ip, retry ):
        if poke.connect_ex( (ip, WAIT_PORT) ): # failed to connect
            if retry > 120: # give up
                print "Network::introducePeer:: peer failed to respond after 60 seconds, giving up!"
            else:
                gobject.timeout_add( 500, self._pokePeer, poke, ip, retry+1 )
        else: # connected
            if Config.DEBUG > 1: print "Netwtork:: introduction succeeded"
            poke.close()

        return False

    
    def addPeer( self, peer, address ):
        if Config.DEBUG > 1: print "Network:: adding peer: %s" % address[0]
        self.connection[peer] = Connection( peer, address )
        self.inputSockets.append( peer )
        self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
        #self.listener.updateSockets( self.inputSockets, self.outputSockets, self.exceptSockets )

    def removePeer( self, peer ):
        if Config.DEBUG > 1: print "Network:: removing peer: %s" % self.connection[peer].address[0]
        self.connection.pop(peer)
        self.inputSockets.remove(peer)
        self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
        #self.listener.updateSockets( self.inputSockets, self.outputSockets, self.exceptSockets )

    # register a status watcher, format: func( self, status, args )
    def addWatcher( self, func ):
        self.statusWatcher.append( func )

    def removeWatcher( self, func ):
        self.statusWatcher.remove( func )

    # connect a message handler, format: func( self, sock, message, data )
    def connectMessage( self, message, func ):
        self.processMessage[message].append(func)

    def connectMessageAfter( self, message, func, after ):
        try:
            ind = self.processMessage[message].index(after)
            self.processMessage[message].insert(ind+1,func)
        except:
            print "Network::connectMessageAfter:: function not registered: " + str(after)

    def connectMessageBefore( self, message, func, before ):
        try:
            ind = self.processMessage[message].index(before)
            self.processMessage[message].insert(ind,func)
        except:
            print "Network::connectMessageBefore:: function not registered: " + str(before)

    def disconnectMessage( self, message, func ):
        try:
            self.processMessage[message].remove(func)
        except:
            print "Network::disconnectMessage:: function not registered: " + str(func)

    def isOffline( self ):
        if self.mode == MD_OFFLINE: return True
        return False

    def isOnline( self ):
        if self.mode != MD_OFFLINE: return True
        return False

    def isHost( self ):
        if self.mode == MD_HOST: return True
        return False

    def isPeer( self ):
        if self.mode == MD_PEER: return True
        return False

    def isWaiting( self ):
        if self.mode == MD_WAIT: return True
        return False
        

    #-----------------------------------------------------------------------
    # Message Senders

    # basic send function
    # - message type will be automatically inserted before the data
    # - message size will be automatically inserted if applicable
    # - to is only defined in HOST mode
    def send( self, message, data = "", to = None ): 
        if self.mode == MD_OFFLINE: 
            return

        length = len(data)
        size = MSG_SIZE[message]

        if size >= 0:
            if length != size:
                print "Network:: message wrong length! Got %d expected %d: %s" % (len(data), MSG_SIZE[message], MSG_NAME[message])
                return
            msg = chr(message) + data
        elif size == -1:
            if length > 255:
                print "Network:: oversized message! Got %d, max size 255: %s" % (length, MSG_NAME[message])
                return
            msg = chr(message) + chr(length) + data
        else: # size == -2
            self.packer.pack_uint(size)
            msg = chr(message) + self.packer.get_buffer() + data
            self.packer.reset()

        if self.mode == MD_PEER:
            try:
                self.socket.send( msg )
                #print "Network:: sent %d bytes" % (len(msg))
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.hostAddress[0], errmsg)
                # TODO something intelligent
        else: # MD_HOST
            try:
                to.send( msg )
                #print "Network:: sent %d bytes" % (len(msg))
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.connection[to].address[0], errmsg)
                # TODO something intelligent


    def sendAll( self, message, data = "" ):
        if self.mode != MD_HOST:
            return

        length = len(data)
        size = MSG_SIZE[message]

        if size >= 0:
            if length != size:
                print "Network:: message wrong length! Got %d expected %d: %s" % (MSG_SIZE[message], len(data), MSG_NAME[message])
                return
            msg = chr(message) + data
        elif size == -1:
            if length > 255:
                print "Network:: oversized message! Size %d, max size 255: %s" % (length, MSG_NAME[message])
                return
            msg = chr(message) + chr(length) + data
        else: # size == -2
            self.packer.pack_uint(size)
            msg = chr(message) + self.packer.get_buffer() + data
            self.packer.reset()

        for sock in self.connection:
            if sock == self.socket: 
                continue
            try:
                sock.send( msg )
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.connection[sock].address[0], errmsg)
                # TODO something intelligent
    
    def sendLatencyQuery( self, handler ):
        if self.mode != MD_PEER:
            return

        self.packer.pack_float(random.random())
        hash = self.packer.get_buffer()
        self.packer.reset()
        self.latencyQueryHandler[hash] = handler
        self.latencyQueryStart[hash] = time.time()
        self.send(PR_LATENCY_QUERY,hash)

    #-----------------------------------------------------------------------
    # Message Handlers

    def _processSockets( self, inputReady ):

        self._fromListener = True 

        if self.mode == MD_HOST:

            for s in inputReady:
                if s == self.socket:
                    # accept new connections
                    try:
                        peer, address = self.socket.accept()
                        self.addPeer( peer, address )
                    except socket.error, (value, message):
                        print "Network:: error accepting connection: " + message
                
                else:
                    try:
                        data = s.recv(MAX_SIZE)
                        #print "Network:: recv %d bytes: %s" % (len(data), data)
                        if not len(data): # no data to read, socket must be closed
                            self.removePeer(s)
                        else:
                            self.processStream( s, data )
                    except socket.error, (value, message):
                        print "Network:: error reading data: " + message

        elif self.mode == MD_PEER:
            
            for s in inputReady:
                try:
                    data = s.recv(MAX_SIZE)
                    if not len(data): # no data to read, socket must be closed
                        self.setMode( MD_OFFLINE )
                    else:
                        #print "Network:: recv %d bytes: %s" % (len(data), data)
                        self.processStream( s, data )
                except socket.error, (value, message):
                    print "Network:: error reading data: " + message

        else: # MD_WAIT
            
            for s in inputReady:
                try:
                    peer, address = self.socket.accept()
                    self.setMode( MD_PEER, (address[0], PORT) )
                except socket.error, (value, message):
                    print "Network:: error accepting connection: " + message

        self._fromListener = False


    def processStream( self, sock, newData = "" ):
        con = self.connection[sock]
        con.recvBuf += newData
        
        if con.waitingForData == -1: # message size in char
            con.waitingForData = ord(con.recvBuf[0])
            con.recvBuf = con.recvBuf[1:]

        elif con.waitingForData == -2: # message size in uint
            if len(con.recvBuf) >= 4:
                self.unpacker.reset(con.recvBuf[0:4])
                con.waitingForData = self.unpacker.unpack_uint()
                con.recvBuf = con.recvBuf[4:]
            else:
                return # wait for more data

        elif con.waitingForData:
            if len(con.recvBuf) >= con.waitingForData:
                data = con.recvBuf[0:con.waitingForData]
                con.recvBuf = con.recvBuf[con.waitingForData:]
                con.waitingForData = 0
                for func in self.processMessage[con.message]:
                    gobject.idle_add( func, sock, con.message, data )
            else:
                return # wait for more data

        else:
            con.message = ord(con.recvBuf[0])
            if MSG_SIZE[con.message] == 0:
                con.recvBuf = con.recvBuf[1:]
                for func in self.processMessage[con.message]:
                    gobject.idle_add( func, sock, con.message, "" )
            else:
                con.waitingForData = MSG_SIZE[con.message]
                con.recvBuf = con.recvBuf[1:]

        if len(con.recvBuf):
            self.processStream( sock )
                
    #-- HOST handlers ------------------------------------------------------
    def processPR_LATENCY_QUERY( self, sock, message, data ):
        self.send( HT_LATENCY_REPLY, data, sock )

    #-- PEER handlers ------------------------------------------------------
    def processHT_LATENCY_REPLY( self, sock, message, data ):
        t = time.time()
        latency = t - self.latencyQueryStart[data]
        #print "got latency reply %d" % (latency*1000)
        self.latencyQueryHandler[data]( latency )
        self.latencyQueryHandler.pop(data)
        self.latencyQueryStart.pop(data)


