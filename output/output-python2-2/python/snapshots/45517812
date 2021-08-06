import pymsn.gnet
import pymsn.gnet.proxy
import pymsn.gnet.proxy.HTTPConnect
import gobject

mainloop = gobject.MainLoop()

def sent(client, data, length):
    print '>>>', data

def received(client, data, length):
    print '<<<', data

def error(client, err_code):
    print err_code

def status_change(client, param):
    if client.status == pymsn.gnet.IoStatus.OPEN:
        print "OPEN"
        c.send('GET / HTTP/1.1\r\nHost: localhost:9443\r\n\r\n')
    elif client.status == pymsn.gnet.IoStatus.CLOSED:
        print "CLOSED"
        mainloop.quit()
    elif client.status == pymsn.gnet.IoStatus.OPENING:
        print "OPENING"
    elif client.status == pymsn.gnet.IoStatus.CLOSING:
        print "CLOSING"

#c = pymsn.gnet.io.SSLTCPClient('localhost', 9443)
p = pymsn.gnet.proxy.ProxyFactory('http://127.0.0.1:8123')
client = pymsn.gnet.io.SSLTCPClient('www.google.com', 443)
c = pymsn.gnet.proxy.HTTPConnect.HTTPConnectProxy(client, p)

c.connect("sent", sent)
c.connect("received", received)
c.connect("notify::status", status_change)
c.connect("error", error)

c.open()
mainloop.run()
