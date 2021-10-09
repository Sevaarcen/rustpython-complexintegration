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
import statistics


class WordCounter(ModuleObject):
    def check(self, input: DataObject) -> bool:
        return isinstance(input, str)
    
    def run(self, input: DataObject) -> dict:
        data = input.input_data
        sentences = data.split(".")
        sentence_counts = []
        for sentence in sentences:
            sentence_split = sentence.split()
            words_in_sentence = len(sentence_split)
            sentence_counts.append(words_in_sentence)
        # build results and add to meta
        results = {}
        results["max"] = max(sentence_counts)
        results["min"] = min(sentence_counts)
        results["median"] = statistics.median(sentence_counts)
        results["mode"] = statistics.mode(sentence_counts)
        results["raw"] = sentence_counts
        return results
