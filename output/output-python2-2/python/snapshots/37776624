class CClipboard(object):
    """
    Class that represents clipboard for elements
    """
    
    def __init__(self):
        """
        Initialize clippboard and set it to empty
        """
        self.Clear()
    
    def Clear(self):
        """
        Clears the content of the clipboard
        """
        self.content = []
    
    def SetContent(self, content):
        """
        Sets the content of the clipboard
        
        @param content: List of elements to put into clipboard
        @type  content: iterator over L{CElement<lib.Drawing.Element.CElement>}(s)
        """
        self.content = [el for el in content]
    
    def GetContent(self):
        """
        Gets the content of the clipboard
        
        @return: List of elements in clipboard
        @rtype:  iterator over L{CElement<lib.Drawing.Element.CElement>}(s)
        """
        for el in self.content:
            yield el
