class SynthObjectsParameters:

    def __init__( self ):
        self.types = [0,0,0,0,0,0,0,0,0,0,0,0]
        self.controlsParameters = [.5,1,0,0,.5,1,0,0,.5,1,0,0,.5,1,0,0]
        self.sourcesParameters = [1,.5,5,1,1,.5,5,1,1,.5,5,1,1,.5,5,1]
        self.fxsParameters = [100,3000,.8,1,100,3000,.8,1,100,3000,.8,1,100,3000,.8,1]
        self.outputParameters = [.01, .05, .9, .05]
        self.choiceParamsSet = [self.controlsParameters, self.sourcesParameters, self.fxsParameters, self.outputParameters]

    def update( self ):
        self.choiceParamsSet = [self.controlsParameters, self.sourcesParameters, self.fxsParameters, self.outputParameters]

    def getTypes( self ):
        return self.types

    def getControlsParameters( self ):
        return self.controlsParameters

    def getSourcesParameters( self ):
        return self.sourcesParameters

    def getFxsParameters( self ):
        return self.fxsParameters

    def getOutputParameters( self ):
        return self.outputParameters

    def setType( self, pos, value ):
        self.types[pos] = value
    
    def setControlParameter( self, pos, value ):
        self.controlsParameters[pos] = value

    def setSourceParameter( self, pos, value ):
        self.sourcesParameters[pos] = value

    def setFxParameter( self, pos, value ):
        self.fxsParameters[pos] = value   
         
    def setOutputParameter( self, pos, value ):
        self.outputParameters[pos] = value  
