from pyo3_common import Output, Input


def run(input: Input):
    print(f"in python! got input: {input}", flush=True)
    output = Output(input.x * 2, input.y * 2)
    print("from python! calling calc: ", output.calc(), flush=True)

    return output
