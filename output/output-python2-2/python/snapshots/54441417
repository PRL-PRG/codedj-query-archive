#!/usr/bin/python2.5
#
# Copyright (c) 2008 JT Olds
# http://www.jtolds.com/
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.

"""This module holds classes that fit into the Tube object.
  The Cell object is the parent object of Viruses and Pills. Viruses and Pills,
  of course, are elements inside the Tube during a Viricide game, but the
  Cell object is where their commonalities are implemented. The Cell object is
  never expected to be used by itself.

  This module also holds all the logic for virus placement.
  """

import random,sys,time,math

__author__ = "JT Olds"

COLORS = ["red","green","blue"]

# these are arguments for virus placement
MIN_CONCENTRATION_THRESHOLD = .1
MIN_ROW_LIMIT_PERCENT = .5
CONCENTRATION_THRESHOLD_INCREASE_FACTOR = 5.0

MAX_PLACEMENT_SECONDS = 3 #hack, until a better placement algorithm is used


class Error_(Exception): pass
class LogicError(Error_): pass
class VirusPlacementError(LogicError): pass
class ComboPlacementError(LogicError): pass


class Cell(object):
  """Cell class
    For use with Viricide implementations
    This is the parent class for Viruses and Pills"""

  def __init__(self,color=None):
    """On cell creation, the cell needs to know what color it is."""
    if color == None:
      color = COLORS[random.randint(0,len(COLORS)-1)]
    self.color = color

class Pill(Cell):
  """Pill class
    For use with Viricide implementations
    This Pill class holds just one half of a whole pill. In other words,
    This class only maintains the square cells that are always a uniform
    color.
    Both Pills and Viruses don't do much themselves."""
  pass

class Virus(Cell):
  """Virus class
    For use with Viricide implementations
    Both Pills and Viruses don't do much themselves."""
  pass

class VirusPlacer(object):
  """Places virus_number viruses into a cell array. Viruses are placed with
    respect to rows and cols, and are not placed in such a way as to make rows
    or columns of consecutive viruses of the same color exceeding half of the
    combo length (rounded down). Viruses are preferentially placed nearer the
    bottom, but are placed to maintain an even concentration over the rows they
    are placed in. Viruses are not placed within combo_length - 1 rows from the
    top. Returns a rows by cols array of cells or none."""

  def __init__(self, rows, cols, combo_length, virus_number):
    self.rows = rows
    self.cols = cols
    self.combo_length = combo_length
    self.virus_number = virus_number
    if (virus_number > (rows-(combo_length-1))*cols or virus_number < 0 or
        combo_length <= 1):
      raise VirusPlacementError, "invalid arguments"
    self.cells = [[None for c in xrange(cols)] for r in xrange(rows)]
    
  def getViruses(self):
    self.start_time = time.time()
    cells = self._placeNextVirus(0,int(self.rows*MIN_ROW_LIMIT_PERCENT))
    if not cells: raise VirusPlacementError, "could not place all viruses"
    return cells

  def _placeNextVirus(self,viruses_placed,row_limit):
    if time.time() - self.start_time > MAX_PLACEMENT_SECONDS:
      raise VirusPlacementError, "virus placement taking too long"
    if self.virus_number - viruses_placed <= 0: return self.cells
    if float(viruses_placed)/(row_limit*self.cols) > self._concentrationThreshold(row_limit):
      row_limit += 1
    for r in _randomIndex(row_limit):
      r += self.rows - row_limit
      for c in _randomIndex(self.cols):
        if self.cells[r][c] == None:
          virus = Virus()
          self.cells[r][c] = virus.color
          if self._virusPositionOkay(r,c,virus.color):
            attempt = self._placeNextVirus(viruses_placed+1,row_limit)
            if attempt: return attempt
          self.cells[r][c] = None
    return False

  def _concentrationThreshold(self,current_row_limit):
    """this function maps current_row_limit to a concentration threshold, given
      some domain parameters. it is an exponential equation formed from the 2
      points of (min_limit,MIN_CONCENTRATION_THRESHOLD) and (max_limit,1)"""
    max_limit = self.rows-(self.combo_length-1)
    min_limit = int(max_limit*MIN_ROW_LIMIT_PERCENT)
    effective_row_limit = min(max(current_row_limit,min_limit),max_limit)

    l1,l2,x = min_limit,max_limit,effective_row_limit
    y1 = MIN_CONCENTRATION_THRESHOLD # y2 is 1
    b = CONCENTRATION_THRESHOLD_INCREASE_FACTOR
    return (b**(float(x-l1)/(l2-l1))-1)*(1-y1)/(b-1)+y1
         
  def _virusPositionOkay(self,r,c,color):
    for direction in [(0,1), (1,0), (-1,0), (0,-1)]:
      for combo_pos in xrange(int(math.ceil(float(self.combo_length)/2.0))):
        already_placed_cells = 0
        for i in xrange(self.combo_length):
          new_r = r + direction[0]*i - direction[0]*combo_pos
          new_c = c + direction[1]*i - direction[1]*combo_pos
          cell = self._getCellAt(new_r,new_c)
          if isinstance(cell,str):
            if cell == color:
              already_placed_cells += 1
            else:
              break
        if already_placed_cells > self.combo_length/2:
          return False
    return True
          
  def _getCellAt(self,row,col):
    if row >= self.rows or row < 0 or col >= self.cols or col < 0:
      return None
    return self.cells[row][col]


class ComboPlacer(object):

  def __init__(self, cols, max_combos=None):
    self.cols = cols
    self.cells = [None for c in xrange(cols)]
    self.combo_colors = []
    self.max_combos = max_combos
    if self.max_combos is None:
      self.max_combos = int(math.ceil(float(self.cols)/2))
      
  def getComboPlacement(self,combo_colors):
    self.start_time = time.time()
    self.combo_colors = combo_colors
    cells = self._placeNextCombo(0)
    if not cells: raise ComboPlacementError, "could not place all combos"
    return cells

  def _placeNextCombo(self,placed_combos):
    if time.time() - self.start_time > MAX_PLACEMENT_SECONDS:
      raise ComboPlacementError, "combo placement taking too long"
    if len(self.combo_colors) <= 0: return self.cells
    if placed_combos == self.max_combos: return self.cells
    for c in _randomIndex(self.cols):
      if self.cells[c] == None:
        self.cells[c] = self.combo_colors.pop(0)
        if self._comboPositionOkay(c):
          attempt = self._placeNextCombo(placed_combos+1)
          if attempt: return attempt
        self.combo_colors.insert(0,self.cells[c])
        self.cells[c] = None
    return False

  def _comboPositionOkay(self,c):
    return self._getCellAt(c-1) == None and self._getCellAt(c+1) == None
          
  def _getCellAt(self,col):
    if col >= self.cols or col < 0:
      return None
    return self.cells[col]

      
def _randomIndex(length):
  indices = range(length)
  while len(indices) > 0:
    r = random.randint(0,len(indices)-1)
    yield indices[r]
    del indices[r]
    
    
if __name__ == "__main__":
  for i in xrange(17):
    print i, _concentrationThreshold(16,4,i)
