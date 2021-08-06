# -*- coding: utf-8 -*-
#	© Crown copyright 2008 - Rosie Clarkson, Chris Eveleigh (development@planningportal.gov.uk) for the Planning Portal
#
#	You may re-use the Crown copyright protected material (not including the Royal Arms and other departmental or agency logos)
#	free of charge in any format. The material must be acknowledged as Crown copyright and the source given.
#
#
# 23/03/2009 Patched by Miguel Tremblay, Environment Canada
#  Script is now supposed to be functionnal
#   with french characters in UTF-8
# 6 Dec 2012 fix from Andrew White at Typesafe
########################################################

import sys, os, time, tarfile
from email.Parser import Parser
from xml.dom import minidom
import xml.parsers.expat
import xml.sax.saxutils as saxutils
import htmlentitydefs
import re
from HTMLParser import HTMLParser
from urllib import quote, unquote
from urlparse import urljoin
from optparse import OptionParser

# add any other links you may want to map between wikis here
url_maps = {'http://tikiwiki.org/RFCWiki': 'http://meta.wikimedia.org/wiki/Cheatsheet'}


# checks for HTML tags
class HTMLChecker(HTMLParser):

    def handle_starttag(self, tag, attrs):
        global validate
        validate = True
        return True

    def handle_endtag(self, tag):
        global validate
        return True


# Mediawiki relies on having the right number of new lines between syntax - for example having two new lines in a list starts a new list.
# The elements that do/don't start a new line in HTML can be controlled by the CSS. The CSS used depends on which skin you're using.
class HTMLToMwiki(HTMLParser):
    global wikitext
    global sourceurl
    global pages
    global uploads
    global headings
    link = False  # if the parser is within a link
    src = ''
    innowiki = False
    inem = False  # if the parser is within italics
    instrong = False  # if the parser is within bold
    inheading = False  # if the parser is within a heading
    list = 0  # whether the parser is within an ordered list (is numeric to deal with nested lists)
    litem = 0  # whether the parser is within a list item - in order to deal with <p> and <br/> tags in ways that wont break it
    ul_count = 0  # the number of ul tags used for nested lists
    ol_count = 0  # the number of ol tags used for nested lists
    col_count = 0

    def handle_starttag(self, tag, attrs):
        if self.innowiki:
            completeTag = '<' + tag
            for attr in attrs:
                completeTag += ' ' + attr[0] + '="' + attr[1] + '"'
            wikitext.append(completeTag + '>')
        else:
            if tag == 'nowiki':
                wikitext.append('<nowiki>')
                self.innowiki = True
            if tag == 'a':
                self.src = ''
                for att in attrs:
                    if att[0] == 'href':
                        self.src = att[1]
                if self.src in url_maps:
                    self.src = url_maps[self.src]
                # deals with uploads
                if 'tiki-download_file.php' in self.src:
                    uploads.append(self.src)
                self.link = True
            if tag == 'ol':
                self.ol_count += 1
                self.list += 1

            if tag == 'ul':
                self.ul_count += 1
            if tag == 'li':
                # append the right no. of # or *s according to the level of nesting
                self.litem += 1
                if self.list > 0:
                    wikitext.append('\n' + ('#' * self.ol_count))
                else:
                    wikitext.append('\n' + ('*' * self.ul_count))
            if tag == 'img':
                src = ''
                for att in attrs:
                    if att[0] == 'src':
                        src = att[1]
                src = quote(src)
                # we have several different ways of specifying image sources in our tiki
                imagepath = urljoin(sourceurl, src)
                if options.newImagepath != '':
                    imagepath = urljoin(options.newImagepath, src.split('/')[-1])
                # the pic tag is used later to identify this as a picture and process the correct mwiki syntax
                wikitext.append('<pic>' + imagepath + ' ')
            if tag == 'table':
                wikitext.append('\n{|')
                for att in attrs:
                    # table formatting
                    wikitext.append(' ' + att[0] + '="' + att[1] + '"')
            if tag == 'tr':
                wikitext.append('\n|-')
                self.col_count = 0
            if tag == 'td':
                self.col_count += 1
                if self.col_count > 1:
                    wikitext.append('\n||')
                else:
                    wikitext.append('\n|')
            if tag == 'caption':
                wikitext.append('\n|+')
            if tag in ('strong', 'b'):
                self.instrong = True
                wikitext.append("'''")
            if tag in ('em', 'i'):
                self.inem = True
                wikitext.append("''")
            if tag == 'p':
                # new lines in the middle of lists break the list so we have to use the break tag
                if self.litem == 0:
                    br = '\n'
                else:
                    br = '<br/>'
                # newlines in the middle of formatted text break the formatting so we have to end and restart the formatting around the new lines
                if self.inem == True:
                    br = "''" + br + br + "''"
                if self.instrong == True:
                    br = "'''" + br + br + "'''"
                wikitext.append(br)
            if tag == 'h1':
                self.inheading = True
                # headings must start on a new line
                wikitext.append('\n\n==')
                headings.append(tag)
            if tag == 'h2':
                self.inheading = True
                wikitext.append('\n\n===')
                headings.append(tag)
            if tag == 'h3':
                self.inheading = True
                wikitext.append('\n\n====')
                headings.append(tag)
            else:
                wikitext.append('<' + tag + '>')

    def handle_endtag(self, tag):
        if tag == 'nowiki':
            wikitext.append('</nowiki>')
            self.innowiki = False
        if not self.innowiki:
            if self.link == True:
                self.src = ''
                self.link = False
            if tag == 'img':
                wikitext.append('</pic>')
            if tag == 'ol':
                self.ol_count -= 1
                self.list -= 1
                wikitext.append('\n\n')
            if tag == 'ul':
                self.ul_count -= 1
                wikitext.append('\n\n')
            if tag == 'li':
                self.litem -= 1
            if tag == 'table':
                wikitext.append('\n\n|}')
            if tag in ('strong', 'b'):
                self.instrong = False
                wikitext.append("'''")
            if tag in ('em', 'i'):
                self.inem = False
                wikitext.append("''")
            if tag == 'h1':
                self.inheading = False
                wikitext.append('==\n\n')
            if tag == 'h2':
                self.inheading = False
                wikitext.append('===\n\n')
            if tag == 'h3':
                self.inheading = False
                wikitext.append('====\n\n')
            if tag == 'p':
                if self.inheading == True:
                    br = ''
                elif self.litem == 0:
                    br = '\n'
                else:
                    br = '<br/>'
                if self.inem == True:
                    br = " ''" + br + "''"
                if self.instrong == True:
                    br = " '''" + br + "'''"
                wikitext.append(br)
            if tag == 'br':
                if self.inheading == True:
                    br = ''
                elif self.litem == 0:
                    br = '\n'
                else:
                    br = '<br/>'
                if self.inem == True:
                    br = " ''" + br + "''"
                if self.instrong == True:
                    br = " '''" + br + "'''"
                wikitext.append(br)
            if tag == 'hr':
                wikitext.append('\n----\n')
            else:
                wikitext.append('</' + tag + '>')
        else:
            wikitext.append('</' + tag + '>')

    # check for symbols which are mwiki syntax when at the start of a line
    def check_append(self, data):
        stripped = data.lstrip()
        for symbol in ('----', '*', '#', '{|', '==', '===', '===='):
            if stripped.startswith(symbol):
                if len(wikitext) > 2 and wikitext[-3] == '\n':
                    if not symbol.startswith('='):
                        data = '<nowiki>' + symbol + '</nowiki>' + stripped[len(symbol):]
                    else:
                        if data.find(symbol, len(symbol)):
                            data = '<nowiki>' + symbol + '</nowiki>' + stripped[len(symbol):]
        return data

    def handle_data(self, data):
        if self.link == True:
            # sometimes spaces are in the piped data (probably because of our editor) so we need to make sure we add that before the link
            space = ''
            if data.startswith(' '):
                space = ' '
            if self.src.startswith(sourceurl + 'tiki-download_file.php'):
                wikitext.append(space + '[' + self.src + ' ' + data + ']')
            elif self.src.startswith(sourceurl):
                if 'page=' in self.src:
                    ptitle = self.src.split('page=')
                    page = ptitle[1].replace('+', ' ')
                    for file in pages:
                        # mwiki is case sensitive to page names and tikiwiki isn't so check that the file actually exists
                        if file.lower() == page.lower():
                            page = file
                    wikitext.append(space + '[[' + page + '|' + data + ']]')
            else:
                # catch relative urls
                if self.src.startswith('..'):
                    self.src = urljoin(sourceurl, self.src)
                wikitext.append(space + '[' + self.src + ' ' + data + ']')
        elif self.litem:
            # if we're in a list put nowiki tags around data begining with * or # so it isnt counted as nesting
            if data[0] in ('*', '#'):
                data = '<nowiki>' + data[0] + '</nowiki>' + data[1:]
            wikitext.append(data)
        else:
            data = self.check_append(data)
            wikitext.append(data)

    def handle_entityref(self, data):
        data = "&amp;" + data + ";"
        if self.link:
            wikitext.append(' ' + data)
        elif self.litem:
            wikitext.append(data)
        else:
            wikitext.append(data)

    def handle_charref(self, data):
        data = "&amp;" + data + ";"
        if self.link:
            wikitext.append(' ' + data)
        elif self.litem:
            wikitext.append(data)
        else:
            wikitext.append(data)


def insertImage(word, words):
    global image
    global imagenames
    global imageids
    global imagepath
    # there are even more ways to specify pic sources in our tiki
    if 'name=' in word:
        parts = word.split('=')
        try:
            filename = imagenames[parts[2]]
        except KeyError:
            sys.stderr.write(parts[2] + ' doesn\'t exist in your image XML file and won\'t be displayed properly\n')
            filename = parts[2]
        filename = quote(filename)
        imagepath = urljoin(urljoin(sourceurl, imageurl), filename)
        if options.newImagepath != '':
            imagepath = urljoin(options.newImagepath, filename)
        words.append('<pic>' + imagepath)
    if 'id=' in word:
        parts = word.split('=')
        try:
            filename = imageids[parts[2]]
        except KeyError:
            sys.stderr.write('The image with ID ' + parts[
                2] + ' doesn\'t exist in your image XML file and won\'t be displayed properly\n')
            filename = parts[2]
        filename = quote(filename)
        imagepath = urljoin(urljoin(sourceurl, imageurl), filename)
        if options.newImagepath != '':
            imagepath = urljoin(options.newImagepath, filename)
        words.append('<pic>' + imagepath)
    if '}' in word:
        bracket = word.find('}')
        if word[-1] != '}':
            if word[bracket + 1] != ' ':
                word = word.replace('}', '</pic> ')
            else:
                word = word.replace('}', '</pic>')
        word = word.replace('}', '</pic>')
        words.append(word)
        image = False

    return words


def insertLink(word):
    global intLink
    global page
    global words
    global pages
    first = False
    # the link may be split if it contains spaces so it may be sent in parts
    brackets = word.find('((')
    if brackets != -1:
        word = word.replace('((', '[[')
        page = word[brackets:]
        words.append(word[:brackets])
        if '))' in word:
            word = word.replace('))', ']]')
            end = word.find(']]')
            text = word[brackets + 2:end]
            # again check the filenames to ensure case sensitivity is ok
            for file in pages:
                if unicode(file, "Latin-1").lower() \
                        == text.lower():
                    text = file
            text = '[[' + text + word[end:]
            if text[-1] != '\n':
                words.append(text + ' ')
            else:
                words.append(text)
            page = ''
            intLink = False

    elif '))' in word:
        word = word.replace('))', ']]')
        page += ' ' + word
        pipe = page.find('|')
        if pipe != -1:
            end = pipe
            text = page[2:pipe]
        else:
            brackets = page.find(']]')
            end = brackets
            text = page[2:brackets]
        for file in pages:
            if unicode(file, "latin-1").lower() == text.lower():
                page = page[:2] + file + page[end:]
        if page[-1] != '\n':
            words.append(page + ' ')
        else:
            words.append(page)
        page = ''
        intLink = False
    else:
        first = False
        page += ' ' + word


parser = OptionParser()
parser.add_option("-n", "--notableofcontents",
                  action="store_true", dest="notoc", default=False,
                  help="disable all automatic contents tables")
parser.add_option("-m", "--maxfilesize",
                  action="store", type="int", dest="max", default=1,
                  help="the maximum import file size")
parser.add_option("-j", "--newimageurl",
                  action="store", type="string", dest="newImagepath", default='',
                  help="the new location of any images (inc. trailing slash)")
parser.add_option("-i", "--imageurl",
                  action="store", type="string", dest="imageurl", default='',
                  help="the relative URL used in tiki to access images (inc. trailing slash)")
parser.add_option("-p", "--privatepages",
                  action="store", type="string", dest="privatexml", default='',
                  help="an XML file containing any private pages not to be added to the wiki")
parser.add_option("-o", "--outputfile",
                  action="store", type="string", dest="outputFile", default='',
                  help="the name of the output wiki XML file(s)")
parser.add_option("-k", "--imagexml",
                  action="store", type="string", dest="imagexml", default='',
                  help="an XML file containing metadata for the images in the tiki")

(options, args) = parser.parse_args()

# the tar file containing the tiki file export - if not specified read from stdin
# stdin doesn't work at the moment and fails after you've used extractfile as this returns nothing
if len(args) > 1:
    archive = tarfile.open(args[1])
    # add all files in the export tar to the list of pages
    pages = archive.getnames()
    if options.outputFile == '':
        outputFile = args[1].replace('.tar', '.xml')
    else:
        outputFile = options.outputFile
else:
    pages = []
    # if reading from stdin you can't iterate through the files again so pages is left empty and links are not corrected
    archive = tarfile.open(name=sys.stdin.name, mode='r|', fileobj=sys.stdin)
    # if you're reading from stdin and don't specify an output file output to stdout
    if options.outputFile == '':
        options.outputFile = '-'
p = Parser()

# multiple files may be created so this is added to the output file string to identify them
fileCount = 0

# the string to name all outputfiles the fileCount is added to this
if options.outputFile == '-':
    mwikixml = sys.stdout
else:
    mwikixml = open(outputFile[:-4] + str(fileCount) + outputFile[-4:], 'wb')
    sys.stderr.write('Creating new wiki xml file ' + outputFile[:-4] + str(fileCount) + outputFile[-4:])

# the source URL of the tiki - in the form http://[your url]/tiki/
sourceurl = args[0]

# the relative address used to access pictures in TikiWiki
imageurl = options.imageurl

privatePages = []
if options.privatexml != '':
    privateparse = minidom.parse(options.privatexml)
    rows = privateparse.getElementsByTagName('row')
    for row in rows:
        fields = row.getElementsByTagName('field')
        for field in fields:
            if field.getAttribute('name') == 'pageName':
                privatePages.append(field.firstChild.data)
# fill the lookup table with the image information
# a file containing an xml dump from the tiki DB
imagenames = {}
imageids = {}
if options.imagexml != '':
    imagexml = options.imagexml
    lookup = minidom.parse(imagexml)

    rows = lookup.getElementsByTagName('row')
    for row in rows:
        fields = row.getElementsByTagName('field')
        for field in fields:
            if field.getAttribute('name') == 'name':
                iname = field
            if field.getAttribute('name') == 'filename':
                ifile = field
            if field.getAttribute('name') == 'imageID':
                iid = field
        imagenames[iname.firstChild.data] = ifile.firstChild.data
        imageids[iid.firstChild.data] = ifile.firstChild.data

# list of users who have edited pages
authors = []
filepages = {}
totalSize = 0
pagecount = 0
versioncount = 0

# write mediawiki xml file
mwikixml.write('<mediawiki xml:lang="en">\n')

for member in archive:
    if member.name not in privatePages:
        # add each file in the tiki export directory
        tikifile = archive.extractfile(member)
        mimefile = p.parse(tikifile)
        mwikixml.write('<page>\n')
        partcount = 0
        uploads = []

        if not mimefile.is_multipart():
            partcount = 1
        for part in mimefile.walk():
            outputpage = ''
            if partcount == 1:
                title = unquote(part.get_param('pagename'))
                outputpage += '<title>' + title + '</title>'
            partcount += 1
            if part.get_params() is not None and \
                    ('application/x-tikiwiki', '') in part.get_params():
                versioncount += 1
                headings = []
                if part.get_param('lastmodified') == None:
                    break
                outputpage += '<revision>\n'
                outputpage += '<timestamp>' + \
                              time.strftime('%Y-%m-%dT%H:%M:%SZ',
                                            time.gmtime(eval(part.get_param('lastmodified')))) + '</timestamp>\n'
                outputpage += '<contributor><username>' + part.get_param('author') + '</username></contributor>\n'
                # add author to list of contributors to be output at the end
                if part.get_param('author') not in authors:
                    authors.append(part.get_param('author'))
                outputpage += '<text xml:space="preserve">\n'
                mwiki = ''
                # we add the tiki description to the page in bold and italic (much as it was in tikiwiki)
                # for them to function properly we need to ensure that these strings are followed by a new line
                # the </br> is used as a placeholder and is converted to \n later
                if part.get_param('description') not in (None, ''):
                    mwiki += "'''''" + unquote(part.get_param('description')) + "'''''</br>"
                # then add the table of contents (or specify none)
                if options.notoc:
                    mwiki = mwiki + "__NOTOC__</br>"
                else:
                    mwiki += "__TOC__</br>"
                mwiki += part.get_payload().decode('utf-8')

                # does the validator do anything?!
                validate = False
                validator = HTMLChecker()
                validator.feed(mwiki)
                # fixes pages that end up on a single line (these were probably created by our WYSWYG editor being used on windows and linux)
                if not validate:
                    mwiki = mwiki.replace('\t', '    ')
                    mwiki = mwiki.replace('  ', ' &nbsp;')
                    mwiki = mwiki.replace('<', '&lt;')
                    mwiki = mwiki.replace('>', '&gt;')

                    # make sure newlines after headings are preserved
                    next = 0
                    while '\r\n!' in mwiki[next:] or '&lt;/br&gt;!' in mwiki[next:] or mwiki[next:].startswith('!'):
                        if mwiki[next:].startswith('!'):
                            found = next
                        else:
                            foundreturn = mwiki.find('\r\n!', next)
                            foundbreak = mwiki.find('&lt;/br&gt;!', next)
                            if (foundreturn != -1 and foundreturn < foundbreak) or foundbreak == -1:
                                found = foundreturn + 2
                            else:
                                found = foundbreak + 11

                        next = mwiki.find('\r\n', found)
                        if next == -1: break
                        mwiki = mwiki[:next] + '</br>' + mwiki[next + 2:]
                        next += 5

                    # as validate is false the page does not contain any html so whitespace needs to be preserved
                    mwiki = mwiki.replace('\r\n', '</br>')

                # double escape < and > entities so that &lt; is not unescaped to < which is then treated as HTML tags
                # mwiki=mwiki.replace('&amp;', '&amp;amp;')
                mwiki = mwiki.replace('&amp;lt;', '&amp;amp;lt;')
                mwiki = mwiki.replace('&amp;gt;', '&amp;amp;gt;')
                mwiki = mwiki.replace('&lt;', '&amp;lt;')
                mwiki = mwiki.replace('&gt;', '&amp;gt;')
                mwiki = mwiki.replace(u'\ufffd', '&nbsp;')

                # unescape XML entities
                entitydefs = dict(("&" + k + ";", unichr(v)) for k, v in htmlentitydefs.name2codepoint.items())
                entitydefs.pop("&amp;")
                entitydefs.pop("&gt;")
                entitydefs.pop("&lt;")
                mwiki = saxutils.unescape(mwiki, entitydefs)

                # replace tiki syntax that will be interpreted badly with tiki syntax the parser will understand
                # empty formatting tags will be converted to many "'"s which then confuses mwiki
                mwiki = mwiki.replace('[[', '~np~[~/np~')
                # need to replace no wiki tags here in case any html/xml is inside them that we want to keep
                mwiki = mwiki.replace('~np~', '<nowiki>')
                mwiki = mwiki.replace('~/np~', '</nowiki>')
                mwiki = mwiki.replace('<em></em>', '')
                mwiki = mwiki.replace('<em><em>', '<em>')
                mwiki = mwiki.replace('</em></em>', '</em>')
                mwiki = mwiki.replace('<strong></strong>', '')
                mwiki = mwiki.replace('<strong><strong>', '<strong>')
                mwiki = mwiki.replace('</strong></strong>', '</strong>')
                # this makes sure definitions keep their preceding newline
                mwiki = mwiki.replace('\n;', '</br>;')
                mwiki = mwiki.replace('\n', ' ')
                mwiki = mwiki.replace('</br>', '\n')
                mwiki = mwiki.replace('&lt;/br&gt;', '\n')
                mwiki = mwiki.replace('\r', ' ')
                mwiki = mwiki.replace('\t', ' ')

                # Mediawiki automatically creates a table of content
                mwiki = mwiki.replace('Table of content', '')
                mwiki = mwiki.replace('{maketoc}', '')

                # convert === underline syntax before the html converter as
                # headings in mwiki use =s and h3 tags will become ===heading===
                next = 0
                while '===' in mwiki[next:]:
                    start = mwiki.find('===', next)
                    end = mwiki.find('===', start + 3)

                    if end != -1:
                        mwiki = mwiki[:start] + '<u>' + mwiki[start + 3:end] + '</u>' + mwiki[end + 3:]
                    next = start + 1
                # if there is another === convert them both

                # print mwiki

                wikitext = []

                # convert any HTML tags to mediawiki syntax
                htmlConverter = HTMLToMwiki()
                htmlConverter.feed(mwiki)

                mwiki = ''.join(wikitext)

                # replace tiki syntax with mwiki
                mwiki = mwiki.replace('__', "'''")

                # split the text into lines and then strings to parse
                words = []
                image = False
                intLink = False
                box = False
                colour = False
                inColourTag = False
                page = ''
                centre = False
                for line in mwiki.splitlines(True):
                    # Convert external links to mediawiki syntax
                    m = re.match(r'(.*)\[(.*)\|(.*)\](.*)', line)
                    if m:
                        line = m.group(1) + "[" + re.sub(r'(.*)&amp;(.*);(.*)', r'\1&\2\3', m.group(2)) + " " + m.group(
                            3) + "]" + m.group(4) + "\n"

                    # Convert 'CODE' samples
                    line = re.sub(r'(.*){CODE\(caption=&amp;gt;(.*)\)}(.*)', r'\1<!-- \2 --><pre>\3', line)
                    line = re.sub(r'(.*){CODE}(.*)', r'\1</pre>\2', line)

                    # Convert anchor
                    line = re.sub(r'(.*){ANAME\(\)}(.*){ANAME}(.*)', r'\1<span id=&quot;\2&quot;></span>\3', line)
                    # Convert anchor links
                    line = re.sub(r'(.*){ALINK\(aname=(?:")?([^"]*)(?:")?\)}(.*){ALINK}(.*)', r'\1[[#\2|\3]]\4', line)

                    heading = False
                    noCentre = False
                    # if there are an odd no. of ::s don't convert to centered text
                    if line.count('::') % 2 != 0:
                        noCentre = True
                    count = 0
                    spl = line.split(' ')
                    if spl[0].find('!') == 0: heading = True
                    for word in spl:
                        # handle headings
                        if heading is True:
                            if count is 0 and word:
                                # replace !s
                                bangs = 0;
                                while word[bangs] == '!':
                                    word = word.replace('!', '=', 1)
                                    bangs += 1
                                    if bangs >= len(word):
                                        if len(spl) == 1: bangs /= 2
                                        break
                            if count is len(spl) - 1:
                                # add =s to end
                                end = word.find('\n')
                                if end != -1:
                                    word = word[:end] + (bangs * '=') + word[end:]
                                else:
                                    word = word[:end] + (bangs * '=')
                        # handle centered text
                        if '::' in word and not noCentre:
                            next = 0
                            while '::' in word[next:]:
                                next = word.find('::')
                                if centre:
                                    centre = False
                                    word = word.replace('::', '</center>', 1)
                                else:
                                    centre = True
                                    word = word.replace('::', '<center>', 1)
                        # handle font colours
                        if inColourTag:
                            colon = word.find(':')
                            if colon != -1:
                                word = word[:colon] + '">' + word[colon + 1:]
                                inColourTag = False
                        if '~~' in word:
                            next = 0
                            while '~~' in word[next:]:
                                next = word.find('~~')
                                if colour == True:
                                    # end span
                                    colour = False
                                    word = word.replace('~~', '</span>', 1)
                                else:
                                    # start span
                                    colour = True
                                    colon = word.find(':', next)
                                    extratext = ''
                                    if colon != -1:
                                        word = word[:next] + "<span style='color:" + word[next + 2:colon] + "'>" + word[
                                                                                                                   colon + 1:]
                                    else:
                                        word = word[:next] + '<span style="color:' + word[next + 2:]
                                        inColourTag = True
                                next += 1
                        # handle boxes
                        if '^' in word:
                            hats = word.count('^')
                            for hat in range(1, hats + 1):
                                index = word.find('^')
                                if not box:
                                    word = word[:index] + '<pre>' + word[index + 1:]
                                    box = True
                                else:
                                    word = word[:index] + '</pre>' + word[index + 1:]
                                    box = False
                        if '{img' in word:
                            image = True
                        if '((' in word:
                            intLink = True
                        if image:
                            words = insertImage(word, words)
                        elif intLink:
                            insertLink(word)
                        else:
                            # stops mwiki automatically creating links (which can then be broken by formatting
                            if (
                                    'http' in word or 'ftp://' in word) and '[' not in word and ']' not in word and '<pic>' not in word and '<pre>' not in word and '</pre>' not in word and not box:
                                index = 0
                                format = False
                                formatted = ''
                                for char in word:
                                    index += 1
                                    if char == "'":
                                        if not format:
                                            format = True
                                            formatted = formatted + '</nowiki>'
                                    else:
                                        if format:
                                            format = False
                                            formatted = formatted + '<nowiki>'

                                    formatted += char

                                word = '<nowiki>' + formatted + '</nowiki>'
                            if word != '':
                                if '\n' in word[-1]:
                                    words.append(word)
                                else:
                                    words.append(word + ' ')
                        count += 1

                mwiki = ''.join(words)
                # get rid of pic placeholder tags
                mwiki = mwiki.replace("<pic>", "")
                mwiki = mwiki.replace("</pic>", "")

                # make sure there are no single newlines - mediawiki just ignores them. Replace multiple lines with single and then single with double.
                while "\n\n" in mwiki or "\n \n" in mwiki:
                    mwiki = mwiki.replace("\n\n", "\n")
                    mwiki = mwiki.replace("\n \n", "\n")
                mwiki = mwiki.replace('\n', '\n\n')

                # replace multiple lines with single where they would break formatting - such as in a list
                mwiki = mwiki.replace('\n\n#', '\n#')
                mwiki = mwiki.replace('\n\n*', '\n*')
                mwiki = mwiki.replace('*<br/>', '*')
                mwiki = mwiki.replace('#<br/>', '#')
                mwiki = mwiki.lstrip('\n')

                lines = []
                for line in mwiki.splitlines(True):
                    if line.startswith(':'):
                        line = '<nowiki>:</nowiki>' + line[1:]
                    lines.append(line)
                mwiki = ''.join(lines)

                entitydefs = dict((unichr(k), "&amp;" + v + ";") for k, v in htmlentitydefs.codepoint2name.items())
                entitydefs.pop('<')
                entitydefs.pop('>')
                entitydefs.pop('&')
                mwiki = saxutils.escape(mwiki, entitydefs)

                for n in range(len(mwiki)):
                    if mwiki[n] < " " and mwiki[n] != '\n' and mwiki[n] != '\r' and mwiki[n] != '\t':
                        mwiki = mwiki[:n] + "?" + mwiki[n + 1:]

                mwiki = mwiki.replace('amp;lt;', 'lt;')
                mwiki = mwiki.replace('amp;gt;', 'gt;')

                while "  " in mwiki:
                    mwiki = mwiki.replace("  ", " ")
                mwiki = mwiki.replace('&lt;!--', '<!--')
                mwiki = mwiki.replace('--&gt;', '-->')

                # the table of contents will have been seen as bold formatting
                if len(headings) >= 3:
                    mwiki = mwiki.replace("'''TOC'''", '__TOC__')
                    mwiki = mwiki.replace("'''NOTOC'''", '__NOTOC__')
                else:
                    mwiki = mwiki.replace("'''TOC'''\n\n", '')
                    mwiki = mwiki.replace("'''NOTOC'''\n\n", '')
                    mwiki = mwiki.replace("'''TOC'''\n",
                                          '')  # if it's before bullets/numbers the second \n will have gone
                    mwiki = mwiki.replace("'''NOTOC'''\n", '')

                outputpage = unicode(outputpage, "Latin-1")
                outputpage += mwiki + '</text>\n'
                outputpage += '</revision>\n'
                outputpage = outputpage.encode('utf-8')
                totalSize += len(outputpage)

                # mediawiki has a maximum import file size so start a new file after that limit
                if options.outputFile != '-':
                    if totalSize > options.max * 1024 * 1024:
                        totalSize = len(unicode(outputpage, "Latin-1"))
                        mwikixml.write('</page>')
                        mwikixml.write('</mediawiki>')
                        fileCount += 1
                        mwikixml = open(outputFile[:-4] + str(fileCount) + outputFile[-4:], 'wb')
                        sys.stderr.write(
                            'Creating new wiki xml file ' + outputFile[:-4] + str(fileCount) + outputFile[-4:] + '\n')
                        mwikixml.write('<mediawiki xml:lang="en">\n')
                        # if this isn't the first part write page and title
                        mwikixml.write('<page>\n')
                        mwikixml.write('<title>' + title + '</title>')
                    mwikixml.write(outputpage)
                else:
                    mwikixml.write(outputpage)
            else:
                if partcount != 1:
                    if sys.stdout == False:
                        sys.stderr.write(str(part.get_param('pagename')) + ' version ' + str(
                            part.get_param('version')) + ' wasn\'t counted')

        mwikixml.write('</page>')
        if uploads != []:
            filepages[title] = uploads
        pagecount += 1
mwikixml.write('</mediawiki>')
sys.stderr.write('\nnumber of pages = ' + str(pagecount) + ' number of versions = ' + str(versioncount) + '\n')
sys.stderr.write('with contributions by ' + str(authors) + '\n')
sys.stderr.write('and file uploads on these pages: ' + str(filepages.keys()) + '\n')