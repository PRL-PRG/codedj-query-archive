# -*- coding: UTF-8 -*-
# Copyright 2007-2008 One Laptop Per Child
# Copyright 2007 Gerard J. Cerchio <www.circlesoft.com>
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

import logging
_logger = logging.getLogger('PlayGo')

from gettext import gettext as _

class GoGame:
    """ This class administrates a go board.
        It keeps track of the stones currently on the board in the dictionary self.status,
        and of the moves played so far in self.undostack

        It has methods to clear the board, play a stone, undo a move. """

    def __init__(self, boardSize = 19):
        self.size = boardSize
        self.status = {}
        self.undostack = []
        self.score = {'B' : 0,  'W' : 0}
        _logger.setLevel( logging.DEBUG )

    def get_score(self):
        return self.score
        
    def increase_score(self, color):
        self.score[color] = self.score[color] + 1

    def neighbors(self,x):
        """ Returns the coordinates of the 4 (resp. 3 resp. 2 at the side 1 in the corner) intersections
            adjacent to the given one. """
        if   x[0]== 0                :     l0 = [1]
        elif x[0]== self.size-1 :     l0 = [self.size-2]
        else:                            l0 = [x[0]-1, x[0]+1]

        if   x[1]== 0                :     l1 = [1]
        elif x[1]== self.size-1 :     l1 = [self.size-2]
        else:                            l1 = [x[1]-1, x[1]+1]

        l = []
        for i in l0: l.append((i,x[1]))
        for j in l1: l.append((x[0],j))

        return l

    def is_occupied(self, x, y):
        return  self.status.has_key((x, y))
        
    def clear(self):
        """ Clear the board """
        self.status = {}
        self.undostack=[]     
        self.score = {'B' : 0,  'W' : 0}

    def play(self,pos,color):
        """ This plays a color=black/white stone at pos, if that is a legal move 
            and deletes stones captured by that move.
            It returns 1 if the move has been played, 0 if not. """
        if self.status.has_key(pos):                # check if empty
            return 0

        if self.legal(pos,color): # legal move?
            self.status[pos] = color
            captures = self.get_captures(pos, color)
            if captures:
                for x in captures: 
                    del self.status[x]   # remove captured stones, if any
                    self.increase_score(color)
            self.undostack.append((pos,color,captures))   # remember move + captured stones for easy undo
            return captures
        else: 
            return 0
    
    def get_captures(self,  pos,  color):
        """Returns a list of captured stones resulting from placing a color stone at pos """
        c = [] # captured stones
        
        for x in self.neighbors(pos):
            if self.status.has_key(x) and self.status[x]==self.invert(color):
                c = c + self.hasNoLibExcP(x, self.invert(color), pos)
                
        if c:
            captures = []
            for x in c:
                if not x in captures: captures.append(x)
            return captures
            
        return 0
        
    def checkKo(self,  pos, color):
        ''' Check if a move by color at pos would be a basic Ko infraction '''
        # Basically what we need to check, is if the current play would undo
        #   all that was done by the last entry in undostack (capture what was placed
        #   and place what was captured). 
        if self.undostack:
            lastpos,  lastcolor,  lastcaptures = self.undostack[-1]
            currentcaptures = self.get_captures(pos, color)
            if lastcaptures != 0 and currentcaptures != 0:
                if lastcolor != color and lastcaptures[0] == pos and lastpos == currentcaptures[0]:
                    return 1
        return 0
        
    def legal(self, pos, color):
        """ Check if a play by color at pos would be a legal move. """
        if self.status.has_key(pos):
            return 0
            
        # If the play at pos would leave that stone without liberties, we have two possibilities: 
        # 1- It's a capturing move
        # 2- It's an illegal move
        if self.hasNoLibExcP(pos, color): 
            # Check if it would capture any stones
            if self.get_captures(pos, color):
                return 1
            # It didnt, so I guess it's illegal
            return 0
        else: return not self.checkKo(pos, color)

    def illegal(self, x, y, color):
        """ Check if a play by color at pos would be an illigal move, and return pretty errors"""
        if self.status.has_key((x, y)):
            return _('There already is a stone there!')
        if self.checkKo((x, y), color):
            return _('Ko violation!')
        
        # If the play at pos would leave that stone without liberties, we have two possibilities: 
        # 1- It's a capturing move
        # 2- It's an illegal move
        if self.hasNoLibExcP((x, y), color): 
            # Check if it would capture any stones
            if self.get_captures((x, y), color):
                return False
            # It didnt, so I guess it's illegal
            return _('Illegal move.')
        else: return False

    def hasNoLibExcP(self, pos, color, exc = None):
        """ This function checks if the string (=solidly connected) of stones containing
            the stone at pos has a liberty (resp. has a liberty besides that at exc).
            If no liberties are found, a list of all stones in the string is returned.

            The algorithm is a non-recursive  implementation of a simple flood-filling:
            starting from the stone at pos, the main while-loop looks at the intersections
            directly adjacent to the stones found so far, for liberties or other stones that belong
            to the string. Then it looks at the neighbors of those newly found stones, and so
            on, until it finds a liberty, or until it doesn't find any new stones belonging
            to the string, which means that there are no liberties.
            Once a liberty is found, the function returns immediately. """
            
        st = []            # in the end, this list will contain all stones solidly connected to the
                           # one at pos, if this string has no liberties
        newlyFound = [pos] # in the while loop, we will look at the neighbors of stones in newlyFound
        foundNew = 1
        
        while foundNew:
            foundNew = 0
            n = []         # this will contain the stones found in this iteration of the loop
            for x in newlyFound:
                for y in self.neighbors(x):
                    if not self.status.has_key(y) and y != exc and y != pos:    # found a liberty
                        return []
                    elif self.status.has_key(y) and self.status[y]==color \
                        and not y in newlyFound and not y in st: # found another stone of same color
                        n.append(y)
                        foundNew = 1

            st[:0] = newlyFound
            newlyFound = n

        return st     # no liberties found, return list of all stones connected to the original one

    def undo(self, no=1):
        """ Undo the last no moves. """
        for i in range(no):
            if self.undostack:
                pos, color, captures = self.undostack.pop()
                del self.status[pos]
                if captures:
                    for p in captures: self.status[p] = self.invert(color)
                return True
            else:
                return False

    def invert(self,color):
        if color == 'B': return 'W'
        else: return 'B'
        
    def get_status(self):
        return self.status
