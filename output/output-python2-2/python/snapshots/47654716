from django.conf.urls.defaults import *
from django.views.generic.list_detail import object_list
from djangoapps import views

urlpatterns = patterns('',
    url(r'^list/popular/$', views.popular_list, name="da_popular_list"),
    url(r'^list/hot/$', views.hot_list, name="da_hot_list"),
    url(r'^list/new/$', views.new_list, name="da_new_list"),
    url(r'^hotclub/$', views.hotclub, name="da_hotclub"),
    url(r'^detail/(?P<slug>[a-zA-Z0-9_-]+)/$', views.detail, name="da_detail"),
    url(r'^submit/$', views.submit, name="da_submit"),
    url(r'^/$', views.index, name="da_index"),
)