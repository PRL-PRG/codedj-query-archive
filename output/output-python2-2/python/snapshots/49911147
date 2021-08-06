
# models.py
from django.contrib.gis.db import models
from django.contrib.gis import admin
from django.contrib import databrowse

class HacksWorld(models.Model):
    cat = models.IntegerField()
    fips_cntry = models.CharField(max_length=80, )
    cntry_name = models.CharField(max_length=80, )
    area = models.DecimalField(max_digits=15, decimal_places=2, help_text="decimal")
    pop_cntry = models.DecimalField(max_digits=15, decimal_places=2, )
    shapes = models.MultiPolygonField(srid=4326)
    objects = models.GeoManager()

    def __unicode__(self): return self.cntry_name

class HacksWorldAdmin(admin.OSMGeoAdmin):
    """
    Note: another common practice is to put the AdminOptions in a file called admin.py
    """
    list_display = ('cntry_name','fips_cntry','cntry_name',)
    search_fields = ('cntry_name',)
    ordering = ('cntry_name',)
    list_filter = ('fips_cntry','cntry_name',)

    # Default OL map options: uncomment and modify as desired
    #default_lon = 0
    #default_lat = 0
    #default_zoom = 4
    #display_wkt = False
    #display_srid = 4326
    #display_projection = True
    #max_zoom = False
    #min_zoom = False
    #units = 'm'
    #max_resolution = False
    #max_extent = False
    #modifiable = True
    #hoverable = False
    #mouse_position = True
    #scale_text = True
    #layerswitcher = True
    #scrollable = True
    #map_width = 600
    #map_height = 400
    #debug = False
    #wms_layer = 'basic'
    #openlayers_url = 'http://openlayers.org/api/2.6/OpenLayers.js'
    #wms_url = 'http://labs.metacarta.com/wms/vmap0'
    #wms_name = 'OpenLayers WMS'
    #map_template = 'gis/admin/osm.html'
    #extra_js = ['http://openstreetmap.org/openlayers/OpenStreetMap.js']
    #num_zoom = 20
    #map_srid = 900913

class World(models.Model):
    cat = models.IntegerField()
    fips_cntry = models.CharField(max_length=80, )
    cntry_name = models.CharField(max_length=80, )
    area = models.DecimalField(max_digits=15, decimal_places=2, help_text="decimal")
    pop_cntry = models.DecimalField(max_digits=15, decimal_places=2, )
    shapes = models.PolygonField(srid=4326)
    objects = models.GeoManager()

    def __unicode__(self): return self.cntry_name
    
admin.site.register(HacksWorld,HacksWorldAdmin)
admin.site.register(World)
databrowse.site.register(HacksWorld)
