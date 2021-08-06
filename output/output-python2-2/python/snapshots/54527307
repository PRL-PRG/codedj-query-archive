import os, random, string, sys
from stat import *
import Image

random.seed()

global depth 

valid_extensions = ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'tiff']
root = '/home/justin/media/pictures'
hroot = '/media/pictures'
nails = '/home/justin/media/nails/'
hnails = '/media/nails/'
thumbs_dir = nails + 'thumbs'
hands = nails + 'hands'
hthumbs_dir = hnails + 'thumbs'
hhands = hnails + 'hands'
use_nails = True;
depth = 0

def dispatch(_req, _form):
	global req, form
	req = _req
	form = _form
	request = form.getfirst('req', '/')
	actual = root + request

	req.write('<div id="content">')

	req.write('<div id="content_title">')
	req.write('<h2>')
	req.write('<a href="./?p=pictures;req=/">Pictures</a>')
	tree = request.split('/')[1:]
	squirrel = ''
	for leaf in tree:
		bud = squirrel + '/' + leaf
		req.write('/<a href="./?p=pictures;req=' + bud + '/">' + leaf + '</a>')
		squirrel = bud
	req.write('</h2>')
	req.write('</div>')

	try:
		info = os.stat(actual)[ST_MODE]
	except:
		req.write('<p>')
		req.write('I can\'t find ' + request)
		req.write('</p>')
		return
	else:
		if S_ISDIR(info):
			# show a directory
			if request[-1] != '/':
				request = request + '/'
			(actual, dirs, files) = os.walk(actual).next()

			if dirs:
				req.write('<div id="dir_list">')
				dirs.sort()
				for dirname in dirs:
					req.write('<div class="item">')
					url = './?p=pictures;req=' + request + dirname
					if use_nails:
						path = request + dirname
						rand_thumb = get_random_thumb_from_dir(path)
						my_thumb = hthumbs_dir + ext_to_jpeg(path + '/' + rand_thumb)
						my_url = '?p=pictures;req=' + path + '/' + rand_thumb
						req.write('<div class="thumb">')
						if do_thumbs(path, rand_thumb):
							req.write('<a href="' + my_url + '"><img alt="' + my_thumb + '/" src="' + my_thumb + '"/></a>')
						else:
							req.write('<div class="broken_nail">Broken Thumb: <a href="' + my_url + '">' + my_thumb + '</a></div>')
						req.write('</div>')
					req.write('<div class="link">')
					req.write('<a class="dir_link" href="' + url + '/">' + dirname + '/</a>')
					req.write('</div>')
					req.write('</div>')
				req.write('</div>')

			if files:
				req.write('<div id="file_list">')
				files.sort()
				for filename in files:
					if filename.split('.')[-1].lower() in valid_extensions:
						url = './?p=pictures;req=' + request + filename
						req.write('<div class="item">')
						if use_nails:
							thumb = hthumbs_dir + request + ext_to_jpeg(filename)
							req.write('<div class="thumb">')
							if do_thumbs(request, filename):
								req.write('<a href="' + url + '"><img alt="' + filename + '" src="' + thumb + '"/></a>')
							else:
								req.write('<div class="broken_nail"><p>Broken Thumb: <a href="' + url + '">' + filename + '</a></p></div>')
							req.write('</div>')
						req.write('<div class="link"><a href="' + url + '">' + filename + '</a></div>')
						req.write('</div>')
				req.write('</div>')

		elif S_ISREG(info) or S_ISLNK(info):
			# show a file
			tmp = request.split('/')
			path = '/'.join(tmp[:-1])
			filename = tmp[-1]
			req.write('<div id="hand">')
			if do_hands(path, filename):
				#req.write('showing nail at ' + hhands + ext_to_jpeg(request) + '.'
				req.write('<p><a href="' + hroot + request + '"><img src="' + hhands + ext_to_jpeg(request) + '" alt="' + request + '"/></a></p>')
			else:
				req.write('p class="broken_nail">Broken Hand: <a href="' + hroot + request + '">' + request +'</a></p>')
			req.write('</div>')
		else:
			# not a directory or file
			req.write('<p>')
			req.write('I don\'t know what to do with ' + request)
			req.write('</p>')

	req.write('</div>')


def do_thumbs(path, filename):
	try:
		os.stat(thumbs_dir + path)
	except:
		os.makedirs(thumbs_dir + path, 01775)

	try:
		# does a thumbnail exist?
		thumbname = ext_to_jpeg(filename)
		os.stat(thumbs_dir + path + '/' + thumbname)
	except:
		# nope, try make one
		try:
			img = Image.open(root + path + '/' + filename)
			img.thumbnail((128, 128))
			img.save(thumbs_dir + path + '/' + thumbname, 'JPEG')
		except:
			# couldn't make one
			return False
	# either we found or made a thumbnail
	return True


def do_hands(path, filename):
	try:
		#req.write('looking for hands dir.'
		os.stat(hands + path)
	except:
		#req.write('no hands dir, tyring to make it.'
		os.makedirs(hands + path, 0775)

	try:
		# does a thumbnail exist?
		#req.write('looking for hand.'
		thumbname = ext_to_jpeg(filename)
		os.stat(hands + path + '/' + thumbname)
	except:
		#req.write('no hand, try to make it.'
		# nope, try make one
		try:
			#req.write('opening thumbfile.'
			thumbfile = open(hands + path + '/' + thumbname, 'w')
			#req.write('opening original: '+ root + path + filename + '.'
			img = Image.open(root + path + '/' + filename)
			#req.write('making nail.'
			img.thumbnail((640, 480))
			#req.write('writing to thumbfile.'
			img.save(thumbfile, 'JPEG')
		except Exception, (e):
			req.write(str(e))
			#req.write('could not make hand.'
			#req.write('closing thumbfile.'
			thumbfile.close()
			#req.write('removing thumbfile.'
			os.remove(hands + path + '/' + thumbname)
			# couldn't make one
			return False
		else:
			#req.write('closing thumbfile.'
			thumbfile.close()
	# either we found or made a thumbnail
	#req.write('showing hand.'
	return True


def get_random_thumb_from_dir(dirname):
	global depth
	depth += 1
	l = os.walk(root + dirname).next()
	subdirs = l[1]
	subfiles = l[2]
	thumb = False
	if subfiles:
		n = random.randint(1, len(subfiles)) - 1
		thumb = subfiles[n]
	elif subdirs:
		while not thumb:
			n = random.randint(1, len(subdirs)) - 1
			thumb = '/' + subdirs[n] +'/' + get_random_thumb_from_dir(dirname + '/' + subdirs[n])
	
	if thumb.split('.')[-1].lower() in valid_extensions:
		depth -= 1
		return thumb
	else:
		if depth < 10:
			return get_random_thumb_from_dir(dirname)
		else:
			return thumb


def ext_to_jpeg(filename):
	if '.' in filename:
		filename = filename + '.jpeg'
	return filename
