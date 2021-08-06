#!/usr/bin/env python
"""
Custom widget: LineEdit with blinking background
"""

from PyQt4 import QtCore, QtGui

class QLineEditWErrState(QtGui.QLineEdit):
    
    def __init__(self, parent=None):
        QtGui.QLineEdit.__init__(self, parent)
        self.timeout = 800
        self.errorCss = 'background-color: antiquewhite'
        self._origCss = self.styleSheet()
    
    def setErrorState(self):
        self.emit(QtCore.SIGNAL("errorStateSet()"))
        self.setStyleSheet(self.errorCss)
        QtCore.QTimer.singleShot(self.timeout, self.resetErrorState)

    def resetErrorState(self):
        self.setStyleSheet(self._origCss)
        self.emit(QtCore.SIGNAL("errorStateReseted()"))

