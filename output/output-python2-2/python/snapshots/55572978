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
        
        _insertSeparator()
        _insertSeparator()
        _insertSeparator()
        _insertSeparator()
        _insertSeparator()
        _insertSeparator()
        
        self.resetButton = ToolButton('reset')
        self.resetButton.connect('clicked',self.synthLab.handleReset)
        self.insert(self.resetButton, -1)
        self.resetButton.show()
        self.resetButton.set_tooltip(_('Reset the worktable'))
        
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
        
        self.preset1Button = RadioToolButton('preset1', group = None)
        self.preset1Button.connect('clicked',self.synthLab.presetCallback,1)
        self.insert(self.preset1Button, -1)
        self.preset1Button.show()
        self.preset1Button.set_tooltip(_('Preset 1'))
        
        self.preset2Button = RadioToolButton('preset2', group = self.preset1Button)
        self.preset2Button.connect('clicked',self.synthLab.presetCallback,2)
        self.insert(self.preset2Button, -1)
        self.preset2Button.show()
        self.preset2Button.set_tooltip(_('Preset 2'))
        
        self.preset3Button = RadioToolButton('preset3', group = self.preset1Button)
        self.preset3Button.connect('clicked',self.synthLab.presetCallback,3)
        self.insert(self.preset3Button, -1)
        self.preset3Button.show()
        self.preset3Button.set_tooltip(_('Preset 3'))
        
        self.preset4Button = RadioToolButton('preset4', group = self.preset1Button)
        self.preset4Button.connect('clicked',self.synthLab.presetCallback,4)
        self.insert(self.preset4Button, -1)
        self.preset4Button.show()
        self.preset4Button.set_tooltip(_('Preset 4'))
        
        self.preset4Button = RadioToolButton('preset4', group = self.preset1Button)
        self.preset4Button.connect('clicked',self.synthLab.presetCallback,4)
        self.insert(self.preset4Button, -1)
        self.preset4Button.show()
        self.preset4Button.set_tooltip(_('Preset 4'))
        
        self.preset5Button = RadioToolButton('preset5', group = self.preset1Button)
        self.preset5Button.connect('clicked',self.synthLab.presetCallback,5)
        self.insert(self.preset5Button, -1)
        self.preset5Button.show()
        self.preset5Button.set_tooltip(_('Preset 5'))
        
        self.preset6Button = RadioToolButton('preset6', group = self.preset1Button)
        self.preset6Button.connect('clicked',self.synthLab.presetCallback,6)
        self.insert(self.preset6Button, -1)
        self.preset6Button.show()
        self.preset6Button.set_tooltip(_('Preset 6'))
        
        self.preset7Button = RadioToolButton('preset7', group = self.preset1Button)
        self.preset7Button.connect('clicked',self.synthLab.presetCallback,7)
        self.insert(self.preset7Button, -1)
        self.preset7Button.show()
        self.preset7Button.set_tooltip(_('Preset 7'))
        
        self.preset8Button = RadioToolButton('preset8', group = self.preset1Button)
        self.preset8Button.connect('clicked',self.synthLab.presetCallback,8)
        self.insert(self.preset8Button, -1)
        self.preset8Button.show()
        self.preset8Button.set_tooltip(_('Preset 8'))
        
        self.preset9Button = RadioToolButton('preset9', group = self.preset1Button)
        self.preset9Button.connect('clicked',self.synthLab.presetCallback,9)
        self.insert(self.preset9Button, -1)
        self.preset9Button.show()
        self.preset9Button.set_tooltip(_('Preset 9'))
        
        self.preset10Button = RadioToolButton('preset10', group = self.preset1Button)
        self.preset10Button.connect('clicked',self.synthLab.presetCallback,10)
        self.insert(self.preset10Button, -1)
        self.preset10Button.show()
        self.preset10Button.set_tooltip(_('Preset 10'))