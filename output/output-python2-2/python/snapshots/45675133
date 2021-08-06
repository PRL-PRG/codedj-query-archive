#!/usr/bin/python2.2
#
# Translate ply files into LaTeX.

from __future__ import generators
import sys,string,re

preamble = r'''\documentclass[letterpaper]{article}
\usepackage{newplay}
\begin{document}
'''

class plywood(object):

  def __init__(self, filename):
    self.filename=filename
    self.newfile = string.join((filename[:filename.rfind(".ply")],"tex"),".")
    self.infile=open(self.filename,'r')
    self.outfile=open(self.newfile,'w')
    
  def segments(infile):
    accum=""
    fileg=infile.__iter__()
    for line in fileg:
      sline=string.strip(line)
      if sline=='':
        yield accum
        accum=""
        while sline=='':
          sline=string.strip(fileg.next())
      accum="%s\n%s"%(accum,sline)
    yield accum
  segments = staticmethod(segments)

  
  title_re=re.compile(r'\s*title:\s*(.*\S)\s*$',re.MULTILINE)
  author_re=re.compile(r'\s*author:\s*(.*\S)\s*$',re.MULTILINE)
  act_re=re.compile(r'act:\s*(.*\S)\s*$',re.MULTILINE|re.IGNORECASE)
  scene_re=re.compile(r'scene:\s*(.*\S)\s*$',re.MULTILINE|re.IGNORECASE)
  double_re=re.compile(r'"([^"]*)"',re.MULTILINE)
  dots_re=re.compile(r'\.\s*\.\s*\.')
  it_re=re.compile(r'/([^/]+)/')
  bf_re=re.compile(r'\*\*+([^\*]+)\*\*+')
  long_re=re.compile(r'^\s*\[\s*([^\]]+\S)\s*\]\s*$')
  direction_re=re.compile(r'\s*[^{]\[\s*([^\]]+\S)\s*\]\s*')
  line_re=re.compile(r'^\s*([^=]+\S)\s*=\s*(.*\S)\s*',re.MULTILINE|re.DOTALL)

  def do_title(self,match):
    self.title=match.group(1)
    return r'\title{%s}' % self.title
  
  def replaces(self, line):
      line = self.title_re.sub(self.do_title,line)
      line = self.author_re.sub(r'\\author{\1}',line)
      line = self.act_re.sub(r'\section{%s Act \1}' % self.title,line)
      line = self.scene_re.sub(r'\subsection{Scene \1}',line)
      line = self.double_re.sub(r'``\1"',line)
      line = self.dots_re.sub(r'\dots',line)
      line = self.it_re.sub(r'\\textit{\1}',line)
      line = self.bf_re.sub(r'\\textbf{\1}',line)
      line = self.long_re.sub(r'\longdirection{(\1)}',line)
      line = self.direction_re.sub(r' \direction{(\1)}',line)
      line = self.line_re.sub(r'\line{\1}{\2}',line)
      return line
    

  def process(self):

    print "Generating %s from %s" % (self.newfile, self.filename)
    self.outfile.write(preamble)
    
    for line in self.segments(self.infile):
      self.outfile.write("%s\n" % self.replaces(line))
    self.outfile.write("%s\n" % r'\end{document}')

  def close(self):
    self.infile.close()
    self.outfile.close()
    
if __name__=="__main__":
  ply=plywood(sys.argv[1])
  ply.process()
  ply.close()
  
