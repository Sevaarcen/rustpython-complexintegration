#===============================================================================
#  Pretent this is being defined and loaded from an installed library
#===============================================================================
from abc import ABC, abstractmethod

import uuid


class DataObject:
    def __init__(self, input_data: bytes):
        self.meta = {
            'uuid': uuid.uuid4()
        }
        self.input_data = input_data
        super().__init__()


class ModuleObject(ABC):
    @abstractmethod
    def check(self, input: DataObject) -> bool:
        pass
    
    @abstractmethod
    def run(self, input: DataObject):
        pass

#===============================================================================
#  Class code below
#===============================================================================
import json

class JsonLoader(ModuleObject):
    def check(self, input: DataObject) -> bool:
        try:
            json.loads(input)
            return True
        except Exception:
            return False

    
    def run(self, input: DataObject) -> dict:
        return json.loads(input.input_data)
