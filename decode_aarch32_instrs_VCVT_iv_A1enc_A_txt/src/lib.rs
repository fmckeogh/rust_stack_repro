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
use FPRoundingMode::*;
use FPSCR_read::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VCVT_iv_Op_A_txt::*;
use HaveFP16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_iv_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    opc2: u8,
    Vd: u8,
    size: u8,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        to_integer: bool,
        esize: i64,
        gs_308135: bool,
        gs_308132: bool,
        gs_308127: bool,
        gs_308134: bool,
        gs_308133: bool,
        d: i64,
        is_unsigned: bool,
        rounding: u32,
        cond: u8,
        D: bool,
        opc2: u8,
        Vd: u8,
        size: u8,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        opc2,
        Vd,
        size,
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
        // D s_2_6: read-var opc2:u8
        let s_2_6: u8 = fn_state.opc2;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 3u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // D s_2_10: cmp-ne s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) != (s_2_9));
        // N s_2_11: branch s_2_10 b40 b3
        if s_2_10 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#308132 <= s_3_0
        fn_state.gs_308132 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308132:u8
        let s_4_0: bool = fn_state.gs_308132;
        // N s_4_1: branch s_4_0 b39 b5
        if s_4_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b38 b6
        if s_5_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
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
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b37 b7
        if s_6_4 {
            return block_37(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#308133 <= s_7_0
        fn_state.gs_308133 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#308133:u8
        let s_8_0: bool = fn_state.gs_308133;
        // D s_8_1: write-var gs#308134 <= s_8_0
        fn_state.gs_308134 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#308134:u8
        let s_9_0: bool = fn_state.gs_308134;
        // N s_9_1: branch s_9_0 b36 b10
        if s_9_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b35 b11
        if s_10_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#308135 <= s_11_0
        fn_state.gs_308135 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#308135:u8
        let s_12_0: bool = fn_state.gs_308135;
        // N s_12_1: branch s_12_0 b34 b13
        if s_12_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16s : i64
        let s_13_0: i64 = 16;
        // D s_13_1: write-var esize <= s_13_0
        fn_state.esize = s_13_0;
        // C s_13_2: const #2s : i
        let s_13_2: i128 = 2;
        // D s_13_3: read-var opc2:u8
        let s_13_3: u8 = fn_state.opc2;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 3u16);
        // C s_13_5: const #1u : u64
        let s_13_5: u64 = 1;
        // D s_13_6: bit-extract s_13_4 s_13_2 s_13_5
        let s_13_6: Bits = (Bits::new(
            ((s_13_4) >> (s_13_2)).value(),
            u16::try_from(s_13_5).unwrap(),
        ));
        // D s_13_7: cast reint s_13_6 -> u8
        let s_13_7: bool = ((s_13_6.value()) != 0);
        // C s_13_8: const #0s : i
        let s_13_8: i128 = 0;
        // C s_13_9: const #0u : u64
        let s_13_9: u64 = 0;
        // D s_13_10: cast zx s_13_7 -> u64
        let s_13_10: u64 = (s_13_7 as u64);
        // C s_13_11: const #1u : u64
        let s_13_11: u64 = 1;
        // D s_13_12: and s_13_10 s_13_11
        let s_13_12: u64 = ((s_13_10) & (s_13_11));
        // D s_13_13: cmp-eq s_13_12 s_13_11
        let s_13_13: bool = ((s_13_12) == (s_13_11));
        // D s_13_14: lsl s_13_10 s_13_8
        let s_13_14: u64 = s_13_10 << s_13_8;
        // D s_13_15: or s_13_9 s_13_14
        let s_13_15: u64 = ((s_13_9) | (s_13_14));
        // D s_13_16: cmpl s_13_14
        let s_13_16: u64 = !s_13_14;
        // D s_13_17: and s_13_9 s_13_16
        let s_13_17: u64 = ((s_13_9) & (s_13_16));
        // D s_13_18: select s_13_13 s_13_15 s_13_17
        let s_13_18: u64 = if s_13_13 { s_13_15 } else { s_13_17 };
        // D s_13_19: cast trunc s_13_18 -> u8
        let s_13_19: bool = ((s_13_18) != 0);
        // D s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // C s_13_21: const #1u : u8
        let s_13_21: bool = true;
        // C s_13_22: cast zx s_13_21 -> bv
        let s_13_22: Bits = Bits::new(s_13_21 as u128, 1u16);
        // D s_13_23: cmp-eq s_13_20 s_13_22
        let s_13_23: bool = ((s_13_20) == (s_13_22));
        // D s_13_24: write-var to_integer <= s_13_23
        fn_state.to_integer = s_13_23;
        // D s_13_25: read-var to_integer:u8
        let s_13_25: bool = fn_state.to_integer;
        // N s_13_26: branch s_13_25 b23 b14
        if s_13_25 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var op:u8
        let s_14_0: bool = fn_state.op;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var is_unsigned <= s_14_4
        fn_state.is_unsigned = s_14_4;
        // C s_14_6: const #() : ()
        let s_14_6: () = ();
        // S s_14_7: call FPSCR_read(s_14_6)
        let s_14_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_14_6);
        // S s_14_8: call FPRoundingMode(s_14_7)
        let s_14_8: u32 = FPRoundingMode(state, tracer, s_14_7);
        // D s_14_9: write-var rounding <= s_14_8
        fn_state.rounding = s_14_8;
        // D s_14_10: read-var Vm:u8
        let s_14_10: u8 = fn_state.Vm;
        // D s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 4u16);
        // D s_14_12: read-var M:u8
        let s_14_12: bool = fn_state.M;
        // D s_14_13: cast zx s_14_12 -> bv
        let s_14_13: Bits = Bits::new(s_14_12 as u128, 1u16);
        // D s_14_14: cast reint s_14_11 -> u128
        let s_14_14: u128 = (s_14_11.value() as u128);
        // D s_14_15: size-of s_14_11
        let s_14_15: u16 = s_14_11.length();
        // D s_14_16: cast reint s_14_13 -> u128
        let s_14_16: u128 = (s_14_13.value() as u128);
        // D s_14_17: size-of s_14_13
        let s_14_17: u16 = s_14_13.length();
        // D s_14_18: lsl s_14_14 s_14_17
        let s_14_18: u128 = s_14_14 << s_14_17;
        // D s_14_19: or s_14_18 s_14_16
        let s_14_19: u128 = ((s_14_18) | (s_14_16));
        // D s_14_20: add s_14_15 s_14_17
        let s_14_20: u16 = (s_14_15 + s_14_17);
        // D s_14_21: create-bits s_14_19 s_14_20
        let s_14_21: Bits = Bits::new(s_14_19, s_14_20);
        // D s_14_22: cast reint s_14_21 -> u8
        let s_14_22: u8 = (s_14_21.value() as u8);
        // D s_14_23: cast zx s_14_22 -> bv
        let s_14_23: Bits = Bits::new(s_14_22 as u128, 5u16);
        // D s_14_24: cast zx s_14_23 -> i
        let s_14_24: i128 = (s_14_23.value() as i128);
        // D s_14_25: cast reint s_14_24 -> i64
        let s_14_25: i64 = (s_14_24 as i64);
        // D s_14_26: write-var m <= s_14_25
        fn_state.m = s_14_25;
        // D s_14_27: read-var size:u8
        let s_14_27: u8 = fn_state.size;
        // D s_14_28: cast zx s_14_27 -> bv
        let s_14_28: Bits = Bits::new(s_14_27 as u128, 2u16);
        // C s_14_29: const #1u : u8
        let s_14_29: u8 = 1;
        // C s_14_30: cast zx s_14_29 -> bv
        let s_14_30: Bits = Bits::new(s_14_29 as u128, 2u16);
        // D s_14_31: cmp-eq s_14_28 s_14_30
        let s_14_31: bool = ((s_14_28) == (s_14_30));
        // D s_14_32: not s_14_31
        let s_14_32: bool = !s_14_31;
        // N s_14_33: branch s_14_32 b18 b15
        if s_14_32 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // D s_15_2: read-var Vd:u8
        let s_15_2: u8 = fn_state.Vd;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
        // D s_15_4: read-var D:u8
        let s_15_4: bool = fn_state.D;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cast reint s_15_3 -> u128
        let s_15_6: u128 = (s_15_3.value() as u128);
        // D s_15_7: size-of s_15_3
        let s_15_7: u16 = s_15_3.length();
        // D s_15_8: cast reint s_15_5 -> u128
        let s_15_8: u128 = (s_15_5.value() as u128);
        // D s_15_9: size-of s_15_5
        let s_15_9: u16 = s_15_5.length();
        // D s_15_10: lsl s_15_6 s_15_9
        let s_15_10: u128 = s_15_6 << s_15_9;
        // D s_15_11: or s_15_10 s_15_8
        let s_15_11: u128 = ((s_15_10) | (s_15_8));
        // D s_15_12: add s_15_7 s_15_9
        let s_15_12: u16 = (s_15_7 + s_15_9);
        // D s_15_13: create-bits s_15_11 s_15_12
        let s_15_13: Bits = Bits::new(s_15_11, s_15_12);
        // D s_15_14: cast reint s_15_13 -> u8
        let s_15_14: u8 = (s_15_13.value() as u8);
        // D s_15_15: cast zx s_15_14 -> bv
        let s_15_15: Bits = Bits::new(s_15_14 as u128, 5u16);
        // D s_15_16: cast zx s_15_15 -> i
        let s_15_16: i128 = (s_15_15.value() as i128);
        // D s_15_17: cast reint s_15_16 -> i64
        let s_15_17: i64 = (s_15_16 as i64);
        // D s_15_18: write-var d <= s_15_17
        fn_state.d = s_15_17;
        // N s_15_19: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var m:i64
        let s_17_0: i64 = fn_state.m;
        // D s_17_1: read-var esize:i64
        let s_17_1: i64 = fn_state.esize;
        // D s_17_2: read-var d:i64
        let s_17_2: i64 = fn_state.d;
        // D s_17_3: read-var rounding:u32
        let s_17_3: u32 = fn_state.rounding;
        // D s_17_4: read-var to_integer:u8
        let s_17_4: bool = fn_state.to_integer;
        // D s_17_5: read-var is_unsigned:u8
        let s_17_5: bool = fn_state.is_unsigned;
        // D s_17_6: call execute_aarch32_instrs_VCVT_iv_Op_A_txt(s_17_2, s_17_1, s_17_0, s_17_3, s_17_4, s_17_5)
        let s_17_6: () = execute_aarch32_instrs_VCVT_iv_Op_A_txt(
            state,
            tracer,
            s_17_2,
            s_17_1,
            s_17_0,
            s_17_3,
            s_17_4,
            s_17_5,
        );
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // D s_19_2: read-var Vd:u8
        let s_19_2: u8 = fn_state.Vd;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: read-var D:u8
        let s_19_4: bool = fn_state.D;
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cast reint s_19_3 -> u128
        let s_19_6: u128 = (s_19_3.value() as u128);
        // D s_19_7: size-of s_19_3
        let s_19_7: u16 = s_19_3.length();
        // D s_19_8: cast reint s_19_5 -> u128
        let s_19_8: u128 = (s_19_5.value() as u128);
        // D s_19_9: size-of s_19_5
        let s_19_9: u16 = s_19_5.length();
        // D s_19_10: lsl s_19_6 s_19_9
        let s_19_10: u128 = s_19_6 << s_19_9;
        // D s_19_11: or s_19_10 s_19_8
        let s_19_11: u128 = ((s_19_10) | (s_19_8));
        // D s_19_12: add s_19_7 s_19_9
        let s_19_12: u16 = (s_19_7 + s_19_9);
        // D s_19_13: create-bits s_19_11 s_19_12
        let s_19_13: Bits = Bits::new(s_19_11, s_19_12);
        // D s_19_14: cast reint s_19_13 -> u8
        let s_19_14: u8 = (s_19_13.value() as u8);
        // D s_19_15: cast zx s_19_14 -> bv
        let s_19_15: Bits = Bits::new(s_19_14 as u128, 5u16);
        // D s_19_16: cast zx s_19_15 -> i
        let s_19_16: i128 = (s_19_15.value() as i128);
        // D s_19_17: cast reint s_19_16 -> i64
        let s_19_17: i64 = (s_19_16 as i64);
        // D s_19_18: write-var d <= s_19_17
        fn_state.d = s_19_17;
        // N s_19_19: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var size:u8
        let s_20_0: u8 = fn_state.size;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #3u : u8
        let s_20_2: u8 = 3;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: write-var esize <= s_21_0
        fn_state.esize = s_21_0;
        // D s_21_2: read-var D:u8
        let s_21_2: bool = fn_state.D;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: read-var Vd:u8
        let s_21_4: u8 = fn_state.Vd;
        // D s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 4u16);
        // D s_21_6: cast reint s_21_3 -> u128
        let s_21_6: u128 = (s_21_3.value() as u128);
        // D s_21_7: size-of s_21_3
        let s_21_7: u16 = s_21_3.length();
        // D s_21_8: cast reint s_21_5 -> u128
        let s_21_8: u128 = (s_21_5.value() as u128);
        // D s_21_9: size-of s_21_5
        let s_21_9: u16 = s_21_5.length();
        // D s_21_10: lsl s_21_6 s_21_9
        let s_21_10: u128 = s_21_6 << s_21_9;
        // D s_21_11: or s_21_10 s_21_8
        let s_21_11: u128 = ((s_21_10) | (s_21_8));
        // D s_21_12: add s_21_7 s_21_9
        let s_21_12: u16 = (s_21_7 + s_21_9);
        // D s_21_13: create-bits s_21_11 s_21_12
        let s_21_13: Bits = Bits::new(s_21_11, s_21_12);
        // D s_21_14: cast reint s_21_13 -> u8
        let s_21_14: u8 = (s_21_13.value() as u8);
        // D s_21_15: cast zx s_21_14 -> bv
        let s_21_15: Bits = Bits::new(s_21_14 as u128, 5u16);
        // D s_21_16: cast zx s_21_15 -> i
        let s_21_16: i128 = (s_21_15.value() as i128);
        // D s_21_17: cast reint s_21_16 -> i64
        let s_21_17: i64 = (s_21_16 as i64);
        // D s_21_18: write-var d <= s_21_17
        fn_state.d = s_21_17;
        // N s_21_19: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i
        let s_23_0: i128 = 0;
        // D s_23_1: read-var opc2:u8
        let s_23_1: u8 = fn_state.opc2;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 3u16);
        // C s_23_3: const #1u : u64
        let s_23_3: u64 = 1;
        // D s_23_4: bit-extract s_23_2 s_23_0 s_23_3
        let s_23_4: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_3).unwrap(),
        ));
        // D s_23_5: cast reint s_23_4 -> u8
        let s_23_5: bool = ((s_23_4.value()) != 0);
        // C s_23_6: const #0s : i
        let s_23_6: i128 = 0;
        // C s_23_7: const #0u : u64
        let s_23_7: u64 = 0;
        // D s_23_8: cast zx s_23_5 -> u64
        let s_23_8: u64 = (s_23_5 as u64);
        // C s_23_9: const #1u : u64
        let s_23_9: u64 = 1;
        // D s_23_10: and s_23_8 s_23_9
        let s_23_10: u64 = ((s_23_8) & (s_23_9));
        // D s_23_11: cmp-eq s_23_10 s_23_9
        let s_23_11: bool = ((s_23_10) == (s_23_9));
        // D s_23_12: lsl s_23_8 s_23_6
        let s_23_12: u64 = s_23_8 << s_23_6;
        // D s_23_13: or s_23_7 s_23_12
        let s_23_13: u64 = ((s_23_7) | (s_23_12));
        // D s_23_14: cmpl s_23_12
        let s_23_14: u64 = !s_23_12;
        // D s_23_15: and s_23_7 s_23_14
        let s_23_15: u64 = ((s_23_7) & (s_23_14));
        // D s_23_16: select s_23_11 s_23_13 s_23_15
        let s_23_16: u64 = if s_23_11 { s_23_13 } else { s_23_15 };
        // D s_23_17: cast trunc s_23_16 -> u8
        let s_23_17: bool = ((s_23_16) != 0);
        // D s_23_18: cast zx s_23_17 -> bv
        let s_23_18: Bits = Bits::new(s_23_17 as u128, 1u16);
        // C s_23_19: const #0u : u8
        let s_23_19: bool = false;
        // C s_23_20: cast zx s_23_19 -> bv
        let s_23_20: Bits = Bits::new(s_23_19 as u128, 1u16);
        // D s_23_21: cmp-eq s_23_18 s_23_20
        let s_23_21: bool = ((s_23_18) == (s_23_20));
        // D s_23_22: write-var is_unsigned <= s_23_21
        fn_state.is_unsigned = s_23_21;
        // D s_23_23: read-var op:u8
        let s_23_23: bool = fn_state.op;
        // D s_23_24: cast zx s_23_23 -> bv
        let s_23_24: Bits = Bits::new(s_23_23 as u128, 1u16);
        // C s_23_25: const #1u : u8
        let s_23_25: bool = true;
        // C s_23_26: cast zx s_23_25 -> bv
        let s_23_26: Bits = Bits::new(s_23_25 as u128, 1u16);
        // D s_23_27: cmp-eq s_23_24 s_23_26
        let s_23_27: bool = ((s_23_24) == (s_23_26));
        // N s_23_28: branch s_23_27 b33 b24
        if s_23_27 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call FPSCR_read(s_24_0)
        let s_24_1: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_24_0);
        // S s_24_2: call FPRoundingMode(s_24_1)
        let s_24_2: u32 = FPRoundingMode(state, tracer, s_24_1);
        // D s_24_3: write-var rounding <= s_24_2
        fn_state.rounding = s_24_2;
        // N s_24_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var Vd:u8
        let s_25_0: u8 = fn_state.Vd;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 4u16);
        // D s_25_2: read-var D:u8
        let s_25_2: bool = fn_state.D;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cast reint s_25_1 -> u128
        let s_25_4: u128 = (s_25_1.value() as u128);
        // D s_25_5: size-of s_25_1
        let s_25_5: u16 = s_25_1.length();
        // D s_25_6: cast reint s_25_3 -> u128
        let s_25_6: u128 = (s_25_3.value() as u128);
        // D s_25_7: size-of s_25_3
        let s_25_7: u16 = s_25_3.length();
        // D s_25_8: lsl s_25_4 s_25_7
        let s_25_8: u128 = s_25_4 << s_25_7;
        // D s_25_9: or s_25_8 s_25_6
        let s_25_9: u128 = ((s_25_8) | (s_25_6));
        // D s_25_10: add s_25_5 s_25_7
        let s_25_10: u16 = (s_25_5 + s_25_7);
        // D s_25_11: create-bits s_25_9 s_25_10
        let s_25_11: Bits = Bits::new(s_25_9, s_25_10);
        // D s_25_12: cast reint s_25_11 -> u8
        let s_25_12: u8 = (s_25_11.value() as u8);
        // D s_25_13: cast zx s_25_12 -> bv
        let s_25_13: Bits = Bits::new(s_25_12 as u128, 5u16);
        // D s_25_14: cast zx s_25_13 -> i
        let s_25_14: i128 = (s_25_13.value() as i128);
        // D s_25_15: cast reint s_25_14 -> i64
        let s_25_15: i64 = (s_25_14 as i64);
        // D s_25_16: write-var d <= s_25_15
        fn_state.d = s_25_15;
        // D s_25_17: read-var size:u8
        let s_25_17: u8 = fn_state.size;
        // D s_25_18: cast zx s_25_17 -> bv
        let s_25_18: Bits = Bits::new(s_25_17 as u128, 2u16);
        // C s_25_19: const #1u : u8
        let s_25_19: u8 = 1;
        // C s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 2u16);
        // D s_25_21: cmp-eq s_25_18 s_25_20
        let s_25_21: bool = ((s_25_18) == (s_25_20));
        // D s_25_22: not s_25_21
        let s_25_22: bool = !s_25_21;
        // N s_25_23: branch s_25_22 b28 b26
        if s_25_22 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #16s : i64
        let s_26_0: i64 = 16;
        // D s_26_1: write-var esize <= s_26_0
        fn_state.esize = s_26_0;
        // D s_26_2: read-var Vm:u8
        let s_26_2: u8 = fn_state.Vm;
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 4u16);
        // D s_26_4: read-var M:u8
        let s_26_4: bool = fn_state.M;
        // D s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cast reint s_26_3 -> u128
        let s_26_6: u128 = (s_26_3.value() as u128);
        // D s_26_7: size-of s_26_3
        let s_26_7: u16 = s_26_3.length();
        // D s_26_8: cast reint s_26_5 -> u128
        let s_26_8: u128 = (s_26_5.value() as u128);
        // D s_26_9: size-of s_26_5
        let s_26_9: u16 = s_26_5.length();
        // D s_26_10: lsl s_26_6 s_26_9
        let s_26_10: u128 = s_26_6 << s_26_9;
        // D s_26_11: or s_26_10 s_26_8
        let s_26_11: u128 = ((s_26_10) | (s_26_8));
        // D s_26_12: add s_26_7 s_26_9
        let s_26_12: u16 = (s_26_7 + s_26_9);
        // D s_26_13: create-bits s_26_11 s_26_12
        let s_26_13: Bits = Bits::new(s_26_11, s_26_12);
        // D s_26_14: cast reint s_26_13 -> u8
        let s_26_14: u8 = (s_26_13.value() as u8);
        // D s_26_15: cast zx s_26_14 -> bv
        let s_26_15: Bits = Bits::new(s_26_14 as u128, 5u16);
        // D s_26_16: cast zx s_26_15 -> i
        let s_26_16: i128 = (s_26_15.value() as i128);
        // D s_26_17: cast reint s_26_16 -> i64
        let s_26_17: i64 = (s_26_16 as i64);
        // D s_26_18: write-var m <= s_26_17
        fn_state.m = s_26_17;
        // N s_26_19: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var size:u8
        let s_28_0: u8 = fn_state.size;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 2u16);
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 2u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #32s : i64
        let s_29_0: i64 = 32;
        // D s_29_1: write-var esize <= s_29_0
        fn_state.esize = s_29_0;
        // D s_29_2: read-var Vm:u8
        let s_29_2: u8 = fn_state.Vm;
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // D s_29_4: read-var M:u8
        let s_29_4: bool = fn_state.M;
        // D s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cast reint s_29_3 -> u128
        let s_29_6: u128 = (s_29_3.value() as u128);
        // D s_29_7: size-of s_29_3
        let s_29_7: u16 = s_29_3.length();
        // D s_29_8: cast reint s_29_5 -> u128
        let s_29_8: u128 = (s_29_5.value() as u128);
        // D s_29_9: size-of s_29_5
        let s_29_9: u16 = s_29_5.length();
        // D s_29_10: lsl s_29_6 s_29_9
        let s_29_10: u128 = s_29_6 << s_29_9;
        // D s_29_11: or s_29_10 s_29_8
        let s_29_11: u128 = ((s_29_10) | (s_29_8));
        // D s_29_12: add s_29_7 s_29_9
        let s_29_12: u16 = (s_29_7 + s_29_9);
        // D s_29_13: create-bits s_29_11 s_29_12
        let s_29_13: Bits = Bits::new(s_29_11, s_29_12);
        // D s_29_14: cast reint s_29_13 -> u8
        let s_29_14: u8 = (s_29_13.value() as u8);
        // D s_29_15: cast zx s_29_14 -> bv
        let s_29_15: Bits = Bits::new(s_29_14 as u128, 5u16);
        // D s_29_16: cast zx s_29_15 -> i
        let s_29_16: i128 = (s_29_15.value() as i128);
        // D s_29_17: cast reint s_29_16 -> i64
        let s_29_17: i64 = (s_29_16 as i64);
        // D s_29_18: write-var m <= s_29_17
        fn_state.m = s_29_17;
        // N s_29_19: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var size:u8
        let s_30_0: u8 = fn_state.size;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #3u : u8
        let s_30_2: u8 = 3;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 2u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i64
        let s_31_0: i64 = 64;
        // D s_31_1: write-var esize <= s_31_0
        fn_state.esize = s_31_0;
        // D s_31_2: read-var M:u8
        let s_31_2: bool = fn_state.M;
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: read-var Vm:u8
        let s_31_4: u8 = fn_state.Vm;
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 4u16);
        // D s_31_6: cast reint s_31_3 -> u128
        let s_31_6: u128 = (s_31_3.value() as u128);
        // D s_31_7: size-of s_31_3
        let s_31_7: u16 = s_31_3.length();
        // D s_31_8: cast reint s_31_5 -> u128
        let s_31_8: u128 = (s_31_5.value() as u128);
        // D s_31_9: size-of s_31_5
        let s_31_9: u16 = s_31_5.length();
        // D s_31_10: lsl s_31_6 s_31_9
        let s_31_10: u128 = s_31_6 << s_31_9;
        // D s_31_11: or s_31_10 s_31_8
        let s_31_11: u128 = ((s_31_10) | (s_31_8));
        // D s_31_12: add s_31_7 s_31_9
        let s_31_12: u16 = (s_31_7 + s_31_9);
        // D s_31_13: create-bits s_31_11 s_31_12
        let s_31_13: Bits = Bits::new(s_31_11, s_31_12);
        // D s_31_14: cast reint s_31_13 -> u8
        let s_31_14: u8 = (s_31_13.value() as u8);
        // D s_31_15: cast zx s_31_14 -> bv
        let s_31_15: Bits = Bits::new(s_31_14 as u128, 5u16);
        // D s_31_16: cast zx s_31_15 -> i
        let s_31_16: i128 = (s_31_15.value() as i128);
        // D s_31_17: cast reint s_31_16 -> i64
        let s_31_17: i64 = (s_31_16 as i64);
        // D s_31_18: write-var m <= s_31_17
        fn_state.m = s_31_17;
        // N s_31_19: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #3u : u32
        let s_33_0: u32 = 3;
        // D s_33_1: write-var rounding <= s_33_0
        fn_state.rounding = s_33_0;
        // N s_33_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var cond:u8
        let s_35_0: u8 = fn_state.cond;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 4u16);
        // C s_35_2: const #14u : u8
        let s_35_2: u8 = 14;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 4u16);
        // D s_35_4: cmp-ne s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) != (s_35_3));
        // D s_35_5: write-var gs#308135 <= s_35_4
        fn_state.gs_308135 = s_35_4;
        // N s_35_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call HaveFP16Ext(s_37_0)
        let s_37_1: bool = HaveFP16Ext(state, tracer, s_37_0);
        // S s_37_2: not s_37_1
        let s_37_2: bool = !s_37_1;
        // D s_37_3: write-var gs#308133 <= s_37_2
        fn_state.gs_308133 = s_37_2;
        // N s_37_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#308134 <= s_38_0
        fn_state.gs_308134 = s_38_0;
        // N s_38_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var opc2:u8
        let s_40_0: u8 = fn_state.opc2;
        // C s_40_1: const #1s : i
        let s_40_1: i128 = 1;
        // D s_40_2: cast zx s_40_0 -> bv
        let s_40_2: Bits = Bits::new(s_40_0 as u128, 3u16);
        // C s_40_3: const #1s : i64
        let s_40_3: i64 = 1;
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #1s : i
        let s_40_5: i128 = 1;
        // C s_40_6: add s_40_5 s_40_4
        let s_40_6: i128 = (s_40_5 + s_40_4);
        // D s_40_7: bit-extract s_40_2 s_40_1 s_40_6
        let s_40_7: Bits = (Bits::new(
            ((s_40_2) >> (s_40_1)).value(),
            u16::try_from(s_40_6).unwrap(),
        ));
        // D s_40_8: cast reint s_40_7 -> u8
        let s_40_8: u8 = (s_40_7.value() as u8);
        // D s_40_9: cast zx s_40_8 -> bv
        let s_40_9: Bits = Bits::new(s_40_8 as u128, 2u16);
        // C s_40_10: const #2u : u8
        let s_40_10: u8 = 2;
        // C s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 2u16);
        // D s_40_12: cmp-eq s_40_9 s_40_11
        let s_40_12: bool = ((s_40_9) == (s_40_11));
        // D s_40_13: not s_40_12
        let s_40_13: bool = !s_40_12;
        // N s_40_14: branch s_40_13 b43 b41
        if s_40_13 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#308127 <= s_41_0
        fn_state.gs_308127 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#308127:u8
        let s_42_0: bool = fn_state.gs_308127;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // D s_42_2: write-var gs#308132 <= s_42_1
        fn_state.gs_308132 = s_42_1;
        // N s_42_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#308127 <= s_43_0
        fn_state.gs_308127 = s_43_0;
        // N s_43_2: jump b42
        return block_42(state, tracer, fn_state);
    }
}
