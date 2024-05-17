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
use BigEndian::*;
use IsAligned__1::*;
use MemA_read::*;
use R_read::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_LDRD_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    index: bool,
    m: i64,
    n: i64,
    t: i64,
    t2: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        gs_879856: Bits,
        gs_879848: Bits,
        gs_879853: Bits,
        offset_addr: u32,
        datashadow_7167: u64,
        add: bool,
        index: bool,
        m: i64,
        n: i64,
        t: i64,
        t2: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        index,
        m,
        n,
        t,
        t2,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var add:u8
        let s_0_0: bool = fn_state.add;
        // N s_0_1: branch s_0_0 b16 b1
        if s_0_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call R_read(s_1_4)
        let s_1_5: u32 = R_read(state, tracer, s_1_4);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_8: sub s_1_6 s_1_7
        let s_1_8: Bits = ((s_1_6) - (s_1_7));
        // D s_1_9: cast reint s_1_8 -> u32
        let s_1_9: u32 = (s_1_8.value() as u32);
        // D s_1_10: write-var offset_addr <= s_1_9
        fn_state.offset_addr = s_1_9;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:u8
        let s_2_0: bool = fn_state.index;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_3: write-var address <= s_3_2
        fn_state.address = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var address:u32
        let s_4_1: u32 = fn_state.address;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_3: call IsAligned__1(s_4_2, s_4_0)
        let s_4_3: bool = IsAligned__1(state, tracer, s_4_2, s_4_0);
        // N s_4_4: branch s_4_3 b11 b5
        if s_4_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #4s : i64
        let s_5_0: i64 = 4;
        // D s_5_1: read-var address:u32
        let s_5_1: u32 = fn_state.address;
        // D s_5_2: call MemA_read(s_5_1, s_5_0)
        let s_5_2: Bits = MemA_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: write-var gs#879848 <= s_5_2
        fn_state.gs_879848 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#879848:bv
        let s_6_0: Bits = fn_state.gs_879848;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // D s_6_2: read-var t:i64
        let s_6_2: i64 = fn_state.t;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call R_set(s_6_3, s_6_1)
        let s_6_4: () = R_set(state, tracer, s_6_3, s_6_1);
        // C s_6_5: const #4s : i
        let s_6_5: i128 = 4;
        // D s_6_6: read-var address:u32
        let s_6_6: u32 = fn_state.address;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 32u16);
        // C s_6_8: cast cvt s_6_5 -> bv
        let s_6_8: Bits = Bits::new(s_6_5 as u128, 128);
        // D s_6_9: add s_6_7 s_6_8
        let s_6_9: Bits = (s_6_7 + s_6_8);
        // D s_6_10: cast reint s_6_9 -> u32
        let s_6_10: u32 = (s_6_9.value() as u32);
        // C s_6_11: const #4s : i64
        let s_6_11: i64 = 4;
        // D s_6_12: call MemA_read(s_6_10, s_6_11)
        let s_6_12: Bits = MemA_read(state, tracer, s_6_10, s_6_11);
        // D s_6_13: write-var gs#879853 <= s_6_12
        fn_state.gs_879853 = s_6_12;
        // N s_6_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#879853:bv
        let s_7_0: Bits = fn_state.gs_879853;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: read-var t2:i64
        let s_7_2: i64 = fn_state.t2;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: call R_set(s_7_3, s_7_1)
        let s_7_4: () = R_set(state, tracer, s_7_3, s_7_1);
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wback:u8
        let s_8_0: bool = fn_state.wback;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var offset_addr:u32
        let s_10_2: u32 = fn_state.offset_addr;
        // D s_10_3: call R_set(s_10_1, s_10_2)
        let s_10_3: () = R_set(state, tracer, s_10_1, s_10_2);
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #8s : i64
        let s_11_0: i64 = 8;
        // D s_11_1: read-var address:u32
        let s_11_1: u32 = fn_state.address;
        // D s_11_2: call MemA_read(s_11_1, s_11_0)
        let s_11_2: Bits = MemA_read(state, tracer, s_11_1, s_11_0);
        // D s_11_3: write-var gs#879856 <= s_11_2
        fn_state.gs_879856 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#879856:bv
        let s_12_0: Bits = fn_state.gs_879856;
        // D s_12_1: cast reint s_12_0 -> u64
        let s_12_1: u64 = (s_12_0.value() as u64);
        // D s_12_2: write-var datashadow#7167 <= s_12_1
        fn_state.datashadow_7167 = s_12_1;
        // C s_12_3: const #1u : u32
        let s_12_3: u32 = 1;
        // S s_12_4: call BigEndian(s_12_3)
        let s_12_4: bool = BigEndian(state, tracer, s_12_3);
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var datashadow#7167:u64
        let s_13_1: u64 = fn_state.datashadow_7167;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #31s : i
        let s_13_5: i128 = 31;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u32
        let s_13_8: u32 = (s_13_7.value() as u32);
        // D s_13_9: read-var t:i64
        let s_13_9: i64 = fn_state.t;
        // D s_13_10: cast zx s_13_9 -> i
        let s_13_10: i128 = (i128::try_from(s_13_9).unwrap());
        // D s_13_11: call R_set(s_13_10, s_13_8)
        let s_13_11: () = R_set(state, tracer, s_13_10, s_13_8);
        // C s_13_12: const #32s : i
        let s_13_12: i128 = 32;
        // D s_13_13: read-var datashadow#7167:u64
        let s_13_13: u64 = fn_state.datashadow_7167;
        // D s_13_14: cast zx s_13_13 -> bv
        let s_13_14: Bits = Bits::new(s_13_13 as u128, 64u16);
        // C s_13_15: const #1s : i64
        let s_13_15: i64 = 1;
        // C s_13_16: cast zx s_13_15 -> i
        let s_13_16: i128 = (i128::try_from(s_13_15).unwrap());
        // C s_13_17: const #31s : i
        let s_13_17: i128 = 31;
        // C s_13_18: add s_13_17 s_13_16
        let s_13_18: i128 = (s_13_17 + s_13_16);
        // D s_13_19: bit-extract s_13_14 s_13_12 s_13_18
        let s_13_19: Bits = (Bits::new(
            ((s_13_14) >> (s_13_12)).value(),
            u16::try_from(s_13_18).unwrap(),
        ));
        // D s_13_20: cast reint s_13_19 -> u32
        let s_13_20: u32 = (s_13_19.value() as u32);
        // D s_13_21: read-var t2:i64
        let s_13_21: i64 = fn_state.t2;
        // D s_13_22: cast zx s_13_21 -> i
        let s_13_22: i128 = (i128::try_from(s_13_21).unwrap());
        // D s_13_23: call R_set(s_13_22, s_13_20)
        let s_13_23: () = R_set(state, tracer, s_13_22, s_13_20);
        // N s_13_24: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i
        let s_14_0: i128 = 32;
        // D s_14_1: read-var datashadow#7167:u64
        let s_14_1: u64 = fn_state.datashadow_7167;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 64u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #31s : i
        let s_14_5: i128 = 31;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_0 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u32
        let s_14_8: u32 = (s_14_7.value() as u32);
        // D s_14_9: read-var t:i64
        let s_14_9: i64 = fn_state.t;
        // D s_14_10: cast zx s_14_9 -> i
        let s_14_10: i128 = (i128::try_from(s_14_9).unwrap());
        // D s_14_11: call R_set(s_14_10, s_14_8)
        let s_14_11: () = R_set(state, tracer, s_14_10, s_14_8);
        // C s_14_12: const #0s : i
        let s_14_12: i128 = 0;
        // D s_14_13: read-var datashadow#7167:u64
        let s_14_13: u64 = fn_state.datashadow_7167;
        // D s_14_14: cast zx s_14_13 -> bv
        let s_14_14: Bits = Bits::new(s_14_13 as u128, 64u16);
        // C s_14_15: const #1s : i64
        let s_14_15: i64 = 1;
        // C s_14_16: cast zx s_14_15 -> i
        let s_14_16: i128 = (i128::try_from(s_14_15).unwrap());
        // C s_14_17: const #31s : i
        let s_14_17: i128 = 31;
        // C s_14_18: add s_14_17 s_14_16
        let s_14_18: i128 = (s_14_17 + s_14_16);
        // D s_14_19: bit-extract s_14_14 s_14_12 s_14_18
        let s_14_19: Bits = (Bits::new(
            ((s_14_14) >> (s_14_12)).value(),
            u16::try_from(s_14_18).unwrap(),
        ));
        // D s_14_20: cast reint s_14_19 -> u32
        let s_14_20: u32 = (s_14_19.value() as u32);
        // D s_14_21: read-var t2:i64
        let s_14_21: i64 = fn_state.t2;
        // D s_14_22: cast zx s_14_21 -> i
        let s_14_22: i128 = (i128::try_from(s_14_21).unwrap());
        // D s_14_23: call R_set(s_14_22, s_14_20)
        let s_14_23: () = R_set(state, tracer, s_14_22, s_14_20);
        // N s_14_24: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var offset_addr:u32
        let s_15_0: u32 = fn_state.offset_addr;
        // D s_15_1: write-var address <= s_15_0
        fn_state.address = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_16_3: read-var m:i64
        let s_16_3: i64 = fn_state.m;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: call R_read(s_16_4)
        let s_16_5: u32 = R_read(state, tracer, s_16_4);
        // D s_16_6: cast zx s_16_2 -> bv
        let s_16_6: Bits = Bits::new(s_16_2 as u128, 32u16);
        // D s_16_7: cast zx s_16_5 -> bv
        let s_16_7: Bits = Bits::new(s_16_5 as u128, 32u16);
        // D s_16_8: add s_16_6 s_16_7
        let s_16_8: Bits = (s_16_6 + s_16_7);
        // D s_16_9: cast reint s_16_8 -> u32
        let s_16_9: u32 = (s_16_8.value() as u32);
        // D s_16_10: write-var offset_addr <= s_16_9
        fn_state.offset_addr = s_16_9;
        // N s_16_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
