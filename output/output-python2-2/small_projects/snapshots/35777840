# This module was used to generated parse the input files included in
# python-dev-parsed-text.tar.bz2, but it shouldn't need to be run under
# routine circumstances.

import os, os.path, re, shutil

MONTHS = {
    'January':    1,
    'February':   2,
    'March':      3,
    'April':      4,
    'May':        5,
    'June':       6,
    'July':       7,
    'August':     8,
    'September':  9,
    'October':   10,
    'November':  11,
    'December':  12
  }

rawTextFilenamePat = re.compile(r'(?P<year>\d{4})-(?P<month>%s).txt' % '|'.join(MONTHS.keys()))

# Sample input for msgHeaderPat:
#   'From skip@mojam.com (Skip Montanaro)  Wed Apr 21 14:17:32 1999'
msgHeaderPat = re.compile(
    r'^(From\s+[\w\-][\w\-\.]+@[\w\-][\w\-\.]+[a-zA-Z]{1,4}'
    r'(?:\s+\(.*?\))?'
    r'\s+(?:Sun|Mon|Tue|Wed|Thu|Fri|Sat).+)$',
    re.MULTILINE
  )


def parseRawText(rawTextRoot, outputRoot):
    for d in rawTextRoot, outputRoot:
        if not os.path.isdir(d):
            raise IOError('Not a directory: "%s"' % d)

    rawTextFiles = os.listdir(rawTextRoot)

    for rawFN in rawTextFiles:
        m = rawTextFilenamePat.match(rawFN)
        year, month = m.group('year'), str(MONTHS[m.group('month')]).zfill(2)
        print year, month
        outputDirThisMonth = os.path.join(outputRoot, year, month)
        if os.path.exists(outputDirThisMonth):
            shutil.rmtree(outputDirThisMonth)
        os.makedirs(outputDirThisMonth)

        rawFNFull = os.path.join(rawTextRoot, rawFN)
        rawText = file(rawFNFull, 'rb').read()

        msgs = [m.strip() for m in msgHeaderPat.split(rawText)]
        msgs = [m for m in msgs if m.startswith('From:')]

        for i, m in enumerate(msgs):
            file(os.path.join(outputDirThisMonth, str(i).zfill(6) + '.txt'), 'wb').write(m)


def main():
    rawTextRoot = r'D:\tmp\python-dev-raw-text'
    outputRoot = r'D:\tmp\python-dev-parsed-text'
    parseRawText(rawTextRoot, outputRoot)
