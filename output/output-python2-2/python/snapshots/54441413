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

import cells,tube,pygame

__author__ = "JT Olds"

TICK,MOVE_LEFT,MOVE_RIGHT,MOVE_DOWN = [i+pygame.USEREVENT+1 for i in range(4)]
KEYDELAY = {MOVE_LEFT: 150,
            MOVE_RIGHT: 150,
            MOVE_DOWN: 150}
KEYSPEED = {MOVE_LEFT: 150,
            MOVE_RIGHT: 150,
            MOVE_DOWN: 80}

class Driver(object):
  """This class drives the game of Viricide. It holds the rules and does most of
    the actual game physics processing"""
    
  def __init__(self,rows,cols,speed,combo_length,virus_number,comlink):
    """The supplied rows and columns are passed on as the size definition
      for the tube object the driver uses."""
    pygame.init()
    self.tube = tube.Tube(rows,cols)
    self.speed = speed
    self.combo_length = max([combo_length,2])
    self.virus_number = min([max([virus_number,0]),(rows-(combo_length-1))*cols])
    self._selected_pill = None
    self._game_paused = False
    self._comlink = comlink
    self._current_combos = []
    
  def PlaceViruses(self):
    viruses = self._comlink.GetViruses()
    for r in xrange(self.tube.rows):
      for c in xrange(self.tube.cols):
        if viruses[r][c] != None:
          self.tube.AddCell(cells.Virus(viruses[r][c]),r,c,False)
    self.tube.UpdateScreen()

  def InitGame(self):
    """Starts the game. Returns after game initialization"""
    self.PlaceViruses()
    pygame.time.set_timer(TICK,self.speed)
    self._game_paused = False
    
  def CleanupGame(self):
    pygame.time.set_timer(TICK,-1)
    
  def HandleEvent(self, event):
    if event.type == pygame.QUIT:
      self._comlink.NotifyGameOver(
          remaining_virus_number=self.tube.VirusesRemaining())
      return
    elif event.type == pygame.KEYDOWN:
      if event.key in [pygame.K_ESCAPE, pygame.K_q]:
        self._comlink.NotifyGameOver(
            remaining_virus_number=self.tube.VirusesRemaining())
        return
      elif event.key in [pygame.K_p, pygame.K_PAUSE]:
        self._game_paused = not self._game_paused
      if not self._game_paused:
        if event.key in [pygame.K_LEFT, pygame.K_RIGHT]:
          move_event = {pygame.K_LEFT:MOVE_LEFT,
                    pygame.K_RIGHT:MOVE_RIGHT}[event.key]
          self._MoveSelectedPill(move_event,KEYDELAY[move_event])
        elif event.key == pygame.K_DOWN:
          self._MoveSelectedPill(MOVE_DOWN,KEYDELAY[MOVE_DOWN])
        elif event.key == pygame.K_a:
          self.tube.RotatePill(self._selected_pill,clockwise=False)
        elif event.key == pygame.K_s:
          self.tube.RotatePill(self._selected_pill,clockwise=True)
    if not self._game_paused:
      if event.type == TICK:
        self._GameTick()
      elif event.type in [MOVE_LEFT, MOVE_RIGHT, MOVE_DOWN]:
        self._MoveSelectedPill(event.type,KEYSPEED[event.type])

  def _MoveSelectedPill(self,direction,timer_delay):
    """direction must be MOVE_LEFT, MOVE_RIGHT, or MOVE_DOWN."""
    if direction in [MOVE_LEFT, MOVE_RIGHT]:
      if pygame.key.get_pressed()[{MOVE_LEFT:pygame.K_LEFT,
                                   MOVE_RIGHT:pygame.K_RIGHT}[direction]]:
        pygame.time.set_timer(direction,timer_delay)
        self.tube.MovePill(self._selected_pill,0,
                                      {MOVE_LEFT:-1,MOVE_RIGHT:1}[direction],
                                      blocking=False,move_connected=True)
      else:
        pygame.time.set_timer(direction,-1)
    elif direction == MOVE_DOWN:
        if pygame.key.get_pressed()[pygame.K_DOWN]:
          pygame.time.set_timer(MOVE_DOWN,timer_delay)
          if not self.tube.MovePill(self._selected_pill,1,0,blocking=False,move_connected=True):
            self._selected_pill = None
        else:
          pygame.time.set_timer(MOVE_DOWN,-1)

  def _GameTick(self):
    # we need to see if any pills can be moved. we initialize with the selected
    # pill and see if the user is actively moving it. if that is the case we
    # don't need to scan. otherwise, we scan the tube and attempt to move all
    # pills down, recording if we moved any. when a pill is "selected", it
    # and it's connected half are the only pills that can possibly be falling
    # due to the rules of the game, so we're okay to try and not implement
    # gravity if that's the case.
    any_moved = self._selected_pill and pygame.key.get_pressed()[pygame.K_DOWN] \
                or self.tube.MovePill(self._selected_pill,1,0)
    if not any_moved:
      for pill in self.tube.cells(cells.Pill):
        any_moved = self.tube.MovePill(pill,1,0) or any_moved
    # well, now we see if we could move any pills. if we couldn't we clear the
    # top row of any pieces that got rotated there, and attempt to clear any
    # combos. if there are, we remove them and don't add a new pill, to allow
    # the combo remainders to fall. if we can't even clear anything, we add
    # a new pill. if we can't even do that, the tube must be full and the game
    # is over.
    if not any_moved:
      self._selected_pill = None
      self.tube.ClearVeryTopRow()
      if not self._ClearCells():
        self._SendCombos()
        if not self._PlaceCombos():
          if not self._AddNewPill():
            self._comlink.NotifyGameOver(
                remaining_virus_number=self.tube.VirusesRemaining())
      if self.tube.VirusesRemaining() <= 0:
        self._comlink.NotifyGameOver(
            remaining_virus_number=self.tube.VirusesRemaining())
      self._comlink.UpdateVirusNumber(self.tube.VirusesRemaining())
    self.tube.FinishIteration()

  def _IsPillSelected(self,pill):
    return pill == self._selected_pill or \
           (pill.connected and pill.connected == self._selected_pill)
           
  def _SendCombos(self):
    if len(self._current_combos) > 1:
      self._comlink.SendCombos(self._current_combos, [])
    self._current_combos = []
    
  def _PlaceCombos(self):
    combo_colors = self._comlink.GetCombos()
    if not combo_colors: return False
    combo_placer = cells.ComboPlacer(self.tube.cols)
    for i, combo in enumerate(combo_placer.getComboPlacement(combo_colors)):
      if combo != None:
        self._PlaceCombo(cells.Pill(combo), 0, i)
    return True
    
  def _PlaceCombo(self, pill, row, col):
    if self.tube.GetCellAt(row,col) != None:
      self.tube.RemoveCells([self.tube.GetCellAt(row,col)])
    self.tube.AddCell(pill, row, col)

  def _ClearCells(self):
    """return true if any things get cleared"""
    cells_to_clear = []
    for cell in self.tube.cells():
      cells_to_clear_attempts = [ self._CellsToClearFromCell(cell,[],cell.color,"horizontal"),
                                  self._CellsToClearFromCell(cell,[],cell.color,"vertical") ]
      for cells_to_clear_attempt in cells_to_clear_attempts:
        if len(cells_to_clear_attempt) >= self.combo_length:
          if cells_to_clear_attempt[0] not in cells_to_clear:
            self._current_combos.append(cells_to_clear_attempt[0].color)
          for cell_to_clear in cells_to_clear_attempt:
            if cell_to_clear not in cells_to_clear:
              cells_to_clear.append(cell_to_clear)
    self.tube.RemoveCells(cells_to_clear)
    return len(cells_to_clear) > 0

  def _CellsToClearFromCell(self,cell,running_list,color,direction):
    """big gross implementation note: the running_list argument gets modified"""
    if not isinstance(cell,cells.Cell) or cell in running_list or cell.color != color:
      return running_list
    running_list.append(cell)
    r,c = self.tube.GetPositionOf(cell)
    next_cells = [None,None]
    if direction=="horizontal":
      next_cells = [self.tube.GetCellAt(r,c+1), self.tube.GetCellAt(r,c-1)]
    elif direction=="vertical":
      next_cells = [self.tube.GetCellAt(r+1,c), self.tube.GetCellAt(r-1,c)]
    for next_cell in next_cells:
      self._CellsToClearFromCell(next_cell,running_list,color,direction)
    return running_list
    
  def _AddNewPill(self):
    pills = self.tube.GetNextPills()
    new_pills = self._comlink.GetNewPills()
    self.tube.SetNextPills(*new_pills)
    if pills == []: return True
    if not self.tube.AddCell(pills[1],0,int(self.cols/2),update_screen=False):
      return False
    if not self.tube.AddCell(pills[0],0,int(self.cols/2)-1,update_screen=False):
      return False
    self.tube.ConnectPills(*pills)
    self._selected_pill = pills[0]
    return True

  def __getattr__(self,name):
    """Redirects requests for the attributes "rows" and "cols" to the Tube"""
    if name == "rows":
      return self.tube.rows
    elif name == "cols":
      return self.tube.cols
    else:
      raise AttributeError(name)
      
