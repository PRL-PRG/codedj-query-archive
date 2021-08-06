""" mod_python site builder

Author: Justin White
Created: 2006-05-08
"""

import os
import string
import time

from mod_python import apache, util, Cookie
import genshi
from genshi.template import TemplateLoader

journal = apache.import_module("journal")
#pictures = apache.import_module("pictures")


config_dir = "/home/www/config/"
base_dir = os.path.dirname(__file__)
content_dir = os.path.join(base_dir, "content")
menu_file = os.path.join(content_dir, "menu.htf")
footer_file = os.path.join(content_dir, "footer.htf")
http_date_stamp = "%a, %d %b %Y %H:%M:%S GMT"

templateLoader = TemplateLoader(search_path=base_dir, auto_reload=True)

def handler(req):
	template = templateLoader.load("main.xml")

	menu = file(menu_file)
	footer = file(footer_file)

	w = req.write
	"""MS Internet Explorer doesn"t understand application/xhtml+xml.
	If the request came from MSIE and lie to it, using text/html instead"""
	agent = req.headers_in["User-Agent"]
	#if "MSIE" in agent:
	if True:
		req.content_type = "text/html; charset=utf-8"
		#w("User-Agent is IE: %s" % agent)
	else:
		req.content_type = "application/xhtml+xml; charset=utf-8"
		#w("User-Agent is not IE: %s" % agent)

	form = util.FieldStorage(req)
	page = form.getfirst("p", "home")

	req.headers_out["Date"] = time.strftime(http_date_stamp, time.gmtime(time.time()))

	try:
		page_file = os.path.join(content_dir, page + ".htf")
		content = file(page_file, "r")
	except IOError:
		page_file = os.path.join(content_dir, "home.htf")
		content = file(page_file, "r")

	mtime = os.stat(page_file)[8]
	req.update_mtime(mtime)
	req.set_last_modified()

	pretty_mtime = time.strftime(http_date_stamp, time.localtime(mtime))

	stream = template.generate(
		page=page,
		menu=menu,
		content=content,
		mtime=pretty_mtime,
		footer=footer
	)
	req.write(stream.render())

	return apache.OK
