from os.path import join, dirname, abspath, expanduser, isdir
import sys
import imp

if (hasattr(sys, "frozen") or hasattr(sys, "importers") or imp.is_frozen("__main__")):
    ROOT_PATH = abspath(join(dirname(sys.executable), '..'))
else:
    ROOT_PATH = abspath(join(dirname(__file__), '..'))

ETC_PATH = join(ROOT_PATH, 'etc')

MAIN_CONFIG_PATH = join(ETC_PATH, 'config.xml')

SPLASH_TIMEOUT = 0

VERSIONS_PATH = 'versions'
DIAGRAMS_PATH = 'diagrams'
ELEMENTS_PATH = 'elements'
CONNECTIONS_PATH = 'connections'
ICONS_PATH = 'icons'

ARROW_IMAGE = 'arrow.png'

DEFAULT_TEMPLATE_ICON = 'default_icon.png'
SPLASH_IMAGE = 'splash.png'
STARTPAGE_IMAGE = 'startpage.png'
GRAB_CURSOR = 'grab.png'
GRABBING_CURSOR = 'grabbing.png'

PROJECT_EXTENSION = '.frip'
PROJECT_TPL_EXTENSION = '.frit'

METAMODEL_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/metamodel.xsd}'
UMLPROJECT_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/umlproject.xsd}'
RECENTFILES_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/recentfiles.xsd}'
CONFIG_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/config.xsd}'

WEB = 'http://umlfri.kst.fri.uniza.sk/'
MAIL = 'projekt@umlfri.kst.fri.uniza.sk'
DEBUG=False

LABELS_CLICKABLE = False # Used to ignore labels at drawing area
