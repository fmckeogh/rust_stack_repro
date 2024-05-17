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
use InITBlock::*;
use execute_aarch32_instrs_VMAXNM_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMAXNM_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    size: u8,
    N: bool,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_325360: bool,
        esize: i64,
        n: i64,
        ga_365842: i128,
        ga_365844: i128,
        d: i64,
        maximum: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        size: u8,
        N: bool,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        size,
        N,
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
        // N s_0_2: branch s_0_1 b14 b1
        if s_0_1 {
            return block_14(state, tracer, fn_state);
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
        // N s_1_5: branch s_1_4 b13 b2
        if s_1_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#325360 <= s_2_0
        fn_state.gs_325360 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#325360:u8
        let s_3_0: bool = fn_state.gs_325360;
        // N s_3_1: branch s_3_0 b12 b4
        if s_3_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u8
        let s_4_0: bool = fn_state.op;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var maximum <= s_4_4
        fn_state.maximum = s_4_4;
        // C s_4_6: const #16s : i64
        let s_4_6: i64 = 16;
        // D s_4_7: write-var esize <= s_4_6
        fn_state.esize = s_4_6;
        // D s_4_8: read-var size:u8
        let s_4_8: u8 = fn_state.size;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 2u16);
        // C s_4_10: const #1u : u8
        let s_4_10: u8 = 1;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cmp-eq s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) == (s_4_11));
        // D s_4_13: not s_4_12
        let s_4_13: bool = !s_4_12;
        // N s_4_14: branch s_4_13 b7 b5
        if s_4_13 {
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
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: write-var esize <= s_5_0
        fn_state.esize = s_5_0;
        // D s_5_2: read-var Vd:u8
        let s_5_2: u8 = fn_state.Vd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: read-var D:u8
        let s_5_4: bool = fn_state.D;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
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
        // D s_5_18: write-var d <= s_5_17
        fn_state.d = s_5_17;
        // D s_5_19: read-var Vn:u8
        let s_5_19: u8 = fn_state.Vn;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 4u16);
        // D s_5_21: read-var N:u8
        let s_5_21: bool = fn_state.N;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_23: cast reint s_5_20 -> u128
        let s_5_23: u128 = (s_5_20.value() as u128);
        // D s_5_24: size-of s_5_20
        let s_5_24: u16 = s_5_20.length();
        // D s_5_25: cast reint s_5_22 -> u128
        let s_5_25: u128 = (s_5_22.value() as u128);
        // D s_5_26: size-of s_5_22
        let s_5_26: u16 = s_5_22.length();
        // D s_5_27: lsl s_5_23 s_5_26
        let s_5_27: u128 = s_5_23 << s_5_26;
        // D s_5_28: or s_5_27 s_5_25
        let s_5_28: u128 = ((s_5_27) | (s_5_25));
        // D s_5_29: add s_5_24 s_5_26
        let s_5_29: u16 = (s_5_24 + s_5_26);
        // D s_5_30: create-bits s_5_28 s_5_29
        let s_5_30: Bits = Bits::new(s_5_28, s_5_29);
        // D s_5_31: cast reint s_5_30 -> u8
        let s_5_31: u8 = (s_5_30.value() as u8);
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 5u16);
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (s_5_32.value() as i128);
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: write-var n <= s_5_34
        fn_state.n = s_5_34;
        // D s_5_36: read-var Vm:u8
        let s_5_36: u8 = fn_state.Vm;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 4u16);
        // D s_5_38: read-var M:u8
        let s_5_38: bool = fn_state.M;
        // D s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 1u16);
        // D s_5_40: cast reint s_5_37 -> u128
        let s_5_40: u128 = (s_5_37.value() as u128);
        // D s_5_41: size-of s_5_37
        let s_5_41: u16 = s_5_37.length();
        // D s_5_42: cast reint s_5_39 -> u128
        let s_5_42: u128 = (s_5_39.value() as u128);
        // D s_5_43: size-of s_5_39
        let s_5_43: u16 = s_5_39.length();
        // D s_5_44: lsl s_5_40 s_5_43
        let s_5_44: u128 = s_5_40 << s_5_43;
        // D s_5_45: or s_5_44 s_5_42
        let s_5_45: u128 = ((s_5_44) | (s_5_42));
        // D s_5_46: add s_5_41 s_5_43
        let s_5_46: u16 = (s_5_41 + s_5_43);
        // D s_5_47: create-bits s_5_45 s_5_46
        let s_5_47: Bits = Bits::new(s_5_45, s_5_46);
        // D s_5_48: cast reint s_5_47 -> u8
        let s_5_48: u8 = (s_5_47.value() as u8);
        // D s_5_49: cast zx s_5_48 -> bv
        let s_5_49: Bits = Bits::new(s_5_48 as u128, 5u16);
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (s_5_49.value() as i128);
        // D s_5_51: cast reint s_5_50 -> i64
        let s_5_51: i64 = (s_5_50 as i64);
        // D s_5_52: write-var m <= s_5_51
        fn_state.m = s_5_51;
        // N s_5_53: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var n:i64
        let s_6_0: i64 = fn_state.n;
        // D s_6_1: read-var m:i64
        let s_6_1: i64 = fn_state.m;
        // D s_6_2: read-var esize:i64
        let s_6_2: i64 = fn_state.esize;
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
        // D s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #0u : u8
        let s_6_6: bool = false;
        // D s_6_7: read-var ga#365842:i
        let s_6_7: i128 = fn_state.ga_365842;
        // D s_6_8: read-var maximum:u8
        let s_6_8: bool = fn_state.maximum;
        // D s_6_9: read-var ga#365844:i
        let s_6_9: i128 = fn_state.ga_365844;
        // D s_6_10: call execute_aarch32_instrs_VMAXNM_Op_A_txt(s_6_6, s_6_3, s_6_7, s_6_5, s_6_1, s_6_8, s_6_0, s_6_9)
        let s_6_10: () = execute_aarch32_instrs_VMAXNM_Op_A_txt(
            state,
            tracer,
            s_6_6,
            s_6_3,
            s_6_7,
            s_6_5,
            s_6_1,
            s_6_8,
            s_6_0,
            s_6_9,
        );
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:u8
        let s_7_0: u8 = fn_state.size;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
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
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // D s_8_2: read-var Vd:u8
        let s_8_2: u8 = fn_state.Vd;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 4u16);
        // D s_8_4: read-var D:u8
        let s_8_4: bool = fn_state.D;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: cast reint s_8_5 -> u128
        let s_8_8: u128 = (s_8_5.value() as u128);
        // D s_8_9: size-of s_8_5
        let s_8_9: u16 = s_8_5.length();
        // D s_8_10: lsl s_8_6 s_8_9
        let s_8_10: u128 = s_8_6 << s_8_9;
        // D s_8_11: or s_8_10 s_8_8
        let s_8_11: u128 = ((s_8_10) | (s_8_8));
        // D s_8_12: add s_8_7 s_8_9
        let s_8_12: u16 = (s_8_7 + s_8_9);
        // D s_8_13: create-bits s_8_11 s_8_12
        let s_8_13: Bits = Bits::new(s_8_11, s_8_12);
        // D s_8_14: cast reint s_8_13 -> u8
        let s_8_14: u8 = (s_8_13.value() as u8);
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 5u16);
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (s_8_15.value() as i128);
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // D s_8_18: write-var d <= s_8_17
        fn_state.d = s_8_17;
        // D s_8_19: read-var Vn:u8
        let s_8_19: u8 = fn_state.Vn;
        // D s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 4u16);
        // D s_8_21: read-var N:u8
        let s_8_21: bool = fn_state.N;
        // D s_8_22: cast zx s_8_21 -> bv
        let s_8_22: Bits = Bits::new(s_8_21 as u128, 1u16);
        // D s_8_23: cast reint s_8_20 -> u128
        let s_8_23: u128 = (s_8_20.value() as u128);
        // D s_8_24: size-of s_8_20
        let s_8_24: u16 = s_8_20.length();
        // D s_8_25: cast reint s_8_22 -> u128
        let s_8_25: u128 = (s_8_22.value() as u128);
        // D s_8_26: size-of s_8_22
        let s_8_26: u16 = s_8_22.length();
        // D s_8_27: lsl s_8_23 s_8_26
        let s_8_27: u128 = s_8_23 << s_8_26;
        // D s_8_28: or s_8_27 s_8_25
        let s_8_28: u128 = ((s_8_27) | (s_8_25));
        // D s_8_29: add s_8_24 s_8_26
        let s_8_29: u16 = (s_8_24 + s_8_26);
        // D s_8_30: create-bits s_8_28 s_8_29
        let s_8_30: Bits = Bits::new(s_8_28, s_8_29);
        // D s_8_31: cast reint s_8_30 -> u8
        let s_8_31: u8 = (s_8_30.value() as u8);
        // D s_8_32: cast zx s_8_31 -> bv
        let s_8_32: Bits = Bits::new(s_8_31 as u128, 5u16);
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (s_8_32.value() as i128);
        // D s_8_34: cast reint s_8_33 -> i64
        let s_8_34: i64 = (s_8_33 as i64);
        // D s_8_35: write-var n <= s_8_34
        fn_state.n = s_8_34;
        // D s_8_36: read-var Vm:u8
        let s_8_36: u8 = fn_state.Vm;
        // D s_8_37: cast zx s_8_36 -> bv
        let s_8_37: Bits = Bits::new(s_8_36 as u128, 4u16);
        // D s_8_38: read-var M:u8
        let s_8_38: bool = fn_state.M;
        // D s_8_39: cast zx s_8_38 -> bv
        let s_8_39: Bits = Bits::new(s_8_38 as u128, 1u16);
        // D s_8_40: cast reint s_8_37 -> u128
        let s_8_40: u128 = (s_8_37.value() as u128);
        // D s_8_41: size-of s_8_37
        let s_8_41: u16 = s_8_37.length();
        // D s_8_42: cast reint s_8_39 -> u128
        let s_8_42: u128 = (s_8_39.value() as u128);
        // D s_8_43: size-of s_8_39
        let s_8_43: u16 = s_8_39.length();
        // D s_8_44: lsl s_8_40 s_8_43
        let s_8_44: u128 = s_8_40 << s_8_43;
        // D s_8_45: or s_8_44 s_8_42
        let s_8_45: u128 = ((s_8_44) | (s_8_42));
        // D s_8_46: add s_8_41 s_8_43
        let s_8_46: u16 = (s_8_41 + s_8_43);
        // D s_8_47: create-bits s_8_45 s_8_46
        let s_8_47: Bits = Bits::new(s_8_45, s_8_46);
        // D s_8_48: cast reint s_8_47 -> u8
        let s_8_48: u8 = (s_8_47.value() as u8);
        // D s_8_49: cast zx s_8_48 -> bv
        let s_8_49: Bits = Bits::new(s_8_48 as u128, 5u16);
        // D s_8_50: cast zx s_8_49 -> i
        let s_8_50: i128 = (s_8_49.value() as i128);
        // D s_8_51: cast reint s_8_50 -> i64
        let s_8_51: i64 = (s_8_50 as i64);
        // D s_8_52: write-var m <= s_8_51
        fn_state.m = s_8_51;
        // N s_8_53: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_9_2: const #3u : u8
        let s_9_2: u8 = 3;
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
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // D s_10_2: read-var D:u8
        let s_10_2: bool = fn_state.D;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: read-var Vd:u8
        let s_10_4: u8 = fn_state.Vd;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 4u16);
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
        // D s_10_18: write-var d <= s_10_17
        fn_state.d = s_10_17;
        // D s_10_19: read-var N:u8
        let s_10_19: bool = fn_state.N;
        // D s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: read-var Vn:u8
        let s_10_21: u8 = fn_state.Vn;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 4u16);
        // D s_10_23: cast reint s_10_20 -> u128
        let s_10_23: u128 = (s_10_20.value() as u128);
        // D s_10_24: size-of s_10_20
        let s_10_24: u16 = s_10_20.length();
        // D s_10_25: cast reint s_10_22 -> u128
        let s_10_25: u128 = (s_10_22.value() as u128);
        // D s_10_26: size-of s_10_22
        let s_10_26: u16 = s_10_22.length();
        // D s_10_27: lsl s_10_23 s_10_26
        let s_10_27: u128 = s_10_23 << s_10_26;
        // D s_10_28: or s_10_27 s_10_25
        let s_10_28: u128 = ((s_10_27) | (s_10_25));
        // D s_10_29: add s_10_24 s_10_26
        let s_10_29: u16 = (s_10_24 + s_10_26);
        // D s_10_30: create-bits s_10_28 s_10_29
        let s_10_30: Bits = Bits::new(s_10_28, s_10_29);
        // D s_10_31: cast reint s_10_30 -> u8
        let s_10_31: u8 = (s_10_30.value() as u8);
        // D s_10_32: cast zx s_10_31 -> bv
        let s_10_32: Bits = Bits::new(s_10_31 as u128, 5u16);
        // D s_10_33: cast zx s_10_32 -> i
        let s_10_33: i128 = (s_10_32.value() as i128);
        // D s_10_34: cast reint s_10_33 -> i64
        let s_10_34: i64 = (s_10_33 as i64);
        // D s_10_35: write-var n <= s_10_34
        fn_state.n = s_10_34;
        // D s_10_36: read-var M:u8
        let s_10_36: bool = fn_state.M;
        // D s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 1u16);
        // D s_10_38: read-var Vm:u8
        let s_10_38: u8 = fn_state.Vm;
        // D s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 4u16);
        // D s_10_40: cast reint s_10_37 -> u128
        let s_10_40: u128 = (s_10_37.value() as u128);
        // D s_10_41: size-of s_10_37
        let s_10_41: u16 = s_10_37.length();
        // D s_10_42: cast reint s_10_39 -> u128
        let s_10_42: u128 = (s_10_39.value() as u128);
        // D s_10_43: size-of s_10_39
        let s_10_43: u16 = s_10_39.length();
        // D s_10_44: lsl s_10_40 s_10_43
        let s_10_44: u128 = s_10_40 << s_10_43;
        // D s_10_45: or s_10_44 s_10_42
        let s_10_45: u128 = ((s_10_44) | (s_10_42));
        // D s_10_46: add s_10_41 s_10_43
        let s_10_46: u16 = (s_10_41 + s_10_43);
        // D s_10_47: create-bits s_10_45 s_10_46
        let s_10_47: Bits = Bits::new(s_10_45, s_10_46);
        // D s_10_48: cast reint s_10_47 -> u8
        let s_10_48: u8 = (s_10_47.value() as u8);
        // D s_10_49: cast zx s_10_48 -> bv
        let s_10_49: Bits = Bits::new(s_10_48 as u128, 5u16);
        // D s_10_50: cast zx s_10_49 -> i
        let s_10_50: i128 = (s_10_49.value() as i128);
        // D s_10_51: cast reint s_10_50 -> i64
        let s_10_51: i64 = (s_10_50 as i64);
        // D s_10_52: write-var m <= s_10_51
        fn_state.m = s_10_51;
        // N s_10_53: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#325360 <= s_13_0
        fn_state.gs_325360 = s_13_0;
        // N s_13_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
