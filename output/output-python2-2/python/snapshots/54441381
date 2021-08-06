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
import socket, sys, network, time, random, threading

DEFAULT_ADDRESS = ("localhost", 54353)
PROTOCOL_VERSION = "0.2.1"

class Error_(Exception): pass
class LogicError(Error_): pass

class GameOverThread(threading.Thread):
  def __init__(self, sock, game_over):
    self.sock = sock
    self.game_over = game_over
    threading.Thread.__init__(self)
  def run(self):
    time.sleep(random.randint(2,10))
    if not self.game_over:
      self.sock.sendObject({
          "message_type": "game_over",
          "virus_number": 0,
        })

def test_client():
  sock = network.FancySocket()
  sock.connect(DEFAULT_ADDRESS)
  try:
    newlines = 0
    print "receiving: ",
    while True:
      byte = sock.recvall(1)
      sys.stdout.write(byte)
      if byte == "\n":
        newlines += 1
        if newlines >= 2:
          break
        else:
          print "receiving: ",
    sock.sendObject({
        "protocol_version": PROTOCOL_VERSION,
        "game_id": "testgameid",
        "starting_amount_of_players": 1,
        "rows": 5,
        "cols": 4,
        "virus_number": 6,
        "combo_length": 4,
        "client_id": "jt test client",
        })
    ret_msg = sock.getObject(short_int = False)
    if not ret_msg["success"]: return
    print "waiting for game start"
    while True:
      obj = sock.getObject()
      if obj["message_type"] == "game_start": break
    print "game started"
    game_over = []
    x = GameOverThread(sock, game_over)
    x.start()
    while True:
      obj = sock.getObject()
      if obj["message_type"] == "game_over":
        game_over.append(True)
        break
      
  finally:
    print "closing socket, waiting for GameOverThread if applicable"
    sock.close()
  
def main(argv):
  test_client()
  
if __name__ == "__main__":
  main(sys.argv)
