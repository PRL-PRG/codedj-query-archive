# Create your views here.

from django.shortcuts import render_to_response, get_list_or_404
from django.contrib.gis.utils import GeoIP


def whereami(request):
	g = GeoIP()
	remote_ip = request.META['REMOTE_ADDR']
	remote_location = g.city(remote_ip)
	
	return render_to_response('whereami.html', {'remote_location': remote_location})
