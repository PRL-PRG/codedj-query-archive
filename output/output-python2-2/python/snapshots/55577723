import select
import sys
import socket
import csnd
import threading
import time

from Framework.CSound.CSoundConstants import CSoundConstants

#----------------------------------------------------------------------
# This class was borrowed from Simon Schampijer.  Thanks Simon!
#----------------------------------------------------------------------

# this is a multiple-client csound server
# the listener is put in a separate thread

class CsoundServerMult:           
    # server start-up
    def __init__(self, addr):
        self.server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server.bind(addr)
        self.size = 8196
        print "*** CsServer: Csound Python server listening at: @%s:%d" % (addr[0], addr[1])
        self.server.listen(32)
        self.input = [self.server,sys.stdin]
        self.running = 1
    
    # this is the interpreter function
    # if something is seen on the socket
    # it executes it as Python code
    def interpret(self):
        # run the universal orchestra        
        csound = csnd.Csound()
        
        perf = csnd.CsoundPerformanceThread(csound)
                
        csound.Compile( CSoundConstants.FILES_DIR + '/univorc.csd' )
        perf.Play()
        
        while self.running:
            inputready,outputready,exceptready = select.select(self.input,[],[])

            for s in inputready:
                if s == self.server:
                    # handle the server socket
                    client, address = self.server.accept()
                    print'*** CsServer: Client has been accepted on: ',address
                    self.input.append(client)

                elif s == sys.stdin:
                    # handle standard input
                    junk = sys.stdin.readline()
                    csound.SetChannel('udprecv.0.on', 0)
                    perf.Stop()
                    perf.Join()                                    
                    csound.Reset()
                    csound = None
                    print '*** CsServer: The csound instance has been reset successfully.'
                    self.running = 0 
              
                else:
                    # handle all other sockets
                    data = s.recv( self.size )
                    if data.strip('\n') == 'off()':
                        csound.SetChannel('udprecv.0.on', 0)
                        perf.Stop()
                        perf.Join()                                    
                        csound.Reset()
                        csound = None
                        print '*** CsServer: The csound instance has been reset successfully.'
                        self.running = 0 
                        break
        
                    print 'data = ', data
                    if data:
                        try:
                            exec data
                        except:
                            pass #print "exception in code: " + data
                    else:
                        print '*** CsServer: remove socket: ', s.fileno()
                        s.close()
                        self.input.remove(s)

        for i in self.input:
            i.close()
            self.input.remove(i)
        self.server.close()
        print '*** CsServer: The server has been closed.'
