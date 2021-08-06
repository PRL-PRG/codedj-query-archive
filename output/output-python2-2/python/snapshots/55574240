import Config
from Generation.GenerationConstants import GenerationConstants

def gen():
    punch_list = [[], ]
    low_list = [[], ]
    mid_list = [[], ]
    high_list = [[], ]

    f = open('/home/olpc/tamtam/Generation/drumTablesList', 'w')

    # gen punch list
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.PUNCH_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        punch_list.append(downBeats)

    string = '    DRUM_PUNCH_PROB = ' + str(punch_list) + '\n'
    f.write(string)

    # gen low list
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
        beats = []
        downBeats = []
        for beat in range( beatsPerPage ):
            beats.append( beat * Config.TICKS_PER_BEAT )
        for i in range( len( beats ) ):
            downBeats.append( ( beats[ GenerationConstants.LOW_ACCENTS[ beatsPerPage ][ i ] ], int( pow( float( len( beats ) - i) / len( beats ), 1.5 ) * 100.) ) )
        low_list.append(downBeats)

    string = '    DRUM_LOW_PROB = ' + str(low_list) + '\n'
    f.write(string)

    # gen mid list
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
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

    # gen high list
    for beatsPerPage in [1,2,3,4,5,6,7,8,9,10,11,12]:
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

    f.close()

