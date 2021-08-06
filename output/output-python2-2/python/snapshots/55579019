from Framework.CSound.CSoundClient import CSoundClient
from Framework.NoteLooper import NoteLooper as NL

import time
import math

nl = NL( 0.5, 18.0, ['drum1kit'], [1.0], [1.0] )

nl.notes = [
        (0, 
            {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'drum1kick', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.92029484351363555, 'pitch': 24, 'duration': 1, 'noteID': 36, 'pan': 0.5}, ''),
        ( 0, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'flute', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.92525578482634807, 'pitch': 29, 'duration': 4, 'noteID': 1, 'pan': 0.5}, ''),
        (0, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'koto', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.939434987358156, 'pitch': 38, 'duration': 5, 'noteID': 8, 'pan': 0.5}, ''),
        (0, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'gam', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.94494314329224738, 'pitch': 24, 'duration': 5, 'noteID': 22, 'pan': 0.5}, ''),
        (0, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'guit', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.94691792606223446, 'pitch': 35, 'duration': 5, 'noteID': 29, 'pan': 0.5}, ''),
        (0, {'fullDuration': True, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'gam', 'filterType': 0, 'tied': False, 'onset': 0, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.98273554224748583, 'pitch': 33, 'duration': 6, 'noteID': 15, 'pan': 0.5}, ''),
        (6, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'gam', 'filterType': 0, 'tied': False, 'onset': 6, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.65465607337722664, 'pitch': 26, 'duration': 7, 'noteID': 23, 'pan': 0.5}, ''),
        (6, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'gam', 'filterType': 0, 'tied': False, 'onset': 6, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.66382312969855872, 'pitch': 28, 'duration': 5, 'noteID': 16, 'pan': 0.5}, ''),
        (6, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'koto', 'filterType': 0, 'tied': False, 'onset': 6, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.69112283960585841, 'pitch': 33, 'duration': 5, 'noteID': 9, 'pan': 0.5}, ''),
        (6, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'drum1hatshoulder', 'filterType': 0, 'tied': False, 'onset': 6, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.70739520974639736, 'pitch': 46, 'duration': 1, 'noteID': 46, 'pan': 0.5}, ''),
        (6, {'fullDuration': False, 'pageID': 0, 'decay': 0.098000000000000004, 'trackID': 0, 'instrumentFlag': 'flute', 'filterType': 0, 'tied': False, 'onset': 6, 'filterCutoff': 1000, 'attack': 0.002, 'reverbSend': 0.10000000000000001, 'overlap': False, 'amplitude': 0.72961742679087493, 'pitch': 31, 'duration': 5, 'noteID': 2, 'pan': 0.5}, '')
        ]
nl.setDuration(12)
nl.setTick(0)


cmd = [
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5019 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5020 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5021 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5018 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5017 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5011 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5012 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5013 0.002 0.004 0 1000.000000')",
    "perf.InputMessage('i 5003.5 0.0 0.050000 1.000000 0.100000 0.773 0.500000 5012 0.002 0.004 0 1000.000000')"
    ]

CSoundClient.initialize()
CSoundClient.setMasterVolume(100.0)



i = 0
t1 = time.time()
m = v = vv = 0.0


while True : 

    t0 = t1
    i = i + 1
    time.sleep(0.3)
    if False:
        next = nl.next()
        for n in next:
            CSoundClient.sendText(n)
    else: 
        j = i % 3
        if j == 0:
            CSoundClient.sendText( cmd[0] )
            CSoundClient.sendText( cmd[1] )
            CSoundClient.sendText( cmd[2] )
            CSoundClient.sendText( cmd[3] )
        elif j == 1:
            CSoundClient.sendText( cmd[4] )
            CSoundClient.sendText( cmd[5] )
            CSoundClient.sendText( cmd[6] )
            CSoundClient.sendText( cmd[7] )
        else:
            CSoundClient.sendText( cmd[8] )

    t1 = time.time()

    r = 1.0 / i
    d = t1 - t0
    m = r * d + (1.0 - r) * m
    v = r * d * d + (1.0 - r) * v
    vv = r * d * d * d + (1.0 - r) * v

    #print m, math.sqrt(v - m * m ), math.pow( vv - m*m*m, 0.333333)

