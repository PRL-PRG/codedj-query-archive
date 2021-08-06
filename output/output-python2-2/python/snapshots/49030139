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

import textwrap
from ubiquity.components.partman_commit import PartmanCommit

class Summary(PartmanCommit):
    def __init__(self, frontend, manual_partitioning=False):
        super(Summary, self).__init__(frontend, manual_partitioning, True)
        self.using_grub = False

    def prepare(self):
        prep = list(super(Summary, self).prepare())
        prep[0] = '/usr/share/ubiquity/summary'
        prep[1].append('^ubiquity/summary.*')
        return prep

    def metaget(self, question, field):
        if question == 'ubiquity/summary/grub':
            self.using_grub = True

    def run(self, priority, question):
        if question == 'ubiquity/summary':
            text = ''
            wrapper = textwrap.TextWrapper(width=76)
            for line in self.extended_description(question).split("\n"):
                text += wrapper.fill(line) + "\n"

            self.frontend.set_summary_text(text)
            if self.using_grub:
                # TODO cjwatson 2006-09-04: a bit inelegant, and possibly
                # Ubuntu-specific?
                self.frontend.set_summary_device('(hd0)')

            # This component exists only to gather some information and then
            # get out of the way.
            return True

        else:
            return super(Summary, self).run(priority, question)
