import logging
import gtk
import random
import hippo
import dbus
import game
import boardwidget

#from buddiespanel import BuddiesPanel
from infopanel import InfoPanel

board = game.GoBoard( 19 )

logger = logging.getLogger('PlayGO')

def redraw(grid):
    """Utility function to force a redraw of a Gtk widget."""
    grid.window.invalidate_rect(grid.get_allocation(), True)

def key_press_cb(window, event, grid, player):
    
    key = gtk.gdk.keyval_name(event.keyval)

    if key in ('Left',):
        for x in range( 19 ):
            board.setPoint(x, 3, 'Black' )
        redraw(grid)
        
    elif key in ('Right',):
        for x in range( 19 ):
            board.setPoint(x, 3, 'Empty' )
        redraw(grid)

    elif key in ( 'r', ):
        for x in range( 19 ):
            for y in range( 19 ):
                board.setPointi(x, y, random.randint( 0, 3 ) ) 
                
        redraw(grid)
       
    elif key in ( 'c', ):
        board.clear()
        redraw(grid)
       
    elif gtk.gdk.keyval_name(event.keyval) in ('Escape', 'q'):
        gtk.main_quit()

def main():
    
    logger.setLevel( logging.DEBUG )
    
    for x in range( 19 ):
        board.setPoint(x, 0, 'White' )
        
    window = gtk.Window()
    window.resize( 1200, 850 )
    boardWidget = boardwidget.BoardWidget( board )
    
    info_panels = InfoPanel()
    info_panels.show( " hello there I am the side layout test ")

    info_panel = InfoPanel()
    info_panel.show( " hello there I am the lower layout test ")

    vbox = hippo.CanvasBox(spacing=4,
        orientation=hippo.ORIENTATION_VERTICAL)

    hbox = hippo.CanvasBox(spacing=4,
        orientation=hippo.ORIENTATION_HORIZONTAL)

    hbox.append(hippo.CanvasWidget(widget=boardWidget), hippo.PACK_EXPAND )
    hbox.append(info_panels)
    
    vbox.append(hbox, hippo.PACK_EXPAND)
    vbox.append(info_panel, hippo.PACK_END)

    canvas = hippo.Canvas()
    canvas.set_root(vbox)

    window.add( canvas )
    window.show_all()
    window.connect('key-press-event', key_press_cb, boardWidget, [1])
   

# simple single window test
#
#    window = gtk.Window()
#    window.resize( 800, 800 )
#    window.connect('destroy', gtk.main_quit)
#    window.connect('key-press-event', key_press_cb, boardWidget, [1])
#    window.add(boardWidget)
#    window.show_all()

    try:
        gtk.main()
    except KeyboardInterrupt:
        pass

if __name__ == '__main__':
    main()

