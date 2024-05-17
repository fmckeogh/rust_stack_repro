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
pub fn decode_stp_gen_aarch64_instrs_memory_pair_general_offset<T: Tracer>(
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
        t: i64,
        t2: i64,
        n: i64,
        memop: u32,
        gs_161543: bool,
        gs_161542: bool,
        c: u32,
        is_signed: bool,
        tagchecked: bool,
        gs_161539: bool,
        gs_161536: bool,
        offset: u64,
        gs_161523: bool,
        datasize: i64,
        rt_unknown: bool,
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
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rt:u8
        let s_0_5: u8 = fn_state.Rt;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var t <= s_0_8
        fn_state.t = s_0_8;
        // D s_0_10: read-var Rt2:u8
        let s_0_10: u8 = fn_state.Rt2;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var t2 <= s_0_13
        fn_state.t2 = s_0_13;
        // D s_0_15: read-var L:u8
        let s_0_15: bool = fn_state.L;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b30 b1
        if s_0_19 {
            return block_30(state, tracer, fn_state);
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
        // N s_2_34: branch s_2_33 b29 b3
        if s_2_33 {
            return block_29(state, tracer, fn_state);
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
        // D s_3_5: write-var gs#161523 <= s_3_4
        fn_state.gs_161523 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#161523:u8
        let s_4_0: bool = fn_state.gs_161523;
        // N s_4_1: branch s_4_0 b28 b5
        if s_4_0 {
            return block_28(state, tracer, fn_state);
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
        // C s_5_61: const #31s : i
        let s_5_61: i128 = 31;
        // D s_5_62: read-var n:i64
        let s_5_62: i64 = fn_state.n;
        // D s_5_63: cast zx s_5_62 -> i
        let s_5_63: i128 = (i128::try_from(s_5_62).unwrap());
        // D s_5_64: call neq_int(s_5_63, s_5_61)
        let s_5_64: bool = neq_int(state, tracer, s_5_63, s_5_61);
        // D s_5_65: write-var tagchecked <= s_5_64
        fn_state.tagchecked = s_5_64;
        // C s_5_66: const #0u : u8
        let s_5_66: bool = false;
        // D s_5_67: write-var rt_unknown <= s_5_66
        fn_state.rt_unknown = s_5_66;
        // D s_5_68: read-var memop:u32
        let s_5_68: u32 = fn_state.memop;
        // C s_5_69: const #0u : u32
        let s_5_69: u32 = 0;
        // D s_5_70: cmp-eq s_5_68 s_5_69
        let s_5_70: bool = ((s_5_68) == (s_5_69));
        // N s_5_71: branch s_5_70 b27 b6
        if s_5_70 {
            return block_27(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#161536 <= s_6_0
        fn_state.gs_161536 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#161536:u8
        let s_7_0: bool = fn_state.gs_161536;
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
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasize:i64
        let s_9_0: i64 = fn_state.datasize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #32s : i
        let s_9_4: i128 = 32;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasize:i64
        let s_10_0: i64 = fn_state.datasize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #64s : i
        let s_10_4: i128 = 64;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // D s_10_7: write-var gs#161539 <= s_10_6
        fn_state.gs_161539 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#161539:u8
        let s_11_0: bool = fn_state.gs_161539;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // D s_11_2: read-var datasize:i64
        let s_11_2: i64 = fn_state.datasize;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // D s_11_5: read-var memop:u32
        let s_11_5: u32 = fn_state.memop;
        // D s_11_6: read-var n:i64
        let s_11_6: i64 = fn_state.n;
        // C s_11_7: const #0u : u8
        let s_11_7: bool = false;
        // D s_11_8: read-var offset:u64
        let s_11_8: u64 = fn_state.offset;
        // C s_11_9: const #0u : u8
        let s_11_9: bool = false;
        // D s_11_10: read-var rt_unknown:u8
        let s_11_10: bool = fn_state.rt_unknown;
        // D s_11_11: read-var is_signed:u8
        let s_11_11: bool = fn_state.is_signed;
        // D s_11_12: read-var t:i64
        let s_11_12: i64 = fn_state.t;
        // D s_11_13: read-var t2:i64
        let s_11_13: i64 = fn_state.t2;
        // D s_11_14: read-var tagchecked:u8
        let s_11_14: bool = fn_state.tagchecked;
        // C s_11_15: const #0u : u8
        let s_11_15: bool = false;
        // C s_11_16: const #0u : u8
        let s_11_16: bool = false;
        // D s_11_17: call execute_aarch64_instrs_memory_pair_general_post_idx(s_11_4, s_11_5, s_11_6, s_11_7, s_11_8, s_11_9, s_11_10, s_11_11, s_11_12, s_11_13, s_11_14, s_11_15, s_11_16)
        let s_11_17: () = execute_aarch64_instrs_memory_pair_general_post_idx(
            state,
            tracer,
            s_11_4,
            s_11_5,
            s_11_6,
            s_11_7,
            s_11_8,
            s_11_9,
            s_11_10,
            s_11_11,
            s_11_12,
            s_11_13,
            s_11_14,
            s_11_15,
            s_11_16,
        );
        // N s_11_18: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#161539 <= s_12_0
        fn_state.gs_161539 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // S s_13_1: call ConstrainUnpredictable(s_13_0)
        let s_13_1: u32 = ConstrainUnpredictable(state, tracer, s_13_0);
        // D s_13_2: write-var c <= s_13_1
        fn_state.c = s_13_1;
        // D s_13_3: read-var c:u32
        let s_13_3: u32 = fn_state.c;
        // C s_13_4: const #1u : u32
        let s_13_4: u32 = 1;
        // D s_13_5: cmp-eq s_13_3 s_13_4
        let s_13_5: bool = ((s_13_3) == (s_13_4));
        // N s_13_6: branch s_13_5 b26 b14
        if s_13_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var c:u32
        let s_14_0: u32 = fn_state.c;
        // C s_14_1: const #2u : u32
        let s_14_1: u32 = 2;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b25 b15
        if s_14_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var c:u32
        let s_15_0: u32 = fn_state.c;
        // C s_15_1: const #4u : u32
        let s_15_1: u32 = 4;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: write-var gs#161542 <= s_15_2
        fn_state.gs_161542 = s_15_2;
        // N s_15_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#161542:u8
        let s_16_0: bool = fn_state.gs_161542;
        // D s_16_1: write-var gs#161543 <= s_16_0
        fn_state.gs_161543 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#161543:u8
        let s_17_0: bool = fn_state.gs_161543;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // C s_17_2: const #1u : u32
        let s_17_2: u32 = 1;
        // D s_17_3: read-var c:u32
        let s_17_3: u32 = fn_state.c;
        // D s_17_4: cmp-eq s_17_2 s_17_3
        let s_17_4: bool = ((s_17_2) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b20 b18
        if s_17_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var rt_unknown <= s_18_0
        fn_state.rt_unknown = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #2u : u32
        let s_20_0: u32 = 2;
        // D s_20_1: read-var c:u32
        let s_20_1: u32 = fn_state.c;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
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
        // C s_22_0: const #4u : u32
        let s_22_0: u32 = 4;
        // D s_22_1: read-var c:u32
        let s_22_1: u32 = fn_state.c;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EndOfInstruction(s_23_0)
        let s_23_1: () = EndOfInstruction(state, tracer, s_23_0);
        // N s_23_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#161542 <= s_25_0
        fn_state.gs_161542 = s_25_0;
        // N s_25_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#161543 <= s_26_0
        fn_state.gs_161543 = s_26_0;
        // N s_26_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var t:i64
        let s_27_0: i64 = fn_state.t;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: read-var t2:i64
        let s_27_2: i64 = fn_state.t2;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#161536 <= s_27_4
        fn_state.gs_161536 = s_27_4;
        // N s_27_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#161523 <= s_29_0
        fn_state.gs_161523 = s_29_0;
        // N s_29_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u32
        let s_30_0: u32 = 0;
        // D s_30_1: write-var memop <= s_30_0
        fn_state.memop = s_30_0;
        // N s_30_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
