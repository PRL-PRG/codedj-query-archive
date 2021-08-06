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

"""Viricide
This game is an internet-multiplayer-ready version of a famous virus curing
falling pieces puzzle game.
"""
__author__ = "JT Olds"

import driver, communication, pygame, sys
import gflags as flags


FLAGS = flags.FLAGS
flags.DEFINE_boolean("network_game", False, "If True, play across a network "
    "against other people. If False, play against yourself")
flags.DEFINE_string("host", "localhost", "For network games: the server to "
    "connect to.")
flags.DEFINE_integer("port", 54353, "For network games: the TCP port over which"
    "to connect to the host")
flags.DEFINE_string("game_id", "test_game_id", "For network games: the game id "
    "to connect to. You can make one up, but it must match the one your friends"
    " pick.")
flags.DEFINE_integer("starting_amount_of_players", 2, "For network games: the "
    "amount of people to wait for before starting the game.")
flags.DEFINE_integer("rows", 16, "The amount of rows your tube has")
flags.DEFINE_integer("columns", 8, "The amount of columns your tube has")
flags.DEFINE_integer("speed", 300, "The speed of the game. Smaller is faster")
flags.DEFINE_integer("combo_length", 4, "The amount of colors you need to get "
    "in a row to make any go away. This is hardly ever anything but 4.")
flags.DEFINE_integer("virus_number", 40, "The amount of viruses that must be "
    "cleared. In a similar and more famous game featuring a cartoon physician, "
    "the amount of viruses is chosen as (level + 1) * 4")


class Game(object):
  """Game class
    For use with Viricide implementations
    Sets up, configures, and starts the Driver"""

  def __init__(self,rows=16,cols=8,speed=500,combo_length=4,virus_number=8,
      comlink=None):
    self.comlink = comlink or communication.SinglePlayer()
    self.driver = driver.Driver(rows, cols, speed, combo_length, virus_number,
        self.comlink)
    self.game_over = False
    
  def Run(self):
    self.game_over = False
    win = False
    self.comlink.InitGame(self.driver)
    self.driver.InitGame()
    while not self.game_over:
      event = pygame.event.wait()
      self.comlink.HandleEvent(event)
      self.driver.HandleEvent(event)
      self.game_over, win = self.comlink.CheckGameOver()
    self.driver.CleanupGame()
    self.comlink.CleanupGame()
    if win:
      print "Game Over: You Win!"
    else:
      print "Game Over: You did not win. :("


def client():
  if FLAGS.network_game:
    comlink = communication.NetworkComlink(
        FLAGS.host,
        FLAGS.game_id,
        FLAGS.starting_amount_of_players,
        port=FLAGS.port)
  else:
    comlink = communication.SinglePlayer()
  Game(
      rows=FLAGS.rows,
      cols=FLAGS.columns,
      speed=FLAGS.speed,
      combo_length=FLAGS.combo_length,
      virus_number=FLAGS.virus_number,
      comlink=comlink
    ).Run()


def main(argv):
  try:
    argv = FLAGS(argv)
  except flags.FlagsError, e:
    print "%s\nUsage: %s [flags]" % (e, sys.argv[0])
    print "Try --help for more information"
    return 1
  try: client()
  except KeyboardInterrupt, e: pass
  print "Goodbye."
  return 0


if __name__ == "__main__":
  sys.exit(main(sys.argv))
