import random

class PitchReverse:
    def __init__( self ):
        self.pitchList = []

    def reorderPitch( self, notesList ):
        self.extractOneValue(notesList)
        self.pitchList.reverse()
        for i in range(len(notesList)):
            notesList[i].pitch = self.pitchList[i]

        return notesList

    def extractOneValue( self, notesList ):
        self.pitchList = []
        for note in notesList:
            self.pitchList.append(note.pitch)

class PitchSort( PitchReverse ):
    def __init__( self ):
        PitchReverse.__init__( self )

    def reorderPitch( self, notesList ):
        PitchReverse.extractOneValue(self, notesList)
        self.pitchList.sort()
        for i in range(len(notesList)):
            notesList[i].pitch = self.pitchList[i]

        return notesList

class PitchShuffle( PitchReverse ):
    def __init__( self ):
        PitchReverse.__init__ ( self )

    def reorderPitch( self, notesList ):
        PitchReverse.extractOneValue(self, notesList)
        self.pitchList = random.sample(self.pitchList, len(self.pitchList))
        for i in range(len(notesList)):
            notesList[i].pitch = self.pitchList[i]

        return notesList

class PitchMarkov:
    def __init__( self ):
        self.originalList = []

    def getNewList( self, notesList, order=1 ):
        self.playedNotes = []
        self.extractOneValue( notesList, order )
        self.playedNotes = self.originalList[:order]
        
        for i in range(len(self.originalList) - order):
            self.playedNotes.append(self.pickupNewValue(order))

        for i in range(len(notesList)):
            notesList[i].pitch = self.playedNotes[i]

        return notesList

    def extractOneValue( self, notesList, order ):
        self.originalList = []
        for note in notesList:
            self.originalList.append(note.pitch)
        for i in range(order):
            self.originalList.append(self.originalList[i])

    def pickupNewValue( self, order ):
        condition = False
        self.probTable = []
        for ilist in range(len(self.originalList) - order):         
            for iord in range(order):
                if self.playedNotes[len(self.playedNotes) - (iord + 1)] != self.originalList[(order - 1) + ilist - iord]:
                    condition = False
                    break
                else:
                    condition = True

            if condition == True:
                self.probTable.append(self.originalList[ilist + order])

        return self.probTable[random.randint(0, (len(self.probTable) - 1))]
