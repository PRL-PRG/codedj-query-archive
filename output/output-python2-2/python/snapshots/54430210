#!/usr/bin/env python

from google.appengine.ext import webapp
from google.appengine.api import users
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from google.appengine.api import mail

class StaticPage(webapp.RequestHandler):
  def post(self): return self.redirect("/")
  def static_page(self, template_path, path):
    user = users.get_current_user()
    if user:
      logout_url = users.create_logout_url(path)
    else:
      login_url = users.create_login_url(path)
    self.response.out.write(template.render(template_path, locals()))

class Landing(StaticPage):
  def get(self):
    self.static_page("templates/landing.html", "/")

class About(StaticPage):
  def get(self):
    self.static_page("templates/about.html", "/about")

class FAQ(StaticPage):
  def get(self):
    self.static_page("templates/faq.html", "/faq")

class Support(StaticPage):
  def get(self):
    self.static_page("templates/support.html", "/support")
  def post(self):
    mail.send_mail(sender="jtolds@gmail.com", to="jtolds@gmail.com",
            subject=self.request.get('subject'),
            body="Sender: %s\n---\n%s" % (
                   self.request.get('sender_email'),
                   self.request.get('message')))
    self.static_page("templates/sent.html", "/support")

def main():
  app = webapp.WSGIApplication([
      ('/', Landing),
      ('/about', About),
      ('/faq', FAQ),
      ('/support', Support),
    ], debug=True)
  util.run_wsgi_app(app)

if __name__ == '__main__':
  main()
