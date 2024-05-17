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
use ConstrainUnpredictable::*;
use EndOfInstruction::*;
use execute_aarch64_instrs_memory_ordered_pair_ldiapp::*;
use common::*;
pub fn decode_ldiapp_aarch64_instrs_memory_ordered_pair_ldiapp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    opc2: u8,
    Rt2: u8,
    L: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        t2: i64,
        gs_160494: bool,
        gs_160475: bool,
        n: i64,
        memop: u32,
        gs_160479: bool,
        gs_160496: bool,
        gs_160471: bool,
        gs_160485: bool,
        gs_160476: bool,
        gs_160468: bool,
        c: u32,
        gs_160482: bool,
        scale: i64,
        gs_160474: bool,
        postindex: bool,
        tagchecked: bool,
        gs_160469: bool,
        offsetshadow_1640: i128,
        gs_160473: bool,
        gs_160486: bool,
        gs_160505: bool,
        u_2126: u32,
        offset: i128,
        u_2125: u32,
        wback: bool,
        gs_160470: bool,
        datasize: i64,
        rt_unknown: bool,
        gs_160506: bool,
        gs_160507: bool,
        gs_160495: bool,
        wb_unknown: bool,
        gs_160478: bool,
        Rt: u8,
        Rn: u8,
        opc2: u8,
        Rt2: u8,
        L: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        opc2,
        Rt2,
        L,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var opc2:u8
        let s_0_1: u8 = fn_state.opc2;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 4u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // C s_0_19: const #0u : u8
        let s_0_19: bool = false;
        // C s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_21: cmp-eq s_0_18 s_0_20
        let s_0_21: bool = ((s_0_18) == (s_0_20));
        // D s_0_22: write-var postindex <= s_0_21
        fn_state.postindex = s_0_21;
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // D s_0_24: read-var opc2:u8
        let s_0_24: u8 = fn_state.opc2;
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 4u16);
        // C s_0_26: const #1u : u64
        let s_0_26: u64 = 1;
        // D s_0_27: bit-extract s_0_25 s_0_23 s_0_26
        let s_0_27: Bits = (Bits::new(
            ((s_0_25) >> (s_0_23)).value(),
            u16::try_from(s_0_26).unwrap(),
        ));
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: bool = ((s_0_27.value()) != 0);
        // C s_0_29: const #0s : i
        let s_0_29: i128 = 0;
        // C s_0_30: const #0u : u64
        let s_0_30: u64 = 0;
        // D s_0_31: cast zx s_0_28 -> u64
        let s_0_31: u64 = (s_0_28 as u64);
        // C s_0_32: const #1u : u64
        let s_0_32: u64 = 1;
        // D s_0_33: and s_0_31 s_0_32
        let s_0_33: u64 = ((s_0_31) & (s_0_32));
        // D s_0_34: cmp-eq s_0_33 s_0_32
        let s_0_34: bool = ((s_0_33) == (s_0_32));
        // D s_0_35: lsl s_0_31 s_0_29
        let s_0_35: u64 = s_0_31 << s_0_29;
        // D s_0_36: or s_0_30 s_0_35
        let s_0_36: u64 = ((s_0_30) | (s_0_35));
        // D s_0_37: cmpl s_0_35
        let s_0_37: u64 = !s_0_35;
        // D s_0_38: and s_0_30 s_0_37
        let s_0_38: u64 = ((s_0_30) & (s_0_37));
        // D s_0_39: select s_0_34 s_0_36 s_0_38
        let s_0_39: u64 = if s_0_34 { s_0_36 } else { s_0_38 };
        // D s_0_40: cast trunc s_0_39 -> u8
        let s_0_40: bool = ((s_0_39) != 0);
        // D s_0_41: cast zx s_0_40 -> bv
        let s_0_41: Bits = Bits::new(s_0_40 as u128, 1u16);
        // C s_0_42: const #0u : u8
        let s_0_42: bool = false;
        // C s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 1u16);
        // D s_0_44: cmp-eq s_0_41 s_0_43
        let s_0_44: bool = ((s_0_41) == (s_0_43));
        // D s_0_45: write-var wback <= s_0_44
        fn_state.wback = s_0_44;
        // D s_0_46: read-var Rn:u8
        let s_0_46: u8 = fn_state.Rn;
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 5u16);
        // D s_0_48: cast zx s_0_47 -> i
        let s_0_48: i128 = (s_0_47.value() as i128);
        // D s_0_49: cast reint s_0_48 -> i64
        let s_0_49: i64 = (s_0_48 as i64);
        // D s_0_50: write-var n <= s_0_49
        fn_state.n = s_0_49;
        // D s_0_51: read-var Rt:u8
        let s_0_51: u8 = fn_state.Rt;
        // D s_0_52: cast zx s_0_51 -> bv
        let s_0_52: Bits = Bits::new(s_0_51 as u128, 5u16);
        // D s_0_53: cast zx s_0_52 -> i
        let s_0_53: i128 = (s_0_52.value() as i128);
        // D s_0_54: cast reint s_0_53 -> i64
        let s_0_54: i64 = (s_0_53 as i64);
        // D s_0_55: write-var t <= s_0_54
        fn_state.t = s_0_54;
        // D s_0_56: read-var Rt2:u8
        let s_0_56: u8 = fn_state.Rt2;
        // D s_0_57: cast zx s_0_56 -> bv
        let s_0_57: Bits = Bits::new(s_0_56 as u128, 5u16);
        // D s_0_58: cast zx s_0_57 -> i
        let s_0_58: i128 = (s_0_57.value() as i128);
        // D s_0_59: cast reint s_0_58 -> i64
        let s_0_59: i64 = (s_0_58 as i64);
        // D s_0_60: write-var t2 <= s_0_59
        fn_state.t2 = s_0_59;
        // C s_0_61: const #0s : i
        let s_0_61: i128 = 0;
        // D s_0_62: read-var size:u8
        let s_0_62: u8 = fn_state.size;
        // D s_0_63: cast zx s_0_62 -> bv
        let s_0_63: Bits = Bits::new(s_0_62 as u128, 2u16);
        // C s_0_64: const #1u : u64
        let s_0_64: u64 = 1;
        // D s_0_65: bit-extract s_0_63 s_0_61 s_0_64
        let s_0_65: Bits = (Bits::new(
            ((s_0_63) >> (s_0_61)).value(),
            u16::try_from(s_0_64).unwrap(),
        ));
        // D s_0_66: cast reint s_0_65 -> u8
        let s_0_66: bool = ((s_0_65.value()) != 0);
        // C s_0_67: const #0s : i
        let s_0_67: i128 = 0;
        // C s_0_68: const #0u : u64
        let s_0_68: u64 = 0;
        // D s_0_69: cast zx s_0_66 -> u64
        let s_0_69: u64 = (s_0_66 as u64);
        // C s_0_70: const #1u : u64
        let s_0_70: u64 = 1;
        // D s_0_71: and s_0_69 s_0_70
        let s_0_71: u64 = ((s_0_69) & (s_0_70));
        // D s_0_72: cmp-eq s_0_71 s_0_70
        let s_0_72: bool = ((s_0_71) == (s_0_70));
        // D s_0_73: lsl s_0_69 s_0_67
        let s_0_73: u64 = s_0_69 << s_0_67;
        // D s_0_74: or s_0_68 s_0_73
        let s_0_74: u64 = ((s_0_68) | (s_0_73));
        // D s_0_75: cmpl s_0_73
        let s_0_75: u64 = !s_0_73;
        // D s_0_76: and s_0_68 s_0_75
        let s_0_76: u64 = ((s_0_68) & (s_0_75));
        // D s_0_77: select s_0_72 s_0_74 s_0_76
        let s_0_77: u64 = if s_0_72 { s_0_74 } else { s_0_76 };
        // D s_0_78: cast trunc s_0_77 -> u8
        let s_0_78: bool = ((s_0_77) != 0);
        // D s_0_79: cast zx s_0_78 -> bv
        let s_0_79: Bits = Bits::new(s_0_78 as u128, 1u16);
        // D s_0_80: cast zx s_0_79 -> i
        let s_0_80: i128 = (s_0_79.value() as i128);
        // D s_0_81: cast reint s_0_80 -> i64
        let s_0_81: i64 = (s_0_80 as i64);
        // C s_0_82: const #2s : i
        let s_0_82: i128 = 2;
        // D s_0_83: cast zx s_0_81 -> i
        let s_0_83: i128 = (i128::try_from(s_0_81).unwrap());
        // D s_0_84: add s_0_82 s_0_83
        let s_0_84: i128 = (s_0_82 + s_0_83);
        // D s_0_85: cast reint s_0_84 -> i64
        let s_0_85: i64 = (s_0_84 as i64);
        // D s_0_86: write-var scale <= s_0_85
        fn_state.scale = s_0_85;
        // C s_0_87: const #8s : i64
        let s_0_87: i64 = 8;
        // D s_0_88: read-var scale:i64
        let s_0_88: i64 = fn_state.scale;
        // D s_0_89: lsl s_0_87 s_0_88
        let s_0_89: i64 = s_0_87 << s_0_88;
        // D s_0_90: write-var datasize <= s_0_89
        fn_state.datasize = s_0_89;
        // D s_0_91: read-var L:u8
        let s_0_91: bool = fn_state.L;
        // D s_0_92: cast zx s_0_91 -> bv
        let s_0_92: Bits = Bits::new(s_0_91 as u128, 1u16);
        // C s_0_93: const #1u : u8
        let s_0_93: bool = true;
        // C s_0_94: cast zx s_0_93 -> bv
        let s_0_94: Bits = Bits::new(s_0_93 as u128, 1u16);
        // D s_0_95: cmp-eq s_0_92 s_0_94
        let s_0_95: bool = ((s_0_92) == (s_0_94));
        // N s_0_96: branch s_0_95 b97 b1
        if s_0_95 {
            return block_97(state, tracer, fn_state);
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
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: read-var opc2:u8
        let s_1_3: u8 = fn_state.opc2;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 4u16);
        // C s_1_5: const #1u : u64
        let s_1_5: u64 = 1;
        // D s_1_6: bit-extract s_1_4 s_1_2 s_1_5
        let s_1_6: Bits = (Bits::new(
            ((s_1_4) >> (s_1_2)).value(),
            u16::try_from(s_1_5).unwrap(),
        ));
        // D s_1_7: cast reint s_1_6 -> u8
        let s_1_7: bool = ((s_1_6.value()) != 0);
        // C s_1_8: const #0s : i
        let s_1_8: i128 = 0;
        // C s_1_9: const #0u : u64
        let s_1_9: u64 = 0;
        // D s_1_10: cast zx s_1_7 -> u64
        let s_1_10: u64 = (s_1_7 as u64);
        // C s_1_11: const #1u : u64
        let s_1_11: u64 = 1;
        // D s_1_12: and s_1_10 s_1_11
        let s_1_12: u64 = ((s_1_10) & (s_1_11));
        // D s_1_13: cmp-eq s_1_12 s_1_11
        let s_1_13: bool = ((s_1_12) == (s_1_11));
        // D s_1_14: lsl s_1_10 s_1_8
        let s_1_14: u64 = s_1_10 << s_1_8;
        // D s_1_15: or s_1_9 s_1_14
        let s_1_15: u64 = ((s_1_9) | (s_1_14));
        // D s_1_16: cmpl s_1_14
        let s_1_16: u64 = !s_1_14;
        // D s_1_17: and s_1_9 s_1_16
        let s_1_17: u64 = ((s_1_9) & (s_1_16));
        // D s_1_18: select s_1_13 s_1_15 s_1_17
        let s_1_18: u64 = if s_1_13 { s_1_15 } else { s_1_17 };
        // D s_1_19: cast trunc s_1_18 -> u8
        let s_1_19: bool = ((s_1_18) != 0);
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // C s_1_21: const #0u : u8
        let s_1_21: bool = false;
        // C s_1_22: cast zx s_1_21 -> bv
        let s_1_22: Bits = Bits::new(s_1_21 as u128, 1u16);
        // D s_1_23: cmp-eq s_1_20 s_1_22
        let s_1_23: bool = ((s_1_20) == (s_1_22));
        // N s_1_24: branch s_1_23 b96 b2
        if s_1_23 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: write-var offset <= s_2_0
        fn_state.offset = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var offset:i
        let s_4_0: i128 = fn_state.offset;
        // D s_4_1: write-var offsetshadow#1640 <= s_4_0
        fn_state.offsetshadow_1640 = s_4_0;
        // D s_4_2: read-var wback:u8
        let s_4_2: bool = fn_state.wback;
        // N s_4_3: branch s_4_2 b95 b5
        if s_4_2 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call neq_int(s_5_2, s_5_0)
        let s_5_3: bool = neq_int(state, tracer, s_5_2, s_5_0);
        // D s_5_4: write-var gs#160468 <= s_5_3
        fn_state.gs_160468 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#160468:u8
        let s_6_0: bool = fn_state.gs_160468;
        // D s_6_1: write-var tagchecked <= s_6_0
        fn_state.tagchecked = s_6_0;
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // D s_6_3: write-var rt_unknown <= s_6_2
        fn_state.rt_unknown = s_6_2;
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // D s_6_5: write-var wb_unknown <= s_6_4
        fn_state.wb_unknown = s_6_4;
        // D s_6_6: read-var memop:u32
        let s_6_6: u32 = fn_state.memop;
        // C s_6_7: const #0u : u32
        let s_6_7: u32 = 0;
        // D s_6_8: cmp-eq s_6_6 s_6_7
        let s_6_8: bool = ((s_6_6) == (s_6_7));
        // N s_6_9: branch s_6_8 b94 b7
        if s_6_8 {
            return block_94(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#160469 <= s_7_0
        fn_state.gs_160469 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#160469:u8
        let s_8_0: bool = fn_state.gs_160469;
        // N s_8_1: branch s_8_0 b90 b9
        if s_8_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#160471 <= s_9_0
        fn_state.gs_160471 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#160471:u8
        let s_10_0: bool = fn_state.gs_160471;
        // N s_10_1: branch s_10_0 b89 b11
        if s_10_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#160473 <= s_11_0
        fn_state.gs_160473 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#160473:u8
        let s_12_0: bool = fn_state.gs_160473;
        // N s_12_1: branch s_12_0 b70 b13
        if s_12_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var memop:u32
        let s_14_0: u32 = fn_state.memop;
        // C s_14_1: const #1u : u32
        let s_14_1: u32 = 1;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b69 b15
        if s_14_2 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#160474 <= s_15_0
        fn_state.gs_160474 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#160474:u8
        let s_16_0: bool = fn_state.gs_160474;
        // N s_16_1: branch s_16_0 b65 b17
        if s_16_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#160476 <= s_17_0
        fn_state.gs_160476 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#160476:u8
        let s_18_0: bool = fn_state.gs_160476;
        // N s_18_1: branch s_18_0 b64 b19
        if s_18_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#160478 <= s_19_0
        fn_state.gs_160478 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#160478:u8
        let s_20_0: bool = fn_state.gs_160478;
        // N s_20_1: branch s_20_0 b45 b21
        if s_20_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var memop:u32
        let s_22_0: u32 = fn_state.memop;
        // C s_22_1: const #0u : u32
        let s_22_1: u32 = 0;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // N s_22_3: branch s_22_2 b44 b23
        if s_22_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#160479 <= s_23_0
        fn_state.gs_160479 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#160479:u8
        let s_24_0: bool = fn_state.gs_160479;
        // N s_24_1: branch s_24_0 b30 b25
        if s_24_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
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
        // C s_26_4: const #32s : i
        let s_26_4: i128 = 32;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-eq s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) == (s_26_4));
        // N s_26_7: branch s_26_6 b29 b27
        if s_26_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var datasize:i64
        let s_27_0: i64 = fn_state.datasize;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #64s : i
        let s_27_4: i128 = 64;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#160482 <= s_27_6
        fn_state.gs_160482 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#160482:u8
        let s_28_0: bool = fn_state.gs_160482;
        // N s_28_1: assert s_28_0
        let s_28_1: () = assert!(s_28_0);
        // D s_28_2: read-var datasize:i64
        let s_28_2: i64 = fn_state.datasize;
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_4: cast reint s_28_3 -> i64
        let s_28_4: i64 = (s_28_3 as i64);
        // D s_28_5: read-var memop:u32
        let s_28_5: u32 = fn_state.memop;
        // D s_28_6: read-var n:i64
        let s_28_6: i64 = fn_state.n;
        // D s_28_7: read-var offsetshadow#1640:i
        let s_28_7: i128 = fn_state.offsetshadow_1640;
        // D s_28_8: read-var postindex:u8
        let s_28_8: bool = fn_state.postindex;
        // D s_28_9: read-var rt_unknown:u8
        let s_28_9: bool = fn_state.rt_unknown;
        // D s_28_10: read-var t:i64
        let s_28_10: i64 = fn_state.t;
        // D s_28_11: read-var t2:i64
        let s_28_11: i64 = fn_state.t2;
        // D s_28_12: read-var tagchecked:u8
        let s_28_12: bool = fn_state.tagchecked;
        // D s_28_13: read-var wb_unknown:u8
        let s_28_13: bool = fn_state.wb_unknown;
        // D s_28_14: read-var wback:u8
        let s_28_14: bool = fn_state.wback;
        // D s_28_15: call execute_aarch64_instrs_memory_ordered_pair_ldiapp(s_28_4, s_28_5, s_28_6, s_28_7, s_28_8, s_28_9, s_28_10, s_28_11, s_28_12, s_28_13, s_28_14)
        let s_28_15: () = execute_aarch64_instrs_memory_ordered_pair_ldiapp(
            state,
            tracer,
            s_28_4,
            s_28_5,
            s_28_6,
            s_28_7,
            s_28_8,
            s_28_9,
            s_28_10,
            s_28_11,
            s_28_12,
            s_28_13,
            s_28_14,
        );
        // N s_28_16: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#160482 <= s_29_0
        fn_state.gs_160482 = s_29_0;
        // N s_29_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #3u : u32
        let s_30_0: u32 = 3;
        // S s_30_1: call ConstrainUnpredictable(s_30_0)
        let s_30_1: u32 = ConstrainUnpredictable(state, tracer, s_30_0);
        // D s_30_2: write-var u#2126 <= s_30_1
        fn_state.u_2126 = s_30_1;
        // D s_30_3: read-var u#2126:u32
        let s_30_3: u32 = fn_state.u_2126;
        // C s_30_4: const #1u : u32
        let s_30_4: u32 = 1;
        // D s_30_5: cmp-eq s_30_3 s_30_4
        let s_30_5: bool = ((s_30_3) == (s_30_4));
        // N s_30_6: branch s_30_5 b43 b31
        if s_30_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var u#2126:u32
        let s_31_0: u32 = fn_state.u_2126;
        // C s_31_1: const #2u : u32
        let s_31_1: u32 = 2;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // N s_31_3: branch s_31_2 b42 b32
        if s_31_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var u#2126:u32
        let s_32_0: u32 = fn_state.u_2126;
        // C s_32_1: const #4u : u32
        let s_32_1: u32 = 4;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: write-var gs#160485 <= s_32_2
        fn_state.gs_160485 = s_32_2;
        // N s_32_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#160485:u8
        let s_33_0: bool = fn_state.gs_160485;
        // D s_33_1: write-var gs#160486 <= s_33_0
        fn_state.gs_160486 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#160486:u8
        let s_34_0: bool = fn_state.gs_160486;
        // N s_34_1: assert s_34_0
        let s_34_1: () = assert!(s_34_0);
        // C s_34_2: const #1u : u32
        let s_34_2: u32 = 1;
        // D s_34_3: read-var u#2126:u32
        let s_34_3: u32 = fn_state.u_2126;
        // D s_34_4: cmp-eq s_34_2 s_34_3
        let s_34_4: bool = ((s_34_2) == (s_34_3));
        // D s_34_5: not s_34_4
        let s_34_5: bool = !s_34_4;
        // N s_34_6: branch s_34_5 b37 b35
        if s_34_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var rt_unknown <= s_35_0
        fn_state.rt_unknown = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #2u : u32
        let s_37_0: u32 = 2;
        // D s_37_1: read-var u#2126:u32
        let s_37_1: u32 = fn_state.u_2126;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b39 b38
        if s_37_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #4u : u32
        let s_39_0: u32 = 4;
        // D s_39_1: read-var u#2126:u32
        let s_39_1: u32 = fn_state.u_2126;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EndOfInstruction(s_40_0)
        let s_40_1: () = EndOfInstruction(state, tracer, s_40_0);
        // N s_40_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#160485 <= s_42_0
        fn_state.gs_160485 = s_42_0;
        // N s_42_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#160486 <= s_43_0
        fn_state.gs_160486 = s_43_0;
        // N s_43_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var t:i64
        let s_44_0: i64 = fn_state.t;
        // D s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_2: read-var t2:i64
        let s_44_2: i64 = fn_state.t2;
        // D s_44_3: cast zx s_44_2 -> i
        let s_44_3: i128 = (i128::try_from(s_44_2).unwrap());
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#160479 <= s_44_4
        fn_state.gs_160479 = s_44_4;
        // N s_44_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #2u : u32
        let s_45_0: u32 = 2;
        // S s_45_1: call ConstrainUnpredictable(s_45_0)
        let s_45_1: u32 = ConstrainUnpredictable(state, tracer, s_45_0);
        // D s_45_2: write-var u#2125 <= s_45_1
        fn_state.u_2125 = s_45_1;
        // D s_45_3: read-var u#2125:u32
        let s_45_3: u32 = fn_state.u_2125;
        // C s_45_4: const #0u : u32
        let s_45_4: u32 = 0;
        // D s_45_5: cmp-eq s_45_3 s_45_4
        let s_45_5: bool = ((s_45_3) == (s_45_4));
        // N s_45_6: branch s_45_5 b63 b46
        if s_45_5 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var u#2125:u32
        let s_46_0: u32 = fn_state.u_2125;
        // C s_46_1: const #1u : u32
        let s_46_1: u32 = 1;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // N s_46_3: branch s_46_2 b62 b47
        if s_46_2 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var u#2125:u32
        let s_47_0: u32 = fn_state.u_2125;
        // C s_47_1: const #2u : u32
        let s_47_1: u32 = 2;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // N s_47_3: branch s_47_2 b61 b48
        if s_47_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var u#2125:u32
        let s_48_0: u32 = fn_state.u_2125;
        // C s_48_1: const #4u : u32
        let s_48_1: u32 = 4;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: write-var gs#160494 <= s_48_2
        fn_state.gs_160494 = s_48_2;
        // N s_48_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#160494:u8
        let s_49_0: bool = fn_state.gs_160494;
        // D s_49_1: write-var gs#160495 <= s_49_0
        fn_state.gs_160495 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#160495:u8
        let s_50_0: bool = fn_state.gs_160495;
        // D s_50_1: write-var gs#160496 <= s_50_0
        fn_state.gs_160496 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#160496:u8
        let s_51_0: bool = fn_state.gs_160496;
        // N s_51_1: assert s_51_0
        let s_51_1: () = assert!(s_51_0);
        // C s_51_2: const #0u : u32
        let s_51_2: u32 = 0;
        // D s_51_3: read-var u#2125:u32
        let s_51_3: u32 = fn_state.u_2125;
        // D s_51_4: cmp-eq s_51_2 s_51_3
        let s_51_4: bool = ((s_51_2) == (s_51_3));
        // D s_51_5: not s_51_4
        let s_51_5: bool = !s_51_4;
        // N s_51_6: branch s_51_5 b54 b52
        if s_51_5 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var rt_unknown <= s_52_0
        fn_state.rt_unknown = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u32
        let s_54_0: u32 = 1;
        // D s_54_1: read-var u#2125:u32
        let s_54_1: u32 = fn_state.u_2125;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // N s_54_4: branch s_54_3 b56 b55
        if s_54_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var rt_unknown <= s_55_0
        fn_state.rt_unknown = s_55_0;
        // N s_55_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #2u : u32
        let s_56_0: u32 = 2;
        // D s_56_1: read-var u#2125:u32
        let s_56_1: u32 = fn_state.u_2125;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b58 b57
        if s_56_3 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: panic
        panic!("{:?}", ());
        // N s_57_1: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #4u : u32
        let s_58_0: u32 = 4;
        // D s_58_1: read-var u#2125:u32
        let s_58_1: u32 = fn_state.u_2125;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EndOfInstruction(s_59_0)
        let s_59_1: () = EndOfInstruction(state, tracer, s_59_0);
        // N s_59_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#160494 <= s_61_0
        fn_state.gs_160494 = s_61_0;
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
        // D s_62_1: write-var gs#160495 <= s_62_0
        fn_state.gs_160495 = s_62_0;
        // N s_62_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#160496 <= s_63_0
        fn_state.gs_160496 = s_63_0;
        // N s_63_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #31s : i
        let s_64_0: i128 = 31;
        // D s_64_1: read-var n:i64
        let s_64_1: i64 = fn_state.n;
        // D s_64_2: cast zx s_64_1 -> i
        let s_64_2: i128 = (i128::try_from(s_64_1).unwrap());
        // D s_64_3: call neq_int(s_64_2, s_64_0)
        let s_64_3: bool = neq_int(state, tracer, s_64_2, s_64_0);
        // D s_64_4: write-var gs#160478 <= s_64_3
        fn_state.gs_160478 = s_64_3;
        // N s_64_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var t:i64
        let s_65_0: i64 = fn_state.t;
        // D s_65_1: cast zx s_65_0 -> i
        let s_65_1: i128 = (i128::try_from(s_65_0).unwrap());
        // D s_65_2: read-var n:i64
        let s_65_2: i64 = fn_state.n;
        // D s_65_3: cast zx s_65_2 -> i
        let s_65_3: i128 = (i128::try_from(s_65_2).unwrap());
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // N s_65_5: branch s_65_4 b68 b66
        if s_65_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var t2:i64
        let s_66_0: i64 = fn_state.t2;
        // D s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // D s_66_2: read-var n:i64
        let s_66_2: i64 = fn_state.n;
        // D s_66_3: cast zx s_66_2 -> i
        let s_66_3: i128 = (i128::try_from(s_66_2).unwrap());
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#160475 <= s_66_4
        fn_state.gs_160475 = s_66_4;
        // N s_66_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#160475:u8
        let s_67_0: bool = fn_state.gs_160475;
        // D s_67_1: write-var gs#160476 <= s_67_0
        fn_state.gs_160476 = s_67_0;
        // N s_67_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#160475 <= s_68_0
        fn_state.gs_160475 = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var wback:u8
        let s_69_0: bool = fn_state.wback;
        // D s_69_1: write-var gs#160474 <= s_69_0
        fn_state.gs_160474 = s_69_0;
        // N s_69_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u32
        let s_70_0: u32 = 1;
        // S s_70_1: call ConstrainUnpredictable(s_70_0)
        let s_70_1: u32 = ConstrainUnpredictable(state, tracer, s_70_0);
        // D s_70_2: write-var c <= s_70_1
        fn_state.c = s_70_1;
        // D s_70_3: read-var c:u32
        let s_70_3: u32 = fn_state.c;
        // C s_70_4: const #11u : u32
        let s_70_4: u32 = 11;
        // D s_70_5: cmp-eq s_70_3 s_70_4
        let s_70_5: bool = ((s_70_3) == (s_70_4));
        // N s_70_6: branch s_70_5 b88 b71
        if s_70_5 {
            return block_88(state, tracer, fn_state);
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
        // C s_71_1: const #1u : u32
        let s_71_1: u32 = 1;
        // D s_71_2: cmp-eq s_71_0 s_71_1
        let s_71_2: bool = ((s_71_0) == (s_71_1));
        // N s_71_3: branch s_71_2 b87 b72
        if s_71_2 {
            return block_87(state, tracer, fn_state);
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
        // C s_72_1: const #2u : u32
        let s_72_1: u32 = 2;
        // D s_72_2: cmp-eq s_72_0 s_72_1
        let s_72_2: bool = ((s_72_0) == (s_72_1));
        // N s_72_3: branch s_72_2 b86 b73
        if s_72_2 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var c:u32
        let s_73_0: u32 = fn_state.c;
        // C s_73_1: const #4u : u32
        let s_73_1: u32 = 4;
        // D s_73_2: cmp-eq s_73_0 s_73_1
        let s_73_2: bool = ((s_73_0) == (s_73_1));
        // D s_73_3: write-var gs#160505 <= s_73_2
        fn_state.gs_160505 = s_73_2;
        // N s_73_4: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#160505:u8
        let s_74_0: bool = fn_state.gs_160505;
        // D s_74_1: write-var gs#160506 <= s_74_0
        fn_state.gs_160506 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#160506:u8
        let s_75_0: bool = fn_state.gs_160506;
        // D s_75_1: write-var gs#160507 <= s_75_0
        fn_state.gs_160507 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#160507:u8
        let s_76_0: bool = fn_state.gs_160507;
        // N s_76_1: assert s_76_0
        let s_76_1: () = assert!(s_76_0);
        // C s_76_2: const #11u : u32
        let s_76_2: u32 = 11;
        // D s_76_3: read-var c:u32
        let s_76_3: u32 = fn_state.c;
        // D s_76_4: cmp-eq s_76_2 s_76_3
        let s_76_4: bool = ((s_76_2) == (s_76_3));
        // D s_76_5: not s_76_4
        let s_76_5: bool = !s_76_4;
        // N s_76_6: branch s_76_5 b79 b77
        if s_76_5 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var wback <= s_77_0
        fn_state.wback = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u32
        let s_79_0: u32 = 1;
        // D s_79_1: read-var c:u32
        let s_79_1: u32 = fn_state.c;
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // D s_79_3: not s_79_2
        let s_79_3: bool = !s_79_2;
        // N s_79_4: branch s_79_3 b81 b80
        if s_79_3 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var wb_unknown <= s_80_0
        fn_state.wb_unknown = s_80_0;
        // N s_80_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #2u : u32
        let s_81_0: u32 = 2;
        // D s_81_1: read-var c:u32
        let s_81_1: u32 = fn_state.c;
        // D s_81_2: cmp-eq s_81_0 s_81_1
        let s_81_2: bool = ((s_81_0) == (s_81_1));
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // N s_81_4: branch s_81_3 b83 b82
        if s_81_3 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #4u : u32
        let s_83_0: u32 = 4;
        // D s_83_1: read-var c:u32
        let s_83_1: u32 = fn_state.c;
        // D s_83_2: cmp-eq s_83_0 s_83_1
        let s_83_2: bool = ((s_83_0) == (s_83_1));
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // N s_83_4: branch s_83_3 b85 b84
        if s_83_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EndOfInstruction(s_84_0)
        let s_84_1: () = EndOfInstruction(state, tracer, s_84_0);
        // N s_84_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#160505 <= s_86_0
        fn_state.gs_160505 = s_86_0;
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
        // D s_87_1: write-var gs#160506 <= s_87_0
        fn_state.gs_160506 = s_87_0;
        // N s_87_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#160507 <= s_88_0
        fn_state.gs_160507 = s_88_0;
        // N s_88_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #31s : i
        let s_89_0: i128 = 31;
        // D s_89_1: read-var n:i64
        let s_89_1: i64 = fn_state.n;
        // D s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (i128::try_from(s_89_1).unwrap());
        // D s_89_3: call neq_int(s_89_2, s_89_0)
        let s_89_3: bool = neq_int(state, tracer, s_89_2, s_89_0);
        // D s_89_4: write-var gs#160473 <= s_89_3
        fn_state.gs_160473 = s_89_3;
        // N s_89_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var t:i64
        let s_90_0: i64 = fn_state.t;
        // D s_90_1: cast zx s_90_0 -> i
        let s_90_1: i128 = (i128::try_from(s_90_0).unwrap());
        // D s_90_2: read-var n:i64
        let s_90_2: i64 = fn_state.n;
        // D s_90_3: cast zx s_90_2 -> i
        let s_90_3: i128 = (i128::try_from(s_90_2).unwrap());
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // N s_90_5: branch s_90_4 b93 b91
        if s_90_4 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var t2:i64
        let s_91_0: i64 = fn_state.t2;
        // D s_91_1: cast zx s_91_0 -> i
        let s_91_1: i128 = (i128::try_from(s_91_0).unwrap());
        // D s_91_2: read-var n:i64
        let s_91_2: i64 = fn_state.n;
        // D s_91_3: cast zx s_91_2 -> i
        let s_91_3: i128 = (i128::try_from(s_91_2).unwrap());
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#160470 <= s_91_4
        fn_state.gs_160470 = s_91_4;
        // N s_91_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#160470:u8
        let s_92_0: bool = fn_state.gs_160470;
        // D s_92_1: write-var gs#160471 <= s_92_0
        fn_state.gs_160471 = s_92_0;
        // N s_92_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#160470 <= s_93_0
        fn_state.gs_160470 = s_93_0;
        // N s_93_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var wback:u8
        let s_94_0: bool = fn_state.wback;
        // D s_94_1: write-var gs#160469 <= s_94_0
        fn_state.gs_160469 = s_94_0;
        // N s_94_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#160468 <= s_95_0
        fn_state.gs_160468 = s_95_0;
        // N s_95_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #2s : i
        let s_96_0: i128 = 2;
        // D s_96_1: read-var scale:i64
        let s_96_1: i64 = fn_state.scale;
        // D s_96_2: cast zx s_96_1 -> i
        let s_96_2: i128 = (i128::try_from(s_96_1).unwrap());
        // D s_96_3: lsl s_96_0 s_96_2
        let s_96_3: i128 = s_96_0 << s_96_2;
        // C s_96_4: const #-1s : i
        let s_96_4: i128 = -1;
        // D s_96_5: mul s_96_4 s_96_3
        let s_96_5: i128 = ((s_96_4) * (s_96_3));
        // D s_96_6: write-var offset <= s_96_5
        fn_state.offset = s_96_5;
        // N s_96_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u32
        let s_97_0: u32 = 0;
        // D s_97_1: write-var memop <= s_97_0
        fn_state.memop = s_97_0;
        // C s_97_2: const #0s : i
        let s_97_2: i128 = 0;
        // D s_97_3: read-var opc2:u8
        let s_97_3: u8 = fn_state.opc2;
        // D s_97_4: cast zx s_97_3 -> bv
        let s_97_4: Bits = Bits::new(s_97_3 as u128, 4u16);
        // C s_97_5: const #1u : u64
        let s_97_5: u64 = 1;
        // D s_97_6: bit-extract s_97_4 s_97_2 s_97_5
        let s_97_6: Bits = (Bits::new(
            ((s_97_4) >> (s_97_2)).value(),
            u16::try_from(s_97_5).unwrap(),
        ));
        // D s_97_7: cast reint s_97_6 -> u8
        let s_97_7: bool = ((s_97_6.value()) != 0);
        // C s_97_8: const #0s : i
        let s_97_8: i128 = 0;
        // C s_97_9: const #0u : u64
        let s_97_9: u64 = 0;
        // D s_97_10: cast zx s_97_7 -> u64
        let s_97_10: u64 = (s_97_7 as u64);
        // C s_97_11: const #1u : u64
        let s_97_11: u64 = 1;
        // D s_97_12: and s_97_10 s_97_11
        let s_97_12: u64 = ((s_97_10) & (s_97_11));
        // D s_97_13: cmp-eq s_97_12 s_97_11
        let s_97_13: bool = ((s_97_12) == (s_97_11));
        // D s_97_14: lsl s_97_10 s_97_8
        let s_97_14: u64 = s_97_10 << s_97_8;
        // D s_97_15: or s_97_9 s_97_14
        let s_97_15: u64 = ((s_97_9) | (s_97_14));
        // D s_97_16: cmpl s_97_14
        let s_97_16: u64 = !s_97_14;
        // D s_97_17: and s_97_9 s_97_16
        let s_97_17: u64 = ((s_97_9) & (s_97_16));
        // D s_97_18: select s_97_13 s_97_15 s_97_17
        let s_97_18: u64 = if s_97_13 { s_97_15 } else { s_97_17 };
        // D s_97_19: cast trunc s_97_18 -> u8
        let s_97_19: bool = ((s_97_18) != 0);
        // D s_97_20: cast zx s_97_19 -> bv
        let s_97_20: Bits = Bits::new(s_97_19 as u128, 1u16);
        // C s_97_21: const #0u : u8
        let s_97_21: bool = false;
        // C s_97_22: cast zx s_97_21 -> bv
        let s_97_22: Bits = Bits::new(s_97_21 as u128, 1u16);
        // D s_97_23: cmp-eq s_97_20 s_97_22
        let s_97_23: bool = ((s_97_20) == (s_97_22));
        // N s_97_24: branch s_97_23 b100 b98
        if s_97_23 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0s : i
        let s_98_0: i128 = 0;
        // D s_98_1: write-var offset <= s_98_0
        fn_state.offset = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #2s : i
        let s_100_0: i128 = 2;
        // D s_100_1: read-var scale:i64
        let s_100_1: i64 = fn_state.scale;
        // D s_100_2: cast zx s_100_1 -> i
        let s_100_2: i128 = (i128::try_from(s_100_1).unwrap());
        // D s_100_3: lsl s_100_0 s_100_2
        let s_100_3: i128 = s_100_0 << s_100_2;
        // D s_100_4: write-var offset <= s_100_3
        fn_state.offset = s_100_3;
        // N s_100_5: jump b99
        return block_99(state, tracer, fn_state);
    }
}
