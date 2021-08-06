from django import oldforms as forms
from models import DjangoApp

class DjangoAppForm(forms.ModelForm):
    

    class Meta:
        model = DjangoApp
        exclude = ('author', 'date_added','is_public','is_hotclub')

class DjangoAppContributionApp(forms.Form):
    name = forms.CharField(max_length=128)
    description = forms.CharField(widget=forms.Textarea)
    long_description = forms.CharField(widget=forms.Textarea)
    homepage = forms.CharField(max_length=128)
    license = forms.CharField(max_length=128)
    version = forms.CharField(max_length=50)
    download_url = forms.URLField()

class ContributorsForm(forms.Form):
    email = forms.EmailField()