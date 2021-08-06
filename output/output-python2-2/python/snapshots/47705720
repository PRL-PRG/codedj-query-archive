import logging

from Numeric import *
from gettext import gettext as _
import gtk

from dbus import Interface
from dbus.service import method, signal
from dbus.gobject_service import ExportedGObject


# XXX: I'm not convinced this is in the right namespace
SERVICE = "org.freedesktop.Telepathy.Tube.Connect"
IFACE = SERVICE
PATH = "/org/freedesktop/Telepathy/Tube/Connect"


_logger = logging.getLogger('PlayGo')


def redraw(grid):
    """Utility function to force a redraw of a Gtk widget."""
    grid.queue_draw()


def dump_grid(seq):
    grid = ''
    for row in seq:
        row_str = ''
        for col in row:
            if col == -1:
                row_str += ' |'
            else:
                row_str += '%d|' % col
        grid = '%s%s\n' % (grid, row_str)
    _logger.debug('Grid state is now:\n%s', grid)
    

##   This abstractBoard is part of Kombilo, a go database program
##   It contains a class implementing an abstract go board 
##   Copyright (C) 2001-3 Ulrich Goertz (u@g0ertz.de)

class abstractBoard:
    """ This class administrates a go board.
        It keeps track of the stones currently on the board in the dictionary self.status,
        and of the moves played so far in self.undostack

        It has methods to clear the board, play a stone, undo a move. """

    def __init__(self, boardSize = 19):
        self.size = boardSize    #TODO: get rid of this
        self.status = {}
        self.undostack = []
        self.boardSize = boardSize
        _logger.setLevel( logging.DEBUG )
        _logger.debug( "init baord size %d", boardSize )

    def neighbors(self,x):
        """ Returns the coordinates of the 4 (resp. 3 resp. 2 at the side / in the corner) intersections
            adjacent to the given one. """
        if   x[0]== 1              :     l0 = [2]
        elif x[0]== self.boardSize :     l0 = [self.boardSize-1]
        else:                            l0 = [x[0]-1, x[0]+1]

        if   x[1]== 1              :     l1 = [2]
        elif x[1]== self.boardSize :     l1 = [self.boardSize-1]
        else:                            l1 = [x[1]-1, x[1]+1]

        l = []
        for i in l0: l.append((i,x[1]))
        for j in l1: l.append((x[0],j))

        return l

    def clear(self):
        """ Clear the board """
        self.status = {}
        self.undostack=[]        

    def play(self,pos,color):
        """ This plays a color=black/white stone at pos, if that is a legal move
            (disregarding ko), and deletes stones captured by that move.
            It returns 1 if the move has been played, 0 if not. """

        if self.status.has_key(pos):                # check if empty
            return 0

        l = self.legal(pos,color)
        if l:                                       # legal move?
            captures = l[1]
            for x in captures: del self.status[x]   # remove captured stones, if any
            self.undostack.append((pos,color,captures))   # remember move + captured stones for easy undo
            return 1
        else: return 0

    def legal(self, pos, color):
        """ Check if a play by color at pos would be a legal move. """
        c = [] # captured stones
        for x in self.neighbors(pos):
            if self.status.has_key(x) and self.status[x]==self.invert(color):
                c = c + self.hasNoLibExcP(x, pos)        

        self.status[pos]=color

        if c:
            captures = []
            for x in c:
                if not x in captures: captures.append(x)
            return (1, captures)

        if self.hasNoLibExcP(pos):
            del self.status[pos]
            return 0
        else: return (1, [])

    def hasNoLibExcP(self, pos, exc = None):
        """ This function checks if the string (=solidly connected) of stones containing
            the stone at pos has a liberty (resp. has a liberty besides that at exc).
            If no liberties are found, a list of all stones in the string is returned.

            The algorithm is a non-recursive  implementation of a simple flood-filling:
            starting from the stone at pos, the main while-loop looks at the intersections
            directly adjacent to the stones found so far, for liberties or other stones that belong
            to the string. Then it looks at the neighbors of those newly found stones, and so
            on, until it finds a liberty, or until it doesn't find any new stones belonging
            to the string, which means that there are no liberties.
            Once a liberty is found, the function returns immediately. """
            
        st = []            # in the end, this list will contain all stones solidly connected to the
                           # one at pos, if this string has no liberties
        newlyFound = [pos] # in the while loop, we will look at the neighbors of stones in newlyFound
        foundNew = 1
        
        while foundNew:
            foundNew = 0
            n = []         # this will contain the stones found in this iteration of the loop
            for x in newlyFound:
                for y in self.neighbors(x):
                    if not self.status.has_key(y) and y != exc:    # found a liberty
                        return []
                    elif self.status.has_key(y) and self.status[y]==self.status[x] \
                         and not y in st and not y in newlyFound: # found another stone of same color
                        n.append(y)
                        foundNew = 1

            st[:0] = newlyFound
            newlyFound = n

        return st     # no liberties found, return list of all stones connected to the original one

    def undo(self, no=1):
        """ Undo the last no moves. """
        for i in range(no):
            if self.undostack:
                pos, color, captures = self.undostack.pop()
                del self.status[pos]
                for p in captures: self.status[p] = self.invert(color)

    def remove(self, pos):
        """ Remove a stone form the board, and store this action in undostack. """
        
        self.undostack.append(((-1,-1), self.invert(self.status[pos]), [pos]))
        del self.status[pos]

    def invert(self,color):
        if color == 'B': return 'W'
        else: return 'B'

    def setPointi( self, x, y, value ):
        
        color = 'W'
        if value == 1 : color = 'B'
        
        _logger.debug( "Setting Point %d, %d to color %s", x, y, color )
        
        return self.play( (x,y), color ) 


      

class GoGame(ExportedGObject):

    def __init__(self, tube, boardwidget, is_initiator, buddies_panel, info_panel,
            owner, get_buddy, activity):
        
        super(GoGame, self).__init__(tube, PATH)
        
        self.tube = tube
        self.boardWidget = boardwidget
        self.is_initiator = is_initiator
        self.entered = False
        self.player_id = None
        self.buddies_panel = buddies_panel
        self.info_panel = info_panel
        self.owner = owner
        self._get_buddy = get_buddy
        self.activity = activity
        
        boardwidget.myGame = self

        # list indexed by player ID
        # 0, 1 are players 0, 1
        # 2+ are the spectator queue, 2 is to play next
        self.ordered_bus_names = []

        self.tube.watch_participants(self.participant_change_cb)
        self.boardWidget.connect('insert-requested', self.insert_requested_cb)

    def participant_change_cb(self, added, removed):
        # Initiator is player 0, other player is player 1.

        _logger.debug('adding participants: %r', added)
        _logger.debug('removing participants: %r', removed)

        for handle, bus_name in added:
            buddy = self._get_buddy(handle)
            _logger.debug('Buddy %r was added', buddy)
            if buddy is not None:
                self.buddies_panel.add_watcher(buddy)

        for handle in removed:
            buddy = self._get_buddy(handle)
            _logger.debug('Buddy %r was removed', buddy)
            if buddy is not None:
                self.buddies_panel.remove_watcher(buddy)
            try:
                self.ordered_bus_names.remove(self.tube.participants[handle])
            except ValueError:
                # already absent
                pass

        if not self.entered:
            self.tube.add_signal_receiver(self.insert_cb, 'Insert', IFACE,
                path=PATH, sender_keyword='sender')
            if self.is_initiator:
                _logger.debug('I am the initiator, so making myself player 0')
                self.add_hello_handler()
                self.ordered_bus_names = [self.tube.get_unique_name()]
                self.player_id = 0
                self.buddies_panel.add_player(self.owner)
            else:
                _logger.debug('Hello, everyone! What did I miss?')
                self.Hello()
        self.entered = True

    @signal(dbus_interface=IFACE, signature='')
    def Hello(self):
        """Request that this player's Welcome method is called to bring it
        up to date with the game state.
        """

    @method(dbus_interface=IFACE, in_signature='aanas', out_signature='')
    def Welcome(self, aBoard, bus_names):
        """To be called on the incoming player by the other players to
        inform them of the game state.

        FIXME: nominate a "referee" (initially the initiator) responsible
        for saying Welcome, elect a new referee when the current referee
        leaves? This could also be used to make the protocol robust against
        cheating/bugs
        """
        if self.player_id is None:
            _logger.debug('Welcomed to the game. Player bus names are %r', bus_names)
            self.boardWidget.myBoard.board = aBoard
            dump_grid( self.boardWidget.myBoard.status )
            self.ordered_bus_names = bus_names
            self.player_id = bus_names.index(self.tube.get_unique_name())
            # OK, now I'm synched with the game, I can welcome others
            self.add_hello_handler()

            buddy = self._get_buddy(self.tube.bus_name_to_handle[bus_names[0]])
            self.buddies_panel.add_player(buddy)
            buddy = self._get_buddy(self.tube.bus_name_to_handle[bus_names[1]])
            self.buddies_panel.add_player(buddy)

            if self.get_active_player() == self.player_id:
                _logger.debug("It's my turn already!")
                self.change_turn()

            redraw( self.boardWidget )
        else:
            _logger.debug("I've already been welcomed, doing nothing")

    def add_hello_handler(self):
        self.tube.add_signal_receiver(self.hello_cb, 'Hello', IFACE,
            path=PATH, sender_keyword='sender')

    @signal(dbus_interface=IFACE, signature='i')
    def Insert(self, column):
        """Signal that the local player has placed a disc."""
        #assert column >= self.boardWidget.myBoard.size
        #assert column < self.boardWidget.myBoard.size

    def hello_cb(self, sender=None):
        
        """Tell the newcomer what's going on."""
        _logger.debug('Newcomer %s has joined', sender)
        self.ordered_bus_names.append(sender)
        
        if len(self.ordered_bus_names) == 2:
            buddy = self._get_buddy(self.tube.bus_name_to_handle[sender])
            self.buddies_panel.add_player(buddy)
        
        _logger.debug('Bus names are now: %r', self.ordered_bus_names)
        _logger.debug('Welcoming newcomer and sending them the game state:')
        dump_grid( self.boardWidget.myBoard.status )
        
        self.tube.get_object(sender, PATH).Welcome(self.boardWidget.myBoard.status,
                                                   self.ordered_bus_names,
                                                   dbus_interface=IFACE)
        
        if (self.player_id == 0 and len(self.ordered_bus_names) == 2):
            _logger.debug("This is my game and an opponent has joined. I go first")
            self.change_turn()

    def insert_cb(self, column, sender=None):
        # Someone placed a stone
        handle = self.tube.bus_name_to_handle[sender]
        _logger.debug('Insert(%d) from %s', column, sender)

        if self.tube.self_handle == handle:
            _logger.debug('Ignoring Insert signal from myself: %d', column)
            return

        try:
            winner = self.boardWidget.insert(column, self.get_active_player())
        except ValueError:
            return

        dump_grid(self.boardWidget.myBoard.status)

        if winner is not None:
            _logger.debug('Player with handle %d wins', handle)
            self.info_panel.show(_('The other player wins!'))
            redraw(self.boardWidget)
            return

        self.change_turn()

    def change_turn(self):
        try:
            bus_name = self.ordered_bus_names[self.get_active_player()]
            buddy = self._get_buddy(self.tube.bus_name_to_handle[bus_name])
            self.buddies_panel.set_is_playing(buddy)
        except:
            _logger.error('argh!', exc_info=1)
            raise

        if self.get_active_player() == self.player_id:
            _logger.debug('It\'s my turn now')
            self.info_panel.show(_('Your turn'))
            self.activity.grab_focus()
        else:
            _logger.debug('It\'s not my turn')
            self.boardWidget.selected_column = None

        redraw(self.boardWidget)

    def get_active_player(self):
        
            return 1

    def key_press_event(self, widget, event):
        
        _logger.debug('Keypress: keyval %s', event.keyval)

        if event.keyval in (gtk.keysyms.Left,):
            _logger.debug('<--')
            if self.boardWidget.selected_column > 0:
                self.boardWidget.selected_column -= 1
                redraw(self.boardWidget)
        elif event.keyval in (gtk.keysyms.Right,):
            _logger.debug('-->')
            if self.boardWidget.selected_column < 6:
                self.boardWidget.selected_column += 1
                redraw(self.boardWidget)
        elif event.keyval in (gtk.keysyms.Down, gtk.keysyms.space):
            _logger.debug('v')
            self.insert_requested_cb(self.boardWidget, self.boardWidget.selected_column)

    def insert_requested_cb(self, grid, col):
 
        _logger.debug('Inserting at %d', col)      
        winner = grid.insert(col, self.player_id)
        if winner == -1:
            return
    
        dump_grid(grid.myBoard.status)
        redraw(grid)
        self.Insert(col)
    
        self.change_turn()
    
        if winner is not None:
            _logger.debug("I win")
            self.info_panel.show(_('You win!'))
        else:
            self.info_panel.show(_('Other player\'s turn'))
