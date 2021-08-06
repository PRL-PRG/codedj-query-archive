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
import shutil
import signal
import textwrap
from ubiquity.filteredcommand import FilteredCommand
from ubiquity.parted_server import PartedServer

class PartmanAuto(FilteredCommand):
    def __init__(self, frontend=None):
        super(PartmanAuto, self).__init__(frontend)
        self.resize_desc = ''
        self.manual_desc = ''

    def prepare(self):
        # If an old parted_server is still running, clean it up.
        if os.path.exists('/var/run/parted_server.pid'):
            try:
                pidline = open('/var/run/parted_server.pid').readline().strip()
                pid = int(pidline)
                os.kill(pid, signal.SIGTERM)
            except Exception:
                pass
            try:
                os.unlink('/var/run/parted_server.pid')
            except OSError:
                pass

        # Force autopartitioning to be re-run.
        shutil.rmtree('/var/lib/partman', ignore_errors=True)

        self.autopartition_question = None
        self.resize_allowed = True
        self.resize_min_percent = 0
        self.backup_from_new_size = None
        self.stashed_auto_mountpoints = None

        questions = ['^partman-auto/select_disk$',
                     '^partman-auto/.*automatically_partition$',
                     '^partman-partitioning/new_size$',
                     '^partman/choose_partition$',
                     '^partman/confirm.*',
                     'type:boolean',
                     'ERROR',
                     'PROGRESS']
        return ('/bin/partman', questions, {'PARTMAN_NO_COMMIT': '1'})

    def error(self, priority, question):
        self.frontend.error_dialog(self.description(question),
                                   self.extended_description(question))
        return super(PartmanAuto, self).error(priority, question)

    def parse_size(self, size_str):
        (num, unit) = size_str.split(' ', 1)
        if ',' in num:
            (size_int, size_frac) = num.split(',', 1)
        else:
            (size_int, size_frac) = num.split('.', 1)
        size = float(str("%s.%s" % (size_int, size_frac)))
        # partman measures sizes in decimal units
        if unit == 'B':
            pass
        elif unit == 'kB':
            size *= 1000
        elif unit == 'MB':
            size *= 1000000
        elif unit == 'GB':
            size *= 1000000000
        elif unit == 'TB':
            size *= 1000000000000
        return size

    def subst(self, question, key, value):
        if question == 'partman-partitioning/new_size':
            if key == 'MINSIZE':
                self.resize_min_size = self.parse_size(value)
            elif key == 'MAXSIZE':
                self.resize_max_size = self.parse_size(value)

    def error(self, priority, question):
        if question == 'partman-partitioning/impossible_resize':
            if self.resize_allowed:
                self.resize_allowed = False
                return False
        return super(PartmanAuto, self).error(priority, question)

    def run(self, priority, question):
        if self.stashed_auto_mountpoints is None:
            # We need to extract the automatic mountpoints calculated by
            # partman at some point while parted_server is running, so that
            # they can be used later if manual partitioning is selected.
            # This hack is only necessary because the manual partitioner is
            # NIHed rather than being based on partman.
            self.stashed_auto_mountpoints = {}
            parted = PartedServer()
            for disk in parted.disks():
                parted.select_disk(disk)
                for part in parted.partitions():
                    (p_num, p_id, p_size, p_type, p_fs, p_path, p_name) = part
                    if p_fs == 'free':
                        continue
                    if not parted.has_part_entry(p_id, 'method'):
                        continue
                    method = parted.readline_part_entry(p_id, 'method')
                    if method == 'swap':
                        continue
                    elif p_fs == 'hfs' and method == 'newworld':
                        self.stashed_auto_mountpoints[p_path] = 'newworld'
                    elif parted.has_part_entry(p_id, 'acting_filesystem'):
                        mountpoint = parted.readline_part_entry(p_id,
                                                                'mountpoint')
                        self.stashed_auto_mountpoints[p_path] = mountpoint
            self.frontend.set_auto_mountpoints(self.stashed_auto_mountpoints)

        if self.done:
            # user answered confirmation question or selected manual
            # partitioning
            return self.succeeded

        self.current_question = question

        try:
            qtype = self.db.metaget(question, 'Type')
        except debconf.DebconfError:
            qtype = ''

        if question == 'partman-auto/select_disk':
            self.resize_allowed = True
            self.manual_desc = \
                self.description('partman-auto/text/custom_partitioning')
            if not self.frontend.set_disk_choices(self.choices(question),
                                                  self.manual_desc):
                # disk selector not implemented; just use first disk
                return self.succeeded

        elif question.endswith('automatically_partition'):
            if self.backup_from_new_size is not None:
                # We backed up from the resize question and now need to
                # carry on down a different (preseeded) path.
                self.preseed(question, self.backup_from_new_size)
                self.backup_from_new_size = None
                self.succeeded = True
                return True

            self.autopartition_question = question
            self.resize_desc = \
                self.description('partman-auto/text/resize_use_free')
            self.manual_desc = \
                self.description('partman-auto/text/custom_partitioning')
            choices = self.choices(question)
            if not self.resize_allowed:
                try:
                    del choices[choices.index(self.resize_desc)]
                except ValueError:
                    pass
            self.frontend.set_autopartition_choices(
                choices, self.resize_desc, self.manual_desc)
            if self.resize_desc in choices:
                # The resize option is available, so we need to present the
                # user with an accurate resize slider before passing control
                # to the UI.
                # Don't preseed_as_c, because Perl debconf is buggy in that
                # it doesn't expand variables in the result of METAGET
                # choices-c. All locales have the same variables anyway so
                # it doesn't matter.
                self.preseed(question, self.resize_desc)
                return True

        elif question == 'partman-partitioning/new_size':
            self.backup_from_new_size = None
            self.frontend.set_autopartition_resize_bounds(self.resize_min_size,
                                                          self.resize_max_size)

        elif question.startswith('partman/confirm'):
            if question == 'partman/confirm':
                self.db.set('ubiquity/partman-made-changes', 'true')
            else:
                self.db.set('ubiquity/partman-made-changes', 'false')
            self.preseed(question, 'true')
            self.succeeded = True
            self.done = True
            return True

        elif qtype == 'boolean':
            response = self.frontend.question_dialog(
                self.description(question),
                self.extended_description(question),
                ('ubiquity/text/go_back', 'ubiquity/text/continue'))

            answer_reversed = False
            if (question == 'partman-jfs/jfs_boot' or
                question == 'partman-jfs/jfs_root'):
                answer_reversed = True
            if response is None or response == 'ubiquity/text/continue':
                answer = answer_reversed
            else:
                answer = not answer_reversed
            if answer:
                self.preseed(question, 'true')
            else:
                self.preseed(question, 'false')
            return True

        return super(PartmanAuto, self).run(priority, question)

    def ok_handler(self):
        if self.current_question == 'partman-auto/select_disk':
            disk_choice = self.frontend.get_disk_choice()
            # Don't preseed_as_c, because Perl debconf is buggy in that it
            # doesn't expand variables in the result of METAGET choices-c.
            # All locales have the same variables anyway so it doesn't
            # matter.
            if disk_choice is not None:
                self.preseed(self.current_question, disk_choice)
                if disk_choice == self.manual_desc:
                    self.succeeded = False
                    self.done = True
                else:
                    # Don't exit partman yet.
                    self.succeeded = True
                self.exit_ui_loops()
                return

        elif (self.current_question.endswith('automatically_partition') or
              self.current_question == 'partman-partitioning/new_size'):
            autopartition_choice = self.frontend.get_autopartition_choice()
            # Don't preseed_as_c, because Perl debconf is buggy in that it
            # doesn't expand variables in the result of METAGET choices-c.
            # All locales have the same variables anyway so it doesn't
            # matter.
            if self.autopartition_question is not None:
                self.preseed(self.autopartition_question, autopartition_choice)
            else:
                self.preseed('partman-auto/init_automatically_partition',
                             autopartition_choice)
                self.preseed('partman-auto/automatically_partition',
                             autopartition_choice)

            if autopartition_choice == self.manual_desc:
                # Back up all the way out.
                self.succeeded = False
                self.done = True
            else:
                if autopartition_choice == self.resize_desc:
                    # We're on the right path. Keep going.
                    percent = self.frontend.get_autopartition_resize_percent()
                    self.preseed(self.current_question, '%d%%' % percent)
                    self.succeeded = True
                elif self.current_question == 'partman-partitioning/new_size':
                    # We went forward to the resize question, but that
                    # turned out to be the wrong choice. Back up as far as
                    # the autopartitioning question, and then continue based
                    # on what the user selected for that.
                    self.backup_from_new_size = autopartition_choice
                    self.succeeded = False
                else:
                    # We're on the right path. Keep going.
                    self.succeeded = True
                # Don't exit partman yet.
            self.exit_ui_loops()
            return

        super(PartmanAuto, self).ok_handler()

# Notes:
#
#   partman-auto/init_automatically_partition
#     Resize <partition> and use freed space
#     Erase entire disk: <disk> - <description>
#     Manually edit partition table
#
#   may show multiple disks, in which case massage into disk chooser (later)
#
#   if the resize option shows up, then run os-prober and display at the
#   top?
#
#   resize follow-up question:
#       partman-partitioning/new_size
#   progress bar:
#       partman-partitioning/progress_resizing
#
#   manual editing:
#       partman/choose_partition
#
#   final confirmation:
#       partman/confirm*
