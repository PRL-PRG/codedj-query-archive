import sys
import xml.parsers.expat

input = None
output = None
parser = None
data_buf = ""

def trans(data):
  output.write("#: %s:%d\n" %
      (input.name, parser.CurrentLineNumber))
  data = data.strip()
  if not data or data.isspace(): return
  lines = data.replace(
    "\\", "\\\\").replace("\"", "\\\"").split("\n")
  if len(lines) > 1:
    output.write("msgid \"\"\n\"%s\"\n" %
        lines.join("\\n\"\n\""))
  else:
    output.write("msgid \"%s\"\n" % lines[0])
  output.write("msgstr \"\"\n\n")

def start_element(name, attrs):
  global data_buf
  for a in ['title', 'label', 'end', 'unit']:
    if attrs.has_key(a): trans(attrs[a])
  data_buf = ""

def end_element(name):
  global data_buf
  trans(data_buf)
  data_buf = ""

def char_data(data):
  global data_buf
  data_buf += data

input = open(sys.argv[1])
output = sys.stdout
parser = xml.parsers.expat.ParserCreate()
parser.StartElementHandler = start_element
parser.EndElementHandler = end_element
parser.CharacterDataHandler = char_data
parser.ParseFile(input)
input.close()
output.close()
