import logging
from gettext import gettext as _

import dbus
import gtk
import telepathy
import telepathy.client

from sugar.activity.activity import Activity, ActivityToolbox
from sugar.presence import presenceservice
import sugar.logger

import boardwidget
from game import SERVICE, abstractBoard, GoGame

from sugar.presence.tubeconn import TubeConnection
from buddiespanel import BuddiesPanel
from infopanel import InfoPanel


logger = logging.getLogger('PlayGO')


class PlayGo(Activity):
    """
    Enter the PlayGo activity.
        1. intialize our parent object
        2. create an empty abstract board
        3. create the graphic objects, boardWidget, buddyPanel, InfoPanel
        4. group them in layout containers
        5. create an ActivityToolBox
        6. open a channel to the presence server
        7.1 if creating the game, make me the only player and go into local play mode
        7.2 if connecting to a neighborhood game, call the activity connection methods
    """
    def __init__(self, handle):
        Activity.__init__(self, handle)
        self._name = handle
        
        self.set_title('PlayGo')

        logger.debug('Starting Playgo activity...')
        
        toolbox = ActivityToolbox(self)
        self.set_toolbox(toolbox)

        board = abstractBoard( 19 )
        self.boardWidget = boardwidget.BoardWidget( board, self )
        self.buddies_panel = BuddiesPanel()
        self.info_panel = InfoPanel()

        #Prepare the main box
        self._main_view = gtk.HBox()
        
        #Prepare the left box
        self.left_view = gtk.VBox()
        self.left_view.pack_start(self.boardWidget)
        self.info_panel = InfoPanel()        
        self.left_view.pack_start(self.info_panel,  False)

        #Pack the left view
        self._main_view.pack_start(self.left_view)
        
        #Prepare the right view
        self.right_view = gtk.VBox()
        self.right_view.pack_start(gtk.Label('Aca va el coso'),  False)
        
        #Pack the right view
        self._main_view.pack_end(self.right_view,  False)
        
        #Set canvas and show all
        self.set_canvas(self._main_view)
        self.show_all()

        self.pservice = presenceservice.get_instance()
        owner = self.pservice.get_owner()
        self.owner = owner
        self.shared = False

        # This displays the buddies_panel even if we fail to connect:
        self.buddies_panel.add_watcher(owner)
        self.info_panel.show(_('Place a black stone to play locally. You may share or invite to play remotely'))

        self.initiating = None

        self.game = None

        self.connect('shared', self._shared_cb)

        if self._shared_activity:
            # we are joining the activity
            self.buddies_panel.add_watcher(owner)
            self.connect('joined', self._joined_cb)
            self._shared_activity.connect('buddy-joined', self._buddy_joined_cb)
            self._shared_activity.connect('buddy-left', self._buddy_left_cb)
            if self.get_shared():
                # oh, OK, we've already joined
                self._joined_cb()
        else:
            # we are creating the activity
            self.buddies_panel.remove_watcher(owner)
            self.buddies_panel.add_player(owner)
            #self.buddies_panel.set_is_playing(owner)
            #self.buddies_panel.set_count(owner, 69)

        self.connect('key-press-event', self.key_press_cb)

    def _get_buddy(self, cs_handle):
        """Get a Buddy from a channel specific handle."""
        # FIXME: After Update.1, design a better API for sugar.presence
        # to track handles of buddies, and use that instead
        logger.debug('Trying to find owner of handle %u...', cs_handle)
        group = self.text_chan[telepathy.CHANNEL_INTERFACE_GROUP]
        my_csh = group.GetSelfHandle()
        logger.debug('My handle in that group is %u', my_csh)
        if my_csh == cs_handle:
            handle = self.conn.GetSelfHandle()
            logger.debug('CS handle %u belongs to me, %u', cs_handle, handle)
        elif group.GetGroupFlags() & telepathy.CHANNEL_GROUP_FLAG_CHANNEL_SPECIFIC_HANDLES:
            handle = group.GetHandleOwners([cs_handle])[0]
            logger.debug('CS handle %u belongs to %u', cs_handle, handle)
        else:
            handle = cs_handle
            logger.debug('non-CS handle %u belongs to itself', handle)

            # XXX: deal with failure to get the handle owner
            assert handle != 0

        return self.pservice.get_buddy_by_telepathy_handle(
            self.conn.service_name, self.conn.object_path, handle)

    def key_press_cb(self, widget, event):
        logger.debug('Keypress: %r, %r', widget, event)

        if self.game is not None:
            self.game.key_press_event(widget, event)

    def _shared_cb(self, activity):
        logger.debug('My PlayGo activity was shared')
        self.shared = True
        self.initiating = True
        self._setup()

        for buddy in self._shared_activity.get_joined_buddies():
            self.buddies_panel.add_watcher(buddy)

        self._shared_activity.connect('buddy-joined', self._buddy_joined_cb)
        self._shared_activity.connect('buddy-left', self._buddy_left_cb)

        logger.debug('This is my activity: making a tube...')
        id = self.tubes_chan[telepathy.CHANNEL_TYPE_TUBES].OfferDBusTube(
            SERVICE, {})
        self.info_panel.show(_('Waiting for another player to join.'))

    def _setup(self):
        if self._shared_activity is None:
            logger.error('Failed to share or join activity')
            return

        self.conn = self._shared_activity.telepathy_conn
        self.tubes_chan = self._shared_activity.telepathy_tubes_chan
        self.text_chan = self._shared_activity.telepathy_text_chan

        self.tubes_chan[telepathy.CHANNEL_TYPE_TUBES].connect_to_signal(
                'NewTube', self._new_tube_cb)

    def _list_tubes_reply_cb(self, tubes):
        for tube_info in tubes:
            self._new_tube_cb(*tube_info)

    def _list_tubes_error_cb(self, e):
        logger.error('ListTubes() failed: %s', e)

    def _joined_cb(self, activity):
        if self.game is not None:
            return

        if not self._shared_activity:
            return

        for buddy in self._shared_activity.get_joined_buddies():
            self.buddies_panel.add_watcher(buddy)

        logger.debug('Joined an existing Connect game')
        self.info_panel.show(_('Joined a game. Waiting for my turn...'))
        self.initiating = False
        self._setup()

        logger.debug('This is not my activity: waiting for a tube...')
        self.tubes_chan[telepathy.CHANNEL_TYPE_TUBES].ListTubes(
            reply_handler=self._list_tubes_reply_cb,
            error_handler=self._list_tubes_error_cb)

    def _new_tube_cb(self, id, initiator, type, service, params, state):
        logger.debug('New tube: ID=%d initator=%d type=%d service=%s '
                     'params=%r state=%d', id, initiator, type, service,
                     params, state)

        if (self.game is None and type == telepathy.TUBE_TYPE_DBUS and
            service == SERVICE):
            if state == telepathy.TUBE_STATE_LOCAL_PENDING:
                self.tubes_chan[telepathy.CHANNEL_TYPE_TUBES].AcceptDBusTube(id)

            tube_conn = TubeConnection(self.conn,
                self.tubes_chan[telepathy.CHANNEL_TYPE_TUBES],
                id, group_iface=self.text_chan[telepathy.CHANNEL_INTERFACE_GROUP])
            
            self.game = GoGame(tube_conn, self.boardWidget, self.initiating,
                self.buddies_panel, self.info_panel, self.owner,
                self._get_buddy, self)

    def _buddy_joined_cb (self, activity, buddy):
        logger.debug("buddy joined")
        self.buddies_panel.add_watcher(buddy)

    def _buddy_left_cb (self,  activity, buddy):
        logger.debug("buddy left")
        self.buddies_panel.remove_watcher(buddy)
