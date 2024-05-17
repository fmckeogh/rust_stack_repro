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
use execute_aarch32_instrs_VPADD_f_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VPADD_f_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    sz: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        elements: i64,
        D: bool,
        sz: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        sz,
        Vn,
        Vd,
        N,
        Q,
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
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b7 b3
        if s_2_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i64
        let s_3_0: i64 = 16;
        // D s_3_1: write-var esize <= s_3_0
        fn_state.esize = s_3_0;
        // C s_3_2: const #2s : i64
        let s_3_2: i64 = 2;
        // D s_3_3: write-var elements <= s_3_2
        fn_state.elements = s_3_2;
        // D s_3_4: read-var sz:u8
        let s_3_4: bool = fn_state.sz;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // C s_3_6: const #0u : u8
        let s_3_6: bool = false;
        // C s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // D s_3_8: cmp-eq s_3_5 s_3_7
        let s_3_8: bool = ((s_3_5) == (s_3_7));
        // D s_3_9: not s_3_8
        let s_3_9: bool = !s_3_8;
        // N s_3_10: branch s_3_9 b6 b4
        if s_3_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i64
        let s_4_0: i64 = 32;
        // D s_4_1: write-var esize <= s_4_0
        fn_state.esize = s_4_0;
        // C s_4_2: const #2s : i64
        let s_4_2: i64 = 2;
        // D s_4_3: write-var elements <= s_4_2
        fn_state.elements = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: read-var elements:i64
        let s_5_1: i64 = fn_state.elements;
        // D s_5_2: read-var D:u8
        let s_5_2: bool = fn_state.D;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: read-var Vd:u8
        let s_5_4: u8 = fn_state.Vd;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 4u16);
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: cast reint s_5_5 -> u128
        let s_5_8: u128 = (s_5_5.value() as u128);
        // D s_5_9: size-of s_5_5
        let s_5_9: u16 = s_5_5.length();
        // D s_5_10: lsl s_5_6 s_5_9
        let s_5_10: u128 = s_5_6 << s_5_9;
        // D s_5_11: or s_5_10 s_5_8
        let s_5_11: u128 = ((s_5_10) | (s_5_8));
        // D s_5_12: add s_5_7 s_5_9
        let s_5_12: u16 = (s_5_7 + s_5_9);
        // D s_5_13: create-bits s_5_11 s_5_12
        let s_5_13: Bits = Bits::new(s_5_11, s_5_12);
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: u8 = (s_5_13.value() as u8);
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 5u16);
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (s_5_15.value() as i128);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: read-var N:u8
        let s_5_18: bool = fn_state.N;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 1u16);
        // D s_5_20: read-var Vn:u8
        let s_5_20: u8 = fn_state.Vn;
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 4u16);
        // D s_5_22: cast reint s_5_19 -> u128
        let s_5_22: u128 = (s_5_19.value() as u128);
        // D s_5_23: size-of s_5_19
        let s_5_23: u16 = s_5_19.length();
        // D s_5_24: cast reint s_5_21 -> u128
        let s_5_24: u128 = (s_5_21.value() as u128);
        // D s_5_25: size-of s_5_21
        let s_5_25: u16 = s_5_21.length();
        // D s_5_26: lsl s_5_22 s_5_25
        let s_5_26: u128 = s_5_22 << s_5_25;
        // D s_5_27: or s_5_26 s_5_24
        let s_5_27: u128 = ((s_5_26) | (s_5_24));
        // D s_5_28: add s_5_23 s_5_25
        let s_5_28: u16 = (s_5_23 + s_5_25);
        // D s_5_29: create-bits s_5_27 s_5_28
        let s_5_29: Bits = Bits::new(s_5_27, s_5_28);
        // D s_5_30: cast reint s_5_29 -> u8
        let s_5_30: u8 = (s_5_29.value() as u8);
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 5u16);
        // D s_5_32: cast zx s_5_31 -> i
        let s_5_32: i128 = (s_5_31.value() as i128);
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // D s_5_34: read-var M:u8
        let s_5_34: bool = fn_state.M;
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 1u16);
        // D s_5_36: read-var Vm:u8
        let s_5_36: u8 = fn_state.Vm;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 4u16);
        // D s_5_38: cast reint s_5_35 -> u128
        let s_5_38: u128 = (s_5_35.value() as u128);
        // D s_5_39: size-of s_5_35
        let s_5_39: u16 = s_5_35.length();
        // D s_5_40: cast reint s_5_37 -> u128
        let s_5_40: u128 = (s_5_37.value() as u128);
        // D s_5_41: size-of s_5_37
        let s_5_41: u16 = s_5_37.length();
        // D s_5_42: lsl s_5_38 s_5_41
        let s_5_42: u128 = s_5_38 << s_5_41;
        // D s_5_43: or s_5_42 s_5_40
        let s_5_43: u128 = ((s_5_42) | (s_5_40));
        // D s_5_44: add s_5_39 s_5_41
        let s_5_44: u16 = (s_5_39 + s_5_41);
        // D s_5_45: create-bits s_5_43 s_5_44
        let s_5_45: Bits = Bits::new(s_5_43, s_5_44);
        // D s_5_46: cast reint s_5_45 -> u8
        let s_5_46: u8 = (s_5_45.value() as u8);
        // D s_5_47: cast zx s_5_46 -> bv
        let s_5_47: Bits = Bits::new(s_5_46 as u128, 5u16);
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (s_5_47.value() as i128);
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // D s_5_50: cast zx s_5_0 -> i
        let s_5_50: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_51: cast reint s_5_50 -> i64
        let s_5_51: i64 = (s_5_50 as i64);
        // D s_5_52: call execute_aarch32_instrs_VPADD_f_Op_A_txt(s_5_17, s_5_1, s_5_51, s_5_49, s_5_33)
        let s_5_52: () = execute_aarch32_instrs_VPADD_f_Op_A_txt(
            state,
            tracer,
            s_5_17,
            s_5_1,
            s_5_51,
            s_5_49,
            s_5_33,
        );
        // N s_5_53: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #4s : i64
        let s_6_2: i64 = 4;
        // D s_6_3: write-var elements <= s_6_2
        fn_state.elements = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
