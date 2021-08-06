#!/usr/bin/python
#-*- coding: utf-8 -*-

class Viewer(object):
    """
    Build a graphical rappresentation of model.
    """
    
    def __init__(self):
        self.lastRow = None
        
    def process(self, model):
        if self.lastRow and model.mainText.count(self.lastRow):                           
            new_text = model.mainText[model.mainText.index(self.lastRow) + 1 : ]
        else:
            new_text = model.mainText
            
        self.lastRow = model.mainText[-1]            
        return ''.join(new_text)
        
