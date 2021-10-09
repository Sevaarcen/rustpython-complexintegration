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
from multiprocessing import Pool
import uuid
import time


class WordCounter(ModuleObject):
    def check(self, input: DataObject) -> bool:
        return True
    
    def run(self, input: DataObject) -> dict:
        # THE CODE BELOW CAUSES UNDEFINED BEHAVIOR THAT ARE LIKELY TO LEAD TO SYSTEM CRASHES AND MEMORY CURRUPTION WHEN USING PYO3 (Maybe CPython?)
        """
        start_time = time.time()
        num_workers = 4
        pool = Pool(num_workers)
        for _ in range(num_workers):
            pool.apply_async(self.work, ())
        pool.close()
        pool.join()
        end_time = time.time()
        print(f"Finished in: {end_time - start_time}")
        """
        return self.work()

    
    def work(self) -> dict:
        start_time = time.time()
        exec_uuid = uuid.uuid4()
        print(f"Spawned: {exec_uuid}")
        num_found = 0
        for num in range(0, 25_000_000):
            if num % 2 == 0 or num % 3 == 0 or num % 5 == 0:
                bitthing = num ^ 0x42
                isthree = bitthing & 3
                if isthree == 3:
                    num_found += 1
                if num_found % 250_000 == 0:
                    print(f"PYTH {exec_uuid} found {num_found} so far in {num}")
        end_time = time.time()
        delta = end_time - start_time
        print(f"{exec_uuid} finished in {delta} seconds")
        return {"computed_result": num_found}


if __name__ == '__main__':
    num_workers = 16
    start_time = time.time()

    counter = WordCounter()

    # run test twice from the top, both in python
    pool = Pool(num_workers)
    for _ in range(num_workers):
        pool.apply_async(counter.run, ())
    pool.close()
    pool.join()

    pool = Pool(num_workers)
    for _ in range(num_workers):
        pool.apply_async(counter.run, ())
    pool.close()
    pool.join()

    end_time = time.time()
    print(f"Finished in: {end_time - start_time}")