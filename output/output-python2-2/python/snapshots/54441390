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

# TODO(jtolds): make client distinction between player being done and game over

"""This module holds classes that support communication for multiplayer"""

import cells, network

PROTOCOL_VERSION = "0.2.2"
MAX_MESSAGE_WAIT_TIME = 1.0
CLIENT_VERSION = "0.1"
CLIENT_IDENTIFICATION = "JT's Viricide Game Client %s" % CLIENT_VERSION
__author__ = "JT Olds"


class Comlink(object):
  """Comlink class
    For use with Viricide implementations
    This is the parent class for NetworkServer, NetworkClient, SinglePlayer,
    etc."""

  def GetViruses(self):
    raise AbstractMethod
    
  def GetNewPills(self):
    raise AbstractMethod
    
  def GetCombos(self):
    raise AbstractMethod
    
  def SendCombos(self, first_colors, other_colors):
    raise AbstractMethod
    
  def NotifyGameOver(self, remaining_virus_number):
    raise AbstractMethod
    
  def CheckGameOver(self):
    raise AbstractMethod
    
  def HandleEvent(self, event):
    raise AbstractMethod
    
  def InitGame(self, driver):
    raise AbstractMethod
    
  def CleanupGame(self):
    raise AbstractMethod
  
  def UpdateVirusNumber(self, number_of_viruses):
    raise AbstractMethod

    
class SinglePlayer(Comlink):

  def __init__(self):
    self.game_over = True
    self.win = False
    self.driver = None
  
  def GetViruses(self):
    virus_placer = cells.VirusPlacer(self.driver.tube.rows,
                                     self.driver.tube.cols,
                                     self.driver.combo_length,
                                     self.driver.virus_number)
    return virus_placer.getViruses()
    
  def GetNewPills(self):
    return [cells.Pill(), cells.Pill()]
    
  def SendCombos(self, first_colors, other_colors):
    pass
    
  def GetCombos(self):
    return []
    
  def HandleEvent(self, event):
    pass
    
  def NotifyGameOver(self, remaining_virus_number):
    self.game_over = True
    self.win = (remaining_virus_number == 0)
    
  def CheckGameOver(self):
    return self.game_over, self.win

  def InitGame(self, driver):
    self.game_over = False
    self.win = False
    self.driver = driver
    
  def CleanupGame(self):
    self.game_over = True
    
  def UpdateVirusNumber(self, number_of_viruses):
    pass
    

class NetworkComlink(Comlink):

  def __init__(self, host, game_id, number_of_players, port=54353):
    self.address = (host, port)
    self.game_id = game_id
    self.number_of_players = number_of_players
    self.sock = network.FancySocket()
    self.game_over = True
    self.win = False
    self.driver = None
    self.viruses = []
    self.new_pills = []
    self.new_combos = []
    self.player_number = -1
    
  def GetNewPills(self):
    if not self.game_over: 
      self.sock.sendObject({"message_type": "needs_pills"})
    while not self.new_pills and not self.game_over:
      self.HandleEvent(None, notimeout=True)
    if self.game_over: return [cells.Pill(), cells.Pill()]  
    return [cells.Pill(x) for x in self.new_pills.pop(0)]

  def GetCombos(self):
    new_combos = self.new_combos
    self.new_combos = []
    return new_combos
    
  def SendCombos(self, first_colors, other_colors):
    if self.game_over: return
    self.sock.sendObject({
        "message_type": "combos",
        "first_colors": first_colors,
        "other_colors": other_colors,
      })
    
  def NotifyGameOver(self, remaining_virus_number):
    if self.game_over: return
    self.sock.sendObject({
        "message_type": "game_over",
        "virus_number": remaining_virus_number,
      })

  def UpdateVirusNumber(self, number_of_viruses):
    if self.game_over: return
    if self.virus_number == number_of_viruses: return
    self.virus_number = number_of_viruses
    self.sock.sendObject({
        "message_type": "virus_number_update",
        "virus_number": number_of_viruses,
      })
    
  def CheckGameOver(self):
    return self.game_over, self.win

  def HandleEvent(self, event, notimeout=False):
    if self.game_over: return
    if notimeout:
      event = self.sock.getObject()
    else:
      event = self.sock.getObject(timeout=0)
    while event:
      if event["message_type"] == "new_pills":
        self.new_pills.append(event["pill_colors"])
      elif event["message_type"] == "combos":
        self.new_combos.extend(event["combos"])
      elif event["message_type"] == "game_over":
        self.game_over = True
        self.win = (event["winning_player_number"] == self.player_number)
        break
      event = self.sock.getObject(timeout=0)

  def GetViruses(self):
    return self.viruses

  def CleanupGame(self):
    self.sock.close()
    self.game_over = True

  def _ServerHandshake(self):
    server_welcome = self.sock.readline().replace('\n','')
    protocol_version = self.sock.readline().replace('\n','')
    assert PROTOCOL_VERSION == protocol_version
    self.virus_number = self.driver.virus_number
    self.sock.sendObject({
        "protocol_version": PROTOCOL_VERSION,
        "game_id": self.game_id,
        "starting_amount_of_players": self.number_of_players,
        "rows": self.driver.tube.rows,
        "cols": self.driver.tube.cols,
        "virus_number": self.virus_number,
        "combo_length": self.driver.combo_length,
        "client_id": CLIENT_IDENTIFICATION,
      })
    resp = self.sock.getObject(short_int=False)
    assert resp["success"]
    self.viruses = resp["virus_placements"]
    self.player_number = resp["player_number"]

  def _WaitForGameStart(self):
    while not self.game_over:
      resp = self.sock.getObject()
      if resp["message_type"] == "game_start":
        self.game_over = False
        break
                         
  def InitGame(self, driver):
    self.game_over = False
    self.win = False
    self.driver = driver
    self.sock.connect(self.address)
    self._ServerHandshake()
    self._WaitForGameStart()
    print "done waiting"
