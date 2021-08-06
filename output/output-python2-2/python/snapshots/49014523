# -*- coding: UTF-8 -*-

# Copyright (C) 2006 Canonical Ltd.
# Written by Colin Watson <cjwatson@ubuntu.com>.
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import os
from ubiquity.filteredcommand import FilteredCommand

class Install(FilteredCommand):
    def prepare(self):
        hostname = self.frontend.get_hostname()
        if hostname is not None and hostname != '':
            self.preseed('netcfg/get_hostname', hostname)

        if os.access('/usr/share/grub-installer/grub-installer', os.X_OK):
            bootdev = self.frontend.get_summary_device()
            if bootdev is None or bootdev == '':
                bootdev = '(hd0)'
            self.preseed('grub-installer/with_other_os', 'false')
            self.preseed('grub-installer/only_debian', 'false')
            self.preseed('grub-installer/bootdev', bootdev)

        questions = ['^.*/apt-install-failed$',
                     'grub-installer/install_to_xfs',
                     'CAPB',
                     'ERROR',
                     'PROGRESS']
        return (['/usr/share/ubiquity/install.py'], questions)

    def capb(self, capabilities):
        self.frontend.debconf_progress_cancellable(
            'progresscancel' in capabilities)

    def error(self, priority, question):
        if question == 'hw-detect/modprobe_error':
            # don't need to display this, and it's non-fatal
            return True
        elif question == 'apt-setup/security-updates-failed':
            fatal = False
        else:
            fatal = True
        self.frontend.error_dialog(self.description(question),
                                   self.extended_description(question), fatal)
        if fatal:
            return super(Install, self).error(priority, question)
        else:
            return True

    def run(self, priority, question):
        if question.endswith('/apt-install-failed'):
            return self.error(priority, question)

        elif question == 'grub-installer/install_to_xfs':
            response = self.frontend.question_dialog(
                self.description(question),
                self.extended_description(question),
                ('ubiquity/text/go_back', 'ubiquity/text/continue'))
            if response is None or response == 'ubiquity/text/continue':
                self.preseed(question, 'true')
            else:
                self.preseed(question, 'false')
            return True

        return super(Install, self).run(priority, question)
