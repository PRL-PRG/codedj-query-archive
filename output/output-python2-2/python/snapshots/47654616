from django.db import models
from django.template import defaultfilters
from django.utils.translation import ugettext_lazy as _
from django.core.urlresolvers import reverse
import datetime

class Author(models.Model):
    name = models.CharField(max_length=128, unique=True, verbose_name=_("Name"))
    slug = models.SlugField(max_length=128, unique=True, verbose_name=_("Slug"))
    email = models.EmailField(blank=True)

class DjangoApp(models.Model):
    PYPI, GOOGLE, SOURCEFORGE = 1, 2, 3
    CODE_STORE_CHOICES = (
        (PYPI, 'PyPI'),
        (GOOGLE, 'Google Code'),
        (SOURCEFORGE, 'Sourceforge'),
    )    
    name = models.CharField(max_length=128, unique=True, verbose_name=_("Name"))
    slug = models.SlugField(max_length=128, unique=True, verbose_name=_("Slug"))
    description = models.TextField(verbose_name=_("Description"))
    long_description = models.TextField(blank=True, verbose_name=_("Long Description"))
    homepage = models.URLField(verify_exists=True, verbose_name=_("Home Page"))
    code_store = models.CharField(max_length=128, choices=CODE_STORE_CHOICES)
    license = models.CharField(max_length=128, blank=True, verbose_name=_("License"))
    date_added = models.DateTimeField(default=datetime.datetime.now, verbose_name=_("Date Added"))
    is_public = models.BooleanField(default=True, verbose_name=_("Is Public"))
    is_hotclub = models.BooleanField(default=False, verbose_name=_("Is a Hotclub Application"))
    version = models.CharField(max_length=50, blank=True, verbose_name=_("Version"))
    download_url = models.URLField(verbose_name=_("Download URL"))
    author = models.ForeignKey(Author)
    
    def save(self):
        if not self.slug:
            self.slug = defaultfilters.slugify(self.name)
        super(DjangoApp, self).save()
    
    def get_absolute_url(self):
        return reverse("da_detail", kwargs={'slug': self.slug})
    
    def __unicode__(self):
        return self.name
    
    class Admin:
        list_display = ('name', 'homepage', 'date_added', 'is_public')
        search_fields = ('name', 'description')
    
    class Meta:
        verbose_name = _("Reusable Django Application")
        verbose_name_plural = _("Reusable Django Applications")
