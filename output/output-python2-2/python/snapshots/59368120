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

import re
from re import escape, compile

import storage


class Trigger(object):
    _SPECIAL_CHARS = {'\\*': '.*?', '\\?': '.?', '\\%d': '(\d+)', '\\%w': '(\w+)'}

    def __init__(self, conn_name):
        trg = storage.triggers(conn_name)
        self._triggers = []
        self._highlights = []

        for pattern, ignore_case, command, bg_color, fg_color in trg:
            p = escape(pattern)
            for old, new in self._SPECIAL_CHARS.iteritems():
                p = p.replace(old, new)

            reg = compile(p, re.I if ignore_case else 0)
            if command:
                self._triggers.append((reg, command))
            else:
                self._highlights.append((reg, bg_color, fg_color))

    def getActions(self, main_text):
        """
        Return the list of action that match with the list of trigger pattern.
        """

        actions = []
        text = main_text.split('\n')
        for reg, command in self._triggers:
            for row in text:
                m = reg.search(row)
                if m:
                    for i, var in enumerate(m.groups()):
                        command = command.replace('%%%d' % (i + 1), var)
                    actions.append(command)

        return actions

    def highlights(self, model):
        """
        Replace main html of the model according with list of hightlight pattern.
        """

        _html_entities = ('&nbsp;', '&lt;', '&gt;', '&amp;', '&quot;', '<br>')

        def getHtmlIndex(html, start, end):
            reg = re.compile('(<span.*?>|</span>|%s)' %
                             '|'.join(_html_entities))
            m = reg.search(html)
            while m:
                s, e = m.span()
                if m.group(0) in _html_entities:
                    l = len(m.group(0)) - 1
                else:
                    l = e - s

                if s < start:
                    start += l
                if s < end:
                    end += l
                    m = reg.search(html, e)
                else:
                    break

            return (start, end)

        def replaceStyle(html, bg, fg):
            """
            Replace the style of an html-string with another based upon bg and
            fg colors.
            """

            colors = []
            if bg:
                colors.append(('background-color', bg))
            if fg:
                colors.append(('color', fg))

            new_html = '<span style="%s">' % \
                ';'.join([k + ':' + v for k, v in colors])
            new_html += compile('(<span.*?>|</span>)').sub('', html) + '</span>'

            span_open = compile('<span.*?>').findall(html)
            span_close = compile('</span>').findall(html)

            if len(span_open) > len(span_close):
                new_html += span_open[-1]
            elif len(span_open) < len(span_close):
                new_html += '</span>'
            elif html.find('</span') < html.find('<span'):
                new_html += '</span>' + span_open[-1]
            return new_html

        text = model.main_text.split('\n')
        html = model.main_html.split('<br>')

        for reg, bg, fg in self._highlights:
            for i, row in enumerate(text):
                m = reg.search(row)
                if m:
                    html_row = html[i]
                    start, end = getHtmlIndex(html_row, *m.span())

                    html[i] = html_row[:start] + \
                        replaceStyle(html_row[start:end], bg, fg) + \
                        html_row[end:]

        model.main_html = '<br>'.join(html)
