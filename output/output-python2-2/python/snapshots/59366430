#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

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
        self.connect(self.tab_widget, SIGNAL("currentChanged(int)"),
                     self._syncTabs)

        self._signalConnAlias(True)
        self.connect(self.save_alias, clicked, self._saveAlias)
        self.connect(self.delete_alias, clicked, self._deleteAlias)

    def _signalConnAlias(self, conn):
        f = self.connect
        if not conn:
            f = self.disconnect

        f(self.list_conn_alias, SIGNAL("currentIndexChanged(QString)"),
          self._loadAliases)

        f(self.list_alias, SIGNAL("currentIndexChanged(int)"), self._loadAlias)

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

        self._text['new_alias'] = QApplication.translate("option",
            "Create New", "alias", QApplication.UnicodeUTF8)
        self._text['alias'] = QApplication.translate("option",
            "Alias", None, QApplication.UnicodeUTF8)
        self._text['label'] = QApplication.translate("option", "Label", None,
                                                    QApplication.UnicodeUTF8)
        self._text['body'] = QApplication.translate("option", "Body", None,
                                                    QApplication.UnicodeUTF8)

    def _syncTabs(self, idx):
        curr_tab = self.tab_widget.currentWidget().objectName()
        if curr_tab == "tab_alias":
            self._signalConnAlias(False)
            self.list_conn_alias.clear()

            # syncronize connections list, with the exception of first
            # element, "create new"
            for i in range(self.list_conn.count() - 1):
                self.list_conn_alias.addItem(self.list_conn.itemText(i + 1))

            self._signalConnAlias(True)
            self._loadAliases(unicode(self.list_conn_alias.currentText()))

            objs = (self.list_alias, self.label_alias, self.body_alias)
            for o in objs:
                o.setEnabled(bool(self.list_conn_alias.count()))

    def _loadAliases(self, conn):
        self._signalConnAlias(False)
        self.list_alias.clear()
        self.list_alias.addItem(self._text['new_alias'])
        self.aliases = self.storage.aliases(unicode(conn))
        self.list_alias.addItems([l for l, b in self.aliases])
        self._signalConnAlias(True)
        self._loadAlias(0)

    def _loadAlias(self, idx):
        if not idx:
            l, b = '', ''
        else:
            l, b = self.aliases[idx - 1]

        self.label_alias.setText(l)
        self.body_alias.setText(b)

    def _checkAliasFields(self):
        """
        Check validity of alias fields.
        """

        msg = []

        alias_fields = {self._text['label']: self.label_alias,
                        self._text['body']: self.body_alias}

        for text, field in alias_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            QtGui.QMessageBox.warning(self, self._text['alias'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False
        return True

    def _saveAlias(self):

        if not self._checkAliasFields():
            return

        alias = (unicode(self.label_alias.text()),
                 unicode(self.body_alias.text()))

        list_idx = self.list_alias.currentIndex()
        if not list_idx:
            self.aliases.append(alias)
            self.list_alias.addItem(alias[0])
        else:
            self.aliases[list_idx - 1] = alias
            self.list_alias.setItemText(list_idx, alias[0])

        self.storage.saveAliases(unicode(self.list_conn_alias.currentText()),
                                 self.aliases)

        self.list_alias.setCurrentIndex(0)
        self._loadAlias(0)

    def _deleteAlias(self):

        list_idx = self.list_alias.currentIndex()
        if not list_idx:
            return

        del self.aliases[list_idx - 1]
        self.list_alias.removeItem(list_idx)
        self.storage.saveAliases(unicode(self.list_conn_alias.currentText()),
                                 self.aliases)

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
        for el in self.connections:
            self.list_conn.addItem(el[1], QtCore.QVariant(el[0]))

    def _loadConnection(self, name):
        """
        Load data of one connection.

        :Parameters:
          name : str
            the name of connection to load
        """

        conn = [el for el in self.connections if el[1] == name]

        if conn:
            n, h, p, d = conn[0][1:]
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

        if not self.list_conn.currentIndex():
            id_conn = 0
        else:
            data = self.list_conn.itemData(self.list_conn.currentIndex())
            id_conn = data.toInt()[0]

        make_default = self.default_conn.checkState() == QtCore.Qt.Checked

        if make_default:
            for idx, conn in enumerate(self.connections):
                if conn[0] != id_conn and conn[4]:
                    c = list(conn)
                    c[4] = 0
                    self.storage.updateConnection(c)
                    self.connections[idx] = tuple(c)

        conn = [id_conn,
                unicode(self.name_conn.text()),
                unicode(self.host_conn.text()),
                int(self.port_conn.text()),
                int(make_default)]

        if not self.list_conn.currentIndex():
            if [el[1] for el in self.connections if
                el[1] == self.name_conn.text()]:
                QtGui.QMessageBox.warning(self, self._text['connection'],
                                          self._text['unique_name'])
            else:
                self.storage.addConnection(conn)
                self.list_conn.addItem(self.name_conn.text(),
                                       QtCore.QVariant(conn[0]))
                self.connections.append(conn)
        else:
            self.connections[self.list_conn.currentIndex() - 1] = conn
            self.list_conn.setItemText(self.list_conn.currentIndex(), conn[1])
            self.storage.updateConnection(conn)

        self.list_conn.setCurrentIndex(0)
        self._loadConnection('')

    def _deleteConnection(self):
        """
        Erase a connection.
        """

        if not self.list_conn.currentIndex():
            return

        index = self.list_conn.currentIndex() - 1
        self.storage.deleteConnection(self.connections[index])
        self.list_conn.removeItem(self.list_conn.currentIndex())
        del self.connections[index]