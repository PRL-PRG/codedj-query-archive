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

import sys
import os.path
import unittest

from PyQt4 import QtCore, QtGui

sys.path.append('..')

from devclient.conf import config
from devclient.storage import Storage
from devclient.gui_option import *


class GuiOptionTest(unittest.TestCase):

    def __init__(self, methodName='runTest'):
        super(GuiOptionTest, self).__init__(methodName)
        if not QtGui.QApplication.instance():
            self.app = QtGui.QApplication([])

    def setUp(self):
        abspath = os.path.abspath('../../data/storage/dbtest.sqlite')
        config['storage'] = {'path': abspath}

        if os.path.exists(config['storage']['path']):
            os.unlink(config['storage']['path'])

    def tearDown(self):
        if os.path.exists(config['storage']['path']):
            os.unlink(config['storage']['path'])


class GuiOptionMock(object):

    def __init__(self):
        self.name_conn = QtGui.QLineEdit()
        self.host_conn = QtGui.QLineEdit()
        self.port_conn = QtGui.QLineEdit()
        self.list_conn = QtGui.QComboBox()
        self.default_conn = QtGui.QCheckBox()
        self.save_conn = QtGui.QPushButton()
        self.delete_conn = QtGui.QPushButton()
        self.connect_conn = QtGui.QPushButton()
        self.list_conn.addItem("Create New")
        self._warning = None

    def connect(self, widget, signal, callback):
        pass

    def _displayWarning(self, title, message):
        self._warning = (title, message)


class TestFormConnection(GuiOptionTest):

    def _formCompare(self, form_conn, conn):
        check = (QtCore.Qt.Unchecked, QtCore.Qt.Checked)[conn[4] != 0]

        if form_conn.w.name_conn.text() != conn[1] or \
           form_conn.w.host_conn.text() != conn[2] or  \
           form_conn.w.port_conn.text() != str(conn[3]) or \
           form_conn.w.default_conn.checkState() != check:
            return False
        return True

    def _setFormFields(self, form_conn, conn):
        form_conn.w.name_conn.setText(conn[0])
        form_conn.w.host_conn.setText(conn[1])
        form_conn.w.port_conn.setText(str(conn[2]))
        d = (QtCore.Qt.Unchecked, QtCore.Qt.Checked)[conn[-1]]
        form_conn.w.default_conn.setCheckState(d)

    def _checkEmptyForm(self, form_conn):
        if not form_conn.w.name_conn.text() and \
           not form_conn.w.host_conn.text() and \
           not form_conn.w.port_conn.text() and \
           form_conn.w.default_conn.checkState() == QtCore.Qt.Unchecked and \
           not form_conn.w.list_conn.currentIndex():
            return True
        return False

    def testEmpty(self):
        form_conn = FormConnection(GuiOptionMock(), Storage())
        self.assert_(len(form_conn.connections) == 0)

    def testLoad(self):
        id_conn = 0
        port = 6000
        default = 0
        name, host = "name", "host"
        conn = [id_conn, name, host, port, default]

        Storage().addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad2(self):
        id_conn = 0
        port = 6000
        default = 0
        name, host = "name", "host"
        conn = [id_conn, name, host, port, default]

        storage = Storage()
        storage.addConnection(conn)
        storage.addConnection([0, 'test', 'test', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())

        form_conn.load(name)
        self.assert_(self._formCompare(form_conn, conn))

    def testLoad3(self):
        form_conn = FormConnection(GuiOptionMock(), Storage())

        form_conn.load('fake')
        self.assert_(self._formCompare(form_conn, [0, '', '', '', 0]))

    def testLoad4(self):
        form_conn = FormConnection(GuiOptionMock(), Storage())

        form_conn.load('')
        self.assert_(self._formCompare(form_conn, [0, '', '', '', 0]))

    def testCheckField1(self):
        """Verify error with empty fields."""

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField2(self):
        """
        Verify error on adding a connection with a name that already exists.
        """

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        self._setFormFields(form_conn, ['name', 'host3', 1000, 0])
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField3(self):
        """Verify error with empty name field """

        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.host_conn.setText('host')
        form_conn.w.port_conn.setText('1232')
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField4(self):
        """Verify error with empty host field """

        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.name_conn.setText('name')
        form_conn.w.port_conn.setText('1232')
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField5(self):
        """Verify error with empty port field """

        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.name_conn.setText('name')
        form_conn.w.host_conn.setText('host')
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testCheckField6(self):
        """Verify no error on right update"""

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.load('name')
        form_conn.w.list_conn.setCurrentIndex(1)
        self.assert_(form_conn._checkFields())
        self.assert_(not form_conn.w._warning)

    def testCheckField7(self):
        """Verify no error on right add"""

        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.name_conn.setText('name')
        form_conn.w.host_conn.setText('host')
        form_conn.w.port_conn.setText('1232')
        self.assert_(form_conn._checkFields())
        self.assert_(not form_conn.w._warning)

    def testCheckField8(self):
        """
        Verify error on update that change name with another that already
        exists.
        """

        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 0])
        conn = [0, 'name2', 'host2', 3000, 0]
        s.addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.list_conn.setCurrentIndex(2)
        self._setFormFields(form_conn, ['name', 'host3', 1000, 1])
        self.assert_(not form_conn._checkFields())
        self.assert_(form_conn.w._warning)

    def testSaveAdd(self):
        """Add a connection on empty storage."""

        form_conn = FormConnection(GuiOptionMock(), Storage())
        conn = ('name', 'host', 1232, 0)
        self._setFormFields(form_conn, conn)
        form_conn.save()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(conn == Storage().connections()[0][1:])

    def testSaveAdd2(self):
        """Add a connection on a storage with one connection."""

        Storage().addConnection([0, 'name', 'host', 4000, 0])
        conn = ('name2', 'host', 1232, 1)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        self._setFormFields(form_conn, conn)
        form_conn.save()
        self.assert_(len(form_conn.connections) == 2)
        self.assert_(conn == Storage().connections()[1][1:])

    def testSaveAdd3(self):
        """
        Verify that adding a connection with default field on set other
        connections default off.
        """

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        self._setFormFields(form_conn, ('name2', 'host', 1232, 1))
        form_conn.save()
        self.assert_(Storage().connections()[0][-1] == 0)

    def testSaveAdd4(self):
        """Verify that after saving form fields are empty."""

        form_conn = FormConnection(GuiOptionMock(), Storage())
        self._setFormFields(form_conn, ('name', 'host', '4000', 0))
        form_conn.save()
        self.assert_(self._checkEmptyForm(form_conn))

    def testSaveUpd(self):
        """Update a connection."""

        conn = [0, 'name', 'host', 4000, 1]
        Storage().addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        conn = [conn[0], 'name2', 'host2', 1234, 0]
        form_conn.w.list_conn.setCurrentIndex(1)
        self._setFormFields(form_conn, conn[1:])
        form_conn.save()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(conn == list(Storage().connections()[0]))

    def testSaveUpd2(self):
        """Verify that update on a connection change the in combo box."""

        conn = [0, 'name', 'host', 4000, 1]
        Storage().addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        conn = [conn[0], 'name2', 'host2', 1234, 0]
        form_conn.w.list_conn.setCurrentIndex(1)
        self._setFormFields(form_conn, conn[1:])
        form_conn.save()
        self.assert_(form_conn.w.list_conn.itemText(1) == conn[1])

    def testSaveUpd3(self):
        """Verify that after saving form fields are empty."""

        conn = [0, 'name', 'host', 4000, 1]
        Storage().addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        conn = [conn[0], 'name2', 'host2', 1234, 0]
        form_conn.w.list_conn.setCurrentIndex(1)
        self._setFormFields(form_conn, conn[1:])
        form_conn.save()
        self.assert_(self._checkEmptyForm(form_conn))

    def testSaveUpd4(self):
        """
        Verify that update a connection setting its default field on set other
        connections default off.
        """

        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.addConnection([0, 'name2', 'host2', 3000, 0])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.list_conn.setCurrentIndex(2)
        self._setFormFields(form_conn, ['name2', 'host2', 3000, 1])
        form_conn.save()
        c = Storage().connections()
        self.assert_(c[0][-1] == 0 and c[1][-1] == 1)

    def testDelete(self):
        """Delete 'create new' item."""

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)

    def testDelete2(self):
        """Delete a connection."""

        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 0)

    def testDelete3(self):
        """Delete a connection on a storage of two connections."""

        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        conn = [0, 'name2', 'host2', 3000, 0]
        s.addConnection(conn)
        form_conn = FormConnection(GuiOptionMock(), Storage())
        form_conn.w.list_conn.setCurrentIndex(1)
        form_conn.delete()
        self.assert_(len(form_conn.connections) == 1)
        self.assert_(len(Storage().connections()) == 1)
        self.assert_(form_conn.connections[0][1:] == tuple(conn[1:]))


class GuiOptionMacroMock(object):

    def __init__(self):
        self.register_macro = QtGui.QPushButton()
        self.save_macro = QtGui.QPushButton()
        self.delete_macro = QtGui.QPushButton()
        self.list_macro = QtGui.QComboBox()
        self.list_conn_macro = QtGui.QComboBox()
        self.command_macro = QtGui.QLineEdit()
        self.keys_macro = QtGui.QLineEdit()
        self._warning = None

    def connect(self, widget, signal, callback):
        pass

    def disconnect(self, widget, signal, callback):
        pass

    def _displayWarning(self, title, message):
        self._warning = (title, message)


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
        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        self.assert_(form_macro.w.list_macro.count() == 1)

    def testLoadEmpty2(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')

        form_macro.load(0)
        self.assert_(not form_macro.w.command_macro.text() and
                     not form_macro.w.keys_macro.text())

    def testLoad(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        macro = ('command', 1, 0, 0, 65)
        s.saveMacros('name', [macro])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')

        form_macro.load(1)
        self.assert_(self._formCompare(form_macro, macro))

    def testLoad2(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        macro = ('command', 0, 1, 1, 72)
        s.saveMacros('name', [('command', 1, 0, 0, 65), macro])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')

    def testCheckField1(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        form_macro.load(1)
        self.assert_(not form_macro._checkFields())

    def testCheckField2(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        form_macro.load(1)
        form_macro.w.list_macro.setCurrentIndex(1)
        self.assert_(form_macro._checkFields())

    def testCheckField3(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        macro = ('command', 1, 0, 1, 77)
        s.saveMacros('name', [('command', 1, 0, 0, 65), macro])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        form_macro.load(1)
        form_macro.w.list_macro.setCurrentIndex(1)
        form_macro.w.keys_macro.setText(form_macro.getKeyDescr(*macro[1:]))
        form_macro.key_seq = macro[1:]
        self.assert_(not form_macro._checkFields())

    def testCheckField4(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        self._setFormFields(form_macro, ('command', 0, 0, 0, 78))
        form_macro.save()
        self._setFormFields(form_macro, ('check', 0, 0, 0, 78))
        self.assert_(not form_macro._checkFields())

    def testSaveAdd(self):
        Storage().addConnection([0, 'name', 'host', 4000, 1])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        macro = ('command', 1, 0, 0, 65)
        self._setFormFields(form_macro, macro)
        form_macro.save()

        self.assert_(not form_macro.w._warning)
        self.assert_(Storage().macros('name')[0] == macro)
        self.assert_(len(form_macro.macros) == 1)

    def testSaveAdd2(self):
        s = Storage()
        s.addConnection([0, 'name', 'host', 4000, 1])
        s.saveMacros('name', [('command', 1, 0, 0, 65)])
        form_macro = FormMacro(GuiOptionMacroMock(), Storage(), 'name')
        macro = ('command', 0, 0, 0, 78)
        self._setFormFields(form_macro, macro)
        form_macro.save()

        self.assert_(not form_macro.w._warning)
        self.assert_(Storage().macros('name')[1] == macro)
        self.assert_(len(form_macro.macros) == 2)




if __name__ == '__main__':
    unittest.main()
