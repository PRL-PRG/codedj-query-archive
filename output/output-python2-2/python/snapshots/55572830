import shelve

class EdSy:
    def __init__(self):
        self.types =[] 
        self.controls = []
        self.sources = []
        self.fxs = []
        self.envelope = []
        self.locations = []
        self.connections = []
        self.duration = 0.

    def load(self, file):
        self.fullPath = "/home/olpc/tamtam/Resources/SynthFiles/" + file
        f = shelve.open( self.fullPath, 'r')
        self.loadState(f)
        f.close()

    def process(self):
        for i in range(len(self.locations)):
            if i < 4:
                if self.locations[i][1] >= 700:
                    self.locations[i] = [450,710]
                else:
                    self.locations[i][0] -= 150
            elif i < 8:
                if self.locations[i][1] >= 700:
                    self.locations[i] = [225,710]
                else:
                    self.locations[i][0] -= 150
            elif i < 12:
                if self.locations[i][1] >= 700:
                    self.locations[i] = [675,710]
                else:
                    self.locations[i][0] -= 150
            elif i == 12:
                self.locations[i] = [450,625]

    def save(self):
        f = shelve.open(self.fullPath, 'n')
        self.saveState(f)
        f.close()

    def saveState( self, state ):
        state['types'] = self.types
        state['controls'] = self.controls
        state['sources'] = self.sources
        state['fxs'] = self.fxs
        state['envelope'] = self.envelope
        state['locations'] = self.locations
        state['connections'] = self.connections
        state['duration'] = self.duration

    def loadState( self, state ):
        self.types = state['types']
        self.controls = state['controls']
        self.sources = state['sources']
        self.fxs = state['fxs']
        self.envelope = state['envelope']
        self.locations = state['locations']
        self.connections = state['connections']
        self.duration = state['duration']

        print "types: ", self.types
        print "controls: ", self.controls
        print "sources: ", self.sources
        print "fxs: ", self.fxs
        print "envelope: ", self.envelope
        print "locations: ", self.locations
        print "connections: ", self.connections
        print "duration: ", self.duration
