#!/usr/bin/env python
# -*- coding: koi8-r -*- 
# twistedpythy_app.py: application for TwistedPythy
# Pythy <the.pythy@gmail.com>

from twisted.application import internet, service
from TwistedPythy import proto, clients

config = {
'listen-port' : 3000,		# порт, который нужно слушать
'delay': 5,		# задержка перед ответом, в сек.
'encoding': 'utf-8',	# кодировка транспорта
}

# -- всё как раньше, определяем клиента и тип фабрики
client = clients.UnicodeDummyClient()
client.pause_time = config['delay']
factory = proto.AsyncUnicodePythyFactory(client, config['encoding'])

# создаем приложение
application = service.Application("TwistedPythy")
# создаем один (в приложении м.б. несколько сервисов) сервис
tp_service = internet.TCPServer(config['listen-port'], factory)
# добавляем сервис в приложение
tp_service.setServiceParent(application)

