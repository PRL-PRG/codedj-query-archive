import optparse, re, sys

# After the Usage string, multiple rSt formatted option blocks are expected
# indented by tabs or spaces
USAGE = re.compile(r'(?s)\s*[Uu]sage: (.*?)(\n\n[^\t ]|$)')
OPTGROUP = re.compile(r'\n[\t ]*-')

def nonzero(self): # will become the nonzero method of optparse.Values
	"True if options were given"
	for v in self.__dict__.itervalues():
		if v is not None: return True
	return False

optparse.Values.__nonzero__ = nonzero # dynamically fix optparse.Values

class ParsingError(Exception): pass

optionstring=""

def exit(msg=""):
	raise SystemExit(msg or optionstring.replace("%prog", sys.argv[0]))

def parse(docstring, argv=None, defaults={}):
	global optionstring
	optionstring = docstring

	match = USAGE.search(optionstring)
	if not match: raise ParsingError("Cannot find the option string")

	optlines = OPTGROUP.split(match.group(1))
	try:
		parser = optparse.OptionParser(optlines[0].strip())

		for line in optlines[1:]:
			opt, help = [p.strip() for p in line.split('\n')][:2]
			short, long = ('-'+opt).strip().split(',')[:2]

			action = 'store'
			choices = None
			default = None
			if '=' in opt:
				# unless the value on the other side of = is the same
				# (modulo case) it is used as the default

				long, default = long.strip().split('=')[:2]
				if default in defaults:
					default = defaults[default]

				elif default.lower() == long:
					default = None

				elif default.startswith('['):
					choices = [v.strip() for v in default.strip('[]').split('|')]
					default = choices[0]

				elif default == 'Count':
					action = 'count'
					default = 0

				elif default in ('True', 'False'):
					action = 'store_true'
					if default == 'True':
						default = True
					else:
						default = False

			else:
				action='store_true'

			parser.add_option(short.strip(), long.strip(),
				action=action, choices=choices, help=help.strip(), default=default)

	except (IndexError, ValueError):
		raise ParsingError("Cannot parse the option string correctly")

	optsv, args = parser.parse_args(argv)

    # Convert the optsv instance to a real dictionary...
	opts = {}
	for option in parser.option_list:
		if hasattr(option, 'dest') and option.dest:
			optname = str(option.dest)
			val = getattr(optsv, optname)
			if val or isinstance(val, int) or isinstance(val, bool):
				opts[optname] = val

	return parser, opts, args

