# Copyright (C) 2002 by Monty Taylor
# Translate ply files into LaTeX.

#     This program is free software; you can redistribute it and/or modify
#     it under the terms of the GNU General Public License as published by
#     the Free Software Foundation; either version 2 of the License, or
#     (at your option) any later version.

#     This program is distributed in the hope that it will be useful,
#     but WITHOUT ANY WARRANTY; without even the implied warranty of
#     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#     GNU General Public License for more details.

#     You should have received a copy of the GNU General Public License
#     along with this program; if not, write to the Free Software
#     Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA


from __future__ import generators
import sys,string,re,os

preamble = r'''\documentclass[letterpaper]{article}
\usepackage{plywood}
\begin{document}
'''

class Plywood(object):

  def __init__(self, filename):
    self.filename=filename
    self.basename = filename[:filename.rfind(".ply")]
    dirindex=filename.rfind(os.sep)
    self.dir='.'
    if dirindex!=-1:
      self.dir= filename[:filename.rfind(os.sep)]
      os.chdir(self.dir)
    self.newfile = string.join((self.basename,"tex"),".")
    self.infile=open(self.filename,'r')
    self.outfile=open(self.newfile,'w')
    self.do_line = self._do_line
    self.chars={'Very unlikely character name': 1}
    self.make_charlist_re()
    self.type=''
    self.act_count=0
    self.scene_count=0

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

  
  amp_re=re.compile(r'\&')
  beat_re=re.compile(r'^-beat-',re.MULTILINE)
  song_re=re.compile(r'\s*song:\s*(.*\S)\s*$',re.MULTILINE)
  chars_re=re.compile(r'\s*characters?:\s*(.*)',re.MULTILINE|re.DOTALL)
  type_re=re.compile(r'\s*type:\s*(.*\S)\s*$',re.MULTILINE)
  title_re=re.compile(r'\s*title:\s*(.*\S)\s*$',re.MULTILINE)
  author_re=re.compile(r'\s*author:\s*(.*\S)\s*$',re.MULTILINE)
  setting_re=re.compile(r'\s*(setting|at rise):\s*(.*)\s*$',re.MULTILINE|re.DOTALL|re.IGNORECASE)
  new_act_scene_re=re.compile(r'^\s*new(act|scene):.*$',re.MULTILINE)
  act_re=re.compile(r'act:\s*(.*\S)\s*$',re.MULTILINE|re.IGNORECASE)
  scene_re=re.compile(r'scene:\s*(.*\S)\s*$',re.MULTILINE|re.IGNORECASE)
  lyric_re=re.compile(r'(lyric|line)s?:\s*$',re.MULTILINE|re.IGNORECASE)
  yelling_re=re.compile(r'!!!\s*(.*)(\s!!!|$)',re.MULTILINE|re.DOTALL)
  double_re=re.compile(r'"([^"]*)"',re.MULTILINE)
  dots_re=re.compile(r'\.\s*\.\s*\.')
  it_re=re.compile(r'/([^/]+)/')
  bf_re=re.compile(r'\*\*+([^\*]+)\*\*+')
  long_re=re.compile(r'^\s*\[\s*([^\]]+\S)\s*\]\s*$')
  direction_re=re.compile(r'\s*[^{]\[\s*([^\]]+\S)\s*\]\s*')
  line_re=re.compile(r'^\s*([^=]+\S)\s*=\s*(.*\S)\s*',re.MULTILINE|re.DOTALL)
  line_para_re=re.compile(r'\n',re.MULTILINE)
  char_entry_re=re.compile(r'^\s*([^-\(]+)\s*(\([^\)]+\))?\s*-\s*(.*)',re.MULTILINE)
  char_direction_re=re.compile(r'direction\{(.*)\}',re.MULTILINE|re.DOTALL)

  def do_title(self,match):
    self.title=match.group(1)
    #return r'\title{%s}' % self.title
    return ''

  def do_author(self,match):
    self.author=match.group(1)
    return ''

  def do_type(self,match):
    self.type=match.group(1)
    return ''
 
  def _do_line(self,match):
    return r'\line{%s}{%s}' % (match.group(1),match.group(2))

  def _do_lyric(self,match):
    line=match.group(2)
    line = self.line_para_re.sub(r'\\par\n',line)
    return r'\lyric{%s}{%s}' % (match.group(1),line)

  
  def flip_lyric(self,match):
    if match.group(1) == 'line':
      self.do_line = self._do_line
    else: 
      self.do_line = self._do_lyric
    return ''

  def do_char(self,match):
    (name,nick,desc) = [ match.group(f) for f in range(1,4) ]
    desc = self.amp_re.sub(r'\&',desc)
    self.chars[string.strip(name)] = 1
    if nick is not None:
      self.chars[nick[1:-1]] = 1
    return r'\textsc{%s} & %s \vspace{12pt}\\%s' % (name, desc, "\n\n")
 
  def make_type(self):
    if self.type == '': # We need one
      acts='acts'
      type_map=['no','one','two','three','four','five','six']
      if self.act_count==1:
        acts='act'
      if self.act_count > len(type_map):
        type_map[self.act_count]='many'
      return 'a play in %s %s' % (type_map[self.act_count],acts)
    return self.type
    
  def make_charlist_re(self):
    charlist = string.join(self.chars.keys(),'|')
    self.charlist_re = re.compile(r'(%s)'%charlist,re.MULTILINE)

  def do_new_act(self,match):
    act_scene=match.group(1)
    if act_scene == 'act':
      self.act_count=self.act_count+1
    else:
      self.scene_count=self.scene_count+1
    return  r'\new%s%s' % (act_scene,"\n")

  def do_chars(self,match):
    char_section=match.group(1)
    char_section=self.char_entry_re.sub(self.do_char,char_section)
    self.make_charlist_re()
    return r'\begin{CharList}%s\end{CharList}' % char_section

  def find_chars(self,match):
    dir = self.charlist_re.sub(r'\\textrm{\\textsc{\1}}',match.group(1))
    return r'direction{%s}'%dir

  def replaces(self, line):
      line = self.chars_re.sub(self.do_chars,line)
      line = self.lyric_re.sub(self.flip_lyric,line)
      line = self.title_re.sub(self.do_title,line)
      line = self.type_re.sub(self.do_type,line)
      line = self.beat_re.sub(r'\\beat ',line)
      line = self.song_re.sub(r'\subsection{\1}',line)
      line = self.author_re.sub(self.do_author,line)
      line = self.act_re.sub(r'\section{%s Act \1}' % self.title,line)
      line = self.new_act_scene_re.sub(self.do_new_act,line)
      line = self.scene_re.sub(r'\subsection{Scene \1}',line)
      line = self.double_re.sub(r'``\1"',line)
      line = self.dots_re.sub(r'\dots',line)
      line = self.it_re.sub(r'\\textit{\1}',line)
      line = self.bf_re.sub(r'\\textbf{\1}',line)
      line = self.yelling_re.sub(r'\\textsc{\1}',line)
      line = self.long_re.sub(r'\longdirection{(\1)}',line)
      line = self.direction_re.sub(r' \direction{(\1)}',line)
      line = self.setting_re.sub(r' \stagedirection{\1:}{\2}',line)
      line = self.char_direction_re.sub(self.find_chars,line)
      line = self.line_re.sub(self.do_line,line)
      return line
    

  def process(self):

    lines=[]
    print "Generating %s from %s" % (self.newfile, self.filename)
    self.outfile.write(preamble)
    for line in self.segments(self.infile):
      lines.append("%s\n" % self.replaces(line))
    type=self.make_type()
    self.outfile.write(r'\title{%s}%s\author{%s}%s\playtitlepage{%s}%s' % (self.title, "\n",self.author, "\n",type,"\n"))
    for line in lines:
      self.outfile.write(line)
    self.outfile.write(r'\end{document}%s' % ( "\n"))

  def makedvi(self):
    print "Running LaTeX on %s" % (self.newfile)
    os.system('latex %s' % (self.newfile))

  def makepdf(self):
    dvi=string.join((self.basename,'dvi'),'.')
    pdf=string.join((self.basename,'pdf'),'.')
    print "Creating %s" % (pdf)
    os.system('dvipdfm -o %s %s' % (pdf,dvi))
    
  def close(self):
    self.infile.close()
    self.outfile.close()
    
if __name__=="__main__":
  ply=Plywood(sys.argv[1])
  ply.process()
  ply.close()
  ply.makedvi()
  ply.makepdf()

  
