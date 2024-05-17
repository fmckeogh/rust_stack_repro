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
use execute_aarch32_instrs_VCVT_xs_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCVT_xs_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm6: u8,
    Vd: u8,
    op: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esizeshadow_7419: i64,
        esize: i64,
        gs_308258: bool,
        gs_308263: bool,
        ga_352536: i64,
        gs_308274: bool,
        frac_bits: i64,
        gs_308264: bool,
        d: i64,
        is_unsigned: bool,
        elements: i64,
        gs_308251: bool,
        gs_308273: bool,
        elementsshadow_7420: i64,
        to_fixed: bool,
        U: bool,
        D: bool,
        imm6: u8,
        Vd: u8,
        op: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm6,
        Vd,
        op,
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
        // D s_2_0: read-var imm6:u8
        let s_2_0: u8 = fn_state.imm6;
        // C s_2_1: const #3s : i
        let s_2_1: i128 = 3;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 6u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #2s : i
        let s_2_5: i128 = 2;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_1 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_1)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // C s_2_10: const #0u : u8
        let s_2_10: u8 = 0;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 3u16);
        // D s_2_12: cmp-eq s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) == (s_2_11));
        // D s_2_13: not s_2_12
        let s_2_13: bool = !s_2_12;
        // N s_2_14: branch s_2_13 b34 b3
        if s_2_13 {
            return block_34(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#308251 <= s_3_0
        fn_state.gs_308251 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308251:u8
        let s_4_0: bool = fn_state.gs_308251;
        // N s_4_1: branch s_4_0 b33 b5
        if s_4_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var op:u8
        let s_5_1: u8 = fn_state.op;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #1u : u64
        let s_5_3: u64 = 1;
        // D s_5_4: bit-extract s_5_2 s_5_0 s_5_3
        let s_5_4: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #0u : u64
        let s_5_7: u64 = 0;
        // D s_5_8: cast zx s_5_5 -> u64
        let s_5_8: u64 = (s_5_5 as u64);
        // C s_5_9: const #1u : u64
        let s_5_9: u64 = 1;
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: u64 = ((s_5_8) & (s_5_9));
        // D s_5_11: cmp-eq s_5_10 s_5_9
        let s_5_11: bool = ((s_5_10) == (s_5_9));
        // D s_5_12: lsl s_5_8 s_5_6
        let s_5_12: u64 = s_5_8 << s_5_6;
        // D s_5_13: or s_5_7 s_5_12
        let s_5_13: u64 = ((s_5_7) | (s_5_12));
        // D s_5_14: cmpl s_5_12
        let s_5_14: u64 = !s_5_12;
        // D s_5_15: and s_5_7 s_5_14
        let s_5_15: u64 = ((s_5_7) & (s_5_14));
        // D s_5_16: select s_5_11 s_5_13 s_5_15
        let s_5_16: u64 = if s_5_11 { s_5_13 } else { s_5_15 };
        // D s_5_17: cast trunc s_5_16 -> u8
        let s_5_17: bool = ((s_5_16) != 0);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #0u : u8
        let s_5_19: bool = false;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // N s_5_22: branch s_5_21 b29 b6
        if s_5_21 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#308263 <= s_6_0
        fn_state.gs_308263 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308263:u8
        let s_7_0: bool = fn_state.gs_308263;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var imm6:u8
        let s_8_0: u8 = fn_state.imm6;
        // C s_8_1: const #5s : i
        let s_8_1: i128 = 5;
        // D s_8_2: cast zx s_8_0 -> bv
        let s_8_2: Bits = Bits::new(s_8_0 as u128, 6u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_1 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_1)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: bool = ((s_8_7.value()) != 0);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // C s_8_10: const #0u : u8
        let s_8_10: bool = false;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 1u16);
        // D s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12: bool = ((s_8_9) == (s_8_11));
        // D s_8_13: not s_8_12
        let s_8_13: bool = !s_8_12;
        // N s_8_14: branch s_8_13 b27 b9
        if s_8_13 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#308264 <= s_9_0
        fn_state.gs_308264 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#308264:u8
        let s_10_0: bool = fn_state.gs_308264;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var Q:u8
        let s_11_0: bool = fn_state.Q;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b22 b12
        if s_11_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#308274 <= s_12_0
        fn_state.gs_308274 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#308274:u8
        let s_13_0: bool = fn_state.gs_308274;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_14_1: read-var op:u8
        let s_14_1: u8 = fn_state.op;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
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
        // D s_14_22: write-var to_fixed <= s_14_21
        fn_state.to_fixed = s_14_21;
        // D s_14_23: read-var imm6:u8
        let s_14_23: u8 = fn_state.imm6;
        // D s_14_24: cast zx s_14_23 -> bv
        let s_14_24: Bits = Bits::new(s_14_23 as u128, 6u16);
        // D s_14_25: cast zx s_14_24 -> i
        let s_14_25: i128 = (s_14_24.value() as i128);
        // D s_14_26: cast reint s_14_25 -> i64
        let s_14_26: i64 = (s_14_25 as i64);
        // C s_14_27: const #64s : i
        let s_14_27: i128 = 64;
        // D s_14_28: cast zx s_14_26 -> i
        let s_14_28: i128 = (i128::try_from(s_14_26).unwrap());
        // D s_14_29: sub s_14_27 s_14_28
        let s_14_29: i128 = ((s_14_27) - (s_14_28));
        // D s_14_30: cast reint s_14_29 -> i64
        let s_14_30: i64 = (s_14_29 as i64);
        // D s_14_31: write-var frac_bits <= s_14_30
        fn_state.frac_bits = s_14_30;
        // D s_14_32: read-var U:u8
        let s_14_32: bool = fn_state.U;
        // D s_14_33: cast zx s_14_32 -> bv
        let s_14_33: Bits = Bits::new(s_14_32 as u128, 1u16);
        // C s_14_34: const #1u : u8
        let s_14_34: bool = true;
        // C s_14_35: cast zx s_14_34 -> bv
        let s_14_35: Bits = Bits::new(s_14_34 as u128, 1u16);
        // D s_14_36: cmp-eq s_14_33 s_14_35
        let s_14_36: bool = ((s_14_33) == (s_14_35));
        // D s_14_37: write-var is_unsigned <= s_14_36
        fn_state.is_unsigned = s_14_36;
        // C s_14_38: const #16s : i64
        let s_14_38: i64 = 16;
        // D s_14_39: write-var esize <= s_14_38
        fn_state.esize = s_14_38;
        // C s_14_40: const #2s : i64
        let s_14_40: i64 = 2;
        // D s_14_41: write-var elements <= s_14_40
        fn_state.elements = s_14_40;
        // C s_14_42: const #1s : i
        let s_14_42: i128 = 1;
        // D s_14_43: read-var op:u8
        let s_14_43: u8 = fn_state.op;
        // D s_14_44: cast zx s_14_43 -> bv
        let s_14_44: Bits = Bits::new(s_14_43 as u128, 2u16);
        // C s_14_45: const #1u : u64
        let s_14_45: u64 = 1;
        // D s_14_46: bit-extract s_14_44 s_14_42 s_14_45
        let s_14_46: Bits = (Bits::new(
            ((s_14_44) >> (s_14_42)).value(),
            u16::try_from(s_14_45).unwrap(),
        ));
        // D s_14_47: cast reint s_14_46 -> u8
        let s_14_47: bool = ((s_14_46.value()) != 0);
        // C s_14_48: const #0s : i
        let s_14_48: i128 = 0;
        // C s_14_49: const #0u : u64
        let s_14_49: u64 = 0;
        // D s_14_50: cast zx s_14_47 -> u64
        let s_14_50: u64 = (s_14_47 as u64);
        // C s_14_51: const #1u : u64
        let s_14_51: u64 = 1;
        // D s_14_52: and s_14_50 s_14_51
        let s_14_52: u64 = ((s_14_50) & (s_14_51));
        // D s_14_53: cmp-eq s_14_52 s_14_51
        let s_14_53: bool = ((s_14_52) == (s_14_51));
        // D s_14_54: lsl s_14_50 s_14_48
        let s_14_54: u64 = s_14_50 << s_14_48;
        // D s_14_55: or s_14_49 s_14_54
        let s_14_55: u64 = ((s_14_49) | (s_14_54));
        // D s_14_56: cmpl s_14_54
        let s_14_56: u64 = !s_14_54;
        // D s_14_57: and s_14_49 s_14_56
        let s_14_57: u64 = ((s_14_49) & (s_14_56));
        // D s_14_58: select s_14_53 s_14_55 s_14_57
        let s_14_58: u64 = if s_14_53 { s_14_55 } else { s_14_57 };
        // D s_14_59: cast trunc s_14_58 -> u8
        let s_14_59: bool = ((s_14_58) != 0);
        // D s_14_60: cast zx s_14_59 -> bv
        let s_14_60: Bits = Bits::new(s_14_59 as u128, 1u16);
        // C s_14_61: const #0u : u8
        let s_14_61: bool = false;
        // C s_14_62: cast zx s_14_61 -> bv
        let s_14_62: Bits = Bits::new(s_14_61 as u128, 1u16);
        // D s_14_63: cmp-eq s_14_60 s_14_62
        let s_14_63: bool = ((s_14_60) == (s_14_62));
        // D s_14_64: not s_14_63
        let s_14_64: bool = !s_14_63;
        // N s_14_65: branch s_14_64 b20 b15
        if s_14_64 {
            return block_20(state, tracer, fn_state);
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
        // C s_15_2: const #4s : i64
        let s_15_2: i64 = 4;
        // D s_15_3: write-var elements <= s_15_2
        fn_state.elements = s_15_2;
        // N s_15_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esize:i64
        let s_16_0: i64 = fn_state.esize;
        // D s_16_1: write-var esizeshadow#7419 <= s_16_0
        fn_state.esizeshadow_7419 = s_16_0;
        // D s_16_2: read-var elements:i64
        let s_16_2: i64 = fn_state.elements;
        // D s_16_3: write-var elementsshadow#7420 <= s_16_2
        fn_state.elementsshadow_7420 = s_16_2;
        // D s_16_4: read-var D:u8
        let s_16_4: bool = fn_state.D;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: read-var Vd:u8
        let s_16_6: u8 = fn_state.Vd;
        // D s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 4u16);
        // D s_16_8: cast reint s_16_5 -> u128
        let s_16_8: u128 = (s_16_5.value() as u128);
        // D s_16_9: size-of s_16_5
        let s_16_9: u16 = s_16_5.length();
        // D s_16_10: cast reint s_16_7 -> u128
        let s_16_10: u128 = (s_16_7.value() as u128);
        // D s_16_11: size-of s_16_7
        let s_16_11: u16 = s_16_7.length();
        // D s_16_12: lsl s_16_8 s_16_11
        let s_16_12: u128 = s_16_8 << s_16_11;
        // D s_16_13: or s_16_12 s_16_10
        let s_16_13: u128 = ((s_16_12) | (s_16_10));
        // D s_16_14: add s_16_9 s_16_11
        let s_16_14: u16 = (s_16_9 + s_16_11);
        // D s_16_15: create-bits s_16_13 s_16_14
        let s_16_15: Bits = Bits::new(s_16_13, s_16_14);
        // D s_16_16: cast reint s_16_15 -> u8
        let s_16_16: u8 = (s_16_15.value() as u8);
        // D s_16_17: cast zx s_16_16 -> bv
        let s_16_17: Bits = Bits::new(s_16_16 as u128, 5u16);
        // D s_16_18: cast zx s_16_17 -> i
        let s_16_18: i128 = (s_16_17.value() as i128);
        // D s_16_19: cast reint s_16_18 -> i64
        let s_16_19: i64 = (s_16_18 as i64);
        // D s_16_20: write-var d <= s_16_19
        fn_state.d = s_16_19;
        // D s_16_21: read-var M:u8
        let s_16_21: bool = fn_state.M;
        // D s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 1u16);
        // D s_16_23: read-var Vm:u8
        let s_16_23: u8 = fn_state.Vm;
        // D s_16_24: cast zx s_16_23 -> bv
        let s_16_24: Bits = Bits::new(s_16_23 as u128, 4u16);
        // D s_16_25: cast reint s_16_22 -> u128
        let s_16_25: u128 = (s_16_22.value() as u128);
        // D s_16_26: size-of s_16_22
        let s_16_26: u16 = s_16_22.length();
        // D s_16_27: cast reint s_16_24 -> u128
        let s_16_27: u128 = (s_16_24.value() as u128);
        // D s_16_28: size-of s_16_24
        let s_16_28: u16 = s_16_24.length();
        // D s_16_29: lsl s_16_25 s_16_28
        let s_16_29: u128 = s_16_25 << s_16_28;
        // D s_16_30: or s_16_29 s_16_27
        let s_16_30: u128 = ((s_16_29) | (s_16_27));
        // D s_16_31: add s_16_26 s_16_28
        let s_16_31: u16 = (s_16_26 + s_16_28);
        // D s_16_32: create-bits s_16_30 s_16_31
        let s_16_32: Bits = Bits::new(s_16_30, s_16_31);
        // D s_16_33: cast reint s_16_32 -> u8
        let s_16_33: u8 = (s_16_32.value() as u8);
        // D s_16_34: cast zx s_16_33 -> bv
        let s_16_34: Bits = Bits::new(s_16_33 as u128, 5u16);
        // D s_16_35: cast zx s_16_34 -> i
        let s_16_35: i128 = (s_16_34.value() as i128);
        // D s_16_36: cast reint s_16_35 -> i64
        let s_16_36: i64 = (s_16_35 as i64);
        // D s_16_37: write-var m <= s_16_36
        fn_state.m = s_16_36;
        // D s_16_38: read-var Q:u8
        let s_16_38: bool = fn_state.Q;
        // D s_16_39: cast zx s_16_38 -> bv
        let s_16_39: Bits = Bits::new(s_16_38 as u128, 1u16);
        // C s_16_40: const #0u : u8
        let s_16_40: bool = false;
        // C s_16_41: cast zx s_16_40 -> bv
        let s_16_41: Bits = Bits::new(s_16_40 as u128, 1u16);
        // D s_16_42: cmp-eq s_16_39 s_16_41
        let s_16_42: bool = ((s_16_39) == (s_16_41));
        // N s_16_43: branch s_16_42 b19 b17
        if s_16_42 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #2s : i64
        let s_17_0: i64 = 2;
        // D s_17_1: write-var ga#352536 <= s_17_0
        fn_state.ga_352536 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var ga#352536:i64
        let s_18_0: i64 = fn_state.ga_352536;
        // D s_18_1: read-var esizeshadow#7419:i64
        let s_18_1: i64 = fn_state.esizeshadow_7419;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // D s_18_4: read-var d:i64
        let s_18_4: i64 = fn_state.d;
        // D s_18_5: read-var elementsshadow#7420:i64
        let s_18_5: i64 = fn_state.elementsshadow_7420;
        // D s_18_6: read-var frac_bits:i64
        let s_18_6: i64 = fn_state.frac_bits;
        // D s_18_7: read-var m:i64
        let s_18_7: i64 = fn_state.m;
        // D s_18_8: read-var to_fixed:u8
        let s_18_8: bool = fn_state.to_fixed;
        // D s_18_9: read-var is_unsigned:u8
        let s_18_9: bool = fn_state.is_unsigned;
        // D s_18_10: call execute_aarch32_instrs_VCVT_xs_Op_A_txt(s_18_4, s_18_5, s_18_3, s_18_6, s_18_7, s_18_0, s_18_8, s_18_9)
        let s_18_10: () = execute_aarch32_instrs_VCVT_xs_Op_A_txt(
            state,
            tracer,
            s_18_4,
            s_18_5,
            s_18_3,
            s_18_6,
            s_18_7,
            s_18_0,
            s_18_8,
            s_18_9,
        );
        // N s_18_11: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1s : i64
        let s_19_0: i64 = 1;
        // D s_19_1: write-var ga#352536 <= s_19_0
        fn_state.ga_352536 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #32s : i64
        let s_20_0: i64 = 32;
        // D s_20_1: write-var esize <= s_20_0
        fn_state.esize = s_20_0;
        // C s_20_2: const #2s : i64
        let s_20_2: i64 = 2;
        // D s_20_3: write-var elements <= s_20_2
        fn_state.elements = s_20_2;
        // N s_20_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var Vd:u8
        let s_22_1: u8 = fn_state.Vd;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 4u16);
        // C s_22_3: const #1u : u64
        let s_22_3: u64 = 1;
        // D s_22_4: bit-extract s_22_2 s_22_0 s_22_3
        let s_22_4: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_3).unwrap(),
        ));
        // D s_22_5: cast reint s_22_4 -> u8
        let s_22_5: bool = ((s_22_4.value()) != 0);
        // C s_22_6: const #0s : i
        let s_22_6: i128 = 0;
        // C s_22_7: const #0u : u64
        let s_22_7: u64 = 0;
        // D s_22_8: cast zx s_22_5 -> u64
        let s_22_8: u64 = (s_22_5 as u64);
        // C s_22_9: const #1u : u64
        let s_22_9: u64 = 1;
        // D s_22_10: and s_22_8 s_22_9
        let s_22_10: u64 = ((s_22_8) & (s_22_9));
        // D s_22_11: cmp-eq s_22_10 s_22_9
        let s_22_11: bool = ((s_22_10) == (s_22_9));
        // D s_22_12: lsl s_22_8 s_22_6
        let s_22_12: u64 = s_22_8 << s_22_6;
        // D s_22_13: or s_22_7 s_22_12
        let s_22_13: u64 = ((s_22_7) | (s_22_12));
        // D s_22_14: cmpl s_22_12
        let s_22_14: u64 = !s_22_12;
        // D s_22_15: and s_22_7 s_22_14
        let s_22_15: u64 = ((s_22_7) & (s_22_14));
        // D s_22_16: select s_22_11 s_22_13 s_22_15
        let s_22_16: u64 = if s_22_11 { s_22_13 } else { s_22_15 };
        // D s_22_17: cast trunc s_22_16 -> u8
        let s_22_17: bool = ((s_22_16) != 0);
        // D s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 1u16);
        // C s_22_19: const #1u : u8
        let s_22_19: bool = true;
        // C s_22_20: cast zx s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 1u16);
        // D s_22_21: cmp-eq s_22_18 s_22_20
        let s_22_21: bool = ((s_22_18) == (s_22_20));
        // N s_22_22: branch s_22_21 b25 b23
        if s_22_21 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i
        let s_23_0: i128 = 0;
        // D s_23_1: read-var Vm:u8
        let s_23_1: u8 = fn_state.Vm;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 4u16);
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
        // C s_23_19: const #1u : u8
        let s_23_19: bool = true;
        // C s_23_20: cast zx s_23_19 -> bv
        let s_23_20: Bits = Bits::new(s_23_19 as u128, 1u16);
        // D s_23_21: cmp-eq s_23_18 s_23_20
        let s_23_21: bool = ((s_23_18) == (s_23_20));
        // D s_23_22: write-var gs#308273 <= s_23_21
        fn_state.gs_308273 = s_23_21;
        // N s_23_23: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#308273:u8
        let s_24_0: bool = fn_state.gs_308273;
        // D s_24_1: write-var gs#308274 <= s_24_0
        fn_state.gs_308274 = s_24_0;
        // N s_24_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#308273 <= s_25_0
        fn_state.gs_308273 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#308264 <= s_27_0
        fn_state.gs_308264 = s_27_0;
        // N s_27_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var imm6:u8
        let s_29_0: u8 = fn_state.imm6;
        // C s_29_1: const #4s : i
        let s_29_1: i128 = 4;
        // D s_29_2: cast zx s_29_0 -> bv
        let s_29_2: Bits = Bits::new(s_29_0 as u128, 6u16);
        // C s_29_3: const #1s : i64
        let s_29_3: i64 = 1;
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #1s : i
        let s_29_5: i128 = 1;
        // C s_29_6: add s_29_5 s_29_4
        let s_29_6: i128 = (s_29_5 + s_29_4);
        // D s_29_7: bit-extract s_29_2 s_29_1 s_29_6
        let s_29_7: Bits = (Bits::new(
            ((s_29_2) >> (s_29_1)).value(),
            u16::try_from(s_29_6).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // D s_29_9: cast zx s_29_8 -> bv
        let s_29_9: Bits = Bits::new(s_29_8 as u128, 2u16);
        // C s_29_10: const #2u : u8
        let s_29_10: u8 = 2;
        // C s_29_11: cast zx s_29_10 -> bv
        let s_29_11: Bits = Bits::new(s_29_10 as u128, 2u16);
        // D s_29_12: cmp-eq s_29_9 s_29_11
        let s_29_12: bool = ((s_29_9) == (s_29_11));
        // D s_29_13: not s_29_12
        let s_29_13: bool = !s_29_12;
        // N s_29_14: branch s_29_13 b32 b30
        if s_29_13 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#308258 <= s_30_0
        fn_state.gs_308258 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#308258:u8
        let s_31_0: bool = fn_state.gs_308258;
        // D s_31_1: write-var gs#308263 <= s_31_0
        fn_state.gs_308263 = s_31_0;
        // N s_31_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#308258 <= s_32_0
        fn_state.gs_308258 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#308251 <= s_34_0
        fn_state.gs_308251 = s_34_0;
        // N s_34_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
