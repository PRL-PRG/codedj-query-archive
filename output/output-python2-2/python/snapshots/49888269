import string,cgi,time
import subprocess
import threading
from Sensor import Sensor
import Driver
from os import curdir, sep
from BaseHTTPServer import BaseHTTPRequestHandler, HTTPServer
from SocketServer import ThreadingMixIn

class ThreadedHTTPServer(ThreadingMixIn, HTTPServer):
    """Handle requests in a separate thread."""

class MyHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        try:
            bits = string.split(self.path, "/")
            if len(bits) > 4:
                id = bits[1]
                getset = bits[2]
                param = bits[3]
                value = bits[4]
                for f in Driver.Drivers:
                    if f[0] == int(id):
                        self.send_response(200)
                        self.send_header('Content-type',    'text/html')
                        self.end_headers()

                        if getset == "set":
                            f[1].setParameter(param, value)
                            self.wfile.write("OK")
                            return
                        elif getset == "get":
                            params = f[1].getParameters()
                            for p in params:
                                if p[0] == param:
                                    self.wfile.write(p[1])
                            return
            self.send_response(200)
            self.send_header('Content-type',	'text/html')
            self.end_headers()
            self.wfile.write("Error")
            return
        except IOError:
            self.send_error(404,'File Not Found: %s' % self.path)

    def do_POST(self):
        return

class APIThread(threading.Thread):
    def run(self):
        try:
            server = ThreadedHTTPServer(('', 8090), MyHandler)
            print 'started httpserver...'
            server.serve_forever()
        except KeyboardInterrupt:
            print '^C received, shutting down server'
            server.socket.close()

class HTTP_API(Sensor):
    def __init__(self):
        Sensor.__init__(self)
        APIThread().start()
