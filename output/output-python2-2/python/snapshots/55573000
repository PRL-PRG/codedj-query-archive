#!/usr/bin/env python

import gtk
import Config

from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.radiotoolbutton import RadioToolButton
from gettext import gettext as _

class mainToolbar(gtk.Toolbar):
    def __init__(self,toolbox, synthLab):
        gtk.Toolbar.__init__(self)
        
        def _insertSeparator():
            self.separator = gtk.SeparatorToolItem()
            self.separator.set_draw(True)
            self.insert(self.separator,-1)
            self.separator.show()        
            
        self.toolbox = toolbox
        self.synthLab = synthLab
        
        self.tooltips = gtk.Tooltips()

        self.durationSliderAdj = gtk.Adjustment(2, .5, 10, .01, .01, 0)
        self.durationSliderAdj.connect("value_changed" , self.synthLab.handleDuration)
        self.durationSlider =  gtk.HScale(adjustment = self.durationSliderAdj)
        self.durationSlider.set_size_request(250,15)
        self.durationSlider.set_inverted(False)
        #self.durationSlider.set_draw_value(False)
        self.durationSliderTool = gtk.ToolItem()
        self.durationSliderTool.add(self.durationSlider)
        self.insert(self.durationSliderTool, -1)
        self.durationSlider.show()
        self.durationSliderTool.show()
        self.durationSliderTool.set_tooltip(self.tooltips, _('Duration'))
        
        _insertSeparator()
        _insertSeparator()
        
        self.synthRec1Button = ToggleToolButton('rec1')
        self.synthRec1Button.connect('clicked',self.synthLab.recordSound,1)
        self.insert(self.synthRec1Button, -1)
        self.synthRec1Button.show()
        self.synthRec1Button.set_tooltip(_('Record Synth sound into slot 1'))
        
        self.synthRec2Button = ToggleToolButton('rec2')
        self.synthRec2Button.connect('clicked',self.synthLab.recordSound,2)
        self.insert(self.synthRec2Button, -1)
        self.synthRec2Button.show()
        self.synthRec2Button.set_tooltip(_('Record Synth sound into slot 2'))
        
        self.synthRec3Button = ToggleToolButton('rec3')
        self.synthRec3Button.connect('clicked',self.synthLab.recordSound,3)
        self.insert(self.synthRec3Button, -1)
        self.synthRec3Button.show()
        self.synthRec3Button.set_tooltip(_('Record Synth sound into slot 3'))
        
        self.synthRec4Button = ToggleToolButton('rec4')
        self.synthRec4Button.connect('clicked',self.synthLab.recordSound,4)
        self.insert(self.synthRec4Button, -1)
        self.synthRec4Button.show()
        self.synthRec4Button.set_tooltip(_('Record Synth sound into slot 4'))
        
        self.synthRec5Button = ToggleToolButton('rec5')
        self.synthRec5Button.connect('clicked',self.synthLab.recordSound,5)
        self.insert(self.synthRec5Button, -1)
        self.synthRec5Button.show()
        self.synthRec5Button.set_tooltip(_('Record Synth sound into slot 5'))
        
        self.synthRec6Button = ToggleToolButton('rec6')
        self.synthRec6Button.connect('clicked',self.synthLab.recordSound,6)
        self.insert(self.synthRec6Button, -1)
        self.synthRec6Button.show()
        self.synthRec6Button.set_tooltip(_('Record Synth sound into slot 6'))
        
class presetToolbar(gtk.Toolbar):
    def __init__(self,toolbox, synthLab):
        gtk.Toolbar.__init__(self)
        
        def _insertSeparator():
            self.separator = gtk.SeparatorToolItem()
            self.separator.set_draw(True)
            self.insert(self.separator,-1)
            self.separator.show()        
            
        self.toolbox = toolbox
        self.synthLab = synthLab
        
        self.preset1Button = ToggleToolButton('preset1')
        self.preset1Button.connect('clicked',self.synthLab.recordSound,1)
        self.insert(self.preset1Button, -1)
        self.preset1Button.show()
        self.preset1Button.set_tooltip(_('Record Synth sound into slot 1'))
        
