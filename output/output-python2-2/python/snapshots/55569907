import random

import common.Config as Config

# remplacer position dans notesList par l'attribut de CSoundNote
class RythmShuffle:

    def getNewList( self, notesList, nbeats ):
        self.barLength = Config.TICKS_PER_BEAT * nbeats
        self.onsetDelta = 0
        self.newOnsetList = []
        self.oldDuration = []
        self.newDuration = []
        self.extractOnsetValue(notesList)

        self.newOnsetList = random.sample(range(len(self.originalList)), len(self.originalList))
        self.getOldDuration(notesList)
        self.getNewDuration(notesList)

        for i in range(len(notesList)):
            notesList[i].onset = self.onsetDelta
            notesList[i].duration = self.oldDuration[i] * self.newDuration[i]
            self.onsetDelta = self.onsetDelta + self.originalList[self.newOnsetList[i]]

        return notesList
    
    def extractOnsetValue( self, notesList ):
        self.originalList = []
        for note in notesList:
            self.originalList.append(note.onset)

        for i in range(len(self.originalList) -1):
            self.originalList[i] = self.originalList[i+1] - self.originalList[i]

        self.originalList[-1] = self.barLength - (self.originalList[-1] % self.barLength)

    def getOldDuration( self, notesList ):
        for i in range(len(notesList)):
            if (i+1) == len(notesList):
                self.oldDuration.append(notesList[i].duration / (self.barLength - (notesList[i].onset % self.barLength)))
            else:
                self.oldDuration.append(notesList[i].duration / (notesList[i+1].onset - notesList[i].onset))

    def getNewDuration( self, notesList ):
        for i in self.newOnsetList:
            if (i+1) == len(notesList):
                self.newDuration.append(self.barLength - (notesList[i].onset % self.barLength))
            else:
                self.newDuration.append(notesList[i+1].onset - notesList[i].onset)

class RythmReverse( RythmShuffle ):

    def getNewList( self, notesList, nbeats ):
        self.barLength = Config.TICKS_PER_BEAT * nbeats
        self.onsetDelta = 0
        self.newOnsetList = []
        self.oldDuration = []
        self.newDuration = []
        RythmShuffle.extractOnsetValue( self, notesList )

        for i in range( len( self.originalList ) ):
            self.newOnsetList.append( i )

        self.newOnsetList.reverse() 

        RythmShuffle.getOldDuration( self, notesList )
        RythmShuffle.getNewDuration( self, notesList )

        for i in range(len(notesList)):
            notesList[i].onset = self.onsetDelta
            notesList[i].duration = self.oldDuration[i] * self.newDuration[i]
            self.onsetDelta = self.onsetDelta + self.originalList[self.newOnsetList[i]]

        return notesList
