#!/usr/bin/python
#-*- coding: utf-8 -*-

import re
import sys
import Queue

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import SIGNAL

from gui_ui import Ui_dev_client
import event_type
import viewer
import gui_option

class Gui(QtGui.QMainWindow, Ui_dev_client):
    """
    The Gui class written with Qt, that inherits the real gui interface
    designed by Qt-designer.
    """

    def __init__(self, config, q_app_gui, q_gui_app):
        self.config = config
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.app = QtGui.QApplication([])

        translator = QtCore.QTranslator()
        translator.load(config['translation']['path'])
        QtGui.QApplication.installTranslator(translator)
        
        QtGui.QMainWindow.__init__(self)
        self.setupUi(self)

        self.connect(self.action_exit, SIGNAL("triggered()"),
                     self._endApplication)

        self.connect(self.action_connect, SIGNAL("triggered()"),
                     self._connect)

        self.connect(self.action_option, SIGNAL("triggered()"),
                     self._showOption)

        self.connect(self.text_input, SIGNAL("returnPressed()"),
                     self._sendText)

        timer = QtCore.QTimer(self)
        self.connect(timer, SIGNAL("timeout()"), self._processIncoming)
        timer.start(100)

        self.text_input.setFocus()
        self.mainViewer = viewer.Viewer()

    def closeEvent(self, event):
        self._endApplication()
        event.accept()

    def _showOption(self):
        opt = gui_option.GuiOption(self, self.config)
        opt.show()

    def _connect(self):
        self.q_gui_app.put((event_type.CONNECT, ""))

    def _endApplication(self):
        self.q_gui_app.put((event_type.END_APP, ""))

    def _sendText(self):
        self.q_gui_app.put((event_type.MSG, unicode(self.text_input.text())))
        self.text_input.clear()

    def _setOutputColors(self, bg, fg):
        """
        Set output default colors.
        """

        style = str(self.text_output.styleSheet())
        m = re.search('QTextEdit\s*{(.*)}', style)
        if m:
            oldstyle = m.group(1)
            tmp = [el.split(':') for el in oldstyle.split(';')]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        if bg: d['background-color'] = '#' + bg
        if fg: d['color'] = '#' + fg

        newstyle = ';'.join([k + ':' + v for k,v in d.iteritems()])

        if oldstyle:
            self.text_output.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            self.text_output.setStyleSheet('QTextEdit {%s}' % style)

    def _processIncoming(self):
        try:
            cmd, msg = self.q_app_gui.get(0)
            if cmd == event_type.MODEL:
                text, bg, fg = self.mainViewer.process(msg)
                self.text_output.insertHtml(text)
                self.text_output.moveCursor(QtGui.QTextCursor.End)
                if bg or fg:
                    self._setOutputColors(bg, fg)

        except Queue.Empty:
            pass

    def mainLoop(self):
        self.show()
        sys.exit(self.app.exec_())
