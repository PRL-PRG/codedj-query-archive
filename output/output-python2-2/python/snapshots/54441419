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
import unittest, mocker, random
from server import *

flags.DEFINE_boolean("ignore_time_based_tests", False, "if True, does not run "
    "tests that run code with time.sleep")

class ExtendedTestCase(mocker.MockerTestCase):

  def setUp(self):
    mocker.MockerTestCase.setUp(self)
    self.all_args = []
    self.all_kwargs = []
  
  def addArgsIfCalled(self, *args, **kwargs):
    self.all_args.append(args)
    self.all_kwargs.append(kwargs)
  
  def raiseErrorIfCalled(self, *args, **kwargs):
    raise Exception

  def assertListEquals(self, list1, list2):
    try:
      self.assertEquals(type(list1), type(list2))
      self.assertEquals(type(list1), list)
      self.assertEquals(len(list1), len(list2))
      for i, j in zip(list1, list2):
        self.assertEquals(i, j)
    except AssertionError, e:
      raise AssertionError, "\n%s\n!=\n%s\n(%s)" % (list1, list2, e)

  def assertListNotEquals(self, list1, list2):
    self.assertEquals(type(list1), type(list2))
    self.assertEquals(type(list1), list)
    try:
      self.assertListEquals(list1, list2)
    except AssertionError, e: return
    raise AssertionError, "\n%s\n==\n%s" % (list1, list2)


class TestViricidePlayer(ExtendedTestCase):

  def testInit(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    player_number = random.randint(0, 10000)
    x = ViricidePlayer(mock_socket, player_number, game)
    self.assertEquals(x.sock, mock_socket)
    self.assertEquals(x.player_number, player_number)
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.game, game)
  
  def testStates(self):
    mock_socket = self.mocker.mock()
    self.mocker.replay()
    player_number = random.randint(0, 10000)
    
    self.assertListEquals(["active", "finished", "disconnected"],
        ViricidePlayer.VALID_STATES)
    
    for state in ViricidePlayer.VALID_STATES:
      x = ViricidePlayer(mock_socket, player_number, None)
      x.setGameExit("won") # bypass the finish game_exit requirement
      x.setState(state)
      self.assertEquals(x.getState(), state)
    
    self.assertRaises(InvalidPlayerState, x.setState, "bad state")
  
  def testBadPlayerNumber(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    self.assertRaises(ValueError, ViricidePlayer, mock_socket, "not integer",
        game)

  def testGameFinish(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getState(), "active")
    self.assertRaises(PlayerGameExitUnset, x.setState, "finished")
    self.assertEquals(x.getGameExit(), "none")
    x.setGameExit("won")
    x.setState("finished")
    self.assertEquals(x.getGameExit(), "won")
    self.assertEquals(x.getState(), "finished")

  def testGameExits(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()

    self.assertListEquals(["none", "won", "lost"],
        ViricidePlayer.VALID_GAME_EXITS)

    for game_exit in ViricidePlayer.VALID_GAME_EXITS:
      x = ViricidePlayer(mock_socket, 1, game)
      x.setGameExit(game_exit)
      self.assertEquals(x.getGameExit(), game_exit)
    
    self.assertRaises(BadGameExitType, x.setGameExit, "bad game exit")
  
  def testSendObject1(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.disconnect()
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")
    # the following will fail if it tries to do sock.sendObject, as the mock
    # socket isn't expecting it
    x.sendObject("test_obj", short_int="nonce1")
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")
    self.assertListEquals(self.all_args, [(x,)])
    
  def testSendObject2(self):
    game = self.mocker.mock()
    mock_socket = self.mocker.mock()
    mock_socket.sendObject("test_obj", short_int="nonce1")
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.sendObject("test_obj", short_int="nonce1")
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.getGameExit(), "none")
    self.assertListEquals(self.all_args, [])
  
  def testSendObject3(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.sendObject("test_obj2", short_int="nonce2")
    self.mocker.throw(socket.error)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.sendObject("test_obj2", short_int="nonce2")
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")
    self.assertListEquals(self.all_args, [(x,)])
    
  def testSendObject4(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.sendObject("test_obj2", short_int="nonce2")
    self.mocker.throw(socket.error)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.setGameExit("won")
    x.setState("finished")
    x.sendObject("test_obj2", short_int="nonce2")
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "won")
    self.assertListEquals(self.all_args, [(x,)])
    
  def testSendObject5(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.sendObject("test_obj2", short_int="nonce2")
    self.mocker.throw(network.UnexpectedSocketClose)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.sendObject("test_obj2", short_int="nonce2")
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")
    self.assertListEquals(self.all_args, [(x,)])

  def testGetObject1(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    x.disconnect()
    self.assertRaises(network.SocketClosed, x.getObject)
    self.assertListEquals(self.all_args, [(x,)])

  def testGetObject2(self):
    game = self.mocker.mock()
    mock_socket = self.mocker.mock()
    mock_socket.getObject(short_int="nonce1", timeout="nonce2")
    self.mocker.result("nonce3")
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getObject(short_int="nonce1", timeout="nonce2"),
        "nonce3")

  def testGetObject3(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.getObject(short_int=True, timeout=None)
    self.mocker.throw(socket.error)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.getGameExit(), "none")
    self.assertRaises(network.UnexpectedSocketClose, x.getObject)
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")    
    self.assertListEquals(self.all_args, [(x,)])

  def testGetObject4(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.getObject(short_int=True, timeout=None)
    self.mocker.throw(socket.error)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.getGameExit(), "none")
    x.setGameExit("won")
    self.assertRaises(network.UnexpectedSocketClose, x.getObject)
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "won")
    self.assertListEquals(self.all_args, [(x,)])

  def testGetObject5(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.getObject(short_int=True, timeout=None)
    self.mocker.throw(network.UnexpectedSocketClose)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.getGameExit(), "none")
    self.assertRaises(network.UnexpectedSocketClose, x.getObject)
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "lost")    
    self.assertListEquals(self.all_args, [(x,)])

  def testGetObject6(self):
    game = self.mocker.mock()
    game.notifyDisconnectedPlayer
    self.mocker.result(self.addArgsIfCalled)
    mock_socket = self.mocker.mock()
    mock_socket.getObject(short_int=True, timeout=None)
    self.mocker.throw(network.UnexpectedSocketClose)
    mock_socket.close()
    self.mocker.replay()
    x = ViricidePlayer(mock_socket, 1, game)
    self.assertEquals(x.getState(), "active")
    self.assertEquals(x.getGameExit(), "none")
    x.setGameExit("won")
    self.assertRaises(network.UnexpectedSocketClose, x.getObject)
    self.assertEquals(x.getState(), "disconnected")
    self.assertEquals(x.getGameExit(), "won")
    self.assertListEquals(self.all_args, [(x,)])

  def testDisconnect1(self):
    player = self.mocker.mock(ViricidePlayer)
    player.getGameExit()
    self.mocker.result("none")
    player.setGameExit("lost")
    player.getState()
    self.mocker.result("disconnected")
    self.mocker.replay()
    ViricidePlayer.disconnect(player)
    
  def testDisconnect2(self):
    player = self.mocker.mock(ViricidePlayer)
    player.getGameExit()
    self.mocker.result("lost")
    player.getState()
    self.mocker.result("disconnected")
    self.mocker.replay()
    ViricidePlayer.disconnect(player)
    
  def testDisconnect3(self):
    player = self.mocker.mock(ViricidePlayer)
    player.getGameExit()
    self.mocker.result("won")
    player.getState()
    self.mocker.result("disconnected")
    self.mocker.replay()
    ViricidePlayer.disconnect(player)
    
  def testDisconnect4(self):
    player = self.mocker.mock(ViricidePlayer)
    sock = self.mocker.mock()
    game = self.mocker.mock(ViricideGame)
    player.getGameExit()
    self.mocker.result("none")
    player.setGameExit("lost")
    player.getState()
    self.mocker.result("active")
    player.sock
    self.mocker.result(sock)
    sock.close()
    player.setState("disconnected")
    player.game
    self.mocker.result(game)
    game.notifyDisconnectedPlayer(player)
    self.mocker.replay()
    ViricidePlayer.disconnect(player)
    
  def testDisconnect5(self):
    player = self.mocker.mock(ViricidePlayer)
    sock = self.mocker.mock()
    game = self.mocker.mock(ViricideGame)
    player.getGameExit()
    self.mocker.result("won")
    player.getState()
    self.mocker.result("finished")
    player.sock
    self.mocker.result(sock)
    sock.close()
    player.setState("disconnected")
    player.game
    self.mocker.result(game)
    game.notifyDisconnectedPlayer(player)
    self.mocker.replay()
    ViricidePlayer.disconnect(player)

  def testDisconnect6(self):
    player = self.mocker.mock(ViricidePlayer)
    sock = self.mocker.mock()
    game = self.mocker.mock(ViricideGame)
    player.getGameExit()
    self.mocker.result("lost")
    player.getState()
    self.mocker.result("finished")
    player.sock
    self.mocker.result(sock)
    sock.close()
    player.setState("disconnected")
    player.game
    self.mocker.result(game)
    game.notifyDisconnectedPlayer(player)
    self.mocker.replay()
    ViricidePlayer.disconnect(player)


class TestViricideGame(ExtendedTestCase):

  def setUp(self):
    ExtendedTestCase.setUp(self)
    self.game_id = str(random.random())
    self.number_of_players = random.randint(1, 10000)
    self.test_game = ViricideGame(self.game_id, self.number_of_players, None)

  def tearDown(self):
    del self.test_game
    ExtendedTestCase.tearDown(self)

  def testInit(self):
    game_id = str(random.random())
    number_of_players = random.randint(1, 10000)
    x = ViricideGame(game_id, number_of_players, "nonce")
    self.assertEquals(x.game_id, game_id)
    self.assertEquals(x.starting_amount_of_players, number_of_players)
    self.assertEquals(x.game_store, "nonce")

  
  def testBadPlayerAmount(self):
    game_id = str(random.random())
    self.assertRaises(ValueError, ViricideGame, game_id, "not integer", None)
    self.assertRaises(WrongNumberOfPlayers, ViricideGame, game_id, -1, None)
    self.assertRaises(WrongNumberOfPlayers, ViricideGame, game_id, 0, None)
    

  def testConnectedAndActivePlayersHeartbeat(self):
    send_object_called = []
    def MockSendObject(obj):
      self.assertEquals(len(obj), 1)
      self.assertEquals(obj["message_type"], "heartbeat")
      send_object_called.append("called")
    self.test_game.sendObject = MockSendObject
    self.test_game.connectedPlayers()
    self.assertEquals(len(send_object_called), 0)
    self.test_game.connectedPlayers(check_with_heartbeat=True)
    self.assertEquals(len(send_object_called), 1)
    self.test_game.activePlayers()
    self.assertEquals(len(send_object_called), 1)
    self.test_game.activePlayers(check_with_heartbeat=True)
    self.assertEquals(len(send_object_called), 2)

  def testPlayerFilters(self):
    def MockSendObject(obj):
      self.assertEquals(len(obj), 1)
      self.assertEquals(obj["message_type"], "heartbeat")
    self.test_game.sendObject = MockSendObject
    self.assertListEquals(self.test_game.all_player_list, [])

    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    j = 0
    for i in xrange(3):
      for state in ViricidePlayer.VALID_STATES:
        player = ViricidePlayer(mock_socket, j, game)
        player.setGameExit("won") # bypass the finish game_exit requirement
        player.setState(state)
        self.test_game.all_player_list.append(player)
        j += 1

    self.assertEquals([x.player_number for x in self.test_game.activePlayers()],
        [0, 3, 6])
    self.assertEquals([x.player_number
        for x in self.test_game.connectedPlayers()], [0, 1, 3, 4, 6, 7])

  def testAddPlayer(self):
    mock_socket = self.mocker.mock()
    mock_socket_2 = self.mocker.mock()
    self.mocker.replay()
    self.assertEquals(len(self.test_game.activePlayers()), 0)
    new_player = self.test_game.addPlayer(mock_socket)
    self.assertEquals(len(self.test_game.activePlayers()), 1)
    self.assertEquals(len(self.test_game.all_player_list), 1)
    self.assertIn(new_player, self.test_game.activePlayers())
    self.assertIn(new_player, self.test_game.all_player_list)

    new_player_2 = self.test_game.addPlayer(mock_socket_2)
    new_player_2.setGameExit("won") # bypass the finish game_exit requirement
    new_player_2.setState("finished")
    self.assertEquals(len(self.test_game.activePlayers()), 1)
    self.assertEquals(len(self.test_game.connectedPlayers()), 2)
    self.assertEquals(len(self.test_game.all_player_list), 2)
    self.assertNotIn(new_player_2, self.test_game.activePlayers())
    self.assertIn(new_player_2, self.test_game.connectedPlayers())
    self.assertIn(new_player_2, self.test_game.all_player_list)
    self.assertEquals(new_player_2.getState(), "finished")
    
    self.assertNotEquals(new_player, new_player_2)
    self.assertNotEquals(new_player.player_number, new_player_2.player_number)

  def testAddPlayerSameSocket(self):
    mock_socket = self.mocker.mock()
    self.mocker.replay()
    self.test_game.addPlayer(mock_socket)
    self.assertRaises(SocketAlreadyAdded, self.test_game.addPlayer, mock_socket)

  def testDisconnectedPlayer(self):
    mock_socket_1 = self.mocker.mock()
    mock_socket_1.close()
    mock_socket_2 = self.mocker.mock()
    mock_socket_2.close()
    self.mocker.replay()
    self.test_game.notifyDisconnectedPlayer = self.addArgsIfCalled
    new_player = self.test_game.addPlayer(mock_socket_1)
    new_player_2 = self.test_game.addPlayer(mock_socket_2)
    new_player_2.setGameExit("won")
    new_player_2.setState("finished")
    self.assertEquals(len(self.test_game.all_player_list), 2)
    self.assertEquals(len(self.test_game.connectedPlayers()), 2)
    self.assertEquals(len(self.test_game.activePlayers()), 1)
    
    self.assertListEquals(["active", "finished"],
        [x.getState() for x in self.test_game.all_player_list])

    new_player.disconnect()
    self.assertEquals(len(self.test_game.all_player_list), 2)
    self.assertEquals(len(self.test_game.connectedPlayers()), 1)
    self.assertEquals(len(self.test_game.activePlayers()), 0)
    self.assertEquals(new_player.getState(), "disconnected")
    self.assertListEquals(["disconnected", "finished"],
        [x.getState() for x in self.test_game.all_player_list])

    new_player_2.disconnect()
    self.assertEquals(len(self.test_game.all_player_list), 2)
    self.assertEquals(len(self.test_game.connectedPlayers()), 0)
    self.assertEquals(len(self.test_game.activePlayers()), 0)
    self.assertEquals(new_player_2.getState(), "disconnected")
    self.assertListEquals(["disconnected", "disconnected"],
        [x.getState() for x in self.test_game.all_player_list])
    
    self.assertEquals(new_player.getGameExit(), "lost")
    self.assertEquals(new_player_2.getGameExit(), "won")
    self.assertListEquals(self.all_args, [(new_player,), (new_player_2,)])

  def testFinishedPlayerBadGameExit(self):
    mock_socket = self.mocker.mock()
    self.mocker.replay()
    new_player = self.test_game.addPlayer(mock_socket)
    new_player.setGameExit("won")
    self.assertRaises(BadGameExitType, self.test_game.finishPlayer, new_player,
        False)
    new_player.setGameExit("lost")
    self.assertRaises(BadGameExitType, self.test_game.finishPlayer, new_player,
        False)

  def testFinishedNonExistentPlayer(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    new_player = ViricidePlayer(mock_socket, 4, game)
    self.assertRaises(PlayerDoesntExist, self.test_game.finishPlayer, 
        new_player, False)
    
  def testNotifyDisconnectedPlayerNonExistentPlayer(self):
    mock_socket = self.mocker.mock()
    game = self.mocker.mock()
    self.mocker.replay()
    new_player = ViricidePlayer(mock_socket, 4, game)
    self.assertRaises(PlayerDoesntExist,
        self.test_game.notifyDisconnectedPlayer, new_player)
  
  def testWinningPlayer(self):
    mock_socket = self.mocker.mock()
    self.mocker.replay()
    new_player = self.test_game.addPlayer(mock_socket)
    self.assertIs(self.test_game.winningPlayer(), None)
    new_player.setGameExit("won")
    new_player.setState("finished")
    self.assertIs(self.test_game.winningPlayer(), new_player)

  def testFinishedPlayerForInactive(self):
    mock_socket = self.mocker.mock()
    mock_socket.close()
    mock_game = self.mocker.mock()
    mock_game.notifyDisconnectedPlayer(mocker.MATCH(lambda x:
        isinstance(x, ViricidePlayer)))
    self.mocker.replay()

    test_game_1 = ViricideGame("test_game_id", 1, None)
    new_player = test_game_1.addPlayer(mock_socket)
    new_player.game = mock_game
    new_player.disconnect()
    new_player.setGameExit = self.raiseErrorIfCalled
    new_player.setState = self.raiseErrorIfCalled
    test_game_1.endGame = self.addArgsIfCalled
    test_game_1.finishPlayer(new_player, True)
    self.assertListEquals(self.all_args, [()])
        
    test_game_2 = ViricideGame("test_game_id_2", 1, None)
    new_player2 = test_game_2.addPlayer(mock_socket)
    new_player2.setGameExit("lost")
    new_player2.setState("finished")
    new_player2.setGameExit = self.raiseErrorIfCalled
    new_player2.setState = self.raiseErrorIfCalled
    test_game_2.endGame = self.addArgsIfCalled
    test_game_2.finishPlayer(new_player2, True)
    self.assertListEquals(self.all_args, [(),()])
  
  def testFinishedPlayerFor1Player(self):
    mock_socket = self.mocker.mock()
    mock_socket.sendObject({"message_type": "player_done", "player_number": 0},
        short_int=True)
    mock_socket.sendObject({"message_type": "player_done", "player_number": 0},
        short_int=True)
    self.mocker.replay()

    test_game_3 = ViricideGame("test_game_id_3", 1, None)
    new_player3 = test_game_3.addPlayer(mock_socket)
    test_game_3.endGame = self.addArgsIfCalled
    test_game_3.finishPlayer(new_player3, True)
    self.assertEquals(new_player3.getState(), "finished")
    self.assertEquals(new_player3.getGameExit(), "won")
    self.assertListEquals(self.all_args, [()])
    
    test_game_4 = ViricideGame("test_game_id_4", 1, None)
    new_player4 = test_game_4.addPlayer(mock_socket)
    test_game_4.endGame = self.addArgsIfCalled
    test_game_4.finishPlayer(new_player4, False)
    self.assertEquals(new_player4.getState(), "finished")
    self.assertEquals(new_player4.getGameExit(), "lost")
    self.assertListEquals(self.all_args, [(), ()])

  def setUpFinishedPlayerForMultiplayer(self, sockets_that_close=[],
      game_store_removals=[], notify_disconnected_players=0,
      add_notify_args=True):
    mock_game_store = self.mocker.mock(ViricideGameStore)
    game = self.mocker.mock(ViricideGame)
    for i in xrange(notify_disconnected_players):
      game.notifyDisconnectedPlayer
      def temp(x):
        if add_notify_args:
          self.addArgsIfCalled("notify_disconnected_player", x)
      self.mocker.result(temp)
    mock_sockets = []
    players = []
    for i in xrange(5):
      mock_sockets.append(self.mocker.mock())
      if i in sockets_that_close:
        mock_sockets[i].close()
    for game_id in game_store_removals:
      mock_game_store.removeGameFromGameStore(game_id)
    self.mocker.replay() #expected by testAbortGame
    test_game = ViricideGame("test_game", 5, mock_game_store)
    for i in xrange(5):
      players.append(test_game.addPlayer(mock_sockets[i]))
      players[i].game = game
    return test_game, players

  def assertPlayerStates(self, game, active_ids, connected_ids, winning_id):
    self.assertListEquals(game.activePlayers(),
        [game.all_player_list[i] for i in active_ids])
    self.assertListEquals(game.connectedPlayers(),
        [game.all_player_list[i] for i in connected_ids])
    if winning_id is None:
      self.assertIs(game.winningPlayer(), None)
    else:
      self.assertEquals(game.winningPlayer(),
          game.all_player_list[winning_id])
          
  def testNotifyDisconnectedPlayer(self):
    game1 = self.mocker.mock(ViricideGame)
    game2 = self.mocker.mock(ViricideGame)
    game3 = self.mocker.mock(ViricideGame)
    game4 = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game1.game_lock
    self.mocker.result(threading.RLock())
    game1.all_player_list
    self.mocker.result([player])
    game1.winningPlayer()
    self.mocker.result(object())
    game1.endGame()
    game2.game_lock
    self.mocker.result(threading.RLock())
    game2.all_player_list
    self.mocker.result([player])
    game2.winningPlayer()
    self.mocker.result(None)
    game2.activePlayers()
    self.mocker.result([])
    game2.endGame()
    game3.game_lock
    self.mocker.result(threading.RLock())
    game3.all_player_list
    self.mocker.result([player])
    game3.winningPlayer()
    self.mocker.result(None)
    game3.activePlayers()
    self.mocker.result([1])
    game3.endGame()
    game4.game_lock
    self.mocker.result(threading.RLock())
    game4.all_player_list
    self.mocker.result([player])
    game4.winningPlayer()
    self.mocker.result(None)
    game4.activePlayers()
    self.mocker.result([1,2])
    self.mocker.replay()
    ViricideGame.notifyDisconnectedPlayer(game1, player)
    ViricideGame.notifyDisconnectedPlayer(game2, player)
    ViricideGame.notifyDisconnectedPlayer(game3, player)
    ViricideGame.notifyDisconnectedPlayer(game4, player)

  def testFinishedPlayerForMultiplayer1(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer()

    test_game.endGame = self.addArgsIfCalled
    test_game.sendObject = self.addArgsIfCalled

    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[1], False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},)])
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[0], True)
    self.assertEquals(player[0].getState(), "finished")
    self.assertEquals(player[0].getGameExit(), "won")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [2, 3, 4], [0, 1, 2, 3, 4], 0)

    test_game.finishPlayer(player[2], False)
    self.assertEquals(player[2].getState(), "finished")
    self.assertEquals(player[2].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
      ({'player_number': 1, 'message_type': 'player_done'},), ({'player_number':
      0, 'message_type': 'player_done'},), (), ({'player_number': 2,
      'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [3, 4], [0, 1, 2, 3, 4], 0)

    test_game.finishPlayer(player[3], True)
    self.assertEquals(player[3].getState(), "finished")
    self.assertEquals(player[3].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},), (),
        ({'player_number': 2, 'message_type': 'player_done'},), (),
        ({'player_number': 3, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [4], [0, 1, 2, 3, 4], 0)

  def testFinishedPlayerForMultiplayer2(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,2], [], 2)

    test_game.endGame = self.addArgsIfCalled
    test_game.sendObject = self.addArgsIfCalled
    
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)

    player[0].disconnect()
    self.assertPlayerStates(test_game, [1, 2, 3, 4], [1, 2, 3, 4], None)

    test_game.finishPlayer(player[1], False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[0]), ({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},)])
    self.assertPlayerStates(test_game, [2, 3, 4], [1, 2, 3, 4], None)
    
    player[2].disconnect()
    self.assertPlayerStates(test_game, [3, 4], [1, 3, 4], None)
    
    test_game.finishPlayer(player[3], False)
    self.assertEquals(player[3].getState(), "finished")
    self.assertEquals(player[3].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[0]), ({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ("notify_disconnected_player", player[2]),
        ({'message_type': 'heartbeat'},), ({'player_number': 3, 'message_type':
        'player_done'},), ()])
    self.assertPlayerStates(test_game, [4], [1, 3, 4], None)
    
    test_game.finishPlayer(player[4], False)
    self.assertEquals(player[4].getState(), "finished")
    self.assertEquals(player[4].getGameExit(), "won")
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[0]), ({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ("notify_disconnected_player", player[2]),
        ({'message_type': 'heartbeat'},), ({'player_number': 3, 'message_type':
        'player_done'},), (), ({'message_type': 'heartbeat'},),
        ({'player_number': 4, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [], [1, 3, 4], 4)

    self.assertEquals(player[2].getState(), "disconnected")
    self.assertEquals(player[2].getGameExit(), "lost")
    test_game.finishPlayer(player[2], True)
    self.assertEquals(player[2].getState(), "disconnected")
    self.assertEquals(player[2].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[0]), ({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ("notify_disconnected_player", player[2]),
        ({'message_type': 'heartbeat'},), ({'player_number': 3, 'message_type':
        'player_done'},), (), ({'message_type': 'heartbeat'},),
        ({'player_number': 4, 'message_type': 'player_done'},), (), ()])
    self.assertPlayerStates(test_game, [], [1, 3, 4], 4)
    
  def testFinishedPlayerForMultiplayer3(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0], [], 1)

    test_game.endGame = self.addArgsIfCalled
    test_game.sendObject = self.addArgsIfCalled
    
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[1], False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},)])
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[0], True)
    self.assertEquals(player[0].getState(), "finished")
    self.assertEquals(player[0].getGameExit(), "won")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [2, 3, 4], [0, 1, 2, 3, 4], 0)

    player[0].disconnect()
    self.assertPlayerStates(test_game, [2, 3, 4], [1, 2, 3, 4], 0)

    test_game.finishPlayer(player[2], False)
    self.assertEquals(player[2].getState(), "finished")
    self.assertEquals(player[2].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},), (),
        ("notify_disconnected_player", player[0]),
        ({'player_number': 2, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [3, 4], [1, 2, 3, 4], 0)

    test_game.finishPlayer(player[3], True)
    self.assertEquals(player[3].getState(), "finished")
    self.assertEquals(player[3].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},), (),
        ("notify_disconnected_player", player[0]),
        ({'player_number': 2, 'message_type': 'player_done'},), (),
        ({'player_number': 3, 'message_type': 'player_done'},), ()])
    self.assertPlayerStates(test_game, [4], [1, 2, 3, 4], 0)
    
  def testFinishedPlayerForMultiplayer4(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0, 2, 4], [], 3)

    test_game.endGame = self.addArgsIfCalled
    test_game.sendObject = self.addArgsIfCalled

    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)
        
    test_game.finishPlayer(player[1], False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({"message_type": "heartbeat"},),
        ({"message_type": "player_done", "player_number": 1},)])
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)
    
    player[0].disconnect()
    self.assertPlayerStates(test_game, [2, 3, 4], [1, 2, 3, 4], None)

    player[2].disconnect()
    self.assertPlayerStates(test_game, [3,4], [1,3,4], None)
    
    test_game.finishPlayer(player[3], False)
    self.assertEquals(player[3].getState(), "finished")
    self.assertEquals(player[3].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ("notify_disconnected_player", player[0]),
        ("notify_disconnected_player", player[2]), ({'message_type':
        'heartbeat'},), ({'player_number': 3, 'message_type': 'player_done'},),
        ()])
    self.assertPlayerStates(test_game, [4], [1,3,4], None)

    player[4].disconnect()
    self.assertEquals(player[4].getState(), "disconnected")
    self.assertEquals(player[4].getGameExit(), "lost")
    self.assertPlayerStates(test_game, [], [1,3], None)

    test_game.finishPlayer(player[4], True)
    self.assertEquals(player[4].getState(), "disconnected")
    self.assertEquals(player[4].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ("notify_disconnected_player", player[0]),
        ("notify_disconnected_player", player[2]), 
        ({'message_type': 'heartbeat'},), ({'player_number': 3, 'message_type':
        'player_done'},), (), ("notify_disconnected_player", player[4]), ()])
    self.assertPlayerStates(test_game, [], [1, 3], None)

  def testFinishPlayerForceGameExit1(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,1,2,3,4],
        ["test_game"], 5)
    test_game.sendObject = self.addArgsIfCalled
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)
    test_game.finishPlayer(player[1], False, force_game_exit="won")
    self.assertEquals(player[1].getState(), "disconnected")
    self.assertEquals(player[1].getGameExit(), "won")
    self.assertListEquals(self.all_args, [({'player_number': 1, 'message_type':
        'player_done'},), ({'player_number': 0, 'message_type':
        'player_done'},), ({'player_number': 2, 'message_type':
        'player_done'},), ({'player_number': 3, 'message_type':
        'player_done'},), ({'player_number': 4, 'message_type':
        'player_done'},), ({'message_type': 'game_over',
        'winning_player_number': 1},),
        ("notify_disconnected_player", player[0]),
        ("notify_disconnected_player", player[1]),
        ("notify_disconnected_player", player[2]),
        ("notify_disconnected_player", player[3]),
        ("notify_disconnected_player", player[4]),])
    self.assertPlayerStates(test_game, [], [], 1)
    
  def testFinishPlayerForceGameExit2(self):
    mock_socket = self.mocker.mock()
    mock_game_store = self.mocker.mock(ViricideGameStore)
    mock_game_store.removeGameFromGameStore("test_game")
    mock_socket.close()
    self.mocker.replay()
    test_game = ViricideGame("test_game", 1, mock_game_store)
    player = test_game.addPlayer(mock_socket)
    test_game.sendObject = self.addArgsIfCalled
    self.assertPlayerStates(test_game, [0], [0], None)
    test_game.finishPlayer(player, True, force_game_exit="lost")
    self.assertEquals(player.getState(), "disconnected")
    self.assertEquals(player.getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'player_number': 0, 'message_type':
        'player_done'},), ({'message_type': 'game_over',
        'winning_player_number': None},)])
    self.assertPlayerStates(test_game, [], [], None)
    
  def testStartGameAndReadyToStart(self):
    mock_socket = self.mocker.mock()
    mock_socket2 = self.mocker.mock()
    self.mocker.replay()
    test_game = ViricideGame("test_game_2", 2, None)
    test_game.sendObject = self.addArgsIfCalled
    test_game.addPlayer(mock_socket)
    self.assertRaises(NotReadyToStart, test_game.startGame)
    self.assertListEquals(self.all_args, [({"message_type": "heartbeat"},)])
    test_game.addPlayer(mock_socket2)
    test_game.startGame()
    self.assertListEquals(self.all_args, [({"message_type": "heartbeat"},),
        ({"message_type": "heartbeat"},), ({'message_type': 'game_start'},)])
    test_game_2 = ViricideGame("test_game_3", 1, None)
    self.all_args = []
    test_game_2.sendObject = self.addArgsIfCalled
    test_game_2.addPlayer(mock_socket)
    self.assert_(test_game_2.readyToStart())
    test_game_2.started = True
    self.assert_(not test_game_2.readyToStart())
    self.assertRaises(NotReadyToStart, test_game_2.startGame)
    self.assertListEquals(self.all_args, [({"message_type": "heartbeat"},),
        ({"message_type": "heartbeat"},), ({"message_type": "heartbeat"},)])
  
  def testEndGame1(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,1,2,3,4],
        ["test_game"], 5)

    test_game.sendObject = self.addArgsIfCalled

    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[1], False)
    self.assertEquals(test_game.game_over, False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},)])
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[0], True)
    self.assertEquals(test_game.game_over, True)
    self.assertEquals(player[0].getState(), "disconnected")
    self.assertEquals(player[0].getGameExit(), "won")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'player_number': 0, 'message_type': 'player_done'},),
        ({'player_number': 2, 'message_type': 'player_done'},),
        ({'player_number': 3, 'message_type': 'player_done'},),
        ({'player_number': 4, 'message_type': 'player_done'},),
        ({'message_type': 'game_over', 'winning_player_number': 0},),
        ("notify_disconnected_player", player[0]),
        ("notify_disconnected_player", player[1]),
        ("notify_disconnected_player", player[2]),
        ("notify_disconnected_player", player[3]),
        ("notify_disconnected_player", player[4]),])
    self.assertPlayerStates(test_game, [], [], 0)

  def testEndGame2(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,1,2,3,4],
        ["test_game"], 5)
    test_game.sendObject = self.addArgsIfCalled
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[1], False)
    self.assertEquals(test_game.game_over, False)
    self.assertEquals(player[1].getState(), "finished")
    self.assertEquals(player[1].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},)])
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)

    test_game.finishPlayer(player[0], False)
    self.assertEquals(test_game.game_over, False)
    self.assertEquals(player[0].getState(), "finished")
    self.assertEquals(player[0].getGameExit(), "lost")
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'message_type': 'heartbeat'},), ({'player_number': 0, 'message_type':
        'player_done'},)])
    self.assertPlayerStates(test_game, [2,3,4], [0, 1, 2, 3, 4], None)
    
    test_game.endGame()
    self.assertEquals(test_game.game_over, True)
    self.assertListEquals(self.all_args, [({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},),
        ({'message_type': 'heartbeat'},), ({'player_number': 0, 'message_type':
        'player_done'},), ({'player_number': 2, 'message_type':
        'player_done'},), ({'player_number': 3, 'message_type':
        'player_done'},), ({'player_number': 4, 'message_type':
        'player_done'},), ({'message_type': 'game_over',
        'winning_player_number': None},),
        ("notify_disconnected_player", player[0]),
        ("notify_disconnected_player", player[1]),
        ("notify_disconnected_player", player[2]),
        ("notify_disconnected_player", player[3]),
        ("notify_disconnected_player", player[4]),])
    self.assertPlayerStates(test_game, [], [], None)
  
  def testAbortGame(self):
    test_event_1 = self.mocker.mock()
    test_event_2 = self.mocker.mock()
    test_event_1.set()
    test_event_2.set()
    
    # setUpFinishedPlayerForMultiplayer calls self.mocker.replay()
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,1,2,3,4],
        ["test_game"], 5)
    test_game.game_over_event = test_event_1
    test_game.game_start_event = test_event_2
    
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)
    self.assertEquals(test_game.started, False)
    self.assertEquals(test_game.game_over, False)
    
    test_game.abortGame()
    self.assertPlayerStates(test_game, [], [], None)
    self.assertEquals(test_game.started, True)
    self.assertEquals(test_game.game_over, True)
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[i]) for i in xrange(5)])

  def testGetVirusPlacements(self):
    self.assertEquals(len(self.test_game.virus_placements), 0)
    virus_placements_1 = self.test_game.getVirusPlacements(80,40,4,25)
    self.assertEquals(len(self.test_game.virus_placements), 1)
    virus_placements_2 = self.test_game.getVirusPlacements(79,39,5,24)
    self.assertEquals(len(self.test_game.virus_placements), 2)
    self.assertListNotEquals(virus_placements_1, virus_placements_2)
    virus_placements_3 = self.test_game.getVirusPlacements(80,40,4,25)
    self.assertEquals(len(self.test_game.virus_placements), 2)
    self.assertListEquals(virus_placements_1, virus_placements_3)
    virus_placements_4 = self.test_game.getVirusPlacements(79,39,5,24)
    self.assertEquals(len(self.test_game.virus_placements), 2)
    self.assertListEquals(virus_placements_2, virus_placements_4)
    
  def testGetNewPills(self):
    # so, these assertions will occasionally fail, because pills are generated
    # randomly and might be the same pills. however, we want to make sure that
    # similarities are simply a result of the inherent randomness and are not
    # consistent. if we get errors everytime, then perhaps there is a more
    # endemic problem. so run it a few times
    assertion_errors = []
    for i in xrange(3):
      try:
        test_game = ViricideGame("test_game", 3, None)
        self.assertEquals(len(test_game.new_pill_list), 0)
        pill1 = test_game.getNewPills(6)
        self.assertEquals(len(test_game.new_pill_list), 7)
        pill2 = test_game.getNewPills(6)
        self.assertEquals(len(test_game.new_pill_list), 7)
        self.assertListEquals(pill1, pill2)
        pill3 = test_game.getNewPills(3)
        self.assertEquals(len(test_game.new_pill_list), 7)
        self.assertListNotEquals(pill1, pill3)
        pill4 = test_game.getNewPills(3)
        self.assertEquals(len(test_game.new_pill_list), 7)
        self.assertListEquals(pill4, pill3)
        pill5 = test_game.getNewPills(0)
        self.assertEquals(len(test_game.new_pill_list), 7)
        pill6 = test_game.getNewPills(7)
        self.assertEquals(len(test_game.new_pill_list), 8)
        pill7 = test_game.getNewPills(20)
        self.assertEquals(len(test_game.new_pill_list), 21)
      except AssertionError, e:
        assertion_errors.append(e)
    if len(assertion_errors) > 2:
      raise AssertionError, ", ".join([str(e) for e in assertion_errors])
  
  def testSendObject(self):
    test_game, players = self.setUpFinishedPlayerForMultiplayer([3, 2, 0, 1, 4],
        ["test_game"], 5, False)
    for player in players:
      player.sendObject = self.addArgsIfCalled
    test_game.sendObject("test_obj", short_int="nonce4")
    self.assertListEquals(self.all_args, [("test_obj",)] * len(players))
    self.assertListEquals(self.all_kwargs,
        [{'short_int': 'nonce4'}] * len(players))
    self.all_args, self.all_kwargs = [], []
    players[3].disconnect()
    test_game.sendObject("test_obj2", short_int="nonce5")
    self.assertListEquals(self.all_args, [("test_obj2",)] * (len(players) - 1))
    self.assertListEquals(self.all_kwargs, 
        [{'short_int': 'nonce5'}] * (len(players) - 1))
    self.all_args, self.all_kwargs = [], []
    test_game.finishPlayer(players[1], False)
    self.assertIs(test_game.winningPlayer(), None)
    test_game.sendObject("test_obj3", short_int="nonce6")
    self.assertListEquals(self.all_args, ([({"message_type": "heartbeat"},)] *
        (len(players) - 1) + [({"message_type": "player_done",
        "player_number": 1},)] * (len(players) - 1) + [("test_obj3",)] *
        (len(players) - 1)))
    self.assertListEquals(self.all_kwargs, ([{'short_int': True}] * (2 *
        (len(players) - 1)) + [{'short_int': 'nonce6'}] * (len(players) - 1)))
    self.all_args, self.all_kwargs = [], []
    test_game.finishPlayer(players[4], True)
    self.assertEquals(test_game.winningPlayer(), players[4])
    test_game.sendObject("test_obj4", short_int="nonce7")
    self.assertListEquals(self.all_args, [({"message_type": "player_done",
        "player_number": 4},)] * (len(players) - 1) + [({"message_type":
        "player_done", "player_number": 0},)] * (len(players) - 1) +
        [({"message_type": "player_done", "player_number": 2},)] * 
        (len(players) - 1) + [({"message_type": "game_over",
        "winning_player_number": 4},)] * (len(players) - 1))
    self.assertListEquals(self.all_kwargs, [{'short_int': True}] *
        ((len(players) - 1) * 4))
    self.all_args, self.all_kwargs = [], []
    players[2].disconnect()
    self.assertEquals(test_game.winningPlayer(), players[4])
    test_game.sendObject("test_obj5", short_int="nonce8")
    self.assertListEquals(self.all_args, [])
    self.assertListEquals(self.all_kwargs, [])
      
  def testActivePlayerAfter1(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([1,2,4], [], 3)
    for p in player:
      p.sendObject = self.addArgsIfCalled
    self.assertEquals(test_game.activePlayerAfter(player[0]), player[1])
    self.assertEquals(test_game.activePlayerAfter(player[1]), player[2])
    self.assertEquals(test_game.activePlayerAfter(player[2]), player[3])
    self.assertEquals(test_game.activePlayerAfter(player[3]), player[4])
    self.assertEquals(test_game.activePlayerAfter(player[4]), player[0])
    player[1].disconnect()
    self.assertEquals(test_game.activePlayerAfter(player[0]), player[2])
    self.assertEquals(test_game.activePlayerAfter(player[2]), player[3])
    self.assertEquals(test_game.activePlayerAfter(player[3]), player[4])
    self.assertEquals(test_game.activePlayerAfter(player[4]), player[0])
    player[4].disconnect()
    self.assertEquals(test_game.activePlayerAfter(player[0]), player[2])
    self.assertEquals(test_game.activePlayerAfter(player[2]), player[3])
    self.assertEquals(test_game.activePlayerAfter(player[3]), player[0])
    test_game.finishPlayer(player[3], False)
    self.assertEquals(test_game.activePlayerAfter(player[0]), player[2])
    self.assertEquals(test_game.activePlayerAfter(player[2]), player[0])
    player[2].disconnect()
    self.assertIs(test_game.activePlayerAfter(player[0]), None)
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[1])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[2])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[3])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[4])
    self.assertListEquals(self.all_args, [("notify_disconnected_player",
        player[i]) for i in [1, 4]] + [({'message_type': 'heartbeat'},)] * 3
        + [({'message_type': 'player_done', "player_number": 3},)] * 3 +
        [("notify_disconnected_player", player[2])])
    self.assertListEquals(self.all_kwargs, [{}] * 2 + [{'short_int': True}] * 6
        + [{}])
    
  def testActivePlayerAfter2(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,1,2,3,4],
        ["test_game"], 5)
    for p in player:
      p.sendObject = self.addArgsIfCalled
    self.assertEquals(test_game.activePlayerAfter(player[0]), player[1])
    self.assertEquals(test_game.activePlayerAfter(player[1]), player[2])
    self.assertEquals(test_game.activePlayerAfter(player[2]), player[3])
    self.assertEquals(test_game.activePlayerAfter(player[3]), player[4])
    self.assertEquals(test_game.activePlayerAfter(player[4]), player[0])
    test_game.finishPlayer(player[3], True)
    self.assertEquals(test_game.game_over, True)
    self.assertEquals(test_game.winningPlayer(), player[3])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[0])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[1])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[2])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[3])
    self.assertRaises(AssertionError, test_game.activePlayerAfter, player[4])
    sent_args = []
    for i in [3,0,1,2,4]:
      sent_args.extend([({'player_number': i, 'message_type': 'player_done'},)]
          * len(player))
    sent_args.extend([({'message_type': 'game_over', "winning_player_number":
        3},)] * len(player))
    sent_args.extend([("notify_disconnected_player", player[i])
        for i in xrange(5)])
    self.assertListEquals(self.all_args, sent_args)
    self.assertListEquals(self.all_kwargs, [{"short_int": True}] *
        (len(player)**2+len(player)) + [{}] * len(player))

  def testSendPlayerCountUpdate(self):
    test_game, player = self.setUpFinishedPlayerForMultiplayer([0,2],[],2)
    test_game.sendObject = self.addArgsIfCalled
    self.assertListEquals(self.all_args, [])
    test_game.sendPlayerCountUpdate()
    self.assertPlayerStates(test_game, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], None)
    test_game.finishPlayer(player[1], False)
    test_game.sendPlayerCountUpdate()
    self.assertPlayerStates(test_game, [0, 2, 3, 4], [0, 1, 2, 3, 4], None)
    player[0].disconnect()
    test_game.sendPlayerCountUpdate()
    self.assertPlayerStates(test_game, [2, 3, 4], [1, 2, 3, 4], None)
    player[2].disconnect()
    test_game.sendPlayerCountUpdate()
    self.assertListEquals(self.all_args, [({'count': 5, 'message_type':
        'player_count_update'},), ({'message_type': 'heartbeat'},),
        ({'player_number': 1, 'message_type': 'player_done'},), ({'count': 5,
        'message_type': 'player_count_update'},), ("notify_disconnected_player",
        player[0]), ({'count': 4, 'message_type': 'player_count_update'},),
        ("notify_disconnected_player", player[2]), ({'count': 3, 'message_type':
        'player_count_update'},)])
    self.assertPlayerStates(test_game, [3,4], [1,3,4], None)

  def testSendVirusCountUpdate(self):
    mock_socket = self.mocker.mock()
    self.mocker.replay()
    self.test_game.sendObject = self.addArgsIfCalled
    player = self.test_game.addPlayer(mock_socket)
    self.test_game.sendVirusCountUpdate(player, "nonce")
    self.assertListEquals(self.all_args, [({'player_number': 0, 'message_type':
        'virus_number_update', 'virus_number': 'nonce'},)])
    self.assertRaises(PlayerDoesntExist, self.test_game.sendVirusCountUpdate,
        "nonce1", "nonce2")
    

class TestViricideGameStore(ExtendedTestCase):

  def testInit(self):
    test_game_store = ViricideGameStore(timeout_thread_class="nonce")
    self.assertEquals(len(test_game_store.games_dict), 0)
    self.assertEquals(test_game_store.timeout_thread_class, "nonce")
  
  def testGetGame(self):
    test_game_store = ViricideGameStore(timeout_thread_class=None)
    self.assertEquals(len(test_game_store.games_dict), 0)
    test_game_store.games_dict["test_game_id_1"] = "nonce"
    self.assertEquals(len(test_game_store.games_dict), 1)
    self.assertEquals(test_game_store.getGame("test_game_id_1"), "nonce")
    self.assertEquals(test_game_store.getGame("test_game_id_2"), None)
  
  def testEndGame(self):
    test_game_store = ViricideGameStore(timeout_thread_class=None)
    self.assertEquals(len(test_game_store.games_dict), 0)
    test_game = self.mocker.mock(ViricideGame)
    test_game.endGame()
    self.mocker.replay()
    test_game_store.games_dict["test_game_id_1"] = test_game
    self.assertEquals(test_game_store.getGame("test_game_id_1"), test_game)
    self.assertEquals(test_game_store.getGame("test_game_id_2"), None)
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.endGame("test_game_id_2")
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.endGame("test_game_id_1")
    self.assertEquals(len(test_game_store.games_dict), 1)
    self.assertEquals(test_game_store.getGame("test_game_id_1"), test_game)
    self.assertEquals(test_game_store.getGame("test_game_id_2"), None)
  
  def testAbortGame(self):
    test_game_store = ViricideGameStore(timeout_thread_class=None)
    self.assertEquals(len(test_game_store.games_dict), 0)
    test_game = self.mocker.mock(ViricideGame)
    test_game.abortGame()
    self.mocker.replay()
    test_game_store.games_dict["test_game_id_1"] = test_game
    self.assertEquals(test_game_store.getGame("test_game_id_1"), test_game)
    self.assertEquals(test_game_store.getGame("test_game_id_2"), None)
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.abortGame("test_game_id_2")
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.abortGame("test_game_id_1")
    self.assertEquals(len(test_game_store.games_dict), 1)
    self.assertEquals(test_game_store.getGame("test_game_id_1"), test_game)
    self.assertEquals(test_game_store.getGame("test_game_id_2"), None)
  
  def testCreateGame(self):
    number_of_players = random.randint(1,1000)
    thread_instantiation = self.mocker.mock()
    thread_instantiation.start()
    thread_class = self.mocker.mock()
    thread_class.init(mocker.MATCH(lambda x: isinstance(x, ViricideGameStore)),
        "test_game_id")
    self.mocker.result(thread_instantiation)
    self.mocker.replay()
    test_game_store = ViricideGameStore(timeout_thread_class=thread_class.init)
    self.assertEquals(test_game_store.getGame("test_game_id"), None)
    self.assertEquals(len(test_game_store.games_dict), 0)
    test_game = test_game_store.createGame("test_game_id", number_of_players)
    self.assertEquals(len(test_game_store.games_dict), 1)
    self.assertEquals(test_game_store.getGame("test_game_id"), test_game)
    self.assertEquals(test_game.starting_amount_of_players, number_of_players)
    self.assertRaises(WrongNumberOfPlayers, test_game_store.createGame,
        "test_game_id", number_of_players + 1)
    self.assertEquals(len(test_game_store.games_dict), 1)
    self.assertEquals(test_game, test_game_store.createGame("test_game_id",
        number_of_players))
    self.assertEquals(len(test_game_store.games_dict), 1)

  def testRemoveGameFromGameStore(self):
    test_game_store = ViricideGameStore()
    test_game_store.games_dict["test1"] = "nonce"
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.removeGameFromGameStore("test2")
    self.assertEquals(len(test_game_store.games_dict), 1)
    test_game_store.removeGameFromGameStore("test1")
    self.assertEquals(len(test_game_store.games_dict), 0)

  
class TestViricideGameStartTimeoutThread(ExtendedTestCase):

  def setUp(self):
    ExtendedTestCase.setUp(self)
    self.old_game_start_timeout = FLAGS.game_start_timeout
  
  def tearDown(self):
    FLAGS.game_start_timeout = self.old_game_start_timeout
    self.assertNotEquals(FLAGS.game_start_timeout, "nonce")
    ExtendedTestCase.tearDown(self)

  def testInit1(self):
    game_store = self.mocker.mock(ViricideGameStore)
    self.mocker.replay()
    FLAGS.game_start_timeout = "nonce"
    thread = ViricideGameStartTimeoutThread(game_store, "game_id")
    self.assertEquals(thread.game_store, game_store)
    self.assertEquals(thread.game_id, "game_id")
    self.assertEquals(thread.last_player_count, -1)
    self.assertEquals(thread.termination_countdown, "nonce")
    self.assertEquals(thread.game_start_timeout, "nonce")
    self.assert_(thread.isDaemon())

  def testInit2(self):
    game_store = self.mocker.mock(ViricideGameStore)
    self.mocker.replay()
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout="nonce2")
    self.assertEquals(thread.game_store, game_store)
    self.assertEquals(thread.game_id, "game_id")
    self.assertEquals(thread.last_player_count, -1)
    self.assertEquals(thread.termination_countdown, "nonce2")
    self.assertEquals(thread.game_start_timeout, "nonce2")
    self.assert_(thread.isDaemon())
  
  def testRun1(self):
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(None)
    self.mocker.replay()
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout=20)
    thread.run()
    self.assertEquals(thread.termination_countdown, 20)

  def testRun2(self):
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(True)
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    self.mocker.replay()
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout=20)
    thread.run()
    self.assertEquals(thread.termination_countdown, 20)
  
  def testRun3a(self):
    if FLAGS.ignore_time_based_tests:
      print "ignoring testRun3a"
      return
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([])
    game.started
    self.mocker.result(True)
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    self.mocker.replay()
  
    FLAGS.game_start_timeout = 31
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout=21)
    thread.termination_countdown = 10
    x = time.time()
    thread.run()
    x = time.time() - x
    self.assertApproximates(x, 1.0, .01)
    self.assertEquals(thread.termination_countdown, 20)
    self.assertEquals(thread.last_player_count, 0)
    self.assertEquals(thread.max_player_count, 0)
  
  def testRun3b(self):
    if FLAGS.ignore_time_based_tests:
      print "ignoring testRun3b"
      return
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([])
    game.started
    self.mocker.result(True)
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    self.mocker.replay()
  
    FLAGS.game_start_timeout = 31
    thread = ViricideGameStartTimeoutThread(game_store, "game_id")
    thread.termination_countdown = 10
    x = time.time()
    thread.run()
    x = time.time() - x
    self.assertApproximates(x, 1.0, .01)
    self.assertEquals(thread.termination_countdown, 30)
    self.assertEquals(thread.last_player_count, 0)
    self.assertEquals(thread.max_player_count, 0)
  
  def testRun4a(self):
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([1,2,3,4])
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.abortGame("game_id")
    self.mocker.replay()
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
      game_start_timeout=0)
    thread.last_player_count = 3
    thread.termination_countdown = 10
    thread.run()
    self.assertEquals(thread.termination_countdown, 0)
    self.assertEquals(thread.last_player_count, 4)
  
  def testRun4b(self):
    if FLAGS.ignore_time_based_tests:
      print "ignoring testRun4b"
      return
    game = self.mocker.mock(ViricideGame)
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game.started
    self.mocker.result(False)
    game.started
    self.mocker.result(True)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([1,2,3,4])
    self.mocker.replay()
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
      game_start_timeout=30)
    thread.last_player_count = 3
    thread.termination_countdown = 10
    thread.run()
    self.assertEquals(thread.termination_countdown, 29)
    self.assertEquals(thread.last_player_count, 4)

  def testRun5(self):
    if FLAGS.ignore_time_based_tests:
      print "ignoring testRun5"
      return
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([])
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([1])
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([])
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.abortGame("game_id")
    self.mocker.replay()
  
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout=21)
    thread.termination_countdown = 10
    x = time.time()
    thread.run()
    x = time.time() - x
    self.assertApproximates(x, 2.0, .01)
    self.assertEquals(thread.termination_countdown, 21)
    self.assertEquals(thread.last_player_count, 0)
    self.assertEquals(thread.max_player_count, 1)
  
  def testRun6(self):
    if FLAGS.ignore_time_based_tests:
      print "ignoring testRun6"
      return
    game = self.mocker.mock(ViricideGame)
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([])
    game.started
    self.mocker.result(False)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers(check_with_heartbeat=True)
    self.mocker.result([1])
    game.started
    self.mocker.result(True)
    game_store = self.mocker.mock(ViricideGameStore)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    game_store.getGame("game_id")
    self.mocker.result(game)
    self.mocker.replay()
  
    thread = ViricideGameStartTimeoutThread(game_store, "game_id",
        game_start_timeout=21)
    thread.termination_countdown = 10
    x = time.time()
    thread.run()
    x = time.time() - x
    self.assertApproximates(x, 2.0, .01)
    self.assertEquals(thread.termination_countdown, 20)
    self.assertEquals(thread.last_player_count, 1)
    self.assertEquals(thread.max_player_count, 1)


class OldStyleTestObject(): pass
class TestViricideConnectionHandler(ExtendedTestCase):

  def constructHandleFreeHandler(self, *init_args, **init_kwargs):
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.handle = lambda: None
    SocketServer.BaseRequestHandler.__init__(handler, *init_args, **init_kwargs)
    return handler
  
  def testSendErrors1(self):
    socket = self.mocker.mock()
    socket.sendObject({"success": False, "errors": "nonce1"}, short_int=False)
    server = self.mocker.mock(ViricideServer)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.errors = "nonce1"
    handler.responded = False
    handler.sendErrors()
    self.assert_(handler.responded)
  
  def testSendErrors2(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.errors = "nonce1"
    handler.responded = True
    self.assertRaises(AssertionError, handler.sendErrors)
  
  def testSendErrors3(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.errors = []
    handler.responded = False
    self.assertRaises(AssertionError, handler.sendErrors)
  
  def testSendCombos1(self):
    main_player = self.mocker.mock(ViricidePlayer)
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos([], cells.COLORS)
    
  def testSendCombos2(self):
    main_player = self.mocker.mock(ViricidePlayer)
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(main_player)
    self.mocker.result(None)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos(cells.COLORS, cells.COLORS)

  def testSendCombos3(self):
    main_player = self.mocker.mock(ViricidePlayer)
    other_player = self.mocker.mock(ViricidePlayer)
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(main_player)
    self.mocker.result(other_player)
    game.activePlayerAfter(main_player)
    self.mocker.result(main_player)
    game.activePlayerAfter(main_player)
    self.mocker.result(main_player)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    self.assertRaises(AssertionError, handler.sendCombos, cells.COLORS,
        cells.COLORS)
    
  def testSendCombos4(self):
    main_player = self.mocker.mock(ViricidePlayer)
    other_player = self.mocker.mock(ViricidePlayer)
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(main_player)
    self.mocker.result(other_player)
    game.activePlayerAfter(main_player)
    self.mocker.result(other_player)
    other_player.sendObject({"message_type": "combos", "combos": cells.COLORS +
        cells.COLORS})
    for i in xrange(len(cells.COLORS)-1):
      game.activePlayerAfter(other_player)
      self.mocker.result(main_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos(cells.COLORS, cells.COLORS)
    
  def testSendCombos5(self):
    with self.mocker.order():
      main_player = self.mocker.mock(ViricidePlayer)
      other_player = self.mocker.mock(ViricidePlayer)
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game = self.mocker.mock(ViricideGame)
      game.game_lock
      self.mocker.result(threading.RLock())
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      other_player.sendObject({"message_type": "combos", "combos": ["red",
          "blue"]})
      for i in xrange(len(cells.COLORS)-1):
        game.activePlayerAfter(other_player)
        self.mocker.result(main_player)
        game.activePlayerAfter(main_player)
        self.mocker.result(other_player)
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos(["red"], ["blue"])
    
  def testSendCombos6(self):
    with self.mocker.order():
      main_player = self.mocker.mock(ViricidePlayer)
      other_player = self.mocker.mock(ViricidePlayer)
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game = self.mocker.mock(ViricideGame)
      game.game_lock
      self.mocker.result(threading.RLock())
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(other_player)
      self.mocker.result(main_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(other_player)
      self.mocker.result(main_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      other_player.sendObject({"message_type": "combos", "combos": ["blue",
          "blue"]})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos(["blue"], ["blue"])
    
  def testSendCombos7(self):
    with self.mocker.order():
      main_player = self.mocker.mock(ViricidePlayer)
      other_player = self.mocker.mock(ViricidePlayer)
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game = self.mocker.mock(ViricideGame)
      game.game_lock
      self.mocker.result(threading.RLock())
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      game.activePlayerAfter(other_player)
      self.mocker.result(main_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
      other_player.sendObject({"message_type": "combos", "combos": ["green",
          "red", "blue"]})
      game.activePlayerAfter(other_player)
      self.mocker.result(main_player)
      game.activePlayerAfter(main_player)
      self.mocker.result(other_player)
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = main_player
    handler.sendCombos(["green"], ["red", "blue"])

  def testSendCombos8(self):
    player = []
    for i in xrange(5):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[3])
    player[1].sendObject({"message_type": "combos", "combos": cells.COLORS +
        cells.COLORS})
    player[2].sendObject({"message_type": "combos", "combos": cells.COLORS +
        cells.COLORS})
    player[3].sendObject({"message_type": "combos", "combos": cells.COLORS +
        cells.COLORS})
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(cells.COLORS, cells.COLORS)
    
  def testSendCombos9(self):
    player = []
    for i in xrange(5):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[3])
    player[2].sendObject({"message_type": "combos", "combos": ["green"] +
        cells.COLORS})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["green"], cells.COLORS)
    
  def testSendCombos10(self):
    player = []
    for i in xrange(5):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[3])
    player[2].sendObject({"message_type": "combos", "combos": ["blue", "green"]
        + cells.COLORS})
    player[3].sendObject({"message_type": "combos", "combos": ["blue", "green"]
        + cells.COLORS})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["blue", "green"], cells.COLORS)
    
  def testSendCombos11(self):
    player = []
    for i in xrange(3):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[0])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    player[1].sendObject({"message_type": "combos", "combos": ["blue", "red"]
        + cells.COLORS})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["blue", "red"], cells.COLORS)
    
  def testSendCombos12(self):
    player = []
    for i in xrange(3):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[0])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    player[1].sendObject({"message_type": "combos", "combos": ["red", "green"]})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["red"], ["green"])
    
  def testSendCombos13(self):
    player = []
    for i in xrange(3):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[0])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    player[1].sendObject({"message_type": "combos", "combos": ["blue", "blue",
        "green"]})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["blue"], ["blue", "green"])
    
  def testSendCombos13(self):
    player = []
    for i in xrange(3):
      player.append(self.mocker.mock(ViricidePlayer))
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    game.activePlayerAfter(player[1])
    self.mocker.result(player[2])
    game.activePlayerAfter(player[2])
    self.mocker.result(player[0])
    game.activePlayerAfter(player[0])
    self.mocker.result(player[1])
    player[2].sendObject({"message_type": "combos", "combos": ["green", "green",
        "blue"]})
    self.mocker.replay()
    self.assertEquals(cells.COLORS, ["red", "green", "blue"])
    handler = self.constructHandleFreeHandler(socket, ("localhost", 1234),
        server)
    handler.game = game
    handler.player = player[0]
    handler.sendCombos(["green"], ["green", "blue"])

  def testRunGame1(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    game.readyToStart()
    self.mocker.result(False)
    game.waitForGameStart()
    game.game_over
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.runGame()
  
  def testRunGame2(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.throw(network.UnexpectedSocketClose)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.runGame()
  
  def testRunGame3(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.result(None)
    game.game_over
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.runGame()
  
  def testRunGame3(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.result(None)
    for i in xrange(20):
      game.game_over
      self.mocker.result(False)
      player.getObject()
      self.mocker.result({"message_type": "needs_pills"})
      game.getNewPills(i)
      self.mocker.result("nonce")
      player.sendObject({"message_type": "new_pills", "pill_colors": "nonce"})
    game.game_over
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.runGame()
  
  def testRunGame4(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.result({"message_type": "combos", "first_colors": "nonce1",
        "other_colors": "nonce2"})
    game.game_over
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.sendCombos = self.addArgsIfCalled
    handler.runGame()
    self.assertListEquals(self.all_args, [("nonce1", "nonce2")])
  
  def assistGameOverTest(self, virus_number, viruses_cleared):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.result({"message_type": "game_over", "virus_number":
        virus_number})
    game.finishPlayer(player, viruses_cleared)
    game.waitForGameEnd()
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.runGame()
  
  def testRunGame5(self):
    self.assistGameOverTest(0, True)
  
  def testRunGame6(self):
    self.assistGameOverTest(-1, False)
  
  def testRunGame7(self):
    self.assistGameOverTest(2, False)

  def testRunGame8(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game = self.mocker.mock(ViricideGame)
    player = self.mocker.mock(ViricidePlayer)
    game.readyToStart()
    self.mocker.result(True)
    game.startGame()
    game.waitForGameStart()
    game.game_over
    self.mocker.result(False)
    player.getObject()
    self.mocker.result({"message_type": "virus_number_update", "virus_number":
        "nonce"})
    game.sendVirusCountUpdate(player, "nonce")
    game.game_over
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("123.210.123.210",
        3241), server)
    handler.game = game
    handler.player = player
    handler.runGame()
  
  def testJoinGame1(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game_store = self.mocker.mock(ViricideGameStore)
    server.game_store
    self.mocker.result(game_store)
    game_store.createGame("nonce1", "nonce2")
    self.mocker.result("nonce3")
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = "nonce2"
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Unknown game creation error."])
    self.assertListEquals(self.all_args, [()])
  
  def testJoinGame2(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game_store = self.mocker.mock(ViricideGameStore)
    game = self.mocker.mock(ViricideGame)
    server.game_store
    self.mocker.result(game_store)
    game_store.createGame("nonce1", 3)
    self.mocker.result(game)
    game.starting_amount_of_players
    self.mocker.result(3)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers()
    self.mocker.result([1,2,3,4])
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Game room full"])
    self.assertListEquals(self.all_args, [()])
  
  def testJoinGame3(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game_store = self.mocker.mock(ViricideGameStore)
    game = self.mocker.mock(ViricideGame)
    server.game_store
    self.mocker.result(game_store)
    game_store.createGame("nonce1", 3)
    self.mocker.result(game)
    game.starting_amount_of_players
    self.mocker.result(3)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers()
    self.mocker.result([1,2,3])
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Game room full"])
    self.assertListEquals(self.all_args, [()])
  
  def testJoinGame4(self):
    socket = self.mocker.mock()
    server = self.mocker.mock(ViricideServer)
    game_store = self.mocker.mock(ViricideGameStore)
    game = self.mocker.mock(ViricideGame)
    server.game_store
    self.mocker.result(game_store)
    game_store.createGame("nonce1", 3)
    self.mocker.result(game)
    game.starting_amount_of_players
    self.mocker.result(3)
    game.game_lock
    self.mocker.result(threading.RLock())
    game.activePlayers()
    self.mocker.result([1,2])
    game.started
    self.mocker.result(True)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Game already started"])
    self.assertListEquals(self.all_args, [()])
  
  def testJoinGame5(self):
    with self.mocker.order():
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game_store = self.mocker.mock(ViricideGameStore)
      game = self.mocker.mock(ViricideGame)
      player = self.mocker.mock(ViricidePlayer)
      server.game_store
      self.mocker.result(game_store)
      game_store.createGame("nonce1", 3)
      self.mocker.result(game)
      game.starting_amount_of_players
      self.mocker.result(3)
      game.game_lock
      self.mocker.result(threading.RLock())
      game.activePlayers()
      self.mocker.result([1,2])
      game.started
      self.mocker.result(False)
      game.getVirusPlacements("nonce2", "nonce3", "nonce4", "nonce5")
      self.mocker.result("nonce6")
      socket.sending_lock
      self.mocker.result(threading.RLock())
      game.addPlayer(socket)
      self.mocker.result(player)
    player.player_number
    self.mocker.result("nonce8")
    game.connectedPlayers()
    self.mocker.result([1,2,3])
    player.sendObject({
        "success": True,
        "current_number_of_players": 3,
        "virus_placements": "nonce6",
        "player_number": "nonce8",
      }, short_int=False)
    game.sendPlayerCountUpdate()
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.runGame = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.rows = "nonce2"
    handler.cols = "nonce3"
    handler.combo_length = "nonce4"
    handler.virus_number = "nonce5"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, [])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.player, player)
    self.assertEquals(handler.responded, True)
  
  def testJoinGame6(self):
    with self.mocker.order():
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game_store = self.mocker.mock(ViricideGameStore)
      server.game_store
      self.mocker.result(game_store)
      game_store.createGame("nonce1", 3)
      self.mocker.throw(WrongNumberOfPlayers)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Wrong number of players: "])
    self.assertListEquals(self.all_args, [()])
  
  def testJoinGame7(self):
    with self.mocker.order():
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game_store = self.mocker.mock(ViricideGameStore)
      game = self.mocker.mock(ViricideGame)
      server.game_store
      self.mocker.result(game_store)
      game_store.createGame("nonce1", 3)
      self.mocker.result(game)
      game.starting_amount_of_players
      self.mocker.result(4)
      game.starting_amount_of_players
      self.mocker.result(4)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.starting_amount_of_players = 3
    self.assertRaises(StartingPlayerAmountMismatch, handler.joinGame)
    self.assertListEquals(handler.errors, [])
    self.assertListEquals(self.all_args, [])
  
  def testJoinGame8(self):
    with self.mocker.order():
      socket = self.mocker.mock()
      server = self.mocker.mock(ViricideServer)
      game_store = self.mocker.mock(ViricideGameStore)
      game = self.mocker.mock(ViricideGame)
      server.game_store
      self.mocker.result(game_store)
      game_store.createGame("nonce1", 3)
      self.mocker.result(game)
      game.starting_amount_of_players
      self.mocker.result(3)
      game.game_lock
      self.mocker.result(threading.RLock())
      game.activePlayers()
      self.mocker.result([1,2])
      game.started
      self.mocker.result(False)
      game.getVirusPlacements("nonce2", "nonce3", "nonce4", "nonce5")
      self.mocker.throw(cells.VirusPlacementError)
    self.mocker.replay()
    handler = self.constructHandleFreeHandler(socket, ("jtolds.com",
        32412), server)
    handler.errors = []
    handler.sendErrors = self.addArgsIfCalled
    handler.game_id = "nonce1"
    handler.rows = "nonce2"
    handler.cols = "nonce3"
    handler.combo_length = "nonce4"
    handler.virus_number = "nonce5"
    handler.starting_amount_of_players = 3
    handler.joinGame()
    self.assertListEquals(handler.errors, ["Virus placement error: "])
    self.assertListEquals(self.all_args, [()])
  
  def testInit(self):
    socket = self.mocker.mock()
    server = self.mocker.mock()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.handle = lambda: None
    ViricideConnectionHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(type(handler.request), network.FancySocket)
    self.assertEquals(handler.request.sock, socket)
    self.assertEquals(handler.server, server)

  def testHandle1(self):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    self.mocker.result("nonce")
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.sendErrors = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(handler.errors, ["Incoming data must be a "
        "dictionary"])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)

  def testHandle2(self):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    self.mocker.result({})
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.sendErrors = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(ViricideConnectionHandler.REQUIRED_INIT_FIELDS,
        ['protocol_version', 'game_id', 'starting_amount_of_players', 'rows',
        'cols', 'virus_number', 'combo_length', 'client_id'])
    self.assertListEquals(handler.errors, ['Must have 8 fields.',
        'Missing field: protocol_version.', 'Missing field: game_id.',
        'Missing field: starting_amount_of_players.', 'Missing field: rows.',
        'Missing field: cols.', 'Missing field: virus_number.',
        'Missing field: combo_length.', 'Missing field: client_id.',
        'Missing protocol version!'])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
    
  def testHandle3(self):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    result_dict = {}
    for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
      result_dict[key] = "nonce"
    self.mocker.result(result_dict)
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.sendErrors = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(ViricideConnectionHandler.REQUIRED_INIT_INT_FIELDS,
        ['starting_amount_of_players', 'rows', 'cols', 'virus_number',
        'combo_length'])
    self.assertListEquals(handler.errors, ['Field starting_amount_of_players '
        'must be an integer.', 'Field rows must be an integer.',
        'Field cols must be an integer.', 'Field virus_number must be an '
        'integer.', 'Field combo_length must be an integer.', 'Wrong protocol '
        'version!'])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
    
  def testHandle4(self):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    result_dict = {}
    for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
      result_dict[key] = 1
    result_dict["protocol_version"] = PROTOCOL_VERSION
    self.mocker.result(result_dict)
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.joinGame = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(handler.errors, [])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
    for key in result_dict:
      self.assertEquals(getattr(handler, key), result_dict[key])
    
  def testHandle5(self):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    player = self.mocker.mock(ViricidePlayer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    result_dict = {}
    for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
      result_dict[key] = 1
    result_dict["protocol_version"] = PROTOCOL_VERSION
    self.mocker.result(result_dict)
    player.disconnect()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.joinGame = self.addArgsIfCalled
    handler.player = player
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(handler.errors, [])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
    for key in result_dict:
      self.assertEquals(getattr(handler, key), result_dict[key])
    

for field in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
  def testHandleMissingField(self, field=field):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    result_dict = {}
    for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
      result_dict[key] = 1
    result_dict["protocol_version"] = PROTOCOL_VERSION
    del result_dict[field]
    self.mocker.result(result_dict)
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.sendErrors = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    if field != "protocol_version":
      self.assertListEquals(handler.errors, ["Must have 8 fields.",
          "Missing field: %s." % field])
    else:
      self.assertListEquals(handler.errors, ["Must have 8 fields.",
          "Missing field: protocol_version.", "Missing protocol version!"])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
  setattr(TestViricideConnectionHandler, "testHandle_missing_field_%s" % field,
      testHandleMissingField)

for field in ViricideConnectionHandler.REQUIRED_INIT_INT_FIELDS:
  def testHandleNonIntField(self, field=field):
    socket = self.mocker.mock(network.FancySocket)
    server = self.mocker.mock(ViricideServer)
    socket.sendall("%s\n%s\n" % (SERVER_WELCOME, PROTOCOL_VERSION))
    socket.getObject()
    result_dict = {}
    for key in ViricideConnectionHandler.REQUIRED_INIT_FIELDS:
      result_dict[key] = 1
    result_dict["protocol_version"] = PROTOCOL_VERSION
    result_dict[field] = "nonce"
    self.mocker.result(result_dict)
    socket.close()
    self.mocker.result(None)
    self.mocker.replay()
    handler = OldStyleTestObject()
    handler.__class__ = ViricideConnectionHandler
    handler.sendErrors = self.addArgsIfCalled
    SocketServer.BaseRequestHandler.__init__(handler, socket, ("localhost",
        123), server)
    self.assertEquals(handler.request, socket)
    self.assertListEquals(handler.errors, ["Field %s must be an integer." %
        field])
    self.assertListEquals(self.all_args, [()])
    self.assertEquals(handler.responded, False)
  setattr(TestViricideConnectionHandler, "testHandle_nonint_field_%s" % field,
      testHandleNonIntField)

    
class TestViricideServer(ExtendedTestCase):

  def setUp(self):
    ExtendedTestCase.setUp(self)
    self.saved_flags = {}
    for key in ["port", "max_connections", "limit_total_connections"]:
      self.saved_flags[key] = getattr(FLAGS, key)

  def testInit1(self):
    FLAGS.port = "nonce1"
    FLAGS.max_connections = "nonce5"
    FLAGS.limit_total_connections = "nonce6"
    server = ViricideServer(address=("localhost", 12345),
        max_connections="nonce2", limit_total_connections="nonce3",
        game_store="nonce4")
    self.assertEquals(server.address, ("localhost", 12345))
    self.assertEquals(server.max_connections, "nonce2")
    self.assertEquals(server.limit_total_connections, "nonce3")
    self.assertEquals(server.game_store, "nonce4")
    self.assertEquals(server.daemon_threads, False)
    self.assertEquals(server.allow_reuse_address, True)
    
  def testInit2(self):
    FLAGS.port = "nonce1"
    FLAGS.max_connections = "nonce5"
    FLAGS.limit_total_connections = "nonce6"
    server = ViricideServer(address=("localhost", 12345),
        max_connections="nonce2", limit_total_connections=False,
        game_store="nonce4")
    self.assertEquals(server.address, ("localhost", 12345))
    self.assertEquals(server.max_connections, "nonce2")
    self.assertEquals(server.limit_total_connections, False)
    self.assertEquals(server.game_store, "nonce4")
    self.assertEquals(server.daemon_threads, True)
    self.assertEquals(server.allow_reuse_address, True)
    
  def testInit3(self):
    FLAGS.port = 55535
    FLAGS.max_connections = "nonce5"
    FLAGS.limit_total_connections = "nonce6"
    server = ViricideServer()
    self.assertEquals(server.address, ("", 55535))
    self.assertEquals(server.max_connections, "nonce5")
    self.assertEquals(server.limit_total_connections, "nonce6")
    self.assertEquals(type(server.game_store), ViricideGameStore)
    self.assertEquals(server.daemon_threads, False)
    self.assertEquals(server.allow_reuse_address, True)
    
  def testInit4(self):
    FLAGS.port = 55535
    FLAGS.max_connections = "nonce5"
    FLAGS.limit_total_connections = False
    server = ViricideServer()
    self.assertEquals(server.address, ("", 55535))
    self.assertEquals(server.max_connections, "nonce5")
    self.assertEquals(server.limit_total_connections, False)
    self.assertEquals(type(server.game_store), ViricideGameStore)
    self.assertEquals(server.daemon_threads, True)
    self.assertEquals(server.allow_reuse_address, True)
  
  def testRun1(self):
    server = ViricideServer()
    server.serve_forever = self.raiseErrorIfCalled
    server.handle_request = self.addArgsIfCalled
    server.limit_total_connections = True
    server.max_connections = 12
    server.run()
    self.assertListEquals(self.all_args, [()] * 12)

  def testRun2(self):
    server = ViricideServer()
    server.handle_request = self.raiseErrorIfCalled
    server.serve_forever = self.addArgsIfCalled
    server.limit_total_connections = False
    server.max_connections = 10
    server.run()
    self.assertListEquals(self.all_args, [()])


if __name__ == '__main__':
  sys.argv = FLAGS(sys.argv)
  SetUpLogging()
  unittest.main()
