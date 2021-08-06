
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from gettext import gettext as _
from sugar.graphics import style
from sugar.graphics.palette import Palette, Invoker, _palette_observer


class NoneInvoker( Invoker ):

    def __init__( self ):
        Invoker.__init__( self )

    def get_default_position( self ):
        return Palette.AT_CURSOR

    def get_rect( self ):
        return gtk.gdk.Rectangle( 0, 0, 0, 0 )

    def get_toplevel( self ):
        return None

class Popup( Palette ):

    def __init__( self, label, owner ):
        Palette.__init__( self, label )

        self.owner = owner

        self.props.invoker = NoneInvoker()
        self.set_property( "position", Palette.AT_CURSOR )
        self.set_group_id( "TamTamPopup" )

        self._set_state( Palette.SECONDARY ) # skip to fully exposed
 
        self.connect( "key-press-event", self.owner.onKeyPress )
        self.connect( "key-release-event", self.owner.onKeyRelease )

        self.connect( "focus_out_event", self.closePopup )

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

        Palette.popdown( self, immediate )

        self.owner.activity.handler_unblock(self.owner.activity.focusOutHandler)
        self.owner.activity.handler_unblock(self.owner.activity.focusInHandler)

    def updatePosition( self ):
        self._update_cursor_position()
        self._update_position()

    def closePopup( self, widget, event ):
        self.popdown( True )


class Instrument( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

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
        self.block = block
        self.GUI["volumeAdjustment"].set_value( block.getData( "volume" ) )
        self.GUI["panAdjustment"].set_value( block.getData( "pan" ) )
        self.GUI["reverbAdjustment"].set_value( block.getData( "reverb" ) )
        self.GUI["exportEntry"].set_text( block.getData( "name" ) )

    def handleVolume( self, widget ):
        self.block.setData( "volume", widget.get_value() )

    def handlePan( self, widget ):
        self.block.setData( "pan", widget.get_value() )

    def handleReverb( self, widget ):
        self.block.setData( "reverb", widget.get_value() )


class Drum( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )

        self.GUI = {}

        self.GUI["mainBox"] = gtk.VBox()
        self.set_content( self.GUI["mainBox"] )

        self.GUI["mainBox"].show_all()


class Shortcut( Popup ):
    
    def __init__( self, label, owner ):
        Popup.__init__( self, label, owner )


