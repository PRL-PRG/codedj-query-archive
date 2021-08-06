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
from re import compile, escape

import exception
from messages import Model


def getParser(server, prompt):
    """
    Build and return the appropriate `Parser` instance for the server.

    :Parameters:
        server : class
          the class of server connected
        prompt : tuple
          the custom prompt
    """

    parser = Parser(server)

    if hasattr(server, 'prompt_reg') or [p for p in prompt if p]:
        parser = PromptParser(parser, prompt)

    if hasattr(server, 'wild_chars'):
        parser = WildMapParser(parser)

    return parser


class Parser(object):
    """
    The Parser class build a `Model` of data received.
    """

    _normal_color = ('000000', 'aa0000', '00aa00', 'aaaa00', '0000aa',
                     'aa00aa', '00aaaa', 'aaaaaa')

    _bright_color = ('444444', 'ff4444', '44ff44', 'ffff44', '4444ff',
                     'ff44ff', '44ffff', 'ffffff')

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

    def _checkDefaultColor(self, model, text):
        if model.bg_color is None and model.fg_color is None and \
           len(text.strip()):
            # Empty colors means default color
            model.bg_color, model.fg_color = '', ''

    def setVars(self, d):
        pass

    def buildModel(self, data):
        """
        Parse data and build the `Model` object.
        """

        model = Model()
        model.bg_color = self._default_bg
        model.fg_color = self._default_fg

        data = data.replace('\r', '')
        html_data, text_data = self._replaceAnsiColor(data, model)

        model.main_text = text_data
        model.main_html = html_data
        model.original_text = text_data
        self._checkDefaultColor(model, model.main_text)

        if model.bg_color != self._default_bg:
            self._default_bg = model.bg_color

        if model.fg_color != self._default_fg:
            self._default_fg = model.fg_color

        return model

    def _replaceTextChars(self, text):
        _text_to_html = (('&', '&amp;'), (' ', '&nbsp;'), ('<', '&lt;'),
                         ('>', '&gt;'), ('"', '&quot;'), ('\n', '<br>'))

        for k, v in _text_to_html:
            text = text.replace(k, v)
        return text

    def _evalStyle(self, ansi_code, model):

        attr = 0
        list_code = map(int, ansi_code.split(';'))

        if 0 in list_code:
            # reset all attributes
            self._fg_code = None
            self._bg_code = None

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

            style.append('color:#%s' % color)


        if self._bg_code is not None:
            color = self._normal_color[self._bg_code]

            if model.bg_color is None:
                model.bg_color = color
            elif color != model.bg_color:
                style.append('background-color:#%s' % color)

        return ';'.join(style)

    def _styleVisible(self, ansi_code, text):
        if not text:
            return False

        list_code = map(int, ansi_code.split(';'))
        # style might not visible only if text contains chars not printable
        if not text.strip():
            if (' ' in text or '\n' in text) and \
               ([el for el in list_code if el >= 40] or 0 in list_code):
                return True
            return False
        return True

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

        ANSI_CODE = [COLOR_TOKEN] + ANSI_CODE_UNSUPPORTED

        if self._incomplete_seq:
            data = self._incomplete_seq + data
            self._incomplete_seq = None

        style = self._style
        parts = data.split(START_TOKEN)
        html_part = self._replaceTextChars(parts[0])
        html_res = ['<span style="%s">%s</span>' % (style, html_part) if style
                    else html_part]

        if len(parts) == 1:
            return ''.join(html_res), parts[0]

        text_res = [parts[0]]
        self._checkDefaultColor(model, parts[0])
        reg = compile('\[(.*?)([%s])' % ''.join(ANSI_CODE), re.I)

        for i, s in enumerate(parts[1:]):
            m = reg.match(s)
            if m:
                ansi_code = m.group(1)
                code_length = len(ansi_code) + len(COLOR_TOKEN) + len('[')
                html_part = self._replaceTextChars(s[code_length:])
                if m.group(2) == COLOR_TOKEN and ansi_code:
                    # evalStyle must be always called to set color attributes
                    style = self._evalStyle(ansi_code, model)
                    if style and self._styleVisible(ansi_code, s[code_length:]):
                        html_res.append('<span style="%s">%s</span>' %
                                         (style, html_part))
                    else:
                        html_res.append(html_part)
                else:
                    html_res.append(html_part)

                text_res.append(s[code_length:])
            else:
                # i == len() - 2 is the last element of list because the loop
                # starts at second element
                if i == len(parts) - 2:
                    self._incomplete_seq = START_TOKEN + s
                else:
                    html_res.append(self._replaceTextChars(s))
                    text_res.append(s)

        self._style = style

        return ''.join(html_res), ''.join(text_res)


class PromptParser(Parser):
    """
    Parse data and build a model for prompt.

    This class is a subclass of Parser that get an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, parser, custom_prompt=[]):
        super(PromptParser, self).__init__(parser._server)
        self._custom_prompt = [p for p in custom_prompt if p]
        self._p = parser

    def _parsePrompt(self, model):
        reg = compile(self._p._server.prompt_reg, re.I)
        m = reg.findall(model.main_text)
        if m:
            p = list(m[-1])
            for i in xrange(3):
                p[i] = p[i].split(self._p._server.prompt_sep)
            model.prompt = {'Hp': p[0], 'Mn': p[1], 'Mv': p[2]}

    def _parseCustomPrompt(self, model):
        prompts = []
        for i, p in enumerate(self._custom_prompt):
            p = escape(p).replace('\*', '.*?')
            for c in 'hHmMvV':
                p = p.replace('\%' + c, '(\d+)')
            m = compile('(.*)%s' % p, re.S).match(model.main_text)
            if m:
                prompts.append((len(m.group(1)), i, list(m.groups())[1:]))

        if prompts:
            i, val = max(prompts)[1:]  # take the last prompt
            key = compile('%[hHmMvV]').findall(self._custom_prompt[i])
            d = dict(zip(key, val))
            model.prompt = {'Hp': (d['%h'], d['%H']),
                            'Mn': (d['%m'], d['%M']),
                            'Mv': (d['%v'], d['%V'])}

    def setVars(self, d):
        self._p.setVars(d)
        if 'custom_prompt' in d:
            self._custom_prompt = [p for p in d['custom_prompt'] if p]

    def buildModel(self, data):
        model = self._p.buildModel(data)
        if self._custom_prompt:
            self._parseCustomPrompt(model)
        else:
            self._parsePrompt(model)
        return model


class WildMapParser(Parser):
    """
    Parse data and build a model for wild.

    This class is a subclass of Parser that get an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    _html_entities = ('&nbsp;', '&lt;', '&gt;', '&amp;', '&quot;', '<br>')

    _html_tags = _html_entities + ('<span', '</span>')

    def __init__(self, parser):
        super(WildMapParser, self).__init__(parser._server)
        self._incomplete_map = []
        self._p = parser

    def _getHtmlFromText(self, html, parts):

        def findHtmlTags(html):
            """Find the nearest html-tag and return its position"""
            min_pos = len(html)
            for e in self._html_tags:
                p = html.find(e)
                if p != -1:
                    min_pos = min(p, min_pos)
            return min_pos

        def startsWithEntities(html):
            """Check if html starts with an html-entity and return its length"""
            for e in self._html_entities:
                if html.startswith(e):
                    return len(e)
            return 0

        html_parts = []
        span = ''
        for p in parts:
            p_html = [span] if span else []
            while p:
                if html.startswith('</span>') and span or \
                   html.startswith('<span'):
                    pos = html.find('>') + 1
                    span = html[:pos] if html.startswith('<span') else ''
                    p_html.append(html[:pos])
                    html = html[pos:]
                else:
                    pos = startsWithEntities(html)
                    if pos:
                        p_html.append(html[:pos])
                        html = html[pos:]
                        p = p[1:]
                    else:
                        pos = findHtmlTags(html)
                        p_html.append(html[:pos])
                        html = html[pos:]
                        p = p[pos:]

            if html.startswith('</span>') or span:
                p_html.append('</span>')
                if html.startswith('</span>'):
                    html = html[7:]
                    span = ''

            html_parts.append(''.join(p_html))

        if span:
            html = span + html
        if html:
            html_parts.append(html)
        return html_parts

    def _parseWild(self, model):

        def endswith(text, end):
            """Check if text finishes with 'end' string or a part of it"""

            if not text:
                return True

            t = text[-len(end):]
            while t:
                if end.startswith(t):
                    return True
                t = t[1:]
            return False

        def precChar(c):
            """Check if the char is a char that might be come before wild"""
            if hasattr(self._p._server, 'wild_prec_char'):
                return c in self._p._server.wild_prec_char

            return True

        def extractIncompleteMap(text, html):
            """Return text, html and the incomplete wild map if exist."""

            _text, _map = text, ''
            m = compile('(.*?)(\s[%s]*)$' % wild_chars, re.S).match(text)
            if m and precChar(m.group(1)[-1:]):
                _text, _map = m.group(1), text[len(m.group(1)):]
            else:
                patt = '(.*?)(\s[%s]{8,})' % wild_chars
                m = compile(patt, re.S).match(text)
                if m and m.group(2).strip() and precChar(m.group(1)[-1:]) and \
                   endswith(text, wild_end):
                    _text, _map = m.group(1), text[len(m.group(1)):]

            if _map:
                parts = self._getHtmlFromText(html, (_text, _map))
                return (_text, parts[0], [_map, parts[1]])

            return (text, html, [])

        # to save readability
        text, html = model.main_text, model.main_html

        wild_chars = self._p._server.wild_chars
        wild_end = self._p._server.wild_end_text
        if hasattr(self._p._server, 'room_end_text'):
            room_end = self._p._server.room_end_text
        else:
            room_end = wild_end

        # The incomplete map, came from previous step, is attached at the start
        # of the string to simulate an unique string.
        if self._incomplete_map:
            text = self._incomplete_map[0] + text
            html = self._incomplete_map[1] + html
            self._incomplete_map = []

        # wild end text must contain at least one char that not is contained
        # into room description.
        room_desc = '\w\s\.\'",:;\(\)'
        reg = compile('(.*?\s)([%s]{8,})[%s]*?%s' %
                      (wild_chars, room_desc, escape(wild_end)), re.S)

        if hasattr(self._p._server, 'room_map'):
            room_desc += self._p._server.room_map

        m = reg.match(text)
        if m:
            groups = list(m.groups())
            if groups[0][-1:] == ' ':
                groups[1] = ' ' + groups[1]  # to save alignment
                groups[0] = groups[0][:-1]

            model.map_text = groups[1]
            pos_start = len(groups[0])
            pos_end = pos_start + len(groups[1])
            parts = self._getHtmlFromText(html, groups)
            model.map_html = parts[1]

            # extract wild map from main text
            model.main_text = text[:pos_start] + text[pos_end:]
            model.main_html = parts[0] + parts[2]
            return True

        elif not model.map_text and \
             compile('.*?[%s]*?%s' % (room_desc, escape(room_end)),
                     re.S).match(text):
            model.map_text, model.map_html = None, None

        model.main_text, model.main_html, self._incomplete_map = \
            extractIncompleteMap(text, html)
        return False

    def setVars(self, d):
        self._p.setVars(d)

    def buildModel(self, data):
        model = self._p.buildModel(data)
        while self._parseWild(model):
            pass
        return model
