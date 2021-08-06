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

import exception

def getParser(server):
    """
    Build and return the appropriate `Parser` instance for the server.

    :Parameters:
        server : class
          the class of server connected
    """

    parser = Parser(server)

    if hasattr(server, 'prompt_reg') and hasattr(server, 'prompt_sep'):
        parser = PromptParser(parser)

    if hasattr(server, 'wild_map'):
        parser = WildMapParser(parser)

    return parser


class Model(object):
    """
    Rappresent a model of data that can be viewed by a viewer.
    """

    def __init__(self):
        self.main_text = []
        self.main_html = []
        self.bg_color = None
        self.fg_color = None
        self.wild_map = None
        self.prompt = None


class Parser(object):
    """
    The Parser class build a Model of data received.
    """

    _normal_color = ['000000', 'aa0000', '00aa00', 'aaaa00', '0000aa',
                     'aa00aa', '00aaaa', 'aaaaaa']

    _bright_color = ['444444', 'ff4444', '44ff44', 'ffff44', '4444ff',
                     'ff44ff', '44ffff', 'ffffff']

    def __init__(self, server):
        """
        Create the `Parser` instance.
        """

        self._incomplete_seq = None
        self._style = None
        self._default_bg = None
        self._default_fg = None
        self._bg_code = None
        self._fg_code = None
        self._server = server

    def buildModel(self, data):
        """
        Parse data and build the `Model` object.
        """

        model = Model()
        model.bg_color = self._default_bg
        model.fg_color = self._default_fg

        # spaces must be replace with html code before calling
        # _replaceAnsiColor to prevent replacing inside html tag.
        for t, r in (('\r', ''), (' ', '&nbsp;')):
            data = data.replace(t, r)

        html_data, text_data = self._replaceAnsiColor(data, model)
        html_data = html_data.split('\n')
        text_data = text_data.replace('&nbsp;', ' ').split('\n')

        for i, r in enumerate(text_data):
            model.main_text.append((r, r + '\n')[i < len(text_data) - 1])

        for i, r in enumerate(html_data):
            model.main_html.append((r, r + '<br>')[i < len(html_data) - 1])

        if model.bg_color is None and model.fg_color is None and \
           len(''.join(text_data).strip()):
            # Empty colors means default color
            model.bg_color = ''
            model.fg_color = ''

        if model.bg_color != self._default_bg:
            self._default_bg = model.bg_color

        if model.fg_color != self._default_fg:
            self._default_fg = model.fg_color

        return model

    def _evalStyle(self, ansi_code, model):

        attr = 0
        list_code = [int(c) for c in ansi_code.split(';')]

        for code in list_code:
            if 30 <= code <= 37:
                self._fg_code = code - 30
            elif 40 <= code <= 47:
                self._bg_code = code - 40
            elif code == 1:
                attr = 1

        style = []

        if self._fg_code is not None:
            if attr:
                color = self._bright_color[self._fg_code]
            else:
                color = self._normal_color[self._fg_code]

            if model.fg_color is None:
                model.fg_color = color

            style.append('color:#' + color)


        if self._bg_code is not None:
            color = self._normal_color[self._bg_code]

            if model.bg_color is None:
                model.bg_color = color
            elif color != model.bg_color:
                style.append('background-color:#' + color)

        return ';'.join(style)

    def _replaceAnsiColor(self, data, model):
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
                   Brown      33   #aaaa00           Yellow    1;33   #ffff44
        ----------------  ------  -------- ----------------  ------  --------
                    Blue      34   #0000aa       Light Blue    1;34   #4444ff
        ----------------  ------  -------- ----------------  ------  --------
                  Purple      35   #aa00aa     Light Purple    1;35   #ff44ff
        ----------------  ------  -------- ----------------  ------  --------
                    Cyan      36   #00aaaa       Light Cyan    1;36   #44ffff
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
        text_res = parts[0]
        if self._style:
            html_res = '<span style="%s">%s</span>' % \
                (self._style, parts[0])
        else:
            html_res = parts[0]

        if len(parts) == 1:
            return html_res, text_res

        reg = re.compile('\[(.*?)([%s])' % ''.join(ANSI_CODE), re.I)

        for i, s in enumerate(parts[1:]):
            m = reg.match(s)
            if m:
                ansi_code = m.group(1)
                code_length = len(ansi_code) + len(COLOR_TOKEN) + len('[')
                if m.group(2) == COLOR_TOKEN and ansi_code:
                    self._style = self._evalStyle(ansi_code, model)
                    if self._style and s[code_length:]: 
                        html_res += '<span style="%s">%s</span>' % \
                            (self._style, s[code_length:])
                    else:
                        html_res += s[code_length:]
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

    This class is a subclass of Parser that get an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, parser):
        super(PromptParser, self).__init__(parser._server)
        self._p = parser

    def _parsePrompt(self, model):
        reg = re.compile(self._p._server.prompt_reg, re.I)
        m = reg.findall(''.join(model.main_text))
        if m:
            p = list(m[-1])
            for i in xrange(3):
                p[i] = p[i].split(self._p._server.prompt_sep)
            model.prompt = {'Hp': p[0], 'Mn': p[1], 'Mv': p[2]}

    def buildModel(self, data):
        model = self._p.buildModel(data)
        self._parsePrompt(model)
        return model


class WildMapParser(Parser):
    """
    Parse data and build a model for wild.

    This class is a subclass of Parser that get an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, parser):
        super(WildMapParser, self).__init__(parser._server)
        self._incomplete_map = None
        self._incomplete_end_seq = ''
        self._p = parser

    def _getHtmlFromText(self, model, parts):
        html = ''.join(model.main_html)
        html = html.replace('&nbsp;', ' ').replace('<br>', '\n')
        html_parts = []
        for p in parts:
            p_html = ''
            length = len(p)

            while length > 0:
                if html.startswith('<span') or html.startswith('</span>'):
                    pos = html.find('>') + 1
                    p_html += html[:pos]
                    html = html[pos:]
                else:
                    if html[0] == ' ':
                        p_html += '&nbsp;'
                    elif html[0] == '\n':
                        p_html += '<br>'
                    else:
                        p_html += html[0]
                    html = html[1:]
                    length -= 1

            if html.startswith('</span>'):
                p_html += '</span>'

            html_parts.append(p_html)

        return html_parts

    def _parseWild(self, model):
        text = ''.join(model.main_text)
        if '[Uscite' in text:
            if self._incomplete_map:
                patt = '(.*?)([^0-9a-wy-z\[]*)\[Uscite:'
            else:
                patt = '(.*?\n)([^0-9a-wy-z\[]+)\[Uscite:'
            m = re.compile(patt, re.I|re.S).match(text)
            if m:
                if self._incomplete_map:
                    wild_map = self._incomplete_map + \
                        self._getHtmlFromText(model, m.groups())[1]
                    self._incomplete_map = None
                else:
                    wild_map = self._getHtmlFromText(model, m.groups())[1]
                model.wild_map = wild_map

        elif self._incomplete_end_seq:
           if text.startswith('[Uscite'[len(self._incomplete_end_seq):]):
               model.wild_map = self._incomplete_map
               self._incomplete_end_seq = ''
               self._incomplete_map = None
        else:
            reg = re.compile('(.*?\n)([^0-9a-wy-z\[]+)(.*)', re.I|re.S)
            m = reg.match(text)
            if m and m.groups()[-1].strip()[:7] in '[Uscite':
                groups = m.groups()[:-1]
                self._incomplete_end_seq = m.groups()[-1].strip()[:7]
                self._incomplete_map = self._getHtmlFromText(model, groups)[1]

    def buildModel(self, data):
        model = self._p.buildModel(data)
        self._parseWild(model)
        return model
