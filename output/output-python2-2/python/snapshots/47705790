
import gtk
import random

import game
import boardwidget

board = game.GoBoard( 19 )

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
        
    for x in range( 19 ):
        board.setPoint(x, 0, 'White' )
        
    grid = boardwidget.BoardWidget( board )
    
    window = gtk.Window()
    window.connect('destroy', gtk.main_quit)
    window.connect('key-press-event', key_press_cb, grid, [1])
    window.add(grid)
    window.show_all()

    try:
        gtk.main()
    except KeyboardInterrupt:
        pass

if __name__ == '__main__':
    main()

