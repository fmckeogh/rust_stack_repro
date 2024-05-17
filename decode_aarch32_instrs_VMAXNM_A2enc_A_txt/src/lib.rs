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
use execute_aarch32_instrs_VMAXNM_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMAXNM_A2enc_A_txt<T: Tracer>(
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
        gs_325293: bool,
        esize: i64,
        n: i64,
        d: i64,
        ga_365803: i128,
        ga_365805: i128,
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
        // D s_0_0: read-var size:u8
        let s_0_0: u8 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b12 b1
        if s_0_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#325293 <= s_1_0
        fn_state.gs_325293 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#325293:u8
        let s_2_0: bool = fn_state.gs_325293;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op:u8
        let s_3_0: bool = fn_state.op;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var maximum <= s_3_4
        fn_state.maximum = s_3_4;
        // C s_3_6: const #16s : i64
        let s_3_6: i64 = 16;
        // D s_3_7: write-var esize <= s_3_6
        fn_state.esize = s_3_6;
        // D s_3_8: read-var size:u8
        let s_3_8: u8 = fn_state.size;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 2u16);
        // C s_3_10: const #1u : u8
        let s_3_10: u8 = 1;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 2u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // D s_3_13: not s_3_12
        let s_3_13: bool = !s_3_12;
        // N s_3_14: branch s_3_13 b6 b4
        if s_3_13 {
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
        // C s_4_0: const #16s : i64
        let s_4_0: i64 = 16;
        // D s_4_1: write-var esize <= s_4_0
        fn_state.esize = s_4_0;
        // D s_4_2: read-var Vd:u8
        let s_4_2: u8 = fn_state.Vd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: read-var D:u8
        let s_4_4: bool = fn_state.D;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
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
        // D s_4_18: write-var d <= s_4_17
        fn_state.d = s_4_17;
        // D s_4_19: read-var Vn:u8
        let s_4_19: u8 = fn_state.Vn;
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 4u16);
        // D s_4_21: read-var N:u8
        let s_4_21: bool = fn_state.N;
        // D s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 1u16);
        // D s_4_23: cast reint s_4_20 -> u128
        let s_4_23: u128 = (s_4_20.value() as u128);
        // D s_4_24: size-of s_4_20
        let s_4_24: u16 = s_4_20.length();
        // D s_4_25: cast reint s_4_22 -> u128
        let s_4_25: u128 = (s_4_22.value() as u128);
        // D s_4_26: size-of s_4_22
        let s_4_26: u16 = s_4_22.length();
        // D s_4_27: lsl s_4_23 s_4_26
        let s_4_27: u128 = s_4_23 << s_4_26;
        // D s_4_28: or s_4_27 s_4_25
        let s_4_28: u128 = ((s_4_27) | (s_4_25));
        // D s_4_29: add s_4_24 s_4_26
        let s_4_29: u16 = (s_4_24 + s_4_26);
        // D s_4_30: create-bits s_4_28 s_4_29
        let s_4_30: Bits = Bits::new(s_4_28, s_4_29);
        // D s_4_31: cast reint s_4_30 -> u8
        let s_4_31: u8 = (s_4_30.value() as u8);
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 5u16);
        // D s_4_33: cast zx s_4_32 -> i
        let s_4_33: i128 = (s_4_32.value() as i128);
        // D s_4_34: cast reint s_4_33 -> i64
        let s_4_34: i64 = (s_4_33 as i64);
        // D s_4_35: write-var n <= s_4_34
        fn_state.n = s_4_34;
        // D s_4_36: read-var Vm:u8
        let s_4_36: u8 = fn_state.Vm;
        // D s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 4u16);
        // D s_4_38: read-var M:u8
        let s_4_38: bool = fn_state.M;
        // D s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 1u16);
        // D s_4_40: cast reint s_4_37 -> u128
        let s_4_40: u128 = (s_4_37.value() as u128);
        // D s_4_41: size-of s_4_37
        let s_4_41: u16 = s_4_37.length();
        // D s_4_42: cast reint s_4_39 -> u128
        let s_4_42: u128 = (s_4_39.value() as u128);
        // D s_4_43: size-of s_4_39
        let s_4_43: u16 = s_4_39.length();
        // D s_4_44: lsl s_4_40 s_4_43
        let s_4_44: u128 = s_4_40 << s_4_43;
        // D s_4_45: or s_4_44 s_4_42
        let s_4_45: u128 = ((s_4_44) | (s_4_42));
        // D s_4_46: add s_4_41 s_4_43
        let s_4_46: u16 = (s_4_41 + s_4_43);
        // D s_4_47: create-bits s_4_45 s_4_46
        let s_4_47: Bits = Bits::new(s_4_45, s_4_46);
        // D s_4_48: cast reint s_4_47 -> u8
        let s_4_48: u8 = (s_4_47.value() as u8);
        // D s_4_49: cast zx s_4_48 -> bv
        let s_4_49: Bits = Bits::new(s_4_48 as u128, 5u16);
        // D s_4_50: cast zx s_4_49 -> i
        let s_4_50: i128 = (s_4_49.value() as i128);
        // D s_4_51: cast reint s_4_50 -> i64
        let s_4_51: i64 = (s_4_50 as i64);
        // D s_4_52: write-var m <= s_4_51
        fn_state.m = s_4_51;
        // N s_4_53: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: read-var esize:i64
        let s_5_2: i64 = fn_state.esize;
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_2 -> i
        let s_5_4: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #0u : u8
        let s_5_6: bool = false;
        // D s_5_7: read-var ga#365803:i
        let s_5_7: i128 = fn_state.ga_365803;
        // D s_5_8: read-var maximum:u8
        let s_5_8: bool = fn_state.maximum;
        // D s_5_9: read-var ga#365805:i
        let s_5_9: i128 = fn_state.ga_365805;
        // D s_5_10: call execute_aarch32_instrs_VMAXNM_Op_A_txt(s_5_6, s_5_3, s_5_7, s_5_5, s_5_1, s_5_8, s_5_0, s_5_9)
        let s_5_10: () = execute_aarch32_instrs_VMAXNM_Op_A_txt(
            state,
            tracer,
            s_5_6,
            s_5_3,
            s_5_7,
            s_5_5,
            s_5_1,
            s_5_8,
            s_5_0,
            s_5_9,
        );
        // N s_5_11: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #32s : i64
        let s_7_0: i64 = 32;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // D s_7_2: read-var Vd:u8
        let s_7_2: u8 = fn_state.Vd;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: read-var D:u8
        let s_7_4: bool = fn_state.D;
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
        // D s_7_18: write-var d <= s_7_17
        fn_state.d = s_7_17;
        // D s_7_19: read-var Vn:u8
        let s_7_19: u8 = fn_state.Vn;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 4u16);
        // D s_7_21: read-var N:u8
        let s_7_21: bool = fn_state.N;
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: cast reint s_7_22 -> u128
        let s_7_25: u128 = (s_7_22.value() as u128);
        // D s_7_26: size-of s_7_22
        let s_7_26: u16 = s_7_22.length();
        // D s_7_27: lsl s_7_23 s_7_26
        let s_7_27: u128 = s_7_23 << s_7_26;
        // D s_7_28: or s_7_27 s_7_25
        let s_7_28: u128 = ((s_7_27) | (s_7_25));
        // D s_7_29: add s_7_24 s_7_26
        let s_7_29: u16 = (s_7_24 + s_7_26);
        // D s_7_30: create-bits s_7_28 s_7_29
        let s_7_30: Bits = Bits::new(s_7_28, s_7_29);
        // D s_7_31: cast reint s_7_30 -> u8
        let s_7_31: u8 = (s_7_30.value() as u8);
        // D s_7_32: cast zx s_7_31 -> bv
        let s_7_32: Bits = Bits::new(s_7_31 as u128, 5u16);
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (s_7_32.value() as i128);
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: write-var n <= s_7_34
        fn_state.n = s_7_34;
        // D s_7_36: read-var Vm:u8
        let s_7_36: u8 = fn_state.Vm;
        // D s_7_37: cast zx s_7_36 -> bv
        let s_7_37: Bits = Bits::new(s_7_36 as u128, 4u16);
        // D s_7_38: read-var M:u8
        let s_7_38: bool = fn_state.M;
        // D s_7_39: cast zx s_7_38 -> bv
        let s_7_39: Bits = Bits::new(s_7_38 as u128, 1u16);
        // D s_7_40: cast reint s_7_37 -> u128
        let s_7_40: u128 = (s_7_37.value() as u128);
        // D s_7_41: size-of s_7_37
        let s_7_41: u16 = s_7_37.length();
        // D s_7_42: cast reint s_7_39 -> u128
        let s_7_42: u128 = (s_7_39.value() as u128);
        // D s_7_43: size-of s_7_39
        let s_7_43: u16 = s_7_39.length();
        // D s_7_44: lsl s_7_40 s_7_43
        let s_7_44: u128 = s_7_40 << s_7_43;
        // D s_7_45: or s_7_44 s_7_42
        let s_7_45: u128 = ((s_7_44) | (s_7_42));
        // D s_7_46: add s_7_41 s_7_43
        let s_7_46: u16 = (s_7_41 + s_7_43);
        // D s_7_47: create-bits s_7_45 s_7_46
        let s_7_47: Bits = Bits::new(s_7_45, s_7_46);
        // D s_7_48: cast reint s_7_47 -> u8
        let s_7_48: u8 = (s_7_47.value() as u8);
        // D s_7_49: cast zx s_7_48 -> bv
        let s_7_49: Bits = Bits::new(s_7_48 as u128, 5u16);
        // D s_7_50: cast zx s_7_49 -> i
        let s_7_50: i128 = (s_7_49.value() as i128);
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: write-var m <= s_7_51
        fn_state.m = s_7_51;
        // N s_7_53: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #3u : u8
        let s_8_2: u8 = 3;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
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
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // D s_9_2: read-var D:u8
        let s_9_2: bool = fn_state.D;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: read-var Vd:u8
        let s_9_4: u8 = fn_state.Vd;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 4u16);
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: lsl s_9_6 s_9_9
        let s_9_10: u128 = s_9_6 << s_9_9;
        // D s_9_11: or s_9_10 s_9_8
        let s_9_11: u128 = ((s_9_10) | (s_9_8));
        // D s_9_12: add s_9_7 s_9_9
        let s_9_12: u16 = (s_9_7 + s_9_9);
        // D s_9_13: create-bits s_9_11 s_9_12
        let s_9_13: Bits = Bits::new(s_9_11, s_9_12);
        // D s_9_14: cast reint s_9_13 -> u8
        let s_9_14: u8 = (s_9_13.value() as u8);
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 5u16);
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (s_9_15.value() as i128);
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: write-var d <= s_9_17
        fn_state.d = s_9_17;
        // D s_9_19: read-var N:u8
        let s_9_19: bool = fn_state.N;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: read-var Vn:u8
        let s_9_21: u8 = fn_state.Vn;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 4u16);
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: cast reint s_9_22 -> u128
        let s_9_25: u128 = (s_9_22.value() as u128);
        // D s_9_26: size-of s_9_22
        let s_9_26: u16 = s_9_22.length();
        // D s_9_27: lsl s_9_23 s_9_26
        let s_9_27: u128 = s_9_23 << s_9_26;
        // D s_9_28: or s_9_27 s_9_25
        let s_9_28: u128 = ((s_9_27) | (s_9_25));
        // D s_9_29: add s_9_24 s_9_26
        let s_9_29: u16 = (s_9_24 + s_9_26);
        // D s_9_30: create-bits s_9_28 s_9_29
        let s_9_30: Bits = Bits::new(s_9_28, s_9_29);
        // D s_9_31: cast reint s_9_30 -> u8
        let s_9_31: u8 = (s_9_30.value() as u8);
        // D s_9_32: cast zx s_9_31 -> bv
        let s_9_32: Bits = Bits::new(s_9_31 as u128, 5u16);
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (s_9_32.value() as i128);
        // D s_9_34: cast reint s_9_33 -> i64
        let s_9_34: i64 = (s_9_33 as i64);
        // D s_9_35: write-var n <= s_9_34
        fn_state.n = s_9_34;
        // D s_9_36: read-var M:u8
        let s_9_36: bool = fn_state.M;
        // D s_9_37: cast zx s_9_36 -> bv
        let s_9_37: Bits = Bits::new(s_9_36 as u128, 1u16);
        // D s_9_38: read-var Vm:u8
        let s_9_38: u8 = fn_state.Vm;
        // D s_9_39: cast zx s_9_38 -> bv
        let s_9_39: Bits = Bits::new(s_9_38 as u128, 4u16);
        // D s_9_40: cast reint s_9_37 -> u128
        let s_9_40: u128 = (s_9_37.value() as u128);
        // D s_9_41: size-of s_9_37
        let s_9_41: u16 = s_9_37.length();
        // D s_9_42: cast reint s_9_39 -> u128
        let s_9_42: u128 = (s_9_39.value() as u128);
        // D s_9_43: size-of s_9_39
        let s_9_43: u16 = s_9_39.length();
        // D s_9_44: lsl s_9_40 s_9_43
        let s_9_44: u128 = s_9_40 << s_9_43;
        // D s_9_45: or s_9_44 s_9_42
        let s_9_45: u128 = ((s_9_44) | (s_9_42));
        // D s_9_46: add s_9_41 s_9_43
        let s_9_46: u16 = (s_9_41 + s_9_43);
        // D s_9_47: create-bits s_9_45 s_9_46
        let s_9_47: Bits = Bits::new(s_9_45, s_9_46);
        // D s_9_48: cast reint s_9_47 -> u8
        let s_9_48: u8 = (s_9_47.value() as u8);
        // D s_9_49: cast zx s_9_48 -> bv
        let s_9_49: Bits = Bits::new(s_9_48 as u128, 5u16);
        // D s_9_50: cast zx s_9_49 -> i
        let s_9_50: i128 = (s_9_49.value() as i128);
        // D s_9_51: cast reint s_9_50 -> i64
        let s_9_51: i64 = (s_9_50 as i64);
        // D s_9_52: write-var m <= s_9_51
        fn_state.m = s_9_51;
        // N s_9_53: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#325293 <= s_12_0
        fn_state.gs_325293 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
