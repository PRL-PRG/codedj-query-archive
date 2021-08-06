#!/usr/bin/env python
from plugins import *
from plugins import Sensor
from plugins import HTTP_API
from plugins import CapSwitch
import time
import os

CapSwitch.CapSwitch("/dev/ttyUSB0")

test = Lightbulb.connect(1, "44:A6:E5:03:27:F9", '71', '151')
test2 = Lightbulb.connect(2, "44:A6:E5:03:27:DF", '89', '119')

#test.setStatus(1)
#time.sleep(5)
#test.setParameter("color", 1)
#test.setParameter("temp", 144)
#time.sleep(5)
#test.setStatus(0)

#print test.getParameters()

#b = 100
#while b > 1:
#    test.setParameter("brightness", b)
#    b = b-1
#    time.sleep(0.1)

#test.setParameter("color", 1)
#test.setParameter("brightness", 100)

class TestHandler(Sensor.EventHandler):
    def __init__(self):
        Sensor.EventHandler.__init__(self)
    def handle(self, data):
        print data

class TestSensor(Sensor.Sensor):
    def __init__(self):
        Sensor.Sensor.__init__(self)
    def run(self):
        Sensor.EventQueue.put("test")

#TestHandler()
#TestSensor()

HTTP_API.HTTP_API()


while True:
    try:
        time.sleep(1)
    except KeyboardInterrupt:
        print '^C received, shutting down'
        os._exit(0)
