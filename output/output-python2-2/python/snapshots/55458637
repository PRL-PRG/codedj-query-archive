# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'listempresasbase.ui'
#
# Created: Sat Mar  8 19:32:24 2008
#      by: PyQt4 UI code generator 4.3
#
# WARNING! All changes made in this file will be lost!

from PyQt4 import QtCore, QtGui

class Ui_ListEmpresasBase(object):
    def setupUi(self, ListEmpresasBase):
        ListEmpresasBase.setObjectName("ListEmpresasBase")
        ListEmpresasBase.resize(QtCore.QSize(QtCore.QRect(0,0,800,608).size()).expandedTo(ListEmpresasBase.minimumSizeHint()))

        sizePolicy = QtGui.QSizePolicy(QtGui.QSizePolicy.Fixed,QtGui.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(ListEmpresasBase.sizePolicy().hasHeightForWidth())
        ListEmpresasBase.setSizePolicy(sizePolicy)
        ListEmpresasBase.setMaximumSize(QtCore.QSize(800,608))
        ListEmpresasBase.setWindowIcon(QtGui.QIcon("../images/png/i_empresa.xpm"))

        self.gridlayout = QtGui.QGridLayout(ListEmpresasBase)
        self.gridlayout.setMargin(9)
        self.gridlayout.setSpacing(6)
        self.gridlayout.setObjectName("gridlayout")

        self.mui_listado = QtGui.QTableWidget(ListEmpresasBase)
        self.mui_listado.setFocusPolicy(QtCore.Qt.ClickFocus)
        self.mui_listado.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAsNeeded)
        self.mui_listado.setAutoScroll(True)
        self.mui_listado.setEditTriggers(QtGui.QAbstractItemView.NoEditTriggers)
        self.mui_listado.setTabKeyNavigation(False)
        self.mui_listado.setProperty("showDropIndicator",QtCore.QVariant(False))
        self.mui_listado.setDragDropOverwriteMode(False)
        self.mui_listado.setAlternatingRowColors(True)
        self.mui_listado.setSelectionMode(QtGui.QAbstractItemView.SingleSelection)
        self.mui_listado.setSelectionBehavior(QtGui.QAbstractItemView.SelectRows)
        self.mui_listado.setTextElideMode(QtCore.Qt.ElideLeft)
        self.mui_listado.setVerticalScrollMode(QtGui.QAbstractItemView.ScrollPerItem)
        self.mui_listado.setShowGrid(True)
        self.mui_listado.setGridStyle(QtCore.Qt.DotLine)
        self.mui_listado.setWordWrap(True)
        self.mui_listado.setCornerButtonEnabled(True)
        self.mui_listado.setObjectName("mui_listado")
        self.gridlayout.addWidget(self.mui_listado,0,0,1,1)

        self.mui_textBrowser = QtGui.QTextBrowser(ListEmpresasBase)
        self.mui_textBrowser.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        self.mui_textBrowser.setObjectName("mui_textBrowser")
        self.gridlayout.addWidget(self.mui_textBrowser,1,0,1,1)

        self.mui_cancelar = QtGui.QPushButton(ListEmpresasBase)
        self.mui_cancelar.setObjectName("mui_cancelar")
        self.gridlayout.addWidget(self.mui_cancelar,2,0,1,1)

        self.retranslateUi(ListEmpresasBase)
        QtCore.QObject.connect(self.mui_cancelar,QtCore.SIGNAL("released()"),ListEmpresasBase.reject)
        QtCore.QMetaObject.connectSlotsByName(ListEmpresasBase)

    def retranslateUi(self, ListEmpresasBase):
        ListEmpresasBase.setWindowTitle(QtGui.QApplication.translate("ListEmpresasBase", "Listado de Empresas", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_listado.clear()
        self.mui_listado.setColumnCount(4)
        self.mui_listado.setRowCount(0)

        headerItem = QtGui.QTableWidgetItem()
        headerItem.setText(QtGui.QApplication.translate("ListEmpresasBase", "Nombre", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_listado.setHorizontalHeaderItem(0,headerItem)

        headerItem1 = QtGui.QTableWidgetItem()
        headerItem1.setText(QtGui.QApplication.translate("ListEmpresasBase", "Base de Datos", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_listado.setHorizontalHeaderItem(1,headerItem1)

        headerItem2 = QtGui.QTableWidgetItem()
        headerItem2.setText(QtGui.QApplication.translate("ListEmpresasBase", "Tipo", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_listado.setHorizontalHeaderItem(2,headerItem2)

        headerItem3 = QtGui.QTableWidgetItem()
        headerItem3.setText(QtGui.QApplication.translate("ListEmpresasBase", "Version", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_listado.setHorizontalHeaderItem(3,headerItem3)
        self.mui_cancelar.setText(QtGui.QApplication.translate("ListEmpresasBase", "Cerrar", None, QtGui.QApplication.UnicodeUTF8))

