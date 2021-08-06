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

"""
:copyright: 2007 Gianni Valdambrini, Develer_ S.r.l.
:author: Gianni Valdambrini
:contact: gvaldambrini@develer.com

.. _Develer: http://www.develer.com/

.. digraph:: Overview of DevClient architecture

    ranksep = 0.1;
    node [ shape="box" height="0" width="0" style="filled"
           fillcolor="white" ]

    interface [ label="User interface" shape="ellipse" fillcolor="whitesmoke" ]

    {   node [ width="1.5" height="0" ]
        gui      [  href="<gui>"  fillcolor="olivedrab3" ]
        core     [ href="<core>" fillcolor="slategray2" ]
    }

    {   node [ width="1.2" ]
        parse    [ href="<parse>" fillcolor="slategray2" ]
        model    [ href="<model>" fillcolor="lightsalmon1" ]
        viewer   [ href="<viewer>" fillcolor="olivedrab3"]
    }

    {   node [ shape="ellipse" ]
        server   [ label="Mud server" fillcolor="snow3" ]
    }

    interface -> gui
    gui -> core    [ label=" msg" ]
    core -> server [ label=" msg" ]
    server -> core [ label=" data" ]
    core -> parse  [ label=" create" ]
    parse -> model [ label=" create" ]
    gui -> viewer  [ label=" create" ]
    viewer -> model  [ label=" read" ]


    { rank=max; server interface }
    { rank=max; model }
    { rank=same; viewer parse }
    { rank=min; gui core }


Package DevClient is the main package of client.
It is composed by two processes:

- the `core` that exchange data with server and process it to create a `model`.
- the `gui` that read a `model` and use it to show data.

All `messages` exchanged from `gui` and `core` are sent by non-blocking socket.

The data to/from server is sent/received in asyncronous mode, so when the user
type a message via `gui` the module send the message to `core`, that forward it
to server. On the other hand, when the server send a message to the client,
the `core` receive the message and transform it (with module `parse`) into a
`model` of data, that is sent to `gui`. The `gui` use the module `viewer` to
display data.
"""

__docformat__ = 'restructuredtext'
from constants import PUBLIC_VERSION as __version__
