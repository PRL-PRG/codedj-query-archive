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
from PyQt4.QtGui import QApplication, QLineEdit, QGroupBox, QLabel, QWidget
from PyQt4.QtGui import QPushButton, QComboBox, QCheckBox, QRadioButton

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
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        config['devclient'] = {'path': os.path.abspath('../devclient')}

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        devclient.storage.loadStorage()

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)


class GuiOptionMock(object):

    def __init__(self):
        self._warning = None
        self._question = None
        self._text = {}
        self._lazy_conn = ''
        execfile(os.path.join(config['devclient']['path'], 'gui_option.msg'),
                 self._text)

    def connect(self, *args):
        pass

    def disconnect(self, *args):
        pass

    def _displayWarning(self, title, message):
        self._warning = (title, message)

    def _displayQuestion(self, title, message):
        self._question = (title, message)

    def emit(self, *args):
        pass


class GuiOptionConnMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.name_conn = QLineEdit()
        self.host_conn = QLineEdit()
        self.port_conn = QLineEdit()
        self.list_conn = QComboBox()
        self.save_conn = QPushButton()
        self.delete_conn = QPushButton()
        self.connect_conn = QPushButton()
        self.list_conn.addItem("Create New")


class TestFormConnection(GuiOptionTest):

    def _formCompare(self, form_conn, conn):

        if form_conn.w.name_conn.text() != conn[1] or \
           form_conn.w.host_conn.text() != conn[2] or  \
           form_conn.w.port_conn.text() != str(conn[3]):
            return False
        return True

    def _buildForm(self, name, host, port):
        form_conn = FormConnection(GuiOptionConnMock())
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
        form_conn = FormConnection(GuiOptionConnMock())
        self.assert_(len(form_conn.connections) == 0)

    def testLoad(self):
        id_conn = 0
        port = 6000
        name, host = "name", "host"
        conn = [id_conn, name, host, port]

        storage.addConnection(conn)
        form_conn = FormConnection(GuiOptionConnMock())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad2(self):
        id_conn = 0
        port = 6000
        name, host = "name", "host"
        conn = [id_conn, name, host, port]

        storage.addConnection(conn)
        storage.addConnection([0, 'test', 'test', 4000])
        form_conn = FormConnection(GuiOptionConnMock())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad3(self):
        form_conn = FormConnection(GuiOptionConnMock())

        form_conn.load('fake')
        self.assert_(self._formCompare(form_conn, [0, '', '', '']))

    def testLoad4(self):
        form_conn = FormConnection(GuiOptionConnMock())

        form_conn.load('')
        self.assert_(self._formCompare(form_conn, [0, '', '', '']))

    def testCheckField1(self):
        """Verify error with empty fields."""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionConnMock())
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
        form_conn = FormConnection(GuiOptionConnMock())
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
        form_conn = FormConnection(GuiOptionConnMock())
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)

    def testDelete2(self):
        """Delete a connection."""

        storage.addConnection([0, 'name', 'host', 4000])
        form_conn = FormConnection(GuiOptionConnMock())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 0)

    def testDelete3(self):
        """Delete a connection on a storage of two connections."""

        storage.addConnection([0, 'name', 'host', 4000])
        conn = [0, 'name2', 'host2', 3000]
        storage.addConnection(conn)
        form_conn = FormConnection(GuiOptionConnMock())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(len(storage.connections()) == 1)
        self.assert_(form_conn.connections[0][1:] == tuple(conn[1:]))

    def testDelete4(self):
        """Show a question if the connection has a child"""

        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form_conn = FormConnection(GuiOptionConnMock())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(form_conn.w._question)


class GuiOptionMacroMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.register_macro = QPushButton()
        self.save_macro = QPushButton()
        self.delete_macro = QPushButton()
        self.list_macro = QComboBox()
        self.list_conn_macro = QComboBox()
        self.command_macro = QLineEdit()
        self.keys_macro = QLineEdit()


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


class GuiOptionPrefMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.echo_color = QLabel()
        self.save_preferences = QPushButton()
        self.echo_color_button = QPushButton()
        self.cmd_separator = QLineEdit()
        self.keep_text = QCheckBox()
        self.save_log = QCheckBox()


class TestFormPreferences(GuiOptionTest):

    def testLoadEmpty(self):
        form = FormPreferences(GuiOptionPrefMock())
        self.assert_(form._echo_color == '')
        self.assert_(form.w.keep_text.checkState() == Qt.Unchecked)
        self.assert_(form.w.save_log.checkState() == Qt.Unchecked)
        self.assert_(form.w.cmd_separator.text() == ';')

    def testSavePreferences(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.keep_text.setCheckState(Qt.Checked)
        form._echo_color = '#00AA00'
        form.save()
        self.assert_(storage.preferences() == ('#00AA00', 1, 0, ';'))

    def testSavePreferences2(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.keep_text.setCheckState(Qt.Unchecked)
        form._echo_color ='#CC0000'
        form.w.save_log.setCheckState(Qt.Checked)
        form.w.cmd_separator.setText(':')
        form.save()
        self.assert_(storage.preferences() == ('#CC0000', 0, 1, ':'))

    def testSavePreferences4(self):
        form = FormPreferences(GuiOptionPrefMock())
        form.w.keep_text.setCheckState(Qt.Checked)
        form.w.save_log.setCheckState(Qt.Unchecked)
        form.w.cmd_separator.setText(',')
        form._echo_color ='#CC0000'
        form.save()
        form.w.keep_text.setCheckState(Qt.Unchecked)
        form.w.save_log.setCheckState(Qt.Checked)
        form.w.cmd_separator.setText('#')
        form._echo_color ='#000000'
        form.save()
        self.assert_(storage.preferences() == ('#000000', 0, 1, '#'))


class GuiOptionAccMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.save_account = QCheckBox()
        self.delete_account = QPushButton()
        self.list_account = QComboBox()
        self.list_conn_account = QComboBox()
        self.change_prompt = QPushButton()
        self.save_prompt = QPushButton()
        self.normal_prompt = QLineEdit()
        self.fight_prompt = QLineEdit()
        self.box_prompt = QGroupBox()
        self.delete_account.setEnabled(False)
        self.change_prompt.setEnabled(False)


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


class GuiOptionAliasMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.list_alias = QComboBox()
        self.list_conn_alias = QComboBox()
        self.label_alias = QLineEdit()
        self.body_alias = QLineEdit()
        self.delete_alias = QPushButton()
        self.save_alias = QPushButton()


class TestFormAliases(GuiOptionTest):

    def testLoadEmpty(self):
        form = FormAliases(GuiOptionAliasMock())
        self.assert_(not form.w.list_conn_alias.count())
        self.assert_(not form.w.list_alias.count())
        self.assert_(not form.w.label_alias.text())
        self.assert_(not form.w.body_alias.text())

    def testLoad1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        self.assert_(form.w.list_conn_alias.count() == 1)
        self.assert_(form.w.list_alias.count() == 1)
        self.assert_(not form.w.label_alias.text())
        self.assert_(not form.w.body_alias.text())

    def testLoad2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        self.assert_(form.w.list_alias.count() == 2)
        self.assert_(not form.w.label_alias.text())
        self.assert_(not form.w.body_alias.text())

    def testLoad3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(1)
        form._loadAlias(1)
        self.assert_(form.w.list_alias.count() == 2)
        self.assert_(form.w.label_alias.text() == 'label')
        self.assert_(form.w.body_alias.text() == 'body')

    def testLoad4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body'),
                                     ('label2', 'body2'),
                                     ('label3', 'body3')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(2)
        form._loadAlias(2)
        self.assert_(form.w.list_alias.count() == 4)
        self.assert_(form.w.label_alias.text() == 'label2')
        self.assert_(form.w.body_alias.text() == 'body2')

    def testDelete1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        form._loadAlias(0)
        form._deleteAlias()
        self.assert_(form.w.list_alias.count() == 2)

    def testDelete2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(1)
        form._loadAlias(1)
        form._deleteAlias()
        self.assert_(form.w.list_alias.count() == 1)

    def testDelete3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body'), ('label2', 'body2')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(1)
        form._loadAlias(1)
        form._deleteAlias()
        # Delete an element of combobox change the current index of it and emit
        # a currentIndexChanged signal.
        form._loadAlias(1)
        self.assert_(form.w.list_alias.count() == 2)
        self.assert_(form.w.label_alias.text() == 'label2')
        self.assert_(form.w.body_alias.text() == 'body2')

    def testDelete4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body'), ('label2', 'body2')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(2)
        form._loadAlias(2)
        form._deleteAlias()
        # Delete an element of combobox change the current index of it and emit
        # a currentIndexChanged signal.
        form._loadAlias(1)
        self.assert_(form.w.list_alias.count() == 2)
        self.assert_(form.w.label_alias.text() == 'label')
        self.assert_(form.w.body_alias.text() == 'body')

    def testCheckAliasFields1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        self.assert_(not form._checkAliasFields())
        self.assert_(form.w._warning)

    def testCheckAliasFields2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        form.w.label_alias.setText('label')
        self.assert_(not form._checkAliasFields())
        self.assert_(form.w._warning)

    def testCheckAliasFields3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        form.w.body_alias.setText('body')
        self.assert_(not form._checkAliasFields())
        self.assert_(form.w._warning)

    def testCheckAliasFields4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        form.w.label_alias.setText('label')
        form.w.body_alias.setText('body')
        self.assert_(form._checkAliasFields())
        self.assert_(not form.w._warning)

    def testSave1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormAliases(GuiOptionAliasMock())
        form.w.label_alias.setText('label')
        form.w.body_alias.setText('body')
        form._saveAlias()
        self.assert_(storage.aliases('name') == [('label', 'body')])

    def testSave2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.label_alias.setText('label')
        form.w.body_alias.setText('body')
        form._saveAlias()
        self.assert_(form.w._warning)
        self.assert_(storage.aliases('name') == [('label', 'body')])

    def testSave3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveAliases('name', [('label', 'body')])
        form = FormAliases(GuiOptionAliasMock())
        form.w.list_alias.setCurrentIndex(1)
        form._loadAlias(1)
        form.w.body_alias.setText('body2')
        form._saveAlias()
        self.assert_(not form.w._warning)
        self.assert_(storage.aliases('name') == [('label', 'body2')])


class GuiOptionTriggerMock(GuiOptionMock):

    def __init__(self):
        GuiOptionMock.__init__(self)
        self.w = QWidget()
        self.list_trigger = QComboBox()
        self.list_conn_trigger = QComboBox()
        self.pattern_trigger = QLineEdit()
        self.command_trigger = QLineEdit()
        self.case_trigger = QCheckBox()
        self.delete_trigger = QPushButton()
        self.save_trigger = QPushButton()
        self.radio_command_trigger = QRadioButton(self.w)
        self.radio_command_trigger.setChecked(True)
        self.radio_color_trigger = QRadioButton(self.w)
        self.text_color_trigger_button = QPushButton()
        self.bg_color_trigger_button = QPushButton()
        self.text_color_trigger = QLabel()
        self.bg_color_trigger = QLabel()


class TestFormTriggers(GuiOptionTest):

    def testLoadEmpty(self):
        form = FormTriggers(GuiOptionTriggerMock())
        self.assert_(not form.w.list_conn_trigger.count())
        self.assert_(not form.w.list_trigger.count())
        self.assert_(not form.w.pattern_trigger.text())
        self.assert_(not form.w.command_trigger.text())
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)

    def testLoad1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        self.assert_(form.w.list_conn_trigger.count() == 1)
        self.assert_(form.w.list_trigger.count() == 1)
        self.assert_(not form.w.pattern_trigger.text())
        self.assert_(not form.w.command_trigger.text())
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(form.w.radio_command_trigger.isChecked())
        self.assert_(not form._text_color)
        self.assert_(not form._bg_color)

    def testLoad2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        self.assert_(form.w.list_trigger.count() == 2)
        self.assert_(not form.w.pattern_trigger.text())
        self.assert_(not form.w.command_trigger.text())
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(form.w.radio_command_trigger.isChecked())

    def testLoad3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(form.w.command_trigger.text() == 'bow dwarf')
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(form.w.radio_command_trigger.isChecked())

    def testLoad4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, '', '#FFFF00', '#000000')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(not form.w.command_trigger.text())
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(form.w.radio_color_trigger.isChecked())
        self.assert_(form._text_color == '#000000')
        self.assert_(form._bg_color == '#FFFF00')

    def testLoad5(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('fake', 0, 'fake', '', ''),
                                      ('* dwarf *', 1, 'bow dwarf', '', ''),
                                      ('fake2', 1, 'fake2', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(2)
        form._loadTrigger(2)
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(form.w.command_trigger.text() == 'bow dwarf')
        self.assert_(form.w.case_trigger.checkState() == Qt.Checked)

    def testLoad6(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        form.w.radio_color_trigger.setChecked(True)
        form._toggleChoice()
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(not form.w.command_trigger.text())
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(not form.w.radio_command_trigger.isChecked())

    def testLoad7(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, '', '#FF0000', '#000000')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        form.w.radio_command_trigger.setChecked(True)
        form._toggleChoice()
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(not form._text_color)
        self.assert_(not form._bg_color)
        self.assert_(form.w.case_trigger.checkState() == Qt.Unchecked)
        self.assert_(form.w.radio_command_trigger.isChecked())

    def testLoad8(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('fake', 0, 'fake', '', ''),
                                      ('* dwarf *', 1, '', '#FF0000', '#000000'),
                                      ('fake2', 1, 'fake2', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        self.assert_(form.w.radio_command_trigger.isChecked())
        form.w.list_trigger.setCurrentIndex(2)
        form._loadTrigger(2)
        self.assert_(form.w.pattern_trigger.text() == '* dwarf *')
        self.assert_(form.w.radio_color_trigger.isChecked())
        self.assert_(form._text_color == '#000000')
        self.assert_(form._bg_color == '#FF0000')

    def testLoad9(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('fake', 0, 'fakecmd', '', ''),
                                      ('* dwarf *', 1, '', '#FF0000', '#000000'),
                                      ('fake2', 1, 'fake2', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(2)
        form._loadTrigger(2)
        self.assert_(form.w.radio_color_trigger.isChecked())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        self.assert_(form.w.pattern_trigger.text() == 'fake')
        self.assert_(form.w.command_trigger.text() == 'fakecmd')
        self.assert_(form.w.radio_command_trigger.isChecked())

    def testDelete1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form._loadTrigger(0)
        form._deleteTrigger()
        self.assert_(form.w.list_trigger.count() == 2)

    def testDelete2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        form._deleteTrigger()
        self.assert_(form.w.list_trigger.count() == 1)

    def testDelete3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('* dwarf *', 0, 'bow dwarf', '', ''),
                                      ('fake', 0, 'fake', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        self.assert_(form.w.list_trigger.count() == 3)
        form._deleteTrigger()
        self.assert_(form.w.list_trigger.count() == 2)
        self.assert_(form.triggers == [('fake', 0, 'fake', '', '')])

    def testCheckTriggerFields1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        self.assert_(not form._checkTriggerFields())
        self.assert_(form.w._warning)

    def testCheckTriggerFields2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        self.assert_(not form._checkTriggerFields())
        self.assert_(form.w._warning)

    def testCheckTriggerFields3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.command_trigger.setText('command')
        self.assert_(not form._checkTriggerFields())
        self.assert_(form.w._warning)

    def testCheckTriggerFields4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        form.w.command_trigger.setText('command')
        self.assert_(form._checkTriggerFields())
        self.assert_(not form.w._warning)

    def testCheckTriggerFields5(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        form.w.radio_color_trigger.setChecked(True)
        self.assert_(not form._checkTriggerFields())
        self.assert_(form.w._warning)

    def testCheckTriggerFields6(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        form.w.radio_color_trigger.setChecked(True)
        form._text_color = '#FF0000'
        self.assert_(form._checkTriggerFields())
        self.assert_(not form.w._warning)

    def testSave1(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        form.w.command_trigger.setText('command')
        form._saveTrigger()
        self.assert_([('pattern', 0, 'command', '', '')] ==
                     storage.triggers('name'))

    def testSave2(self):
        storage.addConnection([0, 'name', 'host', 4000])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('pattern')
        form.w.command_trigger.setText('command')
        form.w.case_trigger.setCheckState(Qt.Checked)
        form._saveTrigger()
        self.assert_([('pattern', 1, 'command', '', '')] ==
                     storage.triggers('name'))

    def testSave3(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('pattern', 0, 'command', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.pattern_trigger.setText('PaTTeRn')
        form.w.command_trigger.setText('command')
        form._saveTrigger()
        self.assert_(form.w._warning)
        self.assert_(storage.triggers('name') == [('pattern', 0, 'command', '', '')])

    def testSave4(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('pattern', 0, 'command', '', '')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        form.w.pattern_trigger.setText('pattern2')
        form.w.command_trigger.setText('command2')
        form.w.case_trigger.setCheckState(Qt.Checked)
        form._saveTrigger()
        self.assert_(not form.w._warning)
        self.assert_([('pattern2', 1, 'command2', '', '')] ==
                     storage.triggers('name'))

    def testSave5(self):
        storage.addConnection([0, 'name', 'host', 4000])
        storage.saveTriggers('name', [('pattern', 0, '', '#C0C0C0', '#AAAAAA')])
        form = FormTriggers(GuiOptionTriggerMock())
        form.w.list_trigger.setCurrentIndex(1)
        form._loadTrigger(1)
        form.w.pattern_trigger.setText('pattern2')
        form.w.case_trigger.setCheckState(Qt.Checked)
        form.w.radio_color_trigger.setChecked(True)
        form._text_color = '#FF0000'
        form._bg_color = '#000000'
        form._saveTrigger()
        self.assert_(not form.w._warning)
        self.assert_([('pattern2', 1, '', '#000000', '#FF0000')] ==
                     storage.triggers('name'))

if __name__ == '__main__':
    unittest.main()
