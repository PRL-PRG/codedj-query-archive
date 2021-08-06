from google.appengine.ext import webapp
from django.template.defaultfilters import stringfilter

register = webapp.template.create_template_register()

def split(value, arg=""):
    return value.split(arg)
split = stringfilter(split)

register.filter(split)
