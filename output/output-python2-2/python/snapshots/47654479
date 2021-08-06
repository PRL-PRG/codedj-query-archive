from django.conf.urls.defaults import *
from django.views.generic.create_update import create_object, update_object, delete_object
from models import DjangoApp
from djangoapps import views
from copy import deepcopy


create_dict = {
    'model': DjangoApp,
    'login_required': True,
}
update_dict = create_dict
delete_dict = deepcopy(create_dict)
delete_dict['post_delete_redirect'] = '/'

urlpatterns = patterns('',
    url(r'^list/popular/$', views.popular_list, name="da_popular_list"),
    url(r'^list/hot/$', views.hot_list, name="da_hot_list"),
    url(r'^list/new/$', views.new_list, name="da_new_list"),
    url(r'^hotclub/$', views.hotclub, name="da_hotclub"),
    url(r'^detail/(?P<slug>[a-zA-Z0-9_-]+)/$', views.detail, name="da_detail"),

    url(r'^create/$', djangoapp_create, name="da_create"),
    url(r'^update/(?P<slug>[a-zA-Z0-9_-]+)/$', update_object, update_dict, name="da_update"),
    url(r'^delete/(?P<slug>[a-zA-Z0-9_-]+)/$', delete_object, delete_dict, name="da_delete"),
    
    url(r'^login/$', 'django.contrib.auth.views.login', {'template_name': 'djangoapps/login.html'},name="da_login"),
    url(r'^logout/$', views.logout_view, name="da_logout"),

    url(r'^profile/(?P<username>[a-zA-Z0-9_-]+)/$', user_profile, name="da_userprofile"),   

    url(r'^$', views.index, name="da_index"),
)
