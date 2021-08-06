#!/usr/bin/env python

import hmac
from google.appengine.ext import webapp
from google.appengine.ext.webapp import util
from google.appengine.ext.webapp import template
from google.appengine.api import mail
from models import TodoItem, Phone, GetDefaultCategory, Category
from private import HMAC_KEY

class TextMarkRequest(webapp.RequestHandler):
    def post(self): return self.redirect("/")
    def parse_request(self):
        if not hasattr(self, "phone_number"):
            self.phone_number = self.request.get('phone')
            self.keyword = self.request.get('keyword')
            self.time = self.request.get('time')
            self.msg = self.request.get('msg')
    def invalid_request(self):
        self.parse_request()
        return (hmac.new(HMAC_KEY, self.keyword + self.phone_number +
                self.time).hexdigest() != self.request.get('hash'))

class New(TextMarkRequest):
    def get(self):
        if self.invalid_request(): return self.redirect("/")

        if self.keyword != "TODLO":
            return self.response.out.write("Unknown keyword! Try TODLO")

        phone_numbers = list(Phone.all().filter('phone_number =',
                self.phone_number).filter('enabled =', True))
        if len(phone_numbers) > 1:
            mail.send_mail(sender="jtolds@gmail.com", to="jtolds@gmail.com",
                    subject="new: too many phone numbers!",
                    body=self.phone_number)
            return self.response.out.write("Error! This phone number in system "
                    "twice!")
        if len(phone_numbers) < 1:
            if self.phone_number in ("Simulator", "Anonymous"):
                return self.response.out.write("Please visit "
                        "todlo.appspot.com and sign up!")
            else:
                return self.response.out.write("Please register %s at "
                        "todlo.appspot.com." % self.phone_number)

        if len(self.msg.strip()) <= 0:
            return self.response.out.write("No todo item provided.")

        try:
            cat_obj = list(Category.all().filter("user =",
                    phone_numbers[0].user).filter("name =",
                    GetDefaultCategory(phone_numbers[0].user)).fetch(1))[0]
        except:
            cat_obj = None

        TodoItem(
                user=phone_numbers[0].user,
                title=self.msg,
                body="from %s" % self.phone_number,
                category=cat_obj
            ).put()

        return

class Activate(TextMarkRequest):
    def get(self):
        if self.invalid_request(): return self.redirect("/")
        if self.keyword != "ACTIVATETODLO":
            return self.response.out.write("Unknown keyword! Try TODLO")

        if len(list(Phone.all().filter('phone_number =',
                self.phone_number).filter("enabled =", True))):
            return self.response.out.write("Hmm, this phone number already "
                    "active.")

        register_key = self.msg.strip().upper()

        phone_numbers = list(Phone.all().filter('phone_number =',
                self.phone_number).filter("register_key =", register_key))
        if len(phone_numbers) > 1:
            mail.send_mail(sender="jtolds@gmail.com", to="jtolds@gmail.com",
                    subject="activate: too many phone numbers!",
                    body=self.phone_number)
            return self.response.out.write("Error! This phone number and "
                    "register key in system twice!")
        if len(phone_numbers) < 1:
            if self.phone_number in ("Simulator", "Anonymous"):
                return self.response.out.write("Please visit "
                        "todlo.appspot.com and sign up!")
            else:
                return self.response.out.write("Please register %s at "
                        "todlo.appspot.com." % self.phone_number)

        phone_numbers[0].enabled = True
        phone_numbers[0].put()

        return self.response.out.write("Activation successful!")

class Redirect(webapp.RequestHandler):
    def get(self):
        return self.redirect("/")
    post = get

def main():
    app = webapp.WSGIApplication([
            ('/textmarks/new/*', New),
            ('/textmarks/activate/*', Activate),
            ('.*', Redirect),
        ], debug=True)
    util.run_wsgi_app(app)

if __name__ == '__main__':
    main()
