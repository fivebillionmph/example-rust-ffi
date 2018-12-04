import ctypes
from ctypes import Structure, POINTER, c_int32, c_char_p, c_void_p

class GameT(Structure):
	pass

class Game:
	def __init__(self, libpath):
		# load the library
		self.lib = ctypes.cdll.LoadLibrary(libpath)

		# interface functions and types to library
		self.lib.game_new.restype = POINTER(GameT)

		self.lib.game_free.argtypes = (POINTER(GameT), )

		self.lib.game_get_n.argtypes = (POINTER(GameT), )
		self.lib.game_get_n.restype = c_int32

		self.lib.game_set_n.argtypes = (POINTER(GameT), c_int32)

		self.lib.game_get_name.argtypes = (POINTER(GameT), )
		self.lib.game_get_name.restype = c_void_p

		self.lib.game_set_name.argtypes = (POINTER(GameT), c_char_p)

		self.lib.game_free_string.argtypes = (c_void_p, )

		# create the object
		self.obj = self.lib.game_new()

	def __enter__(self):
		return self

	def __exit__(self, exc_type, exc_value, traceback):
		self.lib.game_free(self.obj)

	def get_name(self):
		ptr = self.lib.game_get_name(self.obj)
		name = ctypes.cast(ptr, c_char_p).value.decode()
		self.lib.game_free_string(ptr)
		return name

	def set_name(self, name):
		self.lib.game_set_name(self.obj, name.encode())

	def get_n(self):
		return self.lib.game_get_n(self.obj)

	def set_n(self, n):
		name_obj = self.lib.game_set_n(self.obj, n)
