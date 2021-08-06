#!/usr/bin/env python
# -*- encoding: utf-8 -*-
"""
Example of dynamic-loaded .ui widget with ui as attribute
"""

from PyQt4 import QtCore, QtGui, uic

class WidgetWithDynLoadedUiAsAttr(QtGui.QWidget):
    def __init__(self, parent=None):
        QtGui.QWidget.__init__(self, parent)
        uiClass, qtBaseClass = uic.loadUiType('designed_ui.ui')
        self.ui = uiClass()
        self.ui.setupUi(self)

if __name__ == '__main__':
    import sys
    app = QtGui.QApplication(sys.argv)
    widget = WidgetWithDynLoadedUiAsAttr()
    widget.show()
    sys.exit(app.exec_())
