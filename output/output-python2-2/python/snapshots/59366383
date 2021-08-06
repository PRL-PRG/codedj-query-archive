# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'gui.ui'
#
# Created: Sun Nov 25 13:07:47 2007
#      by: PyQt4 UI code generator 4.3.1
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_dev_client(object):
    def setupUi(self, dev_client):
        dev_client.setObjectName("dev_client")
        dev_client.resize(QtCore.QSize(QtCore.QRect(0,0,1020,663).size()).expandedTo(dev_client.minimumSizeHint()))

        self.centralwidget = QtGui.QWidget(dev_client)
        self.centralwidget.setObjectName("centralwidget")

        self.gridlayout = QtGui.QGridLayout(self.centralwidget)
        self.gridlayout.setMargin(5)
        self.gridlayout.setSpacing(5)
        self.gridlayout.setObjectName("gridlayout")

        self.text_output = QtGui.QTextEdit(self.centralwidget)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(1)
        sizePolicy.setVerticalStretch(1)
        sizePolicy.setHeightForWidth(self.text_output.sizePolicy().hasHeightForWidth())
        self.text_output.setSizePolicy(sizePolicy)
        self.text_output.setMinimumSize(QtCore.QSize(760,0))
        self.text_output.setReadOnly(True)
        self.text_output.setObjectName("text_output")
        self.gridlayout.addWidget(self.text_output,0,0,1,1)

        self.rightwidget = QtGui.QWidget(self.centralwidget)
        self.rightwidget.setWindowModality(QtCore.Qt.NonModal)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.rightwidget.sizePolicy().hasHeightForWidth())
        self.rightwidget.setSizePolicy(sizePolicy)
        self.rightwidget.setMinimumSize(QtCore.QSize(245,610))
        self.rightwidget.setObjectName("rightwidget")
        self.gridlayout.addWidget(self.rightwidget,0,1,2,1)

        self.text_input = QtGui.QLineEdit(self.centralwidget)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(1)
        sizePolicy.setVerticalStretch(1)
        sizePolicy.setHeightForWidth(self.text_input.sizePolicy().hasHeightForWidth())
        self.text_input.setSizePolicy(sizePolicy)
        self.text_input.setMinimumSize(QtCore.QSize(760,0))
        self.text_input.setObjectName("text_input")
        self.gridlayout.addWidget(self.text_input,1,0,1,1)
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

