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
use execute_aarch32_instrs_VMOV_sr_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_sr_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    U: bool,
    opc1: u8,
    Vn: u8,
    Rt: u8,
    N: bool,
    opc2: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_356538: u8,
        t: i64,
        advsimd: bool,
        esize: i64,
        gs_313371: bool,
        n: i64,
        index: i128,
        b__1: u8,
        indexshadow_7566: i128,
        b__2: u8,
        gs_313395: bool,
        gs_313384: bool,
        is_unsigned: bool,
        advsimdshadow_7565: bool,
        b__3: u8,
        esizeshadow_7567: i64,
        cond: u8,
        U: bool,
        opc1: u8,
        Vn: u8,
        Rt: u8,
        N: bool,
        opc2: u8,
    }
    let fn_state = FunctionState {
        cond,
        U,
        opc1,
        Vn,
        Rt,
        N,
        opc2,
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
        // C s_2_6: const #8s : i64
        let s_2_6: i64 = 8;
        // D s_2_7: write-var esize <= s_2_6
        fn_state.esize = s_2_6;
        // D s_2_8: read-var U:u8
        let s_2_8: bool = fn_state.U;
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: read-var opc1:u8
        let s_2_10: u8 = fn_state.opc1;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 2u16);
        // D s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // D s_2_16: lsl s_2_12 s_2_15
        let s_2_16: u128 = s_2_12 << s_2_15;
        // D s_2_17: or s_2_16 s_2_14
        let s_2_17: u128 = ((s_2_16) | (s_2_14));
        // D s_2_18: add s_2_13 s_2_15
        let s_2_18: u16 = (s_2_13 + s_2_15);
        // D s_2_19: create-bits s_2_17 s_2_18
        let s_2_19: Bits = Bits::new(s_2_17, s_2_18);
        // D s_2_20: cast reint s_2_19 -> u8
        let s_2_20: u8 = (s_2_19.value() as u8);
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 3u16);
        // D s_2_22: read-var opc2:u8
        let s_2_22: u8 = fn_state.opc2;
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 2u16);
        // D s_2_24: cast reint s_2_21 -> u128
        let s_2_24: u128 = (s_2_21.value() as u128);
        // D s_2_25: size-of s_2_21
        let s_2_25: u16 = s_2_21.length();
        // D s_2_26: cast reint s_2_23 -> u128
        let s_2_26: u128 = (s_2_23.value() as u128);
        // D s_2_27: size-of s_2_23
        let s_2_27: u16 = s_2_23.length();
        // D s_2_28: lsl s_2_24 s_2_27
        let s_2_28: u128 = s_2_24 << s_2_27;
        // D s_2_29: or s_2_28 s_2_26
        let s_2_29: u128 = ((s_2_28) | (s_2_26));
        // D s_2_30: add s_2_25 s_2_27
        let s_2_30: u16 = (s_2_25 + s_2_27);
        // D s_2_31: create-bits s_2_29 s_2_30
        let s_2_31: Bits = Bits::new(s_2_29, s_2_30);
        // D s_2_32: cast reint s_2_31 -> u8
        let s_2_32: u8 = (s_2_31.value() as u8);
        // D s_2_33: write-var ga#356538 <= s_2_32
        fn_state.ga_356538 = s_2_32;
        // D s_2_34: read-var ga#356538:u8
        let s_2_34: u8 = fn_state.ga_356538;
        // C s_2_35: const #3s : i
        let s_2_35: i128 = 3;
        // D s_2_36: cast zx s_2_34 -> bv
        let s_2_36: Bits = Bits::new(s_2_34 as u128, 5u16);
        // C s_2_37: const #1s : i64
        let s_2_37: i64 = 1;
        // C s_2_38: cast zx s_2_37 -> i
        let s_2_38: i128 = (i128::try_from(s_2_37).unwrap());
        // C s_2_39: const #0s : i
        let s_2_39: i128 = 0;
        // C s_2_40: add s_2_39 s_2_38
        let s_2_40: i128 = (s_2_39 + s_2_38);
        // D s_2_41: bit-extract s_2_36 s_2_35 s_2_40
        let s_2_41: Bits = (Bits::new(
            ((s_2_36) >> (s_2_35)).value(),
            u16::try_from(s_2_40).unwrap(),
        ));
        // D s_2_42: cast reint s_2_41 -> u8
        let s_2_42: bool = ((s_2_41.value()) != 0);
        // D s_2_43: cast zx s_2_42 -> bv
        let s_2_43: Bits = Bits::new(s_2_42 as u128, 1u16);
        // C s_2_44: const #1u : u8
        let s_2_44: bool = true;
        // C s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 1u16);
        // D s_2_46: cmp-eq s_2_43 s_2_45
        let s_2_46: bool = ((s_2_43) == (s_2_45));
        // D s_2_47: not s_2_46
        let s_2_47: bool = !s_2_46;
        // N s_2_48: branch s_2_47 b13 b3
        if s_2_47 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var advsimd <= s_3_0
        fn_state.advsimd = s_3_0;
        // C s_3_2: const #8s : i64
        let s_3_2: i64 = 8;
        // D s_3_3: write-var esize <= s_3_2
        fn_state.esize = s_3_2;
        // C s_3_4: const #0s : i
        let s_3_4: i128 = 0;
        // D s_3_5: read-var opc1:u8
        let s_3_5: u8 = fn_state.opc1;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 2u16);
        // C s_3_7: const #1u : u64
        let s_3_7: u64 = 1;
        // D s_3_8: bit-extract s_3_6 s_3_4 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_6) >> (s_3_4)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: bool = ((s_3_8.value()) != 0);
        // C s_3_10: const #0s : i
        let s_3_10: i128 = 0;
        // C s_3_11: const #0u : u64
        let s_3_11: u64 = 0;
        // D s_3_12: cast zx s_3_9 -> u64
        let s_3_12: u64 = (s_3_9 as u64);
        // C s_3_13: const #1u : u64
        let s_3_13: u64 = 1;
        // D s_3_14: and s_3_12 s_3_13
        let s_3_14: u64 = ((s_3_12) & (s_3_13));
        // D s_3_15: cmp-eq s_3_14 s_3_13
        let s_3_15: bool = ((s_3_14) == (s_3_13));
        // D s_3_16: lsl s_3_12 s_3_10
        let s_3_16: u64 = s_3_12 << s_3_10;
        // D s_3_17: or s_3_11 s_3_16
        let s_3_17: u64 = ((s_3_11) | (s_3_16));
        // D s_3_18: cmpl s_3_16
        let s_3_18: u64 = !s_3_16;
        // D s_3_19: and s_3_11 s_3_18
        let s_3_19: u64 = ((s_3_11) & (s_3_18));
        // D s_3_20: select s_3_15 s_3_17 s_3_19
        let s_3_20: u64 = if s_3_15 { s_3_17 } else { s_3_19 };
        // D s_3_21: cast trunc s_3_20 -> u8
        let s_3_21: bool = ((s_3_20) != 0);
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // D s_3_23: read-var opc2:u8
        let s_3_23: u8 = fn_state.opc2;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 2u16);
        // D s_3_25: cast reint s_3_22 -> u128
        let s_3_25: u128 = (s_3_22.value() as u128);
        // D s_3_26: size-of s_3_22
        let s_3_26: u16 = s_3_22.length();
        // D s_3_27: cast reint s_3_24 -> u128
        let s_3_27: u128 = (s_3_24.value() as u128);
        // D s_3_28: size-of s_3_24
        let s_3_28: u16 = s_3_24.length();
        // D s_3_29: lsl s_3_25 s_3_28
        let s_3_29: u128 = s_3_25 << s_3_28;
        // D s_3_30: or s_3_29 s_3_27
        let s_3_30: u128 = ((s_3_29) | (s_3_27));
        // D s_3_31: add s_3_26 s_3_28
        let s_3_31: u16 = (s_3_26 + s_3_28);
        // D s_3_32: create-bits s_3_30 s_3_31
        let s_3_32: Bits = Bits::new(s_3_30, s_3_31);
        // D s_3_33: cast reint s_3_32 -> u8
        let s_3_33: u8 = (s_3_32.value() as u8);
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 3u16);
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (s_3_34.value() as i128);
        // D s_3_36: write-var index <= s_3_35
        fn_state.index = s_3_35;
        // N s_3_37: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var advsimd:u8
        let s_4_0: bool = fn_state.advsimd;
        // D s_4_1: write-var advsimdshadow#7565 <= s_4_0
        fn_state.advsimdshadow_7565 = s_4_0;
        // D s_4_2: read-var index:i
        let s_4_2: i128 = fn_state.index;
        // D s_4_3: write-var indexshadow#7566 <= s_4_2
        fn_state.indexshadow_7566 = s_4_2;
        // D s_4_4: read-var esize:i64
        let s_4_4: i64 = fn_state.esize;
        // D s_4_5: write-var esizeshadow#7567 <= s_4_4
        fn_state.esizeshadow_7567 = s_4_4;
        // D s_4_6: read-var Rt:u8
        let s_4_6: u8 = fn_state.Rt;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 4u16);
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: write-var t <= s_4_9
        fn_state.t = s_4_9;
        // D s_4_11: read-var N:u8
        let s_4_11: bool = fn_state.N;
        // D s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 1u16);
        // D s_4_13: read-var Vn:u8
        let s_4_13: u8 = fn_state.Vn;
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 4u16);
        // D s_4_15: cast reint s_4_12 -> u128
        let s_4_15: u128 = (s_4_12.value() as u128);
        // D s_4_16: size-of s_4_12
        let s_4_16: u16 = s_4_12.length();
        // D s_4_17: cast reint s_4_14 -> u128
        let s_4_17: u128 = (s_4_14.value() as u128);
        // D s_4_18: size-of s_4_14
        let s_4_18: u16 = s_4_14.length();
        // D s_4_19: lsl s_4_15 s_4_18
        let s_4_19: u128 = s_4_15 << s_4_18;
        // D s_4_20: or s_4_19 s_4_17
        let s_4_20: u128 = ((s_4_19) | (s_4_17));
        // D s_4_21: add s_4_16 s_4_18
        let s_4_21: u16 = (s_4_16 + s_4_18);
        // D s_4_22: create-bits s_4_20 s_4_21
        let s_4_22: Bits = Bits::new(s_4_20, s_4_21);
        // D s_4_23: cast reint s_4_22 -> u8
        let s_4_23: u8 = (s_4_22.value() as u8);
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 5u16);
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (s_4_24.value() as i128);
        // D s_4_26: cast reint s_4_25 -> i64
        let s_4_26: i64 = (s_4_25 as i64);
        // D s_4_27: write-var n <= s_4_26
        fn_state.n = s_4_26;
        // D s_4_28: read-var U:u8
        let s_4_28: bool = fn_state.U;
        // D s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 1u16);
        // C s_4_30: const #1u : u8
        let s_4_30: bool = true;
        // C s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 1u16);
        // D s_4_32: cmp-eq s_4_29 s_4_31
        let s_4_32: bool = ((s_4_29) == (s_4_31));
        // D s_4_33: write-var is_unsigned <= s_4_32
        fn_state.is_unsigned = s_4_32;
        // C s_4_34: const #15s : i
        let s_4_34: i128 = 15;
        // D s_4_35: read-var t:i64
        let s_4_35: i64 = fn_state.t;
        // D s_4_36: cast zx s_4_35 -> i
        let s_4_36: i128 = (i128::try_from(s_4_35).unwrap());
        // D s_4_37: cmp-eq s_4_36 s_4_34
        let s_4_37: bool = ((s_4_36) == (s_4_34));
        // N s_4_38: branch s_4_37 b12 b5
        if s_4_37 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#7567:i64
        let s_5_0: i64 = fn_state.esizeshadow_7567;
        // C s_5_1: const #8s : i
        let s_5_1: i128 = 8;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i64
        let s_6_0: i64 = 8;
        // D s_6_1: read-var advsimdshadow#7565:u8
        let s_6_1: bool = fn_state.advsimdshadow_7565;
        // D s_6_2: read-var indexshadow#7566:i
        let s_6_2: i128 = fn_state.indexshadow_7566;
        // D s_6_3: read-var n:i64
        let s_6_3: i64 = fn_state.n;
        // D s_6_4: read-var t:i64
        let s_6_4: i64 = fn_state.t;
        // D s_6_5: read-var is_unsigned:u8
        let s_6_5: bool = fn_state.is_unsigned;
        // D s_6_6: call execute_aarch32_instrs_VMOV_sr_Op_A_txt(s_6_1, s_6_0, s_6_2, s_6_3, s_6_4, s_6_5)
        let s_6_6: () = execute_aarch32_instrs_VMOV_sr_Op_A_txt(
            state,
            tracer,
            s_6_1,
            s_6_0,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
        );
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#7567:i64
        let s_7_0: i64 = fn_state.esizeshadow_7567;
        // C s_7_1: const #16s : i
        let s_7_1: i128 = 16;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
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
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // D s_8_1: read-var advsimdshadow#7565:u8
        let s_8_1: bool = fn_state.advsimdshadow_7565;
        // D s_8_2: read-var indexshadow#7566:i
        let s_8_2: i128 = fn_state.indexshadow_7566;
        // D s_8_3: read-var n:i64
        let s_8_3: i64 = fn_state.n;
        // D s_8_4: read-var t:i64
        let s_8_4: i64 = fn_state.t;
        // D s_8_5: read-var is_unsigned:u8
        let s_8_5: bool = fn_state.is_unsigned;
        // D s_8_6: call execute_aarch32_instrs_VMOV_sr_Op_A_txt(s_8_1, s_8_0, s_8_2, s_8_3, s_8_4, s_8_5)
        let s_8_6: () = execute_aarch32_instrs_VMOV_sr_Op_A_txt(
            state,
            tracer,
            s_8_1,
            s_8_0,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
        );
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#7567:i64
        let s_9_0: i64 = fn_state.esizeshadow_7567;
        // C s_9_1: const #32s : i
        let s_9_1: i128 = 32;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
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
        // D s_10_1: read-var advsimdshadow#7565:u8
        let s_10_1: bool = fn_state.advsimdshadow_7565;
        // D s_10_2: read-var indexshadow#7566:i
        let s_10_2: i128 = fn_state.indexshadow_7566;
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: read-var t:i64
        let s_10_4: i64 = fn_state.t;
        // D s_10_5: read-var is_unsigned:u8
        let s_10_5: bool = fn_state.is_unsigned;
        // D s_10_6: call execute_aarch32_instrs_VMOV_sr_Op_A_txt(s_10_1, s_10_0, s_10_2, s_10_3, s_10_4, s_10_5)
        let s_10_6: () = execute_aarch32_instrs_VMOV_sr_Op_A_txt(
            state,
            tracer,
            s_10_1,
            s_10_0,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
        );
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // N s_11_2: return
        return;
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
        // D s_13_0: read-var ga#356538:u8
        let s_13_0: u8 = fn_state.ga_356538;
        // D s_13_1: write-var b__1 <= s_13_0
        fn_state.b__1 = s_13_0;
        // C s_13_2: const #3s : i
        let s_13_2: i128 = 3;
        // D s_13_3: read-var b__1:u8
        let s_13_3: u8 = fn_state.b__1;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 5u16);
        // C s_13_5: const #1s : i64
        let s_13_5: i64 = 1;
        // C s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // C s_13_7: const #0s : i
        let s_13_7: i128 = 0;
        // C s_13_8: add s_13_7 s_13_6
        let s_13_8: i128 = (s_13_7 + s_13_6);
        // D s_13_9: bit-extract s_13_4 s_13_2 s_13_8
        let s_13_9: Bits = (Bits::new(
            ((s_13_4) >> (s_13_2)).value(),
            u16::try_from(s_13_8).unwrap(),
        ));
        // D s_13_10: cast reint s_13_9 -> u8
        let s_13_10: bool = ((s_13_9.value()) != 0);
        // D s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 1u16);
        // C s_13_12: const #0u : u8
        let s_13_12: bool = false;
        // C s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 1u16);
        // D s_13_14: cmp-eq s_13_11 s_13_13
        let s_13_14: bool = ((s_13_11) == (s_13_13));
        // N s_13_15: branch s_13_14 b28 b14
        if s_13_14 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#313371 <= s_14_0
        fn_state.gs_313371 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#313371:u8
        let s_15_0: bool = fn_state.gs_313371;
        // D s_15_1: not s_15_0
        let s_15_1: bool = !s_15_0;
        // N s_15_2: branch s_15_1 b17 b16
        if s_15_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var advsimd <= s_16_0
        fn_state.advsimd = s_16_0;
        // C s_16_2: const #16s : i64
        let s_16_2: i64 = 16;
        // D s_16_3: write-var esize <= s_16_2
        fn_state.esize = s_16_2;
        // C s_16_4: const #0s : i
        let s_16_4: i128 = 0;
        // D s_16_5: read-var opc1:u8
        let s_16_5: u8 = fn_state.opc1;
        // D s_16_6: cast zx s_16_5 -> bv
        let s_16_6: Bits = Bits::new(s_16_5 as u128, 2u16);
        // C s_16_7: const #1u : u64
        let s_16_7: u64 = 1;
        // D s_16_8: bit-extract s_16_6 s_16_4 s_16_7
        let s_16_8: Bits = (Bits::new(
            ((s_16_6) >> (s_16_4)).value(),
            u16::try_from(s_16_7).unwrap(),
        ));
        // D s_16_9: cast reint s_16_8 -> u8
        let s_16_9: bool = ((s_16_8.value()) != 0);
        // C s_16_10: const #0s : i
        let s_16_10: i128 = 0;
        // C s_16_11: const #0u : u64
        let s_16_11: u64 = 0;
        // D s_16_12: cast zx s_16_9 -> u64
        let s_16_12: u64 = (s_16_9 as u64);
        // C s_16_13: const #1u : u64
        let s_16_13: u64 = 1;
        // D s_16_14: and s_16_12 s_16_13
        let s_16_14: u64 = ((s_16_12) & (s_16_13));
        // D s_16_15: cmp-eq s_16_14 s_16_13
        let s_16_15: bool = ((s_16_14) == (s_16_13));
        // D s_16_16: lsl s_16_12 s_16_10
        let s_16_16: u64 = s_16_12 << s_16_10;
        // D s_16_17: or s_16_11 s_16_16
        let s_16_17: u64 = ((s_16_11) | (s_16_16));
        // D s_16_18: cmpl s_16_16
        let s_16_18: u64 = !s_16_16;
        // D s_16_19: and s_16_11 s_16_18
        let s_16_19: u64 = ((s_16_11) & (s_16_18));
        // D s_16_20: select s_16_15 s_16_17 s_16_19
        let s_16_20: u64 = if s_16_15 { s_16_17 } else { s_16_19 };
        // D s_16_21: cast trunc s_16_20 -> u8
        let s_16_21: bool = ((s_16_20) != 0);
        // C s_16_22: const #1s : i
        let s_16_22: i128 = 1;
        // D s_16_23: read-var opc2:u8
        let s_16_23: u8 = fn_state.opc2;
        // D s_16_24: cast zx s_16_23 -> bv
        let s_16_24: Bits = Bits::new(s_16_23 as u128, 2u16);
        // C s_16_25: const #1u : u64
        let s_16_25: u64 = 1;
        // D s_16_26: bit-extract s_16_24 s_16_22 s_16_25
        let s_16_26: Bits = (Bits::new(
            ((s_16_24) >> (s_16_22)).value(),
            u16::try_from(s_16_25).unwrap(),
        ));
        // D s_16_27: cast reint s_16_26 -> u8
        let s_16_27: bool = ((s_16_26.value()) != 0);
        // C s_16_28: const #0s : i
        let s_16_28: i128 = 0;
        // C s_16_29: const #0u : u64
        let s_16_29: u64 = 0;
        // D s_16_30: cast zx s_16_27 -> u64
        let s_16_30: u64 = (s_16_27 as u64);
        // C s_16_31: const #1u : u64
        let s_16_31: u64 = 1;
        // D s_16_32: and s_16_30 s_16_31
        let s_16_32: u64 = ((s_16_30) & (s_16_31));
        // D s_16_33: cmp-eq s_16_32 s_16_31
        let s_16_33: bool = ((s_16_32) == (s_16_31));
        // D s_16_34: lsl s_16_30 s_16_28
        let s_16_34: u64 = s_16_30 << s_16_28;
        // D s_16_35: or s_16_29 s_16_34
        let s_16_35: u64 = ((s_16_29) | (s_16_34));
        // D s_16_36: cmpl s_16_34
        let s_16_36: u64 = !s_16_34;
        // D s_16_37: and s_16_29 s_16_36
        let s_16_37: u64 = ((s_16_29) & (s_16_36));
        // D s_16_38: select s_16_33 s_16_35 s_16_37
        let s_16_38: u64 = if s_16_33 { s_16_35 } else { s_16_37 };
        // D s_16_39: cast trunc s_16_38 -> u8
        let s_16_39: bool = ((s_16_38) != 0);
        // D s_16_40: cast zx s_16_21 -> bv
        let s_16_40: Bits = Bits::new(s_16_21 as u128, 1u16);
        // D s_16_41: cast zx s_16_39 -> bv
        let s_16_41: Bits = Bits::new(s_16_39 as u128, 1u16);
        // D s_16_42: cast reint s_16_40 -> u128
        let s_16_42: u128 = (s_16_40.value() as u128);
        // D s_16_43: size-of s_16_40
        let s_16_43: u16 = s_16_40.length();
        // D s_16_44: cast reint s_16_41 -> u128
        let s_16_44: u128 = (s_16_41.value() as u128);
        // D s_16_45: size-of s_16_41
        let s_16_45: u16 = s_16_41.length();
        // D s_16_46: lsl s_16_42 s_16_45
        let s_16_46: u128 = s_16_42 << s_16_45;
        // D s_16_47: or s_16_46 s_16_44
        let s_16_47: u128 = ((s_16_46) | (s_16_44));
        // D s_16_48: add s_16_43 s_16_45
        let s_16_48: u16 = (s_16_43 + s_16_45);
        // D s_16_49: create-bits s_16_47 s_16_48
        let s_16_49: Bits = Bits::new(s_16_47, s_16_48);
        // D s_16_50: cast reint s_16_49 -> u8
        let s_16_50: u8 = (s_16_49.value() as u8);
        // D s_16_51: cast zx s_16_50 -> bv
        let s_16_51: Bits = Bits::new(s_16_50 as u128, 2u16);
        // D s_16_52: cast zx s_16_51 -> i
        let s_16_52: i128 = (s_16_51.value() as i128);
        // D s_16_53: write-var index <= s_16_52
        fn_state.index = s_16_52;
        // N s_16_54: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#356538:u8
        let s_17_0: u8 = fn_state.ga_356538;
        // D s_17_1: write-var b__2 <= s_17_0
        fn_state.b__2 = s_17_0;
        // C s_17_2: const #3s : i
        let s_17_2: i128 = 3;
        // D s_17_3: read-var b__2:u8
        let s_17_3: u8 = fn_state.b__2;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 5u16);
        // C s_17_5: const #1s : i64
        let s_17_5: i64 = 1;
        // C s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // C s_17_7: const #1s : i
        let s_17_7: i128 = 1;
        // C s_17_8: add s_17_7 s_17_6
        let s_17_8: i128 = (s_17_7 + s_17_6);
        // D s_17_9: bit-extract s_17_4 s_17_2 s_17_8
        let s_17_9: Bits = (Bits::new(
            ((s_17_4) >> (s_17_2)).value(),
            u16::try_from(s_17_8).unwrap(),
        ));
        // D s_17_10: cast reint s_17_9 -> u8
        let s_17_10: u8 = (s_17_9.value() as u8);
        // D s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 2u16);
        // C s_17_12: const #0u : u8
        let s_17_12: u8 = 0;
        // C s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 2u16);
        // D s_17_14: cmp-eq s_17_11 s_17_13
        let s_17_14: bool = ((s_17_11) == (s_17_13));
        // N s_17_15: branch s_17_14 b27 b18
        if s_17_14 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#313384 <= s_18_0
        fn_state.gs_313384 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#313384:u8
        let s_19_0: bool = fn_state.gs_313384;
        // D s_19_1: not s_19_0
        let s_19_1: bool = !s_19_0;
        // N s_19_2: branch s_19_1 b21 b20
        if s_19_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var advsimd <= s_20_0
        fn_state.advsimd = s_20_0;
        // C s_20_2: const #32s : i64
        let s_20_2: i64 = 32;
        // D s_20_3: write-var esize <= s_20_2
        fn_state.esize = s_20_2;
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: read-var opc1:u8
        let s_20_5: u8 = fn_state.opc1;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 2u16);
        // C s_20_7: const #1u : u64
        let s_20_7: u64 = 1;
        // D s_20_8: bit-extract s_20_6 s_20_4 s_20_7
        let s_20_8: Bits = (Bits::new(
            ((s_20_6) >> (s_20_4)).value(),
            u16::try_from(s_20_7).unwrap(),
        ));
        // D s_20_9: cast reint s_20_8 -> u8
        let s_20_9: bool = ((s_20_8.value()) != 0);
        // C s_20_10: const #0s : i
        let s_20_10: i128 = 0;
        // C s_20_11: const #0u : u64
        let s_20_11: u64 = 0;
        // D s_20_12: cast zx s_20_9 -> u64
        let s_20_12: u64 = (s_20_9 as u64);
        // C s_20_13: const #1u : u64
        let s_20_13: u64 = 1;
        // D s_20_14: and s_20_12 s_20_13
        let s_20_14: u64 = ((s_20_12) & (s_20_13));
        // D s_20_15: cmp-eq s_20_14 s_20_13
        let s_20_15: bool = ((s_20_14) == (s_20_13));
        // D s_20_16: lsl s_20_12 s_20_10
        let s_20_16: u64 = s_20_12 << s_20_10;
        // D s_20_17: or s_20_11 s_20_16
        let s_20_17: u64 = ((s_20_11) | (s_20_16));
        // D s_20_18: cmpl s_20_16
        let s_20_18: u64 = !s_20_16;
        // D s_20_19: and s_20_11 s_20_18
        let s_20_19: u64 = ((s_20_11) & (s_20_18));
        // D s_20_20: select s_20_15 s_20_17 s_20_19
        let s_20_20: u64 = if s_20_15 { s_20_17 } else { s_20_19 };
        // D s_20_21: cast trunc s_20_20 -> u8
        let s_20_21: bool = ((s_20_20) != 0);
        // D s_20_22: cast zx s_20_21 -> bv
        let s_20_22: Bits = Bits::new(s_20_21 as u128, 1u16);
        // D s_20_23: cast zx s_20_22 -> i
        let s_20_23: i128 = (s_20_22.value() as i128);
        // D s_20_24: write-var index <= s_20_23
        fn_state.index = s_20_23;
        // N s_20_25: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ga#356538:u8
        let s_21_0: u8 = fn_state.ga_356538;
        // D s_21_1: write-var b__3 <= s_21_0
        fn_state.b__3 = s_21_0;
        // C s_21_2: const #3s : i
        let s_21_2: i128 = 3;
        // D s_21_3: read-var b__3:u8
        let s_21_3: u8 = fn_state.b__3;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 5u16);
        // C s_21_5: const #1s : i64
        let s_21_5: i64 = 1;
        // C s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // C s_21_7: const #1s : i
        let s_21_7: i128 = 1;
        // C s_21_8: add s_21_7 s_21_6
        let s_21_8: i128 = (s_21_7 + s_21_6);
        // D s_21_9: bit-extract s_21_4 s_21_2 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_4) >> (s_21_2)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: u8 = (s_21_9.value() as u8);
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 2u16);
        // C s_21_12: const #2u : u8
        let s_21_12: u8 = 2;
        // C s_21_13: cast zx s_21_12 -> bv
        let s_21_13: Bits = Bits::new(s_21_12 as u128, 2u16);
        // D s_21_14: cmp-eq s_21_11 s_21_13
        let s_21_14: bool = ((s_21_11) == (s_21_13));
        // N s_21_15: branch s_21_14 b26 b22
        if s_21_14 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#313395 <= s_22_0
        fn_state.gs_313395 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#313395:u8
        let s_23_0: bool = fn_state.gs_313395;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // N s_23_2: branch s_23_1 b25 b24
        if s_23_1 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var b__3:u8
        let s_26_1: u8 = fn_state.b__3;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 5u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #1s : i
        let s_26_5: i128 = 1;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: u8 = (s_26_7.value() as u8);
        // D s_26_9: cast zx s_26_8 -> bv
        let s_26_9: Bits = Bits::new(s_26_8 as u128, 2u16);
        // C s_26_10: const #0u : u8
        let s_26_10: u8 = 0;
        // C s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 2u16);
        // D s_26_12: cmp-eq s_26_9 s_26_11
        let s_26_12: bool = ((s_26_9) == (s_26_11));
        // D s_26_13: write-var gs#313395 <= s_26_12
        fn_state.gs_313395 = s_26_12;
        // N s_26_14: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var b__2:u8
        let s_27_1: u8 = fn_state.b__2;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 5u16);
        // C s_27_3: const #1s : i64
        let s_27_3: i64 = 1;
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #1s : i
        let s_27_5: i128 = 1;
        // C s_27_6: add s_27_5 s_27_4
        let s_27_6: i128 = (s_27_5 + s_27_4);
        // D s_27_7: bit-extract s_27_2 s_27_0 s_27_6
        let s_27_7: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_6).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: u8 = (s_27_7.value() as u8);
        // D s_27_9: cast zx s_27_8 -> bv
        let s_27_9: Bits = Bits::new(s_27_8 as u128, 2u16);
        // C s_27_10: const #0u : u8
        let s_27_10: u8 = 0;
        // C s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 2u16);
        // D s_27_12: cmp-eq s_27_9 s_27_11
        let s_27_12: bool = ((s_27_9) == (s_27_11));
        // D s_27_13: write-var gs#313384 <= s_27_12
        fn_state.gs_313384 = s_27_12;
        // N s_27_14: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0s : i
        let s_28_0: i128 = 0;
        // D s_28_1: read-var b__1:u8
        let s_28_1: u8 = fn_state.b__1;
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 5u16);
        // C s_28_3: const #1s : i64
        let s_28_3: i64 = 1;
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #0s : i
        let s_28_5: i128 = 0;
        // C s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // D s_28_7: bit-extract s_28_2 s_28_0 s_28_6
        let s_28_7: Bits = (Bits::new(
            ((s_28_2) >> (s_28_0)).value(),
            u16::try_from(s_28_6).unwrap(),
        ));
        // D s_28_8: cast reint s_28_7 -> u8
        let s_28_8: bool = ((s_28_7.value()) != 0);
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 1u16);
        // C s_28_10: const #1u : u8
        let s_28_10: bool = true;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 1u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // D s_28_13: write-var gs#313371 <= s_28_12
        fn_state.gs_313371 = s_28_12;
        // N s_28_14: jump b15
        return block_15(state, tracer, fn_state);
    }
}
