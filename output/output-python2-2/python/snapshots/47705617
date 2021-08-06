# -*- coding: UTF-8 -*-
# Copyright 2007-2008 One Laptop Per Child
# Copyright 2008 Andrés Ambrois <andresambrois@gmail.com>
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

from subprocess import Popen, PIPE
import logging

from os.path import exists, join, abspath
from os import pathsep, environ
from string import split

logger = logging.getLogger('PlayGo')

def search_for_gnugo():
    paths = split(environ['PATH'], pathsep)
    for path in paths:
        if exists(join(path, 'gnugo')):
            return abspath(join(path, 'gnugo'))
    return False

class gnugo:
    ''' A wrapper for talking to gnugo over GTP '''
    def __init__(self, boardsize=19, handicap=0, komi=5.5, level=3):
        ''' Start the gnugo subprocess '''
        self.size = boardsize
        try: 
            self.gnugo = Popen(['gnugo', '--mode', 'gtp', '--boardsize', str(boardsize),
                                '--handicap', str(handicap), '--komi', str(komi), '--level', str(level) ], 
                                stdout=PIPE, stdin=PIPE)
        except:
            logger.error('Could not start gnugo subprocess')
        else:
            logger.debug('Successfuly loaded gnugo!')
            self.stdin = self.gnugo.stdin
            self.stdout = self.gnugo.stdout
    
    def __del__(self):
        logger.debug('Closing gnugo')
        self.stdin.write('quit \n')
        self.stdin.flush()
    
    def _xy_to_coords(self, x, y):
        return dict(zip(range(25), 'ABCDEFGHJKLMNOPQRSTUVWXYZ'))[x] + str(self.size - y)
        
    def _coords_to_xy(self, coords):
        return int(dict(zip('ABCDEFGHJKLMNOPQRSTUVWXYZ', range(25)))[coords[0]]), self.size - int(coords[1:])
        
    def short_to_long_colors(self, short_color):
        if short_color == 'B':
            return 'black'
        return 'white'
    
    def make_play(self, color, x, y):
        color = self.short_to_long_colors(color)
        self.stdin.write('play %s %s\n' % (color, self._xy_to_coords(x, y)))
        self.stdin.flush()
        logger.debug('Sent play by %s at %s to gnugo', color, self._xy_to_coords(x, y))
        output = self.stdout.readline()
        self.stdout.readline()
        if output[0] == '?':
            return False
        return True
    
    def get_move(self, color):
        color = self.short_to_long_colors(color)
        self.stdin.write('kgs-genmove_cleanup %s\n' % color)
        self.stdin.flush()
        output = self.stdout.readline()
        self.stdout.readline()
        if output[0] == '?':
            # FIXME: Handle error
            return False
        elif output[2:] == 'PASS\n':
            return -1, -1
        logger.debug('Generated move %s', output[2:])
        return self._coords_to_xy(output[2:])

    def pass_move(self, color):
        color = self.short_to_long_colors(color)
        self.stdin.write('%s pass\n' % color)
        self.stdin.flush()
        self.stdout.readline()
        self.stdout.readline()

    def undo(self):
        self.stdin.write('undo\n')
        self.stdin.flush()
        self.stdout.readline()
        self.stdout.readline()
        
    def clear(self):
        self.stdin.write('clear_board\n')
        self.stdin.flush()
        self.stdout.readline()
        self.stdout.readline()
        
    def dump_board(self):
        self.stdin.write('showboard\n')
        self.stdin.flush()
        output = ''
        for i in range(0, self.size+4):
            output = output + self.stdout.readline()
        return output
