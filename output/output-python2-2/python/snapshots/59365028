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
from PyQt4.QtCore import SIGNAL, Qt
from PyQt4.QtGui import QApplication

from storage import Storage
from gui_src.gui_option import Ui_option


class FormConnection(object):
    """
    Manage the connection part of gui option.
    """

    def __init__(self, widget, storage):
        self.w = widget
        self.storage = storage
        self.w.port_conn.setValidator(QtGui.QIntValidator(self.w.port_conn))

        self.connections = self.storage.connections()
        for el in self.connections:
            self.w.list_conn.addItem(el[1], QtCore.QVariant(el[0]))

        self._translateText()
        self._setupSignal()

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

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.save_conn, clicked, self.save)
        self.w.connect(self.w.delete_conn, clicked, self.delete)
        self.w.connect(self.w.list_conn, SIGNAL("currentIndexChanged(QString)"),
                       self.load)

        self.w.connect(self.w.connect_conn, clicked, self.connectReq)

    def load(self, name):
        """
        Load data of one connection.

        :Parameters:
          name : str
            the name of connection to load
        """

        conn = [el for el in self.connections if el[1] == name]

        if conn:
            n, h, p = conn[0][1:-1]
            d = (QtCore.Qt.Unchecked, QtCore.Qt.Checked)[conn[0][-1]]
            connect = True
        else:
            n, h, p, d = ('', '', '', QtCore.Qt.Unchecked)
            connect = False

        self.w.name_conn.setText(n)
        self.w.host_conn.setText(h)
        self.w.port_conn.setText(unicode(p))
        self.w.connect_conn.setEnabled(connect)
        self.w.default_conn.setCheckState(d)

    def _checkFields(self):
        """
        Check validity of fields.
        """

        msg = []

        conn_fields = {self._text['name']: self.w.name_conn,
                       self._text['host']: self.w.host_conn,
                       self._text['port']: self.w.port_conn}

        for text, field in conn_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            self.w._displayWarning(self._text['connection'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False

        if not self.w.list_conn.currentIndex():
            id_conn = 0
        else:
            data = self.w.list_conn.itemData(self.w.list_conn.currentIndex())
            id_conn = data.toInt()[0]

        if [el[0] for el in self.connections if
            el[1] == self.w.name_conn.text() and el[0] != id_conn]:
            self.w._displayWarning(self._text['connection'],
                                   self._text['unique_name'])
            return False

        return True

    def save(self):
        """
        Save a connection after check the fields validity.
        """

        if not self._checkFields():
            return

        if not self.w.list_conn.currentIndex():
            id_conn = 0
        else:
            data = self.w.list_conn.itemData(self.w.list_conn.currentIndex())
            id_conn = data.toInt()[0]

        make_default = self.w.default_conn.checkState() == QtCore.Qt.Checked

        if make_default:
            for idx, conn in enumerate(self.connections):
                if conn[0] != id_conn and conn[4]:
                    c = list(conn)
                    c[4] = 0
                    self.storage.updateConnection(c)
                    self.connections[idx] = tuple(c)

        conn = [id_conn,
                unicode(self.w.name_conn.text()),
                unicode(self.w.host_conn.text()),
                int(self.w.port_conn.text()),
                int(make_default)]

        if not self.w.list_conn.currentIndex():
            self.storage.addConnection(conn)
            self.w.list_conn.addItem(self.w.name_conn.text(),
                                     QtCore.QVariant(conn[0]))
            self.connections.append(conn)
        else:
            self.connections[self.w.list_conn.currentIndex() - 1] = conn
            self.w.list_conn.setItemText(self.w.list_conn.currentIndex(),
                                         conn[1])
            self.storage.updateConnection(conn)

        self.w.list_conn.setCurrentIndex(0)
        self.load('')

    def delete(self):
        """
        Erase a connection.
        """

        if not self.w.list_conn.currentIndex():
            return

        index = self.w.list_conn.currentIndex() - 1
        self.storage.deleteConnection(self.connections[index])
        self.w.list_conn.removeItem(self.w.list_conn.currentIndex())
        del self.connections[index]

    def connectReq(self):
        """
        Send a request to establish a connection.
        """

        id_conn = self.connections[self.w.list_conn.currentIndex() - 1][0]
        self.w.emit(SIGNAL('connectReq(int)'), id_conn)
        self.w.close()


class FormMacro(object):
    """
    Manage the macro part of gui option.
    """

    def __init__(self, widget, storage):
        self.w = widget
        self._translateText()

        self._key_descr = {}
        for k, v in Qt.__dict__.iteritems():
            if k.startswith('Key_'):
                self._key_descr[v] = k[4:]

        self.loadForm(storage)
        self.start_reg = False
        self._setupSignal()

    def loadForm(self, storage):
        self.storage = storage
        connections = storage.connections()
        self.w.list_conn_macro.clear()
        self.w.list_conn_macro.addItems([c[1] for c in connections])
        for o in (self.w.list_macro, self.w.command_macro,
                  self.w.register_macro):
            o.setEnabled(bool(self.w.list_conn_macro.count()))

        conn_name = unicode(connections[0][1]) if connections else None
        self.loadMacros(conn_name, True)

    def _translateText(self):
        self._text = {}

        self._text['new_macro'] = QApplication.translate("option",
            "Create New", "macro",  QApplication.UnicodeUTF8)

        self._text['macro'] = QApplication.translate("option",
            "Macro", None, QApplication.UnicodeUTF8)

        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required", None, QApplication.UnicodeUTF8)

        self._text['keys'] = QApplication.translate("option",
            "Keys", None, QApplication.UnicodeUTF8)

        self._text['command'] = QApplication.translate("option",
            "Command", None, QApplication.UnicodeUTF8)

        self._text['unique_keys'] = QApplication.translate("option",
            "Key sequence must be unique", None, QApplication.UnicodeUTF8)

    def disableSignal(self, disable):
        self.w.list_macro.blockSignals(disable)
        self.w.list_conn_macro.blockSignals(disable)

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.register_macro, clicked, self._register)
        self.w.connect(self.w.save_macro, clicked, self.save)
        self.w.connect(self.w.delete_macro, clicked, self.delete)
        self.w.connect(self.w.list_conn_macro,
                       SIGNAL("currentIndexChanged(QString)"),
                       self.loadMacros)
        self.w.connect(self.w.list_macro,
                       SIGNAL("currentIndexChanged(int)"),
                       self.load)

    def loadMacros(self, conn, signal=False):
        """
        Load all macros for a connection.

        :Parameters:
          conn : str
            the name of connection
          signal : bool
            if False the signal connected with combo must be disconnected
        """

        if not signal:
            self.disableSignal(True)

        self.macros = self.storage.macros(unicode(conn)) if conn else []
        self.w.list_macro.clear()
        self.w.list_macro.addItem(self._text['new_macro'])
        self.w.list_macro.addItems([self.getKeyDescr(*m[1:]) for m in
                                    self.macros])
        if not signal:
            self.disableSignal(False)
        self._clear()

    def load(self, idx):
        if not idx:
            k, c = '', ''
            self.key_seq = None
        else:
            m = self.macros[idx - 1]
            c = m[0]
            k = self.getKeyDescr(*m[1:])
            self.key_seq = m[1:]

        self.w.keys_macro.setText(k)
        self.w.command_macro.setText(c)

    def _checkFields(self):
        """
        Check validity of fields.
        """

        msg = []

        conn_fields = {self._text['keys']: self.w.keys_macro,
                       self._text['command']: self.w.command_macro}

        for text, field in conn_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            self.w._displayWarning(self._text['macro'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False

        cur_idx = self.w.list_macro.currentIndex()
        if [el for idx, el in enumerate(self.macros) if el[1:] == self.key_seq
            and (not cur_idx or idx != cur_idx - 1)]:
            self.w._displayWarning(self._text['macro'],
                                   self._text['unique_keys'])
            return False
        return True

    def save(self):

        if not self._checkFields():
            return

        macro = [unicode(self.w.command_macro.text())]
        macro.extend(self.key_seq)
        macro = tuple(macro)

        list_idx = self.w.list_macro.currentIndex()
        if not list_idx:
            self.macros.append(macro)
            self.w.list_macro.addItem(self.getKeyDescr(*self.key_seq))
        else:
            self.macros[list_idx - 1] = macro
            self.w.list_macro.setItemText(list_idx,
                                          self.getKeyDescr(*macro[1:]))

        conn_name = self.w.list_conn_macro.currentText()
        self.storage.saveMacros(unicode(conn_name), self.macros)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self._clear()

    def delete(self):

        list_idx = self.w.list_macro.currentIndex()
        if not list_idx:
            return

        del self.macros[list_idx - 1]
        self.w.list_macro.removeItem(list_idx)
        conn_name = self.w.list_conn_macro.currentText()
        self.storage.saveMacros(unicode(conn_name), self.macros)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)

    def _clear(self):
        self.w.list_macro.setCurrentIndex(0)
        self.load(0)
        self.w.keys_macro.setStyleSheet('')
        self.start_reg = False

    def _register(self):
        """
        Start register keyboard's event.
        """

        self.w.grabKeyboard()
        self.w.keys_macro.setText('')
        color = self.w.keys_macro.property('highlight_color').toString()
        self.w.keys_macro.setStyleSheet('background-color: %s' % color)
        self.start_reg = True

    def _getKeySeq(self, event):
        """
        Given a keyboard event, return a tuple of its components.

        :Parameters:
          event : QKeyEvent
            the keyboard event

        :return: a tuple of the form (shift, alt, ctrl, keycode)
        """

        def _checkModifier(value, mod):
            """
            Check keyboard's modifier.
            """

            return int((value & mod) == mod)

        s = _checkModifier(event.modifiers(), Qt.ShiftModifier)
        a = _checkModifier(event.modifiers(), Qt.AltModifier)
        c = _checkModifier(event.modifiers(), Qt.ControlModifier)
        return (s, a, c, event.key())

    def getKeyDescr(self, shift, alt, ctrl, key):
        """
        Return a readable description for a sequence of keys.

        :Parameters:
          shift : int
            1 if the shift key is pressed, 0 otherwise
          alt : int
            1 if the alt key is pressed, 0 otherwise
          ctrl : int
            1 if the control key is pressed, 0 otherwise
          key : int
            the code of key
        """

        return ('', 'Ctrl ')[ctrl] + ('', 'Alt ')[alt] + \
               ('', 'Shift ')[shift] + self._key_descr[key]

    def keyPressEvent(self, keyEvent):
        """
        Manage the event keyboard's saving the sequence of keys of a macro.
        """

        if self.start_reg and self._key_descr.has_key(keyEvent.key()) and \
           keyEvent.key() not in (Qt.Key_Shift, Qt.Key_Control,
                                  Qt.Key_Meta, Qt.Key_Alt):

            self.key_seq = self._getKeySeq(keyEvent)
            self.w.releaseKeyboard()
            self.w.keys_macro.setText(self.getKeyDescr(*self.key_seq))
            self.w.keys_macro.setStyleSheet('')
            self.start_reg = False


class GuiOption(QtGui.QDialog, Ui_option):
    """
    The Gui dialog for setup option.
    """

    def __init__(self, parent):
        QtGui.QDialog.__init__(self, parent)
        self.setupUi(self)
        self._setupSignal()
        self._translateText()

        self.storage = Storage()
        """an instance of `Storage`, used to maintain persistence."""

        self.conn = FormConnection(self, self.storage)
        """the `FormConnection` instance, used to manage tab of connections."""

        self.macro = FormMacro(self, self.storage)
        """the `FormMacro` instance, used to manage tab of macros."""

    def _displayWarning(self, title, message):
        QtGui.QMessageBox.warning(self, title, message)

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.connect(self.tab_widget, SIGNAL("currentChanged(int)"),
                     self._syncTabs)

        self.connect(self.save_alias, clicked, self._saveAlias)
        self.connect(self.delete_alias, clicked, self._deleteAlias)
        self.connect(self.list_conn_alias,
                     SIGNAL("currentIndexChanged(QString)"),
                     self._loadAliases)
        self.connect(self.list_alias,
                     SIGNAL("currentIndexChanged(int)"),
                     self._loadAlias)

    def disableSignal(self, disable):
        self.list_alias.blockSignals(disable)
        self.list_conn_alias.blockSignals(disable)

    def _translateText(self):
        self._text = {}

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

    def keyPressEvent(self, keyEvent):
        curr_tab = self.tab_widget.currentWidget().objectName()
        if curr_tab == "tab_macro" and self.macro:
            self.macro.keyPressEvent(keyEvent)

    def _syncTabs(self, idx):
        curr_tab = self.tab_widget.currentWidget().objectName()
        if curr_tab == "tab_alias":
            self.disableSignal(True)
            self.list_conn_alias.clear()
            self.list_conn_alias.addItems([c[1] for c in self.conn.connections])
            self.disableSignal(False)
            self._loadAliases(unicode(self.list_conn_alias.currentText()))

            for o in (self.list_alias, self.label_alias, self.body_alias):
                o.setEnabled(bool(self.list_conn_alias.count()))

        elif curr_tab == "tab_macro":
            self.macro.disableSignal(True)
            self.macro.loadForm(self.storage)
            self.macro.disableSignal(False)

    def _loadAliases(self, conn):
        self.disableSignal(True)
        self.list_alias.clear()
        self.list_alias.addItem(self._text['new_alias'])
        self.aliases = self.storage.aliases(unicode(conn))
        self.list_alias.addItems([l for l, b in self.aliases])
        self.disableSignal(False)
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
            self._displayWarning(self._text['alias'],
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

        conn_name = self.list_conn_alias.currentText()
        self.storage.saveAliases(unicode(conn_name), self.aliases)
        self.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self.list_alias.setCurrentIndex(0)
        self._loadAlias(0)

    def _deleteAlias(self):

        list_idx = self.list_alias.currentIndex()
        if not list_idx:
            return

        del self.aliases[list_idx - 1]
        self.list_alias.removeItem(list_idx)
        conn_name = self.list_conn_alias.currentText()
        self.storage.saveAliases(unicode(conn_name), self.aliases)
        self.emit(SIGNAL('reloadConnData(QString)'), conn_name)
