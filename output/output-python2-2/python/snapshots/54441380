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

from __future__ import with_statement
import SocketServer, sys, threading, Queue, cells, network, traceback, time
import socket, util, logging, random
import gflags as flags


SERVER_VERSION = "0.2"
PROTOCOL_VERSION = "0.2.2"
SERVER_WELCOME = "Viricide Game Server %s" % SERVER_VERSION


FLAGS = flags.FLAGS
flags.DEFINE_boolean("daemonize", False, "Starts the server in daemon mode.")
flags.DEFINE_integer("port", 54353, "The port for the server to listen on.")
flags.DEFINE_integer("max_connections", 1, "Maximum connections to accept "
    "total. Must have limit_total_connections set to True")
flags.DEFINE_boolean("limit_total_connections", False, "Whether or not to "
    "terminate after a certain amount of clients have connected. The amount is "
    "set by the max_connections flag")
flags.DEFINE_integer("game_start_timeout", 20, "Number of seconds to wait "
    "before aborting a game that needs more player. The countdown is reset if "
    "any players join the queue or cancel.")
flags.DEFINE_enum("log_level", "WARNING", ["CRITICAL", "ERROR", "WARNING",
    "INFO", "DEBUG"], "The logging level")


class Error_(Exception): """Generic Viricide Server Error"""
class LogicError(Error_): """Viricide Server Logic Error"""
class WrongNumberOfPlayers(LogicError): pass
class PlayerDoesntExist(LogicError): pass
class PlayerAlreadyDisconnected(LogicError): pass
class PlayerAlreadyFinished(LogicError): pass
class InvalidPlayerState(LogicError): pass
class PlayerGameExitUnset(LogicError): pass
class BadGameExitType(LogicError): pass
class InvalidVirusCount(LogicError): pass
class SocketAlreadyAdded(LogicError): pass
class NotReadyToStart(LogicError): pass
class StartingPlayerAmountMismatch(LogicError): pass
    

class ViricidePlayer(object):
  """A data object that maintains a particular player's information.
  Special attention should be given to the concept of a state, as players
  are simple state machines. The usual state transition is as follows:
  ACTIVE -> FINISHED -> DISCONNECTED
  When a player is active, they have just connected and are still able to
  play in the game. When a player is finished, they are waiting for other
  players to wrap up. When a player is disconnected, they closed their 
  connection and we're just bookkeeping to say they were here."""
  
  VALID_STATES = ["active", "finished", "disconnected"]
  VALID_GAME_EXITS = ["none", "won", "lost"]

  def __init__(self, sock, player_number, game):
    """Args:
      sock: the network socket associated with the player
      player_number: int, the player's number
      game: the game the player is a member of
    """
    self.sock = sock
    self.player_number = int(player_number)
    self.game = game
    self.setGameExit("none")
    self.setState("active")
    logging.debug("creating player %s" % self)

  def __str__(self):
    return "ViricidePlayer(%d, %s)" % (self.player_number, self.getState())
  
  def setState(self, state):
    """State mutator
    Args:
      state: string, the new state of the player. Must be in
          ViricidePlayer.VALID_STATES
    Raises:
      PlayerGameExitUnset
      InvalidPlayerState
    """
    if state == "finished" and self.getGameExit() == "none":
      raise PlayerGameExitUnset, str(self)
    if state not in ViricidePlayer.VALID_STATES:
      raise InvalidPlayerState, str(self)
    self._state = state
  
  def getState(self):
    """State accessor
    Returns:
      state, string, the state of the player
    """
    return self._state

  def setGameExit(self, game_exit):
    """Game exit type mutator
    Args:
      game_exit: string, the type of the game exit. must be in
          ViricidePlayer.VALID_GAME_EXITS
    Raises:
      BadGameExitType
    """
    if game_exit not in ViricidePlayer.VALID_GAME_EXITS:
      raise BadGameExitType, str(self)
    self._game_exit = game_exit
  
  def getGameExit(self):
    """Game exit type accessor
    Returns:
      game exit type, string, the game exit type of the player
    """
    return self._game_exit

  def disconnect(self):
    """Disconnects a player
    Raises:
      PlayerAlreadyDisconnected: when the player was already disconnected
    """
    logging.debug("disconnecting player %s" % self)
    if self.getGameExit() == "none": self.setGameExit("lost")
    if self.getState() != "disconnected":
      self.sock.close()
      self.setState("disconnected")
      self.game.notifyDisconnectedPlayer(self)

  def sendObject(self, obj, short_int=True):
    """Sends object obj to this particular player.
    This function delegates actual sending to FancySocket.sendObject, but
    handles disconnect testing and error handling, endeavoring to set the state
    of the player's connection correctly.
    Args:
      obj: the object to serialize and send
      short_int: boolean, if True, the default size limit for the object is the
          bytesize that can be contained in a short integer, otherwise a normal
          integer.
    Returns:
      None
    """
    if self.getState() == "disconnected": return
    try: self.sock.sendObject(obj, short_int=short_int)
    except socket.error, e: self.disconnect()
    except network.SocketClosed, e: self.disconnect()
  
  def getObject(self, short_int=True, timeout=None):
    """Gets object obj from this particular player.
    This function delegates actual receiving to FancySocket.getObject, but
    handles disconnect testing and error handling, endeavoring to set the state
    of the player's connection correctly.
    Args:
      short_int: boolean, if True, the default size limit for the object is the
          bytesize that can be contained in a short integer, otherwise a normal
          integer.
      timeout: the amount of time to wait for an incoming packet before
          returning None
    Returns:
      an unserialized object, or None if the timeout was reached.
    Raises:
      network.UnexpectedSocketClosed: when the socket closes
      network.SocketClosed: when the player is disconnected
    """
    if self.getState() == "disconnected":
      raise network.SocketClosed, "player %s disconnected" % self
    try: return self.sock.getObject(short_int=short_int, timeout=timeout)
    except socket.error, e:
      self.disconnect()
      raise network.UnexpectedSocketClose, str(e)
    except network.UnexpectedSocketClose, e:
      self.disconnect()
      raise e
    

class ViricideGame(object):
  """A data object that maintains a particular game's information"""
  
  def __init__(self, game_id, starting_amount_of_players, game_store):
    """Args:
      game_id: arbitrary but unique string identifying the game
      starting_amount_of_players: int, the number of players this game should
          start with
      game_store: the game store to clean up in the event of game termination
    """
    self.game_id = game_id
    self.starting_amount_of_players = int(starting_amount_of_players)
    self.game_store = game_store
    if self.starting_amount_of_players <= 0:
      raise WrongNumberOfPlayers, str(self.starting_amount_of_players)

    self.game_lock = threading.RLock()
    
    self.all_player_list = [] # only added to, never removed from.
    
    self.virus_placements = {}
    self.new_pill_list = []
    
    self.started = False
    self.game_over = False
    
    self.game_start_event = threading.Event()
    self.game_start_event.clear()
    self.game_over_event = threading.Event()
    self.game_over_event.clear()
    
    logging.debug("creating game %s" % self)

  def __str__(self):
    return "ViricideGame(%s, %d, started=%s, game_over=%s)" % (self.game_id,
        self.starting_amount_of_players, self.started, self.game_over)

  def connectedPlayers(self, check_with_heartbeat=False):
    """Returns a list of ViricidePlayer objects in this game that are not
      disconnected
    Args:
      check_with_heartbeat: boolean, if true, sends a heartbeat message to all
          known connected users to see if they're still alive
    Returns:
      the list
    """
    if check_with_heartbeat: self.sendObject({"message_type": "heartbeat"})
    with self.game_lock:
      return [x for x in self.all_player_list
          if x.getState() != "disconnected"]

  def activePlayers(self, check_with_heartbeat=False):
    """Returns a list of ViricidePlayer objects in this game that are active
    Args:
      check_with_heartbeat: boolean, if true, sends a heartbeat message to all
          known connected users to see if they're still alive
    Returns:
      the list
    """
    if check_with_heartbeat: self.sendObject({"message_type": "heartbeat"})
    with self.game_lock:
      return [x for x in self.all_player_list if x.getState() == "active"]

  def winningPlayer(self):
    """Returns:
    the player who won the game if one exists, None otherwise
    """
    with self.game_lock:
      winning_players = [x for x in self.all_player_list
                         if x.getGameExit() == "won"]
      assert len(winning_players) <= 1
      if not winning_players: return None
      assert winning_players[0].getState() != "active"
      return winning_players[0]

  def addPlayer(self, sock):
    """Adds a socket as a player
    Args:
      sock: the network socket the player connected on
    Returns:
      the player in the form of a ViricidePlayer instance.
    """
    with self.game_lock:
      if sock in [x.sock for x in self.all_player_list]:
        raise SocketAlreadyAdded, str(sock)
      player = ViricidePlayer(sock, len(self.all_player_list), self)
      logging.debug("adding player %s" % player)
      self.all_player_list.append(player)
      return player

  def finishPlayer(self, player, viruses_cleared, force_game_exit=None):
    """Moves a player out of the active state, also setting the correct game
    exit type. If the player is already finished, returns doing nothing.
    Args:
      player: the ViricidePlayer instance that is finishing their play
      viruses_cleared: boolean, true if the player cleared all of their viruses
      force_game_exit: string, if None, ignored. otherwise, set as the game_exit
    Raises:
      PlayerDoesntExist: when the player was never a member of this game
      InvalidVirusCount: the virus number is invalid.
      BadGameExitType: the player already has a game exit type assigned.
    """
    logging.debug("finishing player %s" % player)
    if player.getState() != "active":
      if self.winningPlayer() != None or len(self.activePlayers()) == 0:
        self.endGame()
      return
    if player.getGameExit() != "none": raise BadGameExitType, str(player)
    with self.game_lock:
      if player not in self.all_player_list:
        raise PlayerDoesntExist, str(player)
      # cases:
      #  1) player is the only player
      #    a) virus_count = 0 is a win
      #    b) virus_count > 0 is a loss
      #  2) player is one of many players
      #    a) virus_count = 0, no others at 0 -> win
      #    b) virus_count = 0, others at 0 -> loss
      #    c) virus_count > 0, still active players -> loss
      #    d) virus_count > 0, no other active players,
      #                        and no finished players with 0 -> win
      #    e) virus_count > 0, no other active players,
      #                        and finished players with 0 -> loss
      game_exit = "none"
      if force_game_exit is not None:
        game_exit = force_game_exit
      else:
        if self.starting_amount_of_players == 1:
          if viruses_cleared:
            game_exit = "won"
          else:
            game_exit = "lost"
        else:
          if self.winningPlayer() != None:
            game_exit = "lost"
          else:
            if viruses_cleared:
              game_exit = "won"
            else:
              if len(self.activePlayers(check_with_heartbeat=True)) > 1:
                game_exit = "lost"
              else:
                game_exit = "won"
      player.setGameExit(game_exit)
      player.setState("finished")
      self.sendObject({
          "message_type": "player_done",
          "player_number": player.player_number,
        })
      if self.winningPlayer() != None or len(self.activePlayers()) <= 1:
        self.endGame()

  def waitForGameStart(self):
    """Waits for the game start event to fire"""
    self.game_start_event.wait()
  
  def startGame(self):
    """Fires the game start event"""
    with self.game_lock:
      if not self.readyToStart():
        raise NotReadyToStart, str(self)
      logging.debug("starting game %s" % self)
      self.started = True
      self.game_start_event.set()
      self.sendObject({"message_type": "game_start"})

  def waitForGameEnd(self):
    """Waits for the game end event to fire"""
    self.game_over_event.wait()
  
  def endGame(self):
    """Fires the game end event, notifies users, and disconnects them"""
    with self.game_lock:
      if self.game_over: return
      logging.debug("ending game %s" % self)
      self.game_over = True
      self.game_over_event.set()
      force_game_exit = None
      if len(self.activePlayers()) > 1:
        force_game_exit = "lost"
      for player in self.activePlayers():
        self.finishPlayer(player, False, force_game_exit=force_game_exit)
      winning_player_number = None
      if self.winningPlayer() is not None:
        winning_player_number = self.winningPlayer().player_number
      self.sendObject({
          "message_type": "game_over",
          "winning_player_number": winning_player_number,
        })
      for player in self.connectedPlayers():
        player.disconnect()
      self.game_store.removeGameFromGameStore(self.game_id)
  
  def notifyDisconnectedPlayer(self, player):
    """Called when a player is disconnected, by the player object. Notifies the
    game.
    Args:
      player: the ViricidePlayer instance that is disconnecting
    Raises:
      PlayerDoesntExist: when the player was never a member of this game
    """
    logging.debug("disconnecting player %s" % player)
    with self.game_lock:
      if player not in self.all_player_list:
        raise PlayerDoesntExist, str(player)
      if self.winningPlayer() != None or len(self.activePlayers()) <= 1:
        self.endGame()

  def abortGame(self):
    """Clears all game events, disconnects all players, and ends the game"""
    logging.debug("aborting game %s" % self)
    with self.game_lock:
      self.started = True
      self.game_start_event.set()
      self.game_over = False
      for player in self.connectedPlayers():
        player.disconnect()
      self.endGame()

  def readyToStart(self):
    """Determines if the game is ready to start
    Returns:
      boolean, true if the game is ready to start
    """
    with self.game_lock:
      return (self.starting_amount_of_players ==
          len(self.activePlayers(check_with_heartbeat=True)) and not
          self.started)

  def getVirusPlacements(self, rows, cols, combo_length, virus_number):
    """Retrieves the cached virus placements for the given parameters if
    applicable, otherwise generates new placements. This way two players in a
    game with identical configuration get the same placements.
    Args:
      rows: int, the number of rows of the tube
      cols: int, the number of columns of the tube
      combo_length: int, the combo_length of the player. this is nearly always 4
      virus_number: int, the amount of viruses to place
    Returns:
      a two dimensional array of viruses. see cells.VirusPlacer for more info
    Raises:
      cells.VirusPlacementError
    """
    virus_placement_tuple = (rows, cols, combo_length, virus_number)
    with self.game_lock:
      if not self.virus_placements.has_key(virus_placement_tuple):
        virus_placement = \
            cells.VirusPlacer(*virus_placement_tuple).getViruses()
        self.virus_placements[virus_placement_tuple] = virus_placement
      return self.virus_placements[virus_placement_tuple]
      
  def getNewPills(self, pill_index):
    """Retrieves the next pill pair for a player, given that they want the pill
    pair indexed by pill_index
    Args:
      pill_index: int, the pill pair retrieved is the pill_index-th pill pair of
          the game
    Returns:
      a length-2 list of color strings
    """
    with self.game_lock:
      while pill_index >= len(self.new_pill_list):
        self.new_pill_list.append([cells.Pill().color, cells.Pill().color])
      return self.new_pill_list[pill_index]

  def sendObject(self, obj, short_int = True):
    """Sends object obj to all connected players of the current game.
    Since this function simply delegates to ViricidePlayer.sendObject,
    disconnect testing and error handling is done there.
    Args:
      obj: the object to serialize and send
      short_int: boolean, if True, the default size limit for the object is the
          bytesize that can be contained in a short integer, otherwise a normal
          integer.
    Returns:
      None
    """
    logging.debug("sending %s to game %s" % (obj, self))
    with self.game_lock:
      for player in self.connectedPlayers():
        player.sendObject(obj, short_int=short_int)

  def activePlayerAfter(self, player):
    """Retrieves the active player to the right of player. Only applicable
    when more than 1 player is still active, otherwise returns None. Otherwise
    always returns the next player in the player cycle.
    Args:
      player: the ViricidePlayer instance in question
    Returns:
      the next player in the player cycle, or None if an invalid question
    """
    with self.game_lock:
      active_players = self.activePlayers()
      assert player in active_players
      if len(active_players) <= 1: return None
      return active_players[(active_players.index(player) + 1) %
          len(active_players)]

  def sendPlayerCountUpdate(self):
    """Sends a message to all connected users informing them of the most
    recent player roster list count
    """
    with self.game_lock:
      self.sendObject({
          "message_type": "player_count_update",
          "count": len(self.connectedPlayers()),
        })
  
  def sendVirusCountUpdate(self, player, virus_count):
    """Sends a message to all connected users informing them of a virus count
    update
    """
    with self.game_lock:
      if player not in self.all_player_list:
        raise PlayerDoesntExist, str(player)
      self.sendObject({
          "message_type": "virus_number_update",
          "player_number": player.player_number,
          "virus_number": virus_count,
        })
  

class ViricideGameStartTimeoutThread(threading.Thread):
  """A thread that is to be started when a game is created. This thread monitors
  the game until it starts, waiting to see if there's any player activity. When
  there's no activity for a long time and the game hasn't started or there's no
  more players associated with the game, this thread aborts the game
  """

  def __init__(self, game_store, game_id, game_start_timeout=None):
    """Args:
      game_store: the ViricideGameStore that stores the game
      game_id: the game id for looking up the game in the game store. We keep
          a pointer to the game in this way, instead of just a ViricideGame
          instance reference, so that it can be garbage collected if the game
          ends even if this thread is still running.
      game_start_timeout: int, the time to wait before aborting the game with no
          activity. If None, set to FLAGS.game_start_timeout
    """
    threading.Thread.__init__(self)
    self.game_store = game_store
    self.game_id = game_id
    self.last_player_count = -1
    self.max_player_count = 0
    if game_start_timeout is None:
      self.game_start_timeout = FLAGS.game_start_timeout
    else:
      self.game_start_timeout = game_start_timeout
    self.termination_countdown = self.game_start_timeout
    self.setDaemon(True)
  
  def run(self):
    """Actual thread code. Every second, checks to see if the game has started.
    If it has, it kills the thread. If it hasn't it checks to see if any new
    players have joined or dropped. If there's activity, it resets the timer.
    Once the timer hits 0, it aborts the game.
    """
    while True:
      game = self.game_store.getGame(self.game_id)
      if game is None or game.started: break
      with game.game_lock:
        current_player_count = \
            len(game.activePlayers(check_with_heartbeat=True))
        self.max_player_count = max(self.max_player_count, current_player_count)
        if (self.last_player_count != current_player_count):
          self.termination_countdown = self.game_start_timeout
          self.last_player_count = current_player_count
        if self.termination_countdown <= 0 or (self.last_player_count <= 0 and
            self.max_player_count > 0):
          self.game_store.abortGame(self.game_id)
          break
      time.sleep(1)
      self.termination_countdown -= 1


class ViricideGameStore(object):
  """An object that maintains all of the available games and manages them"""

  def __init__(self, timeout_thread_class=ViricideGameStartTimeoutThread):
    """Args:
      timeout_thread_class: the threading.Thread class that aborts the game
          after too long of a period of inactivity. This is always the default
          in practice.
    """
    self.main_lock = threading.RLock()
    self.games_dict = {}
    self.timeout_thread_class = timeout_thread_class
    logging.debug("Creating game store")

  def createGame(self, game_id, starting_amount_of_players):
    """Creates OR retrieves a game with the given parameters. If the game
    doesn't already exist, this function both creates a ViricideGame object and
    stores it in the game store. Otherwise, an existing game is returned.
    
    Lastly, if this function creates a new ViricideGame object, it starts the
    timeout thread that aborts the game if no activity occurs for a given time
    period.
    
    Args:
      game_id: string, the game id of the game in question
      starting_amount_of_players: int, the number of players needed to start the
          game
    Returns:
      the ViricideGame instance
    Raises:
      WrongNumberOfPlayers: if an existing game is being retrieved but has a
          differing starting_amount_of_players
    """
    closing_function = lambda: None
    with self.main_lock:
      game = self.getGame(game_id)
      if game is None:
        game = ViricideGame(game_id, starting_amount_of_players, self)
        self.games_dict[game_id] = game
        closing_function =\
            lambda: self.timeout_thread_class(self, game_id).start()
      elif (game.starting_amount_of_players != starting_amount_of_players):
        raise WrongNumberOfPlayers, "Number of players is already %d" %\
            game.starting_amount_of_players
      logging.debug("game_store.createGame returning game %s" % game)
    closing_function()
    return game

  def getGame(self, game_id):
    """Returns an existing game if it exists, otherwise returns None
    Args:
      game_id: string, the game id with which to look up the corresponding game
    Returns:
      None if no game associated with game_id exists, the game otherwise
    """
    with self.main_lock:
      if self.games_dict.has_key(game_id):
        return self.games_dict[game_id]
    return None
  
  def removeGameFromGameStore(self, game_id):
    """Simply removes a game from the gamestore. Called by terminating
    ViricideGames
    Args:
      game_id: string, the game id with which to look up the corresponding game
    """
    logging.debug("removing game %s from gamestore" % game_id)
    with self.main_lock:
      if self.games_dict.has_key(game_id):
        del self.games_dict[game_id]
  
  def endGame(self, game_id):
    """Ends a game. Identical to calling game.endGame, but kept for backwards
    compatibility
    Args:
      game_id: string, the game id with which to look up the corresponding game
    """
    try: self.games_dict[game_id].endGame()
    except KeyError, e:
      if e.message != game_id: raise e
  
  def abortGame(self, game_id):
    """Aborts a game. Identical to calling game.abortGame, but kept for
    backwards compatibility
    Args:
      game_id: string, the game id with which to look up the corresponding game
    """
    try: self.games_dict[game_id].abortGame()
    except KeyError, e:
      if e.message != game_id: raise e


class ViricideConnectionHandler(SocketServer.BaseRequestHandler):
  """Class that handles incoming connection requests and player management.
  This class is ordinarily instantiated as a new thread.
  """
  
  REQUIRED_INIT_FIELDS = ['protocol_version', 'game_id',
      'starting_amount_of_players', 'rows', 'cols', 'virus_number',
      'combo_length', 'client_id']
  REQUIRED_INIT_INT_FIELDS = ['starting_amount_of_players', 'rows', 'cols',
      'virus_number', 'combo_length']
      
  def __init__(self, plain_socket, *args, **kwargs):
    """Args:
      plain_socket: a socket.socket type, to be converted to a
          network.FancySocket
      *args, **kwargs: remaining arguments after the socket argument from
          SocketServer.BaseRequestHandler
    """
    SocketServer.BaseRequestHandler.__init__(self,
        network.FancySocket(plain_socket), *args, **kwargs)
  
  def sendErrors(self):
    """Sends the contents of self.errors to the connecting player. Only valid
    as the first server response.
    """
    assert self.errors
    assert not self.responded
    self.request.sendObject({"success": False, "errors": self.errors},
        short_int=False)
    self.responded = True
  
  def sendCombos(self, first_colors, other_colors):
    """Sends combos to the correct other players, if applicable.
    This function docstring is a great example of why we need formal Viricide
    terminology.
    Args:
      first_colors: list of strings, the colors that went as the very first
          part of the combo. usually this is of length 1
      other_colors: list of strings, the colors that were part of subsequent
          portions of the combo.
    """
    if len(first_colors) <= 0: return
    sent_players = set([])
    with self.game.game_lock:
      if self.game.activePlayerAfter(self.player) == None: return
      target_player = self.player
      for color in cells.COLORS:
        target_player = self.game.activePlayerAfter(target_player)
        if target_player == self.player:
          target_player = self.game.activePlayerAfter(target_player)
        assert target_player != self.player
        if color in first_colors and target_player not in sent_players:
          target_player.sendObject({
              "message_type": "combos",
              "combos": first_colors + other_colors,
            })
          sent_players.add(target_player)
  
  def runGame(self):
    """This function is in charge of running the connected and joined player
    through the game
    """
    if self.game.readyToStart():
      self.game.startGame()
    self.game.waitForGameStart()
    pill_index = 0
    while True:
      if self.game.game_over: return
      try: msg = self.player.getObject()
      except network.UnexpectedSocketClose, e: return
      if msg is None: continue
      msg_type = msg["message_type"]
      if msg_type == "needs_pills":
        self.player.sendObject({
            "message_type": "new_pills",
            "pill_colors": self.game.getNewPills(pill_index),
          })
        pill_index += 1
      elif msg_type == "combos":
        self.sendCombos(msg["first_colors"], msg["other_colors"])
      elif msg_type == "game_over":
        self.game.finishPlayer(self.player, msg["virus_number"] == 0)
        break
      elif msg_type == "virus_number_update":
        self.game.sendVirusCountUpdate(self.player, msg["virus_number"])
    self.game.waitForGameEnd()
  
  def joinGame(self):
    """This function is in charge of helping the connecting player attempt to
    join a game.
    """
    logging.debug("joining game")
    try:
      self.game = self.server.game_store.createGame(self.game_id,
          self.starting_amount_of_players)
      if not isinstance(self.game, ViricideGame):
        return self.errors.append("Unknown game creation error.")
      if (self.game.starting_amount_of_players != 
          self.starting_amount_of_players):
        raise StartingPlayerAmountMismatch, ("%d != %d" %
            (self.game.starting_amount_of_players,
            self.starting_amount_of_players))
      logging.debug("acquiring lock")
      with self.game.game_lock:
        logging.debug("checking up on game status")
        if (len(self.game.activePlayers()) >=
            self.starting_amount_of_players):
          return self.errors.append("Game room full")
        if self.game.started:
          return self.errors.append("Game already started")
        virus_placements = self.game.getVirusPlacements(self.rows, self.cols,
            self.combo_length, self.virus_number)
        with self.request.sending_lock:
          self.player = self.game.addPlayer(self.request)
          self.player.sendObject({
              "success": True,
              "current_number_of_players": len(self.game.connectedPlayers()),
              "virus_placements": virus_placements,
              "player_number": self.player.player_number,
            }, short_int=False)
          self.responded = True
        self.game.sendPlayerCountUpdate()
      self.runGame()
    except WrongNumberOfPlayers, e:
      self.errors.append("Wrong number of players: %s" % e)
    except cells.VirusPlacementError, e:
      self.errors.append("Virus placement error: %s" % e)
    finally:
      if self.errors: self.sendErrors()

  def handle(self):
    """The main function that gets called on a socket connection. This function
    is in charge of checking the first message from the connecting player for
    basic sanity and saving the data
    """
    logging.debug("accepting request")
    self.errors = []
    self.responded = False
    try:
      self.request.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
      data = self.request.getObject()
      if type(data) != dict:
        self.errors.append("Incoming data must be a dictionary")
        return self.sendErrors()
      if len(data) != len(ViricideConnectionHandler.REQUIRED_INIT_FIELDS):
        self.errors.append("Must have %d fields." %
            len(ViricideConnectionHandler.REQUIRED_INIT_FIELDS))
      for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
        if not data.has_key(key):
          self.errors.append("Missing field: %s." % key)
          continue
        if (key in ViricideConnectionHandler.REQUIRED_INIT_INT_FIELDS and
            type(data[key]) != int):
          self.errors.append("Field %s must be an integer." % key)
          continue
        setattr(self, key, data[key])
      if hasattr(self, "protocol_version"):
        if self.protocol_version != PROTOCOL_VERSION:
          self.errors.append("Wrong protocol version!")
      else:
        self.errors.append("Missing protocol version!")
      if self.errors:
        self.sendErrors()
      else:
        self.joinGame()
    except Exception, e:
      print "Error!:", e
      traceback.print_exc()
    finally:
      logging.debug("closing up")
      if hasattr(self, "player"):
        self.player.disconnect()
      else:
        self.request.close()


class ViricideServer(SocketServer.ThreadingTCPServer):
  """The main Viricide Server class. This class starts and delegates to all
  others
  """

  def __init__(self, address=None, max_connections=None,
               limit_total_connections=None, game_store=None):
    """Args:
      address: address tuple, defaults to ("", FLAGS.port). What to listen on
      max_connections: int, Maximum connections to accept total. Must have
          limit_total_connections set to True
      limit_total_connections: boolean, Whether or not to terminate after a
          certain amount of clients have connected. The amount is set by the
          max_connections value
      game_store: ViricideGameStore, if None, this is created.
    """
    logging.debug("creating viricide server")
    if address is None:
      self.address = ("", FLAGS.port)
    else:
      self.address = address
    if max_connections is None:
      self.max_connections = FLAGS.max_connections
    else:
      self.max_connections = max_connections
    if limit_total_connections is None:
      self.limit_total_connections = FLAGS.limit_total_connections
    else:
      self.limit_total_connections = limit_total_connections
    if game_store is None:
      self.game_store = ViricideGameStore()
    else:
      self.game_store = game_store

    self.daemon_threads = not self.limit_total_connections
    self.allow_reuse_address = True

    SocketServer.ThreadingTCPServer.__init__(self, self.address,
        ViricideConnectionHandler)
  
  def run(self):
    """Starts listening for, accepting, and handling requests
    """
    if self.limit_total_connections:
      logging.debug("viricide server running with a %d request limit" %
          self.max_connections)
      for i in xrange(self.max_connections):
        self.handle_request()
    else:
      logging.debug("viricide server running with no request limit")
      self.serve_forever()


def server(address=None, daemon=None):
  if daemon is None: daemon = FLAGS.daemonize
  print SERVER_WELCOME
  print "Starting..."
  svr = ViricideServer(address)
  if daemon: util.daemonize()
  svr.run()

def SetUpLogging():
  logging.basicConfig(level=getattr(logging, FLAGS.log_level))

def main(argv):
  try:
    argv = FLAGS(argv)
  except flags.FlagsError, e:
    print "%s\nUsage: %s [flags]" % (e, sys.argv[0])
    print "Try --help for more information"
    return 1
  SetUpLogging()
  try: server()
  except KeyboardInterrupt, e: pass
  print "Goodbye."
  return 0
  
if __name__ == "__main__":
  sys.exit(main(sys.argv))
