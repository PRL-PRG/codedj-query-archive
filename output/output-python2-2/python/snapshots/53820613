# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Utilities, dealing mostly with strings.
"""
from __future__ import division, absolute_import, with_statement
__all__ = '',

BLACK, RED, GREEN, YELLOW, BLUE, CYAN, MAGENTA, WHITE, GREY_TRANS, GREY_SOLID =\
	range(10)
DEFAULT_COLOR = WHITE

def colors(colortext, *pargs, **kw):
	"""colors((int, string), ..., [noescape=bool]) -> string
	Takes a series of color+text pairs and formats text to match.
	
	Unless noescape is given, colors() will escape the text so that any 
	accidental color codes in text is not parsed.
	"""
	doescape = True
	if 'noescape' in kw and kw['noescape']:
		doescape = False
	if isinstance(colortext, int):
		# Flattened arguments, unflatten
		temp = (colortext,) + pargs
		if len(pargs) % 2 != 0:
			raise TypeError, \
				"colors needs an even number of arguments when flattened, received %i." % len(pargs)
		pargs = [temp[i:i+2] for i in xrange(0, len(temp), 2)]
	else:
		pargs = (colortext,) + pargs
	
	rv = []
	for color,text in pargs:
		if doescape: text = color_escape(text)
		rv.append("^%i%s" % (color,text))
	return ''.join(rv) + ('^%i' % DEFAULT_COLOR)

def color_escape(text):
	"""color_escape(string) -> string
	Escapes color codes in text.
	"""
	return text.replace('^', '^^')

_STRIPCOLORS_PATTERN = re.compile(r'\^[0-9^]')
def stripcolors(text):
	"""stripcolors(string) -> string
	Removes color codes from 
	"""
	return _STRIPCOLORS_PATTERN.sub((lambda m: '^' if m.group() == '^^' else ''), text)

class Quoted(object):
	"""
	Flags a piece of text as pre-quoted.
	"""
	_text = None
	def __init__(self, text, quoted=False):
		"""Quoted(string, [bool])
		Text is the text to flag. Quoted is a flag indicating if it should be 
		escaped and quoted. If False (the default), the text passed to quote().
		"""
		if not quoted:
			self._text = quote(text)
		else:
			self._text = text
	def __str__(self):
		return str(self._text)
	def __unicode__(self):
		return unicode(self._text)
	def __repr__(self):
		return "<%s text=%r>" % (type(self).__name__, self._text)

def quote(text):
	"""quote(string) -> string
	Performs escaping so that text is parsed as a single argument with no 
	variable substitution.
	"""
	# Quote $ as $$
	# "" does token grouping (several words as one argument)
	# \" causes the string to not end, but may or may not be substituted 
	# correctly.
	# \\ is similar, does the right structural thing but may not be parsed
	
	text = text.replace('$', '$$') # Escape vars
	if ' ' in text:
		# Don't quote unless we have to
		text = '"'+text.replace('\\', '\\\\').replace('"', '\\"')+'"'
	return text

"""
Parsing commands (from darkplaces/cmd.c:302)
# This is executed repeatedly
def parsebit():
	quotes = False
	for c in cmd_text:
		if c == '"':
			quotes = not quotes
		if c == '\\' and (c_next == '"' or c_next == '\\'):
			c_increment() # Different from continue
		if !quotes and c == ';':
			break
		if c in '\r\n':
			break
	line = text[:c_index]
	line = line.lstrip()
	if line.split(None, 1)[0] in ('alias', 'bind', 'in_bind'):
		line = preprocess(line)
	execute(line)
def preprocess(line):
	$$ -> $
	$var and ${var} -> value of var
	${var asis} -> value of var, which is further parsed
	${$var} -> value of $var
	argument processing ($*, $0-$9, $1-)
def execute(line):
	tokenization:
		// is a comment
		does quoted text as described above 
		Otherwise, tokens are split by whitespace
	Checks if qc handled it
	
"""
