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

import os
import sys
import shutil
import os.path
import unittest

from PyQt4 import QtCore, QtGui
from PyQt4.QtCore import Qt
from PyQt4.QtGui import QApplication, QLineEdit, QGroupBox
from PyQt4.QtGui import QPushButton, QComboBox, QCheckBox

# FIX
sys.path.append('..')
sys.path.append('../configobj')
sys.path.append('../../resources')

import devclient.storage
from devclient.conf import config
from devclient.gui_option import *


class GuiOptionTest(unittest.TestCase):

    def __init__(self, methodName='runTest'):
        super(GuiOptionTest, self).__init__(methodName)
        if not QApplication.instance():
            self.app = QApplication([])

        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        devclient.storage.loadStorage()

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)


class GuiOptionMock(object):

    def __init__(self):
        self.name_conn = QLineEdit()
        self.host_conn = QLineEdit()
        self.port_conn = QLineEdit()
        self.list_conn = QComboBox()
        self.save_conn = QPushButton()
        self.delete_conn = QPushButton()
        self.connect_conn = QPushButton()
        self.list_conn.addItem("Create New")
        self._warning = None

    def connect(self, widget, signal, callback):
        pass

    def _displayWarning(self, title, message):
        self._warning = (title, message)

    def emit(self, signal, args):
        pass


class TestFormConnection(GuiOptionTest):

    def _formCompare(self, form_conn, conn):

        if form_conn.w.name_conn.text() != conn[1] or \
           form_conn.w.host_conn.text() != conn[2] or  \
           form_conn.w.port_conn.text() != str(conn[3]):
            return False
        return True

    def _buildForm(self, name, host, port):
        form_conn = FormConnection(GuiOptionMock())
        form_conn.w.name_conn.setText(name)
        form_conn.w.host_conn.setText(host)
        form_conn.w.port_conn.setText(str(port))
        return form_conn

    def _checkEmptyForm(self, form_conn):
        if not form_conn.w.name_conn.text() and \
           not form_conn.w.host_conn.text() and \
           not form_conn.w.port_conn.text() and \
           not form_conn.w.list_conn.currentIndex():
            return True
        return False

    def testEmpty(self):
        form_conn = FormConnection(GuiOptionMock())
        self.assert_(len(form_conn.connections) == 0)

    def testLoad(self):
        id_conn = 0
        port = 6000
        name, host = "name", "host"
        conn = [id_conn, name, host, port]

        storage.addConnection(conn)
        form_conn = FormConnection(GuiOptionMock())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad2(self):
        id_conn = 0
        port = 6000
        name, host = "name", "host"
        conn = [id_conn, name, host, port]

        storage.addConnection(conn)
        storage.addConnection([0, 'test', 'test', 4000])
        form_conn = FormConnection(GuiOptionMock())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad3(self):
        form_conn = FormConnection(GuiOptionMock())

        form_conn.load('fake')
        self.assert_(self._formCompare(form_conn, [0, '', '', '']))

    def testLoad4(self):
        form_conn = FormConnection(GuiOptionMock())

        form_conn.load('')
        self.assert_(self._formCompare(form_conn, [0, '', '', '']))

    def testCheckField1(self):
        """Verify error with empty fields."""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionMock())
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField2(self):
        """
        Verify error on adding a connection with a name that already exists.
        """

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = self._buildForm('name', 'host3', 1000)
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField3(self):
        """Verify error with empty name field """

        form_conn = self._buildForm('', 'host', 1232)
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField4(self):
        """Verify error with empty host field """

        form_conn = self._buildForm('name', '', 1232)
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField5(self):
        """Verify error with empty port field """

        form_conn = self._buildForm('name', 'host', '')
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField6(self):
        """Verify no error on right update"""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionMock())
        form_conn.load('name')
        form_conn.w.list_conn.setCurrentIndex(1)
        self.assert_(form_conn._checkFields())
        self.assert_(not form_conn.w._warning)

    def testCheckField7(self):
        """Verify no error on right add"""

        form_conn = self._buildForm('name', 'host', 1232)
        self.assert_(form_conn._checkFields())
        self.assert_(not form_conn.w._warning)

    def testCheckField8(self):
        """
        Verify error on update that change name with another that already
        exists.
        """

        storage.addConnection([0, 'name', 'host', 4000])
        conn = [0, 'name2', 'host2', 3000]
        storage.addConnection(conn)
        form_conn = self._buildForm('name', 'host3', 1000)
        form_conn.w.list_conn.setCurrentIndex(2)
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testSaveAdd(self):
        """Add a connection on empty storage."""

        conn = ('name', 'host', 1232)
        form_conn = self._buildForm(*conn)
        form_conn.save()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(conn == storage.connections()[0][1:])

    def testSaveAdd2(self):
        """Add a connection on a storage with one connection."""

        storage.addConnection([0, 'name', 'host', 4000])
        conn = ('name2', 'host', 1232)
        form_conn = self._buildForm(*conn)
        form_conn.save()
        self.assert_(len(form_conn.connections) == 2)
        self.assert_(conn == storage.connections()[1][1:])

    def testSaveAdd3(self):
        """Verify that after saving form fields are empty."""

        form_conn = self._buildForm('name', 'host', 4000)
        form_conn.save()
        self.assert_(self._checkEmptyForm(form_conn))

    def testSaveUpd(self):
        """Update a connection."""

        conn = [0, 'name', 'host', 4000]
        storage.addConnection(conn)
        conn = [conn[0], 'name2', 'host2', 1234]
        form_conn = self._buildForm(*conn[1:])
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.save()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(conn == list(storage.connections()[0]))

    def testSaveUpd2(self):
        """Verify that update on a connection change the in combo box."""

        conn = [0, 'name', 'host', 4000]
        storage.addConnection(conn)
        conn = [conn[0], 'name2', 'host2', 1234]
        form_conn = self._buildForm(*conn[1:])
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.save()
        self.assert_(form_conn.w.list_conn.itemText(1) == conn[1])

    def testSaveUpd3(self):
        """Verify that after saving form fields are empty."""

        conn = [0, 'name', 'host', 4000]
        storage.addConnection(conn)
        conn = [conn[0], 'name2', 'host2', 1234]
        form_conn = self._buildForm(*conn[1:])
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.save()
        self.assert_(self._checkEmptyForm(form_conn))

    def testDelete(self):
        """Delete 'create new' item."""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionMock())
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)

    def testDelete2(self):
        """Delete a connection."""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionMock())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 0)

    def testDelete3(self):
        """Delete a connection on a storage of two connections."""

        storage.addConnection([0, 'name', 'host', 4000])
        conn = [0, 'name2', 'host2', 3000]
        storage.addConnection(conn)
        form_conn = FormConnection(GuiOptionMock())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(len(storage.connections()) == 1)
        self.assert_(form_conn.connections[0][1:] == tuple(conn[1:]))


class GuiOptionMacroMock(object):

    def __init__(self):
        self.register_macro = QPushButton()
        self.save_macro = QPushButton()
        self.delete_macro = QPushButton()
        self.list_macro = QComboBox()
        self.list_conn_macro = QComboBox()
        self.command_macro = QLineEdit()
        self.keys_macro = QLineEdit()
        self._warning = None

    def connect(self, widget, signal, callback):
        pass

    def disconnect(self, widget, signal, callback):
        pass

    def _displayWarning(self, title, message):
        self._warning = (title, message)

    def emit(self, signal, args):
        pass


class TestFormMacro(GuiOptionTest):

    def _formCompare(self, form_macro, macro):
        if form_macro.w.command_macro.text() != macro[0] or \
           form_macro.key_seq != macro[1:] or \
           form_macro.w.keys_macro.text() != form_macro.getKeyDescr(*macro[1:]):
            return False
        return True

    def _setFormFields(self, form_macro, macro):
        form_macro.w.keys_macro.setText(form_macro.getKeyDescr(*macro[1:]))
        form_macro.w.command_macro.setText(macro[0])
        form_macro.key_seq = macro[1:]

    def testLoadEmpty(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form_macro = FormMacro(GuiOptionMacroMock())
        self.assert_(form_macro.w.list_macro.count() == 1)

    def testLoadEmpty2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock())

        form_macro.load(0)
        self.assert_(not form_macro.w.command_macro.text() and
                     not form_macro.w.keys_macro.text())

    def testLoadEmpty3(self):
        form_macro = FormMacro(GuiOptionMacroMock())
        self.assert_(form_macro.w.list_macro.count() == 1)

    def testLoad(self):
        storage.addConnection([0, 'name', 'host', 4000])
        macro = ('command', 1, 0, 0, 65)
        storage.saveMacros('name', [macro])
        form_macro = FormMacro(GuiOptionMacroMock())

        form_macro.load(1)
        self.assert_(self._formCompare(form_macro, macro))

    def testLoad2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        macro = ('command', 0, 1, 1, 72)
        storage.saveMacros('name', [('command', 1, 0, 0, 65), macro])
        form_macro = FormMacro(GuiOptionMacroMock())

    def testCheckField1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock())
        form_macro.load(1)
        self.assert_(not form_macro._checkFields())

    def testCheckField2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock())
        form_macro.load(1)
        form_macro.w.list_macro.setCurrentIndex(1)
        self.assert_(form_macro._checkFields())

    def testCheckField3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        macro = ('command', 1, 0, 1, 77)
        storage.saveMacros('name', [('command', 1, 0, 0, 65), macro])
        form_macro = FormMacro(GuiOptionMacroMock())
        form_macro.load(1)
        form_macro.w.list_macro.setCurrentIndex(1)
        form_macro.w.keys_macro.setText(form_macro.getKeyDescr(*macro[1:]))
        form_macro.key_seq = macro[1:]
        self.assert_(not form_macro._checkFields())

    def testCheckField4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock())
        self._setFormFields(form_macro, ('command', 0, 0, 0, 78))
        form_macro.save()
        self._setFormFields(form_macro, ('check', 0, 0, 0, 78))
        self.assert_(not form_macro._checkFields())

    def testCheckField5(self):
        storage.addConnection([0, 'name', 'host', 4000])
        macro = ('command', 1, 0, 0, 65)
        storage.saveMacros('name', [macro])
        form_macro = FormMacro(GuiOptionMacroMock())
        form_macro.load(1)
        form_macro.w.list_macro.setCurrentIndex(1)
        form_macro.save()

        self._setFormFields(form_macro, macro)
        form_macro.w.list_macro.setCurrentIndex(0)
        self.assert_(not form_macro._checkFields())

    def testSaveAdd(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form_macro = FormMacro(GuiOptionMacroMock())
        macro = ('command', 1, 0, 0, 65)
        self._setFormFields(form_macro, macro)
        form_macro.save()

        self.assert_(not form_macro.w._warning)
        self.assert_(storage.macros('name')[0] == macro)
        self.assert_(len(form_macro.macros) == 1)

    def testSaveAdd2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock())
        macro = ('command', 0, 0, 0, 78)
        self._setFormFields(form_macro, macro)
        form_macro.save()

        self.assert_(not form_macro.w._warning)
        self.assert_(storage.macros('name')[1] == macro)
        self.assert_(len(form_macro.macros) == 2)


class GuiOptionPrefMock(object):

    def __init__(self):
        self.echo_text = QCheckBox()
        self.echo_color = QLineEdit()
        self.save_preferences = QPushButton()
        self.echo_color_button = QPushButton()
        self.keep_text = QCheckBox()
        self.save_log = QCheckBox()

    def connect(self, widget, signal, callback):
        pass

    def emit(self, signal, args=None):
        pass


class TestFormPreferences(GuiOptionTest):

    def testLoadEmpty(self):
        form = FormPreferences(GuiOptionPrefMock())
        self.assert_(form.w.echo_text.checkState() == Qt.Checked)
        self.assert_(form.w.echo_color.text() == '#00AA00')
        self.assert_(form.w.keep_text.checkState() == Qt.Unchecked)
        self.assert_(form.w.save_log.checkState() == Qt.Unchecked)

    def testSavePreferences(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.echo_text.setCheckState(Qt.Unchecked)
        form.save()
        self.assert_(storage.preferences() == (0, '#00AA00', 0, 0))

    def testSavePreferences2(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.keep_text.setCheckState(Qt.Checked)
        form.w.echo_color.setText('#CC0000')
        form.save()
        self.assert_(storage.preferences() == (1, '#CC0000', 1, 0))

    def testSavePreferences3(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.echo_text.setCheckState(Qt.Checked)
        form.w.keep_text.setCheckState(Qt.Unchecked)
        form.w.echo_color.setText('#CC0000')
        form.w.save_log.setCheckState(Qt.Checked)
        form.save()
        self.assert_(storage.preferences() == (1, '#CC0000', 0, 1))

    def testSavePreferences4(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.echo_text.setCheckState(Qt.Checked)
        form.w.keep_text.setCheckState(Qt.Unchecked)
        form.w.echo_color.setText('#CC0000')
        form.w.save_log.setCheckState(Qt.Unchecked)
        form.save()
        form.w.echo_text.setCheckState(Qt.Unchecked)
        form.w.keep_text.setCheckState(Qt.Checked)
        form.w.echo_color.setText('#000000')
        form.w.save_log.setCheckState(Qt.Checked)
        form.save()
        self.assert_(storage.preferences() == (0, '#000000', 1, 1))


class GuiOptionAccMock(object):

    def __init__(self):
        self.save_account = QCheckBox()
        self.delete_account = QPushButton()
        self.list_account = QComboBox()
        self.list_conn_account = QComboBox()
        self.change_prompt = QPushButton()
        self.save_prompt = QPushButton()
        self.normal_prompt = QLineEdit()
        self.fight_prompt = QLineEdit()
        self.box_prompt = QGroupBox()
        self._warning = None
        self.delete_account.setEnabled(False)
        self.change_prompt.setEnabled(False)

    def _displayWarning(self, title, message):
        self._warning = (title, message)

    def connect(self, widget, signal, callback):
        pass

    def emit(self, signal, args):
        pass


class TestFormAccounts(GuiOptionTest):

    def testLoadEmpty(self):
        form = FormAccounts(GuiOptionAccMock())
        self.assert_(form.w.save_account.checkState() == Qt.Unchecked)
        self.assert_(not form.w.delete_account.isEnabled())
        self.assert_(not form.w.change_prompt.isEnabled())
        self.assert_(form.w.list_conn_account.count() == 0)
        self.assert_(form.w.list_account.count() == 0)
        self.assert_(not form.w.normal_prompt.text())
        self.assert_(not form.w.fight_prompt.text())
        self.assert_(not form.w.box_prompt.isVisible())

    def testLoad1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAccounts(GuiOptionAccMock())
        self.assert_(form.w.list_conn_account.count() == 1)
        self.assert_(form.w.list_account.count() == 0)
        self.assert_(not form.w.delete_account.isEnabled())
        self.assert_(not form.w.normal_prompt.text())
        self.assert_(not form.w.fight_prompt.text())

    def testLoad2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        form = FormAccounts(GuiOptionAccMock())
        self.assert_(form.w.list_conn_account.count() == 1)
        self.assert_(form.w.list_account.count() == 1)
        self.assert_(form.w.delete_account.isEnabled())
        self.assert_(form.w.change_prompt.isEnabled())
        self.assert_(not form.w.normal_prompt.text())
        self.assert_(not form.w.fight_prompt.text())

    def testLoad3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.addConnection([0, 'name2', 'host2', 5000])
        storage.saveAccount(['john', 'john'], 1, 1)
        storage.saveAccount(['sarah', 'sarah'], 2, 1)
        form = FormAccounts(GuiOptionAccMock())
        self.assert_(str(form.w.list_conn_account.currentText()) == 'name')
        self.assert_(str(form.w.list_account.currentText()) == 'john')
        self.assert_(form.w.delete_account.isEnabled())

    def testLoad4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        storage.savePrompt(1, 'john', 'normal prompt', 'fight prompt')
        form = FormAccounts(GuiOptionAccMock())
        self.assert_(str(form.w.normal_prompt.text()) == 'normal prompt')
        self.assert_(str(form.w.fight_prompt.text()) == 'fight prompt')

    def testChangeSaveAccount(self):
        form = FormAccounts(GuiOptionAccMock())
        form.w.save_account.setCheckState(Qt.Checked)
        form._saveAccounts(Qt.Checked)
        self.assert_(storage.option('save_account', 0) == 1)

    def testChangeSaveAccount2(self):
        form = FormAccounts(GuiOptionAccMock())
        form.w.save_account.setCheckState(Qt.Checked)
        form._saveAccounts(Qt.Checked)
        form.w.save_account.setCheckState(Qt.Unchecked)
        form._saveAccounts(Qt.Unchecked)
        self.assert_(storage.option('save_account', 0) == 0)

    def testDeleteAccount(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        form = FormAccounts(GuiOptionAccMock())
        form.deleteAccount()
        self.assert_(not form.w.delete_account.isEnabled())
        self.assert_(not form.w.change_prompt.isEnabled())
        self.assert_(form.w.list_conn_account.count() == 1)
        self.assert_(form.w.list_account.count() == 0)

    def testDeleteAccount2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        storage.saveAccount(['sarah', 'sarah'], 1, 1)
        form = FormAccounts(GuiOptionAccMock())
        form.deleteAccount()
        self.assert_(form.w.delete_account.isEnabled())
        self.assert_(form.w.change_prompt.isEnabled())
        self.assert_(form.w.list_conn_account.count() == 1)
        self.assert_(form.w.list_account.count() == 1)
        self.assert_(str(form.w.list_account.currentText()) == 'sarah')

    def testSavePrompt(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        form = FormAccounts(GuiOptionAccMock())
        form.w.box_prompt.setVisible(True)
        form.w.normal_prompt.setText('Hp:%h/%H Mn:%m/%M Mv:%v/%V>')
        form.w.fight_prompt.setText('Hp:%h/%H Mn:%m/%M Mv:%v/%V Opp:%c>')
        form._savePrompt()
        normal, fight = storage.prompt(1, 'john')
        self.assert_(not form.w.box_prompt.isVisible())
        self.assert_(normal == 'Hp:%h/%H Mn:%m/%M Mv:%v/%V>')
        self.assert_(fight == 'Hp:%h/%H Mn:%m/%M Mv:%v/%V Opp:%c>')

    def testSavePrompt2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAccount(['john', 'john'], 1, 1)
        form = FormAccounts(GuiOptionAccMock())
        form.w.box_prompt.setVisible(True)
        form.w.normal_prompt.setText('normal prompt')
        form.w.fight_prompt.setText('fight prompt')
        form._savePrompt()
        self.assert_(form.w._warning)

if __name__ == '__main__':
    unittest.main()
