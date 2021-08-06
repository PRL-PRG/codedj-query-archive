#!/usr/bin/python
#-*- coding: utf-8 -*-

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import SIGNAL
from PyQt4.QtGui import QApplication

from storage import Storage
from gui_option_ui import Ui_option

class GuiOption(QtGui.QDialog, Ui_option):
    """
    The Gui dialog for setup option.
    """

    def __init__(self, parent):
        QtGui.QDialog.__init__(self, parent)
        self.setupUi(self)

        self.port_conn.setValidator(QtGui.QIntValidator(self.port_conn))

        self._setupSignal()
        self.storage = Storage()
        self._loadConnections()
        self._translateText()

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.connect(self.save_conn, clicked, self._saveConnection)
        self.connect(self.delete_conn, clicked, self._deleteConnection)
        self.connect(self.bg_button_style, clicked, self._chooseBgColor)
        self.connect(self.fg_button_style, clicked, self._chooseFgColor)

        self.connect(self.list_conn, SIGNAL("currentIndexChanged(QString)"),
                     self._loadConnection)

        self.connect(self.connect_conn, clicked, self._connectReq)

    def _translateText(self):
        self._text = {}
        self._text['name'] = QApplication.translate("option", "Name", None,
                                                    QApplication.UnicodeUTF8)
        self._text['host'] = QApplication.translate("option", "Host", None,
                                                    QApplication.UnicodeUTF8)
        self._text['port'] = QApplication.translate("option", "Port", None,
                                                    QApplication.UnicodeUTF8)

        self._text['connection'] = QApplication.translate("option",
            "Connection", None, QApplication.UnicodeUTF8)
        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required", None, QApplication.UnicodeUTF8)
        self._text['unique_name'] = QApplication.translate("option",
            "Connection name must be unique", None, QApplication.UnicodeUTF8)

    def _chooseBgColor(self):
        color = QtGui.QColorDialog.getColor()
        self.bg_style.setText(color.name())

    def _chooseFgColor(self):
        color = QtGui.QColorDialog.getColor()
        self.fg_style.setText(color.name())

    def _connectReq(self):
        self.emit(SIGNAL('connectReq(const QString &, int)'),
                  self.host_conn.text(),
                  int(self.port_conn.text()))
        self.close()

    def _loadConnections(self):
        """
        Load all connections.
        """

        self.connections = self.storage.connections()
        self.list_conn.addItems([el[0] for el in self.connections])

    def _loadConnection(self, name):
        """
        Load data of one connection.

        :Parameters:
          name : str
            the name of connection to load
        """

        conn = [el for el in self.connections if el[0] == name]

        if conn:
            n, h, p, d = conn[0]
            connect = True
        else:
            n, h, p, d = ('', '', '', QtCore.Qt.Unchecked)
            connect = False

        self.name_conn.setText(n)
        self.host_conn.setText(h)
        self.port_conn.setText(unicode(p))
        self.connect_conn.setEnabled(connect)

        if d:
            self.default_conn.setCheckState(QtCore.Qt.Checked)
        else:
            self.default_conn.setCheckState(QtCore.Qt.Unchecked)

    def _checkConnectionFields(self):
        """
        Check validity of connection fields.
        """

        msg = []

        conn_fields = {self._text['name']: self.name_conn,
                       self._text['host']: self.host_conn,
                       self._text['port']: self.port_conn}

        for text, field in conn_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            QtGui.QMessageBox.warning(self, self._text['connection'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False
        return True

    def _saveConnection(self):
        """
        Save a connection after check the fields validity.
        """

        if not self._checkConnectionFields():
            return

        make_default = self.default_conn.checkState() == QtCore.Qt.Checked

        if make_default:
            self.connections = [(n, h, p, 0) for n, h, p, d in
                                self.connections]

        conn = (unicode(self.name_conn.text()),
                unicode(self.host_conn.text()),
                int(self.port_conn.text()),
                int(make_default))

        if not self.list_conn.currentIndex():
            if [el[0] for el in self.connections if
                el[0] == self.name_conn.text()]:
                QtGui.QMessageBox.warning(self, self._text['connection'],
                                          self._text['unique_name'])
            else:
                self.list_conn.addItem(self.name_conn.text())
                self.connections.append(conn)
        else:
            self.connections[self.list_conn.currentIndex() - 1] = conn
            self.list_conn.setItemText(self.list_conn.currentIndex(), conn[0])

        self.storage.saveConnections(self.connections)
        self.list_conn.setCurrentIndex(0)
        self._loadConnection('')

    def _deleteConnection(self):
        """
        Erase a connection.
        """

        if not self.list_conn.currentIndex():
            return

        del self.connections[self.list_conn.currentIndex() - 1]
        self.storage.saveConnections(self.connections)
        self.list_conn.removeItem(self.list_conn.currentIndex())

