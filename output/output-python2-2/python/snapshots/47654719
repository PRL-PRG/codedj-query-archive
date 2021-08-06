from django.conf.urls.defaults import *
from django.views.generic.list_detail import object_list
from djangoapps import views

urlpatterns = patterns('',
    url(r'^list/popular/$', views.popular, name="da_popular"),
    url(r'^list/hot/$', views.hot, name="da_hot"),
    url(r'^list/new/$', views.new, name="da_new"),
    url(r'^detail/(?P<slug>[a-zA-Z0-9_-]+)/$', views.detail, name="da_detail"),
    url(r'^submit/$', views.submit, name="da_submit"),
    url(r'^/$', views.index, name="da_index"),
)