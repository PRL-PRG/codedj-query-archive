#!/usr/bin/python
#-*- coding: utf-8 -*-

class Viewer(object):
    """
    Build a graphical rappresentation of model.
    """

    def __init__(self):
        self.lastRow = None

    def process(self, model):
        if self.lastRow is not None:
            new_text = model.main_text[self.lastRow + 1 : ]
        else:
            new_text = model.main_text

        self.lastRow = len(model.main_text) - 1
        return ''.join(new_text)

