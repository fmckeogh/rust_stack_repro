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
pub fn decode_aarch32_instrs_VMAXNM_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    op: bool,
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
        m: i64,
        gs_325260: bool,
        esize: i64,
        n: i64,
        d: i64,
        elementsshadow_7924: i64,
        elements: i64,
        gs_325264: bool,
        gs_325263: bool,
        esizeshadow_7923: i64,
        ga_365789: i64,
        maximum: bool,
        D: bool,
        op: bool,
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
        op,
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
        // D s_0_0: read-var Q:u8
        let s_0_0: bool = fn_state.Q;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b11 b1
        if s_0_4 {
            return block_11(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#325264 <= s_1_0
        fn_state.gs_325264 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#325264:u8
        let s_2_0: bool = fn_state.gs_325264;
        // N s_2_1: branch s_2_0 b10 b3
        if s_2_0 {
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
        // C s_3_8: const #2s : i64
        let s_3_8: i64 = 2;
        // D s_3_9: write-var elements <= s_3_8
        fn_state.elements = s_3_8;
        // D s_3_10: read-var sz:u8
        let s_3_10: bool = fn_state.sz;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 1u16);
        // C s_3_12: const #0u : u8
        let s_3_12: bool = false;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 1u16);
        // D s_3_14: cmp-eq s_3_11 s_3_13
        let s_3_14: bool = ((s_3_11) == (s_3_13));
        // D s_3_15: not s_3_14
        let s_3_15: bool = !s_3_14;
        // N s_3_16: branch s_3_15 b9 b4
        if s_3_15 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_1: write-var esizeshadow#7923 <= s_5_0
        fn_state.esizeshadow_7923 = s_5_0;
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: write-var elementsshadow#7924 <= s_5_2
        fn_state.elementsshadow_7924 = s_5_2;
        // D s_5_4: read-var D:u8
        let s_5_4: bool = fn_state.D;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: read-var Vd:u8
        let s_5_6: u8 = fn_state.Vd;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 4u16);
        // D s_5_8: cast reint s_5_5 -> u128
        let s_5_8: u128 = (s_5_5.value() as u128);
        // D s_5_9: size-of s_5_5
        let s_5_9: u16 = s_5_5.length();
        // D s_5_10: cast reint s_5_7 -> u128
        let s_5_10: u128 = (s_5_7.value() as u128);
        // D s_5_11: size-of s_5_7
        let s_5_11: u16 = s_5_7.length();
        // D s_5_12: lsl s_5_8 s_5_11
        let s_5_12: u128 = s_5_8 << s_5_11;
        // D s_5_13: or s_5_12 s_5_10
        let s_5_13: u128 = ((s_5_12) | (s_5_10));
        // D s_5_14: add s_5_9 s_5_11
        let s_5_14: u16 = (s_5_9 + s_5_11);
        // D s_5_15: create-bits s_5_13 s_5_14
        let s_5_15: Bits = Bits::new(s_5_13, s_5_14);
        // D s_5_16: cast reint s_5_15 -> u8
        let s_5_16: u8 = (s_5_15.value() as u8);
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 5u16);
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (s_5_17.value() as i128);
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: write-var d <= s_5_19
        fn_state.d = s_5_19;
        // D s_5_21: read-var N:u8
        let s_5_21: bool = fn_state.N;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_23: read-var Vn:u8
        let s_5_23: u8 = fn_state.Vn;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 4u16);
        // D s_5_25: cast reint s_5_22 -> u128
        let s_5_25: u128 = (s_5_22.value() as u128);
        // D s_5_26: size-of s_5_22
        let s_5_26: u16 = s_5_22.length();
        // D s_5_27: cast reint s_5_24 -> u128
        let s_5_27: u128 = (s_5_24.value() as u128);
        // D s_5_28: size-of s_5_24
        let s_5_28: u16 = s_5_24.length();
        // D s_5_29: lsl s_5_25 s_5_28
        let s_5_29: u128 = s_5_25 << s_5_28;
        // D s_5_30: or s_5_29 s_5_27
        let s_5_30: u128 = ((s_5_29) | (s_5_27));
        // D s_5_31: add s_5_26 s_5_28
        let s_5_31: u16 = (s_5_26 + s_5_28);
        // D s_5_32: create-bits s_5_30 s_5_31
        let s_5_32: Bits = Bits::new(s_5_30, s_5_31);
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // D s_5_34: cast zx s_5_33 -> bv
        let s_5_34: Bits = Bits::new(s_5_33 as u128, 5u16);
        // D s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (s_5_34.value() as i128);
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // D s_5_37: write-var n <= s_5_36
        fn_state.n = s_5_36;
        // D s_5_38: read-var M:u8
        let s_5_38: bool = fn_state.M;
        // D s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 1u16);
        // D s_5_40: read-var Vm:u8
        let s_5_40: u8 = fn_state.Vm;
        // D s_5_41: cast zx s_5_40 -> bv
        let s_5_41: Bits = Bits::new(s_5_40 as u128, 4u16);
        // D s_5_42: cast reint s_5_39 -> u128
        let s_5_42: u128 = (s_5_39.value() as u128);
        // D s_5_43: size-of s_5_39
        let s_5_43: u16 = s_5_39.length();
        // D s_5_44: cast reint s_5_41 -> u128
        let s_5_44: u128 = (s_5_41.value() as u128);
        // D s_5_45: size-of s_5_41
        let s_5_45: u16 = s_5_41.length();
        // D s_5_46: lsl s_5_42 s_5_45
        let s_5_46: u128 = s_5_42 << s_5_45;
        // D s_5_47: or s_5_46 s_5_44
        let s_5_47: u128 = ((s_5_46) | (s_5_44));
        // D s_5_48: add s_5_43 s_5_45
        let s_5_48: u16 = (s_5_43 + s_5_45);
        // D s_5_49: create-bits s_5_47 s_5_48
        let s_5_49: Bits = Bits::new(s_5_47, s_5_48);
        // D s_5_50: cast reint s_5_49 -> u8
        let s_5_50: u8 = (s_5_49.value() as u8);
        // D s_5_51: cast zx s_5_50 -> bv
        let s_5_51: Bits = Bits::new(s_5_50 as u128, 5u16);
        // D s_5_52: cast zx s_5_51 -> i
        let s_5_52: i128 = (s_5_51.value() as i128);
        // D s_5_53: cast reint s_5_52 -> i64
        let s_5_53: i64 = (s_5_52 as i64);
        // D s_5_54: write-var m <= s_5_53
        fn_state.m = s_5_53;
        // D s_5_55: read-var Q:u8
        let s_5_55: bool = fn_state.Q;
        // D s_5_56: cast zx s_5_55 -> bv
        let s_5_56: Bits = Bits::new(s_5_55 as u128, 1u16);
        // C s_5_57: const #0u : u8
        let s_5_57: bool = false;
        // C s_5_58: cast zx s_5_57 -> bv
        let s_5_58: Bits = Bits::new(s_5_57 as u128, 1u16);
        // D s_5_59: cmp-eq s_5_56 s_5_58
        let s_5_59: bool = ((s_5_56) == (s_5_58));
        // N s_5_60: branch s_5_59 b8 b6
        if s_5_59 {
            return block_8(state, tracer, fn_state);
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
        // D s_6_1: write-var ga#365789 <= s_6_0
        fn_state.ga_365789 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#365789:i64
        let s_7_0: i64 = fn_state.ga_365789;
        // D s_7_1: read-var esizeshadow#7923:i64
        let s_7_1: i64 = fn_state.esizeshadow_7923;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var elementsshadow#7924:i64
        let s_7_4: i64 = fn_state.elementsshadow_7924;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast zx s_7_0 -> i
        let s_7_6: i128 = (i128::try_from(s_7_0).unwrap());
        // C s_7_7: const #1u : u8
        let s_7_7: bool = true;
        // D s_7_8: read-var d:i64
        let s_7_8: i64 = fn_state.d;
        // D s_7_9: read-var m:i64
        let s_7_9: i64 = fn_state.m;
        // D s_7_10: read-var maximum:u8
        let s_7_10: bool = fn_state.maximum;
        // D s_7_11: read-var n:i64
        let s_7_11: i64 = fn_state.n;
        // D s_7_12: call execute_aarch32_instrs_VMAXNM_Op_A_txt(s_7_7, s_7_8, s_7_5, s_7_3, s_7_9, s_7_10, s_7_11, s_7_6)
        let s_7_12: () = execute_aarch32_instrs_VMAXNM_Op_A_txt(
            state,
            tracer,
            s_7_7,
            s_7_8,
            s_7_5,
            s_7_3,
            s_7_9,
            s_7_10,
            s_7_11,
            s_7_6,
        );
        // N s_7_13: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i64
        let s_8_0: i64 = 1;
        // D s_8_1: write-var ga#365789 <= s_8_0
        fn_state.ga_365789 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16s : i64
        let s_9_0: i64 = 16;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // C s_9_2: const #4s : i64
        let s_9_2: i64 = 4;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vd:u8
        let s_11_1: u8 = fn_state.Vd;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 4u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // N s_11_22: branch s_11_21 b17 b12
        if s_11_21 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vn:u8
        let s_12_1: u8 = fn_state.Vn;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // D s_12_22: write-var gs#325260 <= s_12_21
        fn_state.gs_325260 = s_12_21;
        // N s_12_23: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#325260:u8
        let s_13_0: bool = fn_state.gs_325260;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var Vm:u8
        let s_14_1: u8 = fn_state.Vm;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 4u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#325263 <= s_14_21
        fn_state.gs_325263 = s_14_21;
        // N s_14_23: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#325263:u8
        let s_15_0: bool = fn_state.gs_325263;
        // D s_15_1: write-var gs#325264 <= s_15_0
        fn_state.gs_325264 = s_15_0;
        // N s_15_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#325263 <= s_16_0
        fn_state.gs_325263 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#325260 <= s_17_0
        fn_state.gs_325260 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
}
