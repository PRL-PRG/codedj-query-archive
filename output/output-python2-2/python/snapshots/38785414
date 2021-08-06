from __future__ import division

import numpy as np
import cv2
import sys
import socket
import logging
import time
import colorsys
import pdb
import json

tx_udp = True
im_show = False
sliders = False
wait = False
printer = False

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

FIELD_OF_VIEW = 65
fps_stats = []

# finds the distance between 2 points
def distance_between_points(p1, p2):
    x1, y1 = p1[0]
    x2, y2 = p2[0]
    return ((x1 - x2)**2 + (y1 - y2)**2)**0.5

# finds the squared distance to point
def sq_distance_to_point(x0, y0):
    def distance(pt):
        x, y = pt[0]
        dx, dy = x - x0, y - y0
        return dx*dx + dy*dy
    return distance

# finds the midpoint of 2 points
def midpoint(p1, p2):
    x1, y1 = p1[0]
    x2, y2 = p2[0]
    return (x1 + x2)/2, (y1 + y2)/2

# finds the area of a contour
def area(cnt):
    return cv2.contourArea(cnt)

# finds the perimeter of a contour
def perimeter(cnt):
    return cv2.actLength(cnt, True)

# tests five eighths down the center of the object is not part 
# of the object, insuring it's a U shape
def five_eighths_test(cnt, img, width, height):
    # finds the diagonal extreme of the contour
    upper_left = min(cnt, key=sq_distance_to_point(0, 0))
    upper_right = min(cnt, key=sq_distance_to_point(width, 0))
    bottom_left = min(cnt, key=sq_distance_to_point(0, height))
    bottom_right = min(cnt, key=sq_distance_to_point(width, height))

    # finds height of the U
    left_height = distance_between_points(upper_left, bottom_left)
    right_height = distance_between_points(upper_right, bottom_right)
    height = (left_height + right_height)/2

    # finds the midpoint
    upper_mid = midpoint(upper_left, upper_right)
    # unpacks the x, y values of the midpoint
    midX, midY = upper_mid

    # actually tests each point along the five-eighths from the top line
    testX = midX
    testY = midY + 0.625*height
    testpoint = testX, testY
    return (img[midY:testY, testX] == 0).all()

# tests if the area of the contour is within 2 values
def area_test(cnt, width, height):
    #return width*height*0.005 < area(cnt) < width*height*0.02
    return area(cnt) > 1000

def vertical_test(cnt, img, width, height):
    # finds the diagonal extreme of the contour
    upper_left = min(cnt, key=sq_distance_to_point(0, 0))
    upper_right = min(cnt, key=sq_distance_to_point(width, 0))
    bottom_left = min(cnt, key=sq_distance_to_point(0, height))
    bottom_right = min(cnt, key=sq_distance_to_point(width, height))

    # finds the midpoint
    upper_midpoint = midpoint(upper_left, upper_right)
    bottom_midpoint = midpoint(bottom_left, bottom_right)
    upperX, upperY = upper_midpoint
    bottomX, bottomY = bottom_midpoint
    full_midpoint = (upperX + bottomX)/2, (upperY + bottomY)/2
    midpointX, midpointY = full_midpoint

    # test up a vertical line
    test_pixels = []
    count = 0
    while count < bottomY:
        test_pixels.append(img[count, midpointX] > 0)
        #img[count, midpointX] = 200
        count += 1

    count = 0
    for pixel in test_pixels:
        if pixel:
            count += 1
    return count > len(test_pixels)*.95

# determines the degrees of the U off from the middle
def find_heading(cnt, width, height):
    # finds the diagonal extremes of the contour
    upper_left = min(cnt, key=sq_distance_to_point(0, 0))
    upper_right = min(cnt, key=sq_distance_to_point(width, 0))
    bottom_left = min(cnt, key=sq_distance_to_point(0, width))
    bottom_right = min(cnt, key=sq_distance_to_point(width, height))

    #finds the midpoint
    midpoint_upper = midpoint(upper_left, upper_right)
    midpoint_bottom = midpoint(bottom_left, bottom_right)
    upX, upY = midpoint_upper
    botX, botY = midpoint_bottom
    mid = (upX + botX)/2, (upY + botY)/2
    midX, midY = mid
    pixel_distance = midX - width/2
    heading = ((FIELD_OF_VIEW/2.0) * pixel_distance)/(width/2)
    return int(heading)

# determines the distance of the U from the robot in inches (NOTE!!! NOT ACCURATE)
def find_distance(cnt, width, height):
    # find the diagonal extremes of the contour
    upper_left = min(cnt, key=sq_distance_to_point(0, 0))
    upper_right = min(cnt, key=sq_distance_to_point(width, 0))
    bottom_left = min(cnt, key=sq_distance_to_point(0, height))
    bottom_right = min(cnt, key=sq_distance_to_point(width, height))

    # finds the left and right X values
    bottom_leftX, bottom_leftY = bottom_left[0]
    bottom_rightX, bottom_rightY = bottom_right[0]
    if bottom_leftY > bottom_rightY:
        pixel_height = bottom_leftY - bottom_rightY
    else:
        pixel_height = bottom_rightY - bottom_leftY
    pixel_width = abs(bottom_rightX - bottom_leftX)
    pixel_distance = (pixel_width**2 + pixel_height**2)**0.5
    distance = ((17.25/(pixel_distance/width))**2 - 6724)**0.5
    if (distance >= 0 and distance < 9999):
        return int(distance)
    else:
        return 9999

def nothing(x):
    pass

# sets the video capture
cap = cv2.VideoCapture(-1)
cap.set(cv2.cv.CV_CAP_PROP_FRAME_WIDTH, 320);
cap.set(cv2.cv.CV_CAP_PROP_FRAME_HEIGHT, 240);
cap.set(cv2.cv.CV_CAP_PROP_FPS, 30)

if sliders:
    # creates slider windows
    lower = np.zeros((300,512,3), np.uint8)
    cv2.namedWindow('lower')
    upper = np.zeros((300,512,3), np.uint8)
    cv2.namedWindow('upper')

    # creates the rgb trackbars
    switch = '0 : OFF \n1 : ON'
    cv2.createTrackbar('H','lower',0,255,nothing)
    cv2.createTrackbar('S','lower',0,255,nothing)
    cv2.createTrackbar('V','lower',0,255,nothing)
    cv2.createTrackbar(switch, 'lower',0,1,nothing)
    cv2.createTrackbar('H','upper',0,255,nothing)
    cv2.createTrackbar('S','upper',0,255,nothing)
    cv2.createTrackbar('V','upper',0,255,nothing)
    cv2.createTrackbar(switch, 'upper',0,1,nothing)
    cv2.setTrackbarPos('H', 'lower', 0)
    cv2.setTrackbarPos('S', 'lower', 0)
    cv2.setTrackbarPos('V', 'lower', 212)
    cv2.setTrackbarPos('H', 'upper', 255)
    cv2.setTrackbarPos('S', 'upper', 255)
    cv2.setTrackbarPos('V', 'upper', 255)

# sets up UDP sender
if len(sys.argv) > 1:
   ip = sys.argv[1]
else:
   ip = '10.10.76.2'
port = 5880
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

while (1):
    k = cv2.waitKey(1) & 0xFF
    if k == 27:
        break
    start_time = time.time()
    # captures each frame individually
    ret, frame = cap.read()
    height, width, channels = frame.shape

    if im_show:
        cv2.imshow('source', frame)
    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    # converts frame from BGR to HSV
    hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
    if sliders:
        # lower hsv values
        lower_h = cv2.getTrackbarPos('H','lower')
        lower_s = cv2.getTrackbarPos('S','lower')
        lower_v = cv2.getTrackbarPos('V','lower')
        lower_switch = cv2.getTrackbarPos(switch,'lower')
        # upper rgb values
        upper_h = cv2.getTrackbarPos('H','upper')
        upper_s = cv2.getTrackbarPos('S','upper')
        upper_v = cv2.getTrackbarPos('V','upper')
        upper_switch = cv2.getTrackbarPos(switch,'upper')

        # lower hsv values
        lower_rgb = colorsys.hsv_to_rgb(lower_h, lower_s, lower_v)
        lower_r, lower_g, lower_b = lower_rgb
        if lower_switch == 0:
            lower[:] = 0
        else:
            lower[:] = [lower_b,lower_g,lower_r]
        font = cv2.FONT_HERSHEY_PLAIN
        message = 'H: ' + str(lower_h) + ', S: ' + str(lower_s) + ', V: ' + str(lower_v)
        cv2.putText(lower, message, (100, 100), font, 1.0, (0, 255, 255), 1, False)
        if im_show:
            cv2.imshow('lower', lower)

        # upper hsv values
        upper_rgb = colorsys.hsv_to_rgb(upper_h, upper_s, upper_v)
        upper_r, upper_g, upper_b = upper_rgb
        if upper_switch == 0:
            upper[:] = 0
        else:
            upper[:] = [upper_b,upper_g,upper_r]
        font = cv2.FONT_HERSHEY_PLAIN
        message = 'H: ' + str(upper_h) + ', S: ' + str(upper_s) + ', V: ' + str(upper_v)
        cv2.putText(upper, message, (100, 100), font, 1.0, (0, 255, 255), 1, False)
        if im_show:
            cv2.imshow('upper', upper)
        else:
            pass
    else:
        lower_h = 0
        lower_s = 0
        lower_v = 212
        upper_h = 255
        upper_s = 255
        upper_v = 255

    # range of HSV color values
    lower_green = np.array([lower_h, lower_s, lower_v])
    upper_green = np.array([upper_h, upper_s, upper_v])

    # creates a bw image using the above range of values
    mask = cv2.inRange(hsv, lower_green, upper_green)

    # sets the dilation and erosion factor
    kernel = np.ones((25,5),np.uint8)
    # erodes and dilates the image
    mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)
    # dilates and erodes the image
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)
    mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
    if im_show:
        cv2.imshow('inRange', mask)

    # uses mask to create a color image within the range values
    res = cv2.bitwise_and(frame, frame, mask = mask)

    # creates a grayscale image of just the edges of shape
    edges = cv2.Canny(mask, 100, 200)

    # finds contours
    imgray = cv2.cvtColor(res, cv2.COLOR_BGR2GRAY)
    ret, thresh = cv2.threshold(imgray, 0, 255, 0)
    if im_show:
        cv2.imshow('thresh', thresh)
    cv2.waitKey(1)
    contours, hierarchy  = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)


    # tests the contours and determines which ones are U's
    num_of_contours = len(contours)
    shapes = [False]*num_of_contours
    # fills in the boolean array of whether or not a shape is a U
    count = 0
    for contour in contours:
        passes_vertical = vertical_test(contour, mask, width, height)
        passes_area = area_test(contour, width, height)
        if passes_vertical and passes_area:
            shapes[count] = True
        count += 1

    # determines the number of U's
    num_of_U = 0
    x_values = [0]*num_of_contours
    for shape in shapes:
        if shape:
            num_of_U += 1

    # saves the bottom-left X value of each U, or 10000 if not a U
    count = 0
    if num_of_U > 1:
        for contour in contours:
            coordinate = min(contour, key=sq_distance_to_point(0, height))
            x_coor, y_coor = coordinate[0]
            if shapes[count]:
                x_values[count] = x_coor
            else:
                x_values[count] = 10000
            count += 1
    # sorts the x_values array in ascending numerical order
    x_values.sort()
    # sets the target x-value as the lowest x-value
    if num_of_U > 0:
        targetX = x_values[0]
    else:
        targetX = 0

    # determines the UDP message
    message = ''
    # sets the message for no targets and sends
    if num_of_U == 0:
        data = {
           "sender" : "vision",
           "message" : "none",
           "status" : "no target",
        }
        message = json.dumps(data)
	if tx_udp:
            sock.sendto(message, (ip, port))
            if printer:
                print "Tx:" + message
    # sets the message for the correct number of target (may send 2 messages) and sends
    elif 1 <= num_of_U <= 1:
        count = 0
        for contour in contours:
            if shapes[count]:
                # sets the point at which the origin of the text label is placed on the image
                label_point = min(contour, key=sq_distance_to_point(width, height))
                labelX, labelY = label_point[0]
                # determines the heading of this U
                heading = find_heading(contour, width, height)
                # determines the distance of this U
                distance = find_distance(contour, width, height)
                # actually sets the message based on number of U's
                if num_of_U == 1:
                    font = cv2.FONT_HERSHEY_PLAIN
                    status = "ok"
                else:
                    font = cv2.FONT_HERSHEY_PLAIN
                    coordinate = min(contour, key=sq_distance_to_point(0, height))
                    x_coor, y_coor = coordinate[0]
                    if targetX == x_coor:
                        status = "LEFT"
                    else:
                        status = "RIGHT"
                data = {
                   "sender" : "vision",
                   "range" : distance,
                   "heading" : heading,
                   "message" : "heading and range",
                   "status" : status,
                }
                message = json.dumps(data)
                # sends the message
                if tx_udp:
                    sock.sendto(message, (ip, port))
                    if printer:
		        print "Tx:" + message
                cv2.putText(frame, message, (labelX, labelY), font, 1.0, (255, 255, 255), 1, False)
            count += 1
    # sets the message for too many targets and sends
    else:
        data = {
           "sender" : "vision",
           "message" : "none",
            "status" : "too many targets",
        }
        message = json.dumps(data)
        if tx_udp:
            sock.sendto(message, (ip, port))
            if printer:
	        print "Tx:" + message
    time.sleep(.01)
    # displays the frame with labels
    end_time = time.time()
    time_difference = end_time - start_time
    fps = 1/time_difference
    fps_stats.append(fps)
    average = str(sum(fps_stats)/len(fps_stats))
    font = cv2.FONT_HERSHEY_PLAIN
    cv2.putText(frame, average, (width - 100, height - 100), font, 1.0, (255, 255, 255), 1, False)
    if im_show:
        cv2.imshow('edges', res)
    if wait:
        if not im_show:
            cv2.namedWindow('waitkey placeholder')
        k = cv2.waitKey(0)
        if k == 27:         # wait for ESC key to exit
            cv2.destroyAllWindows()
            break
