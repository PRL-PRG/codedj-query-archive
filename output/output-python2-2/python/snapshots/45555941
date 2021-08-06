# Create your views here.

from django.shortcuts import render_to_response
from django.views.generic.create_update import create_object
from dlt.downloader import models

def list_items(request):
    items = models.DownloadItem.objects.all()
    context = {'items': items}
    return render_to_response('list_items.html', context)

def create_item(*args, **kwargs):
    return create_object(*args, **kwargs)