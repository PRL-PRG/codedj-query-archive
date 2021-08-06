#!/usr/bin/env python
# -*- coding: utf8 -*-
#
# Authors:
#     Lorenzo Gil Sánchez (lgs) <lgs@yaco.es>
#
# Script setupFNMTcert.py - Instalador del certificado raiz de la FNMT
#
# Copyright (C) 2005 Junta de Andalucía
#
# Autor/es (Author/s):
#
# - Lorenzo Gil Sánchez <lgsl@yaco.es>
#
# Este fichero es parte de E-Admin de Guadalinex 2006
#
# setupFNMTcert.py es software libre. Puede redistribuirlo y/o modificarlo
# bajo los términos de la Licencia Pública General de GNU según es
# publicada por la Free Software Foundation, bien de la versión 2 de dicha
# Licencia o bien (según su elección) de cualquier versión posterior.
#
# setupFNMTcert.py se distribuye con la esperanza de que sea útil,
# pero SIN NINGUNA GARANTÍA, incluso sin la garantía MERCANTIL
# implícita o sin garantizar la CONVENIENCIA PARA UN PROPÓSITO
# PARTICULAR. Véase la Licencia Pública General de GNU para más detalles.
# Debería haber recibido una copia de la Licencia Pública General
# junto con Detección de Hardware de Guadalinex 2005 . Si no ha sido así,
# escriba a la Free Software Foundation, Inc.,
# 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
# -------------------------------------------------------------------------
#
# This file is part of E-Admin de Guadalinex 2006
#
# setupFNMTcert.py is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# at your option) any later version.
#
# setupFNMTcert.py is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Foobar; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import optparse
import os
import shutil
import urllib2
import sys

ROOT_CERT_SERVER = 'www.cert.fnmt.es'
ROOT_CERT_PATH = '/content/pages_std/certificados/FNMTClase2CA.cer'

SYSTEM_CERTIFICATES_DIR = '/usr/share/ca-certificates/'
SYSTEM_CERTIFICATES_MASTER_FILE = '/etc/ssl/certs/ca-certificates.crt'

def is_root():
    return 0 == os.getegid()

def download_root_cert(where):
    fin = urllib2.urlopen('http://%s%s' % (ROOT_CERT_SERVER, ROOT_CERT_PATH))
    cert_name = os.path.basename(ROOT_CERT_PATH)
    path = os.path.join(where, cert_name)
    fout = file(path, 'w')
    fout.write(fin.read())
    return path

def der2pem(der):
    cmd = '/usr/bin/openssl x509 -inform DER -in %s > %s'
    root, ext = os.path.splitext(der)
    pem = root + '.pem'
    os.system(cmd % (der, pem))
    return pem

def install_certificate(cert):
    dest_dir = os.path.join(SYSTEM_CERTIFICATES_DIR, 'fnmt')
    if not os.path.exists(dest_dir):
        os.mkdir(dest_dir)

    cert_filename, ext = os.path.splitext(os.path.basename(cert))
    dest_file = os.path.join(dest_dir, cert_filename + '.crt')
    if os.path.exists(dest_file):
        sys.exit('It seems that the FNMT certificate is already installed in your system.\nInstalling it twice is a bad idea. Please uninstall it first.')

    shutil.copy(cert, dest_file)

    # make a symbolic link in /etc/ssl/certs
    os.symlink(dest_file, os.path.join('/etc/ssl/certs',
                                       cert_filename + '.pem'))

    # append it to the ca-certificates.crt
    f = file(SYSTEM_CERTIFICATES_MASTER_FILE, 'a')
    f.write(file(dest_file).read() + '\n')
    f.close()

def cleanup(files):
    for f in files:
        os.unlink(f)

def main():
    if not is_root():
        sys.exit('This program needs to run with admin privileges. Please uso sudo for running it')

    parser = optparse.OptionParser()
    parser.add_option('-v', '--verbose',
                      action='store_true', dest='verbose', default=False)
    parser.add_option('-d', '--der', dest='der',
                      help='DER root certificate to install')
    parser.add_option('-p', '--pem', dest='pem',
                      help='PEM root certificate to install')

    (options, args) = parser.parse_args()

    temp_files = []

    if options.der is None and options.pem is None:
        if options.verbose: print 'Downloading root certificate ...',
        cert = download_root_cert('/tmp')
        if options.verbose: print 'done'
        temp_files.append(cert)
        options.der = cert

    if options.der:
        if options.verbose: print 'Converting der certificate to pem format ...',
        pem = der2pem(options.der)
        if options.verbose: print 'done'
        temp_files.append(pem)
        options.pem = pem

    if options.verbose: print 'Installing the certificate ...',
    install_certificate(options.pem)
    if options.verbose: print 'done'

    if temp_files:
        if options.verbose: print 'Removing temporary files ...',
        cleanup(temp_files)
        if options.verbose: print 'done'

if __name__ == '__main__':
    main()
