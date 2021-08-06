#!/usr/bin/python

"""
./mailman_archive.py http://lists.alioth.debian.org/pipermail/dbconfig-common-devel/

download mailman gziped archives and combine single uncompressed file 

"""


import HTMLParser


class HrefHTMLParser(HTMLParser.HTMLParser):

    def __init__(self):
        HTMLParser.HTMLParser.__init__(self)
        self.hrefs = []

    def handle_starttag(self, tag, attrs):
        if tag == "a":
            attrs_dict = dict(attrs)
            if attrs_dict.get("href", "").endswith(".txt.gz"):
                self.hrefs.append(attrs_dict["href"])
            

sample_html = """\
<HTML>
  <HEAD>
     <title>Title</title>
     <META NAME="robots" CONTENT="noindex,follow">
     <META http-equiv="Content-Type" content="text/html; charset=us-ascii">
  </HEAD>
  <BODY BGCOLOR="#ffffff">
     <h1>Title </h1>

     <p>
      You can get <a href="">more information about this list</a>.
     </p>
     
         <table border=3>
          <tr><td>Archive</td>
          <td>View by:</td>
          <td>Downloadable version</td></tr>

     
        <tr>
            <td>June 2008:</td>
            <td>
              <A href="2008-June/thread.html">[ Thread ]</a>
              <A href="2008-June/subject.html">[ Subject ]</a>
              <A href="2008-June/author.html">[ Author ]</a>

              <A href="2008-June/date.html">[ Date ]</a>
            </td>
            <td><A href="2008-June.txt.gz">[ Gzip'd Text 575 bytes ]</a></td>
            </tr>
              </table>
     </BODY>
     </HTML>
"""


import os
import zlib
import urllib2
import shutil
import sys
import gzip


class DecompressFile(object):
    
    def __init__(self, file_):
        self.file = file_
        self.decompressor = zlib.decompressobj()
    
    def write(self, s):
        self.file.write(self.decompressor.decompress(s))


def main():
    index_url = sys.argv[1]
    tempdir = os.path.splitext(os.path.basename(__file__))[0]
    index_url = index_url.rstrip("/") + "/"
    index_html = urllib2.urlopen(index_url).read()
    p = HrefHTMLParser()
    p.feed(index_html)
    archive = open(os.path.join(tempdir, "archive"), "w")
    for href in p.hrefs:
        print "downloading %s" % href
        src = urllib2.urlopen(index_url + href)
        zip_filepath = os.path.join(tempdir, href)
        shutil.copyfileobj(src, open(zip_filepath, "w"))
        shutil.copyfileobj(gzip.open(zip_filepath), archive)
#        shutil.copyfileobj(gzip.GzipFile(fileobj=src, mode="r"), 
#            open(os.path.join(tempdir, os.path.splitext(href)[0]), "w"))
#        break
    archive.close()
        
        # unzip
    # combine
    
if __name__ == "__main__":
    main()

