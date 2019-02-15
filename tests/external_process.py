import sys
sys.path.append('../inventory-mgt/')

import app
import time

MILLIS = 1000
MICROS = MILLIS * 1000
NANOS = MICROS * 1000

def benchmark():
    for line in sys.stdin:
        iters = int(line.strip())

        # Setup

        start = time.perf_counter()
        for x in range(iters):
            app.update_master()
        end = time.perf_counter()

        # Teardown

        delta = end - start
        nanos = int(delta * NANOS)
        print("%d" % nanos)
        sys.stdout.flush()

benchmark()
