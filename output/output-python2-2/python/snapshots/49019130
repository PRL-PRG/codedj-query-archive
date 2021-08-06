#! /usr/bin/python

import sys
import textwrap
import re
import xml.dom.minidom

if len(sys.argv) < 3:
    print >>sys.stderr, "Usage: %s help.xml install|live" % sys.argv[0]
    sys.exit(1)

document = xml.dom.minidom.parse(sys.argv[1])
mode = sys.argv[2]

text = ""
paratext = ""

def getText(nodelist):
    text = ""
    for node in nodelist:
        if node.nodeType == node.ELEMENT_NODE:
            if node.tagName == "link":
                text += "<\x12%s\x13%s\x10>" % (node.getAttribute("linkend"),
                                                getText(node.childNodes))
            elif node.tagName == "ulink":
                text += "\x11%s\x10" % node.getAttribute("url")
            elif node.tagName == "emphasis" or node.tagName == "screen":
                text += "\x11%s\x10" % getText(node.childNodes)
            elif node.tagName == "userinput":
                text += "\x11%s\x10" % getText(node.childNodes).rstrip("\x10")
            elif node.tagName == "optional":
                text += "[%s]" % getText(node.childNodes)
            else:
                text += getText(node.childNodes)
        elif node.nodeType == node.TEXT_NODE:
            text += node.data
    return re.sub(re.compile(r'^ +| +$', re.MULTILINE), '', text.strip())

def fill(text, width=76, indent=''):
    squashed = re.sub(r'\n+', ' ', text)
    wrapper = textwrap.TextWrapper(width=width, initial_indent=indent,
                                   subsequent_indent=indent)
    # Eww - but textwrap doesn't provide any other way to turn off
    # hyphenation ... This may break in a future version of Python.
    wrapper.wordsep_re = re.compile(r'(\s+)')
    return wrapper.fill(squashed)

def stripLinks(text):
    return re.sub(r'\x12.+?\x13(.+?)\x10', r'\1', text)

def handleReference(reference):
    for refentry in reference.getElementsByTagName("refentry"):
        handleRefEntry(refentry)
    sys.stdout.write("\0\n")

def handleRefEntry(refentry):
    global text
    sys.stdout.write("\x04")
    handleRefNameDiv(refentry.getElementsByTagName("refnamediv")[0])
    handleRefSection(refentry.getElementsByTagName("refsection")[0])
    sys.stdout.write(text.rstrip('\n').encode('UTF-8'))
    text = ''

def handleRefNameDiv(refnamediv):
    global text
    refdescriptor = refnamediv.getElementsByTagName("refdescriptor")[0]
    keycap = refdescriptor.getElementsByTagName("keycap")[0]
    linkname = getText(keycap.childNodes)
    refname = refnamediv.getElementsByTagName("refname")[0]
    title = getText(refname.childNodes)
    text += "\x12%s\x14%s\x10" % (linkname, title)

def handleRefSection(refsection):
    for node in refsection.childNodes:
        if node.nodeType == node.ELEMENT_NODE:
            if node.tagName == "title":
                handleRefSectionTitle(node)
            elif node.tagName == "segmentedlist":
                handleSegmentedList(node)
            elif node.tagName == "variablelist":
                handleVariableList(node)
            elif node.tagName == "informalexample":
                handleInformalExample(node)
            elif node.tagName == "para":
                handlePara(node)
            else:
                handleRefSection(node)

def handleRefSectionTitle(title):
    global text
    if len(title.childNodes) > 0:
        text += "\x11%s\x10" % getText(title.childNodes)
        text += "\n\n"

def handleSegmentedList(segmentedlist):
    global text

    segmentedlistclass = segmentedlist.getAttribute("class")
    if segmentedlistclass == "helpindex":
        keywidth = 7
    elif segmentedlistclass == "bootparams-hardware":
        keywidth = 39
    elif segmentedlistclass == "bootparams-disk":
        keywidth = 29
    else: # segmentedlistclass == "bootparams-installer"
        keywidth = 40

    handleSegmentedListTitle(segmentedlist.getElementsByTagName("title")[0])
    handleSegTitles(segmentedlist.getElementsByTagName("segtitle"), keywidth)
    handleSegListItems(segmentedlist.getElementsByTagName("seglistitem"),
                       keywidth)
    text += "\n"

def handleSegmentedListTitle(title):
    global text
    if len(title.childNodes) > 0:
        text += "\x11%s\x10" % getText(title.childNodes)
        text += "\n\n"

def handleSegTitles(segtitles, keywidth):
    global text
    if len(segtitles) >= 2:
        text += "\x11%-*s%s\x10" % (keywidth, getText(segtitles[0].childNodes),
                                    getText(segtitles[1].childNodes))
        text += "\n\n"

def handleSegListItems(seglistitems, keywidth):
    global text
    for seglistitem in seglistitems:
        segs = seglistitem.getElementsByTagName("seg")
        key = fill(getText(segs[0].childNodes))
        plainkey = stripLinks(key.split("\n")[-1])
        topic = getText(segs[1].childNodes)
        if len(plainkey) > keywidth - 1:
            text += "%s\n%s%s" % (key, " " * keywidth, topic)
        else:
            text += "%s%s%s" % (key, " " * (keywidth - len(plainkey)), topic)
        text += "\n"

def handleVariableList(variablelist):
    global text
    for varlistentry in variablelist.getElementsByTagName("varlistentry"):
        handleVarListEntry(varlistentry)
    text += "\n"

def handleVarListEntry(varlistentry):
    global text
    terms = varlistentry.getElementsByTagName("term")
    text += ", ".join(map(
        lambda term: "\x11%s\x10" % getText(term.childNodes), terms))
    text += "\n"
    listitem = varlistentry.getElementsByTagName("listitem")[0]
    text += fill(getText(listitem.childNodes), 76, '  ')
    text += "\n"

def handleInformalExample(informalexample):
    global text
    for screen in informalexample.getElementsByTagName("screen"):
        text += "  " + getText(screen.childNodes)
    text += "\n\n"

# This whole nobreak business sucks. It's there because
# Locale::Po4a::Docbook doesn't want to translate <phrase> elements within
# <para>s separately, but instead globs them together into one big msgid.
# To work around this, we do <para class="nobreak"> <para class="nobreak">
# ... <para>, but of course then we have to make sure to collect all the
# text up to the break and make sure to fill it together.
def handlePara(para):
    global text, paratext
    if paratext != "":
        lastchar = ord(paratext[-1])
        if lastchar >= 32 and lastchar <= 127 and not paratext[-1].isspace():
            paratext += " "
    paratext += getText(para.childNodes)
    if (not para.hasAttribute("class") or
        para.getAttribute("class") != "nobreak"):
        text += fill(paratext)
        text += "\n\n"
        paratext = ""

def preprocess(parent):
    global mode
    nodelist = parent.childNodes
    for node in nodelist:
        if node.nodeType == node.ELEMENT_NODE:
            if node.hasAttribute("condition"):
                condition = node.getAttribute("condition")
                if condition != "gfxboot" and condition != mode:
                    parent.removeChild(node).unlink()
                    continue
        preprocess(node)

preprocess(document)

reference = document.getElementsByTagName("reference")[0]
handleReference(reference)
