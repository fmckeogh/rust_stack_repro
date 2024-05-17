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
use execute_aarch32_instrs_VCVT_ds_Op_A_txt::*;
use u__id::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_ds_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    Vd: u8,
    size: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        gs_307918: bool,
        double_to_single: bool,
        gs_307919: bool,
        ga_352268: i128,
        gs_307917: bool,
        d: i128,
        ga_352271: i128,
        cond: u8,
        D: bool,
        Vd: u8,
        size: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        Vd,
        size,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var size:u8
        let s_2_6: u8 = fn_state.size;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // C s_2_8: const #3u : u8
        let s_2_8: u8 = 3;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // D s_2_11: write-var double_to_single <= s_2_10
        fn_state.double_to_single = s_2_10;
        // D s_2_12: read-var double_to_single:u8
        let s_2_12: bool = fn_state.double_to_single;
        // N s_2_13: branch s_2_12 b17 b3
        if s_2_12 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var D:u8
        let s_3_0: bool = fn_state.D;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var Vd:u8
        let s_3_2: u8 = fn_state.Vd;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: write-var ga#352268 <= s_3_14
        fn_state.ga_352268 = s_3_14;
        // N s_3_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#352268:i
        let s_4_0: i128 = fn_state.ga_352268;
        // D s_4_1: write-var d <= s_4_0
        fn_state.d = s_4_0;
        // D s_4_2: read-var double_to_single:u8
        let s_4_2: bool = fn_state.double_to_single;
        // N s_4_3: branch s_4_2 b16 b5
        if s_4_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Vm:u8
        let s_5_0: u8 = fn_state.Vm;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: read-var M:u8
        let s_5_2: bool = fn_state.M;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: write-var ga#352271 <= s_5_14
        fn_state.ga_352271 = s_5_14;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#352271:i
        let s_6_0: i128 = fn_state.ga_352271;
        // D s_6_1: write-var m <= s_6_0
        fn_state.m = s_6_0;
        // D s_6_2: read-var m:i
        let s_6_2: i128 = fn_state.m;
        // D s_6_3: call __id(s_6_2)
        let s_6_3: i128 = u__id(state, tracer, s_6_2);
        // C s_6_4: const #0s : i
        let s_6_4: i128 = 0;
        // D s_6_5: cmp-le s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) <= (s_6_3));
        // N s_6_6: branch s_6_5 b9 b7
        if s_6_5 {
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
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#307919 <= s_7_0
        fn_state.gs_307919 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#307919:u8
        let s_8_0: bool = fn_state.gs_307919;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var d:i
        let s_8_2: i128 = fn_state.d;
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var m:i
        let s_8_4: i128 = fn_state.m;
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: read-var double_to_single:u8
        let s_8_6: bool = fn_state.double_to_single;
        // D s_8_7: call execute_aarch32_instrs_VCVT_ds_Op_A_txt(s_8_3, s_8_6, s_8_5)
        let s_8_7: () = execute_aarch32_instrs_VCVT_ds_Op_A_txt(
            state,
            tracer,
            s_8_3,
            s_8_6,
            s_8_5,
        );
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var m:i
        let s_9_0: i128 = fn_state.m;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #31s : i
        let s_9_2: i128 = 31;
        // D s_9_3: cmp-le s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) <= (s_9_2));
        // N s_9_4: branch s_9_3 b12 b10
        if s_9_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#307918 <= s_10_0
        fn_state.gs_307918 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#307918:u8
        let s_11_0: bool = fn_state.gs_307918;
        // D s_11_1: write-var gs#307919 <= s_11_0
        fn_state.gs_307919 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i
        let s_12_0: i128 = fn_state.d;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // C s_12_2: const #0s : i
        let s_12_2: i128 = 0;
        // D s_12_3: cmp-le s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) <= (s_12_1));
        // N s_12_4: branch s_12_3 b15 b13
        if s_12_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#307917 <= s_13_0
        fn_state.gs_307917 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#307917:u8
        let s_14_0: bool = fn_state.gs_307917;
        // D s_14_1: write-var gs#307918 <= s_14_0
        fn_state.gs_307918 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var d:i
        let s_15_0: i128 = fn_state.d;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #31s : i
        let s_15_2: i128 = 31;
        // D s_15_3: cmp-le s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) <= (s_15_2));
        // D s_15_4: write-var gs#307917 <= s_15_3
        fn_state.gs_307917 = s_15_3;
        // N s_15_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var M:u8
        let s_16_0: bool = fn_state.M;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // D s_16_2: read-var Vm:u8
        let s_16_2: u8 = fn_state.Vm;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: cast reint s_16_1 -> u128
        let s_16_4: u128 = (s_16_1.value() as u128);
        // D s_16_5: size-of s_16_1
        let s_16_5: u16 = s_16_1.length();
        // D s_16_6: cast reint s_16_3 -> u128
        let s_16_6: u128 = (s_16_3.value() as u128);
        // D s_16_7: size-of s_16_3
        let s_16_7: u16 = s_16_3.length();
        // D s_16_8: lsl s_16_4 s_16_7
        let s_16_8: u128 = s_16_4 << s_16_7;
        // D s_16_9: or s_16_8 s_16_6
        let s_16_9: u128 = ((s_16_8) | (s_16_6));
        // D s_16_10: add s_16_5 s_16_7
        let s_16_10: u16 = (s_16_5 + s_16_7);
        // D s_16_11: create-bits s_16_9 s_16_10
        let s_16_11: Bits = Bits::new(s_16_9, s_16_10);
        // D s_16_12: cast reint s_16_11 -> u8
        let s_16_12: u8 = (s_16_11.value() as u8);
        // D s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 5u16);
        // D s_16_14: cast zx s_16_13 -> i
        let s_16_14: i128 = (s_16_13.value() as i128);
        // D s_16_15: write-var ga#352271 <= s_16_14
        fn_state.ga_352271 = s_16_14;
        // N s_16_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var Vd:u8
        let s_17_0: u8 = fn_state.Vd;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // D s_17_2: read-var D:u8
        let s_17_2: bool = fn_state.D;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cast reint s_17_1 -> u128
        let s_17_4: u128 = (s_17_1.value() as u128);
        // D s_17_5: size-of s_17_1
        let s_17_5: u16 = s_17_1.length();
        // D s_17_6: cast reint s_17_3 -> u128
        let s_17_6: u128 = (s_17_3.value() as u128);
        // D s_17_7: size-of s_17_3
        let s_17_7: u16 = s_17_3.length();
        // D s_17_8: lsl s_17_4 s_17_7
        let s_17_8: u128 = s_17_4 << s_17_7;
        // D s_17_9: or s_17_8 s_17_6
        let s_17_9: u128 = ((s_17_8) | (s_17_6));
        // D s_17_10: add s_17_5 s_17_7
        let s_17_10: u16 = (s_17_5 + s_17_7);
        // D s_17_11: create-bits s_17_9 s_17_10
        let s_17_11: Bits = Bits::new(s_17_9, s_17_10);
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: u8 = (s_17_11.value() as u8);
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 5u16);
        // D s_17_14: cast zx s_17_13 -> i
        let s_17_14: i128 = (s_17_13.value() as i128);
        // D s_17_15: write-var ga#352268 <= s_17_14
        fn_state.ga_352268 = s_17_14;
        // N s_17_16: jump b4
        return block_4(state, tracer, fn_state);
    }
}
