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


_logger = logging.getLogger('PlayGo.game')


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

class GoBoard( object ) :
    
    def __init__( self, size ):
        
        self.size = size    
        self.board = zeros( [ size, size ], Int )
        self.playNumber = 0;
        
        
    def getPoint( self, x, y ):
        
        assert( x < self.size )
        assert( y < self.size )
        return self.board[x][y]
    
    def setPoint( self, x, y, value ):
        
        if value is 'Empty' :
            n = 0;
        elif value is 'White' :
            n = 1;
        elif value is 'Black' :
            n = 2;
        elif value is 'WhiteKo' :
            n = 3;
        elif value is 'BlackKo' :
            n = 4;
            
        self.board[x][y] = n
        
    def setPointi( self, x, y, value ):
        self.board[x][y] = value
        
    def clear(self):
        for x in range( self.size ):
            for y in range( self.size ):
                self.board[x][y] = 0
        
    def CopyBoard( self ) :
        copy = GoBoard( self.size )
        copy.board = self.board
        copy.playNumber = self.playNumber
        return copy
    
        

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
            dump_grid( aBoard )
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
        _logger.debug('Welcoming newcomer and sending them the game state')
        
        self.tube.get_object(sender, PATH).Welcome(self.boardWidget.myBoard.board,
                                                   self.ordered_bus_names,
                                                   dbus_interface=IFACE)
        
        if (self.player_id == 0 and len(self.ordered_bus_names) == 2):
            _logger.debug("This is my game and an opponent has joined. I go first")
            self.change_turn()

    def insert_cb(self, column, sender=None):
        # Someone placed a disc
        handle = self.tube.bus_name_to_handle[sender]
        _logger.debug('Insert(%d) from %s', column, sender)

        if self.tube.self_handle == handle:
            _logger.debug('Ignoring Insert signal from myself: %d', column)
            return

        try:
            winner = self.boardWidget.insert(column, self.get_active_player())
        except ValueError:
            return

        dump_grid(self.boardWidget.myBoard.board)

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
    
        dump_grid(grid.myBoard.board)
        redraw(grid)
        self.Insert(col)
    
        self.change_turn()
    
        if winner is not None:
            _logger.debug("I win")
            self.info_panel.show(_('You win!'))
        else:
            self.info_panel.show(_('Other player\'s turn'))
