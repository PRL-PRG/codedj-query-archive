#!/usr/bin/env python

import re, operator, datetime; from dateutil import parser as datetime

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
    while True:
        try:
            s = seminar()
            
            s.date, s.place = re.match('(.*)\s+in room\s+(.*)', fp.line()).groups()
            s.speaker, s.affiliation = re.match('(.*)\s+\((.*)\)', fp.line()).groups()
            s.title = fp.line().strip('"')
            s.abstract = fp.block()
            
            s.date = datetime.parse(s.date)
            
            yield s
        except: break


all = list(parse(src('seminars.txt')))
all.sort(key=operator.attrgetter('date'))
