#!/usr/bin/python
#-*- coding: utf-8 -*-

class Viewer(object):
    """
    Build a graphical rappresentation of model.
    """

    def __init__(self):
        self.last_row = None

    def process(self, model):
        new_text = model.main_text.get(self.last_row)
        bgcolor = None
        fgcolor = None

        if self.last_row is None:
            self.last_row = len(new_text) - 1
            bgcolor = model.main_bgcolor
            fgcolor = model.main_fgcolor
        else:
            self.last_row += len(new_text)

        return (''.join(new_text), bgcolor, fgcolor)

