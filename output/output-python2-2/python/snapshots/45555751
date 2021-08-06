#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Example of dynamic-loaded .ui widget
"""

from PyQt4 import QtCore, QtGui, uic

class InheritedWidgetWithDynLoadedUi(QtGui.QWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        uic.loadUi('designed_ui.ui', self)

if __name__ == '__main__':
    import sys
    app = QtGui.QApplication(sys.argv)
    widget = InheritedWidgetWithDynLoadedUi()
    widget.show()
    sys.exit(app.exec_())
