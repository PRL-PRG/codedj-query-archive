# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'designed_ui.ui'
#
# Created: Tue Apr 29 12:02:28 2008
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_DesignedWidget(object):
    def setupUi(self, DesignedWidget):
        DesignedWidget.setObjectName("DesignedWidget")
        DesignedWidget.resize(QtCore.QSize(QtCore.QRect(0,0,219,118).size()).expandedTo(DesignedWidget.minimumSizeHint()))

        self.gridlayout = QtGui.QGridLayout(DesignedWidget)
        self.gridlayout.setObjectName("gridlayout")

        self.seriesLabel = QtGui.QLabel(DesignedWidget)
        self.seriesLabel.setObjectName("seriesLabel")
        self.gridlayout.addWidget(self.seriesLabel,0,0,1,1)

        self.seriesEdit = QtGui.QLineEdit(DesignedWidget)
        self.seriesEdit.setObjectName("seriesEdit")
        self.gridlayout.addWidget(self.seriesEdit,0,1,1,1)

        self.numberLabel = QtGui.QLabel(DesignedWidget)
        self.numberLabel.setObjectName("numberLabel")
        self.gridlayout.addWidget(self.numberLabel,1,0,1,1)

        self.numberEdit = QtGui.QLineEdit(DesignedWidget)
        self.numberEdit.setObjectName("numberEdit")
        self.gridlayout.addWidget(self.numberEdit,1,1,1,1)

        self.hboxlayout = QtGui.QHBoxLayout()
        self.hboxlayout.setObjectName("hboxlayout")

        spacerItem = QtGui.QSpacerItem(40,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout.addItem(spacerItem)

        self.goButton = QtGui.QPushButton(DesignedWidget)
        self.goButton.setObjectName("goButton")
        self.hboxlayout.addWidget(self.goButton)
        self.gridlayout.addLayout(self.hboxlayout,2,0,1,2)
        self.seriesLabel.setBuddy(self.seriesEdit)
        self.numberLabel.setBuddy(self.numberEdit)

        self.retranslateUi(DesignedWidget)
        QtCore.QMetaObject.connectSlotsByName(DesignedWidget)

    def retranslateUi(self, DesignedWidget):
        self.seriesLabel.setText(QtGui.QApplication.translate("DesignedWidget", "&Series", None, QtGui.QApplication.UnicodeUTF8))
        self.numberLabel.setText(QtGui.QApplication.translate("DesignedWidget", "&Number", None, QtGui.QApplication.UnicodeUTF8))
        self.goButton.setText(QtGui.QApplication.translate("DesignedWidget", "Go", None, QtGui.QApplication.UnicodeUTF8))



if __name__ == "__main__":
    import sys
    app = QtGui.QApplication(sys.argv)
    DesignedWidget = QtGui.QWidget()
    ui = Ui_DesignedWidget()
    ui.setupUi(DesignedWidget)
    DesignedWidget.show()
    sys.exit(app.exec_())
