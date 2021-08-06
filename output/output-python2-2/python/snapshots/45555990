"""
URLconf for djangoex
"""
from django.conf.urls.defaults import *
from djangoex.settings import MEDIA_ROOT

#: /static/ and / use django's built-in views: /static/ for representing static content,
#: / for direct template rendering
#: other urls use djangoex.views functions for render pages
urlpatterns = patterns('',
    (r'^static/(?P<path>.*)$', 'django.views.static.serve', {'document_root': MEDIA_ROOT}),
    (r'^$', 'django.views.generic.simple.direct_to_template', {'template': 'index.html'}),
    (r'^forms/$', 'djangoex.views.form'),
    (r'^selfcode/$', 'djangoex.views.selfcode'),
    (r'^dynamic/$', 'djangoex.views.dynamic'),
)
