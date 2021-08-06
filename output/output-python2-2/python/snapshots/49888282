import Sensor
import Driver
import Lightbulb
import serial
import threading

class CapSwitch(object):
    def __init__(self, port):
        self.ser = serial.Serial(port, 9600)
        self.ser.open()
        CapSwitchHandler(self.ser)
        SwitchReader(self.ser).start()

class CapSwitchHandler(Sensor.EventHandler):
    def __init__(self, ser):
        Sensor.EventHandler.__init__(self)
        self.ser = ser
    def handle(self, data):
        if data[0] == "lightbulb":
            switched=0
            for a in Driver.Drivers:
                if isinstance(a[1], Lightbulb.connect):
                    if a[1].getStatus() == 1:
                        switched = 1
            if switched == 1:
                self.ser.write("y")
            else:
                self.ser.write("n")

class SwitchReader(threading.Thread):
    def __init__(self, ser):
        threading.Thread.__init__(self)
        self.ser = ser

    def run(self):
        while True:
            data = self.ser.readline().strip()
            if data == "ON":
                for a in Driver.Drivers:
                    if isinstance(a[1], Lightbulb.connect):
                        a[1].setParameter("status", 1)
            elif data == "OFF":
                for a in Driver.Drivers:
                    if isinstance(a[1], Lightbulb.connect):
                        a[1].setParameter("status", 0)
