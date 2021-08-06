#!/usr/bin/env python

import re, operator, datetime
today = datetime.datetime.today()

class seminar:
    pass

class src(file):
    def line(self):
        return self.next().rstrip()
    def block(self):
        t = ''; empty = False
        for l in self:
            if not l.strip():
                if empty: break
                else: empty = True
            else:
                empty = False
            t += l
        return t.strip()

def parse(fp):
    from dateutil import parser as datetime
    
    while True:
        try:
            s = seminar()
            
            s.date, s.place = re.match('(.*)\s+in\s+(.*)', fp.line()).groups()
            s.prefix, s.speaker, s.affiliation = re.match('(Mr\.\s+|Dr\.\s+|Prof\.\s+|)(.*)\s+\((.*)\)', fp.line()).groups()
            s.title = fp.line().strip('"')
            s.abstract = fp.block()
            
            s.date = datetime.parse(s.date)
            
            yield s
        except: break

all = list(parse(src('seminars.txt')))
all.sort(key=operator.attrgetter('date'))

future = filter(lambda s: s.date >= today, all)
past = filter(lambda s: s.date < today, all)
past.reverse()

next = future[0:1]
last = past[0:1]

def on(date): return filter(lambda s: s.date.strftime('%F') == date, all)
