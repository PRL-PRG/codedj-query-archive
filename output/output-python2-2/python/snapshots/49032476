#!/usr/bin/python
# -*- coding: utf-8 -*- 

# Authors: 
#     Gumersindo Coronel Pérez (gcoronel) <gcoronel@emergya.es> 
#
# Last modified: 
#     $Date$ 
#     $Author$

# Módulo hermes_hardware - Notificador de cambios en el hardware
# 
# Copyright (C) 2005 Junta de Andalucía
# 
# Autor/es (Author/s):
# 
# - Gumersindo Coronel Pérez <gcoronel@emergya.info>
# 
# Este fichero es parte de Detección de Hardware de Guadalinex 2005 
# 
# Detección de Hardware de Guadalinex 2005  es software libre. Puede redistribuirlo y/o modificarlo 
# bajo los términos de la Licencia Pública General de GNU según es 
# publicada por la Free Software Foundation, bien de la versión 2 de dicha
# Licencia o bien (según su elección) de cualquier versión posterior. 
# 
# Detección de Hardware de Guadalinex 2005  se distribuye con la esperanza de que sea útil, 
# pero SIN NINGUNA GARANTÍA, incluso sin la garantía MERCANTIL 
# implícita o sin garantizar la CONVENIENCIA PARA UN PROPÓSITO 
# PARTICULAR. Véase la Licencia Pública General de GNU para más detalles. 
# 
# Debería haber recibido una copia de la Licencia Pública General 
# junto con Detección de Hardware de Guadalinex 2005 . Si no ha sido así, escriba a la Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
# 
# -------------------------------------------------------------------------
# 
# This file is part of Detección de Hardware de Guadalinex 2005 .
# 
# Detección de Hardware de Guadalinex 2005  is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# at your option) any later version.
# 
# Detección de Hardware de Guadalinex 2005  is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with Foobar; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import dbus
if getattr(dbus, "version", (0, 0, 0)) >= (0, 41, 0):
    import dbus.glib
import logging
import gtk
import os
import os.path
import sys
import traceback
import defs

from gettext import gettext as _

# Internacionalización
import gettext, locale

gettext.bindtextdomain('hermes-hardware', os.path.abspath(\
        os.path.join(defs.DATA_DIR, "locale")))
if hasattr(gettext, 'bind_textdomain_codeset'):
        gettext.bind_textdomain_codeset('hermes-hardware','UTF-8')
gettext.textdomain('hermes-hardware')

locale.bindtextdomain('hermes-harware', os.path.abspath(\
        os.path.join(defs.DATA_DIR, "locale")))
if hasattr(locale, 'bind_textdomain_codeset'):
        locale.bind_textdomain_codeset('hermes-hardware','UTF-8')
locale.textdomain('hermes-hardware')


from utils import DeviceList, ColdPlugListener, CaptureLogGui
from optparse import OptionParser
from utils.notification import NotificationDaemon, FileNotification


# notification-daemon spec: -------------------------------------------
# http://www.galago-project.org/specs/notification/0.9/x408.html#command-notify
# UINT32 org.freedesktop.Notifications.Notify 
#   (STRING app_name, 
#   UINT32 replaces_id, 
#   STRING app_icon, 
#   STRING summary, 
#   STRING body, 
#   ARRAY actions, 
#   DICT hints, 
#   INT32 expire_timeout);

# self.iface.Notify("Hermes", #app_name 
#        0, # replaces_id
#        '', # app_icon
#        '', # summary
#        message, # body
#        '', # actions
#        '', # hints
#        0  #expire_timeout
#        )





class DeviceListener:
    
    def __init__(self, message_render):
        self.message_render = message_render
        self.logger = logging.getLogger()

        # Inicialize
        self.bus = dbus.SystemBus()

        obj = self.bus.get_object('org.freedesktop.Hal',
                                  '/org/freedesktop/Hal/Manager')

        self.hal_manager = dbus.Interface(obj, 'org.freedesktop.Hal.Manager')

        self.hal_manager.connect_to_signal('DeviceAdded', self.on_device_added)
        self.hal_manager.connect_to_signal('DeviceRemoved', 
                self.on_device_removed)

        self.udi_dict = {}
        self.modify_handler_dict = {}
        self.devicelist = DeviceList()

        self.__init_actors()

        coldplug = ColdPlugListener(self)
        coldplug.start()

        self.logger.info("DeviceListener iniciado")


    def on_device_added(self, udi, *args):
        self.logger.debug("DeviceAdded: " + str(udi))
        self.devicelist.save()

        obj = self.bus.get_object('org.freedesktop.Hal', udi)
        obj = dbus.Interface(obj, 'org.freedesktop.Hal.Device')

        properties = obj.GetAllProperties()
        self.__print_properties(properties)

        actor = self.add_actor_from_properties(properties)

        if actor: 
            try:
                actor.on_added()
            except:
                self.logger.warning(str(traceback.format_exc()))

            #from actors.deviceactor import DeviceActor
            #if actor.__class__ == DeviceActor:
            #    if properties.has_key('info.product') and \
            #            properties['info.product'] != '':
            #        product = properties['info.product']
            #        self.message_render.show_info(_("Information"),
            #            _("Device CONNECTED:") + "\:n %s" % (product,))
            #    else:
            #        self.message_render.show_warning(_("Warning"), 
            #                _("Device detected, but unidentificated"))


    def on_device_removed(self, udi, *args): 
        self.logger.debug("DeviceRemoved: " + str(udi))
        self.devicelist.save()

        if self.udi_dict.has_key(udi):
            disp = self.udi_dict[udi]
            try:
                disp.on_removed()
            except:
                self.logger.warning(str(traceback.format_exc()))
                
            print
            print
            print "#############################################"
            print "DISCONNECTED ################################"
            print "#############################################"
            self.__print_properties(disp.properties)

            #from actors.deviceactor import DeviceActor
            #if disp.__class__ == DeviceActor:
            #    properties = disp.properties
            #    if properties.has_key('info.product') and \
            #            properties['info.product'] != '':
            #        product = properties['info.product']
            #        self.message_render.show_info(_("Information"), 
            #                _("Device DISCONNECTED:") + "\n %s" % product)

            del self.udi_dict[udi]
        else:
            self.message_render.show_warning(_("Warning"),
                    _("Device REMOVED."))

    def on_property_modified(self, udi, num, values):
        for ele in values:
            key = ele[0]

            if self.udi_dict.has_key(udi):
                # Actualizamos las propiedades del objeto actor
                actor = self.udi_dict[udi]
                obj = self.bus.get_object('org.freedesktop.Hal', udi)
                obj = dbus.Interface(obj, 'org.freedesktop.Hal.Device')

                actor.properties = obj.GetAllProperties()

                print
                print
                print "#############################################"
                print "MODIFIED PROPERTY:"
                print "udi:", udi
                print key, ':', actor.properties[key]
                print "#############################################"
                try:
                    actor.on_modified(key)
                except Exception, e:
                    self.logger.warning(str(traceback.format_exc()))



    def add_actor_from_properties(self, prop):
        """
        Devuelve un actor que pueda actuar para dispositivos con las propiedades
        espeficicadas en prop
        """
        max = 0
        klass = None
        actor_klass = None
        import actors
        for klass in actors.ACTORSLIST:
            count = self.__count_equals(prop, klass.__required__)
            if count > max:
                actor_klass = klass
                max = count

        actor = None 
        udi = prop['info.udi']
        if actor_klass:
            actor = actor_klass(self.message_render, prop)
            self.udi_dict[udi] = actor
            if not self.modify_handler_dict.has_key(udi):
                self.modify_handler_dict[udi] = lambda *args: self.on_property_modified(udi, *args) 
                self.bus.add_signal_receiver(self.modify_handler_dict[udi],
                    dbus_interface = 'org.freedesktop.Hal.Device',
                    signal_name = "PropertyModified",
                    path = udi)
        else:
            # Shorting logger setup (in module actors, logging.getLogger must be
            # invoked _after_ than in main function).
            from actors.deviceactor import DeviceActor
            actor = DeviceActor(self.message_render, prop)
            self.udi_dict[udi] = actor

        return actor


    def __print_properties(self, properties):
        print 
        print 
        print '-----------------------------------------------'
        print "Dispositivo: ", properties['info.udi']
        print 
        keys = properties.keys()
        keys.sort()

        for key in keys:
            print key + ':' + str(properties[key])


    def __count_equals(self, prop, required):
        """
        Devuelve el número de coincidencias entre el diccionario prop y
        required, siempre y cuando TODOS los elementos de required estén en
        prop.
        En caso contrario devuelve 0.
        """
        count = 0
        for key in required.keys():
            if not prop.has_key(key): 
                return 0

            if prop[key] != required[key]:
                return 0
            count += 1

        return  count


    def __init_actors(self):
        obj = self.bus.get_object('org.freedesktop.Hal', '/org/freedesktop/Hal/Manager')
        manager = dbus.Interface(obj, 'org.freedesktop.Hal.Manager')

        for udi in manager.GetAllDevices():
            obj = self.bus.get_object('org.freedesktop.Hal', udi)
            obj = dbus.Interface(obj, 'org.freedesktop.Hal.Device')

            properties = obj.GetAllProperties()
            self.add_actor_from_properties(properties)


def main():
    # Configure options
    parser = OptionParser(usage = 'usage: %prog [options]')
    parser.set_defaults(debug = False)
    parser.set_defaults(capture_log = False)

    parser.add_option('-d', '--debug', 
            action = 'store_true',
            dest = 'debug',
            help = 'start in debug mode')

    parser.add_option('-c', '--capture-log',
            action = 'store_true',
            dest = 'capture_log',
            help = 'Capture device logs.')

    (options, args) = parser.parse_args()
    del args

    
    # Option debug for logging
    if options.debug:
        level = logging.DEBUG
    else:
        level = logging.INFO

    logfilename = '/var/tmp/hermes-hardware-' + \
            os.environ['USER'] + str(os.getuid()) + \
            '.log' 

    logging.basicConfig(level = level,
            format='%(asctime)s %(levelname)s %(message)s',
                    filename = logfilename,
                    filemode='a')

    # Set capture log
    if options.capture_log:
        filepath = '/var/tmp/filenotification-' + \
                os.environ['USER'] + str(os.getuid()) + \
                '.log'
        iface = FileNotification(filepath)
        capture_log_gui = CaptureLogGui()
    else:
        iface = NotificationDaemon()

    global DeviceActor
    from actors.deviceactor import DeviceActor

    ##################################################################
    # Init application.   #
    #######################
    logging.getLogger().info("----------------------------- Hermes init.")

    DeviceListener(iface)
    gtk.threads_init()
    try:
        gtk.main()
    except:
        if 'capture_log_gui' in locals():
            # Close file for write in hd.
            capture_log_gui.logfile.close()

        logging.getLogger().info("----------------------------- Hermes finish.")


if __name__ == "__main__":
    main()


