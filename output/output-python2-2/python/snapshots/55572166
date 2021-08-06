
import pygtk
pygtk.require( '2.0' )
import gtk

from gettext import gettext as _

from sugar.graphics.palette import Palette, WidgetInvoker
from sugar.graphics.radiotoolbutton import RadioToolButton

import common.Config as Config


class JamToolbar( gtk.Toolbar ):
       
    def __init__( self, owner ):
        gtk.Toolbar.__init__( self )

        self.owner = owner

        self.toolItem = {}

        self.volumeImg = gtk.Image()

        self.volumeAdjustment = gtk.Adjustment( 0.0, 0, 1.0, 0.1, 0.1, 0 )
        self.volumeAdjustment.connect( 'value-changed', self.handleVolume )
        self.volumeSlider = gtk.HScale( adjustment = self.volumeAdjustment )
        self.volumeSlider.set_size_request( 450, -1 )
        self.volumeSlider.set_draw_value( False )
        self._add_tooltip( self.volumeSlider, _("Master Volume") )
        self._insert_widget( self.volumeSlider, -1 )
        self._insert_widget( self.volumeImg, -1 )

        self._insert_separator( True )

        self.tempoImg = gtk.Image()

        self.tempoAdjustment = gtk.Adjustment( Config.PLAYER_TEMPO_LOWER, Config.PLAYER_TEMPO_LOWER, Config.PLAYER_TEMPO_UPPER+1, 10, 10, 0 )
        self.tempoAdjustment.connect( 'value-changed', self.handleTempo )
        self.tempoSlider = gtk.HScale( adjustment = self.tempoAdjustment )
        self.tempoSlider.set_size_request( 450, -1 )
        self.tempoSlider.set_draw_value( False )
        self._add_tooltip( self.tempoSlider, _("Tempo") )
        self._insert_widget( self.tempoSlider, -1 )
        self._insert_widget( self.tempoImg, -1 )

        self.show_all()

    #def _add_palette( self, widget, palette, position = Palette.DEFAULT ):
    def _add_palette( self, widget, palette ):
        widget._palette = palette
        widget._palette.props.invoker = WidgetInvoker( widget )
        #widget._palette.set_property( "position", position )
    
    def _add_tooltip( self, widget, tooltip ):
        #self._add_palette( widget, Palette( tooltip ), Palette.DEFAULT )
        self._add_palette( widget, Palette( tooltip ) )

    def _insert_widget( self, widget, pos ):
        self.toolItem[ widget ] = gtk.ToolItem()
        self.toolItem[ widget ].add( widget )
        self.insert( self.toolItem[ widget ], pos )

    def _insert_separator( self, expand = False ):
        separator = gtk.SeparatorToolItem()
        separator.set_draw( False )
        separator.set_expand( expand )
        self.insert( separator, -1 )

    def mapRange( self, value, ilower, iupper, olower, oupper ):
        if value == iupper: 
            return oupper
        return olower + int( (oupper-olower+1)*(value-ilower)/float(iupper-ilower) )
        

    def handleVolume( self, widget ):
        self.owner._setVolume( widget.get_value() )

        img = self.mapRange( widget.value, widget.lower, widget.upper, 0, 3 )
        self.volumeImg.set_from_file(Config.TAM_TAM_ROOT + '/icons/volume' + str(img) + '.svg')

    def handleTempo( self, widget ):
        self.owner._setTempo( widget.get_value() )

        img = self.mapRange( widget.value, widget.lower, widget.upper, 1, 8 )
        self.tempoImg.set_from_file(Config.TAM_TAM_ROOT + '/icons/tempo' + str(img) + '.svg')


class DesktopToolbar( gtk.Toolbar ):

    def __init__( self, owner ):
        gtk.Toolbar.__init__( self )

        self.owner = owner

        self._insert_separator( True )

        self.desktop = []
    
        btn = RadioToolButton( 'preset1', group = None )
        btn.connect( 'toggled', self.setDesktop, 0 )
        btn.set_tooltip( _('Desktop 1') )
        self.insert( btn, -1 )
        self.desktop.append( btn )
        
        for i in range(2,11):
            btn = RadioToolButton( 'preset%d'%i, group = self.desktop[0] )
            btn.connect( 'toggled', self.setDesktop, i-1 )
            btn.set_tooltip( _('Desktop %d'%i) )
            self.insert( btn, -1 )
            self.desktop.append( btn )
 
        self._insert_separator( True )

        self.show_all()

    def _insert_separator( self, expand = False ):
        separator = gtk.SeparatorToolItem()
        separator.set_draw( False )
        separator.set_expand( expand )
        self.insert( separator, -1 )

    def getDesktopButton( self, which ):
        return self.desktop[which]

    def setDesktop( self, widget, which ):
        if widget.get_active():
            self.owner._setDesktop( which )

