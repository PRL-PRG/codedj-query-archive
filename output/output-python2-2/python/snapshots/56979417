import Adafruit_BBIO.PWM as PWM
from joint import joint

class Body(object):
  """docstring for Robot"""
  def __init__(self, config):
    super(Robot, self).__init__()
    joint_dict = {}

    state = config.hip["state_config"].create_state()
    for pin in config.hip["pins"]:
      joint_dict[pin] = Joint(PWM, pin, state)

    state = config.knee["state_config"].create_state()
    for pin in config.knee["pins"]:
      joint_dict[pin] = Joint(PWM, pin, state)

    for (pin, state_config) in config.override:
      joint_dict[pin].set_state(state_config.create_state())

    self.joints = joint_dict.values()

  def possible_next_states():
    next_move = {}
    for joint in self.joints:
      next_move[joint] = joint.get_possible_states()
    return next_move

  def make_move(joint_index, move):
    self.joints[joint_index].reach(move)

  def start():
    for joint in self.joints:
      joint.start()

  def reset():
    for joint in self.joints:
      joint.reset()

