
import pygtk
pygtk.require( '2.0' )
import gtk

from gettext import gettext as _

from sugar.graphics.radiotoolbutton import RadioToolButton

class DesktopToolbar( gtk.Toolbar ):

    def __init__( self, owner ):
        gtk.Toolbar.__init__( self )

        self.owner = owner

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
 
        self.show_all()

    def setDesktop( self, widget, which ):
        if widget.get_active():
            self.owner._setDesktop( which )

