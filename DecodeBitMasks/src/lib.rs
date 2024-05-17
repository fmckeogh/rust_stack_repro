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
use replicate_bits_borealis_internal::*;
use zext_ones::*;
use HighestSetBit::*;
use u__id::*;
use integer_subrange::*;
use ROR::*;
use common::*;
pub fn DecodeBitMasks<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immN: bool,
    imms: u8,
    immr: u8,
    immediate: bool,
    M: i128,
) -> ProductTypebc91b195b0b2a883 {
    #[derive(Default)]
    struct FunctionState {
        levels: u8,
        len: i128,
        gs_22476: bool,
        Mshadow_378: i128,
        return_value: ProductTypebc91b195b0b2a883,
        immN: bool,
        imms: u8,
        immr: u8,
        immediate: bool,
        M: i128,
    }
    let fn_state = FunctionState {
        immN,
        imms,
        immr,
        immediate,
        M,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // D s_0_0: read-var M:i
        let s_0_0: i128 = fn_state.M;
        // D s_0_1: write-var Mshadow#378 <= s_0_0
        fn_state.Mshadow_378 = s_0_0;
        // D s_0_2: read-var imms:u8
        let s_0_2: u8 = fn_state.imms;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 6u16);
        // D s_0_4: not s_0_3
        let s_0_4: Bits = !s_0_3;
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: u8 = (s_0_4.value() as u8);
        // D s_0_6: read-var immN:u8
        let s_0_6: bool = fn_state.immN;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 1u16);
        // D s_0_8: cast zx s_0_5 -> bv
        let s_0_8: Bits = Bits::new(s_0_5 as u128, 6u16);
        // D s_0_9: cast reint s_0_7 -> u128
        let s_0_9: u128 = (s_0_7.value() as u128);
        // D s_0_10: size-of s_0_7
        let s_0_10: u16 = s_0_7.length();
        // D s_0_11: cast reint s_0_8 -> u128
        let s_0_11: u128 = (s_0_8.value() as u128);
        // D s_0_12: size-of s_0_8
        let s_0_12: u16 = s_0_8.length();
        // D s_0_13: lsl s_0_9 s_0_12
        let s_0_13: u128 = s_0_9 << s_0_12;
        // D s_0_14: or s_0_13 s_0_11
        let s_0_14: u128 = ((s_0_13) | (s_0_11));
        // D s_0_15: add s_0_10 s_0_12
        let s_0_15: u16 = (s_0_10 + s_0_12);
        // D s_0_16: create-bits s_0_14 s_0_15
        let s_0_16: Bits = Bits::new(s_0_14, s_0_15);
        // D s_0_17: cast reint s_0_16 -> u8
        let s_0_17: u8 = (s_0_16.value() as u8);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 7u16);
        // D s_0_19: call HighestSetBit(s_0_18)
        let s_0_19: i128 = HighestSetBit(state, tracer, s_0_18);
        // D s_0_20: write-var len <= s_0_19
        fn_state.len = s_0_19;
        // C s_0_21: const #1s : i
        let s_0_21: i128 = 1;
        // D s_0_22: read-var len:i
        let s_0_22: i128 = fn_state.len;
        // D s_0_23: cmp-lt s_0_22 s_0_21
        let s_0_23: bool = ((s_0_22) < (s_0_21));
        // N s_0_24: branch s_0_23 b8 b1
        if s_0_23 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // C s_1_0: const #1s : i
        let s_1_0: i128 = 1;
        // D s_1_1: read-var len:i
        let s_1_1: i128 = fn_state.len;
        // D s_1_2: lsl s_1_0 s_1_1
        let s_1_2: i128 = s_1_0 << s_1_1;
        // D s_1_3: read-var Mshadow#378:i
        let s_1_3: i128 = fn_state.Mshadow_378;
        // D s_1_4: cmp-ge s_1_3 s_1_2
        let s_1_4: bool = ((s_1_3) >= (s_1_2));
        // N s_1_5: assert s_1_4
        let s_1_5: () = assert!(s_1_4);
        // D s_1_6: read-var len:i
        let s_1_6: i128 = fn_state.len;
        // D s_1_7: call __id(s_1_6)
        let s_1_7: i128 = u__id(state, tracer, s_1_6);
        // C s_1_8: const #6s : i
        let s_1_8: i128 = 6;
        // D s_1_9: cmp-ge s_1_8 s_1_7
        let s_1_9: bool = ((s_1_8) >= (s_1_7));
        // N s_1_10: assert s_1_9
        let s_1_10: () = assert!(s_1_9);
        // C s_1_11: const #6s : i
        let s_1_11: i128 = 6;
        // D s_1_12: read-var len:i
        let s_1_12: i128 = fn_state.len;
        // D s_1_13: call zext_ones(s_1_11, s_1_12)
        let s_1_13: Bits = zext_ones(state, tracer, s_1_11, s_1_12);
        // D s_1_14: cast reint s_1_13 -> u8
        let s_1_14: u8 = (s_1_13.value() as u8);
        // D s_1_15: write-var levels <= s_1_14
        fn_state.levels = s_1_14;
        // D s_1_16: read-var immediate:u8
        let s_1_16: bool = fn_state.immediate;
        // N s_1_17: branch s_1_16 b7 b2
        if s_1_16 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#22476 <= s_2_0
        fn_state.gs_22476 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // D s_3_0: read-var gs#22476:u8
        let s_3_0: bool = fn_state.gs_22476;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // D s_4_0: read-var imms:u8
        let s_4_0: u8 = fn_state.imms;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 6u16);
        // D s_4_2: read-var levels:u8
        let s_4_2: u8 = fn_state.levels;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 6u16);
        // D s_4_4: and s_4_1 s_4_3
        let s_4_4: Bits = ((s_4_1) & (s_4_3));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: u8 = (s_4_4.value() as u8);
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 6u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: read-var immr:u8
        let s_4_9: u8 = fn_state.immr;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 6u16);
        // D s_4_11: read-var levels:u8
        let s_4_11: u8 = fn_state.levels;
        // D s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 6u16);
        // D s_4_13: and s_4_10 s_4_12
        let s_4_13: Bits = ((s_4_10) & (s_4_12));
        // D s_4_14: cast reint s_4_13 -> u8
        let s_4_14: u8 = (s_4_13.value() as u8);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 6u16);
        // D s_4_16: cast zx s_4_15 -> i
        let s_4_16: i128 = (s_4_15.value() as i128);
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: cast zx s_4_8 -> i
        let s_4_18: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_19: cast zx s_4_17 -> i
        let s_4_19: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_20: sub s_4_18 s_4_19
        let s_4_20: i128 = ((s_4_18) - (s_4_19));
        // D s_4_21: cast reint s_4_20 -> i64
        let s_4_21: i64 = (s_4_20 as i64);
        // C s_4_22: const #1s : i
        let s_4_22: i128 = 1;
        // D s_4_23: read-var len:i
        let s_4_23: i128 = fn_state.len;
        // D s_4_24: lsl s_4_22 s_4_23
        let s_4_24: i128 = s_4_22 << s_4_23;
        // C s_4_25: const #1s : i
        let s_4_25: i128 = 1;
        // D s_4_26: read-var len:i
        let s_4_26: i128 = fn_state.len;
        // D s_4_27: sub s_4_26 s_4_25
        let s_4_27: i128 = ((s_4_26) - (s_4_25));
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // C s_4_29: const #0s : i
        let s_4_29: i128 = 0;
        // D s_4_30: cast zx s_4_21 -> i
        let s_4_30: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_31: cast zx s_4_28 -> i
        let s_4_31: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_32: call integer_subrange(s_4_30, s_4_31, s_4_29)
        let s_4_32: Bits = integer_subrange(state, tracer, s_4_30, s_4_31, s_4_29);
        // D s_4_33: cast zx s_4_32 -> i
        let s_4_33: i128 = (s_4_32.value() as i128);
        // D s_4_34: cast reint s_4_33 -> i64
        let s_4_34: i64 = (s_4_33 as i64);
        // D s_4_35: call __id(s_4_24)
        let s_4_35: i128 = u__id(state, tracer, s_4_24);
        // D s_4_36: cast zx s_4_8 -> i
        let s_4_36: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_37: call __id(s_4_36)
        let s_4_37: i128 = u__id(state, tracer, s_4_36);
        // D s_4_38: cast reint s_4_37 -> i64
        let s_4_38: i64 = (s_4_37 as i64);
        // C s_4_39: const #1s : i
        let s_4_39: i128 = 1;
        // D s_4_40: cast zx s_4_38 -> i
        let s_4_40: i128 = (i128::try_from(s_4_38).unwrap());
        // D s_4_41: add s_4_40 s_4_39
        let s_4_41: i128 = (s_4_40 + s_4_39);
        // D s_4_42: cast reint s_4_41 -> i64
        let s_4_42: i64 = (s_4_41 as i64);
        // D s_4_43: cast zx s_4_42 -> i
        let s_4_43: i128 = (i128::try_from(s_4_42).unwrap());
        // D s_4_44: cmp-ge s_4_35 s_4_43
        let s_4_44: bool = ((s_4_35) >= (s_4_43));
        // N s_4_45: assert s_4_44
        let s_4_45: () = assert!(s_4_44);
        // C s_4_46: const #1s : i
        let s_4_46: i128 = 1;
        // D s_4_47: cast zx s_4_8 -> i
        let s_4_47: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_48: add s_4_47 s_4_46
        let s_4_48: i128 = (s_4_47 + s_4_46);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: cast zx s_4_49 -> i
        let s_4_50: i128 = (i128::try_from(s_4_49).unwrap());
        // D s_4_51: call zext_ones(s_4_24, s_4_50)
        let s_4_51: Bits = zext_ones(state, tracer, s_4_24, s_4_50);
        // D s_4_52: call __id(s_4_24)
        let s_4_52: i128 = u__id(state, tracer, s_4_24);
        // D s_4_53: cast zx s_4_34 -> i
        let s_4_53: i128 = (i128::try_from(s_4_34).unwrap());
        // D s_4_54: call __id(s_4_53)
        let s_4_54: i128 = u__id(state, tracer, s_4_53);
        // D s_4_55: cast reint s_4_54 -> i64
        let s_4_55: i64 = (s_4_54 as i64);
        // C s_4_56: const #1s : i
        let s_4_56: i128 = 1;
        // D s_4_57: cast zx s_4_55 -> i
        let s_4_57: i128 = (i128::try_from(s_4_55).unwrap());
        // D s_4_58: add s_4_57 s_4_56
        let s_4_58: i128 = (s_4_57 + s_4_56);
        // D s_4_59: cast reint s_4_58 -> i64
        let s_4_59: i64 = (s_4_58 as i64);
        // D s_4_60: cast zx s_4_59 -> i
        let s_4_60: i128 = (i128::try_from(s_4_59).unwrap());
        // D s_4_61: cmp-ge s_4_52 s_4_60
        let s_4_61: bool = ((s_4_52) >= (s_4_60));
        // N s_4_62: assert s_4_61
        let s_4_62: () = assert!(s_4_61);
        // C s_4_63: const #1s : i
        let s_4_63: i128 = 1;
        // D s_4_64: cast zx s_4_34 -> i
        let s_4_64: i128 = (i128::try_from(s_4_34).unwrap());
        // D s_4_65: add s_4_64 s_4_63
        let s_4_65: i128 = (s_4_64 + s_4_63);
        // D s_4_66: cast reint s_4_65 -> i64
        let s_4_66: i64 = (s_4_65 as i64);
        // D s_4_67: cast zx s_4_66 -> i
        let s_4_67: i128 = (i128::try_from(s_4_66).unwrap());
        // D s_4_68: call zext_ones(s_4_24, s_4_67)
        let s_4_68: Bits = zext_ones(state, tracer, s_4_24, s_4_67);
        // D s_4_69: call __id(s_4_24)
        let s_4_69: i128 = u__id(state, tracer, s_4_24);
        // D s_4_70: read-var Mshadow#378:i
        let s_4_70: i128 = fn_state.Mshadow_378;
        // D s_4_71: call __id(s_4_70)
        let s_4_71: i128 = u__id(state, tracer, s_4_70);
        // D s_4_72: call __id(s_4_24)
        let s_4_72: i128 = u__id(state, tracer, s_4_24);
        // D s_4_73: div s_4_71 s_4_72
        let s_4_73: i128 = ((s_4_71) / (s_4_72));
        // D s_4_74: mul s_4_69 s_4_73
        let s_4_74: i128 = ((s_4_69) * (s_4_73));
        // D s_4_75: read-var Mshadow#378:i
        let s_4_75: i128 = fn_state.Mshadow_378;
        // D s_4_76: call __id(s_4_75)
        let s_4_76: i128 = u__id(state, tracer, s_4_75);
        // D s_4_77: cmp-eq s_4_74 s_4_76
        let s_4_77: bool = ((s_4_74) == (s_4_76));
        // N s_4_78: assert s_4_77
        let s_4_78: () = assert!(s_4_77);
        // D s_4_79: cast zx s_4_17 -> i
        let s_4_79: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_80: call ROR(s_4_51, s_4_79)
        let s_4_80: Bits = ROR(state, tracer, s_4_51, s_4_79);
        // D s_4_81: read-var Mshadow#378:i
        let s_4_81: i128 = fn_state.Mshadow_378;
        // D s_4_82: div s_4_81 s_4_24
        let s_4_82: i128 = ((s_4_81) / (s_4_24));
        // D s_4_83: cast reint s_4_82 -> u64
        let s_4_83: u64 = (s_4_82 as u64);
        // D s_4_84: call replicate_bits_borealis_internal(s_4_80, s_4_83)
        let s_4_84: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_4_80,
            s_4_83,
        );
        // D s_4_85: read-var Mshadow#378:i
        let s_4_85: i128 = fn_state.Mshadow_378;
        // D s_4_86: div s_4_85 s_4_24
        let s_4_86: i128 = ((s_4_85) / (s_4_24));
        // D s_4_87: cast reint s_4_86 -> u64
        let s_4_87: u64 = (s_4_86 as u64);
        // D s_4_88: call replicate_bits_borealis_internal(s_4_68, s_4_87)
        let s_4_88: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_4_68,
            s_4_87,
        );
        // D s_4_89: create-product struct = ["s_4_84", "s_4_88"]
        let s_4_89: ProductTypebc91b195b0b2a883 = ProductTypebc91b195b0b2a883 {
            _0: s_4_84,
            _1: s_4_88,
        };
        // D s_4_90: write-var return_value <= s_4_89
        fn_state.return_value = s_4_89;
        // D s_4_91: read-var return_value:struct
        let s_4_91: ProductTypebc91b195b0b2a883 = fn_state.return_value;
        // N s_4_92: return s_4_91
        return s_4_91;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // D s_6_0: read-var return_value:struct
        let s_6_0: ProductTypebc91b195b0b2a883 = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // D s_7_0: read-var imms:u8
        let s_7_0: u8 = fn_state.imms;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 6u16);
        // D s_7_2: read-var levels:u8
        let s_7_2: u8 = fn_state.levels;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 6u16);
        // D s_7_4: and s_7_1 s_7_3
        let s_7_4: Bits = ((s_7_1) & (s_7_3));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: u8 = (s_7_4.value() as u8);
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 6u16);
        // D s_7_7: read-var levels:u8
        let s_7_7: u8 = fn_state.levels;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 6u16);
        // D s_7_9: cmp-eq s_7_6 s_7_8
        let s_7_9: bool = ((s_7_6) == (s_7_8));
        // D s_7_10: write-var gs#22476 <= s_7_9
        fn_state.gs_22476 = s_7_9;
        // N s_7_11: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebc91b195b0b2a883 {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: jump b6
        return block_6(state, tracer, fn_state);
    }
}
