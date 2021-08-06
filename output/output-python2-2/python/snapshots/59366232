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

from model import Model

class Parser(object):
    """
    The Parser class build a Model of data received.
    """

    _normal_color = ['000000', 'aa0000', '00aa00', 'aaaa00', '0000aa',
                     'aa00aa', '00aaaa', 'aaaaaa']

    _bright_color = ['444444', 'ff4444', '44ff44', 'ffff44', '4444ff',
                     'ff44ff', '44ffff', 'ffffff']

    def __init__(self):
        """
        Create the `Parser` instance.
        """

        self.model = Model()  #: the `Model` instance.
        self._incomplete_seq = None

    def parse(self, data):
        """
        Parse data and build the `Model` object.
        """

        # spaces must be replace with html code before calling
        # _replaceAnsiColor to prevent replacing inside html tag.
        for t, r in (('\r', ''), (' ', '&nbsp;')):
            data = data.replace(t, r)

        html_data, text_data = self._replaceAnsiColor(data)

        html_data = html_data.split('\n')
        text_data = text_data.replace('&nbsp;', ' ').split('\n')

        for i, r in enumerate(text_data):
            self.model.main_text.append((r, r + '\n')[i < len(text_data) - 1])

        for i, r in enumerate(html_data):
            self.model.main_html.append((r, r + '<br>')[i < len(html_data) - 1])

    def _evalStyle(self, ansi_code):

        attr = 0
        fg = None
        bg = None

        list_code = [int(c) for c in ansi_code.split(';')]

        for code in list_code:
            if 30 <= code <= 37:
                fg = code - 30
            elif 40 <= code <= 47:
                bg = code - 40
            elif code == 1:
                attr = 1

        style = []

        if fg is not None:
            if attr:
                color = self._bright_color[fg]
            else:
                color = self._normal_color[fg]

            if self.model.main_fgcolor is None:
                self.model.main_fgcolor = color
            elif color != self.model.main_fgcolor:
                style.append('color:#' + color)

        if bg is not None:
            color = self._normal_color[bg]
            if self.model.main_bgcolor is None:
                self.model.main_bgcolor = color
            elif color != self.model.main_bgcolor:
                style.append('background-color:#' + color)

        return ';'.join(style)

    def _replaceAnsiColor(self, data):
        """
        Replace ansi color code with equivalent html color.

        The following table show the conversion rules between ansi and html
        color code:

        ================  ======  ======== ================  ======  ========
        **Normal Color**  *Ansi*    *Html* **Bright color**  *Ansi*    *Html*
        ----------------  ------  -------- ----------------  ------  --------
                   Black      30   #000000        Dark Gray    1;30   #444444
        ----------------  ------  -------- ----------------  ------  --------
                     Red      31   #aa0000        Light Red    1;31   #ff4444
        ----------------  ------  -------- ----------------  ------  --------
                   Green      32   #00aa00      Light Green    1;32   #44ff44
        ----------------  ------  -------- ----------------  ------  --------
                    Blue      33   #0000aa       Light Blue    1;33   #4444ff
        ----------------  ------  -------- ----------------  ------  --------
                    Cyan      34   #00aaaa       Light Cyan    1;34   #44ffff
        ----------------  ------  -------- ----------------  ------  --------
                  Purple      35   #aa00aa     Light Purple    1;35   #ff44ff
        ----------------  ------  -------- ----------------  ------  --------
                   Brown      36   #aaaa00           Yellow    1;36   #ffff44
        ----------------  ------  -------- ----------------  ------  --------
              Light Gray      37   #aaaaaa            White    1;37   #ffffff
        ================  ======  ======== ================  ======  ========

        :return: a pair of (html_data, text_data)
        """

        START_TOKEN = chr(27)
        COLOR_TOKEN = 'm'
        ANSI_CODE_UNSUPPORTED = ['H', 'f', 'A', 'B', 'C', 'D', 'R', 's', 'u',
                                 'J', 'K', 'h', 'l', 'p']

        ANSI_CODE = [COLOR_TOKEN]
        ANSI_CODE.extend(ANSI_CODE_UNSUPPORTED)

        if self._incomplete_seq:
            data = self._incomplete_seq + data
            self._incomplete_seq = None

        parts = data.split(START_TOKEN)

        if len(parts) == 1:
            return parts[0], parts[0]

        html_res = text_res = parts[0]
        reg = re.compile('\[(.*?)([%s])' % ''.join(ANSI_CODE), re.I)

        for i, s in enumerate(parts[1:]):
            m = reg.match(s)
            if m:
                ansi_code = m.group(1)
                code_length = len(ansi_code) + len(COLOR_TOKEN) + len('[')
                if m.group(2) == COLOR_TOKEN and ansi_code:
                    style = self._evalStyle(ansi_code)
                    if style:
                        html_res += '<span style="%s">%s</span>' % \
                            (style, s[code_length:])
                        text_res += s[code_length:]
                    else:
                        html_res += s[code_length:]
                        text_res += s[code_length:]
                else:
                    html_res += s[code_length:]
                    text_res += s[code_length:]
            else:
                # i == len() - 2 is the last element of list because the loop
                # starts at second element
                if i == len(parts) - 2:
                    self._incomplete_seq = START_TOKEN + s
                else:
                    html_res += s
                    text_res += s

        return html_res, text_res


class PromptParser(Parser):
    """
    Parse data and build a model for prompt.

    This class is an abstract template class (see `template pattern`_) that
    parse prompt using hook methods to build a model of it.

    Concrete subclass must override:

    - `_getRegExpPrompt` to define the regular expression that matches with
      prompt
    - `_getSepPrompt` to define separator from min and max values in Hp, Mn, Mv

.. _template pattern: http://en.wikipedia.org/wiki/Template_method_pattern
    """


    def __init__(self):
        super(PromptParser, self).__init__()
        self.last_row = None

    def _getRegExpPrompt(self):
        raise NotImplementedError

    def _getSepPrompt(self):
        raise NotImplementedError

    def _parsePrompt(self):
        new_text = self.model.main_text.get(self.last_row)
        if self.last_row is None:
            self.last_row = len(new_text) - 1
        else:
            self.last_row += len(new_text)

        text = '\n'.join(new_text)
        reg = self._getRegExpPrompt()
        m = reg.findall(text)
        if m:
            p = list(m[-1])
            for i in xrange(3):
                p[i] = p[i].split(self._getSepPrompt())
            self.model.prompt = {'Hp': p[0], 'Mn': p[1], 'Mv': p[2]}

    def parse(self, data):
        super(PromptParser, self).parse(data)
        self._parsePrompt()


class SmaugParser(PromptParser):
    """
    Parse data and build a model specific for Smaug MUD's
    """

    def _getRegExpPrompt(self):
        return re.compile('Pf:\s*(\d+/\d+) Mn:\s*(\d+/\d+) Mv:\s*(\d+/\d+)' +
                          '.*?\>', re.I)

    def _getSepPrompt(self):
        return '/'


class AfkParser(PromptParser):
    """
    Parse data and build a model specific for AFK MUD's
    """

    def _getRegExpPrompt(self):
        return re.compile('\[Pf:\s*(\d+-\d+)\] \[Mana:\s*(\d+-\d+)\] ' +
                          '\[Mv:\s*(\d+-\d+)\] \[Mon:\s*\d+\] ' +
                          '\[S:\s*Xp:\s*\d+\]', re.I)

    def _getSepPrompt(self):
        return '-'
