# Based on https://github.com/moosd/ReverseEngineeredMiLightBluetooth
# By S. Mookerjee 2015

from Driver import Driver
import Sensor
import subprocess
import sys, string
import os.path
import pickle

class connect(Driver):
    def __init__(self, id, mac, id1, id2):
        Driver.__init__(self, id)
        self.mac = mac
        self.id1 = id1
        self.id2 = id2

        if os.path.isfile("persist/"+str(self.id)):
            f = open("persist/"+str(self.id), "rb")
            self.setParameters(pickle.load(f))
            f.close()
        else: 
            self.status = 0
            self.brightness = 100
            self.mode = 1 # 1=temp, 0=color
            self.color = 0
            self.temp = 100
            self.setStatus(self.status)
            self.apply()

    def getStatus(self):
        return self.status

    def setStatus(self, status):
        self.status = status
        if status == 0:
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[85, 161, " + self.id1 + ", " + self.id2 + ", 2, 2, 0, 0, 0, 0, 0]")])
        else:
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[32, 161, " + self.id1 + ", " + self.id2 + ", 2, 1, 0, 0, 0, 0, 0]")])
        Sensor.EventQueue.put(["lightbulb", self.id, self.status])

    def getParameters(self):
        # [[name, value, conditional on other objectid
        return [ ["status", self.status, [], "switch"],
                 ["mode", self.mode, [], "switch"],
                 ["color", self.color, [[self.id, "mode", 0]], "hue" ],
                 ["temp", self.temp, [[self.id, "mode", 1]], "slider"],
                 ["brightness", self.brightness, [], "slider"] ]

    def setParameter(self, param, value):
        if param == "status":
            self.setStatus(int(value))
        elif param == "mode":
            self.mode = int(value)
        elif param == "color":
            self.color = int(value)
            self.mode = 0
        elif param == "temp":
            self.temp = int(value)
            self.mode = 1
        elif param == "brightness":
            self.brightness = int(value)
        self.apply()

    def setParameterInternal(self, param, value):
        if param == "status":
            self.setStatus(int(value))
        elif param == "mode":
            self.mode = int(value)
        elif param == "color":
            self.color = int(value)
        elif param == "temp":
            self.temp = int(value)
        elif param == "brightness":
            self.brightness = int(value)

    def setParameters(self, list):
        internal = False
        if list[0][0] == "status":
            internal = True
        for a in list:
            if internal == True:
                self.setParameterInternal(a[0], a[1])
            else:
                self.setParameter(a[0], a[1])
        self.apply()

    def apply(self):
        if self.mode == 0:
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[85, 161, " + self.id1 + ", " + self.id2 + ", 2, 4, "+str(self.color)+", 100, 0, 0, 0]")])
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[85, 161, " + self.id1 + ", " + self.id2 + ", 2, 5, "+str(self.color)+", "+str(self.brightness)+", 0, 0, 0]")])
        elif self.mode == 1:
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[20, 161, " + self.id1 + ", " + self.id2 + ", 4, 4, "+str(self.temp)+", 255, 0, 0, 0]")])
            subprocess.call(['/usr/bin/gatttool', '-b', self.mac, '--char-write-req', '-a', '0x0012', '-n', self.createPacket("[20, 161, " + self.id1 + ", " + self.id2 + ", 4, 5, "+str(self.temp)+", "+str(self.brightness)+", 0, 0, 0]")])
        f = open( "persist/"+str(self.id), "wb" )
        pickle.dump(self.getParameters(), f)
        f.close()

    def createPacket(self, data):
        input = eval(data)

        k = input[0]
        # checksum
        j = 0
        i = 0

        while i <= 10:
            j += input[i] & 0xff
            i += 1
        checksum = ((( (k ^ j) & 0xff) + 131) & 0xff)

        xored = [(s&0xff)^k for s in input]

        offs = [0, 16, 24, 1, 129, 55, 169, 87, 35, 70, 23, 0]

        adds = [x+y&0xff for(x,y) in zip(xored, offs)]

        adds[0] = k
        adds.append(checksum)

        hexs = [hex(x) for x in adds]
        hexs = [x[2:] for x in hexs]
        hexs = [x.zfill(2) for x in hexs]

        return ''.join(hexs)


    
