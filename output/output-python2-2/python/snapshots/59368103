# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'gui_option.ui'
#
# Created: Tue Jul 15 20:59:19 2008
#      by: PyQt4 UI code generator 4.3.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_option(object):
    def setupUi(self, option):
        option.setObjectName("option")
        option.setWindowModality(QtCore.Qt.ApplicationModal)
        option.resize(QtCore.QSize(QtCore.QRect(0,0,415,375).size()).expandedTo(option.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(option.sizePolicy().hasHeightForWidth())
        option.setSizePolicy(sizePolicy)
        option.setMinimumSize(QtCore.QSize(415,375))
        option.setMaximumSize(QtCore.QSize(415,375))

        self.list_option = QtGui.QListWidget(option)
        self.list_option.setGeometry(QtCore.QRect(5,5,90,325))
        self.list_option.setAutoFillBackground(True)
        self.list_option.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        self.list_option.setTextElideMode(QtCore.Qt.ElideNone)
        self.list_option.setMovement(QtGui.QListView.Static)
        self.list_option.setFlow(QtGui.QListView.TopToBottom)
        self.list_option.setProperty("isWrapping",QtCore.QVariant(False))
        self.list_option.setSpacing(2)
        self.list_option.setViewMode(QtGui.QListView.IconMode)
        self.list_option.setModelColumn(0)
        self.list_option.setUniformItemSizes(True)
        self.list_option.setObjectName("list_option")

        self.page_container = QtGui.QStackedWidget(option)
        self.page_container.setGeometry(QtCore.QRect(95,0,316,326))
        self.page_container.setObjectName("page_container")

        self.conn_page = QtGui.QWidget()
        self.conn_page.setObjectName("conn_page")

        self.layoutWidget = QtGui.QWidget(self.conn_page)
        self.layoutWidget.setGeometry(QtCore.QRect(4,6,311,191))
        self.layoutWidget.setObjectName("layoutWidget")

        self.vboxlayout = QtGui.QVBoxLayout(self.layoutWidget)
        self.vboxlayout.setSpacing(5)
        self.vboxlayout.setObjectName("vboxlayout")

        self.gridlayout = QtGui.QGridLayout()
        self.gridlayout.setObjectName("gridlayout")

        self.label_conn = QtGui.QLabel(self.layoutWidget)
        self.label_conn.setObjectName("label_conn")
        self.gridlayout.addWidget(self.label_conn,0,0,1,2)

        self.list_conn = QtGui.QComboBox(self.layoutWidget)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_conn.sizePolicy().hasHeightForWidth())
        self.list_conn.setSizePolicy(sizePolicy)
        self.list_conn.setObjectName("list_conn")
        self.gridlayout.addWidget(self.list_conn,0,2,1,2)

        self.label_name_conn = QtGui.QLabel(self.layoutWidget)
        self.label_name_conn.setMinimumSize(QtCore.QSize(45,0))
        self.label_name_conn.setMaximumSize(QtCore.QSize(45,16777215))
        self.label_name_conn.setObjectName("label_name_conn")
        self.gridlayout.addWidget(self.label_name_conn,1,0,1,1)

        self.name_conn = QtGui.QLineEdit(self.layoutWidget)
        self.name_conn.setObjectName("name_conn")
        self.gridlayout.addWidget(self.name_conn,1,2,1,2)

        self.label_host_conn = QtGui.QLabel(self.layoutWidget)
        self.label_host_conn.setMinimumSize(QtCore.QSize(45,0))
        self.label_host_conn.setMaximumSize(QtCore.QSize(45,16777215))
        self.label_host_conn.setObjectName("label_host_conn")
        self.gridlayout.addWidget(self.label_host_conn,2,0,1,1)

        spacerItem = QtGui.QSpacerItem(45,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout.addItem(spacerItem,2,1,1,1)

        self.host_conn = QtGui.QLineEdit(self.layoutWidget)
        self.host_conn.setObjectName("host_conn")
        self.gridlayout.addWidget(self.host_conn,2,2,1,2)

        self.label_port_conn = QtGui.QLabel(self.layoutWidget)
        self.label_port_conn.setMinimumSize(QtCore.QSize(45,0))
        self.label_port_conn.setMaximumSize(QtCore.QSize(45,16777215))
        self.label_port_conn.setObjectName("label_port_conn")
        self.gridlayout.addWidget(self.label_port_conn,3,0,1,1)

        spacerItem1 = QtGui.QSpacerItem(135,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout.addItem(spacerItem1,3,1,1,2)

        self.port_conn = QtGui.QLineEdit(self.layoutWidget)
        self.port_conn.setMaxLength(8)
        self.port_conn.setObjectName("port_conn")
        self.gridlayout.addWidget(self.port_conn,3,3,1,1)

        spacerItem2 = QtGui.QSpacerItem(45,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout.addItem(spacerItem2,1,1,1,1)
        self.vboxlayout.addLayout(self.gridlayout)

        spacerItem3 = QtGui.QSpacerItem(271,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.vboxlayout.addItem(spacerItem3)

        self.hboxlayout = QtGui.QHBoxLayout()
        self.hboxlayout.setSpacing(5)
        self.hboxlayout.setContentsMargins(-1,-1,0,-1)
        self.hboxlayout.setObjectName("hboxlayout")

        spacerItem4 = QtGui.QSpacerItem(20,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout.addItem(spacerItem4)

        self.save_conn = QtGui.QPushButton(self.layoutWidget)
        self.save_conn.setMinimumSize(QtCore.QSize(0,28))
        self.save_conn.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_conn.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_conn.setObjectName("save_conn")
        self.hboxlayout.addWidget(self.save_conn)

        self.delete_conn = QtGui.QPushButton(self.layoutWidget)
        self.delete_conn.setMinimumSize(QtCore.QSize(0,28))
        self.delete_conn.setMaximumSize(QtCore.QSize(16777215,28))
        self.delete_conn.setIcon(QtGui.QIcon(":/images/button-cancel.png"))
        self.delete_conn.setObjectName("delete_conn")
        self.hboxlayout.addWidget(self.delete_conn)
        self.vboxlayout.addLayout(self.hboxlayout)
        self.page_container.addWidget(self.conn_page)

        self.account_page = QtGui.QWidget()
        self.account_page.setObjectName("account_page")

        self.save_account = QtGui.QCheckBox(self.account_page)
        self.save_account.setGeometry(QtCore.QRect(5,10,115,22))
        self.save_account.setObjectName("save_account")

        self.box_prompt = QtGui.QGroupBox(self.account_page)
        self.box_prompt.setGeometry(QtCore.QRect(5,165,306,121))
        self.box_prompt.setObjectName("box_prompt")

        self.layoutWidget1 = QtGui.QWidget(self.box_prompt)
        self.layoutWidget1.setGeometry(QtCore.QRect(0,10,306,111))
        self.layoutWidget1.setObjectName("layoutWidget1")

        self.gridlayout1 = QtGui.QGridLayout(self.layoutWidget1)
        self.gridlayout1.setHorizontalSpacing(0)
        self.gridlayout1.setVerticalSpacing(5)
        self.gridlayout1.setObjectName("gridlayout1")

        self.label = QtGui.QLabel(self.layoutWidget1)
        self.label.setMinimumSize(QtCore.QSize(100,0))
        self.label.setMaximumSize(QtCore.QSize(100,16777215))
        self.label.setObjectName("label")
        self.gridlayout1.addWidget(self.label,0,0,1,1)

        self.normal_prompt = QtGui.QLineEdit(self.layoutWidget1)
        self.normal_prompt.setObjectName("normal_prompt")
        self.gridlayout1.addWidget(self.normal_prompt,0,1,1,2)

        self.label_2 = QtGui.QLabel(self.layoutWidget1)
        self.label_2.setMinimumSize(QtCore.QSize(100,0))
        self.label_2.setMaximumSize(QtCore.QSize(100,16777215))
        self.label_2.setObjectName("label_2")
        self.gridlayout1.addWidget(self.label_2,1,0,1,1)

        self.fight_prompt = QtGui.QLineEdit(self.layoutWidget1)
        self.fight_prompt.setObjectName("fight_prompt")
        self.gridlayout1.addWidget(self.fight_prompt,1,1,1,2)

        spacerItem5 = QtGui.QSpacerItem(221,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout1.addItem(spacerItem5,2,0,1,2)

        self.save_prompt = QtGui.QPushButton(self.layoutWidget1)
        self.save_prompt.setMinimumSize(QtCore.QSize(0,28))
        self.save_prompt.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_prompt.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_prompt.setObjectName("save_prompt")
        self.gridlayout1.addWidget(self.save_prompt,2,2,1,1)

        self.layoutWidget2 = QtGui.QWidget(self.account_page)
        self.layoutWidget2.setGeometry(QtCore.QRect(6,31,306,141))
        self.layoutWidget2.setObjectName("layoutWidget2")

        self.gridlayout2 = QtGui.QGridLayout(self.layoutWidget2)
        self.gridlayout2.setContentsMargins(-1,-1,3,-1)
        self.gridlayout2.setHorizontalSpacing(0)
        self.gridlayout2.setVerticalSpacing(5)
        self.gridlayout2.setObjectName("gridlayout2")

        self.label_conn_account = QtGui.QLabel(self.layoutWidget2)
        self.label_conn_account.setObjectName("label_conn_account")
        self.gridlayout2.addWidget(self.label_conn_account,0,0,1,1)

        self.list_conn_account = QtGui.QComboBox(self.layoutWidget2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_conn_account.sizePolicy().hasHeightForWidth())
        self.list_conn_account.setSizePolicy(sizePolicy)
        self.list_conn_account.setMinimumSize(QtCore.QSize(150,0))
        self.list_conn_account.setObjectName("list_conn_account")
        self.gridlayout2.addWidget(self.list_conn_account,0,1,1,1)

        self.line = QtGui.QFrame(self.layoutWidget2)
        self.line.setFrameShape(QtGui.QFrame.HLine)
        self.line.setFrameShadow(QtGui.QFrame.Sunken)
        self.line.setObjectName("line")
        self.gridlayout2.addWidget(self.line,1,0,1,2)

        spacerItem6 = QtGui.QSpacerItem(20,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.gridlayout2.addItem(spacerItem6,2,0,1,2)

        self.label_account_account = QtGui.QLabel(self.layoutWidget2)
        self.label_account_account.setObjectName("label_account_account")
        self.gridlayout2.addWidget(self.label_account_account,3,0,1,1)

        self.list_account = QtGui.QComboBox(self.layoutWidget2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_account.sizePolicy().hasHeightForWidth())
        self.list_account.setSizePolicy(sizePolicy)
        self.list_account.setMinimumSize(QtCore.QSize(150,0))
        self.list_account.setObjectName("list_account")
        self.gridlayout2.addWidget(self.list_account,3,1,1,1)

        self.hboxlayout1 = QtGui.QHBoxLayout()
        self.hboxlayout1.setSpacing(3)
        self.hboxlayout1.setObjectName("hboxlayout1")

        spacerItem7 = QtGui.QSpacerItem(71,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout1.addItem(spacerItem7)

        self.change_prompt = QtGui.QPushButton(self.layoutWidget2)
        self.change_prompt.setEnabled(False)
        self.change_prompt.setMinimumSize(QtCore.QSize(0,28))
        self.change_prompt.setMaximumSize(QtCore.QSize(16777215,28))
        self.change_prompt.setIcon(QtGui.QIcon(":/images/prompt.png"))
        self.change_prompt.setObjectName("change_prompt")
        self.hboxlayout1.addWidget(self.change_prompt)

        self.delete_account = QtGui.QPushButton(self.layoutWidget2)
        self.delete_account.setEnabled(False)
        self.delete_account.setMinimumSize(QtCore.QSize(0,28))
        self.delete_account.setMaximumSize(QtCore.QSize(16777215,28))
        self.delete_account.setIcon(QtGui.QIcon(":/images/button-cancel.png"))
        self.delete_account.setObjectName("delete_account")
        self.hboxlayout1.addWidget(self.delete_account)
        self.gridlayout2.addLayout(self.hboxlayout1,4,0,1,2)
        self.page_container.addWidget(self.account_page)

        self.alias_page = QtGui.QWidget()
        self.alias_page.setObjectName("alias_page")

        self.layoutWidget_2 = QtGui.QWidget(self.alias_page)
        self.layoutWidget_2.setGeometry(QtCore.QRect(5,5,311,236))
        self.layoutWidget_2.setObjectName("layoutWidget_2")

        self.gridlayout3 = QtGui.QGridLayout(self.layoutWidget_2)
        self.gridlayout3.setObjectName("gridlayout3")

        self.gridlayout4 = QtGui.QGridLayout()
        self.gridlayout4.setHorizontalSpacing(0)
        self.gridlayout4.setVerticalSpacing(8)
        self.gridlayout4.setObjectName("gridlayout4")

        self.label_conn_alias = QtGui.QLabel(self.layoutWidget_2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_conn_alias.sizePolicy().hasHeightForWidth())
        self.label_conn_alias.setSizePolicy(sizePolicy)
        self.label_conn_alias.setMinimumSize(QtCore.QSize(66,0))
        self.label_conn_alias.setObjectName("label_conn_alias")
        self.gridlayout4.addWidget(self.label_conn_alias,0,0,1,2)

        self.list_conn_alias = QtGui.QComboBox(self.layoutWidget_2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_conn_alias.sizePolicy().hasHeightForWidth())
        self.list_conn_alias.setSizePolicy(sizePolicy)
        self.list_conn_alias.setObjectName("list_conn_alias")
        self.gridlayout4.addWidget(self.list_conn_alias,0,2,1,2)

        self.label_alias_alias = QtGui.QLabel(self.layoutWidget_2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_alias_alias.sizePolicy().hasHeightForWidth())
        self.label_alias_alias.setSizePolicy(sizePolicy)
        self.label_alias_alias.setMinimumSize(QtCore.QSize(66,0))
        self.label_alias_alias.setObjectName("label_alias_alias")
        self.gridlayout4.addWidget(self.label_alias_alias,3,0,1,1)

        spacerItem8 = QtGui.QSpacerItem(80,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout4.addItem(spacerItem8,3,1,1,2)

        self.list_alias = QtGui.QComboBox(self.layoutWidget_2)
        self.list_alias.setEnabled(False)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_alias.sizePolicy().hasHeightForWidth())
        self.list_alias.setSizePolicy(sizePolicy)
        self.list_alias.setObjectName("list_alias")
        self.gridlayout4.addWidget(self.list_alias,3,3,1,1)

        self.label_label_alias = QtGui.QLabel(self.layoutWidget_2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_label_alias.sizePolicy().hasHeightForWidth())
        self.label_label_alias.setSizePolicy(sizePolicy)
        self.label_label_alias.setMinimumSize(QtCore.QSize(66,0))
        self.label_label_alias.setObjectName("label_label_alias")
        self.gridlayout4.addWidget(self.label_label_alias,4,0,1,1)

        spacerItem9 = QtGui.QSpacerItem(80,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout4.addItem(spacerItem9,4,1,1,2)

        self.label_alias = QtGui.QLineEdit(self.layoutWidget_2)
        self.label_alias.setEnabled(False)
        self.label_alias.setObjectName("label_alias")
        self.gridlayout4.addWidget(self.label_alias,4,3,1,1)

        self.label_body_alias = QtGui.QLabel(self.layoutWidget_2)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_body_alias.sizePolicy().hasHeightForWidth())
        self.label_body_alias.setSizePolicy(sizePolicy)
        self.label_body_alias.setMinimumSize(QtCore.QSize(66,0))
        self.label_body_alias.setObjectName("label_body_alias")
        self.gridlayout4.addWidget(self.label_body_alias,5,0,1,1)

        self.body_alias = QtGui.QLineEdit(self.layoutWidget_2)
        self.body_alias.setEnabled(False)
        self.body_alias.setObjectName("body_alias")
        self.gridlayout4.addWidget(self.body_alias,5,1,1,3)

        self.line_2 = QtGui.QFrame(self.layoutWidget_2)
        self.line_2.setFrameShape(QtGui.QFrame.HLine)
        self.line_2.setFrameShadow(QtGui.QFrame.Sunken)
        self.line_2.setObjectName("line_2")
        self.gridlayout4.addWidget(self.line_2,1,0,1,4)

        spacerItem10 = QtGui.QSpacerItem(20,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.gridlayout4.addItem(spacerItem10,2,0,1,4)
        self.gridlayout3.addLayout(self.gridlayout4,0,0,1,2)

        spacerItem11 = QtGui.QSpacerItem(271,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.gridlayout3.addItem(spacerItem11,1,0,1,2)

        spacerItem12 = QtGui.QSpacerItem(91,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout3.addItem(spacerItem12,2,0,1,1)

        self.hboxlayout2 = QtGui.QHBoxLayout()
        self.hboxlayout2.setSpacing(6)
        self.hboxlayout2.setMargin(0)
        self.hboxlayout2.setObjectName("hboxlayout2")

        self.save_alias = QtGui.QPushButton(self.layoutWidget_2)
        self.save_alias.setMinimumSize(QtCore.QSize(0,28))
        self.save_alias.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_alias.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_alias.setObjectName("save_alias")
        self.hboxlayout2.addWidget(self.save_alias)

        self.delete_alias = QtGui.QPushButton(self.layoutWidget_2)
        self.delete_alias.setMinimumSize(QtCore.QSize(0,28))
        self.delete_alias.setMaximumSize(QtCore.QSize(16777215,28))
        self.delete_alias.setIcon(QtGui.QIcon(":/images/button-cancel.png"))
        self.delete_alias.setObjectName("delete_alias")
        self.hboxlayout2.addWidget(self.delete_alias)
        self.gridlayout3.addLayout(self.hboxlayout2,2,1,1,1)
        self.page_container.addWidget(self.alias_page)

        self.macro_page = QtGui.QWidget()
        self.macro_page.setObjectName("macro_page")

        self.layoutWidget_3 = QtGui.QWidget(self.macro_page)
        self.layoutWidget_3.setGeometry(QtCore.QRect(5,5,311,241))
        self.layoutWidget_3.setObjectName("layoutWidget_3")

        self.gridlayout5 = QtGui.QGridLayout(self.layoutWidget_3)
        self.gridlayout5.setObjectName("gridlayout5")

        self.gridlayout6 = QtGui.QGridLayout()
        self.gridlayout6.setHorizontalSpacing(5)
        self.gridlayout6.setVerticalSpacing(8)
        self.gridlayout6.setObjectName("gridlayout6")

        self.label_conn_macro = QtGui.QLabel(self.layoutWidget_3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_conn_macro.sizePolicy().hasHeightForWidth())
        self.label_conn_macro.setSizePolicy(sizePolicy)
        self.label_conn_macro.setMinimumSize(QtCore.QSize(66,0))
        self.label_conn_macro.setObjectName("label_conn_macro")
        self.gridlayout6.addWidget(self.label_conn_macro,0,0,1,2)

        self.list_conn_macro = QtGui.QComboBox(self.layoutWidget_3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_conn_macro.sizePolicy().hasHeightForWidth())
        self.list_conn_macro.setSizePolicy(sizePolicy)
        self.list_conn_macro.setObjectName("list_conn_macro")
        self.gridlayout6.addWidget(self.list_conn_macro,0,2,1,2)

        self.label_macro_macro = QtGui.QLabel(self.layoutWidget_3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_macro_macro.sizePolicy().hasHeightForWidth())
        self.label_macro_macro.setSizePolicy(sizePolicy)
        self.label_macro_macro.setMinimumSize(QtCore.QSize(66,0))
        self.label_macro_macro.setObjectName("label_macro_macro")
        self.gridlayout6.addWidget(self.label_macro_macro,3,0,1,1)

        spacerItem13 = QtGui.QSpacerItem(80,20,QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Minimum)
        self.gridlayout6.addItem(spacerItem13,3,1,1,2)

        self.list_macro = QtGui.QComboBox(self.layoutWidget_3)
        self.list_macro.setEnabled(False)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_macro.sizePolicy().hasHeightForWidth())
        self.list_macro.setSizePolicy(sizePolicy)
        self.list_macro.setObjectName("list_macro")
        self.gridlayout6.addWidget(self.list_macro,3,3,1,1)

        self.label_keys_macro = QtGui.QLabel(self.layoutWidget_3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_keys_macro.sizePolicy().hasHeightForWidth())
        self.label_keys_macro.setSizePolicy(sizePolicy)
        self.label_keys_macro.setMinimumSize(QtCore.QSize(66,0))
        self.label_keys_macro.setObjectName("label_keys_macro")
        self.gridlayout6.addWidget(self.label_keys_macro,4,0,1,1)

        self.keys_macro = QtGui.QLineEdit(self.layoutWidget_3)
        self.keys_macro.setEnabled(False)
        self.keys_macro.setProperty("highlight_color",QtCore.QVariant(QtGui.QApplication.translate("option", "#e0e0e0", None, QtGui.QApplication.UnicodeUTF8)))
        self.keys_macro.setObjectName("keys_macro")
        self.gridlayout6.addWidget(self.keys_macro,4,3,1,1)

        self.label_command_macro = QtGui.QLabel(self.layoutWidget_3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.label_command_macro.sizePolicy().hasHeightForWidth())
        self.label_command_macro.setSizePolicy(sizePolicy)
        self.label_command_macro.setMinimumSize(QtCore.QSize(66,0))
        self.label_command_macro.setObjectName("label_command_macro")
        self.gridlayout6.addWidget(self.label_command_macro,5,0,1,1)

        self.command_macro = QtGui.QLineEdit(self.layoutWidget_3)
        self.command_macro.setEnabled(False)
        self.command_macro.setObjectName("command_macro")
        self.gridlayout6.addWidget(self.command_macro,5,1,1,3)

        self.register_macro = QtGui.QPushButton(self.layoutWidget_3)
        self.register_macro.setEnabled(False)
        self.register_macro.setMinimumSize(QtCore.QSize(0,26))
        self.register_macro.setMaximumSize(QtCore.QSize(16777215,26))
        self.register_macro.setObjectName("register_macro")
        self.gridlayout6.addWidget(self.register_macro,4,1,1,2)

        self.line_3 = QtGui.QFrame(self.layoutWidget_3)
        self.line_3.setFrameShape(QtGui.QFrame.HLine)
        self.line_3.setFrameShadow(QtGui.QFrame.Sunken)
        self.line_3.setObjectName("line_3")
        self.gridlayout6.addWidget(self.line_3,1,0,1,4)

        spacerItem14 = QtGui.QSpacerItem(20,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.gridlayout6.addItem(spacerItem14,2,0,1,4)
        self.gridlayout5.addLayout(self.gridlayout6,0,0,1,2)

        spacerItem15 = QtGui.QSpacerItem(271,20,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Fixed)
        self.gridlayout5.addItem(spacerItem15,1,0,1,2)

        spacerItem16 = QtGui.QSpacerItem(91,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout5.addItem(spacerItem16,2,0,1,1)

        self.hboxlayout3 = QtGui.QHBoxLayout()
        self.hboxlayout3.setSpacing(6)
        self.hboxlayout3.setMargin(0)
        self.hboxlayout3.setObjectName("hboxlayout3")

        self.save_macro = QtGui.QPushButton(self.layoutWidget_3)
        self.save_macro.setMinimumSize(QtCore.QSize(0,28))
        self.save_macro.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_macro.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_macro.setObjectName("save_macro")
        self.hboxlayout3.addWidget(self.save_macro)

        self.delete_macro = QtGui.QPushButton(self.layoutWidget_3)
        self.delete_macro.setMinimumSize(QtCore.QSize(0,28))
        self.delete_macro.setMaximumSize(QtCore.QSize(16777215,28))
        self.delete_macro.setIcon(QtGui.QIcon(":/images/button-cancel.png"))
        self.delete_macro.setObjectName("delete_macro")
        self.hboxlayout3.addWidget(self.delete_macro)
        self.gridlayout5.addLayout(self.hboxlayout3,2,1,1,1)
        self.page_container.addWidget(self.macro_page)

        self.trigger_page = QtGui.QWidget()
        self.trigger_page.setObjectName("trigger_page")

        self.layoutWidget3 = QtGui.QWidget(self.trigger_page)
        self.layoutWidget3.setGeometry(QtCore.QRect(5,9,311,316))
        self.layoutWidget3.setObjectName("layoutWidget3")

        self.vboxlayout1 = QtGui.QVBoxLayout(self.layoutWidget3)
        self.vboxlayout1.setObjectName("vboxlayout1")

        self.gridlayout7 = QtGui.QGridLayout()
        self.gridlayout7.setObjectName("gridlayout7")

        self.label_conn_trigger = QtGui.QLabel(self.layoutWidget3)
        self.label_conn_trigger.setMinimumSize(QtCore.QSize(115,0))
        self.label_conn_trigger.setObjectName("label_conn_trigger")
        self.gridlayout7.addWidget(self.label_conn_trigger,0,0,1,1)

        self.list_conn_trigger = QtGui.QComboBox(self.layoutWidget3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_conn_trigger.sizePolicy().hasHeightForWidth())
        self.list_conn_trigger.setSizePolicy(sizePolicy)
        self.list_conn_trigger.setObjectName("list_conn_trigger")
        self.gridlayout7.addWidget(self.list_conn_trigger,0,1,1,1)

        self.line_4 = QtGui.QFrame(self.layoutWidget3)
        self.line_4.setMinimumSize(QtCore.QSize(115,0))
        self.line_4.setFrameShape(QtGui.QFrame.HLine)
        self.line_4.setFrameShadow(QtGui.QFrame.Sunken)
        self.line_4.setObjectName("line_4")
        self.gridlayout7.addWidget(self.line_4,1,0,1,2)
        self.vboxlayout1.addLayout(self.gridlayout7)

        spacerItem17 = QtGui.QSpacerItem(304,16,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Expanding)
        self.vboxlayout1.addItem(spacerItem17)

        self.gridlayout8 = QtGui.QGridLayout()
        self.gridlayout8.setHorizontalSpacing(0)
        self.gridlayout8.setVerticalSpacing(5)
        self.gridlayout8.setObjectName("gridlayout8")

        self.label_trigger = QtGui.QLabel(self.layoutWidget3)
        self.label_trigger.setMinimumSize(QtCore.QSize(80,0))
        self.label_trigger.setObjectName("label_trigger")
        self.gridlayout8.addWidget(self.label_trigger,0,0,1,1)

        self.list_trigger = QtGui.QComboBox(self.layoutWidget3)

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Preferred,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.list_trigger.sizePolicy().hasHeightForWidth())
        self.list_trigger.setSizePolicy(sizePolicy)
        self.list_trigger.setMinimumSize(QtCore.QSize(115,0))
        self.list_trigger.setObjectName("list_trigger")
        self.gridlayout8.addWidget(self.list_trigger,0,1,1,2)

        self.label_pattern_trigger = QtGui.QLabel(self.layoutWidget3)
        self.label_pattern_trigger.setMinimumSize(QtCore.QSize(80,0))
        self.label_pattern_trigger.setObjectName("label_pattern_trigger")
        self.gridlayout8.addWidget(self.label_pattern_trigger,1,0,1,1)

        self.pattern_trigger = QtGui.QLineEdit(self.layoutWidget3)
        self.pattern_trigger.setMinimumSize(QtCore.QSize(115,0))
        self.pattern_trigger.setObjectName("pattern_trigger")
        self.gridlayout8.addWidget(self.pattern_trigger,1,1,1,2)

        self.case_trigger = QtGui.QCheckBox(self.layoutWidget3)
        self.case_trigger.setMinimumSize(QtCore.QSize(115,0))
        self.case_trigger.setObjectName("case_trigger")
        self.gridlayout8.addWidget(self.case_trigger,2,0,1,2)

        spacerItem18 = QtGui.QSpacerItem(176,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout8.addItem(spacerItem18,2,2,1,1)
        self.vboxlayout1.addLayout(self.gridlayout8)

        spacerItem19 = QtGui.QSpacerItem(20,40,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Expanding)
        self.vboxlayout1.addItem(spacerItem19)

        self.gridlayout9 = QtGui.QGridLayout()
        self.gridlayout9.setObjectName("gridlayout9")

        self.radio_command_trigger = QtGui.QRadioButton(self.layoutWidget3)
        self.radio_command_trigger.setChecked(True)
        self.radio_command_trigger.setObjectName("radio_command_trigger")
        self.gridlayout9.addWidget(self.radio_command_trigger,0,0,1,1)

        self.command_trigger = QtGui.QLineEdit(self.layoutWidget3)
        self.command_trigger.setMinimumSize(QtCore.QSize(180,0))
        self.command_trigger.setObjectName("command_trigger")
        self.gridlayout9.addWidget(self.command_trigger,0,1,1,2)

        self.radio_color_trigger = QtGui.QRadioButton(self.layoutWidget3)
        self.radio_color_trigger.setObjectName("radio_color_trigger")
        self.gridlayout9.addWidget(self.radio_color_trigger,1,0,1,2)

        spacerItem20 = QtGui.QSpacerItem(146,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout9.addItem(spacerItem20,1,2,1,1)

        self.hboxlayout4 = QtGui.QHBoxLayout()
        self.hboxlayout4.setSpacing(2)
        self.hboxlayout4.setObjectName("hboxlayout4")

        self.text_color_trigger_button = QtGui.QPushButton(self.layoutWidget3)
        self.text_color_trigger_button.setEnabled(False)
        self.text_color_trigger_button.setMinimumSize(QtCore.QSize(0,26))
        self.text_color_trigger_button.setIcon(QtGui.QIcon(":/images/button-color.png"))
        self.text_color_trigger_button.setObjectName("text_color_trigger_button")
        self.hboxlayout4.addWidget(self.text_color_trigger_button)

        self.text_color_trigger = QtGui.QLabel(self.layoutWidget3)
        self.text_color_trigger.setEnabled(False)
        self.text_color_trigger.setMinimumSize(QtCore.QSize(24,24))
        self.text_color_trigger.setMaximumSize(QtCore.QSize(24,24))
        self.text_color_trigger.setFrameShape(QtGui.QFrame.StyledPanel)
        self.text_color_trigger.setObjectName("text_color_trigger")
        self.hboxlayout4.addWidget(self.text_color_trigger)

        spacerItem21 = QtGui.QSpacerItem(40,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout4.addItem(spacerItem21)

        self.bg_color_trigger_button = QtGui.QPushButton(self.layoutWidget3)
        self.bg_color_trigger_button.setEnabled(False)
        self.bg_color_trigger_button.setMinimumSize(QtCore.QSize(0,26))
        self.bg_color_trigger_button.setIcon(QtGui.QIcon(":/images/button-color.png"))
        self.bg_color_trigger_button.setObjectName("bg_color_trigger_button")
        self.hboxlayout4.addWidget(self.bg_color_trigger_button)

        self.bg_color_trigger = QtGui.QLabel(self.layoutWidget3)
        self.bg_color_trigger.setEnabled(False)
        self.bg_color_trigger.setMinimumSize(QtCore.QSize(24,24))
        self.bg_color_trigger.setMaximumSize(QtCore.QSize(24,24))
        self.bg_color_trigger.setFrameShape(QtGui.QFrame.StyledPanel)
        self.bg_color_trigger.setObjectName("bg_color_trigger")
        self.hboxlayout4.addWidget(self.bg_color_trigger)
        self.gridlayout9.addLayout(self.hboxlayout4,2,0,1,3)
        self.vboxlayout1.addLayout(self.gridlayout9)

        spacerItem22 = QtGui.QSpacerItem(20,40,QtGui.QSizePolicy.Minimum,QtGui.QSizePolicy.Expanding)
        self.vboxlayout1.addItem(spacerItem22)

        self.hboxlayout5 = QtGui.QHBoxLayout()
        self.hboxlayout5.setSpacing(5)
        self.hboxlayout5.setObjectName("hboxlayout5")

        spacerItem23 = QtGui.QSpacerItem(20,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout5.addItem(spacerItem23)

        self.save_trigger = QtGui.QPushButton(self.layoutWidget3)
        self.save_trigger.setMinimumSize(QtCore.QSize(0,28))
        self.save_trigger.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_trigger.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_trigger.setObjectName("save_trigger")
        self.hboxlayout5.addWidget(self.save_trigger)

        self.delete_trigger = QtGui.QPushButton(self.layoutWidget3)
        self.delete_trigger.setMinimumSize(QtCore.QSize(0,28))
        self.delete_trigger.setMaximumSize(QtCore.QSize(16777215,28))
        self.delete_trigger.setIcon(QtGui.QIcon(":/images/button-cancel.png"))
        self.delete_trigger.setObjectName("delete_trigger")
        self.hboxlayout5.addWidget(self.delete_trigger)
        self.vboxlayout1.addLayout(self.hboxlayout5)
        self.page_container.addWidget(self.trigger_page)

        self.pref_page = QtGui.QWidget()
        self.pref_page.setObjectName("pref_page")

        self.groupBox = QtGui.QGroupBox(self.pref_page)
        self.groupBox.setGeometry(QtCore.QRect(5,10,301,121))
        self.groupBox.setObjectName("groupBox")

        self.widget = QtGui.QWidget(self.groupBox)
        self.widget.setGeometry(QtCore.QRect(8,12,191,106))
        self.widget.setObjectName("widget")

        self.gridlayout10 = QtGui.QGridLayout(self.widget)
        self.gridlayout10.setHorizontalSpacing(3)
        self.gridlayout10.setVerticalSpacing(5)
        self.gridlayout10.setObjectName("gridlayout10")

        self.echo_color_button = QtGui.QPushButton(self.widget)
        self.echo_color_button.setMinimumSize(QtCore.QSize(0,26))
        self.echo_color_button.setIcon(QtGui.QIcon(":/images/button-color.png"))
        self.echo_color_button.setObjectName("echo_color_button")
        self.gridlayout10.addWidget(self.echo_color_button,0,0,1,1)

        spacerItem24 = QtGui.QSpacerItem(40,20,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout10.addItem(spacerItem24,0,1,1,1)

        self.echo_color = QtGui.QLabel(self.widget)
        self.echo_color.setEnabled(False)
        self.echo_color.setMinimumSize(QtCore.QSize(24,24))
        self.echo_color.setMaximumSize(QtCore.QSize(24,24))
        self.echo_color.setFrameShape(QtGui.QFrame.StyledPanel)
        self.echo_color.setObjectName("echo_color")
        self.gridlayout10.addWidget(self.echo_color,0,2,1,1)

        self.label_cmd_separator = QtGui.QLabel(self.widget)
        self.label_cmd_separator.setObjectName("label_cmd_separator")
        self.gridlayout10.addWidget(self.label_cmd_separator,1,0,1,2)

        self.cmd_separator = QtGui.QLineEdit(self.widget)
        self.cmd_separator.setMaximumSize(QtCore.QSize(25,16777215))

        font = QtGui.QFont()
        font.setWeight(75)
        font.setBold(True)
        self.cmd_separator.setFont(font)
        self.cmd_separator.setMaxLength(1)
        self.cmd_separator.setAlignment(QtCore.Qt.AlignCenter)
        self.cmd_separator.setObjectName("cmd_separator")
        self.gridlayout10.addWidget(self.cmd_separator,1,2,1,1)

        self.keep_text = QtGui.QCheckBox(self.widget)
        self.keep_text.setObjectName("keep_text")
        self.gridlayout10.addWidget(self.keep_text,2,0,1,2)

        self.groupBox_2 = QtGui.QGroupBox(self.pref_page)
        self.groupBox_2.setGeometry(QtCore.QRect(5,135,301,86))
        self.groupBox_2.setObjectName("groupBox_2")

        self.layoutWidget4 = QtGui.QWidget(self.groupBox_2)
        self.layoutWidget4.setGeometry(QtCore.QRect(5,20,291,36))
        self.layoutWidget4.setObjectName("layoutWidget4")

        self.gridlayout11 = QtGui.QGridLayout(self.layoutWidget4)
        self.gridlayout11.setObjectName("gridlayout11")

        self.save_log = QtGui.QCheckBox(self.layoutWidget4)
        self.save_log.setMinimumSize(QtCore.QSize(100,0))
        self.save_log.setObjectName("save_log")
        self.gridlayout11.addWidget(self.save_log,0,0,1,1)

        spacerItem25 = QtGui.QSpacerItem(156,22,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.gridlayout11.addItem(spacerItem25,0,1,1,1)

        self.layoutWidget_4 = QtGui.QWidget(self.pref_page)
        self.layoutWidget_4.setGeometry(QtCore.QRect(5,230,311,46))
        self.layoutWidget_4.setObjectName("layoutWidget_4")

        self.hboxlayout6 = QtGui.QHBoxLayout(self.layoutWidget_4)
        self.hboxlayout6.setObjectName("hboxlayout6")

        spacerItem26 = QtGui.QSpacerItem(209,26,QtGui.QSizePolicy.Expanding,QtGui.QSizePolicy.Minimum)
        self.hboxlayout6.addItem(spacerItem26)

        self.save_preferences = QtGui.QPushButton(self.layoutWidget_4)
        self.save_preferences.setMinimumSize(QtCore.QSize(0,28))
        self.save_preferences.setMaximumSize(QtCore.QSize(16777215,28))
        self.save_preferences.setIcon(QtGui.QIcon(":/images/button-save.png"))
        self.save_preferences.setObjectName("save_preferences")
        self.hboxlayout6.addWidget(self.save_preferences)
        self.page_container.addWidget(self.pref_page)

        self.line_option = QtGui.QFrame(option)
        self.line_option.setGeometry(QtCore.QRect(5,330,400,10))
        self.line_option.setFrameShape(QtGui.QFrame.HLine)
        self.line_option.setFrameShadow(QtGui.QFrame.Sunken)
        self.line_option.setObjectName("line_option")

        self.close_option = QtGui.QPushButton(option)
        self.close_option.setGeometry(QtCore.QRect(323,340,80,27))
        self.close_option.setMinimumSize(QtCore.QSize(80,27))
        self.close_option.setObjectName("close_option")

        self.retranslateUi(option)
        self.list_option.setCurrentRow(-1)
        self.page_container.setCurrentIndex(0)
        QtCore.QObject.connect(self.close_option,QtCore.SIGNAL("clicked()"),option.accept)
        QtCore.QMetaObject.connectSlotsByName(option)
        option.setTabOrder(self.list_option,self.list_conn)
        option.setTabOrder(self.list_conn,self.name_conn)
        option.setTabOrder(self.name_conn,self.host_conn)
        option.setTabOrder(self.host_conn,self.port_conn)
        option.setTabOrder(self.port_conn,self.save_conn)
        option.setTabOrder(self.save_conn,self.delete_conn)
        option.setTabOrder(self.delete_conn,self.list_conn_account)
        option.setTabOrder(self.list_conn_account,self.list_account)
        option.setTabOrder(self.list_account,self.delete_account)
        option.setTabOrder(self.delete_account,self.list_conn_alias)
        option.setTabOrder(self.list_conn_alias,self.list_alias)
        option.setTabOrder(self.list_alias,self.label_alias)
        option.setTabOrder(self.label_alias,self.body_alias)
        option.setTabOrder(self.body_alias,self.save_alias)
        option.setTabOrder(self.save_alias,self.delete_alias)
        option.setTabOrder(self.delete_alias,self.list_conn_macro)
        option.setTabOrder(self.list_conn_macro,self.list_macro)
        option.setTabOrder(self.list_macro,self.register_macro)
        option.setTabOrder(self.register_macro,self.keys_macro)
        option.setTabOrder(self.keys_macro,self.command_macro)
        option.setTabOrder(self.command_macro,self.save_macro)
        option.setTabOrder(self.save_macro,self.delete_macro)
        option.setTabOrder(self.delete_macro,self.echo_color_button)
        option.setTabOrder(self.echo_color_button,self.save_log)

    def retranslateUi(self, option):
        option.setWindowTitle(QtGui.QApplication.translate("option", "Option", None, QtGui.QApplication.UnicodeUTF8))
        self.list_option.setStyleSheet(QtGui.QApplication.translate("option", "QListWidget { background-color: qlineargradient(x1: 0, y1: 0, x2: 0, y2: 1, stop: 0 #E0E0E0, stop: 1 #FFFFFF); color: #00AAFF;selection-background-color: #C8C8C8;selection-color:#000000;font: bold 10px \"Verdana\";  }", None, QtGui.QApplication.UnicodeUTF8))
        self.list_option.clear()

        item = QtGui.QListWidgetItem(self.list_option)
        item.setText(QtGui.QApplication.translate("option", "Connections", None, QtGui.QApplication.UnicodeUTF8))
        item.setIcon(QtGui.QIcon(":/images/connections.png"))

        item1 = QtGui.QListWidgetItem(self.list_option)
        item1.setText(QtGui.QApplication.translate("option", "Accounts", None, QtGui.QApplication.UnicodeUTF8))
        item1.setIcon(QtGui.QIcon(":/images/accounts.png"))

        item2 = QtGui.QListWidgetItem(self.list_option)
        item2.setText(QtGui.QApplication.translate("option", "Aliases", None, QtGui.QApplication.UnicodeUTF8))
        item2.setIcon(QtGui.QIcon(":/images/aliases.png"))

        item3 = QtGui.QListWidgetItem(self.list_option)
        item3.setText(QtGui.QApplication.translate("option", "Macros", None, QtGui.QApplication.UnicodeUTF8))
        item3.setIcon(QtGui.QIcon(":/images/macros.png"))

        item4 = QtGui.QListWidgetItem(self.list_option)
        item4.setText(QtGui.QApplication.translate("option", "Triggers", None, QtGui.QApplication.UnicodeUTF8))
        item4.setIcon(QtGui.QIcon(":/images/triggers.png"))

        item5 = QtGui.QListWidgetItem(self.list_option)
        item5.setText(QtGui.QApplication.translate("option", "Preferences", None, QtGui.QApplication.UnicodeUTF8))
        item5.setIcon(QtGui.QIcon(":/images/preferences.png"))
        self.label_conn.setText(QtGui.QApplication.translate("option", "Connection", None, QtGui.QApplication.UnicodeUTF8))
        self.list_conn.addItem(QtGui.QApplication.translate("option", "Create New", None, QtGui.QApplication.UnicodeUTF8))
        self.label_name_conn.setText(QtGui.QApplication.translate("option", "Name", None, QtGui.QApplication.UnicodeUTF8))
        self.label_host_conn.setText(QtGui.QApplication.translate("option", "Host", None, QtGui.QApplication.UnicodeUTF8))
        self.label_port_conn.setText(QtGui.QApplication.translate("option", "Port", None, QtGui.QApplication.UnicodeUTF8))
        self.save_conn.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.delete_conn.setText(QtGui.QApplication.translate("option", "Delete", None, QtGui.QApplication.UnicodeUTF8))
        self.save_account.setText(QtGui.QApplication.translate("option", "Save accounts", None, QtGui.QApplication.UnicodeUTF8))
        self.box_prompt.setToolTip(QtGui.QApplication.translate("option", "<table>\n"
        "<tr><td colspan=3><b>Prompt format:</b></td></tr>\n"
        "<tr><td>%h</td><td>-></td><td>Current hit points</td></tr>\n"
        "<tr><td>%H</td><td>-></td><td>Maximum hit points</td></tr>\n"
        "<tr><td>%m</td><td>-></td><td>Current mana</td></tr>\n"
        "<tr><td>%M</td><td>-></td><td>Maximum mana</td></tr>\n"
        "<tr><td>%v</td><td>-></td><td>Current moves</td></tr>\n"
        "<tr><td>%V</td><td>-></td><td>Maximum moves</td></tr>\n"
        "<tr><td>*<td>-></td><td>Represent any char, repeated</td></tr>\n"
        "<tr><td colspan=2>&nbsp;</td><td>zero or more times</td></tr>\n"
        "<tr><td colspan=3>&nbsp;</td></tr>\n"
        "</table>\n"
        "<table>\n"
        "<tr><td colspan=3><b>Example:</b></td></tr>\n"
        "<tr><td colspan=3>[  %h/%Hhp %m/%Mmn %v/%Vmv *] ></td></tr>\n"
        "<tr><td colspan=3>is a valid rapresentation for:</td></tr>\n"
        "<tr><td colspan=3>[  111/111hp 100/100mn 500/500mv 1000tnl] ></td></tr>\n"
        "</table>", None, QtGui.QApplication.UnicodeUTF8))
        self.box_prompt.setTitle(QtGui.QApplication.translate("option", "Prompt", None, QtGui.QApplication.UnicodeUTF8))
        self.label.setText(QtGui.QApplication.translate("option", "Normal", None, QtGui.QApplication.UnicodeUTF8))
        self.label_2.setText(QtGui.QApplication.translate("option", "Fight", None, QtGui.QApplication.UnicodeUTF8))
        self.save_prompt.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.label_conn_account.setText(QtGui.QApplication.translate("option", "Connection", None, QtGui.QApplication.UnicodeUTF8))
        self.label_account_account.setText(QtGui.QApplication.translate("option", "Account", None, QtGui.QApplication.UnicodeUTF8))
        self.change_prompt.setText(QtGui.QApplication.translate("option", "Change Prompt", None, QtGui.QApplication.UnicodeUTF8))
        self.delete_account.setText(QtGui.QApplication.translate("option", "Delete", None, QtGui.QApplication.UnicodeUTF8))
        self.label_conn_alias.setText(QtGui.QApplication.translate("option", "Connection", None, QtGui.QApplication.UnicodeUTF8))
        self.label_alias_alias.setText(QtGui.QApplication.translate("option", "Alias", None, QtGui.QApplication.UnicodeUTF8))
        self.label_label_alias.setText(QtGui.QApplication.translate("option", "Label", None, QtGui.QApplication.UnicodeUTF8))
        self.label_body_alias.setText(QtGui.QApplication.translate("option", "Body", None, QtGui.QApplication.UnicodeUTF8))
        self.save_alias.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.delete_alias.setText(QtGui.QApplication.translate("option", "Delete", None, QtGui.QApplication.UnicodeUTF8))
        self.label_conn_macro.setText(QtGui.QApplication.translate("option", "Connection", None, QtGui.QApplication.UnicodeUTF8))
        self.label_macro_macro.setText(QtGui.QApplication.translate("option", "Macro", None, QtGui.QApplication.UnicodeUTF8))
        self.label_keys_macro.setText(QtGui.QApplication.translate("option", "Keys", None, QtGui.QApplication.UnicodeUTF8))
        self.label_command_macro.setText(QtGui.QApplication.translate("option", "Command", None, QtGui.QApplication.UnicodeUTF8))
        self.register_macro.setText(QtGui.QApplication.translate("option", "Register", None, QtGui.QApplication.UnicodeUTF8))
        self.save_macro.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.delete_macro.setText(QtGui.QApplication.translate("option", "Delete", None, QtGui.QApplication.UnicodeUTF8))
        self.label_conn_trigger.setText(QtGui.QApplication.translate("option", "Connection", None, QtGui.QApplication.UnicodeUTF8))
        self.label_trigger.setText(QtGui.QApplication.translate("option", "Trigger", None, QtGui.QApplication.UnicodeUTF8))
        self.label_pattern_trigger.setText(QtGui.QApplication.translate("option", "Pattern", None, QtGui.QApplication.UnicodeUTF8))
        self.case_trigger.setText(QtGui.QApplication.translate("option", "Ignore case", None, QtGui.QApplication.UnicodeUTF8))
        self.radio_command_trigger.setText(QtGui.QApplication.translate("option", "Command", None, QtGui.QApplication.UnicodeUTF8))
        self.radio_color_trigger.setText(QtGui.QApplication.translate("option", "Change color to", None, QtGui.QApplication.UnicodeUTF8))
        self.text_color_trigger_button.setText(QtGui.QApplication.translate("option", "Text", None, QtGui.QApplication.UnicodeUTF8))
        self.text_color_trigger.setStyleSheet(QtGui.QApplication.translate("option", "QLabel { border: 1px solid gray; border-radius: 3px; }", None, QtGui.QApplication.UnicodeUTF8))
        self.bg_color_trigger_button.setText(QtGui.QApplication.translate("option", "Background", None, QtGui.QApplication.UnicodeUTF8))
        self.bg_color_trigger.setStyleSheet(QtGui.QApplication.translate("option", "QLabel { border: 1px solid gray; border-radius: 3px; }", None, QtGui.QApplication.UnicodeUTF8))
        self.save_trigger.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.delete_trigger.setText(QtGui.QApplication.translate("option", "Delete", None, QtGui.QApplication.UnicodeUTF8))
        self.groupBox.setTitle(QtGui.QApplication.translate("option", "Text inserted", None, QtGui.QApplication.UnicodeUTF8))
        self.echo_color_button.setText(QtGui.QApplication.translate("option", "Echo Color", None, QtGui.QApplication.UnicodeUTF8))
        self.echo_color.setStyleSheet(QtGui.QApplication.translate("option", "QLabel { border: 1px solid gray; border-radius: 3px; }", None, QtGui.QApplication.UnicodeUTF8))
        self.label_cmd_separator.setText(QtGui.QApplication.translate("option", "Command separator", None, QtGui.QApplication.UnicodeUTF8))
        self.keep_text.setText(QtGui.QApplication.translate("option", "Keep text entered", None, QtGui.QApplication.UnicodeUTF8))
        self.groupBox_2.setTitle(QtGui.QApplication.translate("option", "General", None, QtGui.QApplication.UnicodeUTF8))
        self.save_log.setText(QtGui.QApplication.translate("option", "Save log", None, QtGui.QApplication.UnicodeUTF8))
        self.save_preferences.setText(QtGui.QApplication.translate("option", "Save", None, QtGui.QApplication.UnicodeUTF8))
        self.close_option.setText(QtGui.QApplication.translate("option", "Close", None, QtGui.QApplication.UnicodeUTF8))

import gui_rc
import gui_option_rc
