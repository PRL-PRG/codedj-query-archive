#!/usr/bin/python
#-*- coding: utf-8 -*-

from PyQt4 import QtCore, QtGui

from gui_option_ui import Ui_gui_option

class GuiOption(QtGui.QDialog, Ui_gui_option):
    """
    The Gui dialog for setup option.
    """

    def __init__(self, parent=None):
        QtGui.QDialog.__init__(self, parent)
        self.setupUi(self)

        self.port_conn.setValidator(QtGui.QIntValidator(self.port_conn))

        self.conn_fields = {'Nome': self.name_conn,
                            'Host': self.host_conn,
                            'Porta': self.port_conn}
        self._setupSignal()

    def _setupSignal(self):
        clicked = QtCore.SIGNAL("clicked()")
        self.connect(self.save_conn, clicked, self._saveConnection)
        self.connect(self.delete_conn, clicked, self._deleteConnection)
        self.connect(self.bg_button_style, clicked, self._chooseBgColor)
        self.connect(self.fg_button_style, clicked, self._chooseFgColor)

        self.connect(self.list_conn,
                     QtCore.SIGNAL("currentIndexChanged(QString)"),
                     self._loadConnection)

    def _loadConnection(self, conn_name):
        print 'load connection:', conn_name

    def _chooseBgColor(self):
        color = QtGui.QColorDialog.getColor()
        self.bg_style.setText(color.name())

    def _chooseFgColor(self):
        color = QtGui.QColorDialog.getColor()
        self.fg_style.setText(color.name())

    def _checkConnectionFields(self):
        msg = []
        for text, field in self.conn_fields.iteritems():
                if not field.text():
                    msg.append(text)

        if msg:
            QtGui.QMessageBox.warning(self, "Connessione",
                                      "E' necessario specificare i campi:\n%s"
                                      % '\n'.join(msg))
            return False
        return True

    def _saveConnection(self):
        if not self._checkConnectionFields():
            return

        if not self.list_conn.currentIndex():
            self.list_conn.addItem(self.name_conn.text())
            for field in self.conn_fields.itervalues():
                field.setText("")

            print 'new connection'
        else:
            print 'save connection'

    def _deleteConnection(self):
        if not self.list_conn.currentIndex():
            return

        self.list_conn.removeItem(self.list_conn.currentIndex())
        print 'delete connection'
