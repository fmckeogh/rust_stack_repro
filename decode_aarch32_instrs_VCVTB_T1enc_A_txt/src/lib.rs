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
use ConditionPassed::*;
use execute_aarch32_instrs_VCVTB_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCVTB_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    op: bool,
    Vd: u8,
    sz: bool,
    T: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_352249: i64,
        convert_from_half: bool,
        lowbit: i64,
        d: i64,
        uses_double: bool,
        D: bool,
        op: bool,
        Vd: u8,
        sz: bool,
        T: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        op,
        Vd,
        sz,
        T,
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
        // D s_2_0: read-var sz:u8
        let s_2_0: bool = fn_state.sz;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var uses_double <= s_2_4
        fn_state.uses_double = s_2_4;
        // D s_2_6: read-var op:u8
        let s_2_6: bool = fn_state.op;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // D s_2_11: write-var convert_from_half <= s_2_10
        fn_state.convert_from_half = s_2_10;
        // D s_2_12: read-var T:u8
        let s_2_12: bool = fn_state.T;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // C s_2_14: const #1u : u8
        let s_2_14: bool = true;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: cmp-eq s_2_13 s_2_15
        let s_2_16: bool = ((s_2_13) == (s_2_15));
        // N s_2_17: branch s_2_16 b10 b3
        if s_2_16 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var ga#352249 <= s_3_0
        fn_state.ga_352249 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#352249:i64
        let s_4_0: i64 = fn_state.ga_352249;
        // D s_4_1: write-var lowbit <= s_4_0
        fn_state.lowbit = s_4_0;
        // D s_4_2: read-var uses_double:u8
        let s_4_2: bool = fn_state.uses_double;
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Vd:u8
        let s_5_0: u8 = fn_state.Vd;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: read-var D:u8
        let s_5_2: bool = fn_state.D;
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
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var Vm:u8
        let s_5_17: u8 = fn_state.Vm;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 4u16);
        // D s_5_19: read-var M:u8
        let s_5_19: bool = fn_state.M;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cast reint s_5_18 -> u128
        let s_5_21: u128 = (s_5_18.value() as u128);
        // D s_5_22: size-of s_5_18
        let s_5_22: u16 = s_5_18.length();
        // D s_5_23: cast reint s_5_20 -> u128
        let s_5_23: u128 = (s_5_20.value() as u128);
        // D s_5_24: size-of s_5_20
        let s_5_24: u16 = s_5_20.length();
        // D s_5_25: lsl s_5_21 s_5_24
        let s_5_25: u128 = s_5_21 << s_5_24;
        // D s_5_26: or s_5_25 s_5_23
        let s_5_26: u128 = ((s_5_25) | (s_5_23));
        // D s_5_27: add s_5_22 s_5_24
        let s_5_27: u16 = (s_5_22 + s_5_24);
        // D s_5_28: create-bits s_5_26 s_5_27
        let s_5_28: Bits = Bits::new(s_5_26, s_5_27);
        // D s_5_29: cast reint s_5_28 -> u8
        let s_5_29: u8 = (s_5_28.value() as u8);
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 5u16);
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (s_5_30.value() as i128);
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // D s_5_33: write-var m <= s_5_32
        fn_state.m = s_5_32;
        // N s_5_34: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var m:i64
        let s_6_0: i64 = fn_state.m;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: read-var convert_from_half:u8
        let s_6_2: bool = fn_state.convert_from_half;
        // D s_6_3: read-var lowbit:i64
        let s_6_3: i64 = fn_state.lowbit;
        // D s_6_4: read-var uses_double:u8
        let s_6_4: bool = fn_state.uses_double;
        // D s_6_5: call execute_aarch32_instrs_VCVTB_Op_A_txt(s_6_2, s_6_1, s_6_3, s_6_0, s_6_4)
        let s_6_5: () = execute_aarch32_instrs_VCVTB_Op_A_txt(
            state,
            tracer,
            s_6_2,
            s_6_1,
            s_6_3,
            s_6_0,
            s_6_4,
        );
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var convert_from_half:u8
        let s_7_0: bool = fn_state.convert_from_half;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var Vd:u8
        let s_8_0: u8 = fn_state.Vd;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // D s_8_2: read-var D:u8
        let s_8_2: bool = fn_state.D;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cast reint s_8_1 -> u128
        let s_8_4: u128 = (s_8_1.value() as u128);
        // D s_8_5: size-of s_8_1
        let s_8_5: u16 = s_8_1.length();
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: lsl s_8_4 s_8_7
        let s_8_8: u128 = s_8_4 << s_8_7;
        // D s_8_9: or s_8_8 s_8_6
        let s_8_9: u128 = ((s_8_8) | (s_8_6));
        // D s_8_10: add s_8_5 s_8_7
        let s_8_10: u16 = (s_8_5 + s_8_7);
        // D s_8_11: create-bits s_8_9 s_8_10
        let s_8_11: Bits = Bits::new(s_8_9, s_8_10);
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: u8 = (s_8_11.value() as u8);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 5u16);
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (s_8_13.value() as i128);
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: write-var d <= s_8_15
        fn_state.d = s_8_15;
        // D s_8_17: read-var M:u8
        let s_8_17: bool = fn_state.M;
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // D s_8_19: read-var Vm:u8
        let s_8_19: u8 = fn_state.Vm;
        // D s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 4u16);
        // D s_8_21: cast reint s_8_18 -> u128
        let s_8_21: u128 = (s_8_18.value() as u128);
        // D s_8_22: size-of s_8_18
        let s_8_22: u16 = s_8_18.length();
        // D s_8_23: cast reint s_8_20 -> u128
        let s_8_23: u128 = (s_8_20.value() as u128);
        // D s_8_24: size-of s_8_20
        let s_8_24: u16 = s_8_20.length();
        // D s_8_25: lsl s_8_21 s_8_24
        let s_8_25: u128 = s_8_21 << s_8_24;
        // D s_8_26: or s_8_25 s_8_23
        let s_8_26: u128 = ((s_8_25) | (s_8_23));
        // D s_8_27: add s_8_22 s_8_24
        let s_8_27: u16 = (s_8_22 + s_8_24);
        // D s_8_28: create-bits s_8_26 s_8_27
        let s_8_28: Bits = Bits::new(s_8_26, s_8_27);
        // D s_8_29: cast reint s_8_28 -> u8
        let s_8_29: u8 = (s_8_28.value() as u8);
        // D s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 5u16);
        // D s_8_31: cast zx s_8_30 -> i
        let s_8_31: i128 = (s_8_30.value() as i128);
        // D s_8_32: cast reint s_8_31 -> i64
        let s_8_32: i64 = (s_8_31 as i64);
        // D s_8_33: write-var m <= s_8_32
        fn_state.m = s_8_32;
        // N s_8_34: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var D:u8
        let s_9_0: bool = fn_state.D;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // D s_9_2: read-var Vd:u8
        let s_9_2: u8 = fn_state.Vd;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 5u16);
        // D s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (s_9_13.value() as i128);
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: write-var d <= s_9_15
        fn_state.d = s_9_15;
        // D s_9_17: read-var Vm:u8
        let s_9_17: u8 = fn_state.Vm;
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 4u16);
        // D s_9_19: read-var M:u8
        let s_9_19: bool = fn_state.M;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cast reint s_9_18 -> u128
        let s_9_21: u128 = (s_9_18.value() as u128);
        // D s_9_22: size-of s_9_18
        let s_9_22: u16 = s_9_18.length();
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: lsl s_9_21 s_9_24
        let s_9_25: u128 = s_9_21 << s_9_24;
        // D s_9_26: or s_9_25 s_9_23
        let s_9_26: u128 = ((s_9_25) | (s_9_23));
        // D s_9_27: add s_9_22 s_9_24
        let s_9_27: u16 = (s_9_22 + s_9_24);
        // D s_9_28: create-bits s_9_26 s_9_27
        let s_9_28: Bits = Bits::new(s_9_26, s_9_27);
        // D s_9_29: cast reint s_9_28 -> u8
        let s_9_29: u8 = (s_9_28.value() as u8);
        // D s_9_30: cast zx s_9_29 -> bv
        let s_9_30: Bits = Bits::new(s_9_29 as u128, 5u16);
        // D s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (s_9_30.value() as i128);
        // D s_9_32: cast reint s_9_31 -> i64
        let s_9_32: i64 = (s_9_31 as i64);
        // D s_9_33: write-var m <= s_9_32
        fn_state.m = s_9_32;
        // N s_9_34: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16s : i64
        let s_10_0: i64 = 16;
        // D s_10_1: write-var ga#352249 <= s_10_0
        fn_state.ga_352249 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
