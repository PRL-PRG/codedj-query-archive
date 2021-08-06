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

import pygame,os,cells

__author__ = "JT Olds"

class Tube(object):
  """Tube class
    implements the test-tube that the game is played in.
    for use in Viricide implementations"""

  CELLRECT = pygame.image.load(os.path.join("images","background.bmp")).get_rect()

  def __init__(self,rows=16,cols=8,title="Viricide"):
    """The Tube class is initialized with the size of the tube, but
      defaults to 16 rows and 8 columns.
      The Tube also takes in a title, to display as the window title for
      the game window."""
    self.rows = rows
    self.cols = cols
    self._cell_list = []
    self._positions_to_clear = []
    self._cells_to_redraw = []
    self._virus_count = 0
    
    # note the rows+1 here. turns out, python is really clever and allows
    # array indices of -1, which means last, -2 means second last, etc.
    # the -1th row is the row above the tube that you can rotate into but
    # can't see.
    self._cells = [[None for c in range(cols)] for r in range(rows+1)]
    
    self._next_pills = []
    
    pygame.init()
    self.images = {}
    for color in cells.COLORS:
      self._LoadImage("virus-"+color,False)
      for pill_type in ["-solid-left", "-solid-right", "-solid-top",
          "-solid-bottom","-broken","-highlight"]:
        self._LoadImage("pill-"+color+pill_type,False)
    self._LoadImage("background",False)

    pygame.display.set_caption(title)
    pygame.display.set_icon(self.images["pill-red-broken"])
    self.screen = pygame.display.set_mode((self.cols * Tube.CELLRECT.width, (self.rows+1) * Tube.CELLRECT.height))
    for image_name in self.images:
      self.images[image_name] = self.images[image_name].convert()
      
    background = pygame.Surface(self.screen.get_size()).convert()
    background.fill((255, 255, 255))
    self.screen.blit(background, (0,0))
    pygame.display.flip()
    self.UpdateScreen()

  def __del__(self):
    """Cleans up, kills the pygame display window."""
    pygame.display.quit()

  def SetNextPills(self,pill1,pill2):
    """Saves and displays the next 2 pill halves."""
    self._next_pills = [pill1,pill2]
    rects = [ Tube.CELLRECT.move((int(self.cols/2)-1) * Tube.CELLRECT.width, 0),
              Tube.CELLRECT.move((int(self.cols/2)) * Tube.CELLRECT.width, 0) ]
    sides = ["left","right"]
    for i in xrange(2):
      image = self._GetPicture("pill-"+self._next_pills[i].color+"-solid-"+sides[i])
      self.screen.blit(image,rects[i])
    pygame.display.update(rects)
        
  def GetNextPills(self):
    """Returns the next 2 pill halves saved."""
    return self._next_pills

  def VirusesRemaining(self):
    """Returns the number of viruses left in the tube."""
    return self._virus_count
    
  def _LoadImage(self,name,convert=True):
    self.images[name] = pygame.image.load(os.path.join("images",name+".bmp"))
    if convert: self.images[name] = self.images[name].convert()

  def GetPositionOf(self,cell):
    """Returns a tuple of row and column, given a cell that has previously
      been added to the tube."""
    if cell not in self._cell_list:
      raise RuntimeError, "Cell has not yet been added to the tube!"
    return (cell.row,cell.col)

  def GetCellAt(self,row,col):
    """Returns the cell at the given row and column, or None if that position
      is empty."""
    if row >= self.rows or row < -1 or col >= self.cols or col < 0:
      return None
    return self._cells[row][col]

  def AddCell(self,cell,row,col,update_screen=True):
    """Adds a cell to the tube object at initial row and column given if
      possible. Returns False otherwise."""
    if row >= self.rows or row < -1 or col >= self.cols or col < 0:
      raise RuntimeError, "Row or column out of range!"
    if self._cells[row][col] != None:
      return False
    if isinstance(cell,cells.Pill):
      cell.connected = False
      cell.moved = False
    elif isinstance(cell,cells.Virus):
      self._virus_count += 1
    self._cell_list.append(cell)
    self._cells[row][col] = cell
    cell.row,cell.col = row,col
    self._cells_to_redraw.append(cell)
    if update_screen: self.UpdateScreen()
    return True
    
  def ConnectPills(self,pill1,pill2):
    """Connects the given two half pills to form a whole pill. Should only be
      run when initially adding a whole pill to the tube."""
    if not (isinstance(pill1,cells.Pill) and isinstance(pill2,cells.Pill)):
      raise RuntimeError, "Arguments are not pills!"
    if pill1 not in self._cell_list or pill2 not in self._cell_list:
      raise RuntimeError, "Pill has not yet been added to the tube!"
    pill1.connected,pill2.connected = pill2,pill1
    self._cells_to_redraw += [pill1,pill2]
    self.UpdateScreen()
    return True

  def RotatePill(self,pill,clockwise=True):
    """Rotates the given pill 45 degrees, if possible. Returns False
      otherwise.
      Unfortunately, the rotation scheme employed by the real game
      is pretty unclean. Nearly every situation in which rotating might
      occur is a special case. As such, the logic here isn't very nicely
      algorithmic."""
    if not isinstance(pill,cells.Pill):
      return False
    if pill not in self._cell_list:
      raise RuntimeError, "Pill has not yet been added to the tube!"
    if not pill.connected:
      return True
    pills = [pill,pill.connected]
    if not self._IsAbove(*pills) and not self._IsLeftOf(*pills):
      pills.reverse()
    rv = self._RotatePillWork(pills,clockwise)
    if rv == True:
      self._cells_to_redraw += pills
      self.UpdateScreen()
    return rv
      
  def _RotatePillWork(self,pills,clockwise):
    if self._IsAbove(*pills):
      if clockwise:
        if self.MovePill(pills[0],1,1,False,False):
          return True
        elif self.MovePill(pills[1],0,-1,False,False):
          self.MovePill(pills[0],1,0,False,False)
          return True
      else:
        if self.MovePill(pills[1],0,1,False,False):
          self.MovePill(pills[0],1,0,False,False)
          return True
        elif self.MovePill(pills[0],1,-1,False,False):
          return True
    else:
      if clockwise:
        if self.MovePill(pills[0],-1,0,False,False):
          self.MovePill(pills[1],0,-1,False,False)
          return True
      else:
        if self.MovePill(pills[1],-1,-1,False,False):
          return True
    return False
    
  def _IsLeftOf(self,pill1,pill2):
    if not (isinstance(pill1,cells.Pill) and isinstance(pill2,cells.Pill)):
      raise RuntimeError, "Arguments are not pills!"
    if pill1 not in self._cell_list or pill2 not in self._cell_list:
      raise RuntimeError, "Pill has not yet been added to the tube!"
    return pill1.row == pill2.row and pill1.col < pill2.col
    
  def _IsAbove(self,pill1,pill2):
    if not (isinstance(pill1,cells.Pill) and isinstance(pill2,cells.Pill)):
      raise RuntimeError, "Arguments are not pills!"
    if pill1 not in self._cell_list or pill2 not in self._cell_list:
      raise RuntimeError, "Pill has not yet been added to the tube!"
    return pill1.col == pill2.col and pill1.row < pill2.row

  def MovePill(self,pill,drow,dcol,blocking=True,move_connected=True):
    """Moves the given pill drow and dcol away from it's current position, if
      possible. Returns False otherwise. Will only move a pill once on any
      iteration, unless blocking is False (to finish an iteration, call
      FinishIteration). If a pill is part of a whole pill, also moves the
      other half unless move_connected is False."""
    if not isinstance(pill,cells.Pill):
      return False
    if pill not in self._cell_list:
      raise RuntimeError, "Pill has not yet been added to the tube!"
    if pill.moved == True:
      return False
    if pill.connected and pill.connected.moved:
      raise RuntimeError, "Pill not moved, but its counterpart has been!"
    if not pill.connected: move_connected = False
    if self._PositionOutOfBounds(pill.row + drow, pill.col + dcol):
      return False
    if move_connected and self._PositionOutOfBounds(pill.connected.row + drow, pill.connected.col + dcol):
      return False
    if not move_connected:
      if self._cells[pill.row + drow][pill.col + dcol] != None:
        return False
    if move_connected:
      if self._cells[pill.connected.row + drow][pill.connected.col + dcol] != None and \
          self._cells[pill.connected.row + drow][pill.connected.col + dcol] != pill:
        return False
      if self._cells[pill.row + drow][pill.col + dcol] != None and \
          self._cells[pill.row + drow][pill.col + dcol] != pill.connected:
        return False
      self._ForcedMovePill(pill.connected,drow,dcol,blocking)
    self._ForcedMovePill(pill,drow,dcol,blocking)
    if move_connected or not pill.connected:
      self.UpdateScreen()
    return True

  def _ForcedMovePill(self,pill,drow,dcol,blocking):
    if self._cells[pill.row][pill.col] == pill:
      self._cells[pill.row][pill.col] = None
      self._positions_to_clear.append((pill.row,pill.col))
    else:
      confused_pill = self._cells[pill.row][pill.col]
      assert confused_pill == self._cells[confused_pill.row][confused_pill.col]
    pill.row += drow
    pill.col += dcol
    self._cells[pill.row][pill.col] = pill
    self._cells_to_redraw.append(pill)
    if blocking: pill.moved = True
    assert pill == self._cells[pill.row][pill.col]
  
  def _PositionOutOfBounds(self,row,col):
    return row >= self.rows or row < -1 or col >= self.cols or col < 0
    
  def cells(self,filter_class=cells.Cell):
    """This generator iterates through all the cells that are currently in the
      tube. Importantly, this generator iterates from the bottom of the tube
      to the top."""
    cell_count = 0
    for r in xrange(self.rows-1,-1,-1):
      for c in xrange(self.cols):
        cell = self._cells[r][c]
        if isinstance(cell,filter_class):
          yield cell
          cell_count += 1
    # the following is just nice checker code that checks for program sanity
    if filter_class == cells.Cell:
      assert cell_count == len(self._cell_list)

  def FinishIteration(self):
    """Call this function after an iteration has completed to clean up pills
      and allow pills to move the next iteration"""
    for cell in self._cell_list:
      if isinstance(cell,cells.Pill):
        cell.moved = False
    self.UpdateScreen()
          
  def RemoveCells(self,cell_list,update=True):
    """Removes cells in a given cell list. It is assumed that these cells
      have been eliminated through combos."""
    for cell in cell_list:
      self._cell_list.remove(cell)
      self._cells[cell.row][cell.col] = None
      self._positions_to_clear.append((cell.row,cell.col))
      cell.row,cell.col = None,None
      if isinstance(cell,cells.Pill) and cell.connected:
        cell.connected.connected = False
        if cell.connected not in cell_list:
          self._cells_to_redraw.append(cell.connected)
        cell.connected = False
      elif isinstance(cell,cells.Virus):
        self._virus_count -= 1
        assert self._virus_count >= 0
    if update: self.UpdateScreen()

  def ClearVeryTopRow(self):
    """Wipes all cells out in the -1st row. This can only be pill cells that
      have been rotated there."""
    top_row_cells = []
    for c in xrange(self.cols):
      if self._cells[-1][c] != None:
        assert not isinstance(self._cells[-1][c],cells.Virus)
        top_row_cells.append(self._cells[-1][c])
    self.RemoveCells(top_row_cells,False)
        
  def _GetPicture(self,cell):
    if isinstance(cell,cells.Pill):
      side = "-broken"
      if cell.connected:
        if self._IsAbove(cell,cell.connected):
          side = "-solid-top"
        elif self._IsLeftOf(cell,cell.connected):
          side = "-solid-left"
        if self._IsAbove(cell.connected,cell):
          side = "-solid-bottom"
        elif self._IsLeftOf(cell.connected,cell):
          side = "-solid-right"
      return self.images["pill-"+cell.color+side]
    elif isinstance(cell,cells.Virus):
      return self.images["virus-"+cell.color]
    else: # cell must be a string
      if self.images.has_key(cell):
        return self.images[cell]
      return self.images["background"]

  def UpdateScreen(self):
    """This function updates the screen by redrawing anything that has
      changed. Unless it is specified otherwise, this is done automatically
      whenever it needs to be done."""
    rects_to_redraw = []
    for position in self._positions_to_clear:
      if position[0] >= 0 and self._cells[position[0]][position[1]] == None:
        rects_to_redraw.append(self._DrawNewRect(position[0],position[1],None))
    self._positions_to_clear = []
    for cell in self._cells_to_redraw:
      if cell in self._cell_list and cell.row >= 0:
        assert self._cells[cell.row][cell.col] == cell
        rects_to_redraw.append(self._DrawNewRect(cell.row,cell.col,cell))
    self._cells_to_redraw = []
    pygame.display.update(rects_to_redraw)

  def _DrawNewRect(self,row,col,picture_cell):
    new_rect = Tube.CELLRECT.move(col * Tube.CELLRECT.width, (row+1) * Tube.CELLRECT.height)
    self.screen.blit(self._GetPicture(picture_cell),new_rect)
    return new_rect
