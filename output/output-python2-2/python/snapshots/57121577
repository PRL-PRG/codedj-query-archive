#!/usr/bin/env python

import htmllib, formatter, urlparse

def links(file):
    try:
        p = htmllib.HTMLParser(formatter.NullFormatter())
        p.feed(open(file).read()); p.close()
        
        return filter(lambda x: ':' not in x, p.anchorlist)
    except:
        return []

def crawl(file, visited):
    if file.endswith('/') or file.endswith('.'):
        file = urlparse.urljoin(file, 'index.html')
    
    if file and file not in visited:
        visited.add(file)
        base = file[0:file.rfind('/')+1]
        
        for f in links(file):
            visited = crawl(urlparse.urljoin(base, f), visited)
    
    return visited

site = list(crawl('.', set())); site.sort()

for i in site: print i