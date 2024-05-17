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
use neq_int::*;
use u__id::*;
use execute_aarch64_instrs_memory_pair_general_post_idx::*;
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use common::*;
pub fn decode_stp_gen_aarch64_instrs_memory_pair_general_pre_idx<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rt2: u8,
    imm7: u8,
    L: bool,
    opc: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_2138: u32,
        t: i64,
        t2: i64,
        gs_161467: bool,
        gs_161477: bool,
        gs_161480: bool,
        gs_161474: bool,
        gs_161503: bool,
        n: i64,
        memop: u32,
        gs_161505: bool,
        gs_161472: bool,
        u_2139: u32,
        c: u32,
        gs_161504: bool,
        gs_161476: bool,
        is_signed: bool,
        gs_161469: bool,
        gs_161484: bool,
        gs_161493: bool,
        gs_161456: bool,
        gs_161492: bool,
        gs_161483: bool,
        gs_161468: bool,
        gs_161473: bool,
        offset: u64,
        wback: bool,
        datasize: i64,
        rt_unknown: bool,
        gs_161494: bool,
        wb_unknown: bool,
        gs_161471: bool,
        Rt: u8,
        Rn: u8,
        Rt2: u8,
        imm7: u8,
        L: bool,
        opc: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rt2,
        imm7,
        L,
        opc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: write-var wback <= s_0_0
        fn_state.wback = s_0_0;
        // D s_0_2: read-var Rn:u8
        let s_0_2: u8 = fn_state.Rn;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 5u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: write-var n <= s_0_5
        fn_state.n = s_0_5;
        // D s_0_7: read-var Rt:u8
        let s_0_7: u8 = fn_state.Rt;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 5u16);
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (s_0_8.value() as i128);
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var t <= s_0_10
        fn_state.t = s_0_10;
        // D s_0_12: read-var Rt2:u8
        let s_0_12: u8 = fn_state.Rt2;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 5u16);
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (s_0_13.value() as i128);
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: write-var t2 <= s_0_15
        fn_state.t2 = s_0_15;
        // D s_0_17: read-var L:u8
        let s_0_17: bool = fn_state.L;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // C s_0_19: const #1u : u8
        let s_0_19: bool = true;
        // C s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_21: cmp-eq s_0_18 s_0_20
        let s_0_21: bool = ((s_0_18) == (s_0_20));
        // N s_0_22: branch s_0_21 b96 b1
        if s_0_21 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var memop <= s_1_0
        fn_state.memop = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var opc:u8
        let s_2_1: u8 = fn_state.opc;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: read-var L:u8
        let s_2_18: bool = fn_state.L;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // D s_2_20: cast zx s_2_17 -> bv
        let s_2_20: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_21: cast reint s_2_19 -> u128
        let s_2_21: u128 = (s_2_19.value() as u128);
        // D s_2_22: size-of s_2_19
        let s_2_22: u16 = s_2_19.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 2u16);
        // C s_2_31: const #1u : u8
        let s_2_31: u8 = 1;
        // C s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 2u16);
        // D s_2_33: cmp-eq s_2_30 s_2_32
        let s_2_33: bool = ((s_2_30) == (s_2_32));
        // N s_2_34: branch s_2_33 b95 b3
        if s_2_33 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var opc:u8
        let s_3_0: u8 = fn_state.opc;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #3u : u8
        let s_3_2: u8 = 3;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var gs#161456 <= s_3_4
        fn_state.gs_161456 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#161456:u8
        let s_4_0: bool = fn_state.gs_161456;
        // N s_4_1: branch s_4_0 b94 b5
        if s_4_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var opc:u8
        let s_5_1: u8 = fn_state.opc;
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
        // D s_5_21: cmp-ne s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) != (s_5_20));
        // D s_5_22: write-var is_signed <= s_5_21
        fn_state.is_signed = s_5_21;
        // C s_5_23: const #1s : i
        let s_5_23: i128 = 1;
        // D s_5_24: read-var opc:u8
        let s_5_24: u8 = fn_state.opc;
        // D s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 2u16);
        // C s_5_26: const #1u : u64
        let s_5_26: u64 = 1;
        // D s_5_27: bit-extract s_5_25 s_5_23 s_5_26
        let s_5_27: Bits = (Bits::new(
            ((s_5_25) >> (s_5_23)).value(),
            u16::try_from(s_5_26).unwrap(),
        ));
        // D s_5_28: cast reint s_5_27 -> u8
        let s_5_28: bool = ((s_5_27.value()) != 0);
        // C s_5_29: const #0s : i
        let s_5_29: i128 = 0;
        // C s_5_30: const #0u : u64
        let s_5_30: u64 = 0;
        // D s_5_31: cast zx s_5_28 -> u64
        let s_5_31: u64 = (s_5_28 as u64);
        // C s_5_32: const #1u : u64
        let s_5_32: u64 = 1;
        // D s_5_33: and s_5_31 s_5_32
        let s_5_33: u64 = ((s_5_31) & (s_5_32));
        // D s_5_34: cmp-eq s_5_33 s_5_32
        let s_5_34: bool = ((s_5_33) == (s_5_32));
        // D s_5_35: lsl s_5_31 s_5_29
        let s_5_35: u64 = s_5_31 << s_5_29;
        // D s_5_36: or s_5_30 s_5_35
        let s_5_36: u64 = ((s_5_30) | (s_5_35));
        // D s_5_37: cmpl s_5_35
        let s_5_37: u64 = !s_5_35;
        // D s_5_38: and s_5_30 s_5_37
        let s_5_38: u64 = ((s_5_30) & (s_5_37));
        // D s_5_39: select s_5_34 s_5_36 s_5_38
        let s_5_39: u64 = if s_5_34 { s_5_36 } else { s_5_38 };
        // D s_5_40: cast trunc s_5_39 -> u8
        let s_5_40: bool = ((s_5_39) != 0);
        // D s_5_41: cast zx s_5_40 -> bv
        let s_5_41: Bits = Bits::new(s_5_40 as u128, 1u16);
        // D s_5_42: cast zx s_5_41 -> i
        let s_5_42: i128 = (s_5_41.value() as i128);
        // D s_5_43: cast reint s_5_42 -> i64
        let s_5_43: i64 = (s_5_42 as i64);
        // C s_5_44: const #2s : i
        let s_5_44: i128 = 2;
        // D s_5_45: cast zx s_5_43 -> i
        let s_5_45: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_46: add s_5_44 s_5_45
        let s_5_46: i128 = (s_5_44 + s_5_45);
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // C s_5_48: const #8s : i64
        let s_5_48: i64 = 8;
        // D s_5_49: lsl s_5_48 s_5_47
        let s_5_49: i64 = s_5_48 << s_5_47;
        // D s_5_50: write-var datasize <= s_5_49
        fn_state.datasize = s_5_49;
        // C s_5_51: const #64s : i
        let s_5_51: i128 = 64;
        // D s_5_52: read-var imm7:u8
        let s_5_52: u8 = fn_state.imm7;
        // D s_5_53: cast zx s_5_52 -> bv
        let s_5_53: Bits = Bits::new(s_5_52 as u128, 7u16);
        // D s_5_54: bits-cast sx s_5_53 -> bv length s_5_51
        let s_5_54: Bits = s_5_53.sign_extend(s_5_51);
        // D s_5_55: cast reint s_5_54 -> u64
        let s_5_55: u64 = (s_5_54.value() as u64);
        // D s_5_56: cast zx s_5_55 -> bv
        let s_5_56: Bits = Bits::new(s_5_55 as u128, 64u16);
        // D s_5_57: cast zx s_5_47 -> i
        let s_5_57: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_58: lsl s_5_56 s_5_57
        let s_5_58: Bits = s_5_56 << s_5_57;
        // D s_5_59: cast reint s_5_58 -> u64
        let s_5_59: u64 = (s_5_58.value() as u64);
        // D s_5_60: write-var offset <= s_5_59
        fn_state.offset = s_5_59;
        // C s_5_61: const #0u : u8
        let s_5_61: bool = false;
        // D s_5_62: write-var rt_unknown <= s_5_61
        fn_state.rt_unknown = s_5_61;
        // C s_5_63: const #0u : u8
        let s_5_63: bool = false;
        // D s_5_64: write-var wb_unknown <= s_5_63
        fn_state.wb_unknown = s_5_63;
        // D s_5_65: read-var memop:u32
        let s_5_65: u32 = fn_state.memop;
        // C s_5_66: const #0u : u32
        let s_5_66: u32 = 0;
        // D s_5_67: cmp-eq s_5_65 s_5_66
        let s_5_67: bool = ((s_5_65) == (s_5_66));
        // N s_5_68: branch s_5_67 b93 b6
        if s_5_67 {
            return block_93(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#161467 <= s_6_0
        fn_state.gs_161467 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#161467:u8
        let s_7_0: bool = fn_state.gs_161467;
        // N s_7_1: branch s_7_0 b89 b8
        if s_7_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#161469 <= s_8_0
        fn_state.gs_161469 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#161469:u8
        let s_9_0: bool = fn_state.gs_161469;
        // N s_9_1: branch s_9_0 b88 b10
        if s_9_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#161471 <= s_10_0
        fn_state.gs_161471 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#161471:u8
        let s_11_0: bool = fn_state.gs_161471;
        // N s_11_1: branch s_11_0 b69 b12
        if s_11_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var memop:u32
        let s_13_0: u32 = fn_state.memop;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b68 b14
        if s_13_2 {
            return block_68(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#161472 <= s_14_0
        fn_state.gs_161472 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#161472:u8
        let s_15_0: bool = fn_state.gs_161472;
        // N s_15_1: branch s_15_0 b64 b16
        if s_15_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#161474 <= s_16_0
        fn_state.gs_161474 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#161474:u8
        let s_17_0: bool = fn_state.gs_161474;
        // N s_17_1: branch s_17_0 b63 b18
        if s_17_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#161476 <= s_18_0
        fn_state.gs_161476 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#161476:u8
        let s_19_0: bool = fn_state.gs_161476;
        // N s_19_1: branch s_19_0 b44 b20
        if s_19_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var memop:u32
        let s_21_0: u32 = fn_state.memop;
        // C s_21_1: const #0u : u32
        let s_21_1: u32 = 0;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b43 b22
        if s_21_2 {
            return block_43(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#161477 <= s_22_0
        fn_state.gs_161477 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#161477:u8
        let s_23_0: bool = fn_state.gs_161477;
        // N s_23_1: branch s_23_0 b29 b24
        if s_23_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var datasize:i64
        let s_25_0: i64 = fn_state.datasize;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #32s : i
        let s_25_4: i128 = 32;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
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
        // D s_26_0: read-var datasize:i64
        let s_26_0: i64 = fn_state.datasize;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call __id(s_26_1)
        let s_26_2: i128 = u__id(state, tracer, s_26_1);
        // D s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: const #64s : i
        let s_26_4: i128 = 64;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-eq s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) == (s_26_4));
        // D s_26_7: write-var gs#161480 <= s_26_6
        fn_state.gs_161480 = s_26_6;
        // N s_26_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#161480:u8
        let s_27_0: bool = fn_state.gs_161480;
        // N s_27_1: assert s_27_0
        let s_27_1: () = assert!(s_27_0);
        // D s_27_2: read-var datasize:i64
        let s_27_2: i64 = fn_state.datasize;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: cast reint s_27_3 -> i64
        let s_27_4: i64 = (s_27_3 as i64);
        // D s_27_5: read-var memop:u32
        let s_27_5: u32 = fn_state.memop;
        // D s_27_6: read-var n:i64
        let s_27_6: i64 = fn_state.n;
        // C s_27_7: const #0u : u8
        let s_27_7: bool = false;
        // D s_27_8: read-var offset:u64
        let s_27_8: u64 = fn_state.offset;
        // C s_27_9: const #0u : u8
        let s_27_9: bool = false;
        // D s_27_10: read-var rt_unknown:u8
        let s_27_10: bool = fn_state.rt_unknown;
        // D s_27_11: read-var is_signed:u8
        let s_27_11: bool = fn_state.is_signed;
        // D s_27_12: read-var t:i64
        let s_27_12: i64 = fn_state.t;
        // D s_27_13: read-var t2:i64
        let s_27_13: i64 = fn_state.t2;
        // C s_27_14: const #1u : u8
        let s_27_14: bool = true;
        // D s_27_15: read-var wb_unknown:u8
        let s_27_15: bool = fn_state.wb_unknown;
        // D s_27_16: read-var wback:u8
        let s_27_16: bool = fn_state.wback;
        // D s_27_17: call execute_aarch64_instrs_memory_pair_general_post_idx(s_27_4, s_27_5, s_27_6, s_27_7, s_27_8, s_27_9, s_27_10, s_27_11, s_27_12, s_27_13, s_27_14, s_27_15, s_27_16)
        let s_27_17: () = execute_aarch64_instrs_memory_pair_general_post_idx(
            state,
            tracer,
            s_27_4,
            s_27_5,
            s_27_6,
            s_27_7,
            s_27_8,
            s_27_9,
            s_27_10,
            s_27_11,
            s_27_12,
            s_27_13,
            s_27_14,
            s_27_15,
            s_27_16,
        );
        // N s_27_18: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#161480 <= s_28_0
        fn_state.gs_161480 = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #3u : u32
        let s_29_0: u32 = 3;
        // S s_29_1: call ConstrainUnpredictable(s_29_0)
        let s_29_1: u32 = ConstrainUnpredictable(state, tracer, s_29_0);
        // D s_29_2: write-var u#2139 <= s_29_1
        fn_state.u_2139 = s_29_1;
        // D s_29_3: read-var u#2139:u32
        let s_29_3: u32 = fn_state.u_2139;
        // C s_29_4: const #1u : u32
        let s_29_4: u32 = 1;
        // D s_29_5: cmp-eq s_29_3 s_29_4
        let s_29_5: bool = ((s_29_3) == (s_29_4));
        // N s_29_6: branch s_29_5 b42 b30
        if s_29_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var u#2139:u32
        let s_30_0: u32 = fn_state.u_2139;
        // C s_30_1: const #2u : u32
        let s_30_1: u32 = 2;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // N s_30_3: branch s_30_2 b41 b31
        if s_30_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var u#2139:u32
        let s_31_0: u32 = fn_state.u_2139;
        // C s_31_1: const #4u : u32
        let s_31_1: u32 = 4;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: write-var gs#161483 <= s_31_2
        fn_state.gs_161483 = s_31_2;
        // N s_31_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#161483:u8
        let s_32_0: bool = fn_state.gs_161483;
        // D s_32_1: write-var gs#161484 <= s_32_0
        fn_state.gs_161484 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#161484:u8
        let s_33_0: bool = fn_state.gs_161484;
        // N s_33_1: assert s_33_0
        let s_33_1: () = assert!(s_33_0);
        // C s_33_2: const #1u : u32
        let s_33_2: u32 = 1;
        // D s_33_3: read-var u#2139:u32
        let s_33_3: u32 = fn_state.u_2139;
        // D s_33_4: cmp-eq s_33_2 s_33_3
        let s_33_4: bool = ((s_33_2) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b36 b34
        if s_33_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var rt_unknown <= s_34_0
        fn_state.rt_unknown = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #2u : u32
        let s_36_0: u32 = 2;
        // D s_36_1: read-var u#2139:u32
        let s_36_1: u32 = fn_state.u_2139;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // N s_36_4: branch s_36_3 b38 b37
        if s_36_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #4u : u32
        let s_38_0: u32 = 4;
        // D s_38_1: read-var u#2139:u32
        let s_38_1: u32 = fn_state.u_2139;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b40 b39
        if s_38_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EndOfInstruction(s_39_0)
        let s_39_1: () = EndOfInstruction(state, tracer, s_39_0);
        // N s_39_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#161483 <= s_41_0
        fn_state.gs_161483 = s_41_0;
        // N s_41_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#161484 <= s_42_0
        fn_state.gs_161484 = s_42_0;
        // N s_42_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var t:i64
        let s_43_0: i64 = fn_state.t;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: read-var t2:i64
        let s_43_2: i64 = fn_state.t2;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#161477 <= s_43_4
        fn_state.gs_161477 = s_43_4;
        // N s_43_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #2u : u32
        let s_44_0: u32 = 2;
        // S s_44_1: call ConstrainUnpredictable(s_44_0)
        let s_44_1: u32 = ConstrainUnpredictable(state, tracer, s_44_0);
        // D s_44_2: write-var u#2138 <= s_44_1
        fn_state.u_2138 = s_44_1;
        // D s_44_3: read-var u#2138:u32
        let s_44_3: u32 = fn_state.u_2138;
        // C s_44_4: const #0u : u32
        let s_44_4: u32 = 0;
        // D s_44_5: cmp-eq s_44_3 s_44_4
        let s_44_5: bool = ((s_44_3) == (s_44_4));
        // N s_44_6: branch s_44_5 b62 b45
        if s_44_5 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var u#2138:u32
        let s_45_0: u32 = fn_state.u_2138;
        // C s_45_1: const #1u : u32
        let s_45_1: u32 = 1;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // N s_45_3: branch s_45_2 b61 b46
        if s_45_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var u#2138:u32
        let s_46_0: u32 = fn_state.u_2138;
        // C s_46_1: const #2u : u32
        let s_46_1: u32 = 2;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // N s_46_3: branch s_46_2 b60 b47
        if s_46_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var u#2138:u32
        let s_47_0: u32 = fn_state.u_2138;
        // C s_47_1: const #4u : u32
        let s_47_1: u32 = 4;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: write-var gs#161492 <= s_47_2
        fn_state.gs_161492 = s_47_2;
        // N s_47_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#161492:u8
        let s_48_0: bool = fn_state.gs_161492;
        // D s_48_1: write-var gs#161493 <= s_48_0
        fn_state.gs_161493 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#161493:u8
        let s_49_0: bool = fn_state.gs_161493;
        // D s_49_1: write-var gs#161494 <= s_49_0
        fn_state.gs_161494 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#161494:u8
        let s_50_0: bool = fn_state.gs_161494;
        // N s_50_1: assert s_50_0
        let s_50_1: () = assert!(s_50_0);
        // C s_50_2: const #0u : u32
        let s_50_2: u32 = 0;
        // D s_50_3: read-var u#2138:u32
        let s_50_3: u32 = fn_state.u_2138;
        // D s_50_4: cmp-eq s_50_2 s_50_3
        let s_50_4: bool = ((s_50_2) == (s_50_3));
        // D s_50_5: not s_50_4
        let s_50_5: bool = !s_50_4;
        // N s_50_6: branch s_50_5 b53 b51
        if s_50_5 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var rt_unknown <= s_51_0
        fn_state.rt_unknown = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u32
        let s_53_0: u32 = 1;
        // D s_53_1: read-var u#2138:u32
        let s_53_1: u32 = fn_state.u_2138;
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // N s_53_4: branch s_53_3 b55 b54
        if s_53_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var rt_unknown <= s_54_0
        fn_state.rt_unknown = s_54_0;
        // N s_54_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #2u : u32
        let s_55_0: u32 = 2;
        // D s_55_1: read-var u#2138:u32
        let s_55_1: u32 = fn_state.u_2138;
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // N s_55_4: branch s_55_3 b57 b56
        if s_55_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #4u : u32
        let s_57_0: u32 = 4;
        // D s_57_1: read-var u#2138:u32
        let s_57_1: u32 = fn_state.u_2138;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // N s_57_4: branch s_57_3 b59 b58
        if s_57_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EndOfInstruction(s_58_0)
        let s_58_1: () = EndOfInstruction(state, tracer, s_58_0);
        // N s_58_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#161492 <= s_60_0
        fn_state.gs_161492 = s_60_0;
        // N s_60_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#161493 <= s_61_0
        fn_state.gs_161493 = s_61_0;
        // N s_61_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#161494 <= s_62_0
        fn_state.gs_161494 = s_62_0;
        // N s_62_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #31s : i
        let s_63_0: i128 = 31;
        // D s_63_1: read-var n:i64
        let s_63_1: i64 = fn_state.n;
        // D s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (i128::try_from(s_63_1).unwrap());
        // D s_63_3: call neq_int(s_63_2, s_63_0)
        let s_63_3: bool = neq_int(state, tracer, s_63_2, s_63_0);
        // D s_63_4: write-var gs#161476 <= s_63_3
        fn_state.gs_161476 = s_63_3;
        // N s_63_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var t:i64
        let s_64_0: i64 = fn_state.t;
        // D s_64_1: cast zx s_64_0 -> i
        let s_64_1: i128 = (i128::try_from(s_64_0).unwrap());
        // D s_64_2: read-var n:i64
        let s_64_2: i64 = fn_state.n;
        // D s_64_3: cast zx s_64_2 -> i
        let s_64_3: i128 = (i128::try_from(s_64_2).unwrap());
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // N s_64_5: branch s_64_4 b67 b65
        if s_64_4 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var t2:i64
        let s_65_0: i64 = fn_state.t2;
        // D s_65_1: cast zx s_65_0 -> i
        let s_65_1: i128 = (i128::try_from(s_65_0).unwrap());
        // D s_65_2: read-var n:i64
        let s_65_2: i64 = fn_state.n;
        // D s_65_3: cast zx s_65_2 -> i
        let s_65_3: i128 = (i128::try_from(s_65_2).unwrap());
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#161473 <= s_65_4
        fn_state.gs_161473 = s_65_4;
        // N s_65_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#161473:u8
        let s_66_0: bool = fn_state.gs_161473;
        // D s_66_1: write-var gs#161474 <= s_66_0
        fn_state.gs_161474 = s_66_0;
        // N s_66_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#161473 <= s_67_0
        fn_state.gs_161473 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var wback:u8
        let s_68_0: bool = fn_state.wback;
        // D s_68_1: write-var gs#161472 <= s_68_0
        fn_state.gs_161472 = s_68_0;
        // N s_68_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u32
        let s_69_0: u32 = 1;
        // S s_69_1: call ConstrainUnpredictable(s_69_0)
        let s_69_1: u32 = ConstrainUnpredictable(state, tracer, s_69_0);
        // D s_69_2: write-var c <= s_69_1
        fn_state.c = s_69_1;
        // D s_69_3: read-var c:u32
        let s_69_3: u32 = fn_state.c;
        // C s_69_4: const #11u : u32
        let s_69_4: u32 = 11;
        // D s_69_5: cmp-eq s_69_3 s_69_4
        let s_69_5: bool = ((s_69_3) == (s_69_4));
        // N s_69_6: branch s_69_5 b87 b70
        if s_69_5 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var c:u32
        let s_70_0: u32 = fn_state.c;
        // C s_70_1: const #1u : u32
        let s_70_1: u32 = 1;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // N s_70_3: branch s_70_2 b86 b71
        if s_70_2 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var c:u32
        let s_71_0: u32 = fn_state.c;
        // C s_71_1: const #2u : u32
        let s_71_1: u32 = 2;
        // D s_71_2: cmp-eq s_71_0 s_71_1
        let s_71_2: bool = ((s_71_0) == (s_71_1));
        // N s_71_3: branch s_71_2 b85 b72
        if s_71_2 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var c:u32
        let s_72_0: u32 = fn_state.c;
        // C s_72_1: const #4u : u32
        let s_72_1: u32 = 4;
        // D s_72_2: cmp-eq s_72_0 s_72_1
        let s_72_2: bool = ((s_72_0) == (s_72_1));
        // D s_72_3: write-var gs#161503 <= s_72_2
        fn_state.gs_161503 = s_72_2;
        // N s_72_4: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#161503:u8
        let s_73_0: bool = fn_state.gs_161503;
        // D s_73_1: write-var gs#161504 <= s_73_0
        fn_state.gs_161504 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#161504:u8
        let s_74_0: bool = fn_state.gs_161504;
        // D s_74_1: write-var gs#161505 <= s_74_0
        fn_state.gs_161505 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#161505:u8
        let s_75_0: bool = fn_state.gs_161505;
        // N s_75_1: assert s_75_0
        let s_75_1: () = assert!(s_75_0);
        // C s_75_2: const #11u : u32
        let s_75_2: u32 = 11;
        // D s_75_3: read-var c:u32
        let s_75_3: u32 = fn_state.c;
        // D s_75_4: cmp-eq s_75_2 s_75_3
        let s_75_4: bool = ((s_75_2) == (s_75_3));
        // D s_75_5: not s_75_4
        let s_75_5: bool = !s_75_4;
        // N s_75_6: branch s_75_5 b78 b76
        if s_75_5 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var wback <= s_76_0
        fn_state.wback = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1u : u32
        let s_78_0: u32 = 1;
        // D s_78_1: read-var c:u32
        let s_78_1: u32 = fn_state.c;
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // N s_78_4: branch s_78_3 b80 b79
        if s_78_3 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var wb_unknown <= s_79_0
        fn_state.wb_unknown = s_79_0;
        // N s_79_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #2u : u32
        let s_80_0: u32 = 2;
        // D s_80_1: read-var c:u32
        let s_80_1: u32 = fn_state.c;
        // D s_80_2: cmp-eq s_80_0 s_80_1
        let s_80_2: bool = ((s_80_0) == (s_80_1));
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // N s_80_4: branch s_80_3 b82 b81
        if s_80_3 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #4u : u32
        let s_82_0: u32 = 4;
        // D s_82_1: read-var c:u32
        let s_82_1: u32 = fn_state.c;
        // D s_82_2: cmp-eq s_82_0 s_82_1
        let s_82_2: bool = ((s_82_0) == (s_82_1));
        // D s_82_3: not s_82_2
        let s_82_3: bool = !s_82_2;
        // N s_82_4: branch s_82_3 b84 b83
        if s_82_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EndOfInstruction(s_83_0)
        let s_83_1: () = EndOfInstruction(state, tracer, s_83_0);
        // N s_83_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: write-var gs#161503 <= s_85_0
        fn_state.gs_161503 = s_85_0;
        // N s_85_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#161504 <= s_86_0
        fn_state.gs_161504 = s_86_0;
        // N s_86_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // D s_87_1: write-var gs#161505 <= s_87_0
        fn_state.gs_161505 = s_87_0;
        // N s_87_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #31s : i
        let s_88_0: i128 = 31;
        // D s_88_1: read-var n:i64
        let s_88_1: i64 = fn_state.n;
        // D s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (i128::try_from(s_88_1).unwrap());
        // D s_88_3: call neq_int(s_88_2, s_88_0)
        let s_88_3: bool = neq_int(state, tracer, s_88_2, s_88_0);
        // D s_88_4: write-var gs#161471 <= s_88_3
        fn_state.gs_161471 = s_88_3;
        // N s_88_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var t:i64
        let s_89_0: i64 = fn_state.t;
        // D s_89_1: cast zx s_89_0 -> i
        let s_89_1: i128 = (i128::try_from(s_89_0).unwrap());
        // D s_89_2: read-var n:i64
        let s_89_2: i64 = fn_state.n;
        // D s_89_3: cast zx s_89_2 -> i
        let s_89_3: i128 = (i128::try_from(s_89_2).unwrap());
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // N s_89_5: branch s_89_4 b92 b90
        if s_89_4 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var t2:i64
        let s_90_0: i64 = fn_state.t2;
        // D s_90_1: cast zx s_90_0 -> i
        let s_90_1: i128 = (i128::try_from(s_90_0).unwrap());
        // D s_90_2: read-var n:i64
        let s_90_2: i64 = fn_state.n;
        // D s_90_3: cast zx s_90_2 -> i
        let s_90_3: i128 = (i128::try_from(s_90_2).unwrap());
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#161468 <= s_90_4
        fn_state.gs_161468 = s_90_4;
        // N s_90_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#161468:u8
        let s_91_0: bool = fn_state.gs_161468;
        // D s_91_1: write-var gs#161469 <= s_91_0
        fn_state.gs_161469 = s_91_0;
        // N s_91_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#161468 <= s_92_0
        fn_state.gs_161468 = s_92_0;
        // N s_92_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#161467 <= s_93_0
        fn_state.gs_161467 = s_93_0;
        // N s_93_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: panic
        panic!("{:?}", ());
        // N s_94_1: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#161456 <= s_95_0
        fn_state.gs_161456 = s_95_0;
        // N s_95_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u32
        let s_96_0: u32 = 0;
        // D s_96_1: write-var memop <= s_96_0
        fn_state.memop = s_96_0;
        // N s_96_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
