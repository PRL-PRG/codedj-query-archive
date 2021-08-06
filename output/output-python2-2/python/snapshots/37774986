from os.path import join, dirname, abspath, expanduser, isdir
import sys
import imp

if (hasattr(sys, "frozen") or hasattr(sys, "importers") or imp.is_frozen("__main__")):
    ROOT_PATH = abspath(join(dirname(sys.executable), '..'))
else:
    ROOT_PATH = abspath(join(dirname(__file__), '..'))

ROOT_PATH = ROOT_PATH.decode(sys.getfilesystemencoding())

ETC_PATH = join(ROOT_PATH, 'etc')

MAIN_CONFIG_PATH = join(ETC_PATH, 'config.xml')

SPLASH_TIMEOUT = 0

VERSIONS_PATH = 'versions'
DIAGRAMS_PATH = 'diagrams'
ELEMENTS_PATH = 'elements'
CONNECTIONS_PATH = 'connections'
ICONS_PATH = 'icons'
DOMAINS_PATH = 'domains'

ARROW_IMAGE = 'arrow.png'

DEFAULT_TEMPLATE_ICON = 'default_icon.png'
SPLASH_IMAGE = 'splash.png'
STARTPAGE_IMAGE = 'startpage.png'
GRAB_CURSOR = 'grab.png'
GRABBING_CURSOR = 'grabbing.png'
# extensions
PROJECT_EXTENSION = '.frip'
PROJECT_TPL_EXTENSION = '.frit'
PROJECT_CLEARXML_EXTENSION ='.fripx'

METAMODEL_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/metamodel.xsd}'
UMLPROJECT_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/umlproject.xsd}'
RECENTFILES_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/recentfiles.xsd}'
CONFIG_NAMESPACE = '{http://umlfri.kst.fri.uniza.sk/xmlschema/config.xsd}'

# UML. FRI server - web page, mail address and address for error logs
WEB = 'http://umlfri.kst.fri.uniza.sk/'
MAIL = 'projekt@umlfri.kst.fri.uniza.sk'
ERROR_LOG_ADDRESS = 'http://umlfri.kst.fri.uniza.sk/errors/log.php'  

DEBUG = True                    # turn DEBUG to true for some more information, e.g. user exceptions will be shown with traceback
ERROR_TO_CONSOLE = False        # only if DEBUG is true, instead of showing the exception in a window it will be printed to console
 
LABELS_CLICKABLE = True         # used to ignore labels at drawing area

# options for zoom 
SCALE_MAX = 5.0
SCALE_MIN = 0.6
SCALE_INCREASE = 0.2
BUFFER_SIZE=(2000.0,1500.0)         # buffer size at the start
BUFFER_MAX_SIZE=(6400.0,6400.0)     # the graphic buffer will be extended to max this values
