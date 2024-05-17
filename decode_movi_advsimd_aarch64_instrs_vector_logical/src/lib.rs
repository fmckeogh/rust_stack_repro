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
use execute_aarch64_instrs_vector_logical::*;
use AdvSIMDExpandImm::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn decode_movi_advsimd_aarch64_instrs_vector_logical<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    h: bool,
    g: bool,
    f: bool,
    e: bool,
    d: bool,
    cmode: u8,
    c: bool,
    b: bool,
    a: bool,
    op: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        rd: i64,
        gs_144381: bool,
        gs_144363: bool,
        ga_250054: u8,
        b__2: u8,
        b__3: u8,
        datasize: i64,
        gs_144387: bool,
        gs_144345: bool,
        b__8: u8,
        b__5: u8,
        imm64: u64,
        gs_144399: bool,
        b__0: u8,
        gs_144393: bool,
        operation: u32,
        b__1: u8,
        ga_250053: i64,
        b__6: u8,
        b__7: u8,
        b__4: u8,
        operationshadow_1087: u32,
        gs_144357: bool,
        gs_144351: bool,
        gs_144375: bool,
        b__9: u8,
        gs_144369: bool,
        Rd: u8,
        h: bool,
        g: bool,
        f: bool,
        e: bool,
        d: bool,
        cmode: u8,
        c: bool,
        b: bool,
        a: bool,
        op: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        h,
        g,
        f,
        e,
        d,
        cmode,
        c,
        b,
        a,
        op,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var rd <= s_0_3
        fn_state.rd = s_0_3;
        // D s_0_5: read-var Q:u8
        let s_0_5: bool = fn_state.Q;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // N s_0_10: branch s_0_9 b61 b1
        if s_0_9 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#250053 <= s_1_0
        fn_state.ga_250053 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#250053:i64
        let s_2_0: i64 = fn_state.ga_250053;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // D s_2_2: read-var cmode:u8
        let s_2_2: u8 = fn_state.cmode;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: read-var op:u8
        let s_2_4: bool = fn_state.op;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: cast reint s_2_5 -> u128
        let s_2_8: u128 = (s_2_5.value() as u128);
        // D s_2_9: size-of s_2_5
        let s_2_9: u16 = s_2_5.length();
        // D s_2_10: lsl s_2_6 s_2_9
        let s_2_10: u128 = s_2_6 << s_2_9;
        // D s_2_11: or s_2_10 s_2_8
        let s_2_11: u128 = ((s_2_10) | (s_2_8));
        // D s_2_12: add s_2_7 s_2_9
        let s_2_12: u16 = (s_2_7 + s_2_9);
        // D s_2_13: create-bits s_2_11 s_2_12
        let s_2_13: Bits = Bits::new(s_2_11, s_2_12);
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: u8 = (s_2_13.value() as u8);
        // D s_2_15: write-var ga#250054 <= s_2_14
        fn_state.ga_250054 = s_2_14;
        // D s_2_16: read-var ga#250054:u8
        let s_2_16: u8 = fn_state.ga_250054;
        // D s_2_17: write-var b__0 <= s_2_16
        fn_state.b__0 = s_2_16;
        // C s_2_18: const #4s : i
        let s_2_18: i128 = 4;
        // D s_2_19: read-var b__0:u8
        let s_2_19: u8 = fn_state.b__0;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 5u16);
        // C s_2_21: const #1s : i64
        let s_2_21: i64 = 1;
        // C s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // C s_2_23: const #0s : i
        let s_2_23: i128 = 0;
        // C s_2_24: add s_2_23 s_2_22
        let s_2_24: i128 = (s_2_23 + s_2_22);
        // D s_2_25: bit-extract s_2_20 s_2_18 s_2_24
        let s_2_25: Bits = (Bits::new(
            ((s_2_20) >> (s_2_18)).value(),
            u16::try_from(s_2_24).unwrap(),
        ));
        // D s_2_26: cast reint s_2_25 -> u8
        let s_2_26: bool = ((s_2_25.value()) != 0);
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 1u16);
        // C s_2_28: const #0u : u8
        let s_2_28: bool = false;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 1u16);
        // D s_2_30: cmp-eq s_2_27 s_2_29
        let s_2_30: bool = ((s_2_27) == (s_2_29));
        // N s_2_31: branch s_2_30 b60 b3
        if s_2_30 {
            return block_60(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#144345 <= s_3_0
        fn_state.gs_144345 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#144345:u8
        let s_4_0: bool = fn_state.gs_144345;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b8 b5
        if s_4_1 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_1: write-var operation <= s_5_0
        fn_state.operation = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var operation:u32
        let s_6_0: u32 = fn_state.operation;
        // D s_6_1: write-var operationshadow#1087 <= s_6_0
        fn_state.operationshadow_1087 = s_6_0;
        // D s_6_2: read-var a:u8
        let s_6_2: bool = fn_state.a;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: read-var b:u8
        let s_6_4: bool = fn_state.b;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u8
        let s_6_14: u8 = (s_6_13.value() as u8);
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 2u16);
        // D s_6_16: read-var c:u8
        let s_6_16: bool = fn_state.c;
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 1u16);
        // D s_6_18: cast reint s_6_15 -> u128
        let s_6_18: u128 = (s_6_15.value() as u128);
        // D s_6_19: size-of s_6_15
        let s_6_19: u16 = s_6_15.length();
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: lsl s_6_18 s_6_21
        let s_6_22: u128 = s_6_18 << s_6_21;
        // D s_6_23: or s_6_22 s_6_20
        let s_6_23: u128 = ((s_6_22) | (s_6_20));
        // D s_6_24: add s_6_19 s_6_21
        let s_6_24: u16 = (s_6_19 + s_6_21);
        // D s_6_25: create-bits s_6_23 s_6_24
        let s_6_25: Bits = Bits::new(s_6_23, s_6_24);
        // D s_6_26: cast reint s_6_25 -> u8
        let s_6_26: u8 = (s_6_25.value() as u8);
        // D s_6_27: cast zx s_6_26 -> bv
        let s_6_27: Bits = Bits::new(s_6_26 as u128, 3u16);
        // D s_6_28: read-var d:u8
        let s_6_28: bool = fn_state.d;
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 1u16);
        // D s_6_30: cast reint s_6_27 -> u128
        let s_6_30: u128 = (s_6_27.value() as u128);
        // D s_6_31: size-of s_6_27
        let s_6_31: u16 = s_6_27.length();
        // D s_6_32: cast reint s_6_29 -> u128
        let s_6_32: u128 = (s_6_29.value() as u128);
        // D s_6_33: size-of s_6_29
        let s_6_33: u16 = s_6_29.length();
        // D s_6_34: lsl s_6_30 s_6_33
        let s_6_34: u128 = s_6_30 << s_6_33;
        // D s_6_35: or s_6_34 s_6_32
        let s_6_35: u128 = ((s_6_34) | (s_6_32));
        // D s_6_36: add s_6_31 s_6_33
        let s_6_36: u16 = (s_6_31 + s_6_33);
        // D s_6_37: create-bits s_6_35 s_6_36
        let s_6_37: Bits = Bits::new(s_6_35, s_6_36);
        // D s_6_38: cast reint s_6_37 -> u8
        let s_6_38: u8 = (s_6_37.value() as u8);
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 4u16);
        // D s_6_40: read-var e:u8
        let s_6_40: bool = fn_state.e;
        // D s_6_41: cast zx s_6_40 -> bv
        let s_6_41: Bits = Bits::new(s_6_40 as u128, 1u16);
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
        // D s_6_52: read-var f:u8
        let s_6_52: bool = fn_state.f;
        // D s_6_53: cast zx s_6_52 -> bv
        let s_6_53: Bits = Bits::new(s_6_52 as u128, 1u16);
        // D s_6_54: cast reint s_6_51 -> u128
        let s_6_54: u128 = (s_6_51.value() as u128);
        // D s_6_55: size-of s_6_51
        let s_6_55: u16 = s_6_51.length();
        // D s_6_56: cast reint s_6_53 -> u128
        let s_6_56: u128 = (s_6_53.value() as u128);
        // D s_6_57: size-of s_6_53
        let s_6_57: u16 = s_6_53.length();
        // D s_6_58: lsl s_6_54 s_6_57
        let s_6_58: u128 = s_6_54 << s_6_57;
        // D s_6_59: or s_6_58 s_6_56
        let s_6_59: u128 = ((s_6_58) | (s_6_56));
        // D s_6_60: add s_6_55 s_6_57
        let s_6_60: u16 = (s_6_55 + s_6_57);
        // D s_6_61: create-bits s_6_59 s_6_60
        let s_6_61: Bits = Bits::new(s_6_59, s_6_60);
        // D s_6_62: cast reint s_6_61 -> u8
        let s_6_62: u8 = (s_6_61.value() as u8);
        // D s_6_63: cast zx s_6_62 -> bv
        let s_6_63: Bits = Bits::new(s_6_62 as u128, 6u16);
        // D s_6_64: read-var g:u8
        let s_6_64: bool = fn_state.g;
        // D s_6_65: cast zx s_6_64 -> bv
        let s_6_65: Bits = Bits::new(s_6_64 as u128, 1u16);
        // D s_6_66: cast reint s_6_63 -> u128
        let s_6_66: u128 = (s_6_63.value() as u128);
        // D s_6_67: size-of s_6_63
        let s_6_67: u16 = s_6_63.length();
        // D s_6_68: cast reint s_6_65 -> u128
        let s_6_68: u128 = (s_6_65.value() as u128);
        // D s_6_69: size-of s_6_65
        let s_6_69: u16 = s_6_65.length();
        // D s_6_70: lsl s_6_66 s_6_69
        let s_6_70: u128 = s_6_66 << s_6_69;
        // D s_6_71: or s_6_70 s_6_68
        let s_6_71: u128 = ((s_6_70) | (s_6_68));
        // D s_6_72: add s_6_67 s_6_69
        let s_6_72: u16 = (s_6_67 + s_6_69);
        // D s_6_73: create-bits s_6_71 s_6_72
        let s_6_73: Bits = Bits::new(s_6_71, s_6_72);
        // D s_6_74: cast reint s_6_73 -> u8
        let s_6_74: u8 = (s_6_73.value() as u8);
        // D s_6_75: cast zx s_6_74 -> bv
        let s_6_75: Bits = Bits::new(s_6_74 as u128, 7u16);
        // D s_6_76: read-var h:u8
        let s_6_76: bool = fn_state.h;
        // D s_6_77: cast zx s_6_76 -> bv
        let s_6_77: Bits = Bits::new(s_6_76 as u128, 1u16);
        // D s_6_78: cast reint s_6_75 -> u128
        let s_6_78: u128 = (s_6_75.value() as u128);
        // D s_6_79: size-of s_6_75
        let s_6_79: u16 = s_6_75.length();
        // D s_6_80: cast reint s_6_77 -> u128
        let s_6_80: u128 = (s_6_77.value() as u128);
        // D s_6_81: size-of s_6_77
        let s_6_81: u16 = s_6_77.length();
        // D s_6_82: lsl s_6_78 s_6_81
        let s_6_82: u128 = s_6_78 << s_6_81;
        // D s_6_83: or s_6_82 s_6_80
        let s_6_83: u128 = ((s_6_82) | (s_6_80));
        // D s_6_84: add s_6_79 s_6_81
        let s_6_84: u16 = (s_6_79 + s_6_81);
        // D s_6_85: create-bits s_6_83 s_6_84
        let s_6_85: Bits = Bits::new(s_6_83, s_6_84);
        // D s_6_86: cast reint s_6_85 -> u8
        let s_6_86: u8 = (s_6_85.value() as u8);
        // D s_6_87: read-var op:u8
        let s_6_87: bool = fn_state.op;
        // D s_6_88: read-var cmode:u8
        let s_6_88: u8 = fn_state.cmode;
        // D s_6_89: call AdvSIMDExpandImm(s_6_87, s_6_88, s_6_86)
        let s_6_89: u64 = AdvSIMDExpandImm(state, tracer, s_6_87, s_6_88, s_6_86);
        // D s_6_90: write-var imm64 <= s_6_89
        fn_state.imm64 = s_6_89;
        // N s_6_91: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i
        let s_7_0: i128 = 64;
        // D s_7_1: read-var datasize:i64
        let s_7_1: i64 = fn_state.datasize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: div s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) / (s_7_0));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: read-var imm64:u64
        let s_7_5: u64 = fn_state.imm64;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_7: cast zx s_7_4 -> i
        let s_7_7: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7 as u64);
        // D s_7_9: call replicate_bits_borealis_internal(s_7_6, s_7_8)
        let s_7_9: Bits = replicate_bits_borealis_internal(state, tracer, s_7_6, s_7_8);
        // D s_7_10: read-var datasize:i64
        let s_7_10: i64 = fn_state.datasize;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: read-var operationshadow#1087:u32
        let s_7_13: u32 = fn_state.operationshadow_1087;
        // D s_7_14: read-var rd:i64
        let s_7_14: i64 = fn_state.rd;
        // D s_7_15: call execute_aarch64_instrs_vector_logical(s_7_12, s_7_9, s_7_13, s_7_14)
        let s_7_15: () = execute_aarch64_instrs_vector_logical(
            state,
            tracer,
            s_7_12,
            s_7_9,
            s_7_13,
            s_7_14,
        );
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#250054:u8
        let s_8_0: u8 = fn_state.ga_250054;
        // D s_8_1: write-var b__1 <= s_8_0
        fn_state.b__1 = s_8_0;
        // C s_8_2: const #4s : i
        let s_8_2: i128 = 4;
        // D s_8_3: read-var b__1:u8
        let s_8_3: u8 = fn_state.b__1;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 5u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #0s : i
        let s_8_7: i128 = 0;
        // C s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: bit-extract s_8_4 s_8_2 s_8_8
        let s_8_9: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_8).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u8
        let s_8_10: bool = ((s_8_9.value()) != 0);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 1u16);
        // C s_8_12: const #0u : u8
        let s_8_12: bool = false;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 1u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // N s_8_15: branch s_8_14 b59 b9
        if s_8_14 {
            return block_59(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#144351 <= s_9_0
        fn_state.gs_144351 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#144351:u8
        let s_10_0: bool = fn_state.gs_144351;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b12 b11
        if s_10_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u32
        let s_11_0: u32 = 1;
        // D s_11_1: write-var operation <= s_11_0
        fn_state.operation = s_11_0;
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#250054:u8
        let s_12_0: u8 = fn_state.ga_250054;
        // D s_12_1: write-var b__2 <= s_12_0
        fn_state.b__2 = s_12_0;
        // C s_12_2: const #4s : i
        let s_12_2: i128 = 4;
        // D s_12_3: read-var b__2:u8
        let s_12_3: u8 = fn_state.b__2;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 5u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #0s : i
        let s_12_7: i128 = 0;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_2 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_2)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: bool = ((s_12_9.value()) != 0);
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 1u16);
        // C s_12_12: const #0u : u8
        let s_12_12: bool = false;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 1u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b58 b13
        if s_12_14 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#144357 <= s_13_0
        fn_state.gs_144357 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#144357:u8
        let s_14_0: bool = fn_state.gs_144357;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: write-var operation <= s_15_0
        fn_state.operation = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#250054:u8
        let s_16_0: u8 = fn_state.ga_250054;
        // D s_16_1: write-var b__3 <= s_16_0
        fn_state.b__3 = s_16_0;
        // C s_16_2: const #4s : i
        let s_16_2: i128 = 4;
        // D s_16_3: read-var b__3:u8
        let s_16_3: u8 = fn_state.b__3;
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 5u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #0s : i
        let s_16_7: i128 = 0;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_2 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_2)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u8
        let s_16_10: bool = ((s_16_9.value()) != 0);
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 1u16);
        // C s_16_12: const #0u : u8
        let s_16_12: bool = false;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 1u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // N s_16_15: branch s_16_14 b57 b17
        if s_16_14 {
            return block_57(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#144363 <= s_17_0
        fn_state.gs_144363 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#144363:u8
        let s_18_0: bool = fn_state.gs_144363;
        // D s_18_1: not s_18_0
        let s_18_1: bool = !s_18_0;
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
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
        // C s_19_0: const #3u : u32
        let s_19_0: u32 = 3;
        // D s_19_1: write-var operation <= s_19_0
        fn_state.operation = s_19_0;
        // N s_19_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#250054:u8
        let s_20_0: u8 = fn_state.ga_250054;
        // D s_20_1: write-var b__4 <= s_20_0
        fn_state.b__4 = s_20_0;
        // C s_20_2: const #3s : i
        let s_20_2: i128 = 3;
        // D s_20_3: read-var b__4:u8
        let s_20_3: u8 = fn_state.b__4;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 5u16);
        // C s_20_5: const #1s : i64
        let s_20_5: i64 = 1;
        // C s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // C s_20_7: const #1s : i
        let s_20_7: i128 = 1;
        // C s_20_8: add s_20_7 s_20_6
        let s_20_8: i128 = (s_20_7 + s_20_6);
        // D s_20_9: bit-extract s_20_4 s_20_2 s_20_8
        let s_20_9: Bits = (Bits::new(
            ((s_20_4) >> (s_20_2)).value(),
            u16::try_from(s_20_8).unwrap(),
        ));
        // D s_20_10: cast reint s_20_9 -> u8
        let s_20_10: u8 = (s_20_9.value() as u8);
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 2u16);
        // C s_20_12: const #2u : u8
        let s_20_12: u8 = 2;
        // C s_20_13: cast zx s_20_12 -> bv
        let s_20_13: Bits = Bits::new(s_20_12 as u128, 2u16);
        // D s_20_14: cmp-eq s_20_11 s_20_13
        let s_20_14: bool = ((s_20_11) == (s_20_13));
        // N s_20_15: branch s_20_14 b56 b21
        if s_20_14 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#144369 <= s_21_0
        fn_state.gs_144369 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#144369:u8
        let s_22_0: bool = fn_state.gs_144369;
        // D s_22_1: not s_22_0
        let s_22_1: bool = !s_22_0;
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
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
        // C s_23_0: const #0u : u32
        let s_23_0: u32 = 0;
        // D s_23_1: write-var operation <= s_23_0
        fn_state.operation = s_23_0;
        // N s_23_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#250054:u8
        let s_24_0: u8 = fn_state.ga_250054;
        // D s_24_1: write-var b__5 <= s_24_0
        fn_state.b__5 = s_24_0;
        // C s_24_2: const #3s : i
        let s_24_2: i128 = 3;
        // D s_24_3: read-var b__5:u8
        let s_24_3: u8 = fn_state.b__5;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 5u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #1s : i
        let s_24_7: i128 = 1;
        // C s_24_8: add s_24_7 s_24_6
        let s_24_8: i128 = (s_24_7 + s_24_6);
        // D s_24_9: bit-extract s_24_4 s_24_2 s_24_8
        let s_24_9: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_8).unwrap(),
        ));
        // D s_24_10: cast reint s_24_9 -> u8
        let s_24_10: u8 = (s_24_9.value() as u8);
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 2u16);
        // C s_24_12: const #2u : u8
        let s_24_12: u8 = 2;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 2u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // N s_24_15: branch s_24_14 b55 b25
        if s_24_14 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#144375 <= s_25_0
        fn_state.gs_144375 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#144375:u8
        let s_26_0: bool = fn_state.gs_144375;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b28 b27
        if s_26_1 {
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
        // C s_27_0: const #1u : u32
        let s_27_0: u32 = 1;
        // D s_27_1: write-var operation <= s_27_0
        fn_state.operation = s_27_0;
        // N s_27_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ga#250054:u8
        let s_28_0: u8 = fn_state.ga_250054;
        // D s_28_1: write-var b__6 <= s_28_0
        fn_state.b__6 = s_28_0;
        // C s_28_2: const #3s : i
        let s_28_2: i128 = 3;
        // D s_28_3: read-var b__6:u8
        let s_28_3: u8 = fn_state.b__6;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 5u16);
        // C s_28_5: const #1s : i64
        let s_28_5: i64 = 1;
        // C s_28_6: cast zx s_28_5 -> i
        let s_28_6: i128 = (i128::try_from(s_28_5).unwrap());
        // C s_28_7: const #1s : i
        let s_28_7: i128 = 1;
        // C s_28_8: add s_28_7 s_28_6
        let s_28_8: i128 = (s_28_7 + s_28_6);
        // D s_28_9: bit-extract s_28_4 s_28_2 s_28_8
        let s_28_9: Bits = (Bits::new(
            ((s_28_4) >> (s_28_2)).value(),
            u16::try_from(s_28_8).unwrap(),
        ));
        // D s_28_10: cast reint s_28_9 -> u8
        let s_28_10: u8 = (s_28_9.value() as u8);
        // D s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 2u16);
        // C s_28_12: const #2u : u8
        let s_28_12: u8 = 2;
        // C s_28_13: cast zx s_28_12 -> bv
        let s_28_13: Bits = Bits::new(s_28_12 as u128, 2u16);
        // D s_28_14: cmp-eq s_28_11 s_28_13
        let s_28_14: bool = ((s_28_11) == (s_28_13));
        // N s_28_15: branch s_28_14 b54 b29
        if s_28_14 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#144381 <= s_29_0
        fn_state.gs_144381 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#144381:u8
        let s_30_0: bool = fn_state.gs_144381;
        // D s_30_1: not s_30_0
        let s_30_1: bool = !s_30_0;
        // N s_30_2: branch s_30_1 b32 b31
        if s_30_1 {
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
        // C s_31_0: const #2u : u32
        let s_31_0: u32 = 2;
        // D s_31_1: write-var operation <= s_31_0
        fn_state.operation = s_31_0;
        // N s_31_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ga#250054:u8
        let s_32_0: u8 = fn_state.ga_250054;
        // D s_32_1: write-var b__7 <= s_32_0
        fn_state.b__7 = s_32_0;
        // C s_32_2: const #3s : i
        let s_32_2: i128 = 3;
        // D s_32_3: read-var b__7:u8
        let s_32_3: u8 = fn_state.b__7;
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 5u16);
        // C s_32_5: const #1s : i64
        let s_32_5: i64 = 1;
        // C s_32_6: cast zx s_32_5 -> i
        let s_32_6: i128 = (i128::try_from(s_32_5).unwrap());
        // C s_32_7: const #1s : i
        let s_32_7: i128 = 1;
        // C s_32_8: add s_32_7 s_32_6
        let s_32_8: i128 = (s_32_7 + s_32_6);
        // D s_32_9: bit-extract s_32_4 s_32_2 s_32_8
        let s_32_9: Bits = (Bits::new(
            ((s_32_4) >> (s_32_2)).value(),
            u16::try_from(s_32_8).unwrap(),
        ));
        // D s_32_10: cast reint s_32_9 -> u8
        let s_32_10: u8 = (s_32_9.value() as u8);
        // D s_32_11: cast zx s_32_10 -> bv
        let s_32_11: Bits = Bits::new(s_32_10 as u128, 2u16);
        // C s_32_12: const #2u : u8
        let s_32_12: u8 = 2;
        // C s_32_13: cast zx s_32_12 -> bv
        let s_32_13: Bits = Bits::new(s_32_12 as u128, 2u16);
        // D s_32_14: cmp-eq s_32_11 s_32_13
        let s_32_14: bool = ((s_32_11) == (s_32_13));
        // N s_32_15: branch s_32_14 b53 b33
        if s_32_14 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#144387 <= s_33_0
        fn_state.gs_144387 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#144387:u8
        let s_34_0: bool = fn_state.gs_144387;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b36 b35
        if s_34_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #3u : u32
        let s_35_0: u32 = 3;
        // D s_35_1: write-var operation <= s_35_0
        fn_state.operation = s_35_0;
        // N s_35_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var ga#250054:u8
        let s_36_0: u8 = fn_state.ga_250054;
        // D s_36_1: write-var b__8 <= s_36_0
        fn_state.b__8 = s_36_0;
        // C s_36_2: const #2s : i
        let s_36_2: i128 = 2;
        // D s_36_3: read-var b__8:u8
        let s_36_3: u8 = fn_state.b__8;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 5u16);
        // C s_36_5: const #1s : i64
        let s_36_5: i64 = 1;
        // C s_36_6: cast zx s_36_5 -> i
        let s_36_6: i128 = (i128::try_from(s_36_5).unwrap());
        // C s_36_7: const #2s : i
        let s_36_7: i128 = 2;
        // C s_36_8: add s_36_7 s_36_6
        let s_36_8: i128 = (s_36_7 + s_36_6);
        // D s_36_9: bit-extract s_36_4 s_36_2 s_36_8
        let s_36_9: Bits = (Bits::new(
            ((s_36_4) >> (s_36_2)).value(),
            u16::try_from(s_36_8).unwrap(),
        ));
        // D s_36_10: cast reint s_36_9 -> u8
        let s_36_10: u8 = (s_36_9.value() as u8);
        // D s_36_11: cast zx s_36_10 -> bv
        let s_36_11: Bits = Bits::new(s_36_10 as u128, 3u16);
        // C s_36_12: const #6u : u8
        let s_36_12: u8 = 6;
        // C s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 3u16);
        // D s_36_14: cmp-eq s_36_11 s_36_13
        let s_36_14: bool = ((s_36_11) == (s_36_13));
        // N s_36_15: branch s_36_14 b52 b37
        if s_36_14 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#144393 <= s_37_0
        fn_state.gs_144393 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#144393:u8
        let s_38_0: bool = fn_state.gs_144393;
        // D s_38_1: not s_38_0
        let s_38_1: bool = !s_38_0;
        // N s_38_2: branch s_38_1 b40 b39
        if s_38_1 {
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
        // C s_39_0: const #0u : u32
        let s_39_0: u32 = 0;
        // D s_39_1: write-var operation <= s_39_0
        fn_state.operation = s_39_0;
        // N s_39_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var ga#250054:u8
        let s_40_0: u8 = fn_state.ga_250054;
        // D s_40_1: write-var b__9 <= s_40_0
        fn_state.b__9 = s_40_0;
        // C s_40_2: const #2s : i
        let s_40_2: i128 = 2;
        // D s_40_3: read-var b__9:u8
        let s_40_3: u8 = fn_state.b__9;
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 5u16);
        // C s_40_5: const #1s : i64
        let s_40_5: i64 = 1;
        // C s_40_6: cast zx s_40_5 -> i
        let s_40_6: i128 = (i128::try_from(s_40_5).unwrap());
        // C s_40_7: const #2s : i
        let s_40_7: i128 = 2;
        // C s_40_8: add s_40_7 s_40_6
        let s_40_8: i128 = (s_40_7 + s_40_6);
        // D s_40_9: bit-extract s_40_4 s_40_2 s_40_8
        let s_40_9: Bits = (Bits::new(
            ((s_40_4) >> (s_40_2)).value(),
            u16::try_from(s_40_8).unwrap(),
        ));
        // D s_40_10: cast reint s_40_9 -> u8
        let s_40_10: u8 = (s_40_9.value() as u8);
        // D s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 3u16);
        // C s_40_12: const #6u : u8
        let s_40_12: u8 = 6;
        // C s_40_13: cast zx s_40_12 -> bv
        let s_40_13: Bits = Bits::new(s_40_12 as u128, 3u16);
        // D s_40_14: cmp-eq s_40_11 s_40_13
        let s_40_14: bool = ((s_40_11) == (s_40_13));
        // N s_40_15: branch s_40_14 b51 b41
        if s_40_14 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#144399 <= s_41_0
        fn_state.gs_144399 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#144399:u8
        let s_42_0: bool = fn_state.gs_144399;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // N s_42_2: branch s_42_1 b44 b43
        if s_42_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u32
        let s_43_0: u32 = 1;
        // D s_43_1: write-var operation <= s_43_0
        fn_state.operation = s_43_0;
        // N s_43_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var ga#250054:u8
        let s_44_0: u8 = fn_state.ga_250054;
        // C s_44_1: const #1s : i
        let s_44_1: i128 = 1;
        // D s_44_2: cast zx s_44_0 -> bv
        let s_44_2: Bits = Bits::new(s_44_0 as u128, 5u16);
        // C s_44_3: const #1s : i64
        let s_44_3: i64 = 1;
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // C s_44_5: const #3s : i
        let s_44_5: i128 = 3;
        // C s_44_6: add s_44_5 s_44_4
        let s_44_6: i128 = (s_44_5 + s_44_4);
        // D s_44_7: bit-extract s_44_2 s_44_1 s_44_6
        let s_44_7: Bits = (Bits::new(
            ((s_44_2) >> (s_44_1)).value(),
            u16::try_from(s_44_6).unwrap(),
        ));
        // D s_44_8: cast reint s_44_7 -> u8
        let s_44_8: u8 = (s_44_7.value() as u8);
        // D s_44_9: cast zx s_44_8 -> bv
        let s_44_9: Bits = Bits::new(s_44_8 as u128, 4u16);
        // C s_44_10: const #14u : u8
        let s_44_10: u8 = 14;
        // C s_44_11: cast zx s_44_10 -> bv
        let s_44_11: Bits = Bits::new(s_44_10 as u128, 4u16);
        // D s_44_12: cmp-eq s_44_9 s_44_11
        let s_44_12: bool = ((s_44_9) == (s_44_11));
        // D s_44_13: not s_44_12
        let s_44_13: bool = !s_44_12;
        // N s_44_14: branch s_44_13 b46 b45
        if s_44_13 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u32
        let s_45_0: u32 = 0;
        // D s_45_1: write-var operation <= s_45_0
        fn_state.operation = s_45_0;
        // N s_45_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var ga#250054:u8
        let s_46_0: u8 = fn_state.ga_250054;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 5u16);
        // C s_46_2: const #30u : u8
        let s_46_2: u8 = 30;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 5u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b48 b47
        if s_46_5 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u32
        let s_47_0: u32 = 0;
        // D s_47_1: write-var operation <= s_47_0
        fn_state.operation = s_47_0;
        // N s_47_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var Q:u8
        let s_48_0: bool = fn_state.Q;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // N s_48_5: branch s_48_4 b50 b49
        if s_48_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u32
        let s_49_0: u32 = 0;
        // D s_49_1: write-var operation <= s_49_0
        fn_state.operation = s_49_0;
        // N s_49_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0s : i
        let s_51_0: i128 = 0;
        // D s_51_1: read-var b__9:u8
        let s_51_1: u8 = fn_state.b__9;
        // D s_51_2: cast zx s_51_1 -> bv
        let s_51_2: Bits = Bits::new(s_51_1 as u128, 5u16);
        // C s_51_3: const #1s : i64
        let s_51_3: i64 = 1;
        // C s_51_4: cast zx s_51_3 -> i
        let s_51_4: i128 = (i128::try_from(s_51_3).unwrap());
        // C s_51_5: const #0s : i
        let s_51_5: i128 = 0;
        // C s_51_6: add s_51_5 s_51_4
        let s_51_6: i128 = (s_51_5 + s_51_4);
        // D s_51_7: bit-extract s_51_2 s_51_0 s_51_6
        let s_51_7: Bits = (Bits::new(
            ((s_51_2) >> (s_51_0)).value(),
            u16::try_from(s_51_6).unwrap(),
        ));
        // D s_51_8: cast reint s_51_7 -> u8
        let s_51_8: bool = ((s_51_7.value()) != 0);
        // D s_51_9: cast zx s_51_8 -> bv
        let s_51_9: Bits = Bits::new(s_51_8 as u128, 1u16);
        // C s_51_10: const #1u : u8
        let s_51_10: bool = true;
        // C s_51_11: cast zx s_51_10 -> bv
        let s_51_11: Bits = Bits::new(s_51_10 as u128, 1u16);
        // D s_51_12: cmp-eq s_51_9 s_51_11
        let s_51_12: bool = ((s_51_9) == (s_51_11));
        // D s_51_13: write-var gs#144399 <= s_51_12
        fn_state.gs_144399 = s_51_12;
        // N s_51_14: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0s : i
        let s_52_0: i128 = 0;
        // D s_52_1: read-var b__8:u8
        let s_52_1: u8 = fn_state.b__8;
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 5u16);
        // C s_52_3: const #1s : i64
        let s_52_3: i64 = 1;
        // C s_52_4: cast zx s_52_3 -> i
        let s_52_4: i128 = (i128::try_from(s_52_3).unwrap());
        // C s_52_5: const #0s : i
        let s_52_5: i128 = 0;
        // C s_52_6: add s_52_5 s_52_4
        let s_52_6: i128 = (s_52_5 + s_52_4);
        // D s_52_7: bit-extract s_52_2 s_52_0 s_52_6
        let s_52_7: Bits = (Bits::new(
            ((s_52_2) >> (s_52_0)).value(),
            u16::try_from(s_52_6).unwrap(),
        ));
        // D s_52_8: cast reint s_52_7 -> u8
        let s_52_8: bool = ((s_52_7.value()) != 0);
        // D s_52_9: cast zx s_52_8 -> bv
        let s_52_9: Bits = Bits::new(s_52_8 as u128, 1u16);
        // C s_52_10: const #0u : u8
        let s_52_10: bool = false;
        // C s_52_11: cast zx s_52_10 -> bv
        let s_52_11: Bits = Bits::new(s_52_10 as u128, 1u16);
        // D s_52_12: cmp-eq s_52_9 s_52_11
        let s_52_12: bool = ((s_52_9) == (s_52_11));
        // D s_52_13: write-var gs#144393 <= s_52_12
        fn_state.gs_144393 = s_52_12;
        // N s_52_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0s : i
        let s_53_0: i128 = 0;
        // D s_53_1: read-var b__7:u8
        let s_53_1: u8 = fn_state.b__7;
        // D s_53_2: cast zx s_53_1 -> bv
        let s_53_2: Bits = Bits::new(s_53_1 as u128, 5u16);
        // C s_53_3: const #1s : i64
        let s_53_3: i64 = 1;
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #1s : i
        let s_53_5: i128 = 1;
        // C s_53_6: add s_53_5 s_53_4
        let s_53_6: i128 = (s_53_5 + s_53_4);
        // D s_53_7: bit-extract s_53_2 s_53_0 s_53_6
        let s_53_7: Bits = (Bits::new(
            ((s_53_2) >> (s_53_0)).value(),
            u16::try_from(s_53_6).unwrap(),
        ));
        // D s_53_8: cast reint s_53_7 -> u8
        let s_53_8: u8 = (s_53_7.value() as u8);
        // D s_53_9: cast zx s_53_8 -> bv
        let s_53_9: Bits = Bits::new(s_53_8 as u128, 2u16);
        // C s_53_10: const #3u : u8
        let s_53_10: u8 = 3;
        // C s_53_11: cast zx s_53_10 -> bv
        let s_53_11: Bits = Bits::new(s_53_10 as u128, 2u16);
        // D s_53_12: cmp-eq s_53_9 s_53_11
        let s_53_12: bool = ((s_53_9) == (s_53_11));
        // D s_53_13: write-var gs#144387 <= s_53_12
        fn_state.gs_144387 = s_53_12;
        // N s_53_14: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0s : i
        let s_54_0: i128 = 0;
        // D s_54_1: read-var b__6:u8
        let s_54_1: u8 = fn_state.b__6;
        // D s_54_2: cast zx s_54_1 -> bv
        let s_54_2: Bits = Bits::new(s_54_1 as u128, 5u16);
        // C s_54_3: const #1s : i64
        let s_54_3: i64 = 1;
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #1s : i
        let s_54_5: i128 = 1;
        // C s_54_6: add s_54_5 s_54_4
        let s_54_6: i128 = (s_54_5 + s_54_4);
        // D s_54_7: bit-extract s_54_2 s_54_0 s_54_6
        let s_54_7: Bits = (Bits::new(
            ((s_54_2) >> (s_54_0)).value(),
            u16::try_from(s_54_6).unwrap(),
        ));
        // D s_54_8: cast reint s_54_7 -> u8
        let s_54_8: u8 = (s_54_7.value() as u8);
        // D s_54_9: cast zx s_54_8 -> bv
        let s_54_9: Bits = Bits::new(s_54_8 as u128, 2u16);
        // C s_54_10: const #2u : u8
        let s_54_10: u8 = 2;
        // C s_54_11: cast zx s_54_10 -> bv
        let s_54_11: Bits = Bits::new(s_54_10 as u128, 2u16);
        // D s_54_12: cmp-eq s_54_9 s_54_11
        let s_54_12: bool = ((s_54_9) == (s_54_11));
        // D s_54_13: write-var gs#144381 <= s_54_12
        fn_state.gs_144381 = s_54_12;
        // N s_54_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0s : i
        let s_55_0: i128 = 0;
        // D s_55_1: read-var b__5:u8
        let s_55_1: u8 = fn_state.b__5;
        // D s_55_2: cast zx s_55_1 -> bv
        let s_55_2: Bits = Bits::new(s_55_1 as u128, 5u16);
        // C s_55_3: const #1s : i64
        let s_55_3: i64 = 1;
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #1s : i
        let s_55_5: i128 = 1;
        // C s_55_6: add s_55_5 s_55_4
        let s_55_6: i128 = (s_55_5 + s_55_4);
        // D s_55_7: bit-extract s_55_2 s_55_0 s_55_6
        let s_55_7: Bits = (Bits::new(
            ((s_55_2) >> (s_55_0)).value(),
            u16::try_from(s_55_6).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: u8 = (s_55_7.value() as u8);
        // D s_55_9: cast zx s_55_8 -> bv
        let s_55_9: Bits = Bits::new(s_55_8 as u128, 2u16);
        // C s_55_10: const #1u : u8
        let s_55_10: u8 = 1;
        // C s_55_11: cast zx s_55_10 -> bv
        let s_55_11: Bits = Bits::new(s_55_10 as u128, 2u16);
        // D s_55_12: cmp-eq s_55_9 s_55_11
        let s_55_12: bool = ((s_55_9) == (s_55_11));
        // D s_55_13: write-var gs#144375 <= s_55_12
        fn_state.gs_144375 = s_55_12;
        // N s_55_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0s : i
        let s_56_0: i128 = 0;
        // D s_56_1: read-var b__4:u8
        let s_56_1: u8 = fn_state.b__4;
        // D s_56_2: cast zx s_56_1 -> bv
        let s_56_2: Bits = Bits::new(s_56_1 as u128, 5u16);
        // C s_56_3: const #1s : i64
        let s_56_3: i64 = 1;
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #1s : i
        let s_56_5: i128 = 1;
        // C s_56_6: add s_56_5 s_56_4
        let s_56_6: i128 = (s_56_5 + s_56_4);
        // D s_56_7: bit-extract s_56_2 s_56_0 s_56_6
        let s_56_7: Bits = (Bits::new(
            ((s_56_2) >> (s_56_0)).value(),
            u16::try_from(s_56_6).unwrap(),
        ));
        // D s_56_8: cast reint s_56_7 -> u8
        let s_56_8: u8 = (s_56_7.value() as u8);
        // D s_56_9: cast zx s_56_8 -> bv
        let s_56_9: Bits = Bits::new(s_56_8 as u128, 2u16);
        // C s_56_10: const #0u : u8
        let s_56_10: u8 = 0;
        // C s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 2u16);
        // D s_56_12: cmp-eq s_56_9 s_56_11
        let s_56_12: bool = ((s_56_9) == (s_56_11));
        // D s_56_13: write-var gs#144369 <= s_56_12
        fn_state.gs_144369 = s_56_12;
        // N s_56_14: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0s : i
        let s_57_0: i128 = 0;
        // D s_57_1: read-var b__3:u8
        let s_57_1: u8 = fn_state.b__3;
        // D s_57_2: cast zx s_57_1 -> bv
        let s_57_2: Bits = Bits::new(s_57_1 as u128, 5u16);
        // C s_57_3: const #1s : i64
        let s_57_3: i64 = 1;
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #1s : i
        let s_57_5: i128 = 1;
        // C s_57_6: add s_57_5 s_57_4
        let s_57_6: i128 = (s_57_5 + s_57_4);
        // D s_57_7: bit-extract s_57_2 s_57_0 s_57_6
        let s_57_7: Bits = (Bits::new(
            ((s_57_2) >> (s_57_0)).value(),
            u16::try_from(s_57_6).unwrap(),
        ));
        // D s_57_8: cast reint s_57_7 -> u8
        let s_57_8: u8 = (s_57_7.value() as u8);
        // D s_57_9: cast zx s_57_8 -> bv
        let s_57_9: Bits = Bits::new(s_57_8 as u128, 2u16);
        // C s_57_10: const #3u : u8
        let s_57_10: u8 = 3;
        // C s_57_11: cast zx s_57_10 -> bv
        let s_57_11: Bits = Bits::new(s_57_10 as u128, 2u16);
        // D s_57_12: cmp-eq s_57_9 s_57_11
        let s_57_12: bool = ((s_57_9) == (s_57_11));
        // D s_57_13: write-var gs#144363 <= s_57_12
        fn_state.gs_144363 = s_57_12;
        // N s_57_14: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0s : i
        let s_58_0: i128 = 0;
        // D s_58_1: read-var b__2:u8
        let s_58_1: u8 = fn_state.b__2;
        // D s_58_2: cast zx s_58_1 -> bv
        let s_58_2: Bits = Bits::new(s_58_1 as u128, 5u16);
        // C s_58_3: const #1s : i64
        let s_58_3: i64 = 1;
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // C s_58_5: const #1s : i
        let s_58_5: i128 = 1;
        // C s_58_6: add s_58_5 s_58_4
        let s_58_6: i128 = (s_58_5 + s_58_4);
        // D s_58_7: bit-extract s_58_2 s_58_0 s_58_6
        let s_58_7: Bits = (Bits::new(
            ((s_58_2) >> (s_58_0)).value(),
            u16::try_from(s_58_6).unwrap(),
        ));
        // D s_58_8: cast reint s_58_7 -> u8
        let s_58_8: u8 = (s_58_7.value() as u8);
        // D s_58_9: cast zx s_58_8 -> bv
        let s_58_9: Bits = Bits::new(s_58_8 as u128, 2u16);
        // C s_58_10: const #2u : u8
        let s_58_10: u8 = 2;
        // C s_58_11: cast zx s_58_10 -> bv
        let s_58_11: Bits = Bits::new(s_58_10 as u128, 2u16);
        // D s_58_12: cmp-eq s_58_9 s_58_11
        let s_58_12: bool = ((s_58_9) == (s_58_11));
        // D s_58_13: write-var gs#144357 <= s_58_12
        fn_state.gs_144357 = s_58_12;
        // N s_58_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0s : i
        let s_59_0: i128 = 0;
        // D s_59_1: read-var b__1:u8
        let s_59_1: u8 = fn_state.b__1;
        // D s_59_2: cast zx s_59_1 -> bv
        let s_59_2: Bits = Bits::new(s_59_1 as u128, 5u16);
        // C s_59_3: const #1s : i64
        let s_59_3: i64 = 1;
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #1s : i
        let s_59_5: i128 = 1;
        // C s_59_6: add s_59_5 s_59_4
        let s_59_6: i128 = (s_59_5 + s_59_4);
        // D s_59_7: bit-extract s_59_2 s_59_0 s_59_6
        let s_59_7: Bits = (Bits::new(
            ((s_59_2) >> (s_59_0)).value(),
            u16::try_from(s_59_6).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // D s_59_9: cast zx s_59_8 -> bv
        let s_59_9: Bits = Bits::new(s_59_8 as u128, 2u16);
        // C s_59_10: const #1u : u8
        let s_59_10: u8 = 1;
        // C s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 2u16);
        // D s_59_12: cmp-eq s_59_9 s_59_11
        let s_59_12: bool = ((s_59_9) == (s_59_11));
        // D s_59_13: write-var gs#144351 <= s_59_12
        fn_state.gs_144351 = s_59_12;
        // N s_59_14: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0s : i
        let s_60_0: i128 = 0;
        // D s_60_1: read-var b__0:u8
        let s_60_1: u8 = fn_state.b__0;
        // D s_60_2: cast zx s_60_1 -> bv
        let s_60_2: Bits = Bits::new(s_60_1 as u128, 5u16);
        // C s_60_3: const #1s : i64
        let s_60_3: i64 = 1;
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #1s : i
        let s_60_5: i128 = 1;
        // C s_60_6: add s_60_5 s_60_4
        let s_60_6: i128 = (s_60_5 + s_60_4);
        // D s_60_7: bit-extract s_60_2 s_60_0 s_60_6
        let s_60_7: Bits = (Bits::new(
            ((s_60_2) >> (s_60_0)).value(),
            u16::try_from(s_60_6).unwrap(),
        ));
        // D s_60_8: cast reint s_60_7 -> u8
        let s_60_8: u8 = (s_60_7.value() as u8);
        // D s_60_9: cast zx s_60_8 -> bv
        let s_60_9: Bits = Bits::new(s_60_8 as u128, 2u16);
        // C s_60_10: const #0u : u8
        let s_60_10: u8 = 0;
        // C s_60_11: cast zx s_60_10 -> bv
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 2u16);
        // D s_60_12: cmp-eq s_60_9 s_60_11
        let s_60_12: bool = ((s_60_9) == (s_60_11));
        // D s_60_13: write-var gs#144345 <= s_60_12
        fn_state.gs_144345 = s_60_12;
        // N s_60_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #128s : i64
        let s_61_0: i64 = 128;
        // D s_61_1: write-var ga#250053 <= s_61_0
        fn_state.ga_250053 = s_61_0;
        // N s_61_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
