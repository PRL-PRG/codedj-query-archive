#!/usr/bin/python
#-*- coding: utf-8 -*-

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import SIGNAL
from PyQt4.QtGui import QApplication

from gui_option_ui import Ui_option


class GuiOption(QtGui.QDialog, Ui_option):
    """
    The Gui dialog for setup option.
    """

    def __init__(self, parent, config):
        self.config = config
        QtGui.QDialog.__init__(self, parent)
        self.setupUi(self)

        self.port_conn.setValidator(QtGui.QIntValidator(self.port_conn))

        name = QApplication.translate("option", "Name", None,
                                      QApplication.UnicodeUTF8)
        host = QApplication.translate("option", "Host", None,
                                      QApplication.UnicodeUTF8)
        port = QApplication.translate("option", "Port", None,
                                      QApplication.UnicodeUTF8)
        
        self.conn_fields = {name: self.name_conn,
                            host: self.host_conn,
                            port: self.port_conn}
        self._setupSignal()

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.connect(self.save_conn, clicked, self._saveConnection)
        self.connect(self.delete_conn, clicked, self._deleteConnection)
        self.connect(self.bg_button_style, clicked, self._chooseBgColor)
        self.connect(self.fg_button_style, clicked, self._chooseFgColor)

        self.connect(self.list_conn, SIGNAL("currentIndexChanged(QString)"),
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
                    msg.append(str(text))

        if msg:
            window = QApplication.translate("option", "Connection", None,
                                            QApplication.UnicodeUTF8)

            title = QApplication.translate("option",
                                           "The following fields are required",
                                           None, QApplication.UnicodeUTF8)

            QtGui.QMessageBox.warning(self, window, "%s:\n%s" %
                                                    (title, '\n'.join(msg)))
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
