# Create your views here.

from sqlalchemy import orm
from django.shortcuts import render_to_response
from dlt.downloader.models_sa2 import DownloadItem

def list_items(request):
    dbsession = orm.create_session()
    query = dbsession.query(DownloadItem)
    items = query.select()
    context = {'items': items, "is_sqlalchemy": True}
    return render_to_response('list_items.html', context)
