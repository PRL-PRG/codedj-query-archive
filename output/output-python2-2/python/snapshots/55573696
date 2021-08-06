
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

import time
import gtk
import gobject
import Config

PORT = 24420
LISTENER_PORT = PORT-1
BACKLOG = 5 # allow a backlog of N new connections
MAX_SIZE = 1024 # max message size to receive in one go

MD_OFFLINE = 0
MD_HOST = 1
MD_PEER = 2

# enumerate message types
# format: "NAME": <message size>
# <message size> specified in bytes
# special:
#    -1 == dynamic, first byte of data containes size
#    -2 == dynamic, first uint32 of data contains size
message_enum = {
"HT_LATENCY_REPLY": 0,          # reply to latency test

"PR_LATENCY_QUERY": 0,          # test latency

"MAX_MSG_ID": 0
}

# Initialize message ids and MSG_NAME/MSG_SIZE arrays
MSG_NAME = [""]
MSG_SIZE = [0]
i = 1
for m in message_enum:
    exec "%s = %d" % (m,i)
    MSG_NAME.append(m)
    MSG_SIZE.append(message_enum[m])
    i += 1
del message_enum # clear memory
if MAX_MSG_ID > 256:
    print "Network:: MAX_MSG_ID exeeds limit of 256!"


class Listener( threading.Thread ):
    
    def __init__( self, owner, listenerSocket, inputSockets, outputSockets, exceptSockets ):
        threading.Thread.__init__(self)
        self.owner = owner
        self.listenerSocket = listenerSocket
        self.inputSockets = inputSockets
        self.outputSockets = outputSockets
        self.exceptSockets = exceptSockets

    def updateSockets( self, inputSockets, outputSockets, exceptSockets ):
        print "update sockets!"
        self.inputSockets = inputSockets
        self.outputSockets = outputSockets
        self.exceptSockets = exceptSockets

    def run(self):
        while 1:  # rely on the owner to kill us when necessary
            try:
                print "ruuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuun"
                inputReady, outputReady, exceptReady = select.select( self.inputSockets, self.outputSockets, self.exceptSockets )
                if self.listenerSocket in inputReady:
                    data, s = self.listenerSocket.recvfrom(MAX_SIZE)
                    if data == "REFRESH":
                        print "reeeeeeeeeeeeeeeeeeeeeeeeeeeefresh"
                        continue
                    if data == "CLEAR":
                        print "cleeeeeeeeeeeeeeeeeeeeeeeeeeear"
                        self.inputSockets = [ self.listenerSocket ]
                        self.outputSockets = []
                        self.exceptSockets = []
                        continue
                    else:
                        print "exxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxit"
                        break # exit thread
                gtk.gdk.threads_enter()
                self.owner.process( inputReady, outputReady, exceptReady )
                print "dooooooooooooooooooooooooooooooooooooooooooooooooooooooone"
                gtk.gdk.threads_leave()
            except socket.error, (value, message):
                print "Listener:: socket error: " + message
                gtk.gdk.threads_leave()
                break

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
            try:
                exec "self.processMessage[" + i + "] = self.process" + MSG_NAME[i]
            except:
                print "Network:: message handler not defined for " + MSG_NAME[i]

        # data packing classes
        self.packer = xdrlib.Packer()
        self.unpacker = xdrlib.Unpacker("")

        self.mode = -1
        self.listener = None
        try:
            self.listenerSocket = socket.socket( socket.AF_INET, socket.SOCK_DGRAM )
            self.listenerSocket.bind( ("localhost", LISTENER_PORT) )
        except socket.error, (value,message):
            print "Network:: FAILED to open listenerSocket: " + message
            mode = MD_OFFLINE
        
        self.inputSockets = [ self.listenerSocket ]
        self.outputSockets = []
        self.exceptSockets = []
        self.address = {}   # dict of addresses indexed by socket
        self.waitingForData = {}
        self.recvBuf = {}
        self.peer = {}      # dict of sockets indexed by addresses
       # self.processTimeout = False

        self.setMode( mode, hostaddress )

    def shutdown( self ):
        if Config.DEBUG > 1: print "Network:: shutting down!"
        
        if self.listener:
            self.listenerSocket.sendto( "EXIT", ("localhost",LISTENER_PORT) )

        if self.mode == MD_HOST:
            for s in self.inputSockets:
                s.close()
        elif self.mode == MD_PEER:
            self.socket.close()
            self.hostAddress = None

    def setMode( self, mode, hostaddress = None ):

        if self.listener: # clear the listener so sockets can close properly
            self.listenerSocket.sendto( "CLEAR", ("localhost",LISTENER_PORT) ) 
            time.sleep(0.01) # get off the cpu so the listerer thread has a chance to clear.. IS THERE A BETTER WAY TO DO THIS?

        # cleanup old mode
        if self.mode == MD_HOST:
            if Config.DEBUG > 1: print "Network:: host - cleaning up old connections"
            for s in self.inputSockets:
                if s != self.listenerSocket: s.close()
            self.socket.close()
            self.socket = None
            self.inputSockets = [ self.listenerSocket ]
            self.peer = {}
            self.address = {}

        elif self.mode == MD_PEER:
            if Config.DEBUG > 1: print "Network:: peer - cleaning up old connections"
            self.socket.close()
            self.socket = None
            self.hostAddress = None

#        if self.processTimeout:
#            gobject.source_remove( self.processTimeout )
#            self.processTimeout = False
        

        # initialize new mode
        self.mode = mode 
        if self.mode == MD_HOST:
            if Config.DEBUG > 1: print "Network:: initializing network, host mode"
            try:
                self.socket = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
                self.socket.bind(("",PORT))
#                self.socket.setblocking(0)
                self.socket.listen(BACKLOG)
                self.recvBuf[socket] = "" 
                self.waitingForData[socket] = 0
                self.inputSockets.append(self.socket)
                self.peer = {}   
                self.address = {}
                if self.listener:
                    self.listener.updateSockets( self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
                else:
                    self.listener = Listener( self, self.listenerSocket, self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listener.start()
            except socket.error, (value, message):
                if self.socket:
                    self.socket.close()
                print "Network:: FAILED to open socket: " + message
                self.mode = MD_OFFLINE
                if self.listener:
                    self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                    self.listener = None

        elif self.mode == MD_PEER:
            if Config.DEBUG > 1: print "Network:: initializing network, client mode: " + self.hostAddress
            self.hostAddress = hostaddress
            try:
                self.socket = socket.socket( socket.AF_INET, socket.SOCK_STREAM )
                self.socket.setblocking(0)
                self.recvBuf[socket] = ""
                self.waitingForData[socket] = 0
                self.socket.connect(self.hostAddress)
                if self.listener:
                    self.listener.updateSockets( self.inputsSockets, self.outputSockets, self.exceptSockets )
                    self.listenerSocket.sendto( "REFRESH", ("localhost", LISTENER_PORT) )
                else:
                    self.listener = Listener( self, self.listenerSocket, self.inputSockets, self.outputSockets, self.exceptSockets )
                    self.listener.start()
            except socket.error, (value, message):
                if self.socket:
                    self.socket.close()
                print "Network:: FAILED to open socket: "  + message
                self.mode = MD_OFFLINE
                self.hostAddress = None
                if self.listener:
                    self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                    self.listener = None

        else:
            if Config.DEBUG > 1: print "Network:: offline"
            if self.listener:
                self.listenerSocket.sendto( "EXIT", ("localhost", LISTENER_PORT) )
                self.listener = None

        #if self.mode != MD_OFFLINE:
           # self.processTimeout = gobject.timeout_add( 10, self.process )

    def addPeer( self, peer, address ):
        if Config.DEBUG > 1: print "Network:: adding peer: %s" % address[0]
        self.address[peer] = address
        self.waitingForData[peer] = 0
        self.recvBuf[peer] = ""
        self.peer[address] = peer
        self.inputSockets.append( peer )
        self.listener.updateSockets( self.inputSockets, self.outputSockets, self.exceptSockets )

    def removePeer( self, peer ):
        if Config.DEBUG > 1: print "Network:: removing peer: %s" % self.address[peer][0]
        self.peer.pop(self.address.pop(peer))
        self.waitingForData.pop(peer)
        self.recvBuf.pop(peer)
        self.inputSockets.remove(peer)
        self.listener.updateSockets( self.inputSockets, self.outputSockets, self.exceptSockets )

    #-----------------------------------------------------------------------
    # Message Senders

    # basic send function
    # - message type will be automatically inserted before the data
    # - message size will be automatically inserted if applicable
    # - sock must be defined in HOST mode
    def send( self, message, data = "", to = None ): 
        if self.mode == MD_OFFLINE: 
            return

        length = len(data)
        size = MSG_SIZE[message]

        if size >= 0:
            if length != size:
                print "Network:: message wrong length! Got %d expected %d: %s %s" % (MSG_SIZE[message], len(data), MSG_NAME[message], data)
                return
            msg = chr(message) + data
        elif size == -1:
            if length > 255:
                print "Network:: oversized message! Got %d, max size 255: %s %s" % (length, MSG_NAME[message], data)
                return
            msg = chr(message) + chr(length) + data
        else: # size == -2
            self.packer.pack_uint(size)
            msg = chr(message) + self.packer.get_buffer() + data
            self.packer.reset()

        if self.mode == MD_PEER:
            try:
                self.socket.send( msg )
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.hostAddress[0], errmsg)
                # TODO something intelligent
        else: # MD_HOST
            try:
                to.send( msg )
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.address[to][0], errmsg)
                # TODO something intelligent

    def sendAll( self, message, data = "" ):
        if self.mode != MD_HOST:
            return

        length = len(data)
        size = MSG_SIZE[message]

        if size >= 0:
            if length != size:
                print "Network:: message wrong length! Got %d expected %d: %s %s" % (MSG_SIZE[message], len(data), MSG_NAME[message], data)
                return
            msg = chr(message) + data
        elif size == -1:
            if length > 255:
                print "Network:: oversized message! Size %d, max size 255: %s %s" % (length, MSG_NAME[message], data)
                return
            msg = chr(message) + chr(length) + data
        else: # size == -2
            self.packer.pack_uint(size)
            msg = chr(message) + self.packer.get_buffer() + data
            self.packer.reset()

        for p in self.peer():
            try:
                p.send( msg )
            except socket.error, (value, errmsg):
                print "Network:: FAILED to send message (%s) to %s: %s" % (MSG_NAME[message], self.address[to][0], errmsg)
                # TODO something intelligent
            

    def queryLatency( self ):
        if self.mode != MD_PEER:
            return

        self.latencyQueryStart = time.time()
        self.send(PR_LATENCY_QUERY)
        
    #-----------------------------------------------------------------------
    # Message Handlers

    def processSockets( self, inputReady, outputReady, exceptReady ):

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
                        print data
                        print len(data)
                        if not len(data): # no data to read, socket must be closed
                            self.removePeer(s)
                        else:
                            self.processStream( s, data )
                    except socket.error, (value, message):
                        print "Network:: error reading data: " + message

        else: # MD_PEER
            
            for s in inputReady:
                try:
                    data = s.recv(MAX_SIZE)
                    if not let(data): # no data to read, socket must be closed
                        self.setMode( MD_OFFLINE )
                    else:
                        print data
                except socket.error, (value, message):
                    print "Network:: error reading data: " + message


    def processStream( self, sock, newData = "" ):
        recvBuf = self.recvBuf[sock] + newData     # assign these to local variables to save a bit of look up time
        waitingForData = self.waitingForData[sock]

        if waitingForData == -1: # message size in char
            self.waitingForData[sock] = ord(recvBuf[0])
            self.recvBuf[sock] = recvBuf[1:]

        elif waitingForData == -2: # message size in uint
            if len(recvBuf) >= 4:
                self.unpacker.reset(sock.recvBuf[0:4])
                self.waitingForData[sock] = self.unpacker.unpack_uint()
                self.recvBuf[sock] = recvBuf[4:]
            else:
                return # wait for more data

        elif waitingForData:
            if len(recvBuf) >= waitingForData:
                data = recvBuf[0:waitingForData]
                self.recvBuf[sock] = recvBuf[waitingForData:]
                self.waitingForData[sock] = 0
                self.processMessage[sock.message]( sock, data ) 
            else:
                return # wait for more data

        else:
            sock.message = ord(recvBuf[0])
            if MSG_SIZE[sock.message] == 0:
                self.recvBuf[sock] = recvBuf[1:]
                self.processMessage[sock.message]( sock, "" ) 
            else:
                self.waitingForData[sock] = MSG_SIZE[sock.message]

        if len(self.recvBuf[sock]):
            self.processStream( sock )
                
    #-- HOST handlers ------------------------------------------------------
    def processPR_LATENCY_QUERY( self, sock, data ):
        sock.send( HT_LATENCY_REPLY )
        print "got latency query from %s" % self.address[sock][0]

    #-- PEER handlers ------------------------------------------------------
    def processHT_LATENCY_REPLY( self, sock, data ):
        latency = time.time() - self.latencyQueryStart
        print "got latency reply %d" % latency*1000

        

