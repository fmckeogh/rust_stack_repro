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
pub fn decode_aarch32_instrs_VMAXNM_T1enc_A_txt<T: Tracer>(
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
        esizeshadow_7929: i64,
        gs_325329: bool,
        esize: i64,
        gs_325330: bool,
        n: i64,
        d: i64,
        elements: i64,
        gs_325326: bool,
        ga_365826: i64,
        elementsshadow_7930: i64,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b19 b1
        if s_0_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b12 b2
        if s_1_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#325330 <= s_2_0
        fn_state.gs_325330 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#325330:u8
        let s_3_0: bool = fn_state.gs_325330;
        // N s_3_1: branch s_3_0 b11 b4
        if s_3_0 {
            return block_11(state, tracer, fn_state);
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
        // C s_4_8: const #2s : i64
        let s_4_8: i64 = 2;
        // D s_4_9: write-var elements <= s_4_8
        fn_state.elements = s_4_8;
        // D s_4_10: read-var sz:u8
        let s_4_10: bool = fn_state.sz;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 1u16);
        // C s_4_12: const #0u : u8
        let s_4_12: bool = false;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 1u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // D s_4_15: not s_4_14
        let s_4_15: bool = !s_4_14;
        // N s_4_16: branch s_4_15 b10 b5
        if s_4_15 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // D s_5_1: write-var esize <= s_5_0
        fn_state.esize = s_5_0;
        // C s_5_2: const #2s : i64
        let s_5_2: i64 = 2;
        // D s_5_3: write-var elements <= s_5_2
        fn_state.elements = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // D s_6_1: write-var esizeshadow#7929 <= s_6_0
        fn_state.esizeshadow_7929 = s_6_0;
        // D s_6_2: read-var elements:i64
        let s_6_2: i64 = fn_state.elements;
        // D s_6_3: write-var elementsshadow#7930 <= s_6_2
        fn_state.elementsshadow_7930 = s_6_2;
        // D s_6_4: read-var D:u8
        let s_6_4: bool = fn_state.D;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: read-var Vd:u8
        let s_6_6: u8 = fn_state.Vd;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 4u16);
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: lsl s_6_8 s_6_11
        let s_6_12: u128 = s_6_8 << s_6_11;
        // D s_6_13: or s_6_12 s_6_10
        let s_6_13: u128 = ((s_6_12) | (s_6_10));
        // D s_6_14: add s_6_9 s_6_11
        let s_6_14: u16 = (s_6_9 + s_6_11);
        // D s_6_15: create-bits s_6_13 s_6_14
        let s_6_15: Bits = Bits::new(s_6_13, s_6_14);
        // D s_6_16: cast reint s_6_15 -> u8
        let s_6_16: u8 = (s_6_15.value() as u8);
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 5u16);
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (s_6_17.value() as i128);
        // D s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // D s_6_20: write-var d <= s_6_19
        fn_state.d = s_6_19;
        // D s_6_21: read-var N:u8
        let s_6_21: bool = fn_state.N;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 1u16);
        // D s_6_23: read-var Vn:u8
        let s_6_23: u8 = fn_state.Vn;
        // D s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 4u16);
        // D s_6_25: cast reint s_6_22 -> u128
        let s_6_25: u128 = (s_6_22.value() as u128);
        // D s_6_26: size-of s_6_22
        let s_6_26: u16 = s_6_22.length();
        // D s_6_27: cast reint s_6_24 -> u128
        let s_6_27: u128 = (s_6_24.value() as u128);
        // D s_6_28: size-of s_6_24
        let s_6_28: u16 = s_6_24.length();
        // D s_6_29: lsl s_6_25 s_6_28
        let s_6_29: u128 = s_6_25 << s_6_28;
        // D s_6_30: or s_6_29 s_6_27
        let s_6_30: u128 = ((s_6_29) | (s_6_27));
        // D s_6_31: add s_6_26 s_6_28
        let s_6_31: u16 = (s_6_26 + s_6_28);
        // D s_6_32: create-bits s_6_30 s_6_31
        let s_6_32: Bits = Bits::new(s_6_30, s_6_31);
        // D s_6_33: cast reint s_6_32 -> u8
        let s_6_33: u8 = (s_6_32.value() as u8);
        // D s_6_34: cast zx s_6_33 -> bv
        let s_6_34: Bits = Bits::new(s_6_33 as u128, 5u16);
        // D s_6_35: cast zx s_6_34 -> i
        let s_6_35: i128 = (s_6_34.value() as i128);
        // D s_6_36: cast reint s_6_35 -> i64
        let s_6_36: i64 = (s_6_35 as i64);
        // D s_6_37: write-var n <= s_6_36
        fn_state.n = s_6_36;
        // D s_6_38: read-var M:u8
        let s_6_38: bool = fn_state.M;
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 1u16);
        // D s_6_40: read-var Vm:u8
        let s_6_40: u8 = fn_state.Vm;
        // D s_6_41: cast zx s_6_40 -> bv
        let s_6_41: Bits = Bits::new(s_6_40 as u128, 4u16);
        // D s_6_42: cast reint s_6_39 -> u128
        let s_6_42: u128 = (s_6_39.value() as u128);
        // D s_6_43: size-of s_6_39
        let s_6_43: u16 = s_6_39.length();
        // D s_6_44: cast reint s_6_41 -> u128
        let s_6_44: u128 = (s_6_41.value() as u128);
        // D s_6_45: size-of s_6_41
        let s_6_45: u16 = s_6_41.length();
        // D s_6_46: lsl s_6_42 s_6_45
        let s_6_46: u128 = s_6_42 << s_6_45;
        // D s_6_47: or s_6_46 s_6_44
        let s_6_47: u128 = ((s_6_46) | (s_6_44));
        // D s_6_48: add s_6_43 s_6_45
        let s_6_48: u16 = (s_6_43 + s_6_45);
        // D s_6_49: create-bits s_6_47 s_6_48
        let s_6_49: Bits = Bits::new(s_6_47, s_6_48);
        // D s_6_50: cast reint s_6_49 -> u8
        let s_6_50: u8 = (s_6_49.value() as u8);
        // D s_6_51: cast zx s_6_50 -> bv
        let s_6_51: Bits = Bits::new(s_6_50 as u128, 5u16);
        // D s_6_52: cast zx s_6_51 -> i
        let s_6_52: i128 = (s_6_51.value() as i128);
        // D s_6_53: cast reint s_6_52 -> i64
        let s_6_53: i64 = (s_6_52 as i64);
        // D s_6_54: write-var m <= s_6_53
        fn_state.m = s_6_53;
        // D s_6_55: read-var Q:u8
        let s_6_55: bool = fn_state.Q;
        // D s_6_56: cast zx s_6_55 -> bv
        let s_6_56: Bits = Bits::new(s_6_55 as u128, 1u16);
        // C s_6_57: const #0u : u8
        let s_6_57: bool = false;
        // C s_6_58: cast zx s_6_57 -> bv
        let s_6_58: Bits = Bits::new(s_6_57 as u128, 1u16);
        // D s_6_59: cmp-eq s_6_56 s_6_58
        let s_6_59: bool = ((s_6_56) == (s_6_58));
        // N s_6_60: branch s_6_59 b9 b7
        if s_6_59 {
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
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#365826 <= s_7_0
        fn_state.ga_365826 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#365826:i64
        let s_8_0: i64 = fn_state.ga_365826;
        // D s_8_1: read-var esizeshadow#7929:i64
        let s_8_1: i64 = fn_state.esizeshadow_7929;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var elementsshadow#7930:i64
        let s_8_4: i64 = fn_state.elementsshadow_7930;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast zx s_8_0 -> i
        let s_8_6: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_7: const #1u : u8
        let s_8_7: bool = true;
        // D s_8_8: read-var d:i64
        let s_8_8: i64 = fn_state.d;
        // D s_8_9: read-var m:i64
        let s_8_9: i64 = fn_state.m;
        // D s_8_10: read-var maximum:u8
        let s_8_10: bool = fn_state.maximum;
        // D s_8_11: read-var n:i64
        let s_8_11: i64 = fn_state.n;
        // D s_8_12: call execute_aarch32_instrs_VMAXNM_Op_A_txt(s_8_7, s_8_8, s_8_5, s_8_3, s_8_9, s_8_10, s_8_11, s_8_6)
        let s_8_12: () = execute_aarch32_instrs_VMAXNM_Op_A_txt(
            state,
            tracer,
            s_8_7,
            s_8_8,
            s_8_5,
            s_8_3,
            s_8_9,
            s_8_10,
            s_8_11,
            s_8_6,
        );
        // N s_8_13: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: write-var ga#365826 <= s_9_0
        fn_state.ga_365826 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16s : i64
        let s_10_0: i64 = 16;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // C s_10_2: const #4s : i64
        let s_10_2: i64 = 4;
        // D s_10_3: write-var elements <= s_10_2
        fn_state.elements = s_10_2;
        // N s_10_4: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vd:u8
        let s_12_1: u8 = fn_state.Vd;
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
        // N s_12_22: branch s_12_21 b18 b13
        if s_12_21 {
            return block_18(state, tracer, fn_state);
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
        // D s_13_1: read-var Vn:u8
        let s_13_1: u8 = fn_state.Vn;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#325326 <= s_13_21
        fn_state.gs_325326 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#325326:u8
        let s_14_0: bool = fn_state.gs_325326;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vm:u8
        let s_15_1: u8 = fn_state.Vm;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // D s_15_22: write-var gs#325329 <= s_15_21
        fn_state.gs_325329 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#325329:u8
        let s_16_0: bool = fn_state.gs_325329;
        // D s_16_1: write-var gs#325330 <= s_16_0
        fn_state.gs_325330 = s_16_0;
        // N s_16_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#325329 <= s_17_0
        fn_state.gs_325329 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#325326 <= s_18_0
        fn_state.gs_325326 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}
