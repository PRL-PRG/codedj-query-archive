#!/usr/bin/python
from Adafruit_BBIO import GPIO
import time

GPIO.setup("P8_12", GPIO.OUT)

for i in range(5):
    GPIO.output("P8_12", GPIO.HIGH)
    time.sleep(1)
    GPIO.output("P8_12", GPIO.LOW)
    time.sleep(1)
