# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Utilities, dealing mostly with strings.
"""
from __future__ import division, absolute_import, with_statement
import re
from functools import wraps
from .nexfs import NexFS
__all__ = (
	'BLACK', 'RED', 'GREEN', 'YELLOW', 'BLUE', 'CYAN', 'MAGENTA', 'WHITE', 
	'GREY_TRANS', 'GREY_SOLID', 'DEFAULT_COLOR',
	'colors', 'color_escape', 'stripcolors', 'Quoted', 'quote', 'parse', 
	'nexdata', 'parseconfig', 'complexdecorator', 'callbyline')

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

def quote(text, say=False):
	"""quote(string, [bool]) -> string
	Performs escaping so that text is parsed as a single argument with no 
	variable substitution.
	
	If say is True, use say/echo/tell escaping rules.
	"""
	# Quote $ as $$
	# "" does token grouping (several words as one argument)
	# \" causes the string to not end
	# \\ is similar to \"
	# Commands which use Cmd_Args() instead of Cmd_Argv() do not follow 
	# standard parsing rules.
	
	text = text.replace('$', '$$') # Escape vars
	if not say:
		if ' ' in text:
			# Don't quote unless we have to
			text = '"'+text.replace('\\', '\\\\').replace('"', '\\"')+'"'
	return text

def parse(text):
	"""parse(string) -> [string, ...]
	Parses a string the same way that DarkPlaces does, except for variables.
	"""
	#FIXME: Handling escaping
	text = text.strip()
	bits = text.split('"') # A trick I learned to parse quotes
	parts = []
	for i, bit in enumerate(bits):
		if i % 2 == 0:
			# No quotes
			if '//' in bit: # Comment found
				# Do that here because it doesn't count inside quoted strings
				bit = bit.split('//', 1)[0]
				parts += bit.split()
				break
			parts += bit.split()
		else:
			# Quoted
			parts.append(bit)
	return parts

filesystem = NexFS()
def parseconfig(fn):
	"""parseconfig(string) -> generator: [string, ...]
	Yields each command of the given file, recursing as necessary.
	"""
	fs = filesystem
	print 'parseconfig %r' % fn
	with fs.open(fn, 'rU') as config:
		for line in config:
			parts = parse(line)
			if len(parts) >= 2 and parts[0] == 'exec':
				try:
					pc = parseconfig(fs.join(fs.dirname(fn), parts[1]))
				except IOError, OSError:
					import traceback
					traceback.print_exc()
				else:
					for c in pc:
						yield c
			else:
				yield parts

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

def complexdecorator(dec):
	"""
	Simplifies writting a decorator with arguments.
	
	def mydec(...):
		process1
		def _(func):
			process2
			return func
		return _
	
	Becomes
	
	@complexdecorator
	def mydec(...):
		process1
		func = yield
		process2
		yield func
	"""
	@wraps(dec)
	def wrapper(*pargs, **kwargs):
		gen = dec(*pargs, **kwargs)
		gen.next() # Move to the first yield
		def realdec(func):
			rv = gen.send(func) # return from first yield and move to second
			gen.close() #clean-up properly and immediately
			return rv
		return realdec
	return wrapper

def callbyline(meth):
	"""callbyline(callable) -> callable
	
	Wraps up a single-argument function so that it is called on each line 
	(saving the tail for the next call). Use as decorator.
	"""
	meth.text_tail = ""
	@wraps(meth)
	def wrapper(text, *pargs):
		self = None
		# Auto-detect method
		if len(pargs) > 1:
			raise TypeError, "One argument at most for methods."
		elif len(pargs) == 1:
			self, text = text, pargs[0]
		
		text = meth.text_tail + text
		lines = text.split('\n')
		for line in lines[:-1]:
			if self is None:
				rv = meth(line)
			else:
				rv = meth(self, line)
		meth.text_tail = lines[-1]
		return rv
	return wrapper
