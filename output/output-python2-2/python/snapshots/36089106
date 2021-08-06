#!/usr/bin/env python
"""dlcs - Operate on a del.icio.us bookmark collection from the command-line.

! while most commands should be working properly, there are some issues to be
resolved mostly with encoding. Be careful when editing, or do proper testing
first.

Overview
--------
`dlcs` is a simple wrapper around pydelicious.DeliciousAPI. If offers
some facilities to get data from your online bookmark collection and
to perfom operations. See::

    % dlcs --help

This tool enables quick access but the server communication may be slow and
when doing multiple expensive requests del.icio.us may throttle you and
return 503's. This all depends on your collection size (posts/tags/bundles)
ofcourse. In any case, the post and tag lists are stored locally and some
changes to your collection may not be noticed until you clear this cache
since posts/update does not notice any edits.

Quickstart
----------
Just start `dlcs` using::

    % dlcs -u <username>

Post any URL using::

    % dlcs postit <URL>

Configuration
-------------
Your username and password can be stored in an INI formatted configuration
file under the section name 'dlcs'. The default location is ~/.dlcs-rc
but this can be changed using command line options. If no username or
password are provided `dlcs` will guess the username and prompt for the
password.

Limitation
----------
- Bundle sizes are restricted by the maximum URL size [@xxx:length?], the
  del.icio.us interface allows bigger bundles.

Integration
-----------
To bookmark http URLs with lynx, put the following line in your lynx.cfg::

    EXTERNAL:http:dlcs postit %s

For the elinks browser, create a uri_passing rule in the configuration file.
Something like the following::

    set document.uri_passing.dlcs = "bash -c \"dlcs postit %c\""

TODO
----
- Output formatting (--outf)
- Pretty JSON printer
- Append recent posts (and tags) to cache
- Some intelligent statistics on the tag collection (tag size, usage)
- Other users, is it possible to: list all posters for a URL, all tags for a URL? Popular tags?
- There are no commands to work on date lists (but 'req' could)
- Tag relations,
- Tag value, could a simple algorithm weigh the value of a specific tag (combination)?
"""
import sys
import os
import optparse
import getpass
import time
import locale
import codecs
from os.path import expanduser, getmtime, exists, abspath
from ConfigParser import ConfigParser
import pydelicious
from pydelicious import DeliciousAPI, dlcs_parse_xml, PyDeliciousException

try:
    # @bvb: simplejson thinks it should be different and deprecated read()
    # and write() not sure why...
    # @XXX: simplejson has UTF-8 default, json uses ASCII I think?
    from simplejson import dumps as jsonwrite, loads as jsonread
except:
    try:
        from json import read as jsonread, write as jsonwrite
    except:
        print >>sys.stderr, "No JSON decoder installed"

__cmds__ = [
    'bundle',
    'bundleadd',
    'bundleremove',
    'bundles',
    'clearcache',
    'deletebundle',
    'deleteposts',
    'findposts',
    'findtags',
    'getbundle',
    'getposts',
    'gettags',
    'help',
    'info',
    'post',
    'postit',
    'posts',
    'postsupdate',
    'recent',
    'rename',
    'req',
    'stats',
    'tag',
    'tags',
    'tagged',
    'untag',
]

__usage__ = """Usage: %prog [options] [command] [args...]

    -c, --config=DLCS_CONFIG
        Use custom config file [%default]

    -C, --keep-cache=False
        Don't update locally cached file(s) if they're out of date.

    -e, --encoding=ENCODING
        Use custom character encoding [locale: %default]

    -u, --username
        del.icio.us username (defaults to config or loginname)

    -p, --password
        Password for the del.icio.us user (usage not recommended, but this will override the config)

    -I, --ignore-case=False
        Ignore case for string searches

    -d, --dump
        Dump entire response (`req` only)

    -o, --outf=[text | json | prettyjson]
        Output formatting

    -s, --shared=[True | False]
        When posting a URL, set the 'shared' parameter.

    -r, --replace=[no | yes]
        When posting a URL, set the 'replace' parameter.

    -v, --verboseness=0
        TODO: Increase or set DEBUG (defaults to 0 or the DLCS_DEBUG env. var.)

""" + """command can be one of:
%s

Use `help` to get more information about a command."""\
    % (", ".join(__cmds__))


DEBUG = 0
if 'DLCS_DEBUG' in os.environ:
    DEBUG = int(os.environ['DLCS_DEBUG'])
    pydelicious.DEBUG = DEBUG

if 'DLCS_CONFIG' in os.environ:
    DLCS_CONFIG = os.environ['DLCS_CONFIG']
elif exists(abspath('./.dlcs-rc')):
    DLCS_CONFIG = abspath('./.dlcs-rc')
else:
    DLCS_CONFIG = expanduser('~/.dlcs-rc')

def prettify_json(json, level=0):

    """Formats a JSON string to separated, indented lines.
    """
    #TODO: prettify json
    return json

    lines = []
    prefix = '\t'*level
    line_buffer = ''

    for c in json:
        if c in ',':
            lines.append(prefix + line_buffer + c)
            line_buffer = ''

        elif c in '[{(':
            for line in prettify_json:
                lines.append()

        else:
            line_buffer += c

    return "\n".join(lines)

# TODO: write output formatting for text
def output_text(data):
    return "\n\n".join([txt_post(p) for p in data])

def output_rst(data):
    return "\n\n".join([rst_post(p) for p in data])

def output_json(data):
    return jsonwrite(data)

def output_prettyjson(data):
    return prettify_json(jsonwrite(data))

def output(cmd, opts, data):
    return data
    #TODO:
    return globals()['output_'+opts['outf']](data)

### Main

def main(argv):

    """This will prepare al input data and call a command function to perform
    the operations. Default command is `info()`.

    Configuration file is loaded and used to store username/password.
    """

    argv.pop(0) # scriptname

    ### Parse argument vector
    import optionparse
    defaults = {
        'DLCS_CONFIG': DLCS_CONFIG,
        'ENCODING': locale.getpreferredencoding()}
    optparser, opts, args = optionparse.parse(__usage__, argv, defaults=defaults)

    if opts['verboseness']:
        v = int(opts['verboseness'])
        DEBUG = v
        pydelicious.DEBUG = v

    # First argument is command
    if len(args) > 0:
        cmdid = args.pop(0)
    else:
        cmdid = 'info'

    if not cmdid in __cmds__:
        optionparse.exit("Command must be one of %s" % ", ".join(__cmds__))

    ### Parse config file
    conf = ConfigParser()
    conf_file = opts['config']
    conf.read(conf_file)

    # Check for default section
    if not 'dlcs' in conf.sections():
        if not 'username' in opts or not 'password' in opts:
            if not 'username' in opts:
                opts['username'] = os.getlogin()

            if not 'password' in opts:
                opts['password'] = getpass.getpass("Password for %s: " % opts['username'])

            v = raw_input("Save username, password and other defaults to config (%s)? [Y]es/No: " % conf_file)
            if v in ('y', 'Y', ''):
                conf.add_section('dlcs')
                conf.set('dlcs', 'username', opts['username'])
                conf.set('dlcs', 'password', opts['password'])
                # Other default settings:
                conf.add_section('local-files')
                conf.set('local-files', 'tags', expanduser("~/.dlcs-tags.xml"))
                conf.set('local-files', 'posts', expanduser("~/.dlcs-posts.xml"))
                conf.write(open(conf_file, 'w'))
                return "Config written. Just run dlcs again or review the default config first."

            else:
                return "Aborted"

        options = opts

    else:
        ### Merge config items under 'dlcs' with opts
        # conf provides defaults, command line options override
        options = dict(conf.items('dlcs'))
        options.update(opts)

    # Force output encoding
    sys.stdout = codecs.getwriter(options['encoding'])(sys.stdout)

    # DeliciousAPI instance to pass to the command functions
    dlcs = DeliciousAPI(options['username'], options['password'],
        codec=options['encoding'])

    ### Defer processing to command function
    cmd = getattr(sys.modules[__name__], cmdid)
    try:
        return cmd(conf, dlcs, *args, **options)
    except PyDeliciousException, e:
        print >> sys.stderr, e


### Command functions

def help(conf, dlcs, cmd='', **opts):

    """Prints the docstring for a command or DeliciousAPI method.
    """

    thismod = sys.modules['__main__']

    if cmd == 'api':
        print "Available API paths: %s " % (DeliciousAPI.paths.keys(),)

    elif cmd in DeliciousAPI.paths.keys():
        # cmd is an API path
        print DeliciousAPI.paths[cmd].__doc__

    elif not cmd:
        print thismod.__doc__

    elif not hasattr(thismod, cmd):
        print "No such command or API path: %s" % (cmd,)

    elif not hasattr(getattr(thismod, cmd), '__doc__'):
        print "No docstring for %s" % (cmd,)

    else:
        print getattr(thismod, cmd).__doc__

def info(conf, dlcs, **opts):

    """Default command.
    """

    u = dlcs.posts_update()['update']['time']
    print "Posts last updated on: %s (UTC)" % time.strftime("%c", u)

    posts_file = conf.get('local-files', 'posts')
    if exists(posts_file):
        postsupd = getmtime(posts_file)
        print "Cached post list on: %s (local)" % time.strftime("%c", time.localtime(postsupd))
    else:
        print "Need to cache post list"

    tags_file = conf.get('local-files', 'tags')
    if exists(tags_file):
        tagsupd = getmtime(tags_file)
        print "Cached tag list on: %s (local)" % time.strftime("%c", time.localtime(tagsupd))
    else:
        print "Need to cache tag list"

    if (exists(tags_file) and u > time.gmtime(tagsupd)) or (exists(posts_file) and u > time.gmtime(postsupd)):
        print "Cache is out of date"

def stats(conf, dlcs, **opts):

    """Statistics
    """

    posts = cached_posts(conf, dlcs, opts['keep_cache'])
    tags = cached_tags(conf, dlcs, opts['keep_cache'])

    # @TODO: Some more intel gathering on tags would be nice
    print "Tags: %s" % len(tags['tags'])
    print "Posts: %s" % len(posts['posts'])

    # Tag usage per post
    taggedhigh = 0
    taggedlow = 0
    for post in posts['posts']:
        tags = len(post['tag'].split(' '))
        if not taggedlow or tags < taggedlow:
            taggedlow = tags
        if not taggedhigh or tags > taggedhigh:
            taggedhigh = tags

    print "Tags per post (min/max): %s/%s" % (taggedlow, taggedhigh)

def req(conf, dlcs, path, **opts):

    """Request data from a (URI-)path using pydelicious.DeliciousAPI. E.g.::

        % dlcs req posts/get?tag=energy
        % dlcs req --outf=raw tags/bundles/all
        % dlcs req -d posts/update

    The `raw` option causes the response XML to be printed as JSON, `dump`
    prints the entire HTTP XML response. Note that since the v1 API is not RESTful
    you can change data using this function too. E.g.::

        % dlcs req "tags/bundles/set?bundle=foo&tags=bar%20baz"
        % dlcs req tags/bundles/delete?bundle=foo

    Ofcourse URL encoding and shell-escaping is up to you.
    """

    if 'dump' in opts and opts['dump']:
        fl = dlcs.request_raw(path)
        print http_dump(fl)

    else:
        data = dlcs.request(path)
        print output(`req`, opts, data)

def post(conf, dlcs, url, description, extended, *tags, **opts):

    """Do a standard post to del.icio.us::

        % dlcs post "URL" "DESCRIPTION" "EXTENDED" tag1 tag2...
    """

    tags = " ".join(tags)

    replace = 'no'
    if 'replace' in opts:
        replace = opts['replace']

    shared = 'yes'
    if 'shared' in opts:
        shared = opts['shared']

    v = dlcs.posts_add(replace=replace,
        shared=shared,
        description=description,
        extended=extended,
        url=url,
        tags=tags)

    print '* Post: "%s <%s>": %s' % \
        (description, url, v['result'][1])

def postit(conf, dlcs, url, shared='yes', replace='no', **opts):

    """Create and edit posts.
    """

    assert 'EDITOR' in os.environ, \
        "postit needs the environmental variable 'EDITOR' set"

    description, extended, tags = '', '', []

    # Use ConfigParser as key/value parser
    conf = ConfigParser()
    tmpf = os.tmpnam() + '.ini'
    os.mknod(tmpf)
    tmpfl = open(tmpf, 'w+')

    # Prepare dictionary for use in ini file
    p = { 'description': description, 'extended': extended,
        'tag': " ".join(tags),
        'shared': shared, 'replace': replace, }

    # Look for existing post
    posts = dlcs.posts_get(url=url)
    if posts['posts']:
        p.update(posts['posts'][0])
        p['replace'] = 'Yes'

    # Fill ini file
    conf.add_section(url)
    for key in p:
        conf.set(url, key, p[key])
    conf.write(tmpfl)
    tmpfl.close()

    #Let user edit file
    mtime = os.stat(tmpf)[8]

    os.system("%s %s" % (os.environ['EDITOR'], tmpf))

    if mtime == os.stat(tmpf)[8]:
        return "! No changes, aborted"

    # Parse data back into locals
    conf.read(tmpf)

    opts['shared'] = conf.get(url, 'shared')
    opts['replace'] = conf.get(url, 'replace')

    description = conf.get(url, 'description')
    extended = conf.get(url, 'extended')
    tags = conf.get(url, 'tag').split(' ')
    if conf.has_option(url, 'href'):
        url = conf.get(url, 'href')

    # Let post handle rest of command
    post(conf, dlcs, url, description, extended, *tags, **opts)

def posts(conf, dlcs, **opts):

    """Retrieves ALL posts and prints the URLs.
    """

    posts = cached_posts(conf, dlcs, opts['keep_cache'])
    for post in posts['posts']:
        print post['href']

def postsupdate(conf, dlcs, **opts):

    """Print last update time.
    """

    u = dlcs.posts_update()
    print str(u['update']['time'])

def updateposts(conf, dlcs, **opts):

    """TODO: Retrieve 15 most recent posts and add to local cache,
    (after which it will be considered up-to-date again).
    """

    fl = dlcs.posts_recent(_raw=True)
    print fl
    append_cache(fl, opts)

def getposts(conf, dlcs, *urls, **opts):

    """Print the posts for the given URLs in JSON.
    """

    out = []
    for url in urls:
        posts = dlcs.posts_get(url=url)['posts']

        if not len(posts)>0:
            print >>sys.stderr,"No posts for %s" % (url,)

        else:
            out.extend(posts)

    print output('getposts', opts, out)

def findposts(conf, dlcs, keyword, **opts):

    """Search all text fields of all posts for the keyword and print machting URLs.

        % dlcs findposts keyword
    """

    posts = cached_posts(conf, dlcs, opts['keep_cache'])
    for post in posts['posts']:
        fields = post['tag']+post['href']+post['description']+post['extended']

        if opts['ignore_case']:
            if fields.lower().find(keyword.lower()) > -1:
                print post['href']

        elif fields.find(keyword) > -1:
            print post['href']

def deleteposts(conf, dlcs, *urls, **opts):

    """Delete one or more URLs.
    """

    for url in urls:
        v = dlcs.posts_delete(url)
        print '* Deleted "%s": %s' % (url, v['result'][1])

def recent(conf, dlcs, **opts):

    """Fetch the 15 most recent posts.
    """

    rs = dlcs.posts_recent()
    for post in rs['posts']:
        print post['href']

def rename(conf, dlcs, oldtag, *newtags, **opts):

    """rename a tag to one or more tags.

        % dlcs rename oldtag newtag(s)
    """

    new = " ".join(newtags)
    v = dlcs.tags_rename(oldtag, new)
    if not v['result'][0]:
        print >>sys.stderr, 'Error renaming "%s" to "%s": %s' % (oldtag, new, v['result'][1])
    else:
        print '* "%s" -> "%s": %s' % (oldtag, new, v['result'][1])

def bundle(conf, dlcs, name, *tags, **opts):

    """Bundle some tags under a name, replaces previous bundle contents::

        % dlcs bundle bundlename tag(s)
    """

    tags = " ".join(tags)
    v = dlcs.bundles_set(name, tags)
    print '* "%s" -> "%s" %s' % (name, tags, v['result'][1])

def bundles(conf, dlcs, **opts):

    """Retrieve all bundles and print their names.
    """

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        print bundle['name'],

    print

def getbundle(conf, dlcs, name, **opts):

    """Retrieve all tags within a bundle.
    """

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            print bundle['tags']
            return

def deletebundle(conf, dlcs, name, **opts):

    """Delete an entire bundle.
    """

    v = dlcs.bundles_delete(name)
    print '* delete bundle "%s": %s' % (name, str(v))

def bundleadd(conf, dlcs, name, *tags, **opts):

    """Add one or more tags to a bundle. Retrieves current bundles, adds the
    tags to the indicated bundle and posts it back to del.icio.us::

        % dlcs bundleadd bundlename tag(s)
    """

    tags = " ".join(tags)

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            tags += ' '+bundle['tags']
            v = dlcs.bundles_set(name, tags)
            print '* "%s" -> "%s": %s' % (name, tags, v['result'][1])
            return

def bundleremove(conf, dlcs, name, *tags, **opts):

    """Remove one or more tags from a bundle. Retrieves current bundles, removes
    the tags from the indicated bundle and posts it back to del.icio.us::

        % dlcs bundleremove bundlename tag(s)
    """

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            curcontents = bundle['tags'].split(' ')
            for tag in tags:
                if tag in curcontents: curcontents.remove(tag)
                else: print >>sys.stderr, "%s not in bundle %s" % (tag, name)
            v = dlcs.bundles_set(name, curcontents)
            print '* "%s" -> "%s" %s' % (name,
                ", ".join(curcontents), v['result'][1])
            return

def tag(conf, dlcs, tags, *urls, **opts):

    """Tag all URLs with the given tag(s)::

        % dlcs tag "tag1 tag2" http://... http://...

    This will retrieve the post for each URL, add the given tags and then
    replace the post at del.icio.us. URLs not in the collection cause
    a message to stderr and are ignored.
    """

    for url in urls:
        posts = dlcs.request('posts/get', url=url)
        if not posts['posts']:
            print >>sys.stderr, '* URL "%s" not in collection' % (url)

        else:
            post = posts['posts'][0]
            if not 'extended' in post:
                post['extended'] = ""
            if not 'tag' in post:
                post['tag'] = ""
            if not 'shared' in post:
                post['shared'] = "True"

            # XXX: del.icio.us takes care of duplicates...
            post['tag'] += ' '+tags

            v = dlcs.posts_add(replace="yes",
                shared=post['shared'],
                description=post['description'],
                extended=post['extended'],
                url=post['href'],
                tags=post['tag'],
                time=post['time'])

            print '* tagged "%s" with "%s": %s' % (url,
                post['tag'], v['result'][1])

def untag(conf, dlcs, tags, *urls, **opts):

    """Reverse of tag, remove given tags from the given URLs.
    Tags and URLs not found are ignored.
    """

    for url in urls:
        posts = dlcs.request('posts/get', url=url)
        if not posts['posts']:
            print >>sys.stderr, '* URL "%s" not in collection' % (url)

        else:
            post = posts['posts'][0]
            if not 'extended' in post:
                post['extended'] = ""
            if not 'tag' in post:
                post['tag'] = ""
            if not 'shared' in post:
                post['shared'] = "True"

            tagged = post['tag'].split(' ')
            untagged = []
            for tag in tags:
                if tag in tagged:
                    tagged.remove(tag)
                    untagged.append(tag)
            post['tag'] = " ".join(tagged)

            v = dlcs.posts_add(replace="yes",
                shared=post['shared'],
                description=post['description'],
                extended=post['extended'],
                url=post['href'],
                tags=post['tag'],
                time=post['time'])

            print '* untagged "%s" from "%s": %s' % (" ".join(untagged),
                url, v['result'][1])

def tagged(conf, dlcs, tag, **opts):

    """Request all posts for a tag and print their URLs.

        % dlcs tagged tag
    """

    posts = cached_posts(conf, dlcs, opts['keep_cache'])
    for post in posts['posts']:

        if opts['ignore_case']:
            tags = post['tag'].lower().split(' ')
        else:
            tags = post['tag'].split(' ')

        if opts['ignore_case'] and tag.lower() in tags:
            print post['href']
        elif tag in tags:
            print post['href']

def tags(conf, dlcs, **opts):

    """Print all tags.
    """

    tags = cached_tags(conf, dlcs, opts['keep_cache'])

    for tag in tags['tags']:
        # @XXX: encoding...
        print tag['tag'].encode('utf-8'),

def gettags(conf, dlcs, *tags, **opts):

    """Print info about tag.
    """

    if opts['ignore_case']:
        tags = [t.lower() for t in tags]

    for tag in cached_tags(conf, dlcs, opts['keep_cache'])['tags']:
        if tag['tag'] in findtags or \
                (opts['ignore_case'] and tag['tag'].lower() in findtags):
            print jsonwrite(tag)

def findtags(conf, dlcs, *tags, **opts):

    """Search all tags with (a part of) a tag.
    """

    for tag in cached_tags(conf, dlcs, opts['keep_cache'])['tags']:
        tag = tag['tag']

        for findtag in tags:
            if opts['ignore_case']:
                if tag.lower().find(findtag.lower()) > -1:
                    print tag

            elif tag.find(findtag) > -1:
                print tag

def clearcache(conf, dlcs, *clear, **opts):

    """Delete all locally cached data::

        % dlcs clear [tags | posts]
    """

    if not clear:
        clear = ['tags', 'posts']

    if 'tags' in clear:
        try:
            tags = conf.get('local-files', 'tags')
            os.unlink(tags)
            print "* Deleted '%s'" % tags
        except: pass

    if 'posts' in clear:
        try:
            posts = conf.get('local-files', 'posts')
            os.unlink(posts)
            print "* Deleted '%s'" % posts
        except: pass

### Utils
def http_dump(fl):

    """Format fileobject wrapped in urllib.addinfourl as HTTP message string
    and return.
    """

    return "\r\n".join([
        str(fl.code) +" "+ fl.msg,
        "".join(fl.headers.headers),
        fl.read().strip()])

def cache_file(fn, data):
    open(fn, 'w').write(data.read())

def cache_append_posts(fl, ):
    pass

def cached_tags(conf, dlcs, noupdate=False):
    """Make sure the tag list is cached locally. Updates when the file is
    older than the last time the posts where updated (according to
    del.icio.us posts/update, which only notes new posts, not any updates).
    """
    tags_file = conf.get('local-files', 'tags')
    if not exists(tags_file):
        print >>sys.stderr, "cached_tags: Fetching new tag list..."
        cache_file(tags_file, dlcs.tags_get(_raw=True))
    else:
        if not noupdate:
            lastupdate = dlcs.posts_update()['update']['time']
            if time.gmtime(getmtime(tags_file)) < lastupdate:
                print >>sys.stderr, "cached_tags: Updating tag list..."
                cache_file(tags_file, dlcs.tags_get(_raw=True))
        elif DEBUG: print >>sys.stderr, "cached_tags: Forced read from cached file..."
    tags = dlcs_parse_xml(open(tags_file))
    return tags

def cached_posts(conf, dlcs, noupdate=False):
    """Same as cached_tags but for the post list.
    """
    posts_file = conf.get('local-files', 'posts')
    if not exists(posts_file):
        print >>sys.stderr, "cached_posts: Fetching new post list..."
        cache_file(posts_file, dlcs.posts_all(_raw=True))
    else:
        if not noupdate:
            lastupdate = dlcs.posts_update()['update']['time']
            if time.gmtime(getmtime(posts_file)) < lastupdate:
                print >>sys.stderr, "cached_posts: Updating post list..."
                cache_file(posts_file, dlcs.posts_all(_raw=True))
        elif DEBUG: print >>sys.stderr, "cached_posts: Forced read from cached file..."
    posts = dlcs_parse_xml(open(posts_file))
    return posts


if __name__ == '__main__':
    try:
        sys.exit(main(sys.argv))
    except KeyboardInterrupt:
        print >>sys.stderr, "User interrupt"
# vim:set expandtab:
