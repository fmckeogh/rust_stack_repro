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
use FPDecodeRM::*;
use HaveFP16Ext::*;
use execute_aarch32_instrs_VCVTA_vfp_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VCVTA_vfp_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    RM: u8,
    Vd: u8,
    size: u8,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        d: i64,
        is_unsigned: bool,
        rounding: u32,
        gs_325201: bool,
        gs_325200: bool,
        D: bool,
        RM: u8,
        Vd: u8,
        size: u8,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        RM,
        Vd,
        size,
        op,
        M,
        Vm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b17 b1
        if s_0_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b16 b2
        if s_1_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #1u : u8
        let s_2_2: u8 = 1;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#325200 <= s_3_0
        fn_state.gs_325200 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#325200:u8
        let s_4_0: bool = fn_state.gs_325200;
        // D s_4_1: write-var gs#325201 <= s_4_0
        fn_state.gs_325201 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#325201:u8
        let s_5_0: bool = fn_state.gs_325201;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var RM:u8
        let s_6_0: u8 = fn_state.RM;
        // D s_6_1: call FPDecodeRM(s_6_0)
        let s_6_1: u32 = FPDecodeRM(state, tracer, s_6_0);
        // D s_6_2: write-var rounding <= s_6_1
        fn_state.rounding = s_6_1;
        // D s_6_3: read-var op:u8
        let s_6_3: bool = fn_state.op;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // C s_6_5: const #0u : u8
        let s_6_5: bool = false;
        // C s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 1u16);
        // D s_6_7: cmp-eq s_6_4 s_6_6
        let s_6_7: bool = ((s_6_4) == (s_6_6));
        // D s_6_8: write-var is_unsigned <= s_6_7
        fn_state.is_unsigned = s_6_7;
        // D s_6_9: read-var Vd:u8
        let s_6_9: u8 = fn_state.Vd;
        // D s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 4u16);
        // D s_6_11: read-var D:u8
        let s_6_11: bool = fn_state.D;
        // D s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 1u16);
        // D s_6_13: cast reint s_6_10 -> u128
        let s_6_13: u128 = (s_6_10.value() as u128);
        // D s_6_14: size-of s_6_10
        let s_6_14: u16 = s_6_10.length();
        // D s_6_15: cast reint s_6_12 -> u128
        let s_6_15: u128 = (s_6_12.value() as u128);
        // D s_6_16: size-of s_6_12
        let s_6_16: u16 = s_6_12.length();
        // D s_6_17: lsl s_6_13 s_6_16
        let s_6_17: u128 = s_6_13 << s_6_16;
        // D s_6_18: or s_6_17 s_6_15
        let s_6_18: u128 = ((s_6_17) | (s_6_15));
        // D s_6_19: add s_6_14 s_6_16
        let s_6_19: u16 = (s_6_14 + s_6_16);
        // D s_6_20: create-bits s_6_18 s_6_19
        let s_6_20: Bits = Bits::new(s_6_18, s_6_19);
        // D s_6_21: cast reint s_6_20 -> u8
        let s_6_21: u8 = (s_6_20.value() as u8);
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 5u16);
        // D s_6_23: cast zx s_6_22 -> i
        let s_6_23: i128 = (s_6_22.value() as i128);
        // D s_6_24: cast reint s_6_23 -> i64
        let s_6_24: i64 = (s_6_23 as i64);
        // D s_6_25: write-var d <= s_6_24
        fn_state.d = s_6_24;
        // C s_6_26: const #16s : i64
        let s_6_26: i64 = 16;
        // D s_6_27: write-var esize <= s_6_26
        fn_state.esize = s_6_26;
        // D s_6_28: read-var size:u8
        let s_6_28: u8 = fn_state.size;
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 2u16);
        // C s_6_30: const #1u : u8
        let s_6_30: u8 = 1;
        // C s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 2u16);
        // D s_6_32: cmp-eq s_6_29 s_6_31
        let s_6_32: bool = ((s_6_29) == (s_6_31));
        // D s_6_33: not s_6_32
        let s_6_33: bool = !s_6_32;
        // N s_6_34: branch s_6_33 b9 b7
        if s_6_33 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // D s_7_2: read-var Vm:u8
        let s_7_2: u8 = fn_state.Vm;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: read-var M:u8
        let s_7_4: bool = fn_state.M;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: cast reint s_7_5 -> u128
        let s_7_8: u128 = (s_7_5.value() as u128);
        // D s_7_9: size-of s_7_5
        let s_7_9: u16 = s_7_5.length();
        // D s_7_10: lsl s_7_6 s_7_9
        let s_7_10: u128 = s_7_6 << s_7_9;
        // D s_7_11: or s_7_10 s_7_8
        let s_7_11: u128 = ((s_7_10) | (s_7_8));
        // D s_7_12: add s_7_7 s_7_9
        let s_7_12: u16 = (s_7_7 + s_7_9);
        // D s_7_13: create-bits s_7_11 s_7_12
        let s_7_13: Bits = Bits::new(s_7_11, s_7_12);
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 5u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: write-var m <= s_7_17
        fn_state.m = s_7_17;
        // N s_7_19: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: read-var d:i64
        let s_8_2: i64 = fn_state.d;
        // D s_8_3: read-var rounding:u32
        let s_8_3: u32 = fn_state.rounding;
        // D s_8_4: read-var is_unsigned:u8
        let s_8_4: bool = fn_state.is_unsigned;
        // D s_8_5: call execute_aarch32_instrs_VCVTA_vfp_Op_A_txt(s_8_2, s_8_1, s_8_0, s_8_3, s_8_4)
        let s_8_5: () = execute_aarch32_instrs_VCVTA_vfp_Op_A_txt(
            state,
            tracer,
            s_8_2,
            s_8_1,
            s_8_0,
            s_8_3,
            s_8_4,
        );
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var size:u8
        let s_9_0: u8 = fn_state.size;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #32s : i64
        let s_10_0: i64 = 32;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // D s_10_2: read-var Vm:u8
        let s_10_2: u8 = fn_state.Vm;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: read-var M:u8
        let s_10_4: bool = fn_state.M;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: lsl s_10_6 s_10_9
        let s_10_10: u128 = s_10_6 << s_10_9;
        // D s_10_11: or s_10_10 s_10_8
        let s_10_11: u128 = ((s_10_10) | (s_10_8));
        // D s_10_12: add s_10_7 s_10_9
        let s_10_12: u16 = (s_10_7 + s_10_9);
        // D s_10_13: create-bits s_10_11 s_10_12
        let s_10_13: Bits = Bits::new(s_10_11, s_10_12);
        // D s_10_14: cast reint s_10_13 -> u8
        let s_10_14: u8 = (s_10_13.value() as u8);
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 5u16);
        // D s_10_16: cast zx s_10_15 -> i
        let s_10_16: i128 = (s_10_15.value() as i128);
        // D s_10_17: cast reint s_10_16 -> i64
        let s_10_17: i64 = (s_10_16 as i64);
        // D s_10_18: write-var m <= s_10_17
        fn_state.m = s_10_17;
        // N s_10_19: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var size:u8
        let s_11_0: u8 = fn_state.size;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #3u : u8
        let s_11_2: u8 = 3;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: write-var esize <= s_12_0
        fn_state.esize = s_12_0;
        // D s_12_2: read-var M:u8
        let s_12_2: bool = fn_state.M;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: read-var Vm:u8
        let s_12_4: u8 = fn_state.Vm;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 4u16);
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: cast reint s_12_5 -> u128
        let s_12_8: u128 = (s_12_5.value() as u128);
        // D s_12_9: size-of s_12_5
        let s_12_9: u16 = s_12_5.length();
        // D s_12_10: lsl s_12_6 s_12_9
        let s_12_10: u128 = s_12_6 << s_12_9;
        // D s_12_11: or s_12_10 s_12_8
        let s_12_11: u128 = ((s_12_10) | (s_12_8));
        // D s_12_12: add s_12_7 s_12_9
        let s_12_12: u16 = (s_12_7 + s_12_9);
        // D s_12_13: create-bits s_12_11 s_12_12
        let s_12_13: Bits = Bits::new(s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u8
        let s_12_14: u8 = (s_12_13.value() as u8);
        // D s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 5u16);
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (s_12_15.value() as i128);
        // D s_12_17: cast reint s_12_16 -> i64
        let s_12_17: i64 = (s_12_16 as i64);
        // D s_12_18: write-var m <= s_12_17
        fn_state.m = s_12_17;
        // N s_12_19: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveFP16Ext(s_15_0)
        let s_15_1: bool = HaveFP16Ext(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#325200 <= s_15_2
        fn_state.gs_325200 = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#325201 <= s_16_0
        fn_state.gs_325201 = s_16_0;
        // N s_16_2: jump b5
        return block_5(state, tracer, fn_state);
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
}
