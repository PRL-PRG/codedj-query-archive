# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'gui.ui'
#
# Created: Wed Oct 24 22:14:57 2007
#      by: PyQt4 UI code generator 4.3.1
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_dev_client(object):
    def setupUi(self, dev_client):
        dev_client.setObjectName("dev_client")
        dev_client.resize(QtCore.QSize(QtCore.QRect(0,0,800,600).size()).expandedTo(dev_client.minimumSizeHint()))

        self.centralwidget = QtGui.QWidget(dev_client)
        self.centralwidget.setObjectName("centralwidget")

        self.vboxlayout = QtGui.QVBoxLayout(self.centralwidget)
        self.vboxlayout.setSpacing(6)
        self.vboxlayout.setMargin(9)
        self.vboxlayout.setObjectName("vboxlayout")

        self.text_output = QtGui.QTextEdit(self.centralwidget)
        self.text_output.setReadOnly(True)
        self.text_output.setObjectName("text_output")
        self.vboxlayout.addWidget(self.text_output)

        self.text_input = QtGui.QLineEdit(self.centralwidget)
        self.text_input.setObjectName("text_input")
        self.vboxlayout.addWidget(self.text_input)
        dev_client.setCentralWidget(self.centralwidget)

        self.toolBar = QtGui.QToolBar(dev_client)
        self.toolBar.setContextMenuPolicy(QtCore.Qt.PreventContextMenu)
        self.toolBar.setMovable(False)
        self.toolBar.setOrientation(QtCore.Qt.Horizontal)
        self.toolBar.setIconSize(QtCore.QSize(32,32))
        self.toolBar.setObjectName("toolBar")
        dev_client.addToolBar(QtCore.Qt.TopToolBarArea,self.toolBar)

        self.action_connect = QtGui.QAction(dev_client)
        self.action_connect.setIcon(QtGui.QIcon("../../resources/images/connect.png"))
        self.action_connect.setObjectName("action_connect")

        self.action_exit = QtGui.QAction(dev_client)
        self.action_exit.setObjectName("action_exit")

        self.action_option = QtGui.QAction(dev_client)
        self.action_option.setIcon(QtGui.QIcon("../../resources/images/option.png"))
        self.action_option.setObjectName("action_option")
        self.toolBar.addAction(self.action_connect)
        self.toolBar.addAction(self.action_option)

        self.retranslateUi(dev_client)
        QtCore.QMetaObject.connectSlotsByName(dev_client)

    def retranslateUi(self, dev_client):
        dev_client.setWindowTitle(QtGui.QApplication.translate("dev_client", "DevClient", None, QtGui.QApplication.UnicodeUTF8))
        self.text_output.setStyleSheet(QtGui.QApplication.translate("dev_client", "QTextEdit { background-color: #FFFFFF; font: 10pt \"Courier\"}", None, QtGui.QApplication.UnicodeUTF8))
        self.action_connect.setText(QtGui.QApplication.translate("dev_client", "Connect", None, QtGui.QApplication.UnicodeUTF8))
        self.action_exit.setText(QtGui.QApplication.translate("dev_client", "Exit", None, QtGui.QApplication.UnicodeUTF8))
        self.action_option.setText(QtGui.QApplication.translate("dev_client", "Option", None, QtGui.QApplication.UnicodeUTF8))

