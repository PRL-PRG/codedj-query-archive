#!/usr/bin/env python
"""
Custom widget: LineEdit with blinking background
"""

from PyQt4 import QtCore, QtGui

class QLineEditWErrState(QtGui.QLineEdit):
    
    __pyqtSignals__ = ("errorStateSet()", "errorStateReseted()")
    
    def __init__(self, *args):
        QtGui.QLineEdit.__init__(self, *args)
        self.resetTimeout()
        self.resetErrorCss()
        self._orig_css = self.styleSheet()
    
    @QtCore.pyqtSignature("setErrorState()")
    def setErrorState(self):
        self.emit(QtCore.SIGNAL("errorStateSet()"))
        self.setStyleSheet(self.error_css)
        QtCore.QTimer.singleShot(self.timeout, self.resetErrorState)

    @QtCore.pyqtSignature("resetErrorState()")
    def resetErrorState(self):
        self.setStyleSheet(self._orig_css)
        self.emit(QtCore.SIGNAL("errorStateReseted()"))

    def getErrorCss(self):
        return self.error_css
    
    def setErrorCss(self, value):
        self.error_css = value
    
    def resetErrorCss(self):
        self.error_css = 'background-color: antiquewhite'
    
    errorCss = QtCore.pyqtProperty("QString", getErrorCss, setErrorCss, resetErrorCss)
    
    def getTimeout(self):
        return self.timeout
    
    def setTimeout(self, value):
        self.timeout = value
    
    def resetTimeout(self):
        self.timeout = 800
    
    stateTimeout = QtCore.pyqtProperty("int", getTimeout, setTimeout, resetTimeout)
