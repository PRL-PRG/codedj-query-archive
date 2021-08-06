#!/usr/bin/env python

import re, operator

class preprint:
    pass

class src(file):
    def skip(self):
        for l in self:
            if l.strip(): break
        return l.rstrip()

def parse(fp):
    while True:
        try:
            p = preprint()
            
            p.id, p.arxiv = re.match('(.*)\s+\[(.*)\]', fp.skip()).groups()
            p.year = re.match('\w+-(\d+)-\d+', p.id).group(1)
            
            p.authors = fp.skip()
            p.authors = re.sub(r'((\w+\.?[~\s]+)*(Frolov|Pogosian|Zhao|Berndsen|Gooding))', r'<B>\1</B>', p.authors)
            p.authors = p.authors.replace('~', '&nbsp;')
            
            p.title = fp.skip().strip('"')
            
            yield p
        except: break

all = list(parse(src('preprints.txt')))
all.sort(key=operator.attrgetter('id'))
all.reverse()
