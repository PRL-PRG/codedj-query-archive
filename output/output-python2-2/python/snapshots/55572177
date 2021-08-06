
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from gettext import gettext as _
from sugar.graphics import style
from sugar.graphics.palette import Palette, Invoker, _palette_observer
import gobject

import Jam.Block as Block
from Util.NoteDB import PARAMETER
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client
from Jam.Parasite import LoopParasite

from Generation.Generator import generator1, GenerationParameters

class SELECTNOTES:
    ALL = -1
    NONE = 0
    ADD = 1
    REMOVE = 2
    FLIP = 3
    EXCLUSIVE = 4

class NoneInvoker( Invoker ):

    def __init__( self ):
        Invoker.__init__( self )
        self._position_hint = Invoker.AT_CURSOR

    def get_rect( self ):
        return gtk.gdk.Rectangle( 0, 0, 0, 0 )

    def get_toplevel( self ):
        return None

class Popup( Palette ):

    def __init__( self, label, owner ):
        Palette.__init__( self, label )

        self.owner = owner

        self.block = None

        self.props.invoker = NoneInvoker()
        self.set_group_id( "TamTamPopup" )

        self._set_state( Palette.SECONDARY ) # skip to fully exposed
 
        self.connect( "key-press-event", self.on_key_press )
        self.connect( "key-release-event", self.on_key_release )

        self.connect( "focus_out_event", self.closePopup )

    def destroy( self ):
        pass

    def _leave_notify_event_cb( self, widget, event ):
        return # don't popdown()

    def _show( self ):
        Palette._show( self )

        if self._palette_popup_sid != None:
            _palette_observer.disconnect( self._palette_popup_sid ) # don't hide when other palettes pop
            self._palette_popup_sid = None

    def popup( self, immediate = False ):
        self.owner.activity.handler_block(self.owner.activity.focusOutHandler)
        self.owner.activity.handler_block(self.owner.activity.focusInHandler)

        Palette.popup( self, immediate )

    def popdown( self, immediate = False ):
        self.block = None

        Palette.popdown( self, immediate )

        self.owner.activity.handler_unblock(self.owner.activity.focusOutHandler)
        self.owner.activity.handler_unblock(self.owner.activity.focusInHandler)

    def updatePosition( self ):
        self.props.invoker._cursor_x = -1
        self.props.invoker._cursor_y = -1
        self._update_position()

    def closePopup( self, widget, event ):
        self.popdown( True )

    def on_key_press( self, widget, event ):
        self.owner.onKeyPress( widget, event )

    def on_key_release( self, widget, event ):
        self.owner.onKeyRelease( widget, event )

    def setBlock( self, block ):
        if self.is_up():
            self.updatePosition()
        else:
            self.popup( True )


class Instrument( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

        self.settingBlock = False 

        self.GUI = {}

        self.GUI["mainBox"] = gtk.VBox()
        self.set_content( self.GUI["mainBox"] )

        #-- Volume --------------------------------------------
        self.GUI["volumeBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["volumeBox"], padding = style.DEFAULT_PADDING )
        self.GUI["volumeLabel"] = gtk.Label( _("Volume:") )
        self.GUI["volumeLabel"].set_size_request( 100, -1 )
        self.GUI["volumeLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["volumeBox"].pack_start( self.GUI["volumeLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["volumeAdjustment"] = gtk.Adjustment( 0.5, 0.0, 1.0, 0.1, 0.1, 0 )
        self.GUI["volumeAdjustment"].connect( 'value-changed', self.handleVolume )
        self.GUI["volumeSlider"] = gtk.HScale( adjustment = self.GUI["volumeAdjustment"] )
        self.GUI["volumeSlider"].set_size_request( 250, -1 )
        self.GUI["volumeSlider"].set_draw_value( False )
        self.GUI["volumeBox"].pack_start( self.GUI["volumeSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["volumeImage"] = gtk.Image()
        self.GUI["volumeBox"].pack_start( self.GUI["volumeImage"], False, padding = style.DEFAULT_PADDING )

        #-- Pan -----------------------------------------------
        self.GUI["panBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["panBox"], padding = style.DEFAULT_PADDING )
        self.GUI["panLabel"] = gtk.Label( _("Pan:") )
        self.GUI["panLabel"].set_size_request( 100, -1 )
        self.GUI["panLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["panBox"].pack_start( self.GUI["panLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["panAdjustment"] = gtk.Adjustment( 0.5, 0, 1.0, 0.1, 0.1, 0 )
        self.GUI["panAdjustment"].connect( 'value-changed', self.handlePan )
        self.GUI["panSlider"] = gtk.HScale( adjustment = self.GUI["panAdjustment"] )
        self.GUI["panSlider"].set_size_request( 250, -1 )
        self.GUI["panSlider"].set_draw_value( False )
        self.GUI["panBox"].pack_start( self.GUI["panSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["panImage"] = gtk.Image()
        self.GUI["panBox"].pack_start( self.GUI["panImage"], False, padding = style.DEFAULT_PADDING )

        #-- Reverb --------------------------------------------
        self.GUI["reverbBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["reverbBox"], padding = style.DEFAULT_PADDING )
        self.GUI["reverbLabel"] = gtk.Label( _("Reverb:") )
        self.GUI["reverbLabel"].set_size_request( 100, -1 )
        self.GUI["reverbLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["reverbBox"].pack_start( self.GUI["reverbLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["reverbAdjustment"] = gtk.Adjustment( 0.5, 0, 1.0, 0.1, 0.1, 0 )
        self.GUI["reverbAdjustment"].connect( 'value-changed', self.handleReverb )
        self.GUI["reverbSlider"] = gtk.HScale( adjustment = self.GUI["reverbAdjustment"] )
        self.GUI["reverbSlider"].set_size_request( 250, -1 )
        self.GUI["reverbSlider"].set_draw_value( False )
        self.GUI["reverbBox"].pack_start( self.GUI["reverbSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["reverbImage"] = gtk.Image()
        self.GUI["reverbBox"].pack_start( self.GUI["reverbImage"], False, padding = style.DEFAULT_PADDING )

        if False: # TEMP quote out
            self.GUI["separator"] = gtk.HSeparator()
            self.GUI["mainBox"].pack_start( self.GUI["separator"], padding = style.DEFAULT_PADDING )

            #-- Export --------------------------------------------
            self.GUI["exportBox"] = gtk.HBox()
            self.GUI["mainBox"].pack_start( self.GUI["exportBox"], padding = style.DEFAULT_PADDING )
            self.GUI["exportEntry"] = gtk.Entry()
            self.GUI["exportEntry"].modify_fg( gtk.STATE_NORMAL, self.owner.colors["black"] )
            self.GUI["exportEntry"].modify_fg( gtk.STATE_ACTIVE, self.owner.colors["black"] )
            self.GUI["exportBox"].pack_start( self.GUI["exportEntry"], padding = style.DEFAULT_PADDING )
            self.GUI["exportButton"] = gtk.Button( "Export" )
            self.GUI["exportBox"].pack_start( self.GUI["exportButton"], False, padding = style.DEFAULT_PADDING )

        self.GUI["mainBox"].show_all()

    def setBlock( self, block ):
        self.settingBlock = True

        self.block = block
        self.GUI["volumeAdjustment"].set_value( block.getData( "volume" ) )
        self.GUI["panAdjustment"].set_value( block.getData( "pan" ) )
        self.GUI["reverbAdjustment"].set_value( block.getData( "reverb" ) )
        #self.GUI["exportEntry"].set_text( block.getData( "name" ) )

        self.settingBlock = False

        Popup.setBlock( self, block )

    def handleVolume( self, widget ):
        if not self.settingBlock:
            self.block.setData( "volume", widget.get_value() )

    def handlePan( self, widget ):
        if not self.settingBlock:
            self.block.setData( "pan", widget.get_value() )

    def handleReverb( self, widget ):
        if not self.settingBlock:
            self.block.setData( "reverb", widget.get_value() )


class Drum( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

        self.settingBlock = False

        self.GUI = {}

        self.GUI["mainBox"] = gtk.VBox()
        self.set_content( self.GUI["mainBox"] )

        #-- Volume --------------------------------------------
        self.GUI["volumeBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["volumeBox"], padding = style.DEFAULT_PADDING )
        self.GUI["volumeLabel"] = gtk.Label( _("Volume:") )
        self.GUI["volumeLabel"].set_size_request( 130, -1 )
        self.GUI["volumeLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["volumeBox"].pack_start( self.GUI["volumeLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["volumeAdjustment"] = gtk.Adjustment( 0.5, 0.0, 1.0, 0.1, 0.1, 0 )
        self.GUI["volumeAdjustment"].connect( 'value-changed', self.handleVolume )
        self.GUI["volumeSlider"] = gtk.HScale( adjustment = self.GUI["volumeAdjustment"] )
        self.GUI["volumeSlider"].set_size_request( 250, -1 )
        self.GUI["volumeSlider"].set_draw_value( False )
        self.GUI["volumeBox"].pack_start( self.GUI["volumeSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["volumeImage"] = gtk.Image()
        self.GUI["volumeBox"].pack_start( self.GUI["volumeImage"], False, padding = style.DEFAULT_PADDING )
       
        #-- Reverb --------------------------------------------
        self.GUI["reverbBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["reverbBox"], padding = style.DEFAULT_PADDING )
        self.GUI["reverbLabel"] = gtk.Label( _("Reverb:") )
        self.GUI["reverbLabel"].set_size_request( 130, -1 )
        self.GUI["reverbLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["reverbBox"].pack_start( self.GUI["reverbLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["reverbAdjustment"] = gtk.Adjustment( 0.5, 0, 1.0, 0.1, 0.1, 0 )
        self.GUI["reverbAdjustment"].connect( 'value-changed', self.handleReverb )
        self.GUI["reverbSlider"] = gtk.HScale( adjustment = self.GUI["reverbAdjustment"] )
        self.GUI["reverbSlider"].set_size_request( 250, -1 )
        self.GUI["reverbSlider"].set_draw_value( False )
        self.GUI["reverbBox"].pack_start( self.GUI["reverbSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["reverbImage"] = gtk.Image()
        self.GUI["reverbBox"].pack_start( self.GUI["reverbImage"], False, padding = style.DEFAULT_PADDING )
       
        self.GUI["generationSeparator"] = gtk.HSeparator()
        self.GUI["mainBox"].pack_start( self.GUI["generationSeparator"], padding = style.DEFAULT_PADDING )

        #-- Beats ---------------------------------------------
        self.GUI["beatsBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["beatsBox"], padding = style.DEFAULT_PADDING )
        self.GUI["beatsLabel"] = gtk.Label( _("Beats:") )
        self.GUI["beatsLabel"].set_size_request( 130, -1 )
        self.GUI["beatsLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["beatsBox"].pack_start( self.GUI["beatsLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["beatsAdjustment"] = gtk.Adjustment( 4, 2, Config.MAXIMUM_BEATS, 1, 1, 0 )
        self.GUI["beatsAdjustment"].connect( 'value-changed', self.handleBeats )
        self.GUI["beatsSlider"] = gtk.HScale( adjustment = self.GUI["beatsAdjustment"] )
        self.GUI["beatsSlider"].set_size_request( 250, -1 )
        self.GUI["beatsSlider"].set_draw_value( False )
        self.GUI["beatsBox"].pack_start( self.GUI["beatsSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["beatsImage"] = gtk.Image()
        self.GUI["beatsBox"].pack_start( self.GUI["beatsImage"], False, padding = style.DEFAULT_PADDING )
        
        #-- Regularity ----------------------------------------
        self.GUI["regularityBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["regularityBox"], padding = style.DEFAULT_PADDING )
        self.GUI["regularityLabel"] = gtk.Label( _("Regularity:") )
        self.GUI["regularityLabel"].set_size_request( 130, -1 )
        self.GUI["regularityLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["regularityBox"].pack_start( self.GUI["regularityLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["regularityAdjustment"] = gtk.Adjustment( 0.5, 0.0, 1.0, 0.1, 0.1, 0 )
        self.GUI["regularityAdjustment"].connect( 'value-changed', self.handleRegularity )
        self.GUI["regularitySlider"] = gtk.HScale( adjustment = self.GUI["regularityAdjustment"] )
        self.GUI["regularitySlider"].set_size_request( 250, -1 )
        self.GUI["regularitySlider"].set_draw_value( False )
        self.GUI["regularityBox"].pack_start( self.GUI["regularitySlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["regularityImage"] = gtk.Image()
        self.GUI["regularityBox"].pack_start( self.GUI["regularityImage"], False, padding = style.DEFAULT_PADDING )

        #-- Generate ------------------------------------------
        self.GUI["generateBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["generateBox"], padding = style.DEFAULT_PADDING )
        self.GUI["regenerateButton"] = gtk.Button( "Regenerate" )
        self.GUI["regenerateButton"].connect( "clicked", self.handleRegenerate )
        self.GUI["generateBox"].pack_start( self.GUI["regenerateButton"], True, False, padding = style.DEFAULT_PADDING )
        self.GUI["clearButton"] = gtk.Button( "Clear" )
        self.GUI["clearButton"].connect( "clicked", self.handleClear )
        self.GUI["generateBox"].pack_start( self.GUI["clearButton"], True, False, padding = style.DEFAULT_PADDING )
 
        self.GUI["mainBox"].show_all()

    def setBlock( self, block ):
        self.settingBlock = True

        self.block = block
        self.GUI["volumeAdjustment"].set_value( block.getData( "volume" ) )
        self.GUI["reverbAdjustment"].set_value( block.getData( "reverb" ) )
        self.GUI["beatsAdjustment"].set_value( block.getData( "beats" ) )
        self.GUI["regularityAdjustment"].set_value( block.getData( "regularity" ) )

        self.settingBlock = False

        Popup.setBlock( self, block )

    def handleVolume( self, widget ):
        if not self.settingBlock:
            self.block.setData( "volume", widget.get_value() )

    def handleReverb( self, widget ):
        if not self.settingBlock:
            self.block.setData( "reverb", widget.get_value() )

    def handleBeats( self, widget ):
        # snap to 0 decimal places
        val = widget.get_value()
        if round( val ) != val:
            widget.set_value( round( val ) )
            return

        if not self.settingBlock:
            self.block.setData( "beats", int(round( widget.get_value() )) )

    def handleRegularity( self, widget ):
        if not self.settingBlock:
            self.block.setData( "regularity", widget.get_value() )

    def handleRegenerate( self, widget ):
        self.block.regenerate()

    def handleClear( self, widget ):
        self.block.clear()

class Loop( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

        self.settingBlock = False

        self.gc = self.owner.gc
        self.colors = self.owner.colors
        self.sampleNoteMask = self.owner.sampleNoteMask

        self.noteDB = self.owner.noteDB
        self.csnd = new_csound_client()

        self.GUI = {}

        self.GUI["mainBox"] = gtk.VBox()
        self.set_content( self.GUI["mainBox"] )

        #-- Beats ---------------------------------------------
        self.GUI["beatsBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["beatsBox"], padding = style.DEFAULT_PADDING )
        self.GUI["beatsLabel"] = gtk.Label( _("Beats:") )
        self.GUI["beatsLabel"].set_size_request( 130, -1 )
        self.GUI["beatsLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["beatsBox"].pack_start( self.GUI["beatsLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["beatsAdjustment"] = gtk.Adjustment( 4, 2, Config.MAXIMUM_BEATS, 1, 1, 0 )
        self.GUI["beatsAdjustment"].connect( 'value-changed', self.handleBeats )
        self.GUI["beatsSlider"] = gtk.HScale( adjustment = self.GUI["beatsAdjustment"] )
        self.GUI["beatsSlider"].set_size_request( 250, -1 )
        self.GUI["beatsSlider"].set_draw_value( False )
        self.GUI["beatsBox"].pack_start( self.GUI["beatsSlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["beatsImage"] = gtk.Image()
        self.GUI["beatsBox"].pack_start( self.GUI["beatsImage"], False, padding = style.DEFAULT_PADDING )
        
        #-- Regularity ----------------------------------------
        self.GUI["regularityBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["regularityBox"], padding = style.DEFAULT_PADDING )
        self.GUI["regularityLabel"] = gtk.Label( _("Regularity:") )
        self.GUI["regularityLabel"].set_size_request( 130, -1 )
        self.GUI["regularityLabel"].set_alignment( 0.0, 0.5 )
        self.GUI["regularityBox"].pack_start( self.GUI["regularityLabel"], False, padding = style.DEFAULT_PADDING )
        self.GUI["regularityAdjustment"] = gtk.Adjustment( 0.5, 0.0, 1.0, 0.1, 0.1, 0 )
        self.GUI["regularityAdjustment"].connect( 'value-changed', self.handleRegularity )
        self.GUI["regularitySlider"] = gtk.HScale( adjustment = self.GUI["regularityAdjustment"] )
        self.GUI["regularitySlider"].set_size_request( 250, -1 )
        self.GUI["regularitySlider"].set_draw_value( False )
        self.GUI["regularityBox"].pack_start( self.GUI["regularitySlider"], False, padding = style.DEFAULT_PADDING )
        self.GUI["regularityImage"] = gtk.Image()
        self.GUI["regularityBox"].pack_start( self.GUI["regularityImage"], False, padding = style.DEFAULT_PADDING )

        #-- Generate ------------------------------------------
        self.GUI["generateBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["generateBox"], padding = style.DEFAULT_PADDING )
        self.GUI["regenerateButton"] = gtk.Button( "Regenerate" )
        self.GUI["regenerateButton"].connect( "clicked", self.handleRegenerate )
        self.GUI["generateBox"].pack_start( self.GUI["regenerateButton"], True, False, padding = style.DEFAULT_PADDING )
        self.GUI["clearButton"] = gtk.Button( "Clear" )
        self.GUI["clearButton"].connect( "clicked", self.handleClear )
        self.GUI["generateBox"].pack_start( self.GUI["clearButton"], True, False, padding = style.DEFAULT_PADDING )
        self.GUI["recordButton"] = gtk.ToggleButton( "Record" )
        self.GUI["recordButton"].connect( "toggled", self.handleRecord )
        self.GUI["generateBox"].pack_start( self.GUI["recordButton"], True, False, padding = style.DEFAULT_PADDING )
  
        #-- Preview -------------------------------------------
        self.GUI["previewBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["previewBox"], padding = style.DEFAULT_PADDING )
        self.GUI["previewEventBox"] = gtk.EventBox()
        self.GUI["previewEventBox"].add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)
        self.GUI["previewEventBox"].connect( "button-press-event", self.handlePreviewPress )
        self.GUI["previewEventBox"].connect( "button-release-event", self.handlePreviewRelease )
        self.GUI["previewEventBox"].connect( "motion-notify-event", self.handlePreviewMotion )
        self.GUI["previewEventBox"].connect( "leave-notify-event", self.handlePreviewLeave )
        self.GUI["previewBox"].pack_start( self.GUI["previewEventBox"], True, padding = style.DEFAULT_PADDING )
        self.previewDA = self.GUI["previewDA"] = gtk.DrawingArea()
        self.GUI["previewDA"].connect( "size-allocate", self.handlePreviewAlloc )
        self.GUI["previewDA"].connect( "expose-event", self.handlePreviewExpose )
        self.GUI["previewEventBox"].add( self.GUI["previewDA"] )

        self.GUI["mainBox"].show_all()

        self.previewDA.alloced = False
        self.previewDirty = False
        self.previewDirtyRect = gtk.gdk.Rectangle( 0, 0, 0, 0 )
        self.dirtyRectToAdd = gtk.gdk.Rectangle( 0, 0, 0, 0 )

        self.sampleBg = self.owner.sampleBg
        self.GUI["previewDA"].set_size_request( -1, self.sampleBg.get_size()[1] )
        self.sampleNoteHeight = self.owner.sampleNoteHeight
        self.sampleNoteMask = self.owner.sampleNoteMask

        self.pitchPerPixel = float(Config.NUMBER_OF_POSSIBLE_PITCHES-1) / (self.sampleBg.get_size()[1] - self.sampleNoteHeight)
        self.pixelsPerPitch = float(self.sampleBg.get_size()[1] - self.sampleNoteHeight)/(Config.MAXIMUM_PITCH - Config.MINIMUM_PITCH)
        # Temporary Initialization
        self.pixelsPerTick = [0] + [ 1 for i in range(1,Config.MAXIMUM_BEATS+1) ]
        self.ticksPerPixel = [0] + [ 1 for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.cursor = { \
            "default":          None, \
            "drag-onset":       gtk.gdk.Cursor(gtk.gdk.SB_RIGHT_ARROW), \
            "drag-pitch":       gtk.gdk.Cursor(gtk.gdk.BOTTOM_SIDE), \
            "drag-duration":    gtk.gdk.Cursor(gtk.gdk.RIGHT_SIDE), \
            "drag-playhead":    gtk.gdk.Cursor(gtk.gdk.SB_H_DOUBLE_ARROW), \
            "pencil":           gtk.gdk.Cursor(gtk.gdk.PENCIL), \
            "paste":            gtk.gdk.Cursor(gtk.gdk.CENTER_PTR), \
            "error":            None }

        self.recording = False
        self.recordLoop = None
        self.recordingNote = None

        self.owner.noteDB.addListener( self, LoopParasite )

    def destroy( self ):
        self.owner.noteDB.deleteListener( self )

        Popup.destroy()

    def setBlock( self, block ):
        self.settingBlock = True

        if self.GUI["recordButton"].get_active():
            self.GUI["recordButton"].set_active( False )

        self.block = block
        self.GUI["beatsAdjustment"].set_value( block.getData( "beats" ) )
        self.GUI["regularityAdjustment"].set_value( block.getData( "regularity" ) )
        
        root = block.getRoot()
        if root.type == Block.Instrument:
            self.instrument = { "id":        root.getData( "id" ),
                                "amplitude": root.getData( "volume" ),
                                "pan":       root.getData( "pan" ),
                                "reverb":    root.getData( "reverb" ) }
        else:
            self.instrument = self.owner.getInstrument()

        self.curPage = block.getData("id")
        self.curBeats = block.getData("beats")

        self.selectedNotes = [ [] for i in range(Config.NUMBER_OF_TRACKS) ]

        self.curAction = False          # stores the current mouse action
        self.curActionObject = False    # stores the object that in handling the action

        self.lastDO = self.lastDP = self.lastDrumDP = self.lastDD = None

        self.clickButton = 0        # used in release and motion events to make sure we where actually the widget originally clicked. (hack for popup windows)
        self.buttonPressCount = 1   # used on release events to indicate double/triple releases
        self.clickLoc = [0,0]       # location of the last click
        self.marqueeLoc = False     # current drag location of the marquee
        self.marqueeRect = [[0,0],[0,0]]

        self.playheadT = 0
        self.playheadX = Config.TRACK_SPACING_DIV2

        self.settingBlock = False

        if self.previewDA.alloced:
            self.invalidatePreview( 0, 0, self.previewDA.width, self.previewDA.height, -1, True )

        Popup.setBlock( self, block )

    def popdown( self, immediate = False ):
        self.applyNoteSelection( SELECTNOTES.NONE, 0, [], self.curPage )

        if self.GUI["recordButton"].get_active():
            self.GUI["recordButton"].set_active( False )

        Popup.popdown( self, immediate )

    def getPage( self ):
        if self.block != None:
            return self.block.getData("id")
        else:
            return -1

    #=======================================================
    # Handelers

    def handleBeats( self, widget ):
        # snap to 0 decimal places
        val = widget.get_value()
        if round( val ) != val:
            widget.set_value( round( val ) )
            return

        if not self.settingBlock:
            self.curBeats = int(round( widget.get_value() )) 
            self.block.setData( "beats", self.curBeats )
            for n in self.owner.noteDB.getNotesByTrack( self.getPage(), 0, self ):
                n.updateTransform( True )
            self.invalidatePreview( 0, 0, self.previewDA.width, self.previewDA.height )

        if self.recordLoop:
            self.recordLoop = self.owner._playLoop( self.instrument["id"], self.instrument["amplitude"], self.instrument["reverb"], [ self.curPage ], self.recordLoop, force = True )

    def handleRegularity( self, widget ):
        if not self.settingBlock:
            self.block.setData( "regularity", widget.get_value() )

    def handleRegenerate( self, widget ):
        parameters = GenerationParameters( 
            rythmRegularity = self.block.getData( "regularity" ), 
            pitchRegularity = self.block.getData( "regularity" ) ) 

        self.owner._generateTrack( self.instrument["id"], self.curPage, 0, parameters, generator1 )
        
        self.block.updateLoop()
        
        if self.recordLoop:
            self.recordLoop = self.owner._playLoop( self.instrument["id"], self.instrument["amplitude"], self.instrument["reverb"], [ self.curPage ], self.recordLoop, force = True )

    def handleClear( self, widget ):
        self.block.clear()

        if self.recordLoop:
            self.recordLoop = self.owner._playLoop( self.instrument["id"], self.instrument["amplitude"], self.instrument["reverb"], [ self.curPage ], self.recordLoop, force = True )

    def handleRecord( self, widget ):
        if widget.get_active():
            self.startRecording()
        else:
            self.stopRecording()

    def handlePreviewPress( self, widget, event ):
        if event.button != 1:
            return

        self.clickButton = event.button

        if event.type == gtk.gdk._2BUTTON_PRESS:   self.buttonPressCount = 2
        elif event.type == gtk.gdk._3BUTTON_PRESS: self.buttonPressCount = 3
        else:                                      self.buttonPressCount = 1

        self.clickLoc = [ int(event.x), int(event.y) ]

        page = self.block.getData("id")
        beats = self.block.getData("beats")

        notes = self.noteDB.getNotesByTrack( page, 0, self )
        last = len(notes)-1
        handled = 0
        for n in range(last+1):
            handled = notes[n].handleButtonPress( self, event )
            if handled == 0:
                continue
            elif handled == 1:
                if not self.curAction: self.curAction = True # it was handled but no action was declared, set curAction to True anyway
                return
            else:      # all other options mean we can stop looking
                break

        if not handled or handled == -1:  # event didn't overlap any notes, so we can draw
            pitch = min( self.pixelsToPitchFloor( self.clickLoc[1] - self.previewDA.height + self.sampleNoteHeight//2 ), Config.NUMBER_OF_POSSIBLE_PITCHES-1) + Config.MINIMUM_PITCH
            onset = self.pixelsToTicksFloor( beats, self.clickLoc[0] )
            cs = CSoundNote( onset,
                             pitch,
                             0.75,
                             0.5,
                             1,
                             0,
                             instrumentId = self.instrument["id"] )
            cs.pageId = page
            id = self.noteDB.addNote( -1, page, 0, cs )
            n = self.noteDB.getNote( page, 0, id, self )
            self.selectNotes( { 0:[n] }, True )
            n.playSampleNote( False )

            noteS = self.noteDB.getNotesByTrack( page, 0 )
            for note in noteS:
                if note.cs.onset < onset and (note.cs.onset + note.cs.duration) > onset:
                    self.noteDB.updateNote(self.curPage, 0, note.id, PARAMETER.DURATION, onset - note.cs.onset)

            self.updateDragLimits()
            self.clickLoc[0] += self.ticksToPixels( beats, 1 )
            self.setCurrentAction( "note-drag-duration", n )
            self.setCursor("drag-duration")

    def handlePreviewRelease( self, widget, event ):
        if not self.clickButton: return # we recieved this event but were never clicked! (probably a popup window was open)
        self.clickButton = 0

        if event.button != 1:
            return

        if not self.curAction:
            self.applyNoteSelection( SELECTNOTES.NONE, 0, [], self.curPage )
            return

        if not self.curActionObject: # there was no real action to carry out
            self.curAction = False
            return

        if self.curActionObject != self:
            self.curActionObject.handleButtonRelease( self, event, self.buttonPressCount )
            self.updateTooltip( event )
        else:
            # we're doing the action ourselves
            if self.curAction == "marquee":         self.doneMarquee( event )
            self.updateTooltip( event )

    def handlePreviewMotion( self, widget, event ):
        if event.is_hint:
            x, y, state = self.previewDA.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        if not self.clickButton: # we recieved this event but were never clicked! (probably a popup window was open)
            self.updateTooltip( event )
            return

        if event.state & gtk.gdk.BUTTON1_MASK:
            if not self.curAction: # no action is in progress yet we're dragging, start a marquee
                self.setCurrentAction( "marquee", self )

            if self.curAction == "note-drag-onset":
                self.noteDragOnset( event )

            elif self.curAction == "note-drag-duration":
                self.noteDragDuration( event )

            elif self.curAction == "note-drag-pitch":
                self.noteDragPitch( event )

            #elif self.curAction == "note-drag-pitch-drum":
            #    self.noteDragPitch( event, True )

            elif self.curAction == "marquee":
                self.updateMarquee( event )
        else:
            self.updateTooltip( event )

    def handlePreviewLeave( self, widget, event ):
        self.setCursor("default")

    def handlePreviewAlloc( self, widget, allocation ):
        self.previewDA.alloced = True
        win = gtk.gdk.get_default_root_window()
        self.previewDA.width = allocation.width
        self.previewDA.height = allocation.height
        self.previewBuffer = gtk.gdk.Pixmap( win, allocation.width, allocation.height )
        self.clearClipMask = gtk.gdk.Rectangle( 0, 0, allocation.width, allocation.height )

        self.pixelsPerTick = [0] + [ self.previewDA.width/float(i*Config.TICKS_PER_BEAT) for i in range(1,Config.MAXIMUM_BEATS+1) ]
        self.ticksPerPixel = [0] + [ 1.0/self.pixelsPerTick[i] for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.beatSpacing = [[0]]
        for i in range(1,Config.MAXIMUM_BEATS+1):
            self.beatSpacing.append( [ self.ticksToPixels( i, Config.TICKS_PER_BEAT*j ) for j in range(i) ] )

        for n in self.owner.noteDB.getNotes( self ):
            n.updateTransform( True )

        self.invalidatePreview( 0, 0, allocation.width, allocation.height, -1, True )

    def on_key_press( self, widget, event ):
        keyval = event.keyval

        # backspace and del keys
        if keyval == gtk.keysyms.Delete or keyval == gtk.keysyms.BackSpace:
            if len( self.selectedNotes[0] ):
                self.owner.noteDB.deleteNotes( 
                    [ self.curPage, 0, len( self.selectedNotes[0] ) ] 
                  + [ n.note.id for n in self.selectedNotes[0] ]
                  + [ -1 ] )
            self.block.updateLoop()
        else:
            self.owner.onKeyPress( widget, event )

    #=======================================================
    # Drawing 

    def previewDraw( self ):
        startX = self.previewDirtyRect.x
        startY = self.previewDirtyRect.y
        stopX = self.previewDirtyRect.x + self.previewDirtyRect.width
        stopY = self.previewDirtyRect.y + self.previewDirtyRect.height

        page = self.block.getData("id")
        beats = self.owner.noteDB.getPage(page).beats

        self.gc.set_clip_rectangle( self.previewDirtyRect )

        # draw background
        self.previewBuffer.draw_drawable( self.gc, self.sampleBg, 0, 0, 0, 0, self.previewDA.width-5, self.previewDA.height )
        self.previewBuffer.draw_drawable( self.gc, self.sampleBg, self.sampleBg.endOffset, 0, self.previewDA.width-5, 0, 5, self.previewDA.height )
 
        # draw beat lines
        self.gc.set_line_attributes( Config.BEAT_LINE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
        self.gc.foreground = self.colors["Beat_Line"]
        for i in range(1,beats):
            x = self.beatSpacing[beats][i]
            self.previewBuffer.draw_line( self.gc, x, 1, x, self.previewDA.height-1 )

        # draw notes
        self.gc.set_clip_mask( self.sampleNoteMask )
        notes = self.owner.noteDB.getNotesByTrack( page, 0, self )
        for n in notes:
            if not n.draw( self.previewBuffer, self.gc, startX, stopX ): break

        self.previewDirty = False

    def handlePreviewExpose( self, widget, event ):
        if self.previewDirty:
            self.previewDraw()

        self.gc.set_clip_rectangle( event.area )

        # draw base
        widget.window.draw_drawable( self.gc, self.previewBuffer, event.area.x, event.area.y, event.area.x, event.area.y, event.area.width, event.area.height )

        if self.marqueeLoc: # draw the selection rect
            self.gc.set_line_attributes( Config.MARQUEE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            self.gc.foreground = self.colors["Preview_Note_Selected"]
            widget.window.draw_rectangle( self.gc, False, self.marqueeRect[0][0], self.marqueeRect[0][1], self.marqueeRect[1][0], self.marqueeRect[1][1] )

        if self.recording: # draw playhead
            self.gc.set_line_attributes( Config.PLAYHEAD_SIZE, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            self.gc.foreground = self.colors["black"]
            widget.window.draw_line( self.gc, self.playheadX, event.area.y, self.playheadX, event.area.y + event.area.height )

    def invalidatePreview( self, x, y, width, height, page = -1, base = True ):
        if page != -1 and page != self.getPage():
            return

        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height

        if base: # the base image has been dirtied
            if not self.previewDirty:
                self.previewDirtyRect.x = x
                self.previewDirtyRect.y = y
                self.previewDirtyRect.width = width
                self.previewDirtyRect.height = height
            else:
               self.previewDirtyRect = self.previewDirtyRect.union( self.dirtyRectToAdd )
            self.previewDirty = True

        if self.previewDA.window != None:
            self.previewDA.window.invalidate_rect( self.dirtyRectToAdd, True )

    #=======================================================
    # Recording

    def startRecording( self ):
        if self.recording:
            return

        #self.owner.setPaused( True )
        self.owner.pushInstrument( self.instrument )
        self.owner.setKeyboardListener( self )
        
        self.recordLoop = self.owner._playLoop( self.instrument["id"], self.instrument["amplitude"], self.instrument["reverb"], [ self.curPage ], force = True )
        self.updatePlayhead()
        self.recordTimeout = gobject.timeout_add( 20, self._record_timeout )
        self.recording = True

    def stopRecording( self ):
        if not self.recording:
            return

        #self.owner.setPaused( False )
        self.owner.popInstrument()
        self.owner.setKeyboardListener( None )

        gobject.source_remove( self.recordTimeout )
        self.recording = False

        if self.recordingNote:
            self.finishNote()

        self.owner._stopLoop( self.recordLoop )
        self.recordLoop = None
        self.clearPlayhead()

    def recordNote( self, pitch ):
        onset = self.csnd.loopGetTick( self.recordLoop )
        #onset = Config.DEFAULT_GRID * int(onset / Config.DEFAULT_GRID + 0.5)

        cs = CSoundNote( onset,
                         pitch,
                         0.75,
                         0.5,
                         Config.DEFAULT_GRID,
                         0,
                         instrumentId = self.instrument["id"] )
        cs.pageId = self.curPage

        for n in self.noteDB.getNotesByTrack( self.curPage, 0 ):
            if onset < n.cs.onset:
                break
            if onset >= n.cs.onset + n.cs.duration:
                continue
            if onset < n.cs.onset + n.cs.duration - 2:
                self.noteDB.deleteNote( n.page, n.track, n.id )
            elif onset - n.cs.onset < 1:
                self.noteDB.deleteNote( n.page, n.track, n.id )
            else:
                self.noteDB.updateNote( n.page, n.track, n.id, PARAMETER.DURATION, onset - n.cs.onset )
            break

        self.recordingNote = self.noteDB.addNote( -1, self.curPage, 0, cs )

        self.recordLoop = self.owner._playLoop( self.instrument["id"], self.instrument["amplitude"], self.instrument["reverb"], [ self.curPage ], self.recordLoop, force = True )

    def finishNote( self ):
        self.recordingNote = None

        self.block.updateLoop()

    def _updateNote( self ):
        tick = self.csnd.loopGetTick( self.recordLoop )
        #tick = Config.DEFAULT_GRID * int(tick / Config.DEFAULT_GRID + 0.5)

        note = self.noteDB.getNote( self.curPage, 0, self.recordingNote )

        if tick < note.cs.onset:
            tick = self.noteDB.getPage( self.curPage ).ticks
            self.noteDB.updateNote( note.page, note.track, note.id, PARAMETER.DURATION, tick - note.cs.onset )
            for n in self.noteDB.getNotesByTrack( self.curPage, 0 ):
                if n.cs.onset <= note.cs.onset: 
                    continue
                if n.cs.onset > note.cs.onset and n.cs.onset < note.cs.onset + note.cs.duration:
                    self.noteDB.deleteNote( n.page, n.track, n.id )
                else:
                    break
            self.finishNote()
        elif tick > note.cs.onset + note.cs.duration:
            self.noteDB.updateNote( note.page, note.track, note.id, PARAMETER.DURATION, tick - note.cs.onset )
            for n in self.noteDB.getNotesByTrack( self.curPage, 0 ):
                if n.cs.onset <= note.cs.onset: 
                    continue
                if n.cs.onset > note.cs.onset and n.cs.onset < note.cs.onset + note.cs.duration:
                    self.noteDB.deleteNote( n.page, n.track, n.id )
                else:
                    break
            
    def _record_timeout( self ):
        self.updatePlayhead()
        if self.recordingNote:
            self._updateNote()
        return True

    def updatePlayhead( self ):
        ticks = self.csnd.loopGetTick( self.recordLoop )
        if self.playheadT != ticks:
            self.invalidatePreview( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.previewDA.height, self.curPage, False )
            self.playheadX = self.ticksToPixels( self.curBeats, ticks )
            self.invalidatePreview( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.previewDA.height, self.curPage, False )
            self.playheadT = ticks

        return True

    def clearPlayhead( self ):
        self.invalidatePreview( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.previewDA.height, self.curPage, False )

    #=======================================================
    # Actions

    def setCurrentAction( self, action, obj = None ):
        if self.curAction:
            self.doneCurrentAction()

        self.curAction = action
        self.curActionObject = obj

        if   action == "note-drag-onset":      self.updateDragLimits()
        elif action == "note-drag-duration":   self.updateDragLimits()
        elif action == "note-drag-pitch":      self.updateDragLimits()
        #elif action == "note-drag-pitch-drum": self.updateDragLimits()

    def doneCurrentAction( self ):
        if not self.curAction: return
        action = self.curAction
        self.curAction = False

        if   action == "note-drag-onset":      self.doneNoteDrag( action )
        elif action == "note-drag-duration":   self.doneNoteDrag( action )
        elif action == "note-drag-pitch":      self.doneNoteDrag( action )
        #elif action == "note-drag-pitch-drum": self.doneNoteDrag( action )

    def selectionChanged( self ):
        if   self.curAction == "note-drag-onset":      self.updateDragLimits()
        elif self.curAction == "note-drag-duration":   self.updateDragLimits()
        elif self.curAction == "note-drag-pitch":      self.updateDragLimits()
        #elif self.curAction == "note-drag-pitch-drum": self.updateDragLimits()

    def applyNoteSelection( self, mode, trackN, which, page = -1 ):
        if page == -1: page = self.curPage
        if mode == SELECTNOTES.ALL:
            track = self.noteDB.getNotesByTrack( page, trackN, self )
            map( lambda note:note.setSelected( True ), track )
            self.selectedNotes[trackN] = []
            map( lambda note:self.selectedNotes[trackN].append(note), track )
        elif mode == SELECTNOTES.NONE:
            track = self.selectedNotes[trackN] #self.noteDB.getNotesByTrack( page, trackN, self )
            map( lambda note:note.setSelected( False ), track )
            self.selectedNotes[trackN] = []
        elif mode == SELECTNOTES.ADD:
            for note in which:
                if note.setSelected( True ):
                    self.selectedNotes[trackN].append( note )
        elif mode == SELECTNOTES.REMOVE:
            for note in which:
                if note.setSelected( False ):
                    self.selectedNotes[trackN].remove( note )
        elif mode == SELECTNOTES.FLIP:
            for note in which:
                if note.getSelected():
                    note.setSelected( False )
                    self.selectedNotes[trackN].remove( note )
                else:
                    note.setSelected( True )
                    self.selectedNotes[trackN].append( note )
        elif mode == SELECTNOTES.EXCLUSIVE:
            notes = self.noteDB.getNotesByTrack( page, trackN, self )
            for n in range(len(notes)):
                if notes[n] in which:
                    if notes[n].setSelected( True ):
                        self.selectedNotes[trackN].append( notes[n] )
                else:
                    if notes[n].setSelected( False ):
                        self.selectedNotes[trackN].remove( notes[n] )

    def selectNotesByBar( self, trackN, start, stop, page = -1 ):
        for i in range(Config.NUMBER_OF_TRACKS):
            if i == trackN:
                notes = []
                track = self.noteDB.getNotesByTrack( self.curPage, trackN, self )
                for n in range(len(track)):
                    if track[n].testOnset( start, stop ): notes.append(track[n])
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, trackN, notes, page )
                else:                           self.applyNoteSelection( SELECTNOTES.ADD, trackN, notes, page )
            else:
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def selectNotesByTrack( self, trackN, page = -1 ):
        if Config.ModKeys.ctrlDown:
            self.applyNoteSelection( SELECTNOTES.ALL, trackN, [], page )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i == trackN: self.applyNoteSelection( SELECTNOTES.ALL, trackN, [], page )
                else:           self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def selectNotes( self, noteDic, ignoreCtrl = False, page = -1 ):
        if Config.ModKeys.ctrlDown and not ignoreCtrl:
            for i in noteDic:
                self.applyNoteSelection( SELECTNOTES.FLIP, i, noteDic[i], page )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i in noteDic: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, i, noteDic[i], page )
                else:            self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def deselectNotes( self, noteDic, page = -1 ):
        for i in noteDic:
            self.applyNoteSelection( SELECTNOTES.REMOVE, i, noteDic[i], page )
        self.selectionChanged()

    def clearSelectedNotes( self, page = -1 ):
        for i in range(Config.NUMBER_OF_TRACKS):
            self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        maxRightBound = self.noteDB.getPage(self.curPage).ticks

        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            track = self.noteDB.getNotesByTrack( self.curPage, i, self )
            leftBound = 0
            skip = True # skip the first note
            for n in range(len(track)):
                if skip:
                    skip = False
                    thisNote = track[n]
                    continue
                nextNote = track[n]
                if not thisNote.getSelected():
                    leftBound = thisNote.getEndTick()
                else:
                    if not nextNote.getSelected():
                        rightBound = min( nextNote.getStartTick(), maxRightBound )
                        widthBound = rightBound
                    else:
                        rightBound = maxRightBound
                        widthBound = min( nextNote.getStartTick(), maxRightBound )
                    thisNote.updateDragLimits( self.dragLimits, leftBound, rightBound, widthBound, maxRightBound )
                thisNote = nextNote
            # do the last note
            if thisNote.getSelected():
                thisNote.updateDragLimits( self.dragLimits, leftBound, maxRightBound, maxRightBound, maxRightBound )

    def noteDragOnset( self, event ):
        do = self.pixelsToTicks( self.curBeats, event.x - self.clickLoc[0] )
        do = min( self.dragLimits[0][1], max( self.dragLimits[0][0], do ) )

        if do != self.lastDO:
            self.lastDO = do
            stream = []
            for i in range(Config.NUMBER_OF_TRACKS):
                tstream = []
                for note in self.selectedNotes[i]:
                    note.noteDragOnset( do, tstream )
                if len(tstream):
                    stream += [ self.curPage, i, PARAMETER.ONSET, len(tstream)//2 ] + tstream
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def noteDragDuration( self, event ):
        dd = self.pixelsToTicks( self.curBeats, event.x - self.clickLoc[0] )
        dd = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dd ) )

        if dd != self.lastDD:
            self.lastDD = dd
            stream = []
            for i in range(Config.NUMBER_OF_TRACKS):
                tstream = []
                for note in self.selectedNotes[i]:
                    note.noteDragDuration( dd, tstream )
                if len(tstream):
                    stream += [ self.curPage, i, PARAMETER.DURATION, len(tstream)//2 ] + tstream
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def noteDragPitch( self, event, drum = False ):
        if not drum: dp = self.pixelsToPitch( event.y - self.clickLoc[1] )
        else: dp = self.pixelsToPitchDrum( event.y - self.clickLoc[1] )
        dp = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dp ) )

        if dp != self.lastDP:
            self.lastDP = dp
            stream = []
            for i in range(Config.NUMBER_OF_TRACKS):
                tstream = []
                for note in self.selectedNotes[i]:
                    note.noteDragPitch( dp, tstream )
                if len(tstream):
                    stream += [ self.curPage, i, PARAMETER.PITCH, len(tstream)//2 ] + tstream
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

            self.curActionObject.playSampleNote( True )

    def doneNoteDrag( self, action ):
       # if action == "note-drag-pitch" or action == "note-drag-pitch-drum":
       #     self.curActionObject.playSampleNote()

        self.lastDO = self.lastDP = self.lastDrumDP = self.lastDD = None

        for i in range(Config.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.doneNoteDrag( self )

        self.block.updateLoop()

    def noteStepOnset( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            track = self.noteDB.getNotesByTrack( self.curPage, i, self )
            if step < 0: # moving to the left, iterate forwards
                leftBound = 0
                for n in range(len(track)):
                    leftBound = track[n].noteDecOnset( step, leftBound, tstream )
            else:        # moving to the right, iterate backwards
                rightBound = self.noteDB.getPage(self.curPage).ticks
                for n in range(len(track)-1, -1, -1 ):
                    rightBound = track[n].noteIncOnset( step, rightBound, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.ONSET, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepPitch( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecPitch( step, tstream )
            else:
                for n in self.selectedNotes[i]:
                    n.noteIncPitch( step, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.PITCH, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepDuration( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecDuration( step, tstream )
            else:
                track = self.noteDB.getNotesByTrack( self.curPage, i, self )
                for j in range(len(track)-1):
                    track[j].noteIncDuration( step, track[j+1].getStartTick(), tstream )
                track[len(track)-1].noteIncDuration( step, self.noteDB.getPage(self.curPage).ticks, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.DURATION, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepVolume( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecVolume( step, tstream )
            else:
                for n in self.selectedNotes[i]:
                    n.noteIncVolume( step, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.AMPLITUDE, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def updateMarquee( self, event ):
        if self.marqueeLoc:
            oldX = self.marqueeRect[0][0]
            oldEndX = self.marqueeRect[0][0] + self.marqueeRect[1][0]
            oldY = self.marqueeRect[0][1]
            oldEndY = self.marqueeRect[0][1] + self.marqueeRect[1][1]
        else:
            oldX = oldEndX = self.clickLoc[0]
            oldY = oldEndY = self.clickLoc[1]

        self.marqueeLoc = [ int(event.x), int(event.y) ]
        if self.marqueeLoc[0] < 0: self.marqueeLoc[0] = 0
        elif self.marqueeLoc[0] > self.previewDA.width: self.marqueeLoc[0] = self.previewDA.width
        if self.marqueeLoc[1] < 0: self.marqueeLoc[1] = 0
        elif self.marqueeLoc[1] > self.previewDA.height: self.marqueeLoc[1] = self.previewDA.height

        if self.marqueeLoc[0] > self.clickLoc[0]:
            self.marqueeRect[0][0] = self.clickLoc[0]
            self.marqueeRect[1][0] = self.marqueeLoc[0] - self.clickLoc[0]
        else:
            self.marqueeRect[0][0] = self.marqueeLoc[0]
            self.marqueeRect[1][0] = self.clickLoc[0] - self.marqueeLoc[0]
        if self.marqueeLoc[1] > self.clickLoc[1]:
            self.marqueeRect[0][1] = self.clickLoc[1]
            self.marqueeRect[1][1] = self.marqueeLoc[1] - self.clickLoc[1]
        else:
            self.marqueeRect[0][1] = self.marqueeLoc[1]
            self.marqueeRect[1][1] = self.clickLoc[1] - self.marqueeLoc[1]

        x = min( self.marqueeRect[0][0], oldX )
        width = max( self.marqueeRect[0][0] + self.marqueeRect[1][0], oldEndX ) - x
        y = min( self.marqueeRect[0][1], oldY )
        height = max( self.marqueeRect[0][1] + self.marqueeRect[1][1], oldEndY ) - y
        self.invalidatePreview( x-1, y-1, width+2, height+2, self.curPage, False )

    def doneMarquee( self, event ):
        if self.marqueeLoc:
            stop =  [ self.marqueeRect[0][0] + self.marqueeRect[1][0], self.marqueeRect[0][1] + self.marqueeRect[1][1] ]

            select = {}

            intersectionY = [ self.marqueeRect[0][1], stop[1] ]
            
            notes = []
            track = self.noteDB.getNotesByTrack( self.getPage(), 0, self )
            for n in range(len(track)):
                hit = track[n].handleMarqueeSelect( self,
                                  [ self.marqueeRect[0][0], intersectionY[0] ], \
                                  [ stop[0], intersectionY[1] ] )
                if hit: notes.append(track[n])

            if len(notes): select[0] = notes

            self.selectNotes( select )

        self.marqueeLoc = False
        self.doneCurrentAction()

        self.invalidatePreview( self.marqueeRect[0][0]-1, self.marqueeRect[0][1]-1, self.marqueeRect[1][0]+2, self.marqueeRect[1][1]+2, self.getPage(), False )

    def updateTooltip( self, event ):

        notes = self.noteDB.getNotesByTrack( self.getPage(), 0, self )
        handled = 0
        for n in range(len(notes)):
            handled = notes[n].updateTooltip( self, event )
            if handled == 0:   continue
            elif handled == 1: return   # event was handled
            else:              break

        if handled == -2: # event X overlapped with a note
            self.setCursor("default")
            return

        self.setCursor("pencil")

    def setCursor( self, cursor ):
        self.window.set_cursor(self.cursor[cursor])

    def ticksToPixels( self, beats, ticks ):
        return int(round( ticks * self.pixelsPerTick[beats] ))
    def pixelsToTicks( self, beats, pixels ):
        return int(round( pixels * self.ticksPerPixel[beats] ))
    def pitchToPixels( self, pitch ):
        return int(round( ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))
    def ticksToPixelsFloor( self, beats, ticks ):
        return int( ticks * self.pixelsPerTick[beats] )
    def pixelsToTicksFloor( self, beats, pixels ):
        return int( pixels * self.ticksPerPixel[beats] )
    def pixelsToPitch( self, pixels ):
        return int(round(-pixels*self.pitchPerPixel))
    def pitchToPixelsFloor( self, pitch ):
        return int(( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch )
    def pixelsToPitchFloor( self, pixels ):
        return int(-pixels*self.pitchPerPixel)
 

class Shortcut( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

        self.gc = self.owner.gc

        self.GUI = {}

        self.GUI["mainBox"] = gtk.VBox()
        self.set_content( self.GUI["mainBox"] )

        #-- Keys ----------------------------------------------
        # match keycodes from JamMain.valid_shortcuts
        layout = [ [ 0.0, [ 18, 19, 20, 21 ] ],
                   [ 0.3, [ 32, 33, 34, 35 ] ],
                   [ 1.7, [ 47, 48, 51 ] ],
                   [ 1.1, [ 60, 61 ] ] ]

        self.GUI["keyBox"] = gtk.VBox()
        self.GUI["mainBox"].pack_start( self.GUI["keyBox"], padding = style.DEFAULT_PADDING - 2 )

        for row in layout:
            offset = row[0]
            hbox = gtk.HBox()
            self.GUI["keyBox"].pack_start( hbox, padding = 2 )
            separator = gtk.Label("")
            separator.set_size_request( int(Block.Block.KEYSIZE*row[0]) + style.DEFAULT_PADDING, -1 )
            hbox.pack_start( separator, False )
            separator = gtk.Label("")
            separator.set_size_request( style.DEFAULT_PADDING, -1 )
            hbox.pack_end( separator, False )
            for key in row[1]:
                self.GUI[key] = gtk.ToggleButton()
                self.GUI[key].connect( "expose-event", self.keyExpose )
                self.GUI[key].connect( "toggled", self.keyToggled )
                self.GUI[key].set_size_request( Block.Block.KEYSIZE, Block.Block.KEYSIZE )
                self.GUI[key].key = key
                self.GUI[key].image = [ self.owner.getKeyImage( key, False ),
                                        self.owner.getKeyImage( key, True ) ]
                hbox.pack_start( self.GUI[key], False, padding = 2 )

        #-- None ----------------------------------------------
        self.GUI["noneBox"] = gtk.HBox()
        self.GUI["mainBox"].pack_start( self.GUI["noneBox"], padding = style.DEFAULT_PADDING )
        self.GUI["noneButton"] = gtk.Button( _("None") )
        self.GUI["noneButton"].connect( "clicked", self.handleNone )
        self.GUI["noneBox"].pack_start( self.GUI["noneButton"], True, False, padding = style.DEFAULT_PADDING )

        self.GUI["mainBox"].show_all()

        self.key = None
 
    def setBlock( self, block ):
        self.ignoreToggle = True

        self.block = block
        self.key = self.block.getData( "key" )

        if self.key != None:
            self.GUI[self.key].set_active( True )

        self.ignoreToggle = False

        Popup.setBlock( self, block )

    def on_key_press( self, widget, event ):
        key = event.hardware_keycode
        if key in self.owner.valid_shortcuts.keys():
            self.block.setData( "key", key )
            if self.key != None: # clear old key
                self.ignoreToggle = True
                self.GUI[self.key].set_active( False )
                self.key = None 
                self.ignoreToggle = False 
            self.popdown( True )
        else:
            self.owner.onKeyPress( widget, event )

    def keyExpose( self, widget, event ):
        self.gc.set_clip_mask( self.owner.blockMask )
        self.gc.set_clip_origin( event.area.x - Block.Block.KEYMASK_START, event.area.y )
        widget.window.draw_drawable( self.gc, widget.image[widget.get_active()], 0, 0, event.area.x, event.area.y, event.area.width, event.area.height )
        return True

    def keyToggled( self, widget ):
        if self.ignoreToggle:
            return

        if widget.get_active():
            self.block.setData( "key", widget.key )
            
            self.ignoreToggle = True

            if self.key != None: # clear old key
                self.GUI[self.key].set_active( False )
                self.key = None

            widget.set_active( False )

            self.ignoreToggle = False

        self.popdown( True )

    def handleNone( self, widget ):
        if self.key != None:
            self.ignoreToggle = True
            self.GUI[self.key].set_active( False )
            self.key = None
            self.ignoreToggle = False 

        self.block.setData( "key", None )

        self.popdown( True )
