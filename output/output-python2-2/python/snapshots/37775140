class UMLException(Exception):
    """
    Exception used for marking incorrect user operations
    """
    def __init__(self, *params):
        """
        Initialize this exception and set its values
        
        @param params: parameters for exception
        @type  params: tuple of anything, or anything
        """
        self.params = params


    def __str__(self): 
        """
        Create string representation of the exception
        
        @return: string representation
        @rtype:  string
        """
        if len(self.params) == 0:
            return ''

        elif len(self.params) == 1:
            return str(self.params[0])

        else:
            txt = ''
            for item in self.params:
                txt = txt + str(item) + ' '
            return txt


    def __getitem__(self, index):
        """
        Get parameter by given index
        
        @param index: index of parameter
        @type  index: integer
        
        @return: parameter value
        @rtype:  anything
        """
        try:
            return self.params[index]
        except:
            return None

    def __repr__(self):
        """
        Get name of this class
        
        @return: class name
        @rtype:  string
        """
        return self.__class__.__name__
    
    def GetName(self):
        '''
        Get first parameter e.g. text passed to exception
        '''
        return self.params[0]
    
    def GetParameter(self, idx):
        '''
        Get parameter of exception
        '''
        return self.params[idx]

