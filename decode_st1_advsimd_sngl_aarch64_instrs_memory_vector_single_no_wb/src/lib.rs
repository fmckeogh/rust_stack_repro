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
use execute_aarch64_instrs_memory_vector_single_no_wb::*;
use u__id::*;
use u_shl_int_general::*;
use u__UNKNOWN_integer::*;
use common::*;
pub fn decode_st1_advsimd_sngl_aarch64_instrs_memory_vector_single_no_wb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    size: u8,
    S: bool,
    opcode: u8,
    o2: bool,
    R: bool,
    L: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        t: i64,
        esize: i128,
        gs_156275: bool,
        n: i64,
        index: i128,
        memop: u32,
        scaleshadow_1522: i128,
        gs_156273: bool,
        gs_156239: bool,
        indexshadow_1523: i128,
        gs_156277: bool,
        scale: i128,
        tagchecked: bool,
        selem: i64,
        ga_257738: i64,
        datasize: i64,
        replicate: bool,
        Rt: u8,
        Rn: u8,
        size: u8,
        S: bool,
        opcode: u8,
        o2: bool,
        R: bool,
        L: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        size,
        S,
        opcode,
        o2,
        R,
        L,
        Q,
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
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var t <= s_0_3
        fn_state.t = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call __UNKNOWN_integer(s_0_10)
        let s_0_11: i128 = u__UNKNOWN_integer(state, tracer, s_0_10);
        // D s_0_12: write-var m <= s_0_11
        fn_state.m = s_0_11;
        // C s_0_13: const #31s : i
        let s_0_13: i128 = 31;
        // D s_0_14: read-var n:i64
        let s_0_14: i64 = fn_state.n;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call neq_int(s_0_15, s_0_13)
        let s_0_16: bool = neq_int(state, tracer, s_0_15, s_0_13);
        // D s_0_17: write-var tagchecked <= s_0_16
        fn_state.tagchecked = s_0_16;
        // C s_0_18: const #1s : i
        let s_0_18: i128 = 1;
        // D s_0_19: read-var opcode:u8
        let s_0_19: u8 = fn_state.opcode;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 3u16);
        // C s_0_21: const #1s : i64
        let s_0_21: i64 = 1;
        // C s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // C s_0_23: const #1s : i
        let s_0_23: i128 = 1;
        // C s_0_24: add s_0_23 s_0_22
        let s_0_24: i128 = (s_0_23 + s_0_22);
        // D s_0_25: bit-extract s_0_20 s_0_18 s_0_24
        let s_0_25: Bits = (Bits::new(
            ((s_0_20) >> (s_0_18)).value(),
            u16::try_from(s_0_24).unwrap(),
        ));
        // D s_0_26: cast reint s_0_25 -> u8
        let s_0_26: u8 = (s_0_25.value() as u8);
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (s_0_27.value() as i128);
        // D s_0_29: cast reint s_0_28 -> i64
        let s_0_29: i64 = (s_0_28 as i64);
        // D s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (i128::try_from(s_0_29).unwrap());
        // D s_0_31: write-var scale <= s_0_30
        fn_state.scale = s_0_30;
        // C s_0_32: const #0s : i
        let s_0_32: i128 = 0;
        // D s_0_33: read-var opcode:u8
        let s_0_33: u8 = fn_state.opcode;
        // D s_0_34: cast zx s_0_33 -> bv
        let s_0_34: Bits = Bits::new(s_0_33 as u128, 3u16);
        // C s_0_35: const #1u : u64
        let s_0_35: u64 = 1;
        // D s_0_36: bit-extract s_0_34 s_0_32 s_0_35
        let s_0_36: Bits = (Bits::new(
            ((s_0_34) >> (s_0_32)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_37: cast reint s_0_36 -> u8
        let s_0_37: bool = ((s_0_36.value()) != 0);
        // C s_0_38: const #0s : i
        let s_0_38: i128 = 0;
        // C s_0_39: const #0u : u64
        let s_0_39: u64 = 0;
        // D s_0_40: cast zx s_0_37 -> u64
        let s_0_40: u64 = (s_0_37 as u64);
        // C s_0_41: const #1u : u64
        let s_0_41: u64 = 1;
        // D s_0_42: and s_0_40 s_0_41
        let s_0_42: u64 = ((s_0_40) & (s_0_41));
        // D s_0_43: cmp-eq s_0_42 s_0_41
        let s_0_43: bool = ((s_0_42) == (s_0_41));
        // D s_0_44: lsl s_0_40 s_0_38
        let s_0_44: u64 = s_0_40 << s_0_38;
        // D s_0_45: or s_0_39 s_0_44
        let s_0_45: u64 = ((s_0_39) | (s_0_44));
        // D s_0_46: cmpl s_0_44
        let s_0_46: u64 = !s_0_44;
        // D s_0_47: and s_0_39 s_0_46
        let s_0_47: u64 = ((s_0_39) & (s_0_46));
        // D s_0_48: select s_0_43 s_0_45 s_0_47
        let s_0_48: u64 = if s_0_43 { s_0_45 } else { s_0_47 };
        // D s_0_49: cast trunc s_0_48 -> u8
        let s_0_49: bool = ((s_0_48) != 0);
        // D s_0_50: cast zx s_0_49 -> bv
        let s_0_50: Bits = Bits::new(s_0_49 as u128, 1u16);
        // D s_0_51: read-var R:u8
        let s_0_51: bool = fn_state.R;
        // D s_0_52: cast zx s_0_51 -> bv
        let s_0_52: Bits = Bits::new(s_0_51 as u128, 1u16);
        // D s_0_53: cast reint s_0_50 -> u128
        let s_0_53: u128 = (s_0_50.value() as u128);
        // D s_0_54: size-of s_0_50
        let s_0_54: u16 = s_0_50.length();
        // D s_0_55: cast reint s_0_52 -> u128
        let s_0_55: u128 = (s_0_52.value() as u128);
        // D s_0_56: size-of s_0_52
        let s_0_56: u16 = s_0_52.length();
        // D s_0_57: lsl s_0_53 s_0_56
        let s_0_57: u128 = s_0_53 << s_0_56;
        // D s_0_58: or s_0_57 s_0_55
        let s_0_58: u128 = ((s_0_57) | (s_0_55));
        // D s_0_59: add s_0_54 s_0_56
        let s_0_59: u16 = (s_0_54 + s_0_56);
        // D s_0_60: create-bits s_0_58 s_0_59
        let s_0_60: Bits = Bits::new(s_0_58, s_0_59);
        // D s_0_61: cast reint s_0_60 -> u8
        let s_0_61: u8 = (s_0_60.value() as u8);
        // D s_0_62: cast zx s_0_61 -> bv
        let s_0_62: Bits = Bits::new(s_0_61 as u128, 2u16);
        // D s_0_63: cast zx s_0_62 -> i
        let s_0_63: i128 = (s_0_62.value() as i128);
        // D s_0_64: cast reint s_0_63 -> i64
        let s_0_64: i64 = (s_0_63 as i64);
        // C s_0_65: const #1s : i
        let s_0_65: i128 = 1;
        // D s_0_66: cast zx s_0_64 -> i
        let s_0_66: i128 = (i128::try_from(s_0_64).unwrap());
        // D s_0_67: add s_0_66 s_0_65
        let s_0_67: i128 = (s_0_66 + s_0_65);
        // D s_0_68: cast reint s_0_67 -> i64
        let s_0_68: i64 = (s_0_67 as i64);
        // D s_0_69: write-var selem <= s_0_68
        fn_state.selem = s_0_68;
        // C s_0_70: const #0u : u8
        let s_0_70: bool = false;
        // D s_0_71: write-var replicate <= s_0_70
        fn_state.replicate = s_0_70;
        // D s_0_72: read-var scale:i
        let s_0_72: i128 = fn_state.scale;
        // C s_0_73: const #3s : i
        let s_0_73: i128 = 3;
        // D s_0_74: cmp-eq s_0_72 s_0_73
        let s_0_74: bool = ((s_0_72) == (s_0_73));
        // D s_0_75: not s_0_74
        let s_0_75: bool = !s_0_74;
        // N s_0_76: branch s_0_75 b23 b1
        if s_0_75 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var L:u8
        let s_1_0: bool = fn_state.L;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b22 b2
        if s_1_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var S:u8
        let s_2_0: bool = fn_state.S;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var gs#156239 <= s_2_4
        fn_state.gs_156239 = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#156239:u8
        let s_3_0: bool = fn_state.gs_156239;
        // N s_3_1: branch s_3_0 b21 b4
        if s_3_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: write-var scale <= s_4_2
        fn_state.scale = s_4_2;
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // D s_4_5: write-var replicate <= s_4_4
        fn_state.replicate = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var scale:i
        let s_5_0: i128 = fn_state.scale;
        // D s_5_1: write-var scaleshadow#1522 <= s_5_0
        fn_state.scaleshadow_1522 = s_5_0;
        // D s_5_2: read-var index:i
        let s_5_2: i128 = fn_state.index;
        // D s_5_3: write-var indexshadow#1523 <= s_5_2
        fn_state.indexshadow_1523 = s_5_2;
        // D s_5_4: read-var L:u8
        let s_5_4: bool = fn_state.L;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // C s_5_6: const #1u : u8
        let s_5_6: bool = true;
        // C s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 1u16);
        // D s_5_8: cmp-eq s_5_5 s_5_7
        let s_5_8: bool = ((s_5_5) == (s_5_7));
        // N s_5_9: branch s_5_8 b20 b6
        if s_5_8 {
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
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var memop <= s_6_0
        fn_state.memop = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Q:u8
        let s_7_0: bool = fn_state.Q;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b19 b8
        if s_7_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: write-var ga#257738 <= s_8_0
        fn_state.ga_257738 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#257738:i64
        let s_9_0: i64 = fn_state.ga_257738;
        // D s_9_1: write-var datasize <= s_9_0
        fn_state.datasize = s_9_0;
        // C s_9_2: const #8s : i
        let s_9_2: i128 = 8;
        // D s_9_3: read-var scaleshadow#1522:i
        let s_9_3: i128 = fn_state.scaleshadow_1522;
        // D s_9_4: call _shl_int_general(s_9_2, s_9_3)
        let s_9_4: i128 = u_shl_int_general(state, tracer, s_9_2, s_9_3);
        // D s_9_5: write-var esize <= s_9_4
        fn_state.esize = s_9_4;
        // D s_9_6: read-var esize:i
        let s_9_6: i128 = fn_state.esize;
        // D s_9_7: call __id(s_9_6)
        let s_9_7: i128 = u__id(state, tracer, s_9_6);
        // C s_9_8: const #8s : i
        let s_9_8: i128 = 8;
        // D s_9_9: cmp-eq s_9_7 s_9_8
        let s_9_9: bool = ((s_9_7) == (s_9_8));
        // N s_9_10: branch s_9_9 b18 b10
        if s_9_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i
        let s_10_0: i128 = fn_state.esize;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #16s : i
        let s_10_2: i128 = 16;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#156273 <= s_10_3
        fn_state.gs_156273 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#156273:u8
        let s_11_0: bool = fn_state.gs_156273;
        // N s_11_1: branch s_11_0 b17 b12
        if s_11_0 {
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
        // D s_12_0: read-var esize:i
        let s_12_0: i128 = fn_state.esize;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // C s_12_2: const #32s : i
        let s_12_2: i128 = 32;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#156275 <= s_12_3
        fn_state.gs_156275 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#156275:u8
        let s_13_0: bool = fn_state.gs_156275;
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
        // D s_14_0: read-var esize:i
        let s_14_0: i128 = fn_state.esize;
        // D s_14_1: call __id(s_14_0)
        let s_14_1: i128 = u__id(state, tracer, s_14_0);
        // C s_14_2: const #64s : i
        let s_14_2: i128 = 64;
        // D s_14_3: cmp-eq s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) == (s_14_2));
        // D s_14_4: write-var gs#156277 <= s_14_3
        fn_state.gs_156277 = s_14_3;
        // N s_14_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#156277:u8
        let s_15_0: bool = fn_state.gs_156277;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var datasize:i64
        let s_15_2: i64 = fn_state.datasize;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // D s_15_5: read-var esize:i
        let s_15_5: i128 = fn_state.esize;
        // D s_15_6: cast reint s_15_5 -> i64
        let s_15_6: i64 = (s_15_5 as i64);
        // D s_15_7: read-var indexshadow#1523:i
        let s_15_7: i128 = fn_state.indexshadow_1523;
        // D s_15_8: read-var m:i
        let s_15_8: i128 = fn_state.m;
        // D s_15_9: read-var memop:u32
        let s_15_9: u32 = fn_state.memop;
        // D s_15_10: read-var n:i64
        let s_15_10: i64 = fn_state.n;
        // C s_15_11: const #0u : u8
        let s_15_11: bool = false;
        // D s_15_12: read-var replicate:u8
        let s_15_12: bool = fn_state.replicate;
        // D s_15_13: read-var selem:i64
        let s_15_13: i64 = fn_state.selem;
        // D s_15_14: read-var t:i64
        let s_15_14: i64 = fn_state.t;
        // D s_15_15: read-var tagchecked:u8
        let s_15_15: bool = fn_state.tagchecked;
        // C s_15_16: const #0u : u8
        let s_15_16: bool = false;
        // D s_15_17: call execute_aarch64_instrs_memory_vector_single_no_wb(s_15_4, s_15_6, s_15_7, s_15_8, s_15_9, s_15_10, s_15_11, s_15_12, s_15_13, s_15_14, s_15_15, s_15_16)
        let s_15_17: () = execute_aarch64_instrs_memory_vector_single_no_wb(
            state,
            tracer,
            s_15_4,
            s_15_6,
            s_15_7,
            s_15_8,
            s_15_9,
            s_15_10,
            s_15_11,
            s_15_12,
            s_15_13,
            s_15_14,
            s_15_15,
            s_15_16,
        );
        // N s_15_18: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#156277 <= s_16_0
        fn_state.gs_156277 = s_16_0;
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
        // D s_17_1: write-var gs#156275 <= s_17_0
        fn_state.gs_156275 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#156273 <= s_18_0
        fn_state.gs_156273 = s_18_0;
        // N s_18_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #128s : i64
        let s_19_0: i64 = 128;
        // D s_19_1: write-var ga#257738 <= s_19_0
        fn_state.ga_257738 = s_19_0;
        // N s_19_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u32
        let s_20_0: u32 = 0;
        // D s_20_1: write-var memop <= s_20_0
        fn_state.memop = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#156239 <= s_22_0
        fn_state.gs_156239 = s_22_0;
        // N s_22_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var scale:i
        let s_23_0: i128 = fn_state.scale;
        // C s_23_1: const #0s : i
        let s_23_1: i128 = 0;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b25 b24
        if s_23_3 {
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
        // D s_24_0: read-var Q:u8
        let s_24_0: bool = fn_state.Q;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // D s_24_2: read-var S:u8
        let s_24_2: bool = fn_state.S;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cast reint s_24_1 -> u128
        let s_24_4: u128 = (s_24_1.value() as u128);
        // D s_24_5: size-of s_24_1
        let s_24_5: u16 = s_24_1.length();
        // D s_24_6: cast reint s_24_3 -> u128
        let s_24_6: u128 = (s_24_3.value() as u128);
        // D s_24_7: size-of s_24_3
        let s_24_7: u16 = s_24_3.length();
        // D s_24_8: lsl s_24_4 s_24_7
        let s_24_8: u128 = s_24_4 << s_24_7;
        // D s_24_9: or s_24_8 s_24_6
        let s_24_9: u128 = ((s_24_8) | (s_24_6));
        // D s_24_10: add s_24_5 s_24_7
        let s_24_10: u16 = (s_24_5 + s_24_7);
        // D s_24_11: create-bits s_24_9 s_24_10
        let s_24_11: Bits = Bits::new(s_24_9, s_24_10);
        // D s_24_12: cast reint s_24_11 -> u8
        let s_24_12: u8 = (s_24_11.value() as u8);
        // D s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 2u16);
        // D s_24_14: read-var size:u8
        let s_24_14: u8 = fn_state.size;
        // D s_24_15: cast zx s_24_14 -> bv
        let s_24_15: Bits = Bits::new(s_24_14 as u128, 2u16);
        // D s_24_16: cast reint s_24_13 -> u128
        let s_24_16: u128 = (s_24_13.value() as u128);
        // D s_24_17: size-of s_24_13
        let s_24_17: u16 = s_24_13.length();
        // D s_24_18: cast reint s_24_15 -> u128
        let s_24_18: u128 = (s_24_15.value() as u128);
        // D s_24_19: size-of s_24_15
        let s_24_19: u16 = s_24_15.length();
        // D s_24_20: lsl s_24_16 s_24_19
        let s_24_20: u128 = s_24_16 << s_24_19;
        // D s_24_21: or s_24_20 s_24_18
        let s_24_21: u128 = ((s_24_20) | (s_24_18));
        // D s_24_22: add s_24_17 s_24_19
        let s_24_22: u16 = (s_24_17 + s_24_19);
        // D s_24_23: create-bits s_24_21 s_24_22
        let s_24_23: Bits = Bits::new(s_24_21, s_24_22);
        // D s_24_24: cast reint s_24_23 -> u8
        let s_24_24: u8 = (s_24_23.value() as u8);
        // D s_24_25: cast zx s_24_24 -> bv
        let s_24_25: Bits = Bits::new(s_24_24 as u128, 4u16);
        // D s_24_26: cast zx s_24_25 -> i
        let s_24_26: i128 = (s_24_25.value() as i128);
        // D s_24_27: write-var index <= s_24_26
        fn_state.index = s_24_26;
        // N s_24_28: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var scale:i
        let s_25_0: i128 = fn_state.scale;
        // C s_25_1: const #1s : i
        let s_25_1: i128 = 1;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b29 b26
        if s_25_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var size:u8
        let s_26_1: u8 = fn_state.size;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 2u16);
        // C s_26_3: const #1u : u64
        let s_26_3: u64 = 1;
        // D s_26_4: bit-extract s_26_2 s_26_0 s_26_3
        let s_26_4: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_3).unwrap(),
        ));
        // D s_26_5: cast reint s_26_4 -> u8
        let s_26_5: bool = ((s_26_4.value()) != 0);
        // C s_26_6: const #0s : i
        let s_26_6: i128 = 0;
        // C s_26_7: const #0u : u64
        let s_26_7: u64 = 0;
        // D s_26_8: cast zx s_26_5 -> u64
        let s_26_8: u64 = (s_26_5 as u64);
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // D s_26_10: and s_26_8 s_26_9
        let s_26_10: u64 = ((s_26_8) & (s_26_9));
        // D s_26_11: cmp-eq s_26_10 s_26_9
        let s_26_11: bool = ((s_26_10) == (s_26_9));
        // D s_26_12: lsl s_26_8 s_26_6
        let s_26_12: u64 = s_26_8 << s_26_6;
        // D s_26_13: or s_26_7 s_26_12
        let s_26_13: u64 = ((s_26_7) | (s_26_12));
        // D s_26_14: cmpl s_26_12
        let s_26_14: u64 = !s_26_12;
        // D s_26_15: and s_26_7 s_26_14
        let s_26_15: u64 = ((s_26_7) & (s_26_14));
        // D s_26_16: select s_26_11 s_26_13 s_26_15
        let s_26_16: u64 = if s_26_11 { s_26_13 } else { s_26_15 };
        // D s_26_17: cast trunc s_26_16 -> u8
        let s_26_17: bool = ((s_26_16) != 0);
        // D s_26_18: cast zx s_26_17 -> bv
        let s_26_18: Bits = Bits::new(s_26_17 as u128, 1u16);
        // C s_26_19: const #1u : u8
        let s_26_19: bool = true;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_21: cmp-eq s_26_18 s_26_20
        let s_26_21: bool = ((s_26_18) == (s_26_20));
        // N s_26_22: branch s_26_21 b28 b27
        if s_26_21 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var Q:u8
        let s_27_0: bool = fn_state.Q;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // D s_27_2: read-var S:u8
        let s_27_2: bool = fn_state.S;
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cast reint s_27_1 -> u128
        let s_27_4: u128 = (s_27_1.value() as u128);
        // D s_27_5: size-of s_27_1
        let s_27_5: u16 = s_27_1.length();
        // D s_27_6: cast reint s_27_3 -> u128
        let s_27_6: u128 = (s_27_3.value() as u128);
        // D s_27_7: size-of s_27_3
        let s_27_7: u16 = s_27_3.length();
        // D s_27_8: lsl s_27_4 s_27_7
        let s_27_8: u128 = s_27_4 << s_27_7;
        // D s_27_9: or s_27_8 s_27_6
        let s_27_9: u128 = ((s_27_8) | (s_27_6));
        // D s_27_10: add s_27_5 s_27_7
        let s_27_10: u16 = (s_27_5 + s_27_7);
        // D s_27_11: create-bits s_27_9 s_27_10
        let s_27_11: Bits = Bits::new(s_27_9, s_27_10);
        // D s_27_12: cast reint s_27_11 -> u8
        let s_27_12: u8 = (s_27_11.value() as u8);
        // C s_27_13: const #1s : i
        let s_27_13: i128 = 1;
        // D s_27_14: read-var size:u8
        let s_27_14: u8 = fn_state.size;
        // D s_27_15: cast zx s_27_14 -> bv
        let s_27_15: Bits = Bits::new(s_27_14 as u128, 2u16);
        // C s_27_16: const #1u : u64
        let s_27_16: u64 = 1;
        // D s_27_17: bit-extract s_27_15 s_27_13 s_27_16
        let s_27_17: Bits = (Bits::new(
            ((s_27_15) >> (s_27_13)).value(),
            u16::try_from(s_27_16).unwrap(),
        ));
        // D s_27_18: cast reint s_27_17 -> u8
        let s_27_18: bool = ((s_27_17.value()) != 0);
        // C s_27_19: const #0s : i
        let s_27_19: i128 = 0;
        // C s_27_20: const #0u : u64
        let s_27_20: u64 = 0;
        // D s_27_21: cast zx s_27_18 -> u64
        let s_27_21: u64 = (s_27_18 as u64);
        // C s_27_22: const #1u : u64
        let s_27_22: u64 = 1;
        // D s_27_23: and s_27_21 s_27_22
        let s_27_23: u64 = ((s_27_21) & (s_27_22));
        // D s_27_24: cmp-eq s_27_23 s_27_22
        let s_27_24: bool = ((s_27_23) == (s_27_22));
        // D s_27_25: lsl s_27_21 s_27_19
        let s_27_25: u64 = s_27_21 << s_27_19;
        // D s_27_26: or s_27_20 s_27_25
        let s_27_26: u64 = ((s_27_20) | (s_27_25));
        // D s_27_27: cmpl s_27_25
        let s_27_27: u64 = !s_27_25;
        // D s_27_28: and s_27_20 s_27_27
        let s_27_28: u64 = ((s_27_20) & (s_27_27));
        // D s_27_29: select s_27_24 s_27_26 s_27_28
        let s_27_29: u64 = if s_27_24 { s_27_26 } else { s_27_28 };
        // D s_27_30: cast trunc s_27_29 -> u8
        let s_27_30: bool = ((s_27_29) != 0);
        // D s_27_31: cast zx s_27_12 -> bv
        let s_27_31: Bits = Bits::new(s_27_12 as u128, 2u16);
        // D s_27_32: cast zx s_27_30 -> bv
        let s_27_32: Bits = Bits::new(s_27_30 as u128, 1u16);
        // D s_27_33: cast reint s_27_31 -> u128
        let s_27_33: u128 = (s_27_31.value() as u128);
        // D s_27_34: size-of s_27_31
        let s_27_34: u16 = s_27_31.length();
        // D s_27_35: cast reint s_27_32 -> u128
        let s_27_35: u128 = (s_27_32.value() as u128);
        // D s_27_36: size-of s_27_32
        let s_27_36: u16 = s_27_32.length();
        // D s_27_37: lsl s_27_33 s_27_36
        let s_27_37: u128 = s_27_33 << s_27_36;
        // D s_27_38: or s_27_37 s_27_35
        let s_27_38: u128 = ((s_27_37) | (s_27_35));
        // D s_27_39: add s_27_34 s_27_36
        let s_27_39: u16 = (s_27_34 + s_27_36);
        // D s_27_40: create-bits s_27_38 s_27_39
        let s_27_40: Bits = Bits::new(s_27_38, s_27_39);
        // D s_27_41: cast reint s_27_40 -> u8
        let s_27_41: u8 = (s_27_40.value() as u8);
        // D s_27_42: cast zx s_27_41 -> bv
        let s_27_42: Bits = Bits::new(s_27_41 as u128, 3u16);
        // D s_27_43: cast zx s_27_42 -> i
        let s_27_43: i128 = (s_27_42.value() as i128);
        // D s_27_44: write-var index <= s_27_43
        fn_state.index = s_27_43;
        // N s_27_45: jump b5
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
        // D s_29_0: read-var scale:i
        let s_29_0: i128 = fn_state.scale;
        // C s_29_1: const #2s : i
        let s_29_1: i128 = 2;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // N s_29_4: branch s_29_3 b37 b30
        if s_29_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1s : i
        let s_30_0: i128 = 1;
        // D s_30_1: read-var size:u8
        let s_30_1: u8 = fn_state.size;
        // D s_30_2: cast zx s_30_1 -> bv
        let s_30_2: Bits = Bits::new(s_30_1 as u128, 2u16);
        // C s_30_3: const #1u : u64
        let s_30_3: u64 = 1;
        // D s_30_4: bit-extract s_30_2 s_30_0 s_30_3
        let s_30_4: Bits = (Bits::new(
            ((s_30_2) >> (s_30_0)).value(),
            u16::try_from(s_30_3).unwrap(),
        ));
        // D s_30_5: cast reint s_30_4 -> u8
        let s_30_5: bool = ((s_30_4.value()) != 0);
        // C s_30_6: const #0s : i
        let s_30_6: i128 = 0;
        // C s_30_7: const #0u : u64
        let s_30_7: u64 = 0;
        // D s_30_8: cast zx s_30_5 -> u64
        let s_30_8: u64 = (s_30_5 as u64);
        // C s_30_9: const #1u : u64
        let s_30_9: u64 = 1;
        // D s_30_10: and s_30_8 s_30_9
        let s_30_10: u64 = ((s_30_8) & (s_30_9));
        // D s_30_11: cmp-eq s_30_10 s_30_9
        let s_30_11: bool = ((s_30_10) == (s_30_9));
        // D s_30_12: lsl s_30_8 s_30_6
        let s_30_12: u64 = s_30_8 << s_30_6;
        // D s_30_13: or s_30_7 s_30_12
        let s_30_13: u64 = ((s_30_7) | (s_30_12));
        // D s_30_14: cmpl s_30_12
        let s_30_14: u64 = !s_30_12;
        // D s_30_15: and s_30_7 s_30_14
        let s_30_15: u64 = ((s_30_7) & (s_30_14));
        // D s_30_16: select s_30_11 s_30_13 s_30_15
        let s_30_16: u64 = if s_30_11 { s_30_13 } else { s_30_15 };
        // D s_30_17: cast trunc s_30_16 -> u8
        let s_30_17: bool = ((s_30_16) != 0);
        // D s_30_18: cast zx s_30_17 -> bv
        let s_30_18: Bits = Bits::new(s_30_17 as u128, 1u16);
        // C s_30_19: const #1u : u8
        let s_30_19: bool = true;
        // C s_30_20: cast zx s_30_19 -> bv
        let s_30_20: Bits = Bits::new(s_30_19 as u128, 1u16);
        // D s_30_21: cmp-eq s_30_18 s_30_20
        let s_30_21: bool = ((s_30_18) == (s_30_20));
        // N s_30_22: branch s_30_21 b36 b31
        if s_30_21 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0s : i
        let s_31_0: i128 = 0;
        // D s_31_1: read-var size:u8
        let s_31_1: u8 = fn_state.size;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 2u16);
        // C s_31_3: const #1u : u64
        let s_31_3: u64 = 1;
        // D s_31_4: bit-extract s_31_2 s_31_0 s_31_3
        let s_31_4: Bits = (Bits::new(
            ((s_31_2) >> (s_31_0)).value(),
            u16::try_from(s_31_3).unwrap(),
        ));
        // D s_31_5: cast reint s_31_4 -> u8
        let s_31_5: bool = ((s_31_4.value()) != 0);
        // C s_31_6: const #0s : i
        let s_31_6: i128 = 0;
        // C s_31_7: const #0u : u64
        let s_31_7: u64 = 0;
        // D s_31_8: cast zx s_31_5 -> u64
        let s_31_8: u64 = (s_31_5 as u64);
        // C s_31_9: const #1u : u64
        let s_31_9: u64 = 1;
        // D s_31_10: and s_31_8 s_31_9
        let s_31_10: u64 = ((s_31_8) & (s_31_9));
        // D s_31_11: cmp-eq s_31_10 s_31_9
        let s_31_11: bool = ((s_31_10) == (s_31_9));
        // D s_31_12: lsl s_31_8 s_31_6
        let s_31_12: u64 = s_31_8 << s_31_6;
        // D s_31_13: or s_31_7 s_31_12
        let s_31_13: u64 = ((s_31_7) | (s_31_12));
        // D s_31_14: cmpl s_31_12
        let s_31_14: u64 = !s_31_12;
        // D s_31_15: and s_31_7 s_31_14
        let s_31_15: u64 = ((s_31_7) & (s_31_14));
        // D s_31_16: select s_31_11 s_31_13 s_31_15
        let s_31_16: u64 = if s_31_11 { s_31_13 } else { s_31_15 };
        // D s_31_17: cast trunc s_31_16 -> u8
        let s_31_17: bool = ((s_31_16) != 0);
        // D s_31_18: cast zx s_31_17 -> bv
        let s_31_18: Bits = Bits::new(s_31_17 as u128, 1u16);
        // C s_31_19: const #0u : u8
        let s_31_19: bool = false;
        // C s_31_20: cast zx s_31_19 -> bv
        let s_31_20: Bits = Bits::new(s_31_19 as u128, 1u16);
        // D s_31_21: cmp-eq s_31_18 s_31_20
        let s_31_21: bool = ((s_31_18) == (s_31_20));
        // N s_31_22: branch s_31_21 b35 b32
        if s_31_21 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var S:u8
        let s_32_0: bool = fn_state.S;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // N s_32_5: branch s_32_4 b34 b33
        if s_32_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var Q:u8
        let s_33_0: bool = fn_state.Q;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // D s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // D s_33_3: write-var index <= s_33_2
        fn_state.index = s_33_2;
        // C s_33_4: const #3s : i
        let s_33_4: i128 = 3;
        // D s_33_5: write-var scale <= s_33_4
        fn_state.scale = s_33_4;
        // N s_33_6: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_35_0: read-var Q:u8
        let s_35_0: bool = fn_state.Q;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // D s_35_2: read-var S:u8
        let s_35_2: bool = fn_state.S;
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cast reint s_35_1 -> u128
        let s_35_4: u128 = (s_35_1.value() as u128);
        // D s_35_5: size-of s_35_1
        let s_35_5: u16 = s_35_1.length();
        // D s_35_6: cast reint s_35_3 -> u128
        let s_35_6: u128 = (s_35_3.value() as u128);
        // D s_35_7: size-of s_35_3
        let s_35_7: u16 = s_35_3.length();
        // D s_35_8: lsl s_35_4 s_35_7
        let s_35_8: u128 = s_35_4 << s_35_7;
        // D s_35_9: or s_35_8 s_35_6
        let s_35_9: u128 = ((s_35_8) | (s_35_6));
        // D s_35_10: add s_35_5 s_35_7
        let s_35_10: u16 = (s_35_5 + s_35_7);
        // D s_35_11: create-bits s_35_9 s_35_10
        let s_35_11: Bits = Bits::new(s_35_9, s_35_10);
        // D s_35_12: cast reint s_35_11 -> u8
        let s_35_12: u8 = (s_35_11.value() as u8);
        // D s_35_13: cast zx s_35_12 -> bv
        let s_35_13: Bits = Bits::new(s_35_12 as u128, 2u16);
        // D s_35_14: cast zx s_35_13 -> i
        let s_35_14: i128 = (s_35_13.value() as i128);
        // D s_35_15: write-var index <= s_35_14
        fn_state.index = s_35_14;
        // N s_35_16: jump b5
        return block_5(state, tracer, fn_state);
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
        // N s_37_0: jump b5
        return block_5(state, tracer, fn_state);
    }
}
