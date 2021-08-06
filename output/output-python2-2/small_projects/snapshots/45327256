""" LetterCounter: Represent Base 26 as letters of the alphabet

    Copyright (c) Nick Murdoch, 2006, 2007

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
"""
import string

class LetterIterator(object):
    """ Iterates over letters A -> Z, AA -> AZ, BA -> BZ, etc.
    """
    def __init__(self, start='A', end=None, step=1, letters=string.uppercase):
        self.started = False
        self.current = start
        self.end = None
        if end:
            self.end = end
        self.step = step
        self.letters = letters

    def __iter__(self):
        return self
        
    def cycle_character(self, c):
        try:
            if c == self.letters[-1]:
                return self.letters[0]
            return self.letters[self.letters.index(c)+1]
        except ValueError, e:
            raise ValueError(str(e) + ": %c is not in %s" % (c, self.letters))

    def _next(self):
        # Edge cases:
        if self.current == self.end:
            raise StopIteration
        
        # Normal behaviour:
        for i in range(1, len(self.current)+2):
            if i <= len(self.current):
                self.current = self.current[:len(self.current)-i] \
                             + self.cycle_character(self.current[-i]) \
                             + self.current[len(self.current)-i+1:]
                if self.current[-i] != self.letters[0]:
                    # letter was not rolled back to A; we're done.
                    break
            else:
                # We've rolled back all the letters, need another column.
                self.current = self.letters[0] + self.current
        return self.current

    def next(self):
        # First iteration should return first number regardless of step
        if not self.started:
            self.started = True
            return self.current

        for i in range(self.step):
            next = self._next()
        return next
    

    def __cmp__(self, other):
        """ Compare to another LetterIterator and return which is higher
            according to LetterIterator logic.
            The two LetterIterators must use the same letters string.
        """
        assert self.letters == other.letters
        if len(self.current) > len(other.current):
            return 1
        elif len(self.current) < len(other.current):
            return -1
        else:
            for pos in range(len(self.current)):
                if self.letters.index(self.current[pos]) > other.letters.index(other.current[pos]):
                    return 1
                elif self.letters.index(self.current[pos]) < other.letters.index(other.current[pos]):
                    return -1
                else:
                    continue
            else:
                return 0


class LetterCounter(object):
    """ Letter Counter: Represent positive integers as letters
        A=0, B, C, ... Z, BA, BB, BC, ... 
        
        A is used as a 0, so AAAAAB is the same as B. For more
        common usage, use LetterIterator
    """

    letters = string.uppercase
    base = 26

    def __init__(self, initial=""):
        self.value = initial
    
    def __repr__(self):
        return 'LetterCounter("'+self.unpad()+'")'
    
    def __str__(self):
        return self.value
    
    def __cmp__(self, other):
        # match lengths of self.value and other.value

        val1 = self.pad(max(len(self.value), len(other.value)))
        val2 = val2.pad(max(len(self.value), len(other.value)))

        if val1 < val2:
            return -1

        if val1 == val2:
            return 0

        if val1 > val2:
            return 1
    
    
    def __nonzero__(self):
        for c in self.value:
            if c != 'A':
                return True
        return False
    
    def __setattr__(self, name, value):
        if name == "value":
            object.__setattr__(self, name, value.upper())
    
    def __getattr__(self, name):
        if name == "value":
            return self.unpad()


    def __add__(self, other):
        return LetterCounter(self.fromInt(int(self) + int(other)))
    
    def __sub__(self, other):
        return LetterCounter(self.fromInt(int(self) - int(other)))
    
    def __mul__(self, other):
        return LetterCounter(self.fromInt(int(self) * int(other)))
    
    def __div__(self, other):
        return LetterCounter(self.fromInt(int(self) / int(other)))
    
    def __mod__(self, other):
        return LetterCounter(self.fromInt(int(self) % int(other)))
    
    def __pow__(self, other):
        return LetterCounter(self.fromInt(pow(int(self), int(other))))
    
    def __int__(self):
        ret = 0
        r = range(len(self.value))
        r.reverse()
        for v in r:
            pos = len(self.value) - 1 - v
            ret += ( ord(self.value[pos]) - ord('A') ) * self.base**v
        return int(ret) # don't return Long if possible

    def fromInt(cls, value):
        if value < 0:
            raise ValueError("value must be >= 0")
        i = 7
        s = ""
        while i >= 0:
            d = value/(cls.base**i)
            if d > 25:
                raise ValueError("value must be <= 208827064575")
            s += cls.letters[d]
            value = value - d*(cls.base**i)
            i -= 1
        return s
    fromInt = classmethod(fromInt)
    

    def pad(self, padding):
        v = self.value
        while len(v) < padding:
            v = "A"+v
        return v
    
    def unpad(self):
        v = self.value
        while v.startswith('A'):
            v = v[1:]
        return v
