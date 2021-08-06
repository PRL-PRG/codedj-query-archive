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

import gobject
import gtk

from ffsecutils import FireFoxSecurityUtils

DNIE_ROOT_CERT_NAME = "AC RAIZ DNIE - DIRECCION GENERAL DE LA POLICIA"
DNIE_ROOT_CERT_FILE = "/usr/share/opensc-dnie/ac_raiz_dnie.crt"
FNMT_ROOT_CERT_NAME = "FNMT"
FNMT_ROOT_CERT_FILE = "/usr/share/ca-certificates/fnmt/FNMTClase2CA.crt"
DNIE_PKCS11_LIB     = "/usr/lib/opensc-pkcs11.so"

class Application(object):
    """Base abstract class Application that can use a certificate"""

    def __init__(self, name):
        self._name = name

    def setup(self, certificates):
        """This method should be overriden in subclasses"""

class FireFoxApp(Application):

    def __init__(self, name='FireFox'):
        super(FireFoxApp, self).__init__(name)
        self._ff = FireFoxSecurityUtils()

    def setup(self, certificates):
        # check that we have the root certificates of relevant spanish agencies
        has_fnmt_cert = self._ff.has_root_ca_certificate(FNMT_ROOT_CERT_NAME)
        has_dnie_cert = self._ff.has_root_ca_certificate(DNIE_ROOT_CERT_NAME)

        if not has_fnmt_cert or not has_dnie_cert:
            # install the root certificates creating a default profile and
            # stopping Firefox if needed
            if self._ff.get_default_profile_dir() is None:
                self._ff.create_default_profile()

            if self._ff.is_firefox_running():
                abort = not self._wait_for_running_instances()
                if abort:
                    return

            if not has_fnmt_cert:
                self._ff.add_root_ca_certificate(FNMT_ROOT_CERT_NAME,
                                                 FNMT_ROOT_CERT_FILE)

            if not has_dnie_cert:
                self._ff.add_root_ca_certificate(DNIE_ROOT_CERT_NAME,
                                                 DNIE_ROOT_CERT_FILE)

        if self._ff.is_firefox_running():
            abort = not self._wait_for_running_instances()
            if abort:
                return

        if not self._ff.has_security_method('DNIe'):
            self._ff.add_security_method('DNIe', DNIE_PKCS11_LIB)

        # install the user certificates
        for cert in certificates:
            self._install_certificate(cert)

    def _wait_for_running_instances(self):
        dialog = gtk.MessageDialog(None, 0, gtk.MESSAGE_INFO,
                                   gtk.BUTTONS_NONE)
        next_btn = dialog.add_button(gtk.STOCK_GO_FORWARD, gtk.RESPONSE_ACCEPT)
        next_btn.set_sensitive(False)
        dialog.add_button(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL)
        dialog.set_title('Configurando %s' % self._name)
        dialog.set_markup('Debe cerrar todas las ventanas de %s para configurar los certificados digitales' % self._name)
        progress = gtk.ProgressBar()
        progress.set_text('Esperando a que finalice %s' % self._name)
        progress.set_pulse_step(0.1)
        progress.pulse()
        dialog.vbox.pack_start(progress, False, False)
        dialog.show_all()
        gobject.timeout_add(300, self._check_app_running, progress, next_btn)
        result = dialog.run()
        dialog.destroy()
        return result == gtk.RESPONSE_ACCEPT

    def _check_app_running(self, progress, next_btn):
        retval = self._ff.is_firefox_running()
        if retval:
            progress.pulse()
        else:
            progress.set_text('%s ha finalizado' % self._name)
            progress.set_fraction(1.0)
            next_btn.set_sensitive(True)

        return retval

    def _install_certificate(self, certificate):
        password = self._ask_for_password(certificate)
        if password:
            self._ff.add_user_certificate(certificate, password)

    def _ask_for_password(self, certificate):
        dialog = gtk.MessageDialog(None, 0, gtk.MESSAGE_INFO,
                                   gtk.BUTTONS_OK_CANCEL)
        dialog.set_title('Configurando %s' % self._name)
        dialog.set_markup('Introduzca la contraseña para desbloquear el certificado situado en el fichero <b>%s</b>' % certificate)
        entry = gtk.Entry()
        entry.set_activates_default(True)
        entry.set_visibility(False) # this entry is for passwords
        entry.show()
        parent = dialog.vbox.get_children()[0].get_children()[1]
        parent.pack_start(entry, False, False)
        result = dialog.run()
        retval = None
        if result == gtk.RESPONSE_OK:
            retval = entry.get_text()
        dialog.destroy()
        return retval

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

    cert_manager = CertManager(options.search_path, [FireFoxApp()])
    cert_manager.run()
