from simpleevent import Event


def handler1(msg):
	print("1: %s" % msg)


def handler2(msg):
	print("2: %s" % msg)

ontestevent = Event()
ontestevent += handler1
ontestevent += handler2

ontestevent("test")

