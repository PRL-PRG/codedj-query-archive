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

from re import compile

from PyQt4 import QtGui
from PyQt4.QtCore import SIGNAL, Qt, QVariant
from PyQt4.QtGui import QApplication, QDialog, QColorDialog, QMessageBox

import storage
from gui_src.gui_option import Ui_option


class FormConnection(object):
    """
    Manage the connection part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self.w.port_conn.setValidator(QtGui.QIntValidator(self.w.port_conn))

        self.connections = storage.connections()
        for el in self.connections:
            self.w.list_conn.addItem(el[1], QVariant(el[0]))

        self._translateText()
        self._setupSignal()

    def _translateText(self):
        self._text = {}
        self._text['name'] = QApplication.translate("option", "Name")
        self._text['host'] = QApplication.translate("option", "Host")
        self._text['port'] = QApplication.translate("option", "Port")

        self._text['connection'] = QApplication.translate("option", "Connection")

        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required")

        self._text['unique_name'] = QApplication.translate("option",
            "Connection name must be unique")

        self._text['confirm_delete'] = QApplication.translate("option",
            "Are you sure to delete the connection?")

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.save_conn, clicked, self.save)
        self.w.connect(self.w.delete_conn, clicked, self.delete)
        self.w.connect(self.w.list_conn, SIGNAL("currentIndexChanged(QString)"),
                       self.load)

    def load(self, name):
        """
        Load data of one connection.

        :Parameters:
          name : str
            the name of connection to load
        """

        conn = [el for el in self.connections if el[1] == name]

        if conn:
            n, h, p = conn[0][1:]
            connect = True
        else:
            n, h, p = ('', '', '')
            connect = False

        self.w.name_conn.setText(n)
        self.w.host_conn.setText(h)
        self.w.port_conn.setText(unicode(p))

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

        conn = [id_conn,
                unicode(self.w.name_conn.text()),
                unicode(self.w.host_conn.text()),
                int(self.w.port_conn.text())]

        if not self.w.list_conn.currentIndex():
            storage.addConnection(conn)
            self.w.list_conn.addItem(self.w.name_conn.text(), QVariant(conn[0]))
            self.connections.append(conn)
        else:
            self.connections[self.w.list_conn.currentIndex() - 1] = conn
            self.w.list_conn.setItemText(self.w.list_conn.currentIndex(),
                                         conn[1])
            storage.updateConnection(conn)

        self.w.list_conn.setCurrentIndex(0)
        self.w.emit(SIGNAL('reloadConnData(QString)'), '')
        self.load('')

    def delete(self):
        """
        Erase a connection.
        """

        if self.w.list_conn.currentIndex() <= 0:
            return

        if storage.connectionHasChild(unicode(self.w.list_conn.currentText())) \
           and not self.w._displayQuestion(self._text['connection'],
                                           self._text['confirm_delete']):
            return

        index = self.w.list_conn.currentIndex() - 1
        storage.deleteConnection(self.connections[index])
        self.w.list_conn.removeItem(self.w.list_conn.currentIndex())
        self.w.emit(SIGNAL('reloadConnData(QString)'), '')
        del self.connections[index]


class FormMacro(object):
    """
    Manage the macro part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self._translateText()

        self._key_descr = {}
        for k, v in Qt.__dict__.iteritems():
            if k.startswith('Key_'):
                self._key_descr[v] = k[4:]

        self.loadForm()
        self.start_reg = False
        self._setupSignal()

    def loadForm(self):
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

        self._text['new_macro'] = QApplication.translate("option", "Create New",
            "macro")

        self._text['macro'] = QApplication.translate("option", "Macro")

        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required")

        self._text['keys'] = QApplication.translate("option", "Keys")
        self._text['command'] = QApplication.translate("option", "Command")

        self._text['unique_keys'] = QApplication.translate("option",
            "Key sequence must be unique")

        self._text['shortcut_keys'] = QApplication.translate("option",
            "Key sequence must be different from all the shortcut keys")

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

        self.macros = storage.macros(unicode(conn)) if conn else []
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

        # FIX: the shortcuts should be read (and write) from storage
        # Format: shift, alt, ctrl, keycode
        shortcuts = [(0, 1, 0, Qt.Key_C),
                     (0, 1, 0, Qt.Key_O),
                     (0, 1, 0, Qt.Key_Q),
                     (0, 0, 0, Qt.Key_Enter),
                     (0, 0, 0, Qt.Key_Return),
                     (0, 0, 0, Qt.Key_Up),
                     (0, 0, 0, Qt.Key_Down)]

        if [el for el in shortcuts if el == self.key_seq]:
            self.w._displayWarning(self._text['macro'],
                                   self._text['shortcut_keys'])
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
        storage.saveMacros(unicode(conn_name), self.macros)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self._clear()

    def delete(self):

        list_idx = self.w.list_macro.currentIndex()
        if list_idx <= 0:
            return

        del self.macros[list_idx - 1]
        self.w.list_macro.removeItem(list_idx)
        conn_name = self.w.list_conn_macro.currentText()
        storage.saveMacros(unicode(conn_name), self.macros)
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

        def _checkModifier(event, mod):
            """
            Check keyboard's modifier.
            """

            return int((event.modifiers() & mod) == mod)

        s = _checkModifier(event, Qt.ShiftModifier)
        a = _checkModifier(event, Qt.AltModifier)
        c = _checkModifier(event, Qt.ControlModifier)
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


class FormPreferences(object):
    """
    Manage the preferences part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self._loadForm()
        self._setupSignal()

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.echo_color_button, clicked, self._getEchoColor)
        self.w.connect(self.w.save_preferences, clicked, self.save)

    def _getEchoColor(self):
        color = QColorDialog.getColor().name()
        self.w.echo_color.setText(unicode(color).upper())

    def _loadForm(self):
        preferences = storage.preferences()
        if preferences:
            echo_text = (Qt.Unchecked, Qt.Checked)[preferences[0]]
            self.w.echo_text.setCheckState(echo_text)
            self.w.echo_color.setText(preferences[1])
            keep_text = (Qt.Unchecked, Qt.Checked)[preferences[2]]
            self.w.keep_text.setCheckState(keep_text)
            save_log = (Qt.Unchecked, Qt.Checked)[preferences[3]]
            self.w.save_log.setCheckState(save_log)

    def save(self):
        preferences = (int(self.w.echo_text.checkState() == Qt.Checked),
                       unicode(self.w.echo_color.text()),
                       int(self.w.keep_text.checkState() == Qt.Checked),
                       int(self.w.save_log.checkState() == Qt.Checked))

        storage.savePreferences(preferences)
        self.w.emit(SIGNAL('reloadPreferences()'))


class FormAccounts(object):
    """
    Manage the accounts part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self.loadForm()
        self._translateText()
        self._setupSignal()

    def loadForm(self):
        connections = storage.connections()
        self.w.list_conn_account.clear()
        for el in connections:
            self.w.list_conn_account.addItem(el[1], QVariant(el[0]))
        if connections:
            self._loadAccounts(0)
        val = storage.option('save_account')
        self.w.save_account.setCheckState(Qt.Checked if val else Qt.Unchecked)
        self.w.box_prompt.setVisible(False)

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        change_idx = SIGNAL("currentIndexChanged(int)")
        self.w.connect(self.w.list_conn_account, change_idx, self._loadAccounts)
        self.w.connect(self.w.delete_account, clicked, self.deleteAccount)
        self.w.connect(self.w.save_account, SIGNAL('stateChanged(int)'),
                       self._saveAccounts)
        self.w.connect(self.w.change_prompt, clicked, self._togglePrompt)
        self.w.connect(self.w.save_prompt, clicked, self._savePrompt)
        self.w.connect(self.w.list_account, change_idx, self._loadAccount)

    def _translateText(self):
        self._text = {}

        self._text['accounts'] = QApplication.translate("option", "Accounts")

        self._text['bad_format'] = QApplication.translate("option",
            "Bad format on prompt", "accounts")

    def _togglePrompt(self):
        self.w.box_prompt.setVisible(not self.w.box_prompt.isVisible())

    def _loadAccounts(self, idx):
        id_conn = self.w.list_conn_account.itemData(idx).toInt()[0]
        self.w.list_account.blockSignals(True)
        self.w.list_account.clear()
        accounts = storage.accounts(id_conn)
        self.w.list_account.addItems(accounts)
        self.w.list_account.blockSignals(False)
        self.w.delete_account.setEnabled(True if accounts else False)
        self.w.change_prompt.setEnabled(True if accounts else False)
        self.w.box_prompt.setVisible(False)
        self._loadAccount()

    def _loadAccount(self, i=0):
        idx = self.w.list_conn_account.currentIndex()
        id_conn = self.w.list_conn_account.itemData(idx).toInt()[0]
        username = unicode(self.w.list_account.currentText())
        if id_conn and username:
            n_prompt, f_prompt = storage.prompt(id_conn, username)
            self.w.normal_prompt.setText(n_prompt)
            self.w.fight_prompt.setText(f_prompt)

    def disableSignal(self, disable):
        self.w.list_conn_account.blockSignals(disable)
        self.w.list_account.blockSignals(disable)

    def deleteAccount(self):
        idx = self.w.list_conn_account.currentIndex()
        id_conn = self.w.list_conn_account.itemData(idx).toInt()[0]
        username = unicode(self.w.list_account.currentText())
        storage.deleteAccount(id_conn, username)
        self.w.list_account.removeItem(self.w.list_account.currentIndex())
        self.w.emit(SIGNAL('reloadConnData(QString)'), '')
        if not self.w.list_account.count():
            self.w.delete_account.setEnabled(False)
            self.w.change_prompt.setEnabled(False)

    def _saveAccounts(self, val):
        storage.setOption('save_account', int(val == Qt.Checked))

    def _savePrompt(self):
        idx = self.w.list_conn_account.currentIndex()
        id_conn = self.w.list_conn_account.itemData(idx).toInt()[0]
        username = unicode(self.w.list_account.currentText())
        normal = unicode(self.w.normal_prompt.text())
        fight = unicode(self.w.fight_prompt.text())
        d = {normal: self.w.normal_prompt, fight: self.w.fight_prompt}
        for text, field in d.iteritems():
            if text:
                for c in 'hHmMvV':
                    if text.count('%' + c) != 1:
                        self.w._displayWarning(self._text['accounts'],
                                               self._text['bad_format'])
                        field.setFocus()
                        return

        storage.savePrompt(id_conn, username, normal, fight)
        conn_name = self.w.list_conn_account.currentText()
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self.w.box_prompt.setVisible(False)


class FormAliases(object):
    """
    Manage the aliases part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self._translateText()
        self.loadForm()
        self._setupSignal()

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.save_alias, clicked, self._saveAlias)
        self.w.connect(self.w.delete_alias, clicked, self._deleteAlias)
        self.w.connect(self.w.list_conn_alias,
                      SIGNAL("currentIndexChanged(QString)"),
                      self._loadAliases)
        self.w.connect(self.w.list_alias,
                      SIGNAL("currentIndexChanged(int)"),
                      self._loadAlias)

    def disableSignal(self, disable):
        self.w.list_alias.blockSignals(disable)
        self.w.list_conn_alias.blockSignals(disable)

    def _translateText(self):
        self._text = {}
        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required")
        self._text['new_alias'] = QApplication.translate("option",
            "Create New", "alias")
        self._text['alias'] = QApplication.translate("option", "Alias")
        self._text['label'] = QApplication.translate("option", "Label")
        self._text['body'] = QApplication.translate("option", "Body")
        self._text['unique_label'] = QApplication.translate("option",
            "Alias label must be unique")

    def loadForm(self):
        self.w.list_conn_alias.clear()
        self.w.list_conn_alias.addItems([c[1] for c in storage.connections()])

        if self.w.list_conn_alias.count():
            self._loadAliases(unicode(self.w.list_conn_alias.currentText()))

        for o in (self.w.list_alias, self.w.label_alias, self.w.body_alias):
            o.setEnabled(bool(self.w.list_conn_alias.count()))

    def _loadAliases(self, conn):
        self.disableSignal(True)
        self.w.list_alias.clear()
        self.w.list_alias.addItem(self._text['new_alias'])
        self.aliases = storage.aliases(unicode(conn))
        self.w.list_alias.addItems([l for l, b in self.aliases])
        self.disableSignal(False)
        self._loadAlias(0)

    def _loadAlias(self, idx):
        if not idx:
            l, b = '', ''
        else:
            l, b = self.aliases[idx - 1]

        self.w.label_alias.setText(l)
        self.w.body_alias.setText(b)

    def _checkAliasFields(self):
        """
        Check validity of alias fields.
        """

        msg = []

        alias_fields = {self._text['label']: self.w.label_alias,
                        self._text['body']: self.w.body_alias}

        for text, field in alias_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            self.w._displayWarning(self._text['alias'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False


        if [el[0] for el in self.aliases if
            el[0] == self.w.label_alias.text() and
            not self.w.list_alias.currentIndex()]:
            self.w._displayWarning(self._text['alias'],
                                   self._text['unique_label'])
            return False

        return True

    def _saveAlias(self):

        if not self._checkAliasFields():
            return

        alias = (unicode(self.w.label_alias.text()),
                 unicode(self.w.body_alias.text()))

        list_idx = self.w.list_alias.currentIndex()
        if not list_idx:
            self.aliases.append(alias)
            self.w.list_alias.addItem(alias[0])
        else:
            self.aliases[list_idx - 1] = alias
            self.w.list_alias.setItemText(list_idx, alias[0])

        conn_name = self.w.list_conn_alias.currentText()
        storage.saveAliases(unicode(conn_name), self.aliases)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self.w.list_alias.setCurrentIndex(0)
        self._loadAlias(0)

    def _deleteAlias(self):

        list_idx = self.w.list_alias.currentIndex()
        if list_idx <= 0:
            return

        del self.aliases[list_idx - 1]
        self.w.list_alias.removeItem(list_idx)
        conn_name = self.w.list_conn_alias.currentText()
        storage.saveAliases(unicode(conn_name), self.aliases)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)


class FormTriggers(object):
    """
    Manage the triggers part of gui option.
    """

    def __init__(self, widget):
        self.w = widget
        self._translateText()
        self.loadForm()
        self._setupSignal()

    def _setupSignal(self):
        clicked = SIGNAL("clicked()")
        self.w.connect(self.w.save_trigger, clicked, self._saveTrigger)
        self.w.connect(self.w.delete_trigger, clicked, self._deleteTrigger)
        self.w.connect(self.w.list_conn_trigger,
                      SIGNAL("currentIndexChanged(QString)"), self._loadTriggers)
        self.w.connect(self.w.list_trigger,
                      SIGNAL("currentIndexChanged(int)"), self._loadTrigger)
        self.w.connect(self.w.radio_command_trigger,
                       SIGNAL("toggled(bool)"), self._toggleChoice)
        self.w.connect(self.w.text_color_trigger_button, clicked,
                       self._getTextColor)
        self.w.connect(self.w.bg_color_trigger_button, clicked,
                       self._getBgColor)

    def _setLabelColor(self, label, color):
        if not color:
            return self._clearLabelColor(label)

        style = unicode(label.styleSheet())
        reg = compile('QLabel\s*{background-color\s*:\s*(#\w{6})}')
        m = reg.search(style)
        if m:
            l, r = m.span(1)
            style = style[:l] + color + style[r:]
        else:
            style += "QLabel{background-color:%s;}" % color

        label.setStyleSheet(style)

    def _clearLabelColor(self, label):
        style = unicode(label.styleSheet())
        reg = compile('QLabel\s*{(background-color\s*:\s*#\w{6}\s*;)}')
        m = reg.search(style)
        if m:
            l, r = m.span(1)
            style = style[:l] + style[r:]
        label.setStyleSheet(style)

    def _getTextColor(self):
        c = QColorDialog.getColor()
        self._text_color = unicode(c.name()) if c.isValid() else ''
        self._setLabelColor(self.w.text_color_trigger, self._text_color)

    def _getBgColor(self):
        c = QColorDialog.getColor()
        self._bg_color = unicode(c.name()) if c.isValid() else ''
        self._setLabelColor(self.w.bg_color_trigger, self._bg_color)

    def disableSignal(self, disable):
        self.w.list_trigger.blockSignals(disable)
        self.w.list_conn_trigger.blockSignals(disable)

    def _toggleChoice(self):
        """
        Toggle the choice between action and highlight trigger
        """

        t_items = [self.w.command_trigger]
        h_items = [self.w.text_color_trigger_button,
                   self.w.text_color_trigger,
                   self.w.bg_color_trigger_button,
                   self.w.bg_color_trigger]

        radio_enabled = self.w.radio_command_trigger.isChecked()

        for i in t_items:
            i.setEnabled(radio_enabled)

        for i in h_items:
            i.setEnabled(not radio_enabled)

        if radio_enabled:
            self._text_color, self._bg_color = '', ''
            self._clearLabelColor(self.w.text_color_trigger)
            self._clearLabelColor(self.w.bg_color_trigger)
        else:
            self.w.command_trigger.setText('')

    def _translateText(self):
        self._text = {}
        self._text['req_fields'] = QApplication.translate("option",
            "The following fields are required")
        self._text['new_trigger'] = QApplication.translate("option",
            "Create New", "trigger")
        self._text['trigger'] = QApplication.translate("option", "Trigger")
        self._text['pattern'] = QApplication.translate("option", "Pattern")
        self._text['command'] = QApplication.translate("option", "Command")
        self._text['unique_pattern'] = QApplication.translate("option",
            "Trigger pattern must be unique")
        self._text['req_colors'] = QApplication.translate("option",
            "At least one between text and background color is required")

    def loadForm(self):
        self.w.list_conn_trigger.clear()
        self.w.list_conn_trigger.addItems([c[1] for c in storage.connections()])

        if self.w.list_conn_trigger.count():
            self._loadTriggers(unicode(self.w.list_conn_trigger.currentText()))

        for o in (self.w.list_trigger, self.w.pattern_trigger,
                  self.w.command_trigger, self.w.case_trigger):
            o.setEnabled(bool(self.w.list_conn_trigger.count()))

        self._text_color, self._bg_color = '', ''

    def _loadTriggers(self, conn):
        self.disableSignal(True)
        self.w.list_trigger.clear()
        self.w.list_trigger.addItem(self._text['new_trigger'])
        self.triggers = storage.triggers(unicode(conn))
        self.w.list_trigger.addItems([el[0] for el in self.triggers])
        self.disableSignal(False)
        self._loadTrigger(0)

    def _loadTrigger(self, idx):
        if not idx:
            patt, case, comm, bg, fg = '', 0, '', '', ''
        else:
            patt, case, comm, bg, fg = self.triggers[idx - 1]

        self.w.pattern_trigger.setText(patt)
        self.w.command_trigger.setText(comm)
        self.w.case_trigger.setCheckState((Qt.Unchecked, Qt.Checked)[case])

        self._text_color = fg
        self._bg_color = bg
        self._setLabelColor(self.w.text_color_trigger, self._text_color)
        self._setLabelColor(self.w.bg_color_trigger, self._bg_color)
        if bool(bg or fg):
            self.w.radio_color_trigger.setChecked(True)
        else:
            self.w.radio_command_trigger.setChecked(True)

    def _checkTriggerFields(self):
        """
        Check validity of trigger fields.
        """

        msg = []

        trigger_fields = {self._text['pattern']: self.w.pattern_trigger}

        if self.w.radio_command_trigger.isChecked():
            trigger_fields[self._text['command']] = self.w.command_trigger
        else:
            if not self._text_color and not self._bg_color:
                self.w._displayWarning(self._text['trigger'],
                                       self._text['req_colors']);
                return False

        for text, field in trigger_fields.iteritems():
            if not field.text():
                msg.append(unicode(text))

        if msg:
            self.w._displayWarning(self._text['trigger'],
                "%s:\n%s" % (self._text['req_fields'], '\n'.join(msg)))
            return False

        if [el[0] for el in self.triggers if
            el[0].upper() == unicode(self.w.pattern_trigger.text()).upper() and
            not self.w.list_trigger.currentIndex()]:
            self.w._displayWarning(self._text['trigger'],
                                   self._text['unique_pattern'])
            return False

        return True

    def _saveTrigger(self):

        if not self._checkTriggerFields():
            return

        trigger = (unicode(self.w.pattern_trigger.text()),
                   int(self.w.case_trigger.checkState() == Qt.Checked),
                   unicode(self.w.command_trigger.text()),
                   self._bg_color,
                   self._text_color)

        list_idx = self.w.list_trigger.currentIndex()
        if not list_idx:
            self.triggers.append(trigger)
            self.w.list_trigger.addItem(trigger[0])
        else:
            self.triggers[list_idx - 1] = trigger
            self.w.list_trigger.setItemText(list_idx, trigger[0])

        conn_name = self.w.list_conn_trigger.currentText()
        storage.saveTriggers(unicode(conn_name), self.triggers)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)
        self.w.list_trigger.setCurrentIndex(0)
        self._loadTrigger(0)

    def _deleteTrigger(self):

        list_idx = self.w.list_trigger.currentIndex()
        if list_idx <= 0:
            return

        del self.triggers[list_idx - 1]
        self.w.list_trigger.removeItem(list_idx)
        conn_name = self.w.list_conn_trigger.currentText()
        storage.saveTriggers(unicode(conn_name), self.triggers)
        self.w.emit(SIGNAL('reloadConnData(QString)'), conn_name)


class GuiOption(QDialog, Ui_option):
    """
    The Gui dialog for setup option.
    """

    def __init__(self, parent):
        QDialog.__init__(self, parent)
        self.setupUi(self)
        self._setupSignal()

        self.conn = FormConnection(self)
        """the `FormConnection` instance, used to manage form of connections."""

        self.macro = FormMacro(self)
        """the `FormMacro` instance, used to manage form of macros."""

        self.preferences = FormPreferences(self)
        """the `FormPreferences` instance, used to manage form of preferences."""

        self.accounts = FormAccounts(self)
        """the FormAccounts instance, used to manage form of accounts."""

        self.alias = FormAliases(self)
        """the FormAliases instance, used to manage form of aliases."""

        self.trigger = FormTriggers(self)
        """the FormTriggers instance, used to manage form of triggers."""

    def _displayWarning(self, title, message):
        QMessageBox.warning(self, title, message)

    def _displayQuestion(self, title, message):
        b = QMessageBox.question(self, title, message,
                                 QMessageBox.Yes, QMessageBox.No)
        return b == QMessageBox.Yes

    def _setupSignal(self):
        self.connect(self.list_option,
                     SIGNAL("currentItemChanged(QListWidgetItem*, QListWidgetItem*)"),
                     self._changeForm)

    def keyPressEvent(self, keyEvent):
        curr_page = self.page_container.currentWidget().objectName()
        if curr_page == "macro_page" and self.macro:
            self.macro.keyPressEvent(keyEvent)

    def _changeForm(self, current, previous):
        self.page_container.setCurrentIndex(self.list_option.currentRow())
        curr_page = str(self.page_container.currentWidget().objectName())

        objs = {'alias_page': self.alias,
                'macro_page': self.macro,
                'account_page': self.accounts,
                'trigger_page': self.trigger}

        form = objs.get(curr_page)
        if form:
            form.disableSignal(True)
            form.loadForm()
            form.disableSignal(False)
