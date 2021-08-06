#!/usr/bin/python
from Adafruit_BBIO import GPIO
import time

GPIO.setup("USR3", GPIO.OUT)

for i in range(5):
    GPIO.output("USR3", GPIO.HIGH)
    time.sleep(1)
    GPIO.output("USR3", GPIO.LOW)
    time.sleep(1)
