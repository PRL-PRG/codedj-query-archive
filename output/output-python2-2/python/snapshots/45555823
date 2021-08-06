from django.conf.urls.defaults import *
from django.views.generic.create_update import create_object

from dlt.downloader import views, models

extra_data = {
      'model': models.DownloadItem,
      'template_name': 'create_item.html',
      'post_save_redirect': '..',
              }

urlpatterns = patterns('',
    # Example:
    (r'^$', views.list_items),
    (r'^create/$', views.create_item, extra_data)
)

