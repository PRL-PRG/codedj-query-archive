from Framework.Constants import Constants

class GUIConstants:
    LANGUAGE = 'En' 
    IMAGE_ROOT = Constants.TAM_TAM_ROOT + '/Resources/Images/'
    
    NOTE_HEIGHT = 6     # pixels
    NOTE_BORDER_SIZE = 1
    NOTE_BORDER_SIZE_DIV2 = NOTE_BORDER_SIZE/2.0
    MAIN_WINDOW_PADDING = 5
    TRACK_SPACING = 1
    BORDER_SIZE = 2
    BORDER_SIZE_DIV2 = BORDER_SIZE/2.0
    BORDER_SIZE_MUL2 = BORDER_SIZE*2
    BEAT_LINE_SIZE = 1
    BEAT_LINE_SIZE_DIV2 = BEAT_LINE_SIZE/2.0
    PLAYHEAD_SIZE = 2
    PLAYHEAD_SIZE_DIV2 = PLAYHEAD_SIZE/2.0
            
    INST_BCK_COLOR = '#979DA8'
    PANEL_BCK_COLOR =  '#FFFFFF'
    PANEL_COLOR = '#707F93'
    
    PAGE_BORDER_SIZE = 2
    PAGE_SELECTED_BORDER_SIZE = 5
    PAGE_WIDTH = 100
    PAGE_HEIGHT = 25
    
    PAGE_THUMBNAIL_WIDTH = 70
    PAGE_THUMBNAIL_WIDTH_DIV2 =     PAGE_THUMBNAIL_WIDTH/2
    PAGE_THUMBNAIL_HEIGHT = 50
    PAGE_THUMBNAIL_PADDING = 4
    PAGE_THUMBNAIL_PADDING_MUL2 = PAGE_THUMBNAIL_PADDING*2
    PAGE_THUMBNAIL_PADDING_DIV2 = PAGE_THUMBNAIL_PADDING/2
	
    NUMBER_OF_PAGE_BANK_ROWS = 2
    NUMBER_OF_PAGE_BANK_COLUMNS = 20


# hardware keycodes for mod keys
MOD_LSHIFT = 50
MOD_RSHIFT = 62
MOD_LCTRL = 37
MOD_RCTRL = 109
MOD_LALT = 64
MOD_RALT = 113

class _ModKeys:
    def __init__( self ):        
        self.shiftDown = False
        self.ctrlDown = False
        self.altDown = False

    def keyPress( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = True
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = True
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = True   

    def keyRelease( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = False
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = False
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = False        

ModKeys = _ModKeys()
