#!/usr/bin/env python

from google.appengine.ext import webapp
from google.appengine.api import users
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from models import TodoItem, Phone, GetDefaultCategory, SetDefaultCategory, \
        Category
import utils

VALID_PHONE_CHARS = "+0123456789"

class Settings(webapp.RequestHandler):
  def get(self):
    user = users.get_current_user()
    logout_url = users.create_logout_url("/")
    phone_numbers = Phone.all().filter('user =', user)
    default_category = GetDefaultCategory(user)
    categories = list(Category.all().filter("user =", user))
    self.response.out.write(template.render("templates/settings.html",
        locals()))

class AddPhone(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard/settings")
    def post(self):
        new_phone = self.request.get('new_phone_number')

        if len(list(Phone.all().filter('phone_number =', new_phone
            ).filter('enabled =', True))) > 0:
          return self.response.out.write("Error: phone number already in use!")

        for char in new_phone:
            if char not in VALID_PHONE_CHARS:
                return self.response.out.write("Error: phone number has "
                        "invalid character '%s'. Phone should be of format "
                        "+18885553333" % char)
        if len(new_phone) < len("+18885553333"):
            return self.response.out.write("Error: phone number is not long "
                    "enough. Phone should be of format +18885553333")

        register_key = utils.register_key()

        Phone(phone_number=new_phone,
            user=users.get_current_user(),
            enabled=False,
            register_key=register_key).put()

        return self.redirect("/dashboard/settings")

class DeletePhone(webapp.RequestHandler):
    def get(self, phone_id): return self.redirect("/dashboard/settings")
    def post(self, phone_id):
        Phone.get_by_id(int(phone_id)).delete()
        return self.redirect("/dashboard/settings")

class DefaultCategory(webapp.RequestHandler):
    def get(self): return self.redirect("/dashboard/settings")
    def post(self):
        user = users.get_current_user()
        SetDefaultCategory(user, self.request.get("default_category"))
        return self.redirect("/dashboard/settings")

class Redirect(webapp.RequestHandler):
  def get(self):
    return self.redirect("/")
  post = get

def main():
  app = webapp.WSGIApplication([
      ('/dashboard/settings/*', Settings),
      ('/dashboard/settings/add_phone/*', AddPhone),
      ('/dashboard/settings/delete_phone/([a-zA-Z0-9]*)/*', DeletePhone),
      ('/dashboard/settings/default_category/*', DefaultCategory),
      ('.*', Redirect),
    ], debug=False)
  util.run_wsgi_app(app)

if __name__ == '__main__':
  main()
