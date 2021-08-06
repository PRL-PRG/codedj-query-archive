from hype._hype import __herequired__
from hype._hype import EST_VERSION
_vers = __herequired__.split('.')
_estvers = EST_VERSION.split('.')
for a, b in zip(map(int, _vers), map(int, _estvers)):
    if a > b:
        raise Exception("Required HyperEstraier Version %s" % (__herequired__,))

from hype._hype import Database, Document, Condition, repair

from hype._hype import HyperEstraierError, DBError
from hype._hype import DocumentError, DBRemoveError
from hype._hype import DBFlushError, DBSyncError
from hype._hype import DBOptimizeError, DBEditError
from hype._hype import DocModifyImmutableError

from hype._hype import dt_to_str, dt_from_str

from hype._hype import ESTDBREADER      # open as a reader
from hype._hype import ESTDBWRITER      # open as a writer
from hype._hype import ESTDBCREAT       # a writer creating
from hype._hype import ESTDBTRUNC       # a writer truncating
from hype._hype import ESTDBNOLCK       # open without locking
from hype._hype import ESTDBLCKNB       # lock without blocking
from hype._hype import ESTDBPERFNG      # use perfect N-gram analyzer
from hype._hype import ESTDBCHRCAT      # use character category analyzer
from hype._hype import ESTDBLARGE       # large tuning (more than 300000 documents)
from hype._hype import ESTDBHUGE        # huge tuning (more than 1000000 documents)
from hype._hype import ESTDBSCVOID      # store scores as void
from hype._hype import ESTDBSCINT       # store scores as integer
from hype._hype import ESTDBSCASIS      # refrain from adjustment of scores

from hype._hype import ESTCONDSURE      # check every N-gram key
from hype._hype import ESTCONDUSUAL     # check N-gram keys skipping by one
from hype._hype import ESTCONDFAST      # check N-gram keys skipping by two
from hype._hype import ESTCONDAGITO     # check N-gram keys skipping by three
from hype._hype import ESTCONDNOIDF     # without TF-IDF tuning
from hype._hype import ESTCONDSIMPLE    # with the simplified phrase
from hype._hype import ESTCONDSCFB      # feed back scores (for debug)

from hype._hype import ESTPDCLEAN       # clean up dispensable regions
from hype._hype import ESTPDWEIGHT      # weight scores statically when indexing

from hype._hype import ESTGDNOATTR      # no attributes
from hype._hype import ESTGDNOTEXT      # no text

from hype._hype import ESTOPTNOPURGE    # omit purging dispensable region of deleted
from hype._hype import ESTOPTNODBOPT    # omit optimization of the database files

from hype._hype import ESTODCLEAN       # clean up dispensable regions

from hype._hype import ESTIDXATTRSEQ    # for multipurpose sequencial access method
from hype._hype import ESTIDXATTRSTR    # for narrowing with attributes as strings
from hype._hype import ESTIDXATTRNUM    # for narrowing with attributes as numbers

from hype._hype import ESTENOERR        # no error
from hype._hype import ESTEINVAL        # invalid argument
from hype._hype import ESTEACCES        # access forbidden
from hype._hype import ESTELOCK         # lock failure
from hype._hype import ESTEDB           # database problem
from hype._hype import ESTEIO           # I/O problem
from hype._hype import ESTENOITEM       # no item
from hype._hype import ESTEMISC         # miscellaneous

from hype._hype import ESTRPSTRICT      # perform strict consistency check
from hype._hype import ESTRPSHODDY      # omit consistency check

from hype._hype import ESTMGCLEAN       # clean up dispensable regions

from hype._hype import ESTECLSIMURL     # eclipse considering similarity and URL
from hype._hype import ESTECLSERV       # eclipse on server basis
from hype._hype import ESTECLDIR        # eclipse on directory basis
from hype._hype import ESTECLFILE       # eclipse on file basis

