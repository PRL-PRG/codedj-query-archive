import common.Config as Config
from Generation.GenerationConstants import GenerationConstants

def gen():
    punch_list = [[], ]
    low_list = [[], ]
    mid_list = [[], ]
    high_list = [[], ]

    f = open('/home/olpc/tamtam/Generation/drumTablesList', 'w')
    g = open('/home/olpc/tamtam/Generation/drumStraightTables', 'w')

    # gen punch list
    beatsList = [[], ]
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        accents = []
        for j in GenerationConstants.PUNCH_ACCENTS[beatsPerPage]:
            accents.append(j * Config.TICKS_PER_BEAT)
        beatsList.append(accents)
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.PUNCH_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        punch_list.append(downBeats)

    string = '    DRUM_PUNCH_PROB = ' + str(punch_list) + '\n'
    f.write(string)
    string = '    DRUM_PUNCH_ACCENT = ' + str(beatsList) + '\n'
    g.write(string)

    # gen low list
    beatsList = [[], ]
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        accents = []
        for j in GenerationConstants.LOW_ACCENTS[beatsPerPage]:
            accents.append(j * Config.TICKS_PER_BEAT)
        beatsList.append(accents)
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.LOW_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        low_list.append(downBeats)

    string = '    DRUM_LOW_PROB = ' + str(low_list) + '\n'
    f.write(string)
    string = '    DRUM_LOW_ACCENT = ' + str(beatsList) + '\n'
    g.write(string)

    # gen mid list
    beatsList = [[], ]
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        accents = []
        for j in GenerationConstants.MID_ACCENTS[beatsPerPage]:
            accents.append(j * Config.TICKS_PER_BEAT / 2)
        beatsList.append(accents)
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
            beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.MID_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        mid_list.append(downBeats)
    
    string = '    DRUM_MID_PROB = ' + str(mid_list) + '\n'
    f.write(string)
    string = '    DRUM_MID_ACCENT = ' + str(beatsList) + '\n'
    g.write(string)

    # gen high list
    beatsList = [[], ]
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        accents = []
        for j in GenerationConstants.HIGH_ACCENTS[beatsPerPage]:
            accents.append(j * Config.TICKS_PER_BEAT / 2)
        beatsList.append(accents)
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
            beats.append( beat * Config.TICKS_PER_BEAT + ( Config.TICKS_PER_BEAT / 2 ) )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.HIGH_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        high_list.append(downBeats)

    string = '    DRUM_HIGH_PROB = ' + str(high_list) + '\n'
    f.write(string)
    string = '    DRUM_HIGH_ACCENT = ' + str(beatsList) + '\n'
    g.write(string)

    f.close()
    g.close()

