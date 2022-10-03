import cocotb
from cocotb.triggers import RisingEdge
from cocotb.clock import Clock


@cocotb.test()
async def test(dut):
    input_sequence = 0b110001101
    expected_output = 0b101100011
    actual_output = 0

    await cocotb.start(Clock(dut.clk, 10, units="ns").start())

    while input_sequence != 0:
        dut.input.value = input_sequence & 1
        await RisingEdge(dut.clk)
        actual_output <<= 1
        actual_output |= dut.output.value

    assert actual_output == expected_output
