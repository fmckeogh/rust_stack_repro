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
pub fn decode_strh_reg_aarch64_instrs_memory_single_general_register<T: Tracer>(
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
        t: i64,
        shift: i64,
        gs_163853: bool,
        n: i64,
        memop: u32,
        extend_type: u32,
        scale: i64,
        is_signed: bool,
        regsize: i64,
        ga_262509: i64,
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
        // D s_0_0: read-var size:u8
        let s_0_0: u8 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var scale <= s_0_3
        fn_state.scale = s_0_3;
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // D s_0_6: read-var option_name:u8
        let s_0_6: u8 = fn_state.option_name;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 3u16);
        // C s_0_8: const #1u : u64
        let s_0_8: u64 = 1;
        // D s_0_9: bit-extract s_0_7 s_0_5 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_7) >> (s_0_5)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: bool = ((s_0_9.value()) != 0);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // C s_0_12: const #0u : u64
        let s_0_12: u64 = 0;
        // D s_0_13: cast zx s_0_10 -> u64
        let s_0_13: u64 = (s_0_10 as u64);
        // C s_0_14: const #1u : u64
        let s_0_14: u64 = 1;
        // D s_0_15: and s_0_13 s_0_14
        let s_0_15: u64 = ((s_0_13) & (s_0_14));
        // D s_0_16: cmp-eq s_0_15 s_0_14
        let s_0_16: bool = ((s_0_15) == (s_0_14));
        // D s_0_17: lsl s_0_13 s_0_11
        let s_0_17: u64 = s_0_13 << s_0_11;
        // D s_0_18: or s_0_12 s_0_17
        let s_0_18: u64 = ((s_0_12) | (s_0_17));
        // D s_0_19: cmpl s_0_17
        let s_0_19: u64 = !s_0_17;
        // D s_0_20: and s_0_12 s_0_19
        let s_0_20: u64 = ((s_0_12) & (s_0_19));
        // D s_0_21: select s_0_16 s_0_18 s_0_20
        let s_0_21: u64 = if s_0_16 { s_0_18 } else { s_0_20 };
        // D s_0_22: cast trunc s_0_21 -> u8
        let s_0_22: bool = ((s_0_21) != 0);
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // C s_0_24: const #0u : u8
        let s_0_24: bool = false;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 1u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b26 b1
        if s_0_26 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var option_name:u8
        let s_1_0: u8 = fn_state.option_name;
        // D s_1_1: call DecodeRegExtend(s_1_0)
        let s_1_1: u32 = DecodeRegExtend(state, tracer, s_1_0);
        // D s_1_2: write-var extend_type <= s_1_1
        fn_state.extend_type = s_1_1;
        // D s_1_3: read-var S:u8
        let s_1_3: bool = fn_state.S;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 1u16);
        // C s_1_5: const #1u : u8
        let s_1_5: bool = true;
        // C s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 1u16);
        // D s_1_7: cmp-eq s_1_4 s_1_6
        let s_1_7: bool = ((s_1_4) == (s_1_6));
        // N s_1_8: branch s_1_7 b25 b2
        if s_1_7 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i64
        let s_2_0: i64 = 0;
        // D s_2_1: write-var ga#262509 <= s_2_0
        fn_state.ga_262509 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#262509:i64
        let s_3_0: i64 = fn_state.ga_262509;
        // D s_3_1: write-var shift <= s_3_0
        fn_state.shift = s_3_0;
        // D s_3_2: read-var Rn:u8
        let s_3_2: u8 = fn_state.Rn;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (s_3_3.value() as i128);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var n <= s_3_5
        fn_state.n = s_3_5;
        // D s_3_7: read-var Rt:u8
        let s_3_7: u8 = fn_state.Rt;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 5u16);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (s_3_8.value() as i128);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var t <= s_3_10
        fn_state.t = s_3_10;
        // D s_3_12: read-var Rm:u8
        let s_3_12: u8 = fn_state.Rm;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var m <= s_3_15
        fn_state.m = s_3_15;
        // C s_3_17: const #32s : i64
        let s_3_17: i64 = 32;
        // D s_3_18: write-var regsize <= s_3_17
        fn_state.regsize = s_3_17;
        // C s_3_19: const #1s : i
        let s_3_19: i128 = 1;
        // D s_3_20: read-var opc:u8
        let s_3_20: u8 = fn_state.opc;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 2u16);
        // C s_3_22: const #1u : u64
        let s_3_22: u64 = 1;
        // D s_3_23: bit-extract s_3_21 s_3_19 s_3_22
        let s_3_23: Bits = (Bits::new(
            ((s_3_21) >> (s_3_19)).value(),
            u16::try_from(s_3_22).unwrap(),
        ));
        // D s_3_24: cast reint s_3_23 -> u8
        let s_3_24: bool = ((s_3_23.value()) != 0);
        // C s_3_25: const #0s : i
        let s_3_25: i128 = 0;
        // C s_3_26: const #0u : u64
        let s_3_26: u64 = 0;
        // D s_3_27: cast zx s_3_24 -> u64
        let s_3_27: u64 = (s_3_24 as u64);
        // C s_3_28: const #1u : u64
        let s_3_28: u64 = 1;
        // D s_3_29: and s_3_27 s_3_28
        let s_3_29: u64 = ((s_3_27) & (s_3_28));
        // D s_3_30: cmp-eq s_3_29 s_3_28
        let s_3_30: bool = ((s_3_29) == (s_3_28));
        // D s_3_31: lsl s_3_27 s_3_25
        let s_3_31: u64 = s_3_27 << s_3_25;
        // D s_3_32: or s_3_26 s_3_31
        let s_3_32: u64 = ((s_3_26) | (s_3_31));
        // D s_3_33: cmpl s_3_31
        let s_3_33: u64 = !s_3_31;
        // D s_3_34: and s_3_26 s_3_33
        let s_3_34: u64 = ((s_3_26) & (s_3_33));
        // D s_3_35: select s_3_30 s_3_32 s_3_34
        let s_3_35: u64 = if s_3_30 { s_3_32 } else { s_3_34 };
        // D s_3_36: cast trunc s_3_35 -> u8
        let s_3_36: bool = ((s_3_35) != 0);
        // D s_3_37: cast zx s_3_36 -> bv
        let s_3_37: Bits = Bits::new(s_3_36 as u128, 1u16);
        // C s_3_38: const #0u : u8
        let s_3_38: bool = false;
        // C s_3_39: cast zx s_3_38 -> bv
        let s_3_39: Bits = Bits::new(s_3_38 as u128, 1u16);
        // D s_3_40: cmp-eq s_3_37 s_3_39
        let s_3_40: bool = ((s_3_37) == (s_3_39));
        // N s_3_41: branch s_3_40 b18 b4
        if s_3_40 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #3u : u8
        let s_4_2: u8 = 3;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b15 b5
        if s_4_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u32
        let s_5_0: u32 = 0;
        // D s_5_1: write-var memop <= s_5_0
        fn_state.memop = s_5_0;
        // D s_5_2: read-var size:u8
        let s_5_2: u8 = fn_state.size;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // C s_5_4: const #2u : u8
        let s_5_4: u8 = 2;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b14 b6
        if s_5_6 {
            return block_14(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#163853 <= s_6_0
        fn_state.gs_163853 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#163853:u8
        let s_7_0: bool = fn_state.gs_163853;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var opc:u8
        let s_8_1: u8 = fn_state.opc;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // N s_8_22: branch s_8_21 b12 b9
        if s_8_21 {
            return block_12(state, tracer, fn_state);
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
        // D s_9_1: write-var regsize <= s_9_0
        fn_state.regsize = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var is_signed <= s_10_0
        fn_state.is_signed = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var regsize:i64
        let s_11_0: i64 = fn_state.regsize;
        // C s_11_1: const #8s : i64
        let s_11_1: i64 = 8;
        // D s_11_2: read-var scale:i64
        let s_11_2: i64 = fn_state.scale;
        // D s_11_3: lsl s_11_1 s_11_2
        let s_11_3: i64 = s_11_1 << s_11_2;
        // D s_11_4: read-var memop:u32
        let s_11_4: u32 = fn_state.memop;
        // C s_11_5: const #2u : u32
        let s_11_5: u32 = 2;
        // D s_11_6: cmp-eq s_11_4 s_11_5
        let s_11_6: bool = ((s_11_4) == (s_11_5));
        // D s_11_7: cast zx s_11_3 -> i
        let s_11_7: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: cast zx s_11_0 -> i
        let s_11_9: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_10: cast reint s_11_9 -> i64
        let s_11_10: i64 = (s_11_9 as i64);
        // D s_11_11: read-var extend_type:u32
        let s_11_11: u32 = fn_state.extend_type;
        // D s_11_12: read-var m:i64
        let s_11_12: i64 = fn_state.m;
        // D s_11_13: read-var memop:u32
        let s_11_13: u32 = fn_state.memop;
        // D s_11_14: read-var n:i64
        let s_11_14: i64 = fn_state.n;
        // C s_11_15: const #0u : u8
        let s_11_15: bool = false;
        // C s_11_16: const #0u : u8
        let s_11_16: bool = false;
        // C s_11_17: const #0u : u8
        let s_11_17: bool = false;
        // D s_11_18: read-var shift:i64
        let s_11_18: i64 = fn_state.shift;
        // D s_11_19: read-var is_signed:u8
        let s_11_19: bool = fn_state.is_signed;
        // D s_11_20: read-var t:i64
        let s_11_20: i64 = fn_state.t;
        // C s_11_21: const #0u : u8
        let s_11_21: bool = false;
        // C s_11_22: const #0u : u8
        let s_11_22: bool = false;
        // D s_11_23: call execute_aarch64_instrs_memory_single_general_register(s_11_8, s_11_11, s_11_12, s_11_13, s_11_14, s_11_15, s_11_16, s_11_10, s_11_17, s_11_18, s_11_19, s_11_20, s_11_6, s_11_21, s_11_22)
        let s_11_23: () = execute_aarch64_instrs_memory_single_general_register(
            state,
            tracer,
            s_11_8,
            s_11_11,
            s_11_12,
            s_11_13,
            s_11_14,
            s_11_15,
            s_11_16,
            s_11_10,
            s_11_17,
            s_11_18,
            s_11_19,
            s_11_20,
            s_11_6,
            s_11_21,
            s_11_22,
        );
        // N s_11_24: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // D s_12_1: write-var regsize <= s_12_0
        fn_state.regsize = s_12_0;
        // N s_12_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var opc:u8
        let s_14_1: u8 = fn_state.opc;
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
        // D s_14_22: write-var gs#163853 <= s_14_21
        fn_state.gs_163853 = s_14_21;
        // N s_14_23: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: write-var memop <= s_15_0
        fn_state.memop = s_15_0;
        // C s_15_2: const #0s : i
        let s_15_2: i128 = 0;
        // D s_15_3: read-var opc:u8
        let s_15_3: u8 = fn_state.opc;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 2u16);
        // C s_15_5: const #1u : u64
        let s_15_5: u64 = 1;
        // D s_15_6: bit-extract s_15_4 s_15_2 s_15_5
        let s_15_6: Bits = (Bits::new(
            ((s_15_4) >> (s_15_2)).value(),
            u16::try_from(s_15_5).unwrap(),
        ));
        // D s_15_7: cast reint s_15_6 -> u8
        let s_15_7: bool = ((s_15_6.value()) != 0);
        // C s_15_8: const #0s : i
        let s_15_8: i128 = 0;
        // C s_15_9: const #0u : u64
        let s_15_9: u64 = 0;
        // D s_15_10: cast zx s_15_7 -> u64
        let s_15_10: u64 = (s_15_7 as u64);
        // C s_15_11: const #1u : u64
        let s_15_11: u64 = 1;
        // D s_15_12: and s_15_10 s_15_11
        let s_15_12: u64 = ((s_15_10) & (s_15_11));
        // D s_15_13: cmp-eq s_15_12 s_15_11
        let s_15_13: bool = ((s_15_12) == (s_15_11));
        // D s_15_14: lsl s_15_10 s_15_8
        let s_15_14: u64 = s_15_10 << s_15_8;
        // D s_15_15: or s_15_9 s_15_14
        let s_15_15: u64 = ((s_15_9) | (s_15_14));
        // D s_15_16: cmpl s_15_14
        let s_15_16: u64 = !s_15_14;
        // D s_15_17: and s_15_9 s_15_16
        let s_15_17: u64 = ((s_15_9) & (s_15_16));
        // D s_15_18: select s_15_13 s_15_15 s_15_17
        let s_15_18: u64 = if s_15_13 { s_15_15 } else { s_15_17 };
        // D s_15_19: cast trunc s_15_18 -> u8
        let s_15_19: bool = ((s_15_18) != 0);
        // D s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // C s_15_21: const #1u : u8
        let s_15_21: bool = true;
        // C s_15_22: cast zx s_15_21 -> bv
        let s_15_22: Bits = Bits::new(s_15_21 as u128, 1u16);
        // D s_15_23: cmp-eq s_15_20 s_15_22
        let s_15_23: bool = ((s_15_20) == (s_15_22));
        // N s_15_24: branch s_15_23 b17 b16
        if s_15_23 {
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
        // N s_16_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var opc:u8
        let s_18_1: u8 = fn_state.opc;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #1u : u8
        let s_18_19: bool = true;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-eq s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) == (s_18_20));
        // N s_18_22: branch s_18_21 b24 b19
        if s_18_21 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u32
        let s_19_0: u32 = 1;
        // D s_19_1: write-var memop <= s_19_0
        fn_state.memop = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
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
        // N s_20_5: branch s_20_4 b23 b21
        if s_20_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #32s : i64
        let s_21_0: i64 = 32;
        // D s_21_1: write-var regsize <= s_21_0
        fn_state.regsize = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var is_signed <= s_22_0
        fn_state.is_signed = s_22_0;
        // N s_22_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // D s_23_1: write-var regsize <= s_23_0
        fn_state.regsize = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u32
        let s_24_0: u32 = 0;
        // D s_24_1: write-var memop <= s_24_0
        fn_state.memop = s_24_0;
        // N s_24_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var scale:i64
        let s_25_0: i64 = fn_state.scale;
        // D s_25_1: write-var ga#262509 <= s_25_0
        fn_state.ga_262509 = s_25_0;
        // N s_25_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
