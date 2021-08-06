from django.contrib.gis.db import models
from django.utils.safestring import mark_safe

class WorldBorders(models.Model):
    name = models.CharField(max_length=50, )
    area = models.IntegerField(help_text="requires integer input",)
    pop2005 = models.IntegerField('Population',help_text="Country wide population in 2005",)
    fips = models.CharField('FIPS Code',max_length=2, help_text=mark_safe('<a href="http://www.census.gov/geo/www/fips/fips.html">Federal Information Processing Standard Code</a>'))
    iso2 = models.CharField('2 Digit ISO', max_length=2, help_text=mark_safe('<a href="http://www.iso.org/">International Organization for Standardization</a>' ))
    iso3 = models.CharField('3 Digit ISO', max_length=3, help_text=mark_safe('<a href="http://www.iso.org/">International Organization for Standardization</a>' ))
    un = models.IntegerField('United Nations Code',help_text="requires integer input")
    region = models.IntegerField('Region Code',help_text="requires integer input",)
    subregion = models.IntegerField('Sub-Region Code',help_text="requires integer input",)
    lon = models.DecimalField('Longitude',max_digits=8, decimal_places=3, help_text="requires decimal input", )
    lat = models.DecimalField('Latitude',max_digits=7, decimal_places=3, help_text="requires decimal input", )
    #hacksworld = models.ManyToManyField(HacksWorld, verbose_name="Related HacksWorld")
    geometry = models.MultiPolygonField(srid=4326)
    objects = models.GeoManager()

    class Meta:
        verbose_name_plural = "World Borders"
        
    def __unicode__(self): return self.name



