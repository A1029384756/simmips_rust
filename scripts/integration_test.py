import pexpect
import pexpect.replwrap as replwrap
import unittest
import os
import sys

sys.path.append(os.getcwd())
TEST_FILE_DIR = os.getcwd() + '/tests/'

# the vtscript executable
cmd = './target/debug/simmips_tui'

# the prompt to expect
prompt = u'simmips> '

registers = ["zero", "0", "at", "1", "v0", "2",  "v1", "3",  "a0", "4",
             "a1", "5",   "a2", "6",  "a3", "7",  "t0", "8",  "t1", "9",
             "t2", "10",  "t3", "11", "t4", "12", "t5", "13", "t6", "14",
             "t7", "15",  "s0", "16", "s1", "17", "s2", "18", "s3", "19",
             "s4", "20",  "s5", "21", "s6", "22", "s7", "23", "t8", "24",
             "t9", "25",  "k0", "26", "k1", "27", "gp", "28", "sp", "29",
             "fp", "30",  "ra", "31", "pc", "hi", "lo"]

class TestsBadFile(unittest.TestCase):

    def test_error(self):
        args = ' /there/is/no/such/file'
        (output, retcode) = pexpect.run(cmd+args, withexitstatus=True, extra_args=args)
        self.assertNotEqual(retcode, 0)
        self.assertTrue(output.strip().startswith(b'Error'))
        
class Test00(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test00.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)
        
    def test_unknown_command(self):
        output = self.wrapper.run_command(u'foo')
        self.assertTrue(output.strip().startswith('Error'))

    def test_unknown_register(self):
        output = self.wrapper.run_command(u'print $32')
        self.assertTrue(output.strip().startswith('Error'))
        
    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")

    def test_all_registers(self):
        for reg in registers:
            output = self.wrapper.run_command(u'print $'+reg)
            self.assertEqual(output.strip(), "0x00000000")
        
    def test_print_pc_step(self):
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")


class Test01(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test01.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print &0x00000008')
        self.assertEqual(output.strip(), "0x01")
        output = self.wrapper.run_command(u'print &0x00000009')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000a')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000b')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000c')
        self.assertEqual(output.strip(), "0xfe")
        output = self.wrapper.run_command(u'print &0x0000000d')
        self.assertEqual(output.strip(), "0xff")
        output = self.wrapper.run_command(u'print &0x0000000e')
        self.assertEqual(output.strip(), "0xff")
        output = self.wrapper.run_command(u'print &0x0000000f')
        self.assertEqual(output.strip(), "0xff")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $t3')
        self.assertEqual(output.strip(), "0xfffffffe")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print $t4')
        self.assertEqual(output.strip(), "0xfffffffe")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000006")
        output = self.wrapper.run_command(u'print $t5')
        self.assertEqual(output.strip(), "0xfffffffe")

class Test02(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test02.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000007")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print &0x00000000')
        self.assertEqual(output.strip(), "0x07")
        output = self.wrapper.run_command(u'print &0x00000001')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000002')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000003')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print &0x00000004')
        self.assertEqual(output.strip(), "0x07")
        output = self.wrapper.run_command(u'print &0x00000005')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000006')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000007')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print &0x00000008')
        self.assertEqual(output.strip(), "0x07")
        output = self.wrapper.run_command(u'print &0x00000009')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000a')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000b')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000006")
        output = self.wrapper.run_command(u'print &0x0000000c')
        self.assertEqual(output.strip(), "0x07")
        output = self.wrapper.run_command(u'print &0x0000000d')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000e')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x0000000f')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000007")
        output = self.wrapper.run_command(u'print &0x00000010')
        self.assertEqual(output.strip(), "0x07")
        output = self.wrapper.run_command(u'print &0x00000011')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000012')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000013')
        self.assertEqual(output.strip(), "0x00")

class Test03(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test03.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000064")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t3')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t4')
        self.assertEqual(output.strip(), "0x00000004")
        
class Test04(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test04.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $14')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $15')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t3')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t4')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t5')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t6')
        self.assertEqual(output.strip(), "0xfffffb2e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $t7')
        self.assertEqual(output.strip(), "0xfffffb2e")

class Test05(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test05.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0x00000001")

class Test06(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test06.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x0000001f")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x0000002b")
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0x0000004a")

class Test07(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test07.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x0000000c")
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000006")
        output = self.wrapper.run_command(u'print $t3')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $t4')
        self.assertEqual(output.strip(), "0x0000000b")
        output = self.wrapper.run_command(u'print $t5')
        self.assertEqual(output.strip(), "0xfffffffd")

class Test08(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test08.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x0000000c")
        output = self.wrapper.run_command(u'print $t2')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000006")
        output = self.wrapper.run_command(u'print $t3')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $t4')
        self.assertEqual(output.strip(), "0x0000000b")
        output = self.wrapper.run_command(u'print $t5')
        self.assertEqual(output.strip(), "0xfffffffd")

class Test09(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test09.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000a")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0xfffffffe")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0xfffffffc")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0xfffffffc")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000f")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000000")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000014")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000000")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0xfffffffc")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0xffffffff")

class Test10(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test10.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000a")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000000")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000001")

class Test11(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test11.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000a")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0xfffffffe")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0xffffffff")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000f")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000000")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x10000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x10000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000014")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000000")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0xfffffffc")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0xf0000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0xf0000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")

class Test12(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test12.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x00000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000002")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        self.assertEqual(output.strip(), "0x0000000a")
        output = self.wrapper.run_command(u'print $t0')
        self.assertEqual(output.strip(), "0x40000001")
        output = self.wrapper.run_command(u'print $t1')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'print $lo')
        self.assertEqual(output.strip(), "0x10000000")
        output = self.wrapper.run_command(u'print $hi')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'print $24')
        self.assertEqual(output.strip(), "0x10000000")
        output = self.wrapper.run_command(u'print $25')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")

class Test13(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test13.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x00000000")

class Test14(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test14.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0xfffffff1")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0xfffffff0")

class Test15(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test15.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x0000000e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x0000000f")


class Test16(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test16.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x00000006")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0x0000000f")


class Test17(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test17.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0xfffffff3")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0xfffffff5")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $10')
        self.assertEqual(output.strip(), "0xfffffffc")

class Test18(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test18.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000001")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000003")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

class Test19(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test19.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000004")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000005")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000007")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000008")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x0000000a")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x0000000b")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x0000000d")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x0000000e")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000010")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000011")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000013")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000014")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000016")
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'step')
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000016")

class Test20(unittest.TestCase):

    def setUp(self):
        args = ' ' + TEST_FILE_DIR + '/vm/test20.asm'
        self.wrapper = replwrap.REPLWrapper(cmd+args, prompt, None)

    def test_intial_status(self):
        output = self.wrapper.run_command(u'status')
        self.assertEqual(output.strip(), "")
        output = self.wrapper.run_command(u'print $pc')
        self.assertEqual(output.strip(), "0x00000000")

    def test_step(self):
        for i in range(54):
            self.wrapper.run_command(u'step')
        
        output = self.wrapper.run_command(u'print &0x00000004')
        self.assertEqual(output.strip(), "0x81")
        output = self.wrapper.run_command(u'print &0x00000005')
        self.assertEqual(output.strip(), "0x01")
        output = self.wrapper.run_command(u'print &0x00000006')
        self.assertEqual(output.strip(), "0x00")
        output = self.wrapper.run_command(u'print &0x00000007')
        self.assertEqual(output.strip(), "0x00")
        
# run the tests
unittest.main()
