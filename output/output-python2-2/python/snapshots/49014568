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
from ubiquity.filteredcommand import FilteredCommand
from ubiquity.parted_server import PartedServer

class PartmanCommit(FilteredCommand):
    def __init__(self, frontend=None, manual_input=False, get_summary=False):
        super(PartmanCommit, self).__init__(frontend)
        self.manual_input = manual_input
        self.get_summary = get_summary

    def prepare(self):
        questions = ['^partman/confirm.*',
                     'type:boolean',
                     'ERROR',
                     'PROGRESS']
        if self.manual_input:
            env = {'PARTMAN_UPDATE_BEFORE_COMMIT': '1'}
        else:
            env = {}
        return ('/bin/partman-commit', questions, env)

    def error(self, priority, question):
        self.frontend.error_dialog(self.description(question),
                                   self.extended_description(question))
        self.succeeded = False
        # Unlike a normal error handler, we want to force exit.
        self.done = True
        return True

    # This is, uh, an "inventive" way to spot when partman-commit has
    # finished starting up parted_server so that we can feed information
    # into it.
    def progress_stop(self, progress_title):
        ret = super(PartmanCommit, self).progress_stop(progress_title)

        if (progress_title == 'partman/progress/init/title' and
            self.manual_input):
            partitions = {}
            parted = PartedServer()
            for disk in parted.disks():
                parted.select_disk(disk)
                for part in parted.partitions():
                    (p_num, p_id, p_size, p_type, p_fs, p_path, p_name) = part
                    partitions[p_path] = (disk, p_id)

            mountpoints = self.frontend.get_mountpoints()
            for device in partitions:
                (disk, p_id) = partitions[device]
                parted.select_disk(disk)
                if device in mountpoints:
                    (path, format, fstype, flags) = mountpoints[device]
                    if path == 'swap':
                        parted.write_part_entry(p_id, 'method', 'swap\n')
                        if format:
                            parted.write_part_entry(p_id, 'format', '')
                        else:
                            parted.remove_part_entry(p_id, 'format')
                        parted.remove_part_entry(p_id, 'use_filesystem')
                    else:
                        if (fstype == 'hfs' and
                            flags is not None and 'boot' in flags):
                            parted.write_part_entry(
                                p_id, 'method', 'newworld\n')
                            parted.write_part_entry(
                                p_id, 'filesystem', 'newworld')
                            parted.remove_part_entry(p_id, 'format')
                            path = None
                        elif format:
                            parted.write_part_entry(p_id, 'method', 'format\n')
                            parted.write_part_entry(p_id, 'format', '')
                            if fstype is None:
                                if parted.has_part_entry(
                                        p_id, 'detected_filesystem'):
                                    fstype = parted.readline_part_entry(
                                        p_id, 'detected_filesystem')
                                else:
                                    # TODO cjwatson 2006-09-27: Why don't we
                                    # know the filesystem type? Fortunately,
                                    # we have an explicit indication from
                                    # the user that it's OK to format this
                                    # filesystem.
                                    fstype = 'ext3'
                            parted.write_part_entry(p_id, 'filesystem', fstype)
                            parted.remove_part_entry(p_id, 'options')
                            parted.mkdir_part_entry(p_id, 'options')
                        else:
                            parted.write_part_entry(p_id, 'method', 'keep\n')
                            parted.remove_part_entry(p_id, 'format')
                            if fstype is not None:
                                parted.write_part_entry(
                                    p_id, 'detected_filesystem', fstype)
                        parted.write_part_entry(p_id, 'use_filesystem', '')
                        if path is not None:
                            parted.write_part_entry(p_id, 'mountpoint', path)
                        else:
                            parted.remove_part_entry(p_id, 'mountpoint')
                elif (parted.has_part_entry(p_id, 'method') and
                      parted.readline_part_entry(p_id, 'method') == 'newworld'):
                    # Leave existing newworld boot partitions alone.
                    pass
                else:
                    parted.remove_part_entry(p_id, 'method')
                    parted.remove_part_entry(p_id, 'format')
                    parted.remove_part_entry(p_id, 'use_filesystem')

        return ret

    def run(self, priority, question):
        if self.done:
            return self.succeeded

        try:
            qtype = self.db.metaget(question, 'Type')
        except debconf.DebconfError:
            qtype = ''

        if question.startswith('partman/confirm'):
            if question == 'partman/confirm':
                self.db.set('ubiquity/partman-made-changes', 'true')
            else:
                self.db.set('ubiquity/partman-made-changes', 'false')
            # If we're being run to get the partitioning summary, then stop
            # here.
            if self.get_summary:
                self.preseed(question, 'false')
                self.succeeded = False
                self.done = True
            else:
                self.preseed(question, 'true')
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
                self.succeeded = False
                self.done = True
                self.frontend.return_to_autopartitioning()
            if answer:
                self.preseed(question, 'true')
            else:
                self.preseed(question, 'false')
            return True

        else:
            return super(PartmanCommit, self).run(priority, question)
