# -*- coding: utf-8 -*-
#
# «gtkui» - GTK user interface
#
# Copyright (C) 2005 Junta de Andalucía
# Copyright (C) 2005, 2006 Canonical Ltd.
#
# Authors:
#
# - Javier Carranza <javier.carranza#interactors._coop>
# - Juan Jesús Ojeda Croissier <juanje#interactors._coop>
# - Antonio Olmo Titos <aolmo#emergya._info>
# - Gumer Coronel Pérez <gcoronel#emergya._info>
# - Colin Watson <cjwatson@ubuntu.com>
#
# This file is part of Ubiquity.
#
# Ubiquity is free software; you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free
# Software Foundation; either version 2 of the License, or at your option)
# any later version.
#
# Ubiquity is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with Ubiquity; if not, write to the Free Software Foundation, Inc., 51
# Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
##################################################################################

""" U{pylint<http://logilab.org/projects/pylint>} mark: -28.40!!! (bad
        indentation and accesses to undefined members) """

import sys
import pygtk
pygtk.require('2.0')

import gobject
import gtk.glade
import os
import time
import datetime
import glob
import subprocess
import math
import traceback
import syslog
import xml.sax.saxutils

import gettext

import debconf
try:
    from debconf import DebconfCommunicator
except ImportError:
    from ubiquity.debconfcommunicator import DebconfCommunicator

try:
    import problem_report
    import apport_utils
except ImportError:
    pass

from ubiquity import filteredcommand, validation
from ubiquity.misc import *
from ubiquity.settings import *
from ubiquity.components import console_setup, language, timezone, usersetup, \
                                partman_auto, partman_commit, summary, install
import ubiquity.emap
import ubiquity.tz
import ubiquity.progressposition

# Define global path
PATH = '/usr/share/ubiquity'

# Define glade path
GLADEDIR = os.path.join(PATH, 'glade')

# Define locale path
LOCALEDIR = "/usr/share/locale"

BREADCRUMB_STEPS = {
    "stepLanguage": 1,
    "stepLocation": 2,
    "stepKeyboardConf": 3,
    "stepUserInfo": 4,
    "stepPartDisk": 5,
    "stepPartAuto": 5,
    "stepPartAdvanced": 5,
    "stepPartMountpoints": 5,
    "stepReady": 6
}
BREADCRUMB_MAX_STEP = 6

# For the font wibbling later
import pango

class Wizard:

    def __init__(self, distro):
        sys.excepthook = self.excepthook

        # declare attributes
        self.distro = distro
        self.current_layout = None
        self.got_disk_choices = False
        self.auto_mountpoints = None
        self.resize_min_size = None
        self.resize_max_size = None
        self.manual_choice = None
        self.manual_partitioning = False
        self.password = ''
        self.hostname_edited = False
        self.gparted_fstype = {}
        self.gparted_flags = {}
        self.mountpoint_widgets = []
        self.size_widgets = []
        self.partition_widgets = []
        self.format_widgets = []
        self.mountpoint_choices = ['', 'swap', '/', '/home',
                                   '/boot', '/usr', '/var']
        self.partition_choices = []
        self.mountpoints = {}
        self.part_labels = {' ' : ' '}
        self.part_devices = {' ' : ' '}
        self.current_page = None
        self.dbfilter = None
        self.locale = None
        self.progress_position = ubiquity.progressposition.ProgressPosition()
        self.progress_cancelled = False
        self.previous_partitioning_page = None
        self.summary_device = None
        self.summary_device_button = None
        self.installing = False
        self.returncode = 0
        self.language_questions = ('live_installer', 'welcome_heading_label',
                                   'welcome_text_label', 'step_label',
                                   'cancel', 'back', 'next')
        self.allowed_change_step = True
        self.allowed_go_forward = True

        self.laptop = ex("laptop-detect")

        # set default language
        dbfilter = language.Language(self, DebconfCommunicator('ubiquity',
                                                               cloexec=True))
        dbfilter.cleanup()
        dbfilter.db.shutdown()

        gobject.timeout_add(30000, self.poke_screensaver)

        # To get a "busy mouse":
        self.watch = gtk.gdk.Cursor(gtk.gdk.WATCH)

        # set custom language
        self.set_locales()

        # load the interface
        self.glade = gtk.glade.XML('%s/ubiquity.glade' % GLADEDIR)

        # get widgets
        for widget in self.glade.get_widget_prefix(""):
            setattr(self, widget.get_name(), widget)
            # We generally want labels to be selectable so that people can
            # easily report problems in them
            # (https://launchpad.net/bugs/41618), but GTK+ likes to put
            # selectable labels in the focus chain, and I can't seem to turn
            # this off in glade and have it stick. Accordingly, make sure
            # labels are unfocusable here.
            if isinstance(widget, gtk.Label):
                widget.set_property('can-focus', False)

        self.translate_widgets()

        self.customize_installer()


    def excepthook(self, exctype, excvalue, exctb):
        """Crash handler."""

        if (issubclass(exctype, KeyboardInterrupt) or
            issubclass(exctype, SystemExit)):
            return

        tbtext = ''.join(traceback.format_exception(exctype, excvalue, exctb))
        syslog.syslog(syslog.LOG_ERR,
                      "Exception in GTK frontend (invoking crash handler):")
        for line in tbtext.split('\n'):
            syslog.syslog(syslog.LOG_ERR, line)
        print >>sys.stderr, ("Exception in GTK frontend"
                             " (invoking crash handler):")
        print >>sys.stderr, tbtext

        if 'problem_report' in sys.modules and 'apport_utils' in sys.modules:
            try:
                pr = problem_report.ProblemReport()
                apport_utils.report_add_package_info(pr, 'ubiquity-frontend-gtk')
                apport_utils.report_add_os_info(pr)
                apport_utils.report_add_proc_info(pr)
                pr['BugDisplayMode'] = 'file'
                pr['ExecutablePath'] = '/usr/bin/ubiquity'
                pr['PythonTraceback'] = tbtext
                if os.path.exists('/var/log/syslog'):
                    pr['UbiquitySyslog'] = ('/var/log/syslog',)
                if os.path.exists('/var/log/partman'):
                    pr['UbiquityPartman'] = ('/var/log/partman',)
                reportfile = open(apport_utils.make_report_path(pr), 'w')
                pr.write(reportfile)
                reportfile.close()
            except (KeyboardInterrupt, SystemExit):
                raise
            except:
                # Out of disk space? Fall back to our own crash handler.
                pass

        self.crash_detail_label.set_text(tbtext)
        self.crash_dialog.run()
        self.crash_dialog.hide()

        sys.exit(1)


    def run(self):
        """run the interface."""

        if os.getuid() != 0:
            title = ('This installer must be run with administrative '
                     'privileges, and cannot continue without them.')
            dialog = gtk.MessageDialog(self.live_installer, gtk.DIALOG_MODAL,
                                       gtk.MESSAGE_ERROR, gtk.BUTTONS_CLOSE,
                                       title)
            dialog.run()
            sys.exit(1)

        # show interface
        got_intro = self.show_intro()
        self.allow_change_step(True)

        # Declare SignalHandler
        self.glade.signal_autoconnect(self)

        # Some signals need to be connected by hand so that we have the
        # handler ids.
        self.hostname_delete_text_id = self.hostname.connect(
            'delete_text', self.on_hostname_delete_text)
        self.hostname_insert_text_id = self.hostname.connect(
            'insert_text', self.on_hostname_insert_text)

        # Start the interface
        if got_intro:
            global BREADCRUMB_STEPS, BREADCRUMB_MAX_STEP
            for step in BREADCRUMB_STEPS:
                BREADCRUMB_STEPS[step] += 1
            BREADCRUMB_STEPS["stepWelcome"] = 1
            BREADCRUMB_MAX_STEP += 1
            first_step = self.stepWelcome
        else:
            first_step = self.stepLanguage
        self.steps.set_current_page(self.steps.page_num(first_step))

        while self.current_page is not None:
            if not self.installing:
                # Make sure any started progress bars are stopped.
                while self.progress_position.depth() != 0:
                    self.debconf_progress_stop()

            self.backup = False
            current_name = self.step_name(self.current_page)
            old_dbfilter = self.dbfilter
            if current_name == "stepLanguage":
                self.dbfilter = language.Language(self)
            elif current_name == "stepLocation":
                self.dbfilter = timezone.Timezone(self)
            elif current_name == "stepKeyboardConf":
                self.dbfilter = console_setup.ConsoleSetup(self)
            elif current_name == "stepUserInfo":
                self.dbfilter = usersetup.UserSetup(self)
            elif current_name in ("stepPartDisk", "stepPartAuto"):
                if isinstance(self.dbfilter, partman_auto.PartmanAuto):
                    syslog.syslog('reusing running partman')
                else:
                    self.dbfilter = partman_auto.PartmanAuto(self)
            elif current_name == "stepReady":
                self.dbfilter = summary.Summary(self, self.manual_partitioning)
            else:
                self.dbfilter = None

            if self.dbfilter is not None and self.dbfilter != old_dbfilter:
                self.allow_change_step(False)
                self.dbfilter.start(auto_process=True)
            else:
                # Non-debconf steps don't have a mechanism for turning this
                # back on, so we do it here. process_step should block until
                # the next step has started up; this will block the UI, but
                # that's probably unavoidable for now. (We only use this for
                # gparted, which has its own UI loop.)
                self.allow_change_step(True)
            gtk.main()

            if self.installing:
                self.progress_loop()
            elif self.current_page is not None and not self.backup:
                self.process_step()

            while gtk.events_pending():
                gtk.main_iteration()

        return self.returncode


    def customize_installer(self):
        """Initial UI setup."""

        gtk.window_set_default_icon_from_file('/usr/share/pixmaps/'
                                              'ubiquity.png')

        PIXMAPSDIR = os.path.join(PATH, 'pixmaps', self.distro)

        # set pixmaps
        if ( gtk.gdk.get_default_root_window().get_screen().get_width() > 1024 ):
            logo = os.path.join(PIXMAPSDIR, "logo_1280.jpg")
            photo = os.path.join(PIXMAPSDIR, "photo_1280.jpg")
        else:
            logo = os.path.join(PIXMAPSDIR, "logo_1024.jpg")
            photo = os.path.join(PIXMAPSDIR, "photo_1024.jpg")
        if not os.path.exists(logo):
            logo = None
        if not os.path.exists(photo):
            photo = None

        self.logo_image.set_from_file(logo)
        self.photo.set_from_file(photo)

        self.live_installer.show()
        self.allow_change_step(False)

        self.tzmap = TimezoneMap(self)
        self.tzmap.tzmap.show()

        if not os.path.exists('/usr/bin/time-admin'):
            self.timezone_time_adjust.hide()

        # set initial bottom bar status
        self.back.hide()


    def poke_screensaver(self):
        """Attempt to make sure that the screensaver doesn't kick in."""
        if os.path.exists('/usr/bin/gnome-screensaver-command'):
            command = ["gnome-screensaver-command", "--poke"]
        elif os.path.exists('/usr/bin/xscreensaver-command'):
            command = ["xscreensaver-command", "--deactivate"]
        else:
            return

        env = ['LC_ALL=C']
        for key, value in os.environ.iteritems():
            if key != 'LC_ALL':
                env.append('%s=%s' % (key, value))
        gobject.spawn_async(command, envp=env,
                            flags=(gobject.SPAWN_SEARCH_PATH |
                                   gobject.SPAWN_STDOUT_TO_DEV_NULL |
                                   gobject.SPAWN_STDERR_TO_DEV_NULL),
                            child_setup=drop_privileges)
        return True


    def set_locales(self):
        """internationalization config. Use only once."""

        domain = self.distro + '-installer'
        gettext.bindtextdomain(domain, LOCALEDIR)
        gtk.glade.bindtextdomain(domain, LOCALEDIR )
        gtk.glade.textdomain(domain)
        gettext.textdomain(domain)
        gettext.install(domain, LOCALEDIR, unicode=1)


    def translate_widgets(self):
        if self.locale is None:
            languages = []
        else:
            languages = [self.locale]
        get_translations(languages=languages,
                         core_names=['ubiquity/text/%s' % q
                                     for q in self.language_questions])

        for widget in self.glade.get_widget_prefix(""):
            self.translate_widget(widget, self.locale)

    def translate_widget(self, widget, lang):
        if isinstance(widget, gtk.Button) and widget.get_use_stock():
            widget.set_label(widget.get_label())

        text = get_string(widget.get_name(), lang)
        if text is None:
            return

        if isinstance(widget, gtk.Label):
            name = widget.get_name()

            if name == 'step_label':
                global BREADCRUMB_STEPS, BREADCRUMB_MAX_STEP
                curstep = '?'
                if self.current_page is not None:
                    current_name = self.step_name(self.current_page)
                    if current_name in BREADCRUMB_STEPS:
                        curstep = str(BREADCRUMB_STEPS[current_name])
                text = text.replace('${INDEX}', curstep)
                text = text.replace('${TOTAL}', str(BREADCRUMB_MAX_STEP))
            widget.set_text(text)

            # Ideally, these attributes would be in the glade file somehow ...
            textlen = len(text.encode("UTF-8"))
            if 'heading_label' in name:
                attrs = pango.AttrList()
                attrs.insert(pango.AttrScale(pango.SCALE_LARGE, 0, textlen))
                attrs.insert(pango.AttrWeight(pango.WEIGHT_BOLD, 0, textlen))
                widget.set_attributes(attrs)
            elif 'extra_label' in name:
                attrs = pango.AttrList()
                attrs.insert(pango.AttrStyle(pango.STYLE_ITALIC, 0, textlen))
                widget.set_attributes(attrs)
            elif name in ('drives_label', 'partition_method_label',
                          'mountpoint_label', 'size_label', 'device_label',
                          'format_label'):
                attrs = pango.AttrList()
                attrs.insert(pango.AttrWeight(pango.WEIGHT_BOLD, 0, textlen))
                widget.set_attributes(attrs)

        elif isinstance(widget, gtk.Button):
            widget.set_label(text)

        elif isinstance(widget, gtk.Window):
            widget.set_title(text)


    def allow_change_step(self, allowed):
        if allowed:
            cursor = None
        else:
            cursor = self.watch
        self.live_installer.window.set_cursor(cursor)
        self.back.set_sensitive(allowed)
        self.next.set_sensitive(allowed and self.allowed_go_forward)
        self.allowed_change_step = allowed

    def allow_go_forward(self, allowed):
        self.next.set_sensitive(allowed and self.allowed_change_step)
        self.allowed_go_forward = allowed


    def show_intro(self):
        """Show some introductory text, if available."""

        intro = os.path.join(PATH, 'intro.txt')

        if os.path.isfile(intro):
            widget = gtk.Label()
            widget.set_line_wrap(True)
            intro_file = open(intro)
            widget.set_markup(intro_file.read().rstrip('\n'))
            intro_file.close()
            self.stepWelcome.add(widget)
            widget.show()
            return True
        else:
            return False


    def step_name(self, step_index):
        return self.steps.get_nth_page(step_index).get_name()


    def set_current_page(self, current):
        global BREADCRUMB_STEPS, BREADCRUMB_MAX_STEP
        self.current_page = current
        self.translate_widget(self.step_label, self.locale)

    # Methods

    def gparted_loop(self):
        """call gparted and embed it into glade interface."""

        syslog.syslog('gparted_loop()')

        disable_swap()

        child = self.embedded.get_child()
        if child is not None:
            self.embedded.remove(child)

        socket = gtk.Socket()
        socket.show()
        self.embedded.add(socket)
        window_id = str(socket.get_id())

        args = ['log-output', '-t', 'ubiquity', '--pass-stdout',
                'gparted', '--installer', window_id]
        for part in self.gparted_fstype:
            args.extend(['--filesystem',
                         '%s:%s' % (part, self.gparted_fstype[part])])
        syslog.syslog(syslog.LOG_DEBUG, 'Running gparted: %s' % ' '.join(args))

        # Save pid to kill gparted when install process starts
        self.gparted_subp = subprocess.Popen(
            args,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, close_fds=True)

        # Wait for gparted to start up before enabling back/forward buttons
        gparted_reply = ''
        while gparted_reply != '- READY':
            gparted_reply = self.gparted_subp.stdout.readline().rstrip('\n')


    def set_size_msg(self, widget):
        """return a string message with size value about
        the partition target by widget argument."""

        # widget is studied in a different manner depending on object type
        if widget.__class__ == str:
            size = float(self.size[widget.split('/')[2]])
        elif widget.get_active_text() in self.part_devices:
            size = float(self.size[self.part_devices[widget.get_active_text()].split('/')[2]])
        else:
            # TODO cjwatson 2006-07-31: Why isn't it in part_devices? This
            # indicates a deeper problem somewhere, but for now we'll just
            # try our best to ignore it.
            return ''

        if size > 1024*1024:
            msg = '%.0f Gb' % (size/1024/1024)
        elif size > 1024:
            msg = '%.0f Mb' % (size/1024)
        else:
            msg = '%.0f Kb' % size
        return msg


    def add_mountpoint_table_row(self):
        """Add a new empty row to the mountpoints table."""
        mountpoint = gtk.combo_box_entry_new_text()
        for mp in self.mountpoint_choices:
            mountpoint.append_text(mp)
        size = gtk.Label()
        size.set_single_line_mode(True)
        partition = gtk.combo_box_new_text()
        for part in self.partition_choices:
            if part in self.part_labels:
                partition.append_text(self.part_labels[part])
            else:
                partition.append_text(part)
        format = gtk.CheckButton()
        format.set_mode(draw_indicator=True)
        format.set_active(False)
        format.set_sensitive(False)

        row = len(self.mountpoint_widgets) + 1
        self.mountpoint_widgets.append(mountpoint)
        self.size_widgets.append(size)
        self.partition_widgets.append(partition)
        self.format_widgets.append(format)

        self.mountpoint_table.resize(row + 1, 4)
        self.mountpoint_table.attach(mountpoint, 0, 1, row, row + 1,
                                     yoptions=0)
        self.mountpoint_table.attach(size, 1, 2, row, row + 1,
                                     xoptions=0, yoptions=0)
        self.mountpoint_table.attach(partition, 2, 3, row, row + 1,
                                     yoptions=0)
        self.mountpoint_table.attach(format, 3, 4, row, row + 1,
                                     xoptions=0, yoptions=0)
        self.mountpoint_table.show_all()


    def progress_loop(self):
        """prepare, copy and config the system in the core install process."""

        syslog.syslog('progress_loop()')

        self.current_page = None

        self.debconf_progress_start(
            0, 100, get_string('ubiquity/install/title', self.locale))
        self.debconf_progress_region(0, 15)

        gvm_automount_drives = '/desktop/gnome/volume_manager/automount_drives'
        gvm_automount_media = '/desktop/gnome/volume_manager/automount_media'
        gconf_dir = 'xml:readwrite:%s' % os.path.expanduser('~/.gconf')
        gconf_previous = {}
        for gconf_key in (gvm_automount_drives, gvm_automount_media):
            subp = subprocess.Popen(['gconftool-2', '--config-source',
                                     gconf_dir, '--get', gconf_key],
                                    stdout=subprocess.PIPE,
                                    stderr=subprocess.PIPE)
            gconf_previous[gconf_key] = subp.communicate()[0].rstrip('\n')
            if gconf_previous[gconf_key] != 'false':
                subprocess.call(['gconftool-2', '--set', gconf_key,
                                 '--type', 'bool', 'false'])

        dbfilter = partman_commit.PartmanCommit(self, self.manual_partitioning)
        if dbfilter.run_command(auto_process=True) != 0:
            # TODO cjwatson 2006-09-03: return to partitioning?
            return

        for gconf_key in (gvm_automount_drives, gvm_automount_media):
            if gconf_previous[gconf_key] == '':
                ex('gconftool-2', '--unset', gconf_key)
            elif gconf_previous[gconf_key] != 'false':
                ex('gconftool-2', '--set', gconf_key,
                   '--type', 'bool', gconf_previous[gconf_key])

        self.debconf_progress_region(15, 100)

        dbfilter = install.Install(self)
        ret = dbfilter.run_command(auto_process=True)
        if ret != 0:
            self.installing = False
            if ret == 3:
                # error already handled by Install
                sys.exit(ret)
            elif os.path.exists('/var/lib/ubiquity/install.trace'):
                tbfile = open('/var/lib/ubiquity/install.trace')
                realtb = tbfile.read()
                tbfile.close()
                raise RuntimeError, ("Install failed with exit code %s\n%s" %
                                     (ret, realtb))
            else:
                raise RuntimeError, ("Install failed with exit code %s; see "
                                     "/var/log/syslog" % ret)

        while self.progress_position.depth() != 0:
            self.debconf_progress_stop()

        # just to make sure
        self.debconf_progress_window.hide()

        self.installing = False

        self.finished_dialog.run()


    def reboot(self, *args):
        """reboot the system after installing process."""

        self.returncode = 10
        self.quit()


    def do_reboot(self):
        """Callback for main program to actually reboot the machine."""

        if (os.path.exists("/usr/bin/gdm-signal") and
            os.path.exists("/usr/bin/gnome-session-save")):
            ex("gdm-signal", "--reboot")
            if 'SUDO_UID' in os.environ:
                user = '#%d' % int(os.environ['SUDO_UID'])
            else:
                user = 'ubuntu'
            ex("sudo", "-u", user, "-H",
               "gnome-session-save", "--kill", "--silent")
        else:
            ex("reboot")


    def quit(self):
        """quit installer cleanly."""

        # exiting from application
        self.current_page = None
        if self.dbfilter is not None:
            self.dbfilter.cancel_handler()
        if gtk.main_level() > 0:
            gtk.main_quit()


    # Callbacks
    def on_cancel_clicked(self, widget):
        self.warning_dialog.show()
        response = self.warning_dialog.run()
        self.warning_dialog.hide()
        if response == gtk.RESPONSE_CLOSE:
            self.current_page = None
            self.quit()
            return False
        else:
            return True # stop processing


    def on_live_installer_delete_event(self, widget, event):
        return self.on_cancel_clicked(widget)


    def on_list_changed(self, widget):
        """check if partition/mountpoint pair is filled and show the next pair
        on mountpoint screen. Also size label associated with partition combobox
        is changed dynamically to show the size partition."""

        if widget.get_active_text() not in ['', None]:
            if widget in self.partition_widgets:
                index = self.partition_widgets.index(widget)
            elif widget in self.mountpoint_widgets:
                index = self.mountpoint_widgets.index(widget)
            else:
                return

            partition_text = self.partition_widgets[index].get_active_text()
            if partition_text == ' ':
                self.size_widgets[index].set_text('')
            elif partition_text != None:
                self.size_widgets[index].set_text(self.set_size_msg(self.partition_widgets[index]))

            # Does the Reformat checkbox make sense?
            if (partition_text == ' ' or
                partition_text not in self.part_devices):
                self.format_widgets[index].set_sensitive(False)
                self.format_widgets[index].set_active(False)
            else:
                partition = self.part_devices[partition_text]
                if partition in self.gparted_fstype:
                    self.format_widgets[index].set_sensitive(False)
                    self.format_widgets[index].set_active(True)
                else:
                    self.format_widgets[index].set_sensitive(True)

            if len(get_partitions()) > len(self.partition_widgets):
                for i in range(len(self.partition_widgets)):
                    partition = self.partition_widgets[i].get_active_text()
                    mountpoint = self.mountpoint_widgets[i].get_active_text()
                    if partition is None or mountpoint == "":
                        break
                else:
                    # All table rows have been filled; create a new one.
                    self.add_mountpoint_table_row()
                    self.mountpoint_widgets[-1].connect("changed",
                                                        self.on_list_changed)
                    self.partition_widgets[-1].connect("changed",
                                                       self.on_list_changed)


    def info_loop(self, widget):
        """check if all entries from Identification screen are filled. Callback
        defined in glade file."""

        if (widget is not None and widget.get_name() == 'username' and
            not self.hostname_edited):
            if self.laptop:
                hostname_suffix = '-laptop'
            else:
                hostname_suffix = '-desktop'
            self.hostname.handler_block(self.hostname_delete_text_id)
            self.hostname.handler_block(self.hostname_insert_text_id)
            self.hostname.set_text(widget.get_text() + hostname_suffix)
            self.hostname.handler_unblock(self.hostname_insert_text_id)
            self.hostname.handler_unblock(self.hostname_delete_text_id)

        complete = True
        for name in ('username', 'password', 'verified_password', 'hostname'):
            if getattr(self, name).get_text() == '':
                complete = False
        self.allow_go_forward(complete)

    def on_hostname_delete_text(self, widget, start, end):
        self.hostname_edited = True

    def on_hostname_insert_text(self, widget,
                                new_text, new_text_length, position):
        self.hostname_edited = True

    def on_next_clicked(self, widget):
        """Callback to control the installation process between steps."""

        if not self.allowed_change_step or not self.allowed_go_forward:
            return

        self.allow_change_step(False)

        step = self.step_name(self.steps.get_current_page())

        if step == "stepUserInfo":
            self.username_error_box.hide()
            self.password_error_box.hide()
            self.hostname_error_box.hide()

        if self.dbfilter is not None:
            self.dbfilter.ok_handler()
            # expect recursive main loops to be exited and
            # debconffilter_done() to be called when the filter exits
        elif gtk.main_level() > 0:
            gtk.main_quit()

    def on_keyboard_layout_selected(self, start_editing, *args):
        if isinstance(self.dbfilter, console_setup.ConsoleSetup):
            layout = self.get_keyboard()
            if layout is not None:
                self.current_layout = layout
                self.dbfilter.change_layout(layout)

    def on_keyboard_variant_selected(self, start_editing, *args):
        if isinstance(self.dbfilter, console_setup.ConsoleSetup):
            layout = self.get_keyboard()
            variant = self.get_keyboard_variant()
            if layout is not None and variant is not None:
                self.dbfilter.apply_keyboard(layout, variant)

    def process_step(self):
        """Process and validate the results of this step."""

        # setting actual step
        step_num = self.steps.get_current_page()
        step = self.step_name(step_num)
        syslog.syslog('Step_before = %s' % step)

        if step.startswith("stepPart"):
            self.previous_partitioning_page = step_num

        # Welcome
        if step == "stepWelcome":
            self.steps.next_page()
        # Language
        elif step == "stepLanguage":
            self.translate_widgets()
            self.steps.next_page()
            self.back.show()
            self.allow_go_forward(self.get_timezone() is not None)
        # Location
        elif step == "stepLocation":
            self.steps.next_page()
        # Keyboard
        elif step == "stepKeyboardConf":
            self.steps.next_page()
            self.info_loop(None)
        # Identification
        elif step == "stepUserInfo":
            self.process_identification()
            self.got_disk_choices = False
        # Disk selection
        elif step == "stepPartDisk":
            self.process_disk_selection()
        # Automatic partitioning
        elif step == "stepPartAuto":
            self.process_autopartitioning()
        # Advanced partitioning
        elif step == "stepPartAdvanced":
            self.gparted_to_mountpoints()
        # Mountpoints
        elif step == "stepPartMountpoints":
            self.mountpoints_to_summary()
        # Ready to install
        elif step == "stepReady":
            self.live_installer.hide()
            self.current_page = None
            self.installing = True
            self.progress_loop()
            return

        step = self.step_name(self.steps.get_current_page())
        syslog.syslog('Step_after = %s' % step)

        if step == "stepReady":
            self.next.set_label("Install")

    def process_identification (self):
        """Processing identification step tasks."""

        error_msg = []
        error = 0

        # Validation stuff

        # checking hostname entry
        hostname = self.hostname.get_property('text')
        for result in validation.check_hostname(hostname):
            if result == validation.HOSTNAME_LENGTH:
                error_msg.append("The hostname must be between 3 and 18 characters long.")
            elif result == validation.HOSTNAME_WHITESPACE:
                error_msg.append("The hostname may not contain spaces.")
            elif result == validation.HOSTNAME_BADCHAR:
                error_msg.append("The hostname may only contain letters, digits, and hyphens.")

        # showing warning message is error is set
        if len(error_msg) != 0:
            self.hostname_error_reason.set_text("\n".join(error_msg))
            self.hostname_error_box.show()
        else:
            self.steps.next_page()


    def process_disk_selection (self):
        """Process disk selection before autopartitioning. This step will be
        skipped if only one disk is present."""

        # For safety, if we somehow ended up improperly initialised
        # then go to manual partitioning.
        choice = self.get_disk_choice()
        if self.manual_choice is None or choice == self.manual_choice:
            self.gparted_loop()
            self.steps.set_current_page(
                self.steps.page_num(self.stepPartAdvanced))
        else:
            self.steps.next_page()


    def process_autopartitioning(self):
        """Processing automatic partitioning step tasks."""

        while gtk.events_pending ():
            gtk.main_iteration ()

        # For safety, if we somehow ended up improperly initialised
        # then go to manual partitioning.
        choice = self.get_autopartition_choice()
        if self.manual_choice is None or choice == self.manual_choice:
            self.gparted_loop()
            self.steps.next_page()
        else:
            # TODO cjwatson 2006-01-10: extract mountpoints from partman
            self.manual_partitioning = False
            self.steps.set_current_page(self.steps.page_num(self.stepReady))


    def gparted_crashed(self):
        """gparted crashed. Ask the user if they want to continue."""

        # TODO cjwatson 2006-07-18: i18n
        text = ('The advanced partitioner (gparted) crashed. Further '
                'information may be found in /var/log/syslog, or by '
                'running gparted directly. Do you want to try the '
                'advanced partitioner again, return to automatic '
                'partitioning, or quit this installer?')
        dialog = gtk.Dialog('GParted crashed', self.live_installer,
                            gtk.DIALOG_MODAL,
                            (gtk.STOCK_QUIT, gtk.RESPONSE_CLOSE,
                             'Automatic partitioning', 1,
                             'Try again', 2))
        label = gtk.Label(text)
        label.set_line_wrap(True)
        label.set_selectable(True)
        dialog.vbox.add(label)
        dialog.show_all()
        response = dialog.run()
        dialog.hide()
        if response == 1:
            self.steps.set_current_page(self.steps.page_num(self.stepPartDisk))
        elif response == gtk.RESPONSE_CLOSE:
            self.current_page = None
            self.quit()
        else:
            self.gparted_loop()


    def gparted_to_mountpoints(self):
        """Processing gparted to mountpoints step tasks."""

        self.gparted_fstype = {}
        self.gparted_flags = {}

        if self.gparted_subp is None:
            self.gparted_crashed()
            return

        try:
            print >>self.gparted_subp.stdin, "apply"
        except IOError:
            # Shut down gparted
            self.gparted_subp.stdin.close()
            self.gparted_subp.wait()
            self.gparted_subp = None
            self.gparted_crashed()
            return

        # read gparted output of format "- FORMAT /dev/hda2 linux-swap"
        gparted_reply = self.gparted_subp.stdout.readline().rstrip('\n')
        while gparted_reply.startswith('- '):
            syslog.syslog('gparted replied: %s' % gparted_reply)
            words = gparted_reply[2:].strip().split()
            if words[0].lower() == 'format' and len(words) >= 3:
                self.gparted_fstype[words[1]] = words[2]
                self.gparted_flags[words[1]] = words[3:]
            gparted_reply = \
                self.gparted_subp.stdout.readline().rstrip('\n')
        syslog.syslog('gparted replied: %s' % gparted_reply)

        if gparted_reply.startswith('1 '):
            # Cancel
            return

        # Shut down gparted
        self.gparted_subp.stdin.close()
        self.gparted_subp.wait()
        self.gparted_subp = None

        if not gparted_reply.startswith('0 '):
            # something other than OK or Cancel
            return

        # Set up list of partition names for use in the mountpoints table.
        self.partition_choices = []
        # The first element is empty to allow deselecting a partition.
        self.partition_choices.append(' ')
        for partition in get_partitions():
            partition = '/dev/' + partition
            label = part_label(partition)
            self.part_labels[partition] = label
            self.part_devices[label] = partition
            self.partition_choices.append(partition)

        # Reinitialise the mountpoints table.
        for child in self.mountpoint_table.get_children():
            if child.get_name() not in ('mountpoint_label', 'size_label',
                                        'device_label', 'format_label'):
                self.mountpoint_table.remove(child)
        self.mountpoint_widgets = []
        self.size_widgets = []
        self.partition_widgets = []
        self.format_widgets = []

        self.add_mountpoint_table_row()

        # Try to get some default mountpoint selections.
        self.size = get_sizes()
        selection = get_default_partition_selection(
            self.size, self.gparted_fstype, self.auto_mountpoints)

        # Setting a default partition preselection
        if len(selection.items()) == 0:
            self.allow_go_forward(False)
        else:
            # Setting default preselection values into ComboBox widgets and
            # setting size values. In addition, the next row is shown if
            # they're validated.
            for mountpoint, partition in selection.items():
                if partition.split('/')[2] not in self.size:
                    syslog.syslog(syslog.LOG_WARNING,
                                  "No size available for partition %s; "
                                  "skipping" % partition)
                    continue
                if partition not in self.partition_choices:
                    # TODO cjwatson 2006-05-27: I don't know why this might
                    # happen, but it does
                    # (https://launchpad.net/bugs/46910). Figure out why. In
                    # the meantime, ignoring this partition is better than
                    # crashing.
                    syslog.syslog(syslog.LOG_WARNING,
                                  "Partition %s not in /proc/partitions?" %
                                  partition)
                    continue
                if mountpoint in self.mountpoint_choices:
                    self.mountpoint_widgets[-1].set_active(
                        self.mountpoint_choices.index(mountpoint))
                else:
                    self.mountpoint_widgets[-1].child.set_text(mountpoint)
                self.size_widgets[-1].set_text(
                    self.set_size_msg(partition))
                self.partition_widgets[-1].set_active(
                    self.partition_choices.index(partition))
                if (mountpoint in ('swap', '/', '/usr', '/var', '/boot') or
                    partition in self.gparted_fstype):
                    self.format_widgets[-1].set_active(True)
                else:
                    self.format_widgets[-1].set_active(False)
                if partition not in self.gparted_fstype:
                    self.format_widgets[-1].set_sensitive(True)
                if len(get_partitions()) > len(self.partition_widgets):
                    self.add_mountpoint_table_row()
                else:
                    break

        # For some reason, GtkTable doesn't seem to queue a resize itself
        # when you attach children to it.
        self.mountpoint_table.queue_resize()

        # We defer connecting up signals until now to avoid the changed
        # signal firing while we're busy populating the table.
        for mountpoint in self.mountpoint_widgets:
            mountpoint.connect("changed", self.on_list_changed)
        for partition in self.partition_widgets:
            partition.connect("changed", self.on_list_changed)

        self.mountpoint_error_reason.hide()
        self.mountpoint_error_image.hide()

        self.steps.next_page()


    def mountpoints_to_summary(self):
        """Processing mountpoints to summary step tasks."""

        # Validating self.mountpoints
        error_msg = []

        mountpoints = {}
        for i in range(len(self.mountpoint_widgets)):
            mountpoint_value = self.mountpoint_widgets[i].get_active_text()
            partition_value = self.partition_widgets[i].get_active_text()
            if partition_value is not None:
                if partition_value in self.part_devices:
                    partition_id = self.part_devices[partition_value]
                else:
                    partition_id = partition_value
            else:
                partition_id = None
            format_value = self.format_widgets[i].get_active()
            fstype = None
            if partition_id in self.gparted_fstype:
                fstype = self.gparted_fstype[partition_id]

            if mountpoint_value == "":
                if partition_value in (None, ' '):
                    continue
                else:
                    error_msg.append(
                        "No mount point selected for %s." % partition_value)
                    break
            else:
                if partition_value in (None, ' '):
                    error_msg.append(
                        "No partition selected for %s." % mountpoint_value)
                    break
                else:
                    flags = None
                    if partition_id in self.gparted_flags:
                        flags = self.gparted_flags[partition_id]
                    mountpoints[partition_id] = \
                        (mountpoint_value, format_value, fstype, flags)
        else:
            self.mountpoints = mountpoints
        syslog.syslog('mountpoints: %s' % self.mountpoints)

        # Checking duplicated devices
        partitions = [w.get_active_text() for w in self.partition_widgets]

        for check in partitions:
            if check in (None, '', ' '):
                continue
            if partitions.count(check) > 1:
                error_msg.append("A partition is assigned to more than one "
                                 "mount point.")
                break

        # Processing more validation stuff
        if len(self.mountpoints) > 0:
            # Supplement filesystem types from gparted FORMAT instructions
            # with those detected from the disk.
            validate_mountpoints = dict(self.mountpoints)
            validate_filesystems = get_filesystems(self.gparted_fstype)
            for device, (path, format, fstype,
                         flags) in validate_mountpoints.items():
                if fstype is None and device in validate_filesystems:
                    validate_mountpoints[device] = \
                        (path, format, validate_filesystems[device], None)
            # Check for some special-purpose partitions detected by partman.
            for device, mountpoint in self.auto_mountpoints.iteritems():
                if device in validate_mountpoints:
                    continue
                if not mountpoint.startswith('/'):
                    validate_mountpoints[device] = \
                        (mountpoint, False, None, None)

            for check in validation.check_mountpoint(validate_mountpoints,
                                                     self.size):
                if check == validation.MOUNTPOINT_NOROOT:
                    error_msg.append(get_string(
                        'partman-target/no_root', self.locale))
                elif check == validation.MOUNTPOINT_DUPPATH:
                    error_msg.append("Two file systems are assigned the same "
                                     "mount point.")
                elif check == validation.MOUNTPOINT_BADSIZE:
                    for mountpoint, format, fstype, flags in \
                            self.mountpoints.itervalues():
                        if mountpoint == 'swap':
                            min_root = MINIMAL_PARTITION_SCHEME['root']
                            break
                    else:
                        min_root = (MINIMAL_PARTITION_SCHEME['root'] +
                                    MINIMAL_PARTITION_SCHEME['swap'])
                    error_msg.append("The partition assigned to '/' is too "
                                     "small (minimum size: %d Mb)." % min_root)
                elif check == validation.MOUNTPOINT_BADCHAR:
                    error_msg.append(get_string(
                        'partman-basicfilesystems/bad_mountpoint',
                        self.locale))
                elif check == validation.MOUNTPOINT_XFSROOT:
                    error_msg.append("XFS may not be used on the filesystem "
                                     "containing /boot. Either use a "
                                     "different filesystem for / or create a "
                                     "non-XFS filesystem for /boot.")
                elif check == validation.MOUNTPOINT_XFSBOOT:
                    error_msg.append("XFS may not be used on the /boot "
                                     "filesystem. Use a different filesystem "
                                     "type for /boot.")
                elif check == validation.MOUNTPOINT_UNFORMATTED:
                    error_msg.append("Filesystems used by the system (/, "
                                     "/boot, /usr, /var) must be reformatted "
                                     "for use by this installer. Other "
                                     "filesystems (/home, /media/*, "
                                     "/usr/local, etc.) may be used without "
                                     "reformatting.")
                elif check == validation.MOUNTPOINT_NEEDPOSIX:
                    error_msg.append("FAT and NTFS filesystems may not be "
                                     "used on filesystems used by the system "
                                     "(/, /boot, /home, /usr, /var, etc.). "
                                     "It is usually best to mount them "
                                     "somewhere under /media/.")
                elif check == validation.MOUNTPOINT_NONEWWORLD:
                    error_msg.append(get_string(
                        'partman-newworld/no_newworld',
                        'extended:%s' % self.locale))

        # showing warning messages
        self.mountpoint_error_reason.set_text("\n".join(error_msg))
        if len(error_msg) != 0:
            self.mountpoint_error_reason.show()
            self.mountpoint_error_image.show()
            return
        else:
            self.mountpoint_error_reason.hide()
            self.mountpoint_error_image.hide()

        self.manual_partitioning = True
        self.steps.next_page()


    def on_back_clicked(self, widget):
        """Callback to set previous screen."""

        if not self.allowed_change_step:
            return

        self.allow_change_step(False)

        self.backup = True

        # Enabling next button
        self.allow_go_forward(True)
        # Setting actual step
        step = self.step_name(self.steps.get_current_page())

        changed_page = False

        if step == "stepLocation":
            self.back.hide()
        elif step == "stepPartAuto":
            if self.got_disk_choices:
                new_step = self.stepPartDisk
            else:
                new_step = self.stepUserInfo
            self.steps.set_current_page(self.steps.page_num(new_step))
            changed_page = True
        elif step == "stepPartAdvanced":
            if self.gparted_subp is not None:
                try:
                    print >>self.gparted_subp.stdin, "undo"
                except IOError:
                    pass
                self.gparted_subp.stdin.close()
                self.gparted_subp.wait()
                self.gparted_subp = None
            self.steps.set_current_page(self.steps.page_num(self.stepPartDisk))
            changed_page = True
        elif step == "stepPartMountpoints":
            self.gparted_loop()
        elif step == "stepReady":
            self.next.set_label("gtk-go-forward")
            self.steps.set_current_page(self.previous_partitioning_page)
            changed_page = True

        if not changed_page:
            self.steps.prev_page()

        if self.dbfilter is not None:
            self.dbfilter.cancel_handler()
            # expect recursive main loops to be exited and
            # debconffilter_done() to be called when the filter exits
        elif gtk.main_level() > 0:
            gtk.main_quit()


    def on_language_treeview_selection_changed (self, selection):
        (model, iterator) = selection.get_selected()
        if iterator is not None:
            value = unicode(model.get_value(iterator, 0))
            lang = self.language_choice_map[value][1]
            # strip encoding; we use UTF-8 internally no matter what
            lang = lang.split('.')[0].lower()
            for widget in self.language_questions:
                self.translate_widget(getattr(self, widget), lang)


    def on_timezone_time_adjust_clicked (self, button):
        invisible = gtk.Invisible()
        invisible.grab_add()
        time_admin_env = dict(os.environ)
        tz = self.tzmap.get_selected_tz_name()
        if tz is not None:
            time_admin_env['TZ'] = tz
        if 'DESKTOP_STARTUP_ID' in time_admin_env:
            del time_admin_env['DESKTOP_STARTUP_ID']
        time_admin_env['GST_NO_INSTALL_NTP'] = '1'
        time_admin_subp = subprocess.Popen(["log-output", "-t", "ubiquity",
                                            "time-admin"], env=time_admin_env)
        gobject.child_watch_add(time_admin_subp.pid, self.on_time_admin_exit,
                                invisible)


    def on_time_admin_exit (self, pid, condition, invisible):
        invisible.grab_remove()


    def on_new_size_scale_format_value (self, widget, value):
        # TODO cjwatson 2006-01-09: get minsize/maxsize through to here
        if self.resize_max_size is not None:
            size = value * self.resize_max_size / 100
            return '%d%% (%s)' % (value, format_size(size))
        else:
            return '%d%%' % value


    def on_steps_switch_page (self, foo, bar, current):
        self.set_current_page(current)
        current_name = self.step_name(current)
        syslog.syslog('switched to page %s' % current_name)


    def on_autopartition_resize_toggled (self, widget):
        """Update autopartitioning screen when the resize button is
        selected."""

        if widget.get_active():
            self.new_size_vbox.show()
        else:
            self.new_size_vbox.hide()


    # Callbacks provided to components.

    def watch_debconf_fd (self, from_debconf, process_input):
        gobject.io_add_watch(from_debconf,
                             gobject.IO_IN | gobject.IO_ERR | gobject.IO_HUP,
                             self.watch_debconf_fd_helper, process_input)


    def watch_debconf_fd_helper (self, source, cb_condition, callback):
        debconf_condition = 0
        if (cb_condition & gobject.IO_IN) != 0:
            debconf_condition |= filteredcommand.DEBCONF_IO_IN
        if (cb_condition & gobject.IO_ERR) != 0:
            debconf_condition |= filteredcommand.DEBCONF_IO_ERR
        if (cb_condition & gobject.IO_HUP) != 0:
            debconf_condition |= filteredcommand.DEBCONF_IO_HUP

        return callback(source, debconf_condition)


    def debconf_progress_start (self, progress_min, progress_max, progress_title):
        if self.progress_cancelled:
            return False
        if self.current_page is not None:
            self.debconf_progress_window.set_transient_for(self.live_installer)
        else:
            self.debconf_progress_window.set_transient_for(None)
        if progress_title is None:
            progress_title = ""
        if self.progress_position.depth() == 0:
            self.debconf_progress_window.set_title(progress_title)

        self.progress_position.start(progress_min, progress_max,
                                     progress_title)
        self.progress_title.set_markup(
            '<big><b>' +
            xml.sax.saxutils.escape(self.progress_position.title()) +
            '</b></big>')
        self.debconf_progress_set(0)
        self.progress_info.set_text('')
        self.debconf_progress_window.show()
        return True

    def debconf_progress_set (self, progress_val):
        if self.progress_cancelled:
            return False
        self.progress_position.set(progress_val)
        fraction = self.progress_position.fraction()
        self.progress_bar.set_fraction(fraction)
        self.progress_bar.set_text('%s%%' % int(fraction * 100))
        return True

    def debconf_progress_step (self, progress_inc):
        if self.progress_cancelled:
            return False
        self.progress_position.step(progress_inc)
        fraction = self.progress_position.fraction()
        self.progress_bar.set_fraction(fraction)
        self.progress_bar.set_text('%s%%' % int(fraction * 100))
        return True

    def debconf_progress_info (self, progress_info):
        if self.progress_cancelled:
            return False
        self.progress_info.set_markup(
            '<i>' + xml.sax.saxutils.escape(progress_info) + '</i>')
        return True

    def debconf_progress_stop (self):
        if self.progress_cancelled:
            self.progress_cancelled = False
            return False
        self.progress_position.stop()
        if self.progress_position.depth() == 0:
            self.debconf_progress_window.hide()
        else:
            self.progress_title.set_markup(
                '<big><b>' +
                xml.sax.saxutils.escape(self.progress_position.title()) +
                '</b></big>')
        return True

    def debconf_progress_region (self, region_start, region_end):
        self.progress_position.set_region(region_start, region_end)

    def debconf_progress_cancellable (self, cancellable):
        if cancellable:
            self.progress_cancel_button.show()
        else:
            self.progress_cancel_button.hide()
            self.progress_cancelled = False

    def on_progress_cancel_button_clicked (self, button):
        self.progress_cancelled = True


    def debconffilter_done (self, dbfilter):
        # TODO cjwatson 2006-02-10: handle dbfilter.status
        if dbfilter is None:
            name = 'None'
        else:
            name = dbfilter.__class__.__name__
        if self.dbfilter is None:
            currentname = 'None'
        else:
            currentname = self.dbfilter.__class__.__name__
        syslog.syslog(syslog.LOG_DEBUG,
                      "debconffilter_done: %s (current: %s)" %
                      (name, currentname))
        if dbfilter == self.dbfilter:
            self.dbfilter = None
            if isinstance(dbfilter, summary.Summary):
                # The Summary component is just there to gather information,
                # and won't call run_main_loop() for itself.
                self.allow_change_step(True)
            elif gtk.main_level() > 0:
                gtk.main_quit()


    def set_language_choices (self, choices, choice_map):
        self.language_choice_map = dict(choice_map)
        if len(self.language_treeview.get_columns()) < 1:
            column = gtk.TreeViewColumn(None, gtk.CellRendererText(), text=0)
            column.set_sizing(gtk.TREE_VIEW_COLUMN_FIXED)
            self.language_treeview.append_column(column)
            selection = self.language_treeview.get_selection()
            selection.connect('changed',
                              self.on_language_treeview_selection_changed)
        list_store = gtk.ListStore(gobject.TYPE_STRING)
        self.language_treeview.set_model(list_store)
        for choice in choices:
            list_store.append([choice])


    def set_language (self, language):
        model = self.language_treeview.get_model()
        iterator = model.iter_children(None)
        while iterator is not None:
            if unicode(model.get_value(iterator, 0)) == language:
                path = model.get_path(iterator)
                self.language_treeview.get_selection().select_path(path)
                self.language_treeview.scroll_to_cell(
                    path, use_align=True, row_align=0.5)
                break
            iterator = model.iter_next(iterator)


    def get_language (self):
        selection = self.language_treeview.get_selection()
        (model, iterator) = selection.get_selected()
        if iterator is None:
            return 'C'
        else:
            value = unicode(model.get_value(iterator, 0))
            return self.language_choice_map[value][0]


    def set_timezone (self, timezone):
        self.tzmap.set_tz_from_name(timezone)


    def get_timezone (self):
        return self.tzmap.get_selected_tz_name()


    def set_fullname(self, value):
        self.fullname.set_text(value)

    def get_fullname(self):
        return self.fullname.get_text()

    def set_username(self, value):
        self.username.set_text(value)

    def get_username(self):
        return self.username.get_text()

    def get_password(self):
        return self.password.get_text()

    def get_verified_password(self):
        return self.verified_password.get_text()

    def username_error(self, msg):
        self.username_error_reason.set_text(msg)
        self.username_error_box.show()

    def password_error(self, msg):
        self.password_error_reason.set_text(msg)
        self.password_error_box.show()


    def set_auto_mountpoints(self, auto_mountpoints):
        self.auto_mountpoints = auto_mountpoints


    def set_disk_choices (self, choices, manual_choice):
        self.got_disk_choices = True

        for child in self.part_disk_vbox.get_children():
            self.part_disk_vbox.remove(child)

        self.manual_choice = manual_choice
        firstbutton = None
        for choice in choices:
            if choice == '':
                self.part_disk_vbox.add(gtk.Alignment())
            else:
                button = gtk.RadioButton(firstbutton, choice, False)
                if firstbutton is None:
                    firstbutton = button
                self.part_disk_vbox.add(button)
        if firstbutton is not None:
            firstbutton.set_active(True)

        self.part_disk_vbox.show_all()

        # make sure we're on the disk selection page
        self.steps.set_current_page(self.steps.page_num(self.stepPartDisk))

        return True


    def get_disk_choice (self):
        for widget in self.part_disk_vbox.get_children():
            if isinstance(widget, gtk.Button) and widget.get_active():
                return widget.get_label()


    def set_autopartition_choices (self, choices, resize_choice, manual_choice):
        for child in self.autopartition_vbox.get_children():
            self.autopartition_vbox.remove(child)

        self.manual_choice = manual_choice
        firstbutton = None
        for choice in choices:
            button = gtk.RadioButton(firstbutton, choice, False)
            if firstbutton is None:
                firstbutton = button
            self.autopartition_vbox.add(button)
            if choice == resize_choice:
                self.on_autopartition_resize_toggled(button)
                button.connect('toggled', self.on_autopartition_resize_toggled)
        if firstbutton is not None:
            firstbutton.set_active(True)
        if resize_choice not in choices:
            self.new_size_vbox.hide()

        self.autopartition_vbox.show_all()

        # make sure we're on the autopartitioning page
        self.steps.set_current_page(self.steps.page_num(self.stepPartAuto))


    def get_autopartition_choice (self):
        for button in self.autopartition_vbox.get_children():
            if button.get_active():
                return button.get_label()


    def set_autopartition_resize_bounds (self, min_size, max_size):
        self.resize_min_size = min_size
        self.resize_max_size = max_size
        if min_size is not None and max_size is not None:
            min_percent = int(math.ceil(100 * min_size / max_size))
            self.new_size_scale.set_range(min_percent, 100)
            self.new_size_scale.set_value(int((min_percent + 100) / 2))


    def get_autopartition_resize_percent (self):
        return self.new_size_scale.get_value()


    def get_hostname (self):
        return self.hostname.get_text()


    def get_mountpoints (self):
        return dict(self.mountpoints)


    def set_keyboard_choices(self, choices):
        layouts = gtk.ListStore(gobject.TYPE_STRING)
        self.keyboardlayoutview.set_model(layouts)
        for v in sorted(choices):
            layouts.append([v])

        if len(self.keyboardlayoutview.get_columns()) < 1:
            column = gtk.TreeViewColumn("Layout", gtk.CellRendererText(), text=0)
            column.set_sizing(gtk.TREE_VIEW_COLUMN_FIXED)
            self.keyboardlayoutview.append_column(column)
            selection = self.keyboardlayoutview.get_selection()
            selection.connect('changed',
                              self.on_keyboard_layout_selected)

        if self.current_layout is not None:
            self.set_keyboard(self.current_layout)

    def set_keyboard (self, layout):
        self.current_layout = layout
        model = self.keyboardlayoutview.get_model()
        if model is None:
            return
        iterator = model.iter_children(None)
        while iterator is not None:
            if unicode(model.get_value(iterator, 0)) == layout:
                path = model.get_path(iterator)
                self.keyboardlayoutview.get_selection().select_path(path)
                self.keyboardlayoutview.scroll_to_cell(
                    path, use_align=True, row_align=0.5)
                break
            iterator = model.iter_next(iterator)

    def get_keyboard (self):
        selection = self.keyboardlayoutview.get_selection()
        (model, iterator) = selection.get_selected()
        if iterator is None:
            return None
        else:
            return unicode(model.get_value(iterator, 0))

    def set_keyboard_variant_choices(self, choices):
        variants = gtk.ListStore(gobject.TYPE_STRING)
        self.keyboardvariantview.set_model(variants)
        for v in sorted(choices):
            variants.append([v])

        if len(self.keyboardvariantview.get_columns()) < 1:
            column = gtk.TreeViewColumn("Variant", gtk.CellRendererText(), text=0)
            column.set_sizing(gtk.TREE_VIEW_COLUMN_FIXED)
            self.keyboardvariantview.append_column(column)
            selection = self.keyboardvariantview.get_selection()
            selection.connect('changed',
                              self.on_keyboard_variant_selected)

    def set_keyboard_variant (self, variant):
        model = self.keyboardvariantview.get_model()
        if model is None:
            return
        iterator = model.iter_children(None)
        while iterator is not None:
            if unicode(model.get_value(iterator, 0)) == variant:
                path = model.get_path(iterator)
                self.keyboardvariantview.get_selection().select_path(path)
                self.keyboardvariantview.scroll_to_cell(
                    path, use_align=True, row_align=0.5)
                break
            iterator = model.iter_next(iterator)

    def get_keyboard_variant (self):
        selection = self.keyboardvariantview.get_selection()
        (model, iterator) = selection.get_selected()
        if iterator is None:
            return None
        else:
            return unicode(model.get_value(iterator, 0))

    def set_summary_text (self, text):
        for child in self.ready_text.get_children():
            self.ready_text.remove(child)

        ready_buffer = gtk.TextBuffer()
        ready_buffer.set_text(text)
        self.ready_text.set_buffer(ready_buffer)
        device_index = text.find("DEVICE")
        if device_index != -1:
            device_start_iter = ready_buffer.get_iter_at_offset(device_index)
            device_end_iter = ready_buffer.get_iter_at_offset(device_index + 6)
            ready_buffer.delete(device_start_iter, device_end_iter)
            device_anchor = ready_buffer.create_child_anchor(device_start_iter)
            self.summary_device_button = gtk.Button()
            self.summary_device_button.connect(
                'clicked', self.on_summary_device_button_clicked)
            self.summary_device_button.show()
            self.ready_text.add_child_at_anchor(self.summary_device_button,
                                                device_anchor)

    def set_summary_device (self, device):
        if not device.startswith('(') and not device.startswith('/dev/'):
            device = '/dev/%s' % device
        self.summary_device = device

        # i.e. set_summary_text has been called
        if self.summary_device_button is None:
            syslog.syslog(syslog.LOG_ERR,
                          "summary_device_button missing (broken "
                          "ubiquity/summary/grub translation?)")
            return
        self.summary_device_button.set_label(device)

    def get_summary_device (self):
        return self.summary_device

    def on_summary_device_button_clicked (self, button):
        self.grub_device_entry.set_text(self.get_summary_device())
        response = self.grub_device_dialog.run()
        self.grub_device_dialog.hide()
        if response == gtk.RESPONSE_OK:
            self.set_summary_device(self.grub_device_entry.get_text())
        return True


    def return_to_autopartitioning (self):
        """If the install progress bar is up but still at the partitioning
        stage, then errors can safely return us to autopartitioning.
        """

        if self.installing and self.current_page is not None:
            # Go back to the autopartitioner and try again.
            # TODO self.previous_partitioning_page
            self.live_installer.show()
            self.steps.set_current_page(self.steps.page_num(self.stepPartDisk))
            self.next.set_label("gtk-go-forward")
            self.backup = True
            self.installing = False

    def error_dialog (self, title, msg, fatal=True):
        # TODO: cancel button as well if capb backup
        self.allow_change_step(True)
        if self.current_page is not None:
            transient = self.live_installer
        else:
            transient = self.debconf_progress_window
        dialog = gtk.MessageDialog(transient, gtk.DIALOG_MODAL,
                                   gtk.MESSAGE_ERROR, gtk.BUTTONS_OK, msg)
        dialog.set_title(title)
        dialog.run()
        dialog.hide()
        if fatal:
            self.return_to_autopartitioning()

    def question_dialog (self, title, msg, option_templates):
        self.allow_change_step(True)
        if self.current_page is not None:
            transient = self.live_installer
        else:
            transient = self.debconf_progress_window
        buttons = []
        for option_template in option_templates:
            text = get_string(option_template, self.locale)
            if text is None:
                text = option_template
            # Work around PyGTK bug; each button text must actually be a
            # subtype of str, which unicode isn't.
            text = str(text)
            buttons.extend((text, len(buttons) / 2 + 1))
        dialog = gtk.Dialog(title, transient, gtk.DIALOG_MODAL, tuple(buttons))
        label = gtk.Label(msg)
        label.set_line_wrap(True)
        label.set_selectable(True)
        label.show()
        dialog.vbox.pack_start(label)
        response = dialog.run()
        dialog.hide()
        if response < 0:
            # something other than a button press, probably destroyed
            return None
        else:
            return option_templates[response - 1]


    def refresh (self):
        while gtk.events_pending():
            gtk.main_iteration()


    # Run the UI's main loop until it returns control to us.
    def run_main_loop (self):
        self.allow_change_step(True)
        gtk.main()


    # Return control to the next level up.
    def quit_main_loop (self):
        if gtk.main_level() > 0:
            gtk.main_quit()


# Much of this timezone map widget is a rough translation of
# gnome-system-tools/src/time/tz-map.c. Thanks to Hans Petter Jansson
# <hpj@ximian.com> for that.

NORMAL_RGBA = 0xc070a0ffL
HOVER_RGBA = 0xffff60ffL
SELECTED_1_RGBA = 0xff60e0ffL
SELECTED_2_RGBA = 0x000000ffL

class TimezoneMap(object):
    def __init__(self, frontend):
        self.frontend = frontend
        self.tzdb = ubiquity.tz.Database()
        self.tzmap = ubiquity.emap.EMap()
        self.update_timeout = None
        self.point_selected = None
        self.point_hover = None
        self.location_selected = None

        zoom_in_file = os.path.join(PATH, 'pixmaps', 'zoom-in.png')
        if os.path.exists(zoom_in_file):
            display = self.frontend.live_installer.get_display()
            pixbuf = gtk.gdk.pixbuf_new_from_file(zoom_in_file)
            self.cursor_zoom_in = gtk.gdk.Cursor(display, pixbuf, 10, 10)
        else:
            self.cursor_zoom_in = None

        self.tzmap.add_events(gtk.gdk.LEAVE_NOTIFY_MASK |
                              gtk.gdk.VISIBILITY_NOTIFY_MASK)

        self.frontend.timezone_map_window.add(self.tzmap)

        timezone_city_combo = self.frontend.timezone_city_combo

        renderer = gtk.CellRendererText()
        timezone_city_combo.pack_start(renderer, True)
        timezone_city_combo.add_attribute(renderer, 'text', 0)
        list_store = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING)
        timezone_city_combo.set_model(list_store)

        prev_continent = ''
        for location in self.tzdb.locations:
            self.tzmap.add_point("", location.longitude, location.latitude,
                                 NORMAL_RGBA)
            zone_bits = location.zone.split('/')
            if len(zone_bits) == 1:
                continue
            continent = zone_bits[0]
            if continent != prev_continent:
                list_store.append(['', None])
                list_store.append(["--- %s ---" % continent, None])
                prev_continent = continent
            human_zone = '/'.join(zone_bits[1:]).replace('_', ' ')
            list_store.append([human_zone, location.zone])

        self.tzmap.connect("map-event", self.mapped)
        self.tzmap.connect("unmap-event", self.unmapped)
        self.tzmap.connect("motion-notify-event", self.motion)
        self.tzmap.connect("button-press-event", self.button_pressed)
        self.tzmap.connect("leave-notify-event", self.out_map)

        timezone_city_combo.connect("changed", self.city_changed)

    def set_city_text(self, name):
        model = self.frontend.timezone_city_combo.get_model()
        iterator = model.get_iter_first()
        while iterator is not None:
            location = model.get_value(iterator, 1)
            if location == name:
                self.frontend.timezone_city_combo.set_active_iter(iterator)
                break
            iterator = model.iter_next(iterator)

    def set_zone_text(self, location):
        offset = location.utc_offset
        if offset >= datetime.timedelta(0):
            minuteoffset = int(offset.seconds / 60)
        else:
            minuteoffset = int(offset.seconds / 60 - 1440)
        if location.zone_letters == 'GMT':
            text = location.zone_letters
        else:
            text = "%s (GMT%+d:%02d)" % (location.zone_letters,
                                         minuteoffset / 60, minuteoffset % 60)
        self.frontend.timezone_zone_text.set_text(text)
        translations = gettext.translation('iso_3166',
                                           languages=[self.frontend.locale],
                                           fallback=True)
        self.frontend.timezone_country_text.set_text(
            translations.ugettext(location.human_country))
        self.update_current_time()

    def update_current_time(self):
        if self.location_selected is not None:
            now = datetime.datetime.now(self.location_selected.info)
            self.frontend.timezone_time_text.set_text(now.strftime('%X'))

    def set_tz_from_name(self, name):
        (longitude, latitude) = (0.0, 0.0)

        for location in self.tzdb.locations:
            if location.zone == name:
                (longitude, latitude) = (location.longitude, location.latitude)
                break
        else:
            return

        if self.point_selected is not None:
            self.tzmap.point_set_color_rgba(self.point_selected, NORMAL_RGBA)

        self.point_selected = self.tzmap.get_closest_point(longitude, latitude,
                                                           False)

        self.location_selected = location
        self.set_city_text(self.location_selected.zone)
        self.set_zone_text(self.location_selected)
        self.frontend.allow_go_forward(True)

    def city_changed(self, widget):
        iterator = widget.get_active_iter()
        if iterator is not None:
            model = widget.get_model()
            location = model.get_value(iterator, 1)
            if location is not None:
                self.set_tz_from_name(location)

    def get_selected_tz_name(self):
        if self.location_selected is not None:
            return self.location_selected.zone
        else:
            return None

    def location_from_point(self, point):
        if point is None:
            return None

        (longitude, latitude) = point.get_location()

        best_location = None
        best_distance = None
        for location in self.tzdb.locations:
            if (abs(location.longitude - longitude) <= 1.0 and
                abs(location.latitude - latitude) <= 1.0):
                distance = ((location.longitude - longitude) ** 2 +
                            (location.latitude - latitude) ** 2) ** 0.5
                if best_distance is None or distance < best_distance:
                    best_location = location
                    best_distance = distance

        return best_location

    def timeout(self):
        self.update_current_time()

        if self.point_selected is None:
            return True

        if self.point_selected.get_color_rgba() == SELECTED_1_RGBA:
            self.tzmap.point_set_color_rgba(self.point_selected,
                                            SELECTED_2_RGBA)
        else:
            self.tzmap.point_set_color_rgba(self.point_selected,
                                            SELECTED_1_RGBA)

        return True

    def mapped(self, widget, event):
        if self.update_timeout is None:
            self.update_timeout = gobject.timeout_add(100, self.timeout)

    def unmapped(self, widget, event):
        if self.update_timeout is not None:
            gobject.source_remove(self.update_timeout)
            self.update_timeout = None

    def motion(self, widget, event):
        if self.tzmap.get_magnification() <= 1.0:
            if self.cursor_zoom_in is not None:
                self.frontend.live_installer.window.set_cursor(
                    self.cursor_zoom_in)
        else:
            self.frontend.live_installer.window.set_cursor(None)

            (longitude, latitude) = self.tzmap.window_to_world(event.x,
                                                               event.y)

            if (self.point_hover is not None and
                self.point_hover != self.point_selected):
                self.tzmap.point_set_color_rgba(self.point_hover, NORMAL_RGBA)

            self.point_hover = self.tzmap.get_closest_point(longitude,
                                                            latitude, True)

            if self.point_hover != self.point_selected:
                self.tzmap.point_set_color_rgba(self.point_hover, HOVER_RGBA)

        return True

    def out_map(self, widget, event):
        if event.mode != gtk.gdk.CROSSING_NORMAL:
            return False

        if (self.point_hover is not None and
            self.point_hover != self.point_selected):
            self.tzmap.point_set_color_rgba(self.point_hover, NORMAL_RGBA)

        self.point_hover = None

        self.frontend.live_installer.window.set_cursor(None)

        return True

    def button_pressed(self, widget, event):
        (longitude, latitude) = self.tzmap.window_to_world(event.x, event.y)

        if event.button != 1:
            self.tzmap.zoom_out()
            if self.cursor_zoom_in is not None:
                self.frontend.live_installer.window.set_cursor(
                    self.cursor_zoom_in)
        elif self.tzmap.get_magnification() <= 1.0:
            self.tzmap.zoom_to_location(longitude, latitude)
            if self.cursor_zoom_in is not None:
                self.frontend.live_installer.window.set_cursor(None)
        else:
            if self.point_selected is not None:
                self.tzmap.point_set_color_rgba(self.point_selected,
                                                NORMAL_RGBA)
            self.point_selected = self.point_hover

            new_location_selected = \
                self.location_from_point(self.point_selected)
            if new_location_selected is not None:
                old_city = self.get_selected_tz_name()
                if old_city is None or old_city != new_location_selected.zone:
                    self.set_city_text(new_location_selected.zone)
                    self.set_zone_text(new_location_selected)
            self.location_selected = new_location_selected
            self.frontend.allow_go_forward(self.location_selected is not None)

        return True
