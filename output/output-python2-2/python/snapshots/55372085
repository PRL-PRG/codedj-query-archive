# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'principal.ui'
#
# Created: Wed Jan 10 21:30:13 2007
#      by: PyQt4 UI code generator 4.0.1
#
# WARNING! All changes made in this file will be lost!

import sys
from PyQt4 import QtCore, QtGui

class Ui_Principal(object):
    def setupUi(self, Principal):
        Principal.setObjectName("Principal")
        Principal.resize(QtCore.QSize(QtCore.QRect(0,0,807,535).size()).expandedTo(Principal.minimumSizeHint()))

        font = QtGui.QFont(Principal.font())
        font.setPointSize(12)
        Principal.setFont(font)

        self.centralwidget = QtGui.QWidget(Principal)
        self.centralwidget.setObjectName("centralwidget")

        self.hboxlayout = QtGui.QHBoxLayout(self.centralwidget)
        self.hboxlayout.setMargin(9)
        self.hboxlayout.setSpacing(6)
        self.hboxlayout.setObjectName("hboxlayout")

        self.dockWidget = QtGui.QDockWidget(self.centralwidget)
        self.dockWidget.setObjectName("dockWidget")

        self.dockWidgetContents = QtGui.QWidget(self.dockWidget)
        self.dockWidgetContents.setObjectName("dockWidgetContents")

        self.vboxlayout = QtGui.QVBoxLayout(self.dockWidgetContents)
        self.vboxlayout.setMargin(9)
        self.vboxlayout.setSpacing(6)
        self.vboxlayout.setObjectName("vboxlayout")

        self.tree = QtGui.QTreeWidget(self.dockWidgetContents)

        font = QtGui.QFont(self.tree.font())
        font.setPointSize(14)
        self.tree.setFont(font)
        self.tree.setAlternatingRowColors(True)
        self.tree.setSortingEnabled(True)
        self.tree.setColumnCount(3)
        self.tree.setObjectName("tree")
        self.vboxlayout.addWidget(self.tree)
        self.dockWidget.setWidget(self.dockWidgetContents)
        self.hboxlayout.addWidget(self.dockWidget)

        self.table = QtGui.QTableWidget(self.centralwidget)
        self.table.setAlternatingRowColors(True)
        self.table.setSelectionMode(QtGui.QAbstractItemView.MultiSelection)
        self.table.setObjectName("table")
        self.hboxlayout.addWidget(self.table)
        Principal.setCentralWidget(self.centralwidget)

        self.menubar = QtGui.QMenuBar(Principal)
        self.menubar.setGeometry(QtCore.QRect(0,0,807,27))
        self.menubar.setObjectName("menubar")

        self.menuAbout = QtGui.QMenu(self.menubar)
        self.menuAbout.setObjectName("menuAbout")

        self.menuPrincipal = QtGui.QMenu(self.menubar)
        self.menuPrincipal.setObjectName("menuPrincipal")
        Principal.setMenuBar(self.menubar)

        self.statusbar = QtGui.QStatusBar(Principal)
        self.statusbar.setObjectName("statusbar")
        Principal.setStatusBar(self.statusbar)

        self.toolBar = QtGui.QToolBar(Principal)
        self.toolBar.setOrientation(QtCore.Qt.Horizontal)
        self.toolBar.setObjectName("toolBar")
        Principal.addToolBar(self.toolBar)

        self.mui_about = QtGui.QAction(Principal)
        self.mui_about.setObjectName("mui_about")

        self.mui_clear = QtGui.QAction(Principal)
        self.mui_clear.setObjectName("mui_clear")

        self.mui_conectar = QtGui.QAction(Principal)
        self.mui_conectar.setObjectName("mui_conectar")

        self.mui_tableclear = QtGui.QAction(Principal)
        self.mui_tableclear.setObjectName("mui_tableclear")

        self.actionLimpiar_Arbol = QtGui.QAction(Principal)
        self.actionLimpiar_Arbol.setObjectName("actionLimpiar_Arbol")

        self.mui_expandtree = QtGui.QAction(Principal)
        self.mui_expandtree.setObjectName("mui_expandtree")

        self.mui_collapseTree = QtGui.QAction(Principal)
        self.mui_collapseTree.setObjectName("mui_collapseTree")
        self.menuAbout.addAction(self.mui_about)
        self.menuPrincipal.addAction(self.mui_conectar)
        self.menuPrincipal.addSeparator()
        self.menuPrincipal.addAction(self.mui_tableclear)
        self.menuPrincipal.addAction(self.actionLimpiar_Arbol)
        self.menubar.addAction(self.menuPrincipal.menuAction())
        self.menubar.addAction(self.menuAbout.menuAction())
        self.toolBar.addAction(self.mui_clear)
        self.toolBar.addAction(self.mui_conectar)
        self.toolBar.addAction(self.mui_expandtree)
        self.toolBar.addAction(self.mui_collapseTree)

        self.retranslateUi(Principal)
        QtCore.QObject.connect(self.actionLimpiar_Arbol,QtCore.SIGNAL("triggered()"),self.tree.clear)
        QtCore.QMetaObject.connectSlotsByName(Principal)

    def retranslateUi(self, Principal):
        Principal.setWindowTitle(QtGui.QApplication.translate("Principal", "MainWindow", None, QtGui.QApplication.UnicodeUTF8))
        self.tree.headerItem().setText(0,QtGui.QApplication.translate("Principal", "1", None, QtGui.QApplication.UnicodeUTF8))
        self.tree.headerItem().setText(1,QtGui.QApplication.translate("Principal", "1", None, QtGui.QApplication.UnicodeUTF8))
        self.tree.headerItem().setText(2,QtGui.QApplication.translate("Principal", "2", None, QtGui.QApplication.UnicodeUTF8))
        self.table.setColumnCount(3)
        self.table.clear()
        self.table.setColumnCount(3)
        self.table.setRowCount(0)
        self.menuAbout.setTitle(QtGui.QApplication.translate("Principal", "About", None, QtGui.QApplication.UnicodeUTF8))
        self.menuPrincipal.setTitle(QtGui.QApplication.translate("Principal", "Principal", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_about.setText(QtGui.QApplication.translate("Principal", "About", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_clear.setText(QtGui.QApplication.translate("Principal", "Limpiar", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_conectar.setText(QtGui.QApplication.translate("Principal", "Conectar", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_tableclear.setText(QtGui.QApplication.translate("Principal", "Limpiar Table", None, QtGui.QApplication.UnicodeUTF8))
        self.actionLimpiar_Arbol.setText(QtGui.QApplication.translate("Principal", "Limpiar Arbol", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_expandtree.setText(QtGui.QApplication.translate("Principal", "Expandir Arbol", None, QtGui.QApplication.UnicodeUTF8))
        self.mui_collapseTree.setText(QtGui.QApplication.translate("Principal", "contraerArbol", None, QtGui.QApplication.UnicodeUTF8))


if __name__ == "__main__":
    app = QtGui.QApplication(sys.argv)
    Principal = QtGui.QMainWindow()
    ui = Ui_Principal()
    ui.setupUi(Principal)
    Principal.show()
    sys.exit(app.exec_())
