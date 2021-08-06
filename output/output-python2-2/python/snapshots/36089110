#!/usr/bin/python -u
"""dlcs - Operate on a del.icio.us bookmark collection from the command-line.

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

Configuration
-------------
Your username and password can be stored in an INI formatted configuration
file under the section name 'dlcs'. The default location is ~/.dlcs-rc
but this can be changed using command line options. If no username or
password are provided `dlcs` will guess the username and prompt for the
password.

TODO
----
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
        print >>sys.stderr, "No JSON decoder installed, using standard Python printing"
        jsonwrite = sys.stdout.write

__cmds__ = [
    'req',
    'info',
    'stats',
    'posts',
    'postsupdate',
    'updateposts',
    'getposts',
    'findposts',
    'deleteposts',
    'recent',
    'tagged',
    'rename',
    'tags',
    'tag',
    'untag',
    'gettags',
    'findtag',
    'bundle',
    'bundles',
    'getbundle',
    'deletebundle',
    'bundleadd',
    'bundleremove',
    'clearcache']
__all__ = __cmds__ + ['main', 'parse_argv', 'http_dump']

DEBUG = 0
if 'DLCS_DEBUG' in os.environ:
	DEBUG = int(os.environ['DLCS_DEBUG'])

if 'DLCS_CONFIG' in os.environ:
    DLCS_CONFIG = os.environ['DLCS_CONFIG']
elif exists(abspath('./.dlcs-rc')):
    DLCS_CONFIG = abspath('./.dlcs-rc')
else:
    DLCS_CONFIG = expanduser('~/.dlcs-rc')


### Main

def main(argv):
    """This will prepare al input data and call a command function to perform
    the operations. Default command is `info()`.

    Arguments are parsed by parse_argv().

    Configuration file is loaded and used to store username/password.
    """
    ### Parse argument vector
    optparser, opts, args = parse_argv(argv)
    args.pop(0) # scriptname

    # First argument is command
    if len(args) > 0:
        cmdid = args.pop(0)
    else:
        cmdid = 'info'

    if not cmdid in __cmds__:
        #optparser.error("! cmd must be one of %s" % ", ".join(__cmds__))
        return "Command must be one of %s" % ", ".join(__cmds__)

    ### Parse config file
    conf = ConfigParser()
    conf_file = opts['conf_file']
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

    ### Defer processing to command function
    cmd = getattr(sys.modules[__name__], cmdid)
    try:
        return cmd(conf, args, **options)
    except PyDeliciousException, e:
        print >> sys.stderr, e


### Command functions

def info(conf, args, **opts):
    """Default command.
    """
    if args: return "'info' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

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

def stats(conf, args, **opts):
    """Statistics
    """
    if args: return "'stats' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    posts = cached_posts(conf, dlcs, opts['keep_cached'])
    tags = cached_tags(conf, dlcs, opts['keep_cached'])

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

def req(conf, args, **opts):
    """Request data from a (URI-)path using pydelicious.DeliciousAPI. E.g.::

        % dlcs req posts/get?tag=energy
        % dlcs req --raw tags/bundles/all
        % dlcs req -d posts/update

    The `raw` option causes the bare XML response to be printed, `dump`
    prints the entire HTTP response. Note that since the v1 API is not RESTful
    you can change data using this function too. E.g.::

        % dlcs req "tags/bundles/set?bundle=foo&tags=bar%20baz"
        % dlcs req tags/bundles/delete?bundle=foo

    Ofcourse URL encoding and shell-escaping is up to you.
    """
    if args == []:
        print >>sys.stderr, "! Argument indicating path (within API) required."
        return 1

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    if opts['raw']:
        fl = dlcs.request_raw(args[0])
        print fl.read()

    elif opts['dump']:
        fl = dlcs.request_raw(args[0])
        print http_dump(fl)

    else:
        print jsonwrite(dlcs.request(args[0]))

def post(conf, args, **opts):
    """Do a standard post to del.icio.us::

        % dlcs post URL DESCRIPTION EXTENDED tag1 tag2...
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    url = args.pop(0)
    description = args.pop(0)
    extended = args.pop(0)

    v = dlcs.posts_add(replace="yes",
        shared="True",
        description=description,
        extended=extended,
        url=url,
        tags=args)

    print '* Posted "%s <%s>": %s' % (description, url, v['result'][1])

def posts(conf, args, **opts):
    """Retrieves ALL posts and prints the URLs.
    """
    if args: return "'posts' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    posts = cached_posts(conf, dlcs, opts['keep_cached'])
    for post in posts['posts']:
        print post['href']

def postsupdate(conf, args, **opts):
    """Print last update time.
    """
    if args: return "'postsupdate' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])
    u = dlcs.posts_update()
    print str(u['update']['time'])

def updateposts(conf, args, **opts):
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])
    rs = dlcs.posts_recent()
    append_cache(rs, opts)
    print rs

def getposts(conf, args, **opts):
    """Print the posts for the given URLs in JSON.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    out = []
    for url in args:
        out.append(dlcs.posts_get(url=url)['posts'][0])

    print jsonwrite(out)

def findposts(conf, args, **opts):
    """Search all text fields of all posts for the keyword and print machting URLs.

        % dlcs findposts keyword
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    keyword = args.pop(0)

    posts = cached_posts(conf, dlcs, opts['keep_cached'])
    for post in posts['posts']:
        fields = post['tag']+post['href']+post['description']+post['extended']

        if opts['ignore_case']:
            if fields.lower().find(keyword.lower()) > -1:
                print post['href']

        elif fields.find(keyword) > -1:
            print post['href']

def deleteposts(conf, args, **opts):
    """Delete one or more URLs.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    for url in args:
        v = dlcs.posts_delete(url)
        print '* Deleted "%s": %s' % (url, v['result'][1])

def recent(conf, args, **opts):
    """
    """
    if args: return "'recentposts' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    rs = dlcs.posts_recent()
    for post in rs['posts']:
        print post['href']

def rename(conf, args, **opts):
    """rename a tag to one or more tags.

        % dlcs rename oldtag newtag(s)
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    old = args.pop(0)
    new = " ".join(args)
    v = dlcs.tags_rename(old, new)
    if not v['result'][0]:
        print >>sys.stderr, 'Error renaming "%s" to "%s": %s' % (old, new, v['result'][1])
    else:
        print '* "%s" -> "%s": %s' % (old, new, v['result'][1])

def bundle(conf, args, **opts):
    """Bundle some tags under a name, replaces previous bundle contents::

        % dlcs bundle bundlename tag(s)
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    name = args.pop(0)
    tags = " ".join(args)
    v = dlcs.bundles_set(name, tags)
    print '* "%s" -> "%s" %s' % (name, tags, v['result'][1])

def bundles(conf, args, **opts):
    """Retrieve all bundles and print their names.
    """
    if args: return "'bundles' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        print bundle['name'],

    print

def getbundle(conf, args, **opts):
    """Retrieve all tags within a bundle.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    name = args.pop(0)

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            print bundle['tags']
            return

def deletebundle(conf, args, **opts):
    """Delete an entire bundle.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    name = args.pop(0)

    v = dlcs.bundles_delete(name)
    print '* delete bundle "%s": %s' % (name, str(v))

def bundleadd(conf, args, **opts):
    """Add one or more tags to a bundle. Retrieves current bundles, adds the
    tags to the indicated bundle and posts it back to del.icio.us::

        % dlcs bundleadd bundlename tag(s)
    """

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    name = args.pop(0)
    tags = " ".join(args)

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            tags += ' '+bundle['tags']
            v = dlcs.bundles_set(name, tags)
            print '* "%s" -> "%s": %s' % (name, tags, v['result'][1])
            return

def bundleremove(conf, args, **opts):
    """Remove one or more tags from a bundle. Retrieves current bundles, removes
    the tags from the indicated bundle and posts it back to del.icio.us::

        % dlcs bundleremove bundlename tag(s)
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    name = args.pop(0)
    tags = args

    bundles = dlcs.bundles_all()['bundles']
    for bundle in bundles:
        if bundle['name'] == name:
            curcontents = bundle['tags'].split(' ')
            for tag in tags:
                if tag in curcontents: curcontents.remove(tag)
                else: print >>sys.stderr, "%s not in bundle %s" % (tag, name)
            v = dlcs.bundles_set(name, curcontents)
            print '* "%s" -> "%s" %s' % (name, ", ".join(curcontents), v['result'][1])
            return

def tag(conf, args, **opts):
    """Tag all URLs with the given tag(s)::

        % dlcs tag "tag1 tag2" http://... http://...

    This will retrieve the post for each URL, add the given tags and then
    replace the post at del.icio.us. URLs not in the collection cause
    a message to stderr and are ignored.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    tags = args.pop(0)
    urls = args

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

            # @xxx: del.icio.us takes care of duplicates...
            post['tag'] += ' '+tags

            v = dlcs.posts_add(replace="yes",
                shared=post['shared'],
                description=post['description'],
                extended=post['extended'],
                url=post['href'],
                tags=post['tag'],
                time=post['time'])

            print '* tagged "%s" with "%s": %s' % (url, post['tag'], v['result'][1])

def untag(conf, args, **opts):
    """Reverse of tag, remove given tags from the given URLs.
    Tags and URLs not found are ignored.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    #@XXX: no ignore case...

    tags = args.pop(0).split(' ')
    urls = args

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

            print '* untagged "%s" from "%s": %s' % (" ".join(untagged), url, v['result'][1])

def tagged(conf, args, **opts):
    """Request all posts for a tag and print their URLs.

        % dlcs tagged tag
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    tag = args.pop(0)

    posts = cached_posts(conf, dlcs, opts['keep_cached'])
    for post in posts['posts']:

        if opts['ignore_case']:
            tags = post['tag'].lower().split(' ')
        else:
            tags = post['tag'].split(' ')

        if opts['ignore_case'] and tag.lower() in tags:
            print post['href']
        elif tag in tags:
            print post['href']

def tags(conf, args, **opts):
    """Print all tags.
    """
    if args: return "'tags' takes no arguments"

    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    tags = cached_tags(conf, dlcs, opts['keep_cached'])

    for tag in tags['tags']:
        # @XXX: encoding...
        print tag['tag'].encode('utf-8'),

def gettags(conf, args, **opts):
    """Print JSON dictionary for each given tag.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    findtags = args
    if opts['ignore_case']:
        findtags = [t.lower() for t in findtags]

    tags = cached_tags(conf, dlcs, opts['keep_cached'])

    for tag in tags['tags']:
        if tag['tag'] in findtags or \
            (opts['ignore_case'] and tag['tag'].lower() in findtags):
            print jsonwrite(tag)

def findtag(conf, args, **opts):
    """Search all tags with (a part of) a tag.
    """
    dlcs = DeliciousAPI(opts['username'], opts['password'], codec=opts['encoding'])

    findtag = args.pop(0)

    tags = cached_tags(conf, dlcs, opts['keep_cached'])
    for tag in tags['tags']:
        tag = tag['tag']

        if opts['ignore_case']:
            if tag.lower().find(findtag.lower()) > -1:
                print tag

        elif tag.find(findtag) > -1:
            print tag

def clearcache(conf, args, **opts):
    """Delete all locally cached data::

        % dlcs clear [tags | posts]
    """
    if args:
        clear = args
    else:
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

def parse_argv(argv):
    """Parse the argument vector to options and a list of arguments.

    Returns tuple with ConfigParser instance, options dictionary and
    arguments list.
    """

    # construct 'usage' string from all cmd function __doc__'s
    usage = """usage: %% dlcs [options] cmdid [args ...]

commands:
%s""" % "\n".join(["  %s: %s" %
        (cmdid, getattr(sys.modules[__name__], cmdid).__doc__)
        for cmdid in __cmds__])

    parser = optparse.OptionParser(usage)

    # define options
    parser.add_option("-c", "--config", dest="conf_file", default=DLCS_CONFIG,
            help="Use custom config file [%default]")
    parser.add_option("-C", "--cache", dest="keep_cached", action="store_true", default=False,
            help="Don't update locally cached file(s) if they're out of date.")
    parser.add_option("-e", "--encoding", dest="encoding", default=locale.getpreferredencoding(),
            help="Use custom character encoding [locale: %default]")
    parser.add_option("-u", "--user", dest="username",
            help="del.icio.us username (defaults to config or loginname)")
    parser.add_option("-p", "--pass", dest="password",
            help="Password for the del.icio.us user (usage not recommended, but this will override the config)")
    parser.add_option("-I", "--ignore-case", dest="ignore_case", action="store_true", default=False,
            help="Ignore case for string searches")
    parser.add_option("-d", "--dump", dest="dump", action="store_true", default=False,
            help="Print entire HTTP response")

    parser.add_option("-j", "--json", dest="raw", action="store_true", default=False,
            help="Print JSON")

    optsv, args = parser.parse_args(argv)

    args = [a.decode('utf-8', 'replace') for a in args]

    # Convert the optsv instance to a real dictionary...
    opts = {}
    for opt in ('conf_file', 'username', 'password', 'ignore_case', 'raw', 'dump', 'keep_cached', 'encoding'):
        val = getattr(optsv, opt)
        if isinstance(val, bool):
            opts[opt] = val
        elif isinstance(val, basestring):
            opts[opt] = val.decode('utf-8')
        elif val:
            opts[opt] = val

    return parser, opts, args

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

def cache_append_posts(posts, ):
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
        print >>sys.stderr, "Program interrupted"
    else:
        raise
# vim:set expandtab:
