""" mod_python site builder

Author: Justin White

wow, this is about a hundred times cleaner with genshi doing template work

"""

import cgi
import Cookie
import os
import time

from genshi.template import MarkupTemplate, TemplateLoader

import journal


base_dir = os.path.dirname(__file__)
template_dir = os.path.join(base_dir, "templates")
content_dir = os.path.join(base_dir, "content")

# datestamp for HTTP headers: LastModified, Expires
http_date_stamp = "%a, %d %b %Y %H:%M:%S GMT"
# datestamp for cookies
cookie_date_stamp = "%a, %d-%b-%Y %H:%M:%S GMT"

# do this outside of handler to take advantage of caching
templateLoader = TemplateLoader(search_path=template_dir, auto_reload=True)

def send_headers(output, headers, status="200 OK"):
	print "Status: %s" % status
	output.write("Status: %s\n" % status)
	for header in headers:
		print header
		output.write(header)
		output.write("\n")
	print
	output.write("\n")

def send_redirect(output, headers, referer):
	headers.append("Location: %s" %referer)
	send_headers(output, headers, "302 Found")

def handler(env, input, output):
	now = time.time()
	headers = []

	# MS Internet Explorer (<= 7) doesn"t understand application/xhtml+xml
	# If the request came from MSIE (<= 7), then use text/html instead
	agent = env.get("HTTP_USER_AGENT", "")
	if "MSIE" in agent:
		headers.append("Content-type: text/html; charset=utf-8")
		#w("User-Agent is IE: %s" % agent)
	else:
		headers.append("Content-type: application/xhtml+xml; charset=utf-8")
		#w("User-Agent is not IE: %s" % agent)

	# set a Date: header. can help caches syncronize (i think)
	headers.append("Date: %s" % time.strftime(http_date_stamp, time.gmtime(time.time())))

	referer = env.get("HTTP_REFERER", "")

	# cookie time!
	cookies = Cookie.SimpleCookie()
	cookies.load(env.get("HTTP_COOKIE", ""))
	user_cookie = cookies.get("user", "")
	session_cookie = cookies.get("session", "")

	#if session_cookie:
		## session is set, but not user, delete session
		#if not user_cookie:
			#session_cookie = ""
			#session_cookie["expires"] = now - 600
			#output.write(cookie.output())
			#session_cookie = ""

	# parse CGI form data
	form = cgi.FieldStorage(environ=env)
	op = form.getfirst("op", "display").lower()

	if op == "login":
		expire_time = time.strftime(cookie_date_stamp, time.gmtime(now + 7 * 24 * 60 * 60))
		# get user from the form, or use the cookie, or the default ""
		if user_cookie:
			user_name = form.getfirst("user", user_cookie.value)
		else:
			user_name = form.getfirst("user", "")
		password = form.getfirst("password", "")
		found_user = ""
		found_password = ""
		if user_name:
			cookies["user"] = user_name
			cookies["user"]["expires"] = expire_time
			passwd_file = file(os.path.join(base_dir, ".htpasswd"))
			for line in passwd_file:
				found_user, found_md5 = line.rstrip().split(":")
				if found_user == user_name:
					break
			if found_md5 == password:
				cookies["session"] = now
				cookies["session"]["expires"] = expire_time
		headers.append(cookies.output())
		send_redirect(output, headers, referer)
		return
	elif op == "logout":
		# clear user and session cookies for a new login
		expire_time = time.strftime(cookie_date_stamp, time.gmtime(now - 7 * 24 * 60 * 60))
		cookies["user"] = ""
		cookies["user"]["expires"] = expire_time
		cookies["session"] = ""
		cookies["session"]["expires"] = expire_time
		headers.append(cookies.output())
		send_redirect(output, headers, referer)
		return
	if op == "dump":
		page = form.getfirst("p", os.path.basename(__file__))
		page_file = os.path.join(base_dir, page)
		try:
			filedata = file(page_file, "r")
		except IOError:
			page_file = os.path.join(base_dir, os.path.basename(__file__))
			filedata = file(page_file, "r")
		else:
			template = MarkupTemplate(
'<pre id="dump" xmlns:py="http://genshi.edgewall.org/">\n\
${filedata}\n\
</pre>'
			)
			stream = template.generate(
				filedata=filedata
			)
			content = stream.render().splitlines()
			title = "File dump: " + page
	else:
		page = form.getfirst("p", "home")
		if page == "journal":
			page_file = os.path.join(base_dir, "journal.py")
			j = journal.Journal(
				form,
				user_cookie,
				session_cookie,
				templateLoader
			)
			content = j.dispatch()
		else:
			# try to open the requested page .htf file
			try:
				page_file = os.path.join(content_dir, page + ".htf")
				content = file(page_file, "r")
			# if not, use home.htf. if it's not there we got bigger probs
			except IOError:
				page = "home"
				page_file = os.path.join(content_dir, page + ".htf")
				content = file(page_file, "r")
		title = page.capitalize()

	# get file mod times for apache and myself
	mod_time = os.stat(page_file)[8]
	# format a nice HTTP style datestamp
	pretty_mod_time = time.strftime(http_date_stamp, time.gmtime(mod_time))
	headers.append("LastModified: %s" % pretty_mod_time)

	send_headers(output, headers)

	# load the template
	template = templateLoader.load("main.xml")
	# call on genshi to do it's template magic
	stream = template.generate(
		title=title,
		user_cookie=user_cookie,
		session_cookie=session_cookie,
		page=page,
		content=content,
		page_file=os.path.basename(page_file),
		mod_time=pretty_mod_time,
	)
	# show it off!
	output.write(stream.render())

	# we done good
	return
