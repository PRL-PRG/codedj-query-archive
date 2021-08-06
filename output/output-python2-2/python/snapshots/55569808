
import time

class TaskProfiler( object ):
    def __init__( self ):
        self.profiles = {}

    def ProfileBegin( self, profile ):
        if self.profiles.has_key(profile) == False:    self.profiles[profile] = TaskProfile(profile)
        self.profiles[profile].begin()
        
    def ProfilePause( self, profile ):
        if self.profiles.has_key(profile) == False: return False
        self.profiles[profile].pause()

    def ProfileEnd( self, profile ):
        if self.profiles.has_key(profile) == False: return False
        self.profiles[profile].end()

    def Profile(self, profile): 
        if profile in self.profiles:
            if self.profiles[profile].inProgress:
                self.profiles[profile].end()
            else:
                self.profiles[profile].begin()
        else:  
            self.profiles[profile] = TaskProfile(profile)
            self.profiles[profile].begin()

    def ProfilePrint( self, profile ):
        if self.profiles.has_key(profile) == False: return "Couldn't find profile: " + profile
        return self.profiles[profile].printoverall()

    def ProfileEndAndPrint( self, profile ):
        if self.profiles.has_key(profile) == False: return "Couldn't find profile: " + profile
        self.profiles[profile].end()
        return self.profiles[profile].printlast()
                

    def PrintAll( self ):
        str = ""
        keys = self.profiles.keys()
        keys.sort()
        return "\n".join( [self.profiles[k].printoverall() for k in keys] )
        #for p in self.profiles:
            #str += "\n" + self.profiles[p].printoverall()
        #return str


class TaskProfile( object ):
    def __init__( self, name ):
        self.name = name
        self.count = 0
        self.min = 666666666.0     # really high
        self.max = -1.0            # pretty low
        self.avg = 0.0
        self.dt = 0
        self.inProgress = False
        self.paused = False

    def begin( self ):
        if self.inProgress: return False # you fucked up your ProfileBegin and ProfileEnd pairs
        self.inProgress = True
        if self.paused:
            self.paused = False
            self.startTime += time.time() - self.pauseTime
        else:
            self.startTime = time.time()
        
    def pause( self ):
        self.pauseTime = time.time()
        self.inProgress = False
        self.paused = True
        
    def end( self ):
        self.dt = time.time() - self.startTime
        if self.paused:
            self.dt = self.pauseTime - self.startTime
        if self.dt > self.max: self.max = self.dt
        if self.dt < self.min: self.min = self.dt
        self.avg = (self.dt + self.avg*self.count)/(self.count+1)
        self.count += 1
        self.inProgress = False
        self.paused = False

    def printlast( self ):
        return "Profile: " + self.name + " last dt: %f" % (self.dt)

    def printoverall( self ):
        if self.count == 0: return "Profile: " + self.name + ", no data!"
        return "Profile: " + self.name + " avg: %f max: %f min: %f count: %d" % (self.avg, self.max, self.min, self.count)

TP = TaskProfiler()
