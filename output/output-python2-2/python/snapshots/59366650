#!/usr/bin/python
#-*- coding: utf-8 -*-

from model import Model

class Parser(object):
    """
    The Parser class build a Model of data received.
    """

    def __init__(self):
        self.model = Model()

    def parse(self, data):
        self.model.mainText.append(data.rstrip('\r\n'))

