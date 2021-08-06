#!/usr/bin/env python
#-*- coding: utf8 -*-

#Módulo certmanager- Módulo que localiza certificados digitales en dispositivos
# de volumen y que configura un conjunto de aplicaciones para que funcionen
# correctamente
#
#Copyright (C) 2005 Junta de Andalucía
#
#Autor/es (Author/s):
#
#- Lorenzo Gil Sanchez <lgs@yaco.es>
#
#Este fichero es parte del módulo E-Admin de Guadalinex 2006
#
#E-Admin de Guadalinex 2006 es software libre. Puede redistribuirlo y/o modificarlo
#bajo los términos de la Licencia Pública General de GNU según es
#publicada por la Free Software Foundation, bien de la versión 2 de dicha
#Licencia o bien (según su elección) de cualquier versión posterior.
#
#Detección de Hardware de Guadalinex 2005  se distribuye con la esperanza de que sea útil,
#pero SIN NINGUNA GARANTÍA, incluso sin la garantía MERCANTIL
#implícita o sin garantizar la CONVENIENCIA PARA UN PROPÓSITO
#PARTICULAR. Véase la Licencia Pública General de GNU para más detalles.
#
#Debería haber recibido una copia de la Licencia Pública General
#junto con Detección de Hardware de Guadalinex 2005 . Si no ha sido así, escriba a la Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
#
#-------------------------------------------------------------------------
#
#This file is part of E-Admin de Guadalinex 2006 .
#
#E-Admin de Guadalinex 2006  is free software; you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation; either version 2 of the License, or
#at your option) any later version.
#
#E-Admin Guadalinex 2006  is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.
#
#You should have received a copy of the GNU General Public License
#along with Foobar; if not, write to the Free Software
#Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import glob
import optparse
import os

import gtk

class Application(object):
    """Base abstract class Application that can use a certificate"""

    def __init__(self, name):
        self._name = name

    def setup(self, certificates):
        """This method should be overriden in subclasses"""

class MozillaApp(Application):
    """ TODO """

class OpenOfficeApp(Application):
    """ TODO """

class Evolution(Application):
    """ TODO """

class CertManager(object):
    """Search for certificates in a specific path and configure applications
    to use the certificates that the user selects
    """

    # We serch for certificate files with this extensions
    known_extensions = ('cert', 'p12')

    def __init__(self, search_path, applications=[]):

        self._search_path = search_path
        self._applications = applications

    def run(self):
        cert_list = self.search_certificates()

        if cert_list:
            selected_certs = self.select_certificates(cert_list)

            if selected_certs:

                for app in self._applications:
                    app.setup(selected_certs)

    def search_certificates(self):
        ret = []
        for extension in self.known_extensions:
            path = os.path.join(self._search_path, '*.%s' % extension)
            ret += glob.glob(path)

        return ret

    def select_certificates(self, cert_list):
        dialog = CertificatesDialog(self._search_path, cert_list)
        ret = []

        if gtk.RESPONSE_ACCEPT == dialog.run():
            ret = dialog.get_selected_certificates()
        dialog.destroy()

        return ret

class CertificatesDialog(gtk.Dialog):
    """Dialog to ask the user which certicicates he/she wishes to use"""
    def __init__(self, path, cert_list, parent=None):
        gtk.Dialog.__init__(self,
                            title="CertManager",
                            parent=parent,
                            flags=gtk.DIALOG_NO_SEPARATOR | gtk.DIALOG_MODAL,
                            buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_REJECT,
                                     gtk.STOCK_OK, gtk.RESPONSE_ACCEPT))
        self.set_border_width(12)

        hbox = gtk.HBox()
        image = gtk.image_new_from_stock(gtk.STOCK_DIALOG_INFO,
                                         gtk.ICON_SIZE_DIALOG)
        image.set_alignment(0.0, 0.0)

        image.show()
        hbox.pack_start(image, False, False)

        vbox = gtk.VBox(spacing=6)

        # Information label
        info = 'Se han encontrado los siguientes\n certificados en %s' % path
        markup = '<span size="large" weight="bold">%s</span>' % info
        label = gtk.Label(markup)
        label.set_use_markup(True)
        label.set_alignment(0.0, 0.5)
        label.show()
        vbox.pack_start(label, False, False)

        # List with certicates
        model = gtk.ListStore(bool, str)
        for cer in cert_list:
            model.append((True, cer))

        self.treeview = gtk.TreeView(model)
        self.treeview.set_headers_visible(False)
        self.treeview.set_rules_hint(True)
        self.treeview.get_selection().set_mode(gtk.SELECTION_NONE)

        toggle_renderer = gtk.CellRendererToggle()
        toggle_renderer.connect('toggled', self._on_cert__toggled)
        column1 = gtk.TreeViewColumn('', toggle_renderer, active=0)
        self.treeview.append_column(column1)

        text_renderer = gtk.CellRendererText()
        column2 = gtk.TreeViewColumn('', text_renderer, text=1)
        self.treeview.append_column(column2)

        self.treeview.show()

        scrolled_window = gtk.ScrolledWindow()
        scrolled_window.add(self.treeview)
        scrolled_window.set_policy(gtk.POLICY_AUTOMATIC, gtk.POLICY_AUTOMATIC)
        scrolled_window.set_shadow_type(gtk.SHADOW_IN)
        scrolled_window.show()
        vbox.pack_start(scrolled_window, True, True)

        # Request label
        request = 'Seleccione aquellos certificados que desee utilizar'
        label = gtk.Label(request)
        label.set_alignment(0.0, 0.5)
        label.show()
        vbox.pack_start(label, False, False)

        vbox.show()
        hbox.pack_start(vbox, True, True)

        hbox.show()
        self.vbox.pack_start(hbox, True, True)

    def _on_cert__toggled(self, renderer, path):
        model = self.treeview.get_model()
        tree_iter = model.get_iter(path)
        oldvalue = model.get_value(tree_iter, 0)
        model.set_value(tree_iter, 0, not oldvalue)

    def get_selected_certificates(self):
        model = self.treeview.get_model()
        ret = []
        for element in model:
            if element[0]:
                ret.append(element[1])
        return ret

if __name__ == '__main__':
    parser = optparse.OptionParser()
    parser.add_option('-p', '--search-path', dest='search_path',
                      default=os.path.expanduser('~'),
                      help='Where to search certificates')

    (options, args) = parser.parse_args()

    cert_manager = CertManager(options.search_path)
    cert_manager.run()
