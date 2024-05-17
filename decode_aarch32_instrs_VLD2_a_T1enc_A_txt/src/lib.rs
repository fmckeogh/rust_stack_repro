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
use neq_int::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VLD2_a_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD2_a_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    T: bool,
    a: bool,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ebytes: i64,
        d2: i64,
        gs_310359: bool,
        ga_354053: i64,
        n: i64,
        d: i64,
        register_index: bool,
        alignment: i64,
        wback: bool,
        gs_310356: bool,
        ga_354055: i64,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        T: bool,
        a: bool,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        T,
        a,
        Rm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
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
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // D s_3_5: lsl s_3_4 s_3_3
        let s_3_5: i64 = s_3_4 << s_3_3;
        // D s_3_6: write-var ebytes <= s_3_5
        fn_state.ebytes = s_3_5;
        // D s_3_7: read-var a:u8
        let s_3_7: bool = fn_state.a;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 1u16);
        // C s_3_9: const #0u : u8
        let s_3_9: bool = false;
        // C s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 1u16);
        // D s_3_11: cmp-eq s_3_8 s_3_10
        let s_3_11: bool = ((s_3_8) == (s_3_10));
        // N s_3_12: branch s_3_11 b17 b4
        if s_3_11 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var ebytes:i64
        let s_4_1: i64 = fn_state.ebytes;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: write-var ga#354053 <= s_4_4
        fn_state.ga_354053 = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#354053:i64
        let s_5_0: i64 = fn_state.ga_354053;
        // D s_5_1: write-var alignment <= s_5_0
        fn_state.alignment = s_5_0;
        // D s_5_2: read-var T:u8
        let s_5_2: bool = fn_state.T;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b16 b6
        if s_5_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var ga#354055 <= s_6_0
        fn_state.ga_354055 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#354055:i64
        let s_7_0: i64 = fn_state.ga_354055;
        // D s_7_1: read-var D:u8
        let s_7_1: bool = fn_state.D;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 1u16);
        // D s_7_3: read-var Vd:u8
        let s_7_3: u8 = fn_state.Vd;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 4u16);
        // D s_7_5: cast reint s_7_2 -> u128
        let s_7_5: u128 = (s_7_2.value() as u128);
        // D s_7_6: size-of s_7_2
        let s_7_6: u16 = s_7_2.length();
        // D s_7_7: cast reint s_7_4 -> u128
        let s_7_7: u128 = (s_7_4.value() as u128);
        // D s_7_8: size-of s_7_4
        let s_7_8: u16 = s_7_4.length();
        // D s_7_9: lsl s_7_5 s_7_8
        let s_7_9: u128 = s_7_5 << s_7_8;
        // D s_7_10: or s_7_9 s_7_7
        let s_7_10: u128 = ((s_7_9) | (s_7_7));
        // D s_7_11: add s_7_6 s_7_8
        let s_7_11: u16 = (s_7_6 + s_7_8);
        // D s_7_12: create-bits s_7_10 s_7_11
        let s_7_12: Bits = Bits::new(s_7_10, s_7_11);
        // D s_7_13: cast reint s_7_12 -> u8
        let s_7_13: u8 = (s_7_12.value() as u8);
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 5u16);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (s_7_14.value() as i128);
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: write-var d <= s_7_16
        fn_state.d = s_7_16;
        // D s_7_18: read-var d:i64
        let s_7_18: i64 = fn_state.d;
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: cast zx s_7_0 -> i
        let s_7_20: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_21: add s_7_19 s_7_20
        let s_7_21: i128 = (s_7_19 + s_7_20);
        // D s_7_22: cast reint s_7_21 -> i64
        let s_7_22: i64 = (s_7_21 as i64);
        // D s_7_23: write-var d2 <= s_7_22
        fn_state.d2 = s_7_22;
        // D s_7_24: read-var Rn:u8
        let s_7_24: u8 = fn_state.Rn;
        // D s_7_25: cast zx s_7_24 -> bv
        let s_7_25: Bits = Bits::new(s_7_24 as u128, 4u16);
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (s_7_25.value() as i128);
        // D s_7_27: cast reint s_7_26 -> i64
        let s_7_27: i64 = (s_7_26 as i64);
        // D s_7_28: write-var n <= s_7_27
        fn_state.n = s_7_27;
        // D s_7_29: read-var Rm:u8
        let s_7_29: u8 = fn_state.Rm;
        // D s_7_30: cast zx s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 4u16);
        // D s_7_31: cast zx s_7_30 -> i
        let s_7_31: i128 = (s_7_30.value() as i128);
        // D s_7_32: cast reint s_7_31 -> i64
        let s_7_32: i64 = (s_7_31 as i64);
        // D s_7_33: write-var m <= s_7_32
        fn_state.m = s_7_32;
        // C s_7_34: const #15s : i
        let s_7_34: i128 = 15;
        // D s_7_35: read-var m:i64
        let s_7_35: i64 = fn_state.m;
        // D s_7_36: cast zx s_7_35 -> i
        let s_7_36: i128 = (i128::try_from(s_7_35).unwrap());
        // D s_7_37: call neq_int(s_7_36, s_7_34)
        let s_7_37: bool = neq_int(state, tracer, s_7_36, s_7_34);
        // D s_7_38: write-var wback <= s_7_37
        fn_state.wback = s_7_37;
        // C s_7_39: const #15s : i
        let s_7_39: i128 = 15;
        // D s_7_40: read-var m:i64
        let s_7_40: i64 = fn_state.m;
        // D s_7_41: cast zx s_7_40 -> i
        let s_7_41: i128 = (i128::try_from(s_7_40).unwrap());
        // D s_7_42: call neq_int(s_7_41, s_7_39)
        let s_7_42: bool = neq_int(state, tracer, s_7_41, s_7_39);
        // N s_7_43: branch s_7_42 b15 b8
        if s_7_42 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#310356 <= s_8_0
        fn_state.gs_310356 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#310356:u8
        let s_9_0: bool = fn_state.gs_310356;
        // D s_9_1: write-var register_index <= s_9_0
        fn_state.register_index = s_9_0;
        // C s_9_2: const #15s : i
        let s_9_2: i128 = 15;
        // D s_9_3: read-var n:i64
        let s_9_3: i64 = fn_state.n;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cmp-eq s_9_4 s_9_2
        let s_9_5: bool = ((s_9_4) == (s_9_2));
        // N s_9_6: branch s_9_5 b14 b10
        if s_9_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #31s : i
        let s_10_0: i128 = 31;
        // D s_10_1: read-var d2:i64
        let s_10_1: i64 = fn_state.d2;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-gt s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) > (s_10_0));
        // D s_10_4: write-var gs#310359 <= s_10_3
        fn_state.gs_310359 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#310359:u8
        let s_11_0: bool = fn_state.gs_310359;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
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
        // D s_12_0: read-var d2:i64
        let s_12_0: i64 = fn_state.d2;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var alignment:i64
        let s_12_2: i64 = fn_state.alignment;
        // D s_12_3: read-var d:i64
        let s_12_3: i64 = fn_state.d;
        // D s_12_4: read-var ebytes:i64
        let s_12_4: i64 = fn_state.ebytes;
        // D s_12_5: read-var m:i64
        let s_12_5: i64 = fn_state.m;
        // D s_12_6: read-var n:i64
        let s_12_6: i64 = fn_state.n;
        // D s_12_7: read-var register_index:u8
        let s_12_7: bool = fn_state.register_index;
        // D s_12_8: read-var wback:u8
        let s_12_8: bool = fn_state.wback;
        // D s_12_9: call execute_aarch32_instrs_VLD2_a_Op_A_txt(s_12_2, s_12_3, s_12_1, s_12_4, s_12_5, s_12_6, s_12_7, s_12_8)
        let s_12_9: () = execute_aarch32_instrs_VLD2_a_Op_A_txt(
            state,
            tracer,
            s_12_2,
            s_12_3,
            s_12_1,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_7,
            s_12_8,
        );
        // N s_12_10: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#310359 <= s_14_0
        fn_state.gs_310359 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #13s : i
        let s_15_0: i128 = 13;
        // D s_15_1: read-var m:i64
        let s_15_1: i64 = fn_state.m;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: call neq_int(s_15_2, s_15_0)
        let s_15_3: bool = neq_int(state, tracer, s_15_2, s_15_0);
        // D s_15_4: write-var gs#310356 <= s_15_3
        fn_state.gs_310356 = s_15_3;
        // N s_15_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i64
        let s_16_0: i64 = 1;
        // D s_16_1: write-var ga#354055 <= s_16_0
        fn_state.ga_354055 = s_16_0;
        // N s_16_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#354053 <= s_17_0
        fn_state.ga_354053 = s_17_0;
        // N s_17_2: jump b5
        return block_5(state, tracer, fn_state);
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
