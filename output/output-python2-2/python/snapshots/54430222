#!/usr/bin/env python

from google.appengine.ext import db

class ExtendedModel(db.Model):
  def key_id(self):
    return self.key().id()

class Category(ExtendedModel):
  user = db.UserProperty(required=True)
  name = db.CategoryProperty(required=True)

class Contact(ExtendedModel):
  user = db.UserProperty(required=True)
  enabled = db.BooleanProperty(required=True, default=False)
  register_key = db.StringProperty(required=True)

class Phone(Contact):
  phone_number = db.PhoneNumberProperty(required=True)

class Email(Contact):
  email = db.EmailProperty(required=True)

class TodoItem(ExtendedModel):
  user = db.UserProperty(required=True)
  ctime = db.DateTimeProperty(auto_now_add=True, required=True)
  mtime = db.DateTimeProperty(auto_now=True, required=True)
  title = db.StringProperty(required=True, multiline=True)
  body = db.StringProperty(multiline=True)
  category = db.ReferenceProperty(Category)
  completed = db.BooleanProperty(default=False)
  archived = db.BooleanProperty(default=False)
