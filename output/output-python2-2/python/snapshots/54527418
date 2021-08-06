""" mod_python site builder

Author: Justin White

wow, this is about a hundred times cleaner with genshi doing template work

"""

import os
import string
import time

from mod_python import apache, util, Cookie
from genshi.template import TemplateLoader

# use apache's import to get changes without a restart
# change to normal import later when they're more stable
#journal = apache.import_module("journal")
#pictures = apache.import_module("pictures")


base_dir = os.path.dirname(__file__)
content_dir = os.path.join(base_dir, "content")
menu_file = os.path.join(content_dir, "menu.htf")
footer_file = os.path.join(content_dir, "footer.htf")

# needed for dates in any headers; ie: LastModified:, Expires:
http_date_stamp = "%a, %d %b %Y %H:%M:%S GMT"

# do this outside of handler to take advantage of caching
templateLoader = TemplateLoader(search_path=base_dir, auto_reload=True)

def handler(req):
	template = templateLoader.load("main.xml")

	# try to open menu and footer .htf file
	# if something goes wrong, use empty sets
	try:
		menu = file(menu_file)
	except IOError:
		menu = []
	try:
		footer = file(footer_file)
	except:
		footer = []

	# convenience
	w = req.write

	# MS Internet Explorer doesn"t understand application/xhtml+xml.
	# If the request came from MSIE and lie to it, using text/html instead
	agent = req.headers_in["User-Agent"]
	if "MSIE" in agent:
		req.content_type = "text/html; charset=utf-8"
		#w("User-Agent is IE: %s" % agent)
	else:
		req.content_type = "application/xhtml+xml; charset=utf-8"
		#w("User-Agent is not IE: %s" % agent)

	# parse CGI form data
	form = util.FieldStorage(req)
	page = form.getfirst("p", "home")

	# set a Date: header. can help caches syncronize (i think)
	req.headers_out["Date"] = time.strftime(
		http_date_stamp,
		time.gmtime(time.time())
	)

	# try to open the requested page .htf file
	try:
		page_file = os.path.join(content_dir, page + ".htf")
		content = file(page_file, "r")
	# if not, use home.htf. if it's not there we got bigger probs, let it trace
	except IOError:
		page_file = os.path.join(content_dir, "home.htf")
		content = file(page_file, "r")

	# get file mod times for apache and myself
	mtime = os.stat(page_file)[8]
	# update mtime and let apache handle the Expires: header
	req.update_mtime(mtime)
	# same with LastModified:
	req.set_last_modified()

	# format a nice HTTP style datestamp
	pretty_mtime = time.strftime(http_date_stamp, time.gmtime(mtime))

	# call on genshi to do it's template magic
	stream = template.generate(
		page=page,
		menu=menu,
		content=content,
		mtime=pretty_mtime,
		footer=footer
	)
	# show it off!
	req.write(stream.render())

	# we done good
	return apache.OK
