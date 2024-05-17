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
use execute_aarch64_instrs_memory_single_general_register::*;
use DecodeRegExtend::*;
use common::*;
pub fn decode_prfm_reg_aarch64_instrs_memory_single_general_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    S: bool,
    option_name: u8,
    Rm: u8,
    opc: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_163718: bool,
        t: i64,
        shift: i64,
        n: i64,
        memop: u32,
        extend_type: u32,
        scale: i64,
        is_signed: bool,
        regsize: i64,
        gs_163697: bool,
        ga_262410: i64,
        Rt: u8,
        Rn: u8,
        S: bool,
        option_name: u8,
        Rm: u8,
        opc: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        S,
        option_name,
        Rm,
        opc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rt:u8
        let s_0_0: u8 = fn_state.Rt;
        // C s_0_1: const #3s : i
        let s_0_1: i128 = 3;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 5u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_1 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // C s_0_10: const #3u : u8
        let s_0_10: u8 = 3;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_9 s_0_11
        let s_0_12: bool = ((s_0_9) == (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b29 b1
        if s_0_13 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var gs#163697 <= s_1_0
        fn_state.gs_163697 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#163697:u8
        let s_2_0: bool = fn_state.gs_163697;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: assert s_2_1
        let s_2_2: () = assert!(s_2_1);
        // D s_2_3: read-var size:u8
        let s_2_3: u8 = fn_state.size;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (s_2_4.value() as i128);
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: write-var scale <= s_2_6
        fn_state.scale = s_2_6;
        // C s_2_8: const #1s : i
        let s_2_8: i128 = 1;
        // D s_2_9: read-var option_name:u8
        let s_2_9: u8 = fn_state.option_name;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 3u16);
        // C s_2_11: const #1u : u64
        let s_2_11: u64 = 1;
        // D s_2_12: bit-extract s_2_10 s_2_8 s_2_11
        let s_2_12: Bits = (Bits::new(
            ((s_2_10) >> (s_2_8)).value(),
            u16::try_from(s_2_11).unwrap(),
        ));
        // D s_2_13: cast reint s_2_12 -> u8
        let s_2_13: bool = ((s_2_12.value()) != 0);
        // C s_2_14: const #0s : i
        let s_2_14: i128 = 0;
        // C s_2_15: const #0u : u64
        let s_2_15: u64 = 0;
        // D s_2_16: cast zx s_2_13 -> u64
        let s_2_16: u64 = (s_2_13 as u64);
        // C s_2_17: const #1u : u64
        let s_2_17: u64 = 1;
        // D s_2_18: and s_2_16 s_2_17
        let s_2_18: u64 = ((s_2_16) & (s_2_17));
        // D s_2_19: cmp-eq s_2_18 s_2_17
        let s_2_19: bool = ((s_2_18) == (s_2_17));
        // D s_2_20: lsl s_2_16 s_2_14
        let s_2_20: u64 = s_2_16 << s_2_14;
        // D s_2_21: or s_2_15 s_2_20
        let s_2_21: u64 = ((s_2_15) | (s_2_20));
        // D s_2_22: cmpl s_2_20
        let s_2_22: u64 = !s_2_20;
        // D s_2_23: and s_2_15 s_2_22
        let s_2_23: u64 = ((s_2_15) & (s_2_22));
        // D s_2_24: select s_2_19 s_2_21 s_2_23
        let s_2_24: u64 = if s_2_19 { s_2_21 } else { s_2_23 };
        // D s_2_25: cast trunc s_2_24 -> u8
        let s_2_25: bool = ((s_2_24) != 0);
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // C s_2_27: const #0u : u8
        let s_2_27: bool = false;
        // C s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 1u16);
        // D s_2_29: cmp-eq s_2_26 s_2_28
        let s_2_29: bool = ((s_2_26) == (s_2_28));
        // N s_2_30: branch s_2_29 b28 b3
        if s_2_29 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var option_name:u8
        let s_3_0: u8 = fn_state.option_name;
        // D s_3_1: call DecodeRegExtend(s_3_0)
        let s_3_1: u32 = DecodeRegExtend(state, tracer, s_3_0);
        // D s_3_2: write-var extend_type <= s_3_1
        fn_state.extend_type = s_3_1;
        // D s_3_3: read-var S:u8
        let s_3_3: bool = fn_state.S;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 1u16);
        // C s_3_5: const #1u : u8
        let s_3_5: bool = true;
        // C s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 1u16);
        // D s_3_7: cmp-eq s_3_4 s_3_6
        let s_3_7: bool = ((s_3_4) == (s_3_6));
        // N s_3_8: branch s_3_7 b27 b4
        if s_3_7 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // D s_4_1: write-var ga#262410 <= s_4_0
        fn_state.ga_262410 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#262410:i64
        let s_5_0: i64 = fn_state.ga_262410;
        // D s_5_1: write-var shift <= s_5_0
        fn_state.shift = s_5_0;
        // D s_5_2: read-var Rn:u8
        let s_5_2: u8 = fn_state.Rn;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 5u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var n <= s_5_5
        fn_state.n = s_5_5;
        // D s_5_7: read-var Rt:u8
        let s_5_7: u8 = fn_state.Rt;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 5u16);
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (s_5_8.value() as i128);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: write-var t <= s_5_10
        fn_state.t = s_5_10;
        // D s_5_12: read-var Rm:u8
        let s_5_12: u8 = fn_state.Rm;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var m <= s_5_15
        fn_state.m = s_5_15;
        // C s_5_17: const #32s : i64
        let s_5_17: i64 = 32;
        // D s_5_18: write-var regsize <= s_5_17
        fn_state.regsize = s_5_17;
        // C s_5_19: const #1s : i
        let s_5_19: i128 = 1;
        // D s_5_20: read-var opc:u8
        let s_5_20: u8 = fn_state.opc;
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 2u16);
        // C s_5_22: const #1u : u64
        let s_5_22: u64 = 1;
        // D s_5_23: bit-extract s_5_21 s_5_19 s_5_22
        let s_5_23: Bits = (Bits::new(
            ((s_5_21) >> (s_5_19)).value(),
            u16::try_from(s_5_22).unwrap(),
        ));
        // D s_5_24: cast reint s_5_23 -> u8
        let s_5_24: bool = ((s_5_23.value()) != 0);
        // C s_5_25: const #0s : i
        let s_5_25: i128 = 0;
        // C s_5_26: const #0u : u64
        let s_5_26: u64 = 0;
        // D s_5_27: cast zx s_5_24 -> u64
        let s_5_27: u64 = (s_5_24 as u64);
        // C s_5_28: const #1u : u64
        let s_5_28: u64 = 1;
        // D s_5_29: and s_5_27 s_5_28
        let s_5_29: u64 = ((s_5_27) & (s_5_28));
        // D s_5_30: cmp-eq s_5_29 s_5_28
        let s_5_30: bool = ((s_5_29) == (s_5_28));
        // D s_5_31: lsl s_5_27 s_5_25
        let s_5_31: u64 = s_5_27 << s_5_25;
        // D s_5_32: or s_5_26 s_5_31
        let s_5_32: u64 = ((s_5_26) | (s_5_31));
        // D s_5_33: cmpl s_5_31
        let s_5_33: u64 = !s_5_31;
        // D s_5_34: and s_5_26 s_5_33
        let s_5_34: u64 = ((s_5_26) & (s_5_33));
        // D s_5_35: select s_5_30 s_5_32 s_5_34
        let s_5_35: u64 = if s_5_30 { s_5_32 } else { s_5_34 };
        // D s_5_36: cast trunc s_5_35 -> u8
        let s_5_36: bool = ((s_5_35) != 0);
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 1u16);
        // C s_5_38: const #0u : u8
        let s_5_38: bool = false;
        // C s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 1u16);
        // D s_5_40: cmp-eq s_5_37 s_5_39
        let s_5_40: bool = ((s_5_37) == (s_5_39));
        // N s_5_41: branch s_5_40 b20 b6
        if s_5_40 {
            return block_20(state, tracer, fn_state);
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
        // C s_6_2: const #3u : u8
        let s_6_2: u8 = 3;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b17 b7
        if s_6_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u32
        let s_7_0: u32 = 0;
        // D s_7_1: write-var memop <= s_7_0
        fn_state.memop = s_7_0;
        // D s_7_2: read-var size:u8
        let s_7_2: u8 = fn_state.size;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // C s_7_4: const #2u : u8
        let s_7_4: u8 = 2;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b16 b8
        if s_7_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#163718 <= s_8_0
        fn_state.gs_163718 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#163718:u8
        let s_9_0: bool = fn_state.gs_163718;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var opc:u8
        let s_10_1: u8 = fn_state.opc;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 2u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // N s_10_22: branch s_10_21 b14 b11
        if s_10_21 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: write-var regsize <= s_11_0
        fn_state.regsize = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var is_signed <= s_12_0
        fn_state.is_signed = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var regsize:i64
        let s_13_0: i64 = fn_state.regsize;
        // C s_13_1: const #8s : i64
        let s_13_1: i64 = 8;
        // D s_13_2: read-var scale:i64
        let s_13_2: i64 = fn_state.scale;
        // D s_13_3: lsl s_13_1 s_13_2
        let s_13_3: i64 = s_13_1 << s_13_2;
        // D s_13_4: read-var memop:u32
        let s_13_4: u32 = fn_state.memop;
        // C s_13_5: const #2u : u32
        let s_13_5: u32 = 2;
        // D s_13_6: cmp-eq s_13_4 s_13_5
        let s_13_6: bool = ((s_13_4) == (s_13_5));
        // D s_13_7: cast zx s_13_3 -> i
        let s_13_7: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_8: cast reint s_13_7 -> i64
        let s_13_8: i64 = (s_13_7 as i64);
        // D s_13_9: cast zx s_13_0 -> i
        let s_13_9: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // D s_13_11: read-var extend_type:u32
        let s_13_11: u32 = fn_state.extend_type;
        // D s_13_12: read-var m:i64
        let s_13_12: i64 = fn_state.m;
        // D s_13_13: read-var memop:u32
        let s_13_13: u32 = fn_state.memop;
        // D s_13_14: read-var n:i64
        let s_13_14: i64 = fn_state.n;
        // C s_13_15: const #0u : u8
        let s_13_15: bool = false;
        // C s_13_16: const #0u : u8
        let s_13_16: bool = false;
        // C s_13_17: const #0u : u8
        let s_13_17: bool = false;
        // D s_13_18: read-var shift:i64
        let s_13_18: i64 = fn_state.shift;
        // D s_13_19: read-var is_signed:u8
        let s_13_19: bool = fn_state.is_signed;
        // D s_13_20: read-var t:i64
        let s_13_20: i64 = fn_state.t;
        // C s_13_21: const #0u : u8
        let s_13_21: bool = false;
        // C s_13_22: const #0u : u8
        let s_13_22: bool = false;
        // D s_13_23: call execute_aarch64_instrs_memory_single_general_register(s_13_8, s_13_11, s_13_12, s_13_13, s_13_14, s_13_15, s_13_16, s_13_10, s_13_17, s_13_18, s_13_19, s_13_20, s_13_6, s_13_21, s_13_22)
        let s_13_23: () = execute_aarch64_instrs_memory_single_general_register(
            state,
            tracer,
            s_13_8,
            s_13_11,
            s_13_12,
            s_13_13,
            s_13_14,
            s_13_15,
            s_13_16,
            s_13_10,
            s_13_17,
            s_13_18,
            s_13_19,
            s_13_20,
            s_13_6,
            s_13_21,
            s_13_22,
        );
        // N s_13_24: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // D s_14_1: write-var regsize <= s_14_0
        fn_state.regsize = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var opc:u8
        let s_16_1: u8 = fn_state.opc;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #1u : u8
        let s_16_19: bool = true;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // D s_16_22: write-var gs#163718 <= s_16_21
        fn_state.gs_163718 = s_16_21;
        // N s_16_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #2u : u32
        let s_17_0: u32 = 2;
        // D s_17_1: write-var memop <= s_17_0
        fn_state.memop = s_17_0;
        // C s_17_2: const #0s : i
        let s_17_2: i128 = 0;
        // D s_17_3: read-var opc:u8
        let s_17_3: u8 = fn_state.opc;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // C s_17_5: const #1u : u64
        let s_17_5: u64 = 1;
        // D s_17_6: bit-extract s_17_4 s_17_2 s_17_5
        let s_17_6: Bits = (Bits::new(
            ((s_17_4) >> (s_17_2)).value(),
            u16::try_from(s_17_5).unwrap(),
        ));
        // D s_17_7: cast reint s_17_6 -> u8
        let s_17_7: bool = ((s_17_6.value()) != 0);
        // C s_17_8: const #0s : i
        let s_17_8: i128 = 0;
        // C s_17_9: const #0u : u64
        let s_17_9: u64 = 0;
        // D s_17_10: cast zx s_17_7 -> u64
        let s_17_10: u64 = (s_17_7 as u64);
        // C s_17_11: const #1u : u64
        let s_17_11: u64 = 1;
        // D s_17_12: and s_17_10 s_17_11
        let s_17_12: u64 = ((s_17_10) & (s_17_11));
        // D s_17_13: cmp-eq s_17_12 s_17_11
        let s_17_13: bool = ((s_17_12) == (s_17_11));
        // D s_17_14: lsl s_17_10 s_17_8
        let s_17_14: u64 = s_17_10 << s_17_8;
        // D s_17_15: or s_17_9 s_17_14
        let s_17_15: u64 = ((s_17_9) | (s_17_14));
        // D s_17_16: cmpl s_17_14
        let s_17_16: u64 = !s_17_14;
        // D s_17_17: and s_17_9 s_17_16
        let s_17_17: u64 = ((s_17_9) & (s_17_16));
        // D s_17_18: select s_17_13 s_17_15 s_17_17
        let s_17_18: u64 = if s_17_13 { s_17_15 } else { s_17_17 };
        // D s_17_19: cast trunc s_17_18 -> u8
        let s_17_19: bool = ((s_17_18) != 0);
        // D s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 1u16);
        // C s_17_21: const #1u : u8
        let s_17_21: bool = true;
        // C s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 1u16);
        // D s_17_23: cmp-eq s_17_20 s_17_22
        let s_17_23: bool = ((s_17_20) == (s_17_22));
        // N s_17_24: branch s_17_23 b19 b18
        if s_17_23 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b13
        return block_13(state, tracer, fn_state);
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
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var opc:u8
        let s_20_1: u8 = fn_state.opc;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 2u16);
        // C s_20_3: const #1u : u64
        let s_20_3: u64 = 1;
        // D s_20_4: bit-extract s_20_2 s_20_0 s_20_3
        let s_20_4: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: bool = ((s_20_4.value()) != 0);
        // C s_20_6: const #0s : i
        let s_20_6: i128 = 0;
        // C s_20_7: const #0u : u64
        let s_20_7: u64 = 0;
        // D s_20_8: cast zx s_20_5 -> u64
        let s_20_8: u64 = (s_20_5 as u64);
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // D s_20_10: and s_20_8 s_20_9
        let s_20_10: u64 = ((s_20_8) & (s_20_9));
        // D s_20_11: cmp-eq s_20_10 s_20_9
        let s_20_11: bool = ((s_20_10) == (s_20_9));
        // D s_20_12: lsl s_20_8 s_20_6
        let s_20_12: u64 = s_20_8 << s_20_6;
        // D s_20_13: or s_20_7 s_20_12
        let s_20_13: u64 = ((s_20_7) | (s_20_12));
        // D s_20_14: cmpl s_20_12
        let s_20_14: u64 = !s_20_12;
        // D s_20_15: and s_20_7 s_20_14
        let s_20_15: u64 = ((s_20_7) & (s_20_14));
        // D s_20_16: select s_20_11 s_20_13 s_20_15
        let s_20_16: u64 = if s_20_11 { s_20_13 } else { s_20_15 };
        // D s_20_17: cast trunc s_20_16 -> u8
        let s_20_17: bool = ((s_20_16) != 0);
        // D s_20_18: cast zx s_20_17 -> bv
        let s_20_18: Bits = Bits::new(s_20_17 as u128, 1u16);
        // C s_20_19: const #1u : u8
        let s_20_19: bool = true;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // D s_20_21: cmp-eq s_20_18 s_20_20
        let s_20_21: bool = ((s_20_18) == (s_20_20));
        // N s_20_22: branch s_20_21 b26 b21
        if s_20_21 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u32
        let s_21_0: u32 = 1;
        // D s_21_1: write-var memop <= s_21_0
        fn_state.memop = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var size:u8
        let s_22_0: u8 = fn_state.size;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #3u : u8
        let s_22_2: u8 = 3;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b25 b23
        if s_22_4 {
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
        // C s_23_0: const #32s : i64
        let s_23_0: i64 = 32;
        // D s_23_1: write-var regsize <= s_23_0
        fn_state.regsize = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var is_signed <= s_24_0
        fn_state.is_signed = s_24_0;
        // N s_24_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // D s_25_1: write-var regsize <= s_25_0
        fn_state.regsize = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u32
        let s_26_0: u32 = 0;
        // D s_26_1: write-var memop <= s_26_0
        fn_state.memop = s_26_0;
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var scale:i64
        let s_27_0: i64 = fn_state.scale;
        // D s_27_1: write-var ga#262410 <= s_27_0
        fn_state.ga_262410 = s_27_0;
        // N s_27_2: jump b5
        return block_5(state, tracer, fn_state);
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
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#163697 <= s_29_0
        fn_state.gs_163697 = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
