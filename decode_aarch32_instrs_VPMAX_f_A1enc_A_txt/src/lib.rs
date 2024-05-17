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
use execute_aarch32_instrs_VPMAX_f_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VPMAX_f_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    op: bool,
    sz: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        elements: i64,
        maximum: bool,
        D: bool,
        op: bool,
        sz: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        op,
        sz,
        Vn,
        Vd,
        N,
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
        // D s_2_0: read-var op:u8
        let s_2_0: bool = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var maximum <= s_2_4
        fn_state.maximum = s_2_4;
        // C s_2_6: const #16s : i64
        let s_2_6: i64 = 16;
        // D s_2_7: write-var esize <= s_2_6
        fn_state.esize = s_2_6;
        // C s_2_8: const #2s : i64
        let s_2_8: i64 = 2;
        // D s_2_9: write-var elements <= s_2_8
        fn_state.elements = s_2_8;
        // D s_2_10: read-var sz:u8
        let s_2_10: bool = fn_state.sz;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // C s_2_12: const #0u : u8
        let s_2_12: bool = false;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // D s_2_14: cmp-eq s_2_11 s_2_13
        let s_2_14: bool = ((s_2_11) == (s_2_13));
        // D s_2_15: not s_2_14
        let s_2_15: bool = !s_2_14;
        // N s_2_16: branch s_2_15 b5 b3
        if s_2_15 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: write-var esize <= s_3_0
        fn_state.esize = s_3_0;
        // C s_3_2: const #2s : i64
        let s_3_2: i64 = 2;
        // D s_3_3: write-var elements <= s_3_2
        fn_state.elements = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esize:i64
        let s_4_0: i64 = fn_state.esize;
        // D s_4_1: read-var elements:i64
        let s_4_1: i64 = fn_state.elements;
        // D s_4_2: read-var D:u8
        let s_4_2: bool = fn_state.D;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: read-var Vd:u8
        let s_4_4: u8 = fn_state.Vd;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 4u16);
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: cast reint s_4_5 -> u128
        let s_4_8: u128 = (s_4_5.value() as u128);
        // D s_4_9: size-of s_4_5
        let s_4_9: u16 = s_4_5.length();
        // D s_4_10: lsl s_4_6 s_4_9
        let s_4_10: u128 = s_4_6 << s_4_9;
        // D s_4_11: or s_4_10 s_4_8
        let s_4_11: u128 = ((s_4_10) | (s_4_8));
        // D s_4_12: add s_4_7 s_4_9
        let s_4_12: u16 = (s_4_7 + s_4_9);
        // D s_4_13: create-bits s_4_11 s_4_12
        let s_4_13: Bits = Bits::new(s_4_11, s_4_12);
        // D s_4_14: cast reint s_4_13 -> u8
        let s_4_14: u8 = (s_4_13.value() as u8);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 5u16);
        // D s_4_16: cast zx s_4_15 -> i
        let s_4_16: i128 = (s_4_15.value() as i128);
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: read-var N:u8
        let s_4_18: bool = fn_state.N;
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 1u16);
        // D s_4_20: read-var Vn:u8
        let s_4_20: u8 = fn_state.Vn;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 4u16);
        // D s_4_22: cast reint s_4_19 -> u128
        let s_4_22: u128 = (s_4_19.value() as u128);
        // D s_4_23: size-of s_4_19
        let s_4_23: u16 = s_4_19.length();
        // D s_4_24: cast reint s_4_21 -> u128
        let s_4_24: u128 = (s_4_21.value() as u128);
        // D s_4_25: size-of s_4_21
        let s_4_25: u16 = s_4_21.length();
        // D s_4_26: lsl s_4_22 s_4_25
        let s_4_26: u128 = s_4_22 << s_4_25;
        // D s_4_27: or s_4_26 s_4_24
        let s_4_27: u128 = ((s_4_26) | (s_4_24));
        // D s_4_28: add s_4_23 s_4_25
        let s_4_28: u16 = (s_4_23 + s_4_25);
        // D s_4_29: create-bits s_4_27 s_4_28
        let s_4_29: Bits = Bits::new(s_4_27, s_4_28);
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: u8 = (s_4_29.value() as u8);
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 5u16);
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (s_4_31.value() as i128);
        // D s_4_33: cast reint s_4_32 -> i64
        let s_4_33: i64 = (s_4_32 as i64);
        // D s_4_34: read-var M:u8
        let s_4_34: bool = fn_state.M;
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 1u16);
        // D s_4_36: read-var Vm:u8
        let s_4_36: u8 = fn_state.Vm;
        // D s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 4u16);
        // D s_4_38: cast reint s_4_35 -> u128
        let s_4_38: u128 = (s_4_35.value() as u128);
        // D s_4_39: size-of s_4_35
        let s_4_39: u16 = s_4_35.length();
        // D s_4_40: cast reint s_4_37 -> u128
        let s_4_40: u128 = (s_4_37.value() as u128);
        // D s_4_41: size-of s_4_37
        let s_4_41: u16 = s_4_37.length();
        // D s_4_42: lsl s_4_38 s_4_41
        let s_4_42: u128 = s_4_38 << s_4_41;
        // D s_4_43: or s_4_42 s_4_40
        let s_4_43: u128 = ((s_4_42) | (s_4_40));
        // D s_4_44: add s_4_39 s_4_41
        let s_4_44: u16 = (s_4_39 + s_4_41);
        // D s_4_45: create-bits s_4_43 s_4_44
        let s_4_45: Bits = Bits::new(s_4_43, s_4_44);
        // D s_4_46: cast reint s_4_45 -> u8
        let s_4_46: u8 = (s_4_45.value() as u8);
        // D s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 5u16);
        // D s_4_48: cast zx s_4_47 -> i
        let s_4_48: i128 = (s_4_47.value() as i128);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: cast zx s_4_0 -> i
        let s_4_50: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_51: cast reint s_4_50 -> i64
        let s_4_51: i64 = (s_4_50 as i64);
        // D s_4_52: read-var maximum:u8
        let s_4_52: bool = fn_state.maximum;
        // D s_4_53: call execute_aarch32_instrs_VPMAX_f_Op_A_txt(s_4_17, s_4_1, s_4_51, s_4_49, s_4_52, s_4_33)
        let s_4_53: () = execute_aarch32_instrs_VPMAX_f_Op_A_txt(
            state,
            tracer,
            s_4_17,
            s_4_1,
            s_4_51,
            s_4_49,
            s_4_52,
            s_4_33,
        );
        // N s_4_54: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: write-var esize <= s_5_0
        fn_state.esize = s_5_0;
        // C s_5_2: const #4s : i64
        let s_5_2: i64 = 4;
        // D s_5_3: write-var elements <= s_5_2
        fn_state.elements = s_5_2;
        // N s_5_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
