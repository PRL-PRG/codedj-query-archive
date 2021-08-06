#!/usr/bin/env python

import os, sys
import ConfigParser
from random import randint
import pygame, serial
import pygame.display
import pygame.joystick
from pygame.locals import *

import gameEngine

PLAYSEQUENCE = pygame.USEREVENT+1

# which button to press to start the whole game
STARTBUTTON = K_RETURN

# change assignment of dance mat numbers here (e.g. if played the other way round)
UP, LEFT, DOWN, RIGHT = 1, 2, 3, 4

# assign the keys that handle game play, lookup in the Pygame doc for joystick buttons
# default: keyboard keys for player 1: i,j,k,l player 2 w,a,s,d
PLAYER1_KEYS = {
    K_i: UP,
    K_j: LEFT,
    K_k: DOWN,
    K_l: RIGHT
}
PLAYER2_KEYS = {
    K_w: UP,
    K_a: LEFT,
    K_s: DOWN,
    K_d: RIGHT
}

# defines the strings that represent the possible light rows
EMPTY, LEFTY, CENTER, RIGHTY = "000", "100", "010", "001"

# assembles the light sequences
LIGHTS = [
    [CENTER, EMPTY,  EMPTY],  # UP
    [EMPTY,  LEFTY,  EMPTY],  # LEFT
    [EMPTY,  EMPTY,  CENTER], # DOWN
    [EMPTY,  RIGHTY, EMPTY],  # RIGHT
]

class Joystick(object):
    """
    A generic Joystick class which opens and closes on initialization and 
    quit.
    """
    def __init__(self):
        pygame.joystick.init()
        pygame.joystick.get_init()
        self.count = pygame.joystick.get_count()
        if self.count > 0:
            self.joystick = pygame.joystick.Joystick(0)
            self.joystick.init()
            print '%d joystick(s) connected: %s' % (self.count, joystick.get_name())
            self.num_axes = joystick.get_numaxes()
            self.num_buttons = joystick.get_numbuttons()
            self.num_hats = joystick.get_numhats()
            print 'Joystick has %d axes, %d buttons and %d hats.' % (num_axes, num_buttons, num_hats)
            pygame.event.pump()
            self.old_axis = []
            for num in range(self.num_axes):
                self.old_axis.append(0.0)

    def both_axis_active(self):
        activated_axis = []
        for num in range(self.num_axes):
            axis = "%.4f" % self.joystick.get_axis(num)
            if self.axis != self.old_axis[num]:
                activated_axis.append(num)
            self.old_axis[num] = self.axis
        if len(activated_axis) == range(self.num_axes):
            return True
        return False

    def close(self):
        pygame.joystick.quit()

class Player(object):
    """
    An abstract player object, which can make a guess about a light sequence,
    has three lives and looses the game if dead.
    """
    def __init__(self, name, keys):
        self.name = name
        self.lives = 3
        self.dead = False
        self.guess = []
        self.keys = keys
    
    def die(self):
        """
        Substract 1 from the live count until dead.
        """
        self.lives -= 1
        if self.lives <= 0:
            self.dead = True
        print "Player lost live: %s (%s more live(s))" % (self.name, self.lives)

    def add_guess(self, guess, level):
        """
        Adds a guess to the players portfolio, returning true if he is not
        dead and not totally dumb
        """
        if guess >= 1 and guess <= 4 and len(self.guess) < level and not self.dead:
            print "Player %s guess: %s" % (self.name, guess)
            self.guess.append(guess)
            return True
        else:
            return False

class Sequence(object):
    """
    An abstract senso light sequence to be guessed/danced by the players.
    """
    def __init__(self, players):
        self.sequence = []
        self.level = 3
        self.players = players
        self.generate()

    def clear_guesses(self):
        """ Clear all guesses by the players """
        for player in self.players:
            player.guess = []

    def generate(self, restart=False):
        """ (Re-)generate the gaming sequence """
        if restart:
            self.level = 3
        self.sequence = []
        self.clear_guesses()
        for i in xrange(self.level):
            self.sequence.append(randint(1, 4))
        print "New sequence:", self.sequence

    def next_level(self):
        """ Jump to the next level of fun! """
        self.clear_guesses()
        self.sequence.append(randint(1, 4))
        self.level += 1
        print "New sequence:", self.sequence

    def completed_by_players(self):
        """ Returns true if all players have finished their sequence """
        finished_players = 0
        for player in self.players:
            if len(player.guess) == len(self.sequence):
                finished_players += 1
        return finished_players == len(self.players)

    def get_sequence(self):
        return self.sequence
    
class Game(gameEngine.Scene):
    """
    An abstract game class, inherited from the gameEngine.Scene class, 
    which basically means that it creates a pygame instance and runs
    doEvents() and update() of this class on every loop cycle with a
    framerate of 30.
    """
    def __init__(self, title, players, config):
        gameEngine.Scene.__init__(self)
        self.screen = pygame.display.set_mode((320, 240))
        self.setCaption(title)
        self.players = players
        self.config = config

        self.joystick = Joystick()
        self.serial = None
        try:
            self.open_serial()
        except:
            print "Serial connection could not be established! Aborting.."
            sys.exit(0)

        self.sequence = Sequence(self.players)
        self.dead_players = 0

        self.ready_for_input = False
        self.ready_for_playing = False
        print "Press start button to create a new game."
        self.start()

    def restart(self):
        """
        Restarts the current game.
        """
        for player in self.players:
            player.dead = False
            player.lives = 3
        self.dead_players = 0
        self.sequence.generate(restart=True)

    def open_serial(self):
        """
        Opens a serial connection.
        """
        if self.config.has_option("simonblinks", "serial"):
            serial_device = self.config.get("simonblinks", "serial")
            self.serial = serial.Serial(serial_device, baudrate=9600,
                parity=serial.PARITY_NONE, bytesize=serial.EIGHTBITS,
                stopbits=serial.STOPBITS_ONE)
        else:
            raise SystemError("Please change the 'serial' setting in simonblinks.cfg")

    def send_serial(self, lights):
        """
        Tries to send a light sequence over the flux compensator to the 
        logic board connected to the serial port. This is the critical part
        when programming the logic boards and may need some treatment on
        customization.
        """
        print lights
        try:
            for row in lights:
                row_int = int(row, 2)
                row_hex = chr(row_int)
                self.serial.write(row_hex)
        except:
            pass

    def live_lights(self):
        """
        Returns a list with light sequences representing the lives of all
        players.
        """
        lights = []
        for x in range(3):
          row = ""
          for player in self.players:
            if player.dead:
                row += "0"
            else:
                row += "1"
          lights.append(row)
        lights.reverse()
        return lights

    def light(self, lights, player):
        """
        Assembles the light sequence and prepares it to be sent to the logic
        board connected to the serial device.
        """
        live_lights = self.live_lights()
        light_list = []
        x = 0
        for light in lights:
            if player == 0:
                light_list.append(light+live_lights[x]+light)
            elif player == 1:
                light_list.append(light+live_lights[x]+EMPTY)
            elif player == 2:
                light_list.append(EMPTY+live_lights[x]+light)
            x += 1
        self.send_serial(light_list)

    def play_sequence(self):
        """
        Plays every frame of a sequence. Add delay here if needed.
        """
        for frame in self.sequence.get_sequence():
            self.light(LIGHTS[frame-1], 0)

    def doEvents(self, event):
        """
        The mighty all-running loop inherited from gameEnging.Scene. Please
        have a look in the docstring of gameEnging.Scene.
        """
        if not self.ready_for_playing:
            # Just in case we actually see this game in reality
            # if event.type == JOYAXISMOTION:
            #     if self.joystick.both_axis_active():
            if event.type == pygame.KEYDOWN:
                if event.key == STARTBUTTON:
                    self.ready_for_playing = True
                    print "Game started. Now guess!"
                    print "Player1 keys: W, A, S, D"
                    print "Player2 keys: I, J, K, L"
                    pygame.event.post(pygame.event.Event(PLAYSEQUENCE))
        if event.type == pygame.KEYDOWN:
            key = event.key
            if key in (pygame.K_ESCAPE, pygame.K_q):
                self.keepGoing = False
            if key == K_c:
                # manual override a.k.a. jump to next level with key "c"
                self.sequence.next_level()
            if self.ready_for_input:
                for player in self.players:
                    if key in player.keys:
                        guess = player.keys[key]
                        if player.add_guess(guess, self.sequence.level):
                            self.light(LIGHTS[guess-1], player.name)
                if self.sequence.completed_by_players():
                    for player in self.players:
                        print "Player %s guessed: %s Sequence is: %s" % (player.name, player.guess, self.sequence.get_sequence())
                        if player.guess != self.sequence.get_sequence():
                            player.die()
                    self.sequence.next_level()
                    self.ready_for_input = True
                    self.ready_for_playing = False
                    pygame.time.delay(500)
        elif event.type == JOYBUTTONDOWN:
            # dummy things for the joystick
            for num in range(num_buttons):
                button = joystick.get_button(num)
                if button:
                    print "Button pressed: " + str(num)
        elif event.type == PLAYSEQUENCE:
            if self.ready_for_playing:
                self.play_sequence()
                self.ready_for_input = True

    def update(self):
        """
        Runs after the event loop, on each cycle.
        """
        for player in self.players:
            if player.dead:
                print "Game over player: %s" % player.name
                self.dead_players += 1
        if self.dead_players == len(self.players):
            print "Restarting game.. done."
            self.restart()

    def quit(self):
        """
        Closes conections to joystick and serial ports.
        """
        self.joystick.close()
        if not game.serial is None:
            game.serial.close()

def main():
    """
    Main function to start the program
    """
    config = ConfigParser.ConfigParser()    
    config.read(['simonblinks.cfg', os.path.expanduser('~/.simonblinks.cfg')])
    
    players = Player(1, PLAYER1_KEYS), Player(2, PLAYER2_KEYS)
    game = Game("Simon blinks", players, config)
    game.quit()
    
if __name__=="__main__":
    main()
