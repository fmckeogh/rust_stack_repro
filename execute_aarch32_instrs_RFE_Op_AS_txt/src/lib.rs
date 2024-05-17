#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use MemA_read::*;
use AArch32_ExceptionReturn::*;
use R_read::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_RFE_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: bool,
    n: i64,
    wback: bool,
    wordhigher: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        spsrshadow_7905: u32,
        gs_912808: Bits,
        new_pc_valueshadow_7904: u32,
        gs_912812: Bits,
        ga_364214: u32,
        address: u32,
        increment_name: bool,
        n: i64,
        wback: bool,
        wordhigher: bool,
    }
    let fn_state = FunctionState {
        increment_name,
        n,
        wback,
        wordhigher,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b18 b1
        if s_0_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b17 b2
        if s_1_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var increment_name:u8
        let s_2_0: bool = fn_state.increment_name;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #8s : i
        let s_3_3: i128 = 8;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // C s_3_5: cast cvt s_3_3 -> bv
        let s_3_5: Bits = Bits::new(s_3_3 as u128, 128);
        // D s_3_6: sub s_3_4 s_3_5
        let s_3_6: Bits = ((s_3_4) - (s_3_5));
        // D s_3_7: cast reint s_3_6 -> u32
        let s_3_7: u32 = (s_3_6.value() as u32);
        // D s_3_8: write-var address <= s_3_7
        fn_state.address = s_3_7;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var wordhigher:u8
        let s_4_0: bool = fn_state.wordhigher;
        // N s_4_1: branch s_4_0 b15 b5
        if s_4_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4s : i64
        let s_6_0: i64 = 4;
        // D s_6_1: read-var address:u32
        let s_6_1: u32 = fn_state.address;
        // D s_6_2: call MemA_read(s_6_1, s_6_0)
        let s_6_2: Bits = MemA_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: write-var gs#912808 <= s_6_2
        fn_state.gs_912808 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#912808:bv
        let s_7_0: Bits = fn_state.gs_912808;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: write-var new_pc_valueshadow#7904 <= s_7_1
        fn_state.new_pc_valueshadow_7904 = s_7_1;
        // C s_7_3: const #4s : i
        let s_7_3: i128 = 4;
        // D s_7_4: read-var address:u32
        let s_7_4: u32 = fn_state.address;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 32u16);
        // C s_7_6: cast cvt s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 128);
        // D s_7_7: add s_7_5 s_7_6
        let s_7_7: Bits = (s_7_5 + s_7_6);
        // D s_7_8: cast reint s_7_7 -> u32
        let s_7_8: u32 = (s_7_7.value() as u32);
        // C s_7_9: const #4s : i64
        let s_7_9: i64 = 4;
        // D s_7_10: call MemA_read(s_7_8, s_7_9)
        let s_7_10: Bits = MemA_read(state, tracer, s_7_8, s_7_9);
        // D s_7_11: write-var gs#912812 <= s_7_10
        fn_state.gs_912812 = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#912812:bv
        let s_8_0: Bits = fn_state.gs_912812;
        // D s_8_1: cast reint s_8_0 -> u32
        let s_8_1: u32 = (s_8_0.value() as u32);
        // D s_8_2: write-var spsrshadow#7905 <= s_8_1
        fn_state.spsrshadow_7905 = s_8_1;
        // D s_8_3: read-var wback:u8
        let s_8_3: bool = fn_state.wback;
        // N s_8_4: branch s_8_3 b11 b9
        if s_8_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var new_pc_valueshadow#7904:u32
        let s_10_0: u32 = fn_state.new_pc_valueshadow_7904;
        // D s_10_1: read-var spsrshadow#7905:u32
        let s_10_1: u32 = fn_state.spsrshadow_7905;
        // D s_10_2: call AArch32_ExceptionReturn(s_10_0, s_10_1)
        let s_10_2: () = AArch32_ExceptionReturn(state, tracer, s_10_0, s_10_1);
        // N s_10_3: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var increment_name:u8
        let s_11_0: bool = fn_state.increment_name;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call R_read(s_12_1)
        let s_12_2: u32 = R_read(state, tracer, s_12_1);
        // C s_12_3: const #8s : i
        let s_12_3: i128 = 8;
        // D s_12_4: cast zx s_12_2 -> bv
        let s_12_4: Bits = Bits::new(s_12_2 as u128, 32u16);
        // C s_12_5: cast cvt s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 128);
        // D s_12_6: sub s_12_4 s_12_5
        let s_12_6: Bits = ((s_12_4) - (s_12_5));
        // D s_12_7: cast reint s_12_6 -> u32
        let s_12_7: u32 = (s_12_6.value() as u32);
        // D s_12_8: write-var ga#364214 <= s_12_7
        fn_state.ga_364214 = s_12_7;
        // N s_12_9: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var ga#364214:u32
        let s_13_2: u32 = fn_state.ga_364214;
        // D s_13_3: call R_set(s_13_1, s_13_2)
        let s_13_3: () = R_set(state, tracer, s_13_1, s_13_2);
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call R_read(s_14_1)
        let s_14_2: u32 = R_read(state, tracer, s_14_1);
        // C s_14_3: const #8s : i
        let s_14_3: i128 = 8;
        // D s_14_4: cast zx s_14_2 -> bv
        let s_14_4: Bits = Bits::new(s_14_2 as u128, 32u16);
        // C s_14_5: cast cvt s_14_3 -> bv
        let s_14_5: Bits = Bits::new(s_14_3 as u128, 128);
        // D s_14_6: add s_14_4 s_14_5
        let s_14_6: Bits = (s_14_4 + s_14_5);
        // D s_14_7: cast reint s_14_6 -> u32
        let s_14_7: u32 = (s_14_6.value() as u32);
        // D s_14_8: write-var ga#364214 <= s_14_7
        fn_state.ga_364214 = s_14_7;
        // N s_14_9: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #4s : i
        let s_15_0: i128 = 4;
        // D s_15_1: read-var address:u32
        let s_15_1: u32 = fn_state.address;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 32u16);
        // C s_15_3: cast cvt s_15_0 -> bv
        let s_15_3: Bits = Bits::new(s_15_0 as u128, 128);
        // D s_15_4: add s_15_2 s_15_3
        let s_15_4: Bits = (s_15_2 + s_15_3);
        // D s_15_5: cast reint s_15_4 -> u32
        let s_15_5: u32 = (s_15_4.value() as u32);
        // D s_15_6: write-var address <= s_15_5
        fn_state.address = s_15_5;
        // N s_15_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call R_read(s_16_1)
        let s_16_2: u32 = R_read(state, tracer, s_16_1);
        // D s_16_3: write-var address <= s_16_2
        fn_state.address = s_16_2;
        // N s_16_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
}
