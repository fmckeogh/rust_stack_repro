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
use HaveEnhancedPAC2::*;
use CalculateBottomPACBit::*;
use EffectiveTBI::*;
use u__id::*;
use HaveFPACCombined::*;
use vector_update_subrange_from_subrange::*;
use EffectiveMTX::*;
use replicate_bits_borealis_internal::*;
use AArch64_PACFailException::*;
use HaveFPAC::*;
use subrange_subrange_eq::*;
use subrange_subrange_concat::*;
use ComputePAC::*;
use common::*;
pub fn Auth<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ptr: u64,
    modifier: u64,
    K: u128,
    data: bool,
    key_number: bool,
    is_combined: bool,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        tbi: bool,
        gs_14872: bool,
        gs_14886: bool,
        gs_14894: bool,
        ga_10971: bool,
        PAC: u64,
        extfield: u64,
        ga_11004: bool,
        gs_14936: bool,
        bottom_PAC_bit: i128,
        gs_14799: bool,
        original_ptr: u64,
        gs_14949: bool,
        gs_14937: bool,
        ga_10947: bool,
        result: u64,
        gs_14794: bool,
        gs_14911: bool,
        mtx: bool,
        return_value: u64,
        gs_14912: bool,
        gs_14901: bool,
        gs_14819: bool,
        gs_14786: bool,
        gs_14843: bool,
        gs_14923: bool,
        gs_14932: bool,
        gs_14862: bool,
        gs_14838: bool,
        gs_14873: bool,
        gs_14814: bool,
        ptr: u64,
        modifier: u64,
        K: u128,
        data: bool,
        key_number: bool,
        is_combined: bool,
    }
    let fn_state = FunctionState {
        ptr,
        modifier,
        K,
        data,
        key_number,
        is_combined,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_0_0: read-var data:u8
        let s_0_0: bool = fn_state.data;
        // D s_0_1: not s_0_0
        let s_0_1: bool = !s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: read-var ptr:u64
        let s_0_4: u64 = fn_state.ptr;
        // D s_0_5: call EffectiveTBI(s_0_4, s_0_1, s_0_3)
        let s_0_5: bool = EffectiveTBI(state, tracer, s_0_4, s_0_1, s_0_3);
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // D s_0_10: write-var tbi <= s_0_9
        fn_state.tbi = s_0_9;
        // D s_0_11: read-var data:u8
        let s_0_11: bool = fn_state.data;
        // D s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // C s_0_13: const #16975u : u32
        let s_0_13: u32 = 16975;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: read-var ptr:u64
        let s_0_15: u64 = fn_state.ptr;
        // D s_0_16: call EffectiveMTX(s_0_15, s_0_12, s_0_14)
        let s_0_16: bool = EffectiveMTX(state, tracer, s_0_15, s_0_12, s_0_14);
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #1u : u8
        let s_0_18: bool = true;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // D s_0_21: write-var mtx <= s_0_20
        fn_state.mtx = s_0_20;
        // C s_0_22: const #55s : i
        let s_0_22: i128 = 55;
        // D s_0_23: read-var ptr:u64
        let s_0_23: u64 = fn_state.ptr;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 64u16);
        // C s_0_25: const #1u : u64
        let s_0_25: u64 = 1;
        // D s_0_26: bit-extract s_0_24 s_0_22 s_0_25
        let s_0_26: Bits = (Bits::new(
            ((s_0_24) >> (s_0_22)).value(),
            u16::try_from(s_0_25).unwrap(),
        ));
        // D s_0_27: cast reint s_0_26 -> u8
        let s_0_27: bool = ((s_0_26.value()) != 0);
        // C s_0_28: const #0s : i
        let s_0_28: i128 = 0;
        // C s_0_29: const #0u : u64
        let s_0_29: u64 = 0;
        // D s_0_30: cast zx s_0_27 -> u64
        let s_0_30: u64 = (s_0_27 as u64);
        // C s_0_31: const #1u : u64
        let s_0_31: u64 = 1;
        // D s_0_32: and s_0_30 s_0_31
        let s_0_32: u64 = ((s_0_30) & (s_0_31));
        // D s_0_33: cmp-eq s_0_32 s_0_31
        let s_0_33: bool = ((s_0_32) == (s_0_31));
        // D s_0_34: lsl s_0_30 s_0_28
        let s_0_34: u64 = s_0_30 << s_0_28;
        // D s_0_35: or s_0_29 s_0_34
        let s_0_35: u64 = ((s_0_29) | (s_0_34));
        // D s_0_36: cmpl s_0_34
        let s_0_36: u64 = !s_0_34;
        // D s_0_37: and s_0_29 s_0_36
        let s_0_37: u64 = ((s_0_29) & (s_0_36));
        // D s_0_38: select s_0_33 s_0_35 s_0_37
        let s_0_38: u64 = if s_0_33 { s_0_35 } else { s_0_37 };
        // D s_0_39: cast trunc s_0_38 -> u8
        let s_0_39: bool = ((s_0_38) != 0);
        // D s_0_40: call CalculateBottomPACBit(s_0_39)
        let s_0_40: i128 = CalculateBottomPACBit(state, tracer, s_0_39);
        // D s_0_41: write-var bottom_PAC_bit <= s_0_40
        fn_state.bottom_PAC_bit = s_0_40;
        // C s_0_42: const #55s : i
        let s_0_42: i128 = 55;
        // D s_0_43: read-var ptr:u64
        let s_0_43: u64 = fn_state.ptr;
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: const #1u : u64
        let s_0_45: u64 = 1;
        // D s_0_46: bit-extract s_0_44 s_0_42 s_0_45
        let s_0_46: Bits = (Bits::new(
            ((s_0_44) >> (s_0_42)).value(),
            u16::try_from(s_0_45).unwrap(),
        ));
        // D s_0_47: cast reint s_0_46 -> u8
        let s_0_47: bool = ((s_0_46.value()) != 0);
        // C s_0_48: const #0s : i
        let s_0_48: i128 = 0;
        // C s_0_49: const #0u : u64
        let s_0_49: u64 = 0;
        // D s_0_50: cast zx s_0_47 -> u64
        let s_0_50: u64 = (s_0_47 as u64);
        // C s_0_51: const #1u : u64
        let s_0_51: u64 = 1;
        // D s_0_52: and s_0_50 s_0_51
        let s_0_52: u64 = ((s_0_50) & (s_0_51));
        // D s_0_53: cmp-eq s_0_52 s_0_51
        let s_0_53: bool = ((s_0_52) == (s_0_51));
        // D s_0_54: lsl s_0_50 s_0_48
        let s_0_54: u64 = s_0_50 << s_0_48;
        // D s_0_55: or s_0_49 s_0_54
        let s_0_55: u64 = ((s_0_49) | (s_0_54));
        // D s_0_56: cmpl s_0_54
        let s_0_56: u64 = !s_0_54;
        // D s_0_57: and s_0_49 s_0_56
        let s_0_57: u64 = ((s_0_49) & (s_0_56));
        // D s_0_58: select s_0_53 s_0_55 s_0_57
        let s_0_58: u64 = if s_0_53 { s_0_55 } else { s_0_57 };
        // D s_0_59: cast trunc s_0_58 -> u8
        let s_0_59: bool = ((s_0_58) != 0);
        // D s_0_60: cast zx s_0_59 -> bv
        let s_0_60: Bits = Bits::new(s_0_59 as u128, 1u16);
        // C s_0_61: const #64u : u64
        let s_0_61: u64 = 64;
        // D s_0_62: call replicate_bits_borealis_internal(s_0_60, s_0_61)
        let s_0_62: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_0_60,
            s_0_61,
        );
        // D s_0_63: cast reint s_0_62 -> u64
        let s_0_63: u64 = (s_0_62.value() as u64);
        // D s_0_64: write-var extfield <= s_0_63
        fn_state.extfield = s_0_63;
        // D s_0_65: read-var tbi:u8
        let s_0_65: bool = fn_state.tbi;
        // N s_0_66: branch s_0_65 b108 b1
        if s_0_65 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#14786 <= s_1_0
        fn_state.gs_14786 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var gs#14786:u8
        let s_2_0: bool = fn_state.gs_14786;
        // N s_2_1: branch s_2_0 b107 b3
        if s_2_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var tbi:u8
        let s_3_0: bool = fn_state.tbi;
        // N s_3_1: branch s_3_0 b100 b4
        if s_3_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var mtx:u8
        let s_4_0: bool = fn_state.mtx;
        // N s_4_1: branch s_4_0 b93 b5
        if s_4_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var bottom_PAC_bit:i
        let s_5_0: i128 = fn_state.bottom_PAC_bit;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #64s : i
        let s_5_2: i128 = 64;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // C s_5_4: const #1s : i
        let s_5_4: i128 = 1;
        // D s_5_5: sub s_5_3 s_5_4
        let s_5_5: i128 = ((s_5_3) - (s_5_4));
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: cmp-le s_5_6 s_5_5
        let s_5_7: bool = ((s_5_6) <= (s_5_5));
        // N s_5_8: branch s_5_7 b92 b6
        if s_5_7 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#14794 <= s_6_0
        fn_state.gs_14794 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#14794:u8
        let s_7_0: bool = fn_state.gs_14794;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var bottom_PAC_bit:i
        let s_7_2: i128 = fn_state.bottom_PAC_bit;
        // D s_7_3: call __id(s_7_2)
        let s_7_3: i128 = u__id(state, tracer, s_7_2);
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_7: sub s_7_6 s_7_5
        let s_7_7: i128 = ((s_7_6) - (s_7_5));
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #0s : i
        let s_7_9: i128 = 0;
        // D s_7_10: cast zx s_7_8 -> i
        let s_7_10: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_11: cmp-le s_7_9 s_7_10
        let s_7_11: bool = ((s_7_9) <= (s_7_10));
        // N s_7_12: branch s_7_11 b91 b8
        if s_7_11 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#14799 <= s_8_0
        fn_state.gs_14799 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#14799:u8
        let s_9_0: bool = fn_state.gs_14799;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // C s_9_2: const #64s : i
        let s_9_2: i128 = 64;
        // D s_9_3: read-var bottom_PAC_bit:i
        let s_9_3: i128 = fn_state.bottom_PAC_bit;
        // D s_9_4: sub s_9_2 s_9_3
        let s_9_4: i128 = ((s_9_2) - (s_9_3));
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // C s_9_6: const #1s : i
        let s_9_6: i128 = 1;
        // D s_9_7: cast zx s_9_5 -> i
        let s_9_7: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_8: sub s_9_7 s_9_6
        let s_9_8: i128 = ((s_9_7) - (s_9_6));
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // C s_9_10: const #1s : i
        let s_9_10: i128 = 1;
        // D s_9_11: read-var bottom_PAC_bit:i
        let s_9_11: i128 = fn_state.bottom_PAC_bit;
        // D s_9_12: sub s_9_11 s_9_10
        let s_9_12: i128 = ((s_9_11) - (s_9_10));
        // D s_9_13: cast reint s_9_12 -> i64
        let s_9_13: i64 = (s_9_12 as i64);
        // C s_9_14: const #64s : i
        let s_9_14: i128 = 64;
        // C s_9_15: const #0s : i
        let s_9_15: i128 = 0;
        // C s_9_16: const #0s : i
        let s_9_16: i128 = 0;
        // D s_9_17: read-var extfield:u64
        let s_9_17: u64 = fn_state.extfield;
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 64u16);
        // D s_9_19: cast zx s_9_9 -> i
        let s_9_19: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_20: read-var ptr:u64
        let s_9_20: u64 = fn_state.ptr;
        // D s_9_21: cast zx s_9_20 -> bv
        let s_9_21: Bits = Bits::new(s_9_20 as u128, 64u16);
        // D s_9_22: cast zx s_9_13 -> i
        let s_9_22: i128 = (i128::try_from(s_9_13).unwrap());
        // D s_9_23: call subrange_subrange_concat(s_9_14, s_9_18, s_9_19, s_9_15, s_9_21, s_9_22, s_9_16)
        let s_9_23: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_9_14,
            s_9_18,
            s_9_19,
            s_9_15,
            s_9_21,
            s_9_22,
            s_9_16,
        );
        // D s_9_24: cast reint s_9_23 -> u64
        let s_9_24: u64 = (s_9_23.value() as u64);
        // D s_9_25: write-var original_ptr <= s_9_24
        fn_state.original_ptr = s_9_24;
        // N s_9_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_10_0: const #64s : i
        let s_10_0: i128 = 64;
        // D s_10_1: read-var K:u128
        let s_10_1: u128 = fn_state.K;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 128u16);
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #63s : i
        let s_10_5: i128 = 63;
        // C s_10_6: add s_10_5 s_10_4
        let s_10_6: i128 = (s_10_5 + s_10_4);
        // D s_10_7: bit-extract s_10_2 s_10_0 s_10_6
        let s_10_7: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D s_10_8: cast reint s_10_7 -> u64
        let s_10_8: u64 = (s_10_7.value() as u64);
        // C s_10_9: const #0s : i
        let s_10_9: i128 = 0;
        // D s_10_10: read-var K:u128
        let s_10_10: u128 = fn_state.K;
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 128u16);
        // C s_10_12: const #1s : i64
        let s_10_12: i64 = 1;
        // C s_10_13: cast zx s_10_12 -> i
        let s_10_13: i128 = (i128::try_from(s_10_12).unwrap());
        // C s_10_14: const #63s : i
        let s_10_14: i128 = 63;
        // C s_10_15: add s_10_14 s_10_13
        let s_10_15: i128 = (s_10_14 + s_10_13);
        // D s_10_16: bit-extract s_10_11 s_10_9 s_10_15
        let s_10_16: Bits = (Bits::new(
            ((s_10_11) >> (s_10_9)).value(),
            u16::try_from(s_10_15).unwrap(),
        ));
        // D s_10_17: cast reint s_10_16 -> u64
        let s_10_17: u64 = (s_10_16.value() as u64);
        // D s_10_18: read-var original_ptr:u64
        let s_10_18: u64 = fn_state.original_ptr;
        // D s_10_19: read-var modifier:u64
        let s_10_19: u64 = fn_state.modifier;
        // C s_10_20: const #0u : u8
        let s_10_20: bool = false;
        // D s_10_21: call ComputePAC(s_10_18, s_10_19, s_10_8, s_10_17, s_10_20)
        let s_10_21: u64 = ComputePAC(
            state,
            tracer,
            s_10_18,
            s_10_19,
            s_10_8,
            s_10_17,
            s_10_20,
        );
        // D s_10_22: write-var PAC <= s_10_21
        fn_state.PAC = s_10_21;
        // D s_10_23: read-var tbi:u8
        let s_10_23: bool = fn_state.tbi;
        // N s_10_24: branch s_10_23 b65 b11
        if s_10_23 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var mtx:u8
        let s_11_0: bool = fn_state.mtx;
        // N s_11_1: branch s_11_0 b43 b12
        if s_11_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HaveEnhancedPAC2(s_12_0)
        let s_12_1: bool = HaveEnhancedPAC2(state, tracer, s_12_0);
        // S s_12_2: not s_12_1
        let s_12_2: bool = !s_12_1;
        // N s_12_3: branch s_12_2 b34 b13
        if s_12_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var ptr:u64
        let s_13_0: u64 = fn_state.ptr;
        // D s_13_1: write-var result <= s_13_0
        fn_state.result = s_13_0;
        // D s_13_2: read-var bottom_PAC_bit:i
        let s_13_2: i128 = fn_state.bottom_PAC_bit;
        // D s_13_3: call __id(s_13_2)
        let s_13_3: i128 = u__id(state, tracer, s_13_2);
        // C s_13_4: const #0s : i
        let s_13_4: i128 = 0;
        // D s_13_5: cmp-le s_13_4 s_13_3
        let s_13_5: bool = ((s_13_4) <= (s_13_3));
        // N s_13_6: branch s_13_5 b33 b14
        if s_13_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#14862 <= s_14_0
        fn_state.gs_14862 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var gs#14862:u8
        let s_15_0: bool = fn_state.gs_14862;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var result:u64
        let s_15_2: u64 = fn_state.result;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_4: read-var PAC:u64
        let s_15_4: u64 = fn_state.PAC;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 64u16);
        // D s_15_6: xor s_15_3 s_15_5
        let s_15_6: Bits = ((s_15_3) ^ (s_15_5));
        // D s_15_7: cast reint s_15_6 -> u64
        let s_15_7: u64 = (s_15_6.value() as u64);
        // C s_15_8: const #64s : i
        let s_15_8: i128 = 64;
        // C s_15_9: const #54s : i
        let s_15_9: i128 = 54;
        // C s_15_10: const #54s : i
        let s_15_10: i128 = 54;
        // D s_15_11: read-var result:u64
        let s_15_11: u64 = fn_state.result;
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 64u16);
        // D s_15_13: cast zx s_15_7 -> bv
        let s_15_13: Bits = Bits::new(s_15_7 as u128, 64u16);
        // D s_15_14: read-var bottom_PAC_bit:i
        let s_15_14: i128 = fn_state.bottom_PAC_bit;
        // D s_15_15: read-var bottom_PAC_bit:i
        let s_15_15: i128 = fn_state.bottom_PAC_bit;
        // D s_15_16: call vector_update_subrange_from_subrange(s_15_8, s_15_12, s_15_9, s_15_14, s_15_13, s_15_10, s_15_15)
        let s_15_16: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_15_8,
            s_15_12,
            s_15_9,
            s_15_14,
            s_15_13,
            s_15_10,
            s_15_15,
        );
        // D s_15_17: cast reint s_15_16 -> u64
        let s_15_17: u64 = (s_15_16.value() as u64);
        // D s_15_18: write-var result <= s_15_17
        fn_state.result = s_15_17;
        // C s_15_19: const #56s : i
        let s_15_19: i128 = 56;
        // D s_15_20: read-var result:u64
        let s_15_20: u64 = fn_state.result;
        // D s_15_21: cast zx s_15_20 -> bv
        let s_15_21: Bits = Bits::new(s_15_20 as u128, 64u16);
        // C s_15_22: const #1s : i64
        let s_15_22: i64 = 1;
        // C s_15_23: cast zx s_15_22 -> i
        let s_15_23: i128 = (i128::try_from(s_15_22).unwrap());
        // C s_15_24: const #7s : i
        let s_15_24: i128 = 7;
        // C s_15_25: add s_15_24 s_15_23
        let s_15_25: i128 = (s_15_24 + s_15_23);
        // D s_15_26: bit-extract s_15_21 s_15_19 s_15_25
        let s_15_26: Bits = (Bits::new(
            ((s_15_21) >> (s_15_19)).value(),
            u16::try_from(s_15_25).unwrap(),
        ));
        // D s_15_27: cast reint s_15_26 -> u8
        let s_15_27: u8 = (s_15_26.value() as u8);
        // C s_15_28: const #56s : i
        let s_15_28: i128 = 56;
        // D s_15_29: read-var PAC:u64
        let s_15_29: u64 = fn_state.PAC;
        // D s_15_30: cast zx s_15_29 -> bv
        let s_15_30: Bits = Bits::new(s_15_29 as u128, 64u16);
        // C s_15_31: const #1s : i64
        let s_15_31: i64 = 1;
        // C s_15_32: cast zx s_15_31 -> i
        let s_15_32: i128 = (i128::try_from(s_15_31).unwrap());
        // C s_15_33: const #7s : i
        let s_15_33: i128 = 7;
        // C s_15_34: add s_15_33 s_15_32
        let s_15_34: i128 = (s_15_33 + s_15_32);
        // D s_15_35: bit-extract s_15_30 s_15_28 s_15_34
        let s_15_35: Bits = (Bits::new(
            ((s_15_30) >> (s_15_28)).value(),
            u16::try_from(s_15_34).unwrap(),
        ));
        // D s_15_36: cast reint s_15_35 -> u8
        let s_15_36: u8 = (s_15_35.value() as u8);
        // D s_15_37: cast zx s_15_27 -> bv
        let s_15_37: Bits = Bits::new(s_15_27 as u128, 8u16);
        // D s_15_38: cast zx s_15_36 -> bv
        let s_15_38: Bits = Bits::new(s_15_36 as u128, 8u16);
        // D s_15_39: xor s_15_37 s_15_38
        let s_15_39: Bits = ((s_15_37) ^ (s_15_38));
        // D s_15_40: cast reint s_15_39 -> u8
        let s_15_40: u8 = (s_15_39.value() as u8);
        // C s_15_41: const #56s : i
        let s_15_41: i128 = 56;
        // D s_15_42: read-var result:u64
        let s_15_42: u64 = fn_state.result;
        // D s_15_43: cast zx s_15_42 -> bv
        let s_15_43: Bits = Bits::new(s_15_42 as u128, 64u16);
        // D s_15_44: cast zx s_15_40 -> bv
        let s_15_44: Bits = Bits::new(s_15_40 as u128, 8u16);
        // C s_15_45: const #7s : i
        let s_15_45: i128 = 7;
        // C s_15_46: const #1u : u64
        let s_15_46: u64 = 1;
        // C s_15_47: cast zx s_15_46 -> bv
        let s_15_47: Bits = Bits::new(s_15_46 as u128, 64u16);
        // C s_15_48: lsl s_15_47 s_15_45
        let s_15_48: Bits = s_15_47 << s_15_45;
        // C s_15_49: sub s_15_48 s_15_47
        let s_15_49: Bits = ((s_15_48) - (s_15_47));
        // D s_15_50: and s_15_44 s_15_49
        let s_15_50: Bits = ((s_15_44) & (s_15_49));
        // D s_15_51: lsl s_15_50 s_15_41
        let s_15_51: Bits = s_15_50 << s_15_41;
        // C s_15_52: lsl s_15_49 s_15_41
        let s_15_52: Bits = s_15_49 << s_15_41;
        // C s_15_53: cmpl s_15_52
        let s_15_53: Bits = !s_15_52;
        // D s_15_54: and s_15_43 s_15_53
        let s_15_54: Bits = ((s_15_43) & (s_15_53));
        // D s_15_55: or s_15_54 s_15_51
        let s_15_55: Bits = ((s_15_54) | (s_15_51));
        // D s_15_56: cast reint s_15_55 -> u64
        let s_15_56: u64 = (s_15_55.value() as u64);
        // D s_15_57: write-var result <= s_15_56
        fn_state.result = s_15_56;
        // C s_15_58: const #() : ()
        let s_15_58: () = ();
        // S s_15_59: call HaveFPACCombined(s_15_58)
        let s_15_59: bool = HaveFPACCombined(state, tracer, s_15_58);
        // N s_15_60: branch s_15_59 b32 b16
        if s_15_59 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveFPAC(s_16_0)
        let s_16_1: bool = HaveFPAC(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b31 b17
        if s_16_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#14872 <= s_17_0
        fn_state.gs_14872 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_18_0: read-var gs#14872:u8
        let s_18_0: bool = fn_state.gs_14872;
        // D s_18_1: write-var gs#14873 <= s_18_0
        fn_state.gs_14873 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var gs#14873:u8
        let s_19_0: bool = fn_state.gs_14873;
        // N s_19_1: branch s_19_0 b24 b20
        if s_19_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_22_0: read-var result:u64
        let s_22_0: u64 = fn_state.result;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var return_value:u64
        let s_23_0: u64 = fn_state.return_value;
        // N s_23_1: return s_23_0
        return s_23_0;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #63s : i
        let s_24_0: i128 = 63;
        // D s_24_1: read-var result:u64
        let s_24_1: u64 = fn_state.result;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 64u16);
        // D s_24_3: read-var bottom_PAC_bit:i
        let s_24_3: i128 = fn_state.bottom_PAC_bit;
        // C s_24_4: const #1s : i64
        let s_24_4: i64 = 1;
        // C s_24_5: cast zx s_24_4 -> i
        let s_24_5: i128 = (i128::try_from(s_24_4).unwrap());
        // D s_24_6: sub s_24_0 s_24_3
        let s_24_6: i128 = ((s_24_0) - (s_24_3));
        // D s_24_7: add s_24_6 s_24_5
        let s_24_7: i128 = (s_24_6 + s_24_5);
        // D s_24_8: bit-extract s_24_2 s_24_3 s_24_7
        let s_24_8: Bits = (Bits::new(
            ((s_24_2) >> (s_24_3)).value(),
            u16::try_from(s_24_7).unwrap(),
        ));
        // C s_24_9: const #55s : i
        let s_24_9: i128 = 55;
        // D s_24_10: read-var result:u64
        let s_24_10: u64 = fn_state.result;
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 64u16);
        // C s_24_12: const #1u : u64
        let s_24_12: u64 = 1;
        // D s_24_13: bit-extract s_24_11 s_24_9 s_24_12
        let s_24_13: Bits = (Bits::new(
            ((s_24_11) >> (s_24_9)).value(),
            u16::try_from(s_24_12).unwrap(),
        ));
        // D s_24_14: cast reint s_24_13 -> u8
        let s_24_14: bool = ((s_24_13.value()) != 0);
        // C s_24_15: const #0s : i
        let s_24_15: i128 = 0;
        // C s_24_16: const #0u : u64
        let s_24_16: u64 = 0;
        // D s_24_17: cast zx s_24_14 -> u64
        let s_24_17: u64 = (s_24_14 as u64);
        // C s_24_18: const #1u : u64
        let s_24_18: u64 = 1;
        // D s_24_19: and s_24_17 s_24_18
        let s_24_19: u64 = ((s_24_17) & (s_24_18));
        // D s_24_20: cmp-eq s_24_19 s_24_18
        let s_24_20: bool = ((s_24_19) == (s_24_18));
        // D s_24_21: lsl s_24_17 s_24_15
        let s_24_21: u64 = s_24_17 << s_24_15;
        // D s_24_22: or s_24_16 s_24_21
        let s_24_22: u64 = ((s_24_16) | (s_24_21));
        // D s_24_23: cmpl s_24_21
        let s_24_23: u64 = !s_24_21;
        // D s_24_24: and s_24_16 s_24_23
        let s_24_24: u64 = ((s_24_16) & (s_24_23));
        // D s_24_25: select s_24_20 s_24_22 s_24_24
        let s_24_25: u64 = if s_24_20 { s_24_22 } else { s_24_24 };
        // D s_24_26: cast trunc s_24_25 -> u8
        let s_24_26: bool = ((s_24_25) != 0);
        // C s_24_27: const #64s : i
        let s_24_27: i128 = 64;
        // D s_24_28: read-var bottom_PAC_bit:i
        let s_24_28: i128 = fn_state.bottom_PAC_bit;
        // D s_24_29: sub s_24_27 s_24_28
        let s_24_29: i128 = ((s_24_27) - (s_24_28));
        // D s_24_30: cast reint s_24_29 -> i64
        let s_24_30: i64 = (s_24_29 as i64);
        // D s_24_31: cast zx s_24_26 -> bv
        let s_24_31: Bits = Bits::new(s_24_26 as u128, 1u16);
        // D s_24_32: cast zx s_24_30 -> i
        let s_24_32: i128 = (i128::try_from(s_24_30).unwrap());
        // D s_24_33: cast reint s_24_32 -> u64
        let s_24_33: u64 = (s_24_32 as u64);
        // D s_24_34: call replicate_bits_borealis_internal(s_24_31, s_24_33)
        let s_24_34: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_24_31,
            s_24_33,
        );
        // D s_24_35: cmp-ne s_24_8 s_24_34
        let s_24_35: bool = ((s_24_8) != (s_24_34));
        // N s_24_36: branch s_24_35 b27 b25
        if s_24_35 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_26_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var data:u8
        let s_27_0: bool = fn_state.data;
        // N s_27_1: branch s_27_0 b30 b28
        if s_27_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var ga#11004 <= s_28_0
        fn_state.ga_11004 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_29_0: read-var ga#11004:u8
        let s_29_0: bool = fn_state.ga_11004;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // D s_29_2: read-var key_number:u8
        let s_29_2: bool = fn_state.key_number;
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cast reint s_29_1 -> u128
        let s_29_4: u128 = (s_29_1.value() as u128);
        // D s_29_5: size-of s_29_1
        let s_29_5: u16 = s_29_1.length();
        // D s_29_6: cast reint s_29_3 -> u128
        let s_29_6: u128 = (s_29_3.value() as u128);
        // D s_29_7: size-of s_29_3
        let s_29_7: u16 = s_29_3.length();
        // D s_29_8: lsl s_29_4 s_29_7
        let s_29_8: u128 = s_29_4 << s_29_7;
        // D s_29_9: or s_29_8 s_29_6
        let s_29_9: u128 = ((s_29_8) | (s_29_6));
        // D s_29_10: add s_29_5 s_29_7
        let s_29_10: u16 = (s_29_5 + s_29_7);
        // D s_29_11: create-bits s_29_9 s_29_10
        let s_29_11: Bits = Bits::new(s_29_9, s_29_10);
        // D s_29_12: cast reint s_29_11 -> u8
        let s_29_12: u8 = (s_29_11.value() as u8);
        // D s_29_13: call AArch64_PACFailException(s_29_12)
        let s_29_13: () = AArch64_PACFailException(state, tracer, s_29_12);
        // N s_29_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var ga#11004 <= s_30_0
        fn_state.ga_11004 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_31_0: read-var is_combined:u8
        let s_31_0: bool = fn_state.is_combined;
        // D s_31_1: not s_31_0
        let s_31_1: bool = !s_31_0;
        // D s_31_2: write-var gs#14872 <= s_31_1
        fn_state.gs_14872 = s_31_1;
        // N s_31_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#14873 <= s_32_0
        fn_state.gs_14873 = s_32_0;
        // N s_32_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_33_0: read-var bottom_PAC_bit:i
        let s_33_0: i128 = fn_state.bottom_PAC_bit;
        // D s_33_1: call __id(s_33_0)
        let s_33_1: i128 = u__id(state, tracer, s_33_0);
        // C s_33_2: const #54s : i
        let s_33_2: i128 = 54;
        // D s_33_3: cmp-le s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) <= (s_33_2));
        // D s_33_4: write-var gs#14862 <= s_33_3
        fn_state.gs_14862 = s_33_3;
        // N s_33_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_34_0: read-var bottom_PAC_bit:i
        let s_34_0: i128 = fn_state.bottom_PAC_bit;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #0s : i
        let s_34_2: i128 = 0;
        // D s_34_3: cmp-le s_34_2 s_34_1
        let s_34_3: bool = ((s_34_2) <= (s_34_1));
        // N s_34_4: branch s_34_3 b42 b35
        if s_34_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#14886 <= s_35_0
        fn_state.gs_14886 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_36_0: read-var gs#14886:u8
        let s_36_0: bool = fn_state.gs_14886;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #54s : i
        let s_36_2: i128 = 54;
        // C s_36_3: const #54s : i
        let s_36_3: i128 = 54;
        // D s_36_4: read-var PAC:u64
        let s_36_4: u64 = fn_state.PAC;
        // D s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 64u16);
        // D s_36_6: read-var ptr:u64
        let s_36_6: u64 = fn_state.ptr;
        // D s_36_7: cast zx s_36_6 -> bv
        let s_36_7: Bits = Bits::new(s_36_6 as u128, 64u16);
        // D s_36_8: read-var bottom_PAC_bit:i
        let s_36_8: i128 = fn_state.bottom_PAC_bit;
        // D s_36_9: read-var bottom_PAC_bit:i
        let s_36_9: i128 = fn_state.bottom_PAC_bit;
        // D s_36_10: call subrange_subrange_eq(s_36_5, s_36_2, s_36_8, s_36_7, s_36_3, s_36_9)
        let s_36_10: bool = subrange_subrange_eq(
            state,
            tracer,
            s_36_5,
            s_36_2,
            s_36_8,
            s_36_7,
            s_36_3,
            s_36_9,
        );
        // N s_36_11: branch s_36_10 b41 b37
        if s_36_10 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#14894 <= s_37_0
        fn_state.gs_14894 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_38_0: read-var gs#14894:u8
        let s_38_0: bool = fn_state.gs_14894;
        // N s_38_1: branch s_38_0 b40 b39
        if s_38_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_39_0: read-var key_number:u8
        let s_39_0: bool = fn_state.key_number;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // D s_39_2: not s_39_1
        let s_39_2: Bits = !s_39_1;
        // D s_39_3: cast reint s_39_2 -> u8
        let s_39_3: bool = ((s_39_2.value()) != 0);
        // D s_39_4: read-var key_number:u8
        let s_39_4: bool = fn_state.key_number;
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cast zx s_39_3 -> bv
        let s_39_6: Bits = Bits::new(s_39_3 as u128, 1u16);
        // D s_39_7: cast reint s_39_5 -> u128
        let s_39_7: u128 = (s_39_5.value() as u128);
        // D s_39_8: size-of s_39_5
        let s_39_8: u16 = s_39_5.length();
        // D s_39_9: cast reint s_39_6 -> u128
        let s_39_9: u128 = (s_39_6.value() as u128);
        // D s_39_10: size-of s_39_6
        let s_39_10: u16 = s_39_6.length();
        // D s_39_11: lsl s_39_7 s_39_10
        let s_39_11: u128 = s_39_7 << s_39_10;
        // D s_39_12: or s_39_11 s_39_9
        let s_39_12: u128 = ((s_39_11) | (s_39_9));
        // D s_39_13: add s_39_8 s_39_10
        let s_39_13: u16 = (s_39_8 + s_39_10);
        // D s_39_14: create-bits s_39_12 s_39_13
        let s_39_14: Bits = Bits::new(s_39_12, s_39_13);
        // D s_39_15: cast reint s_39_14 -> u8
        let s_39_15: u8 = (s_39_14.value() as u8);
        // C s_39_16: const #63s : i
        let s_39_16: i128 = 63;
        // D s_39_17: read-var original_ptr:u64
        let s_39_17: u64 = fn_state.original_ptr;
        // D s_39_18: cast zx s_39_17 -> bv
        let s_39_18: Bits = Bits::new(s_39_17 as u128, 64u16);
        // C s_39_19: const #1u : u64
        let s_39_19: u64 = 1;
        // D s_39_20: bit-extract s_39_18 s_39_16 s_39_19
        let s_39_20: Bits = (Bits::new(
            ((s_39_18) >> (s_39_16)).value(),
            u16::try_from(s_39_19).unwrap(),
        ));
        // D s_39_21: cast reint s_39_20 -> u8
        let s_39_21: bool = ((s_39_20.value()) != 0);
        // C s_39_22: const #0s : i
        let s_39_22: i128 = 0;
        // C s_39_23: const #0u : u64
        let s_39_23: u64 = 0;
        // D s_39_24: cast zx s_39_21 -> u64
        let s_39_24: u64 = (s_39_21 as u64);
        // C s_39_25: const #1u : u64
        let s_39_25: u64 = 1;
        // D s_39_26: and s_39_24 s_39_25
        let s_39_26: u64 = ((s_39_24) & (s_39_25));
        // D s_39_27: cmp-eq s_39_26 s_39_25
        let s_39_27: bool = ((s_39_26) == (s_39_25));
        // D s_39_28: lsl s_39_24 s_39_22
        let s_39_28: u64 = s_39_24 << s_39_22;
        // D s_39_29: or s_39_23 s_39_28
        let s_39_29: u64 = ((s_39_23) | (s_39_28));
        // D s_39_30: cmpl s_39_28
        let s_39_30: u64 = !s_39_28;
        // D s_39_31: and s_39_23 s_39_30
        let s_39_31: u64 = ((s_39_23) & (s_39_30));
        // D s_39_32: select s_39_27 s_39_29 s_39_31
        let s_39_32: u64 = if s_39_27 { s_39_29 } else { s_39_31 };
        // D s_39_33: cast trunc s_39_32 -> u8
        let s_39_33: bool = ((s_39_32) != 0);
        // D s_39_34: cast zx s_39_33 -> bv
        let s_39_34: Bits = Bits::new(s_39_33 as u128, 1u16);
        // D s_39_35: cast zx s_39_15 -> bv
        let s_39_35: Bits = Bits::new(s_39_15 as u128, 2u16);
        // D s_39_36: cast reint s_39_34 -> u128
        let s_39_36: u128 = (s_39_34.value() as u128);
        // D s_39_37: size-of s_39_34
        let s_39_37: u16 = s_39_34.length();
        // D s_39_38: cast reint s_39_35 -> u128
        let s_39_38: u128 = (s_39_35.value() as u128);
        // D s_39_39: size-of s_39_35
        let s_39_39: u16 = s_39_35.length();
        // D s_39_40: lsl s_39_36 s_39_39
        let s_39_40: u128 = s_39_36 << s_39_39;
        // D s_39_41: or s_39_40 s_39_38
        let s_39_41: u128 = ((s_39_40) | (s_39_38));
        // D s_39_42: add s_39_37 s_39_39
        let s_39_42: u16 = (s_39_37 + s_39_39);
        // D s_39_43: create-bits s_39_41 s_39_42
        let s_39_43: Bits = Bits::new(s_39_41, s_39_42);
        // D s_39_44: cast reint s_39_43 -> u8
        let s_39_44: u8 = (s_39_43.value() as u8);
        // C s_39_45: const #0s : i
        let s_39_45: i128 = 0;
        // D s_39_46: read-var original_ptr:u64
        let s_39_46: u64 = fn_state.original_ptr;
        // D s_39_47: cast zx s_39_46 -> bv
        let s_39_47: Bits = Bits::new(s_39_46 as u128, 64u16);
        // C s_39_48: const #1s : i64
        let s_39_48: i64 = 1;
        // C s_39_49: cast zx s_39_48 -> i
        let s_39_49: i128 = (i128::try_from(s_39_48).unwrap());
        // C s_39_50: const #60s : i
        let s_39_50: i128 = 60;
        // C s_39_51: add s_39_50 s_39_49
        let s_39_51: i128 = (s_39_50 + s_39_49);
        // D s_39_52: bit-extract s_39_47 s_39_45 s_39_51
        let s_39_52: Bits = (Bits::new(
            ((s_39_47) >> (s_39_45)).value(),
            u16::try_from(s_39_51).unwrap(),
        ));
        // D s_39_53: cast reint s_39_52 -> u61
        let s_39_53: u64 = (s_39_52.value() as u64);
        // D s_39_54: cast zx s_39_44 -> bv
        let s_39_54: Bits = Bits::new(s_39_44 as u128, 3u16);
        // D s_39_55: cast zx s_39_53 -> bv
        let s_39_55: Bits = Bits::new(s_39_53 as u128, 61u16);
        // D s_39_56: cast reint s_39_54 -> u128
        let s_39_56: u128 = (s_39_54.value() as u128);
        // D s_39_57: size-of s_39_54
        let s_39_57: u16 = s_39_54.length();
        // D s_39_58: cast reint s_39_55 -> u128
        let s_39_58: u128 = (s_39_55.value() as u128);
        // D s_39_59: size-of s_39_55
        let s_39_59: u16 = s_39_55.length();
        // D s_39_60: lsl s_39_56 s_39_59
        let s_39_60: u128 = s_39_56 << s_39_59;
        // D s_39_61: or s_39_60 s_39_58
        let s_39_61: u128 = ((s_39_60) | (s_39_58));
        // D s_39_62: add s_39_57 s_39_59
        let s_39_62: u16 = (s_39_57 + s_39_59);
        // D s_39_63: create-bits s_39_61 s_39_62
        let s_39_63: Bits = Bits::new(s_39_61, s_39_62);
        // D s_39_64: cast reint s_39_63 -> u64
        let s_39_64: u64 = (s_39_63.value() as u64);
        // D s_39_65: write-var result <= s_39_64
        fn_state.result = s_39_64;
        // N s_39_66: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_40_0: read-var original_ptr:u64
        let s_40_0: u64 = fn_state.original_ptr;
        // D s_40_1: write-var result <= s_40_0
        fn_state.result = s_40_0;
        // N s_40_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_41_0: const #56s : i
        let s_41_0: i128 = 56;
        // D s_41_1: read-var PAC:u64
        let s_41_1: u64 = fn_state.PAC;
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 64u16);
        // C s_41_3: const #1s : i64
        let s_41_3: i64 = 1;
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #7s : i
        let s_41_5: i128 = 7;
        // C s_41_6: add s_41_5 s_41_4
        let s_41_6: i128 = (s_41_5 + s_41_4);
        // D s_41_7: bit-extract s_41_2 s_41_0 s_41_6
        let s_41_7: Bits = (Bits::new(
            ((s_41_2) >> (s_41_0)).value(),
            u16::try_from(s_41_6).unwrap(),
        ));
        // D s_41_8: cast reint s_41_7 -> u8
        let s_41_8: u8 = (s_41_7.value() as u8);
        // C s_41_9: const #56s : i
        let s_41_9: i128 = 56;
        // D s_41_10: read-var ptr:u64
        let s_41_10: u64 = fn_state.ptr;
        // D s_41_11: cast zx s_41_10 -> bv
        let s_41_11: Bits = Bits::new(s_41_10 as u128, 64u16);
        // C s_41_12: const #1s : i64
        let s_41_12: i64 = 1;
        // C s_41_13: cast zx s_41_12 -> i
        let s_41_13: i128 = (i128::try_from(s_41_12).unwrap());
        // C s_41_14: const #7s : i
        let s_41_14: i128 = 7;
        // C s_41_15: add s_41_14 s_41_13
        let s_41_15: i128 = (s_41_14 + s_41_13);
        // D s_41_16: bit-extract s_41_11 s_41_9 s_41_15
        let s_41_16: Bits = (Bits::new(
            ((s_41_11) >> (s_41_9)).value(),
            u16::try_from(s_41_15).unwrap(),
        ));
        // D s_41_17: cast reint s_41_16 -> u8
        let s_41_17: u8 = (s_41_16.value() as u8);
        // D s_41_18: cast zx s_41_8 -> bv
        let s_41_18: Bits = Bits::new(s_41_8 as u128, 8u16);
        // D s_41_19: cast zx s_41_17 -> bv
        let s_41_19: Bits = Bits::new(s_41_17 as u128, 8u16);
        // D s_41_20: cmp-eq s_41_18 s_41_19
        let s_41_20: bool = ((s_41_18) == (s_41_19));
        // D s_41_21: write-var gs#14894 <= s_41_20
        fn_state.gs_14894 = s_41_20;
        // N s_41_22: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_42_0: read-var bottom_PAC_bit:i
        let s_42_0: i128 = fn_state.bottom_PAC_bit;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // C s_42_2: const #54s : i
        let s_42_2: i128 = 54;
        // D s_42_3: cmp-le s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) <= (s_42_2));
        // D s_42_4: write-var gs#14886 <= s_42_3
        fn_state.gs_14886 = s_42_3;
        // N s_42_5: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call HaveEnhancedPAC2(s_43_0)
        let s_43_1: bool = HaveEnhancedPAC2(state, tracer, s_43_0);
        // N s_43_2: assert s_43_1
        let s_43_2: () = assert!(s_43_1);
        // D s_43_3: read-var ptr:u64
        let s_43_3: u64 = fn_state.ptr;
        // D s_43_4: write-var result <= s_43_3
        fn_state.result = s_43_3;
        // D s_43_5: read-var bottom_PAC_bit:i
        let s_43_5: i128 = fn_state.bottom_PAC_bit;
        // D s_43_6: call __id(s_43_5)
        let s_43_6: i128 = u__id(state, tracer, s_43_5);
        // C s_43_7: const #0s : i
        let s_43_7: i128 = 0;
        // D s_43_8: cmp-le s_43_7 s_43_6
        let s_43_8: bool = ((s_43_7) <= (s_43_6));
        // N s_43_9: branch s_43_8 b64 b44
        if s_43_8 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#14901 <= s_44_0
        fn_state.gs_14901 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_45_0: read-var gs#14901:u8
        let s_45_0: bool = fn_state.gs_14901;
        // N s_45_1: assert s_45_0
        let s_45_1: () = assert!(s_45_0);
        // D s_45_2: read-var result:u64
        let s_45_2: u64 = fn_state.result;
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 64u16);
        // D s_45_4: read-var PAC:u64
        let s_45_4: u64 = fn_state.PAC;
        // D s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 64u16);
        // D s_45_6: xor s_45_3 s_45_5
        let s_45_6: Bits = ((s_45_3) ^ (s_45_5));
        // D s_45_7: cast reint s_45_6 -> u64
        let s_45_7: u64 = (s_45_6.value() as u64);
        // C s_45_8: const #64s : i
        let s_45_8: i128 = 64;
        // C s_45_9: const #54s : i
        let s_45_9: i128 = 54;
        // C s_45_10: const #54s : i
        let s_45_10: i128 = 54;
        // D s_45_11: read-var result:u64
        let s_45_11: u64 = fn_state.result;
        // D s_45_12: cast zx s_45_11 -> bv
        let s_45_12: Bits = Bits::new(s_45_11 as u128, 64u16);
        // D s_45_13: cast zx s_45_7 -> bv
        let s_45_13: Bits = Bits::new(s_45_7 as u128, 64u16);
        // D s_45_14: read-var bottom_PAC_bit:i
        let s_45_14: i128 = fn_state.bottom_PAC_bit;
        // D s_45_15: read-var bottom_PAC_bit:i
        let s_45_15: i128 = fn_state.bottom_PAC_bit;
        // D s_45_16: call vector_update_subrange_from_subrange(s_45_8, s_45_12, s_45_9, s_45_14, s_45_13, s_45_10, s_45_15)
        let s_45_16: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_45_8,
            s_45_12,
            s_45_9,
            s_45_14,
            s_45_13,
            s_45_10,
            s_45_15,
        );
        // D s_45_17: cast reint s_45_16 -> u64
        let s_45_17: u64 = (s_45_16.value() as u64);
        // D s_45_18: write-var result <= s_45_17
        fn_state.result = s_45_17;
        // C s_45_19: const #60s : i
        let s_45_19: i128 = 60;
        // D s_45_20: read-var result:u64
        let s_45_20: u64 = fn_state.result;
        // D s_45_21: cast zx s_45_20 -> bv
        let s_45_21: Bits = Bits::new(s_45_20 as u128, 64u16);
        // C s_45_22: const #1s : i64
        let s_45_22: i64 = 1;
        // C s_45_23: cast zx s_45_22 -> i
        let s_45_23: i128 = (i128::try_from(s_45_22).unwrap());
        // C s_45_24: const #3s : i
        let s_45_24: i128 = 3;
        // C s_45_25: add s_45_24 s_45_23
        let s_45_25: i128 = (s_45_24 + s_45_23);
        // D s_45_26: bit-extract s_45_21 s_45_19 s_45_25
        let s_45_26: Bits = (Bits::new(
            ((s_45_21) >> (s_45_19)).value(),
            u16::try_from(s_45_25).unwrap(),
        ));
        // D s_45_27: cast reint s_45_26 -> u8
        let s_45_27: u8 = (s_45_26.value() as u8);
        // C s_45_28: const #60s : i
        let s_45_28: i128 = 60;
        // D s_45_29: read-var PAC:u64
        let s_45_29: u64 = fn_state.PAC;
        // D s_45_30: cast zx s_45_29 -> bv
        let s_45_30: Bits = Bits::new(s_45_29 as u128, 64u16);
        // C s_45_31: const #1s : i64
        let s_45_31: i64 = 1;
        // C s_45_32: cast zx s_45_31 -> i
        let s_45_32: i128 = (i128::try_from(s_45_31).unwrap());
        // C s_45_33: const #3s : i
        let s_45_33: i128 = 3;
        // C s_45_34: add s_45_33 s_45_32
        let s_45_34: i128 = (s_45_33 + s_45_32);
        // D s_45_35: bit-extract s_45_30 s_45_28 s_45_34
        let s_45_35: Bits = (Bits::new(
            ((s_45_30) >> (s_45_28)).value(),
            u16::try_from(s_45_34).unwrap(),
        ));
        // D s_45_36: cast reint s_45_35 -> u8
        let s_45_36: u8 = (s_45_35.value() as u8);
        // D s_45_37: cast zx s_45_27 -> bv
        let s_45_37: Bits = Bits::new(s_45_27 as u128, 4u16);
        // D s_45_38: cast zx s_45_36 -> bv
        let s_45_38: Bits = Bits::new(s_45_36 as u128, 4u16);
        // D s_45_39: xor s_45_37 s_45_38
        let s_45_39: Bits = ((s_45_37) ^ (s_45_38));
        // D s_45_40: cast reint s_45_39 -> u8
        let s_45_40: u8 = (s_45_39.value() as u8);
        // C s_45_41: const #60s : i
        let s_45_41: i128 = 60;
        // D s_45_42: read-var result:u64
        let s_45_42: u64 = fn_state.result;
        // D s_45_43: cast zx s_45_42 -> bv
        let s_45_43: Bits = Bits::new(s_45_42 as u128, 64u16);
        // D s_45_44: cast zx s_45_40 -> bv
        let s_45_44: Bits = Bits::new(s_45_40 as u128, 4u16);
        // C s_45_45: const #3s : i
        let s_45_45: i128 = 3;
        // C s_45_46: const #1u : u64
        let s_45_46: u64 = 1;
        // C s_45_47: cast zx s_45_46 -> bv
        let s_45_47: Bits = Bits::new(s_45_46 as u128, 64u16);
        // C s_45_48: lsl s_45_47 s_45_45
        let s_45_48: Bits = s_45_47 << s_45_45;
        // C s_45_49: sub s_45_48 s_45_47
        let s_45_49: Bits = ((s_45_48) - (s_45_47));
        // D s_45_50: and s_45_44 s_45_49
        let s_45_50: Bits = ((s_45_44) & (s_45_49));
        // D s_45_51: lsl s_45_50 s_45_41
        let s_45_51: Bits = s_45_50 << s_45_41;
        // C s_45_52: lsl s_45_49 s_45_41
        let s_45_52: Bits = s_45_49 << s_45_41;
        // C s_45_53: cmpl s_45_52
        let s_45_53: Bits = !s_45_52;
        // D s_45_54: and s_45_43 s_45_53
        let s_45_54: Bits = ((s_45_43) & (s_45_53));
        // D s_45_55: or s_45_54 s_45_51
        let s_45_55: Bits = ((s_45_54) | (s_45_51));
        // D s_45_56: cast reint s_45_55 -> u64
        let s_45_56: u64 = (s_45_55.value() as u64);
        // D s_45_57: write-var result <= s_45_56
        fn_state.result = s_45_56;
        // C s_45_58: const #() : ()
        let s_45_58: () = ();
        // S s_45_59: call HaveFPACCombined(s_45_58)
        let s_45_59: bool = HaveFPACCombined(state, tracer, s_45_58);
        // N s_45_60: branch s_45_59 b63 b46
        if s_45_59 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call HaveFPAC(s_46_0)
        let s_46_1: bool = HaveFPAC(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b62 b47
        if s_46_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#14911 <= s_47_0
        fn_state.gs_14911 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_48_0: read-var gs#14911:u8
        let s_48_0: bool = fn_state.gs_14911;
        // D s_48_1: write-var gs#14912 <= s_48_0
        fn_state.gs_14912 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_49_0: read-var gs#14912:u8
        let s_49_0: bool = fn_state.gs_14912;
        // N s_49_1: branch s_49_0 b52 b50
        if s_49_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_51_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_52_0: const #54s : i
        let s_52_0: i128 = 54;
        // D s_52_1: read-var result:u64
        let s_52_1: u64 = fn_state.result;
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 64u16);
        // D s_52_3: read-var bottom_PAC_bit:i
        let s_52_3: i128 = fn_state.bottom_PAC_bit;
        // C s_52_4: const #1s : i64
        let s_52_4: i64 = 1;
        // C s_52_5: cast zx s_52_4 -> i
        let s_52_5: i128 = (i128::try_from(s_52_4).unwrap());
        // D s_52_6: sub s_52_0 s_52_3
        let s_52_6: i128 = ((s_52_0) - (s_52_3));
        // D s_52_7: add s_52_6 s_52_5
        let s_52_7: i128 = (s_52_6 + s_52_5);
        // D s_52_8: bit-extract s_52_2 s_52_3 s_52_7
        let s_52_8: Bits = (Bits::new(
            ((s_52_2) >> (s_52_3)).value(),
            u16::try_from(s_52_7).unwrap(),
        ));
        // C s_52_9: const #55s : i
        let s_52_9: i128 = 55;
        // D s_52_10: read-var result:u64
        let s_52_10: u64 = fn_state.result;
        // D s_52_11: cast zx s_52_10 -> bv
        let s_52_11: Bits = Bits::new(s_52_10 as u128, 64u16);
        // C s_52_12: const #1u : u64
        let s_52_12: u64 = 1;
        // D s_52_13: bit-extract s_52_11 s_52_9 s_52_12
        let s_52_13: Bits = (Bits::new(
            ((s_52_11) >> (s_52_9)).value(),
            u16::try_from(s_52_12).unwrap(),
        ));
        // D s_52_14: cast reint s_52_13 -> u8
        let s_52_14: bool = ((s_52_13.value()) != 0);
        // C s_52_15: const #0s : i
        let s_52_15: i128 = 0;
        // C s_52_16: const #0u : u64
        let s_52_16: u64 = 0;
        // D s_52_17: cast zx s_52_14 -> u64
        let s_52_17: u64 = (s_52_14 as u64);
        // C s_52_18: const #1u : u64
        let s_52_18: u64 = 1;
        // D s_52_19: and s_52_17 s_52_18
        let s_52_19: u64 = ((s_52_17) & (s_52_18));
        // D s_52_20: cmp-eq s_52_19 s_52_18
        let s_52_20: bool = ((s_52_19) == (s_52_18));
        // D s_52_21: lsl s_52_17 s_52_15
        let s_52_21: u64 = s_52_17 << s_52_15;
        // D s_52_22: or s_52_16 s_52_21
        let s_52_22: u64 = ((s_52_16) | (s_52_21));
        // D s_52_23: cmpl s_52_21
        let s_52_23: u64 = !s_52_21;
        // D s_52_24: and s_52_16 s_52_23
        let s_52_24: u64 = ((s_52_16) & (s_52_23));
        // D s_52_25: select s_52_20 s_52_22 s_52_24
        let s_52_25: u64 = if s_52_20 { s_52_22 } else { s_52_24 };
        // D s_52_26: cast trunc s_52_25 -> u8
        let s_52_26: bool = ((s_52_25) != 0);
        // C s_52_27: const #55s : i
        let s_52_27: i128 = 55;
        // D s_52_28: read-var bottom_PAC_bit:i
        let s_52_28: i128 = fn_state.bottom_PAC_bit;
        // D s_52_29: sub s_52_27 s_52_28
        let s_52_29: i128 = ((s_52_27) - (s_52_28));
        // D s_52_30: cast reint s_52_29 -> i64
        let s_52_30: i64 = (s_52_29 as i64);
        // D s_52_31: cast zx s_52_26 -> bv
        let s_52_31: Bits = Bits::new(s_52_26 as u128, 1u16);
        // D s_52_32: cast zx s_52_30 -> i
        let s_52_32: i128 = (i128::try_from(s_52_30).unwrap());
        // D s_52_33: cast reint s_52_32 -> u64
        let s_52_33: u64 = (s_52_32 as u64);
        // D s_52_34: call replicate_bits_borealis_internal(s_52_31, s_52_33)
        let s_52_34: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_52_31,
            s_52_33,
        );
        // D s_52_35: cmp-ne s_52_8 s_52_34
        let s_52_35: bool = ((s_52_8) != (s_52_34));
        // N s_52_36: branch s_52_35 b61 b53
        if s_52_35 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_53_0: const #60s : i
        let s_53_0: i128 = 60;
        // D s_53_1: read-var result:u64
        let s_53_1: u64 = fn_state.result;
        // D s_53_2: cast zx s_53_1 -> bv
        let s_53_2: Bits = Bits::new(s_53_1 as u128, 64u16);
        // C s_53_3: const #1s : i64
        let s_53_3: i64 = 1;
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #3s : i
        let s_53_5: i128 = 3;
        // C s_53_6: add s_53_5 s_53_4
        let s_53_6: i128 = (s_53_5 + s_53_4);
        // D s_53_7: bit-extract s_53_2 s_53_0 s_53_6
        let s_53_7: Bits = (Bits::new(
            ((s_53_2) >> (s_53_0)).value(),
            u16::try_from(s_53_6).unwrap(),
        ));
        // D s_53_8: cast reint s_53_7 -> u8
        let s_53_8: u8 = (s_53_7.value() as u8);
        // C s_53_9: const #55s : i
        let s_53_9: i128 = 55;
        // D s_53_10: read-var result:u64
        let s_53_10: u64 = fn_state.result;
        // D s_53_11: cast zx s_53_10 -> bv
        let s_53_11: Bits = Bits::new(s_53_10 as u128, 64u16);
        // C s_53_12: const #1u : u64
        let s_53_12: u64 = 1;
        // D s_53_13: bit-extract s_53_11 s_53_9 s_53_12
        let s_53_13: Bits = (Bits::new(
            ((s_53_11) >> (s_53_9)).value(),
            u16::try_from(s_53_12).unwrap(),
        ));
        // D s_53_14: cast reint s_53_13 -> u8
        let s_53_14: bool = ((s_53_13.value()) != 0);
        // C s_53_15: const #0s : i
        let s_53_15: i128 = 0;
        // C s_53_16: const #0u : u64
        let s_53_16: u64 = 0;
        // D s_53_17: cast zx s_53_14 -> u64
        let s_53_17: u64 = (s_53_14 as u64);
        // C s_53_18: const #1u : u64
        let s_53_18: u64 = 1;
        // D s_53_19: and s_53_17 s_53_18
        let s_53_19: u64 = ((s_53_17) & (s_53_18));
        // D s_53_20: cmp-eq s_53_19 s_53_18
        let s_53_20: bool = ((s_53_19) == (s_53_18));
        // D s_53_21: lsl s_53_17 s_53_15
        let s_53_21: u64 = s_53_17 << s_53_15;
        // D s_53_22: or s_53_16 s_53_21
        let s_53_22: u64 = ((s_53_16) | (s_53_21));
        // D s_53_23: cmpl s_53_21
        let s_53_23: u64 = !s_53_21;
        // D s_53_24: and s_53_16 s_53_23
        let s_53_24: u64 = ((s_53_16) & (s_53_23));
        // D s_53_25: select s_53_20 s_53_22 s_53_24
        let s_53_25: u64 = if s_53_20 { s_53_22 } else { s_53_24 };
        // D s_53_26: cast trunc s_53_25 -> u8
        let s_53_26: bool = ((s_53_25) != 0);
        // D s_53_27: cast zx s_53_26 -> bv
        let s_53_27: Bits = Bits::new(s_53_26 as u128, 1u16);
        // C s_53_28: const #4u : u64
        let s_53_28: u64 = 4;
        // D s_53_29: call replicate_bits_borealis_internal(s_53_27, s_53_28)
        let s_53_29: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_53_27,
            s_53_28,
        );
        // D s_53_30: cast reint s_53_29 -> u8
        let s_53_30: u8 = (s_53_29.value() as u8);
        // D s_53_31: cast zx s_53_8 -> bv
        let s_53_31: Bits = Bits::new(s_53_8 as u128, 4u16);
        // D s_53_32: cast zx s_53_30 -> bv
        let s_53_32: Bits = Bits::new(s_53_30 as u128, 4u16);
        // D s_53_33: cmp-ne s_53_31 s_53_32
        let s_53_33: bool = ((s_53_31) != (s_53_32));
        // D s_53_34: write-var gs#14923 <= s_53_33
        fn_state.gs_14923 = s_53_33;
        // N s_53_35: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_54_0: read-var gs#14923:u8
        let s_54_0: bool = fn_state.gs_14923;
        // N s_54_1: branch s_54_0 b57 b55
        if s_54_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_56_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_57_0: read-var data:u8
        let s_57_0: bool = fn_state.data;
        // N s_57_1: branch s_57_0 b60 b58
        if s_57_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var ga#10971 <= s_58_0
        fn_state.ga_10971 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_59_0: read-var ga#10971:u8
        let s_59_0: bool = fn_state.ga_10971;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // D s_59_2: read-var key_number:u8
        let s_59_2: bool = fn_state.key_number;
        // D s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cast reint s_59_1 -> u128
        let s_59_4: u128 = (s_59_1.value() as u128);
        // D s_59_5: size-of s_59_1
        let s_59_5: u16 = s_59_1.length();
        // D s_59_6: cast reint s_59_3 -> u128
        let s_59_6: u128 = (s_59_3.value() as u128);
        // D s_59_7: size-of s_59_3
        let s_59_7: u16 = s_59_3.length();
        // D s_59_8: lsl s_59_4 s_59_7
        let s_59_8: u128 = s_59_4 << s_59_7;
        // D s_59_9: or s_59_8 s_59_6
        let s_59_9: u128 = ((s_59_8) | (s_59_6));
        // D s_59_10: add s_59_5 s_59_7
        let s_59_10: u16 = (s_59_5 + s_59_7);
        // D s_59_11: create-bits s_59_9 s_59_10
        let s_59_11: Bits = Bits::new(s_59_9, s_59_10);
        // D s_59_12: cast reint s_59_11 -> u8
        let s_59_12: u8 = (s_59_11.value() as u8);
        // D s_59_13: call AArch64_PACFailException(s_59_12)
        let s_59_13: () = AArch64_PACFailException(state, tracer, s_59_12);
        // N s_59_14: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var ga#10971 <= s_60_0
        fn_state.ga_10971 = s_60_0;
        // N s_60_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#14923 <= s_61_0
        fn_state.gs_14923 = s_61_0;
        // N s_61_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_62_0: read-var is_combined:u8
        let s_62_0: bool = fn_state.is_combined;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // D s_62_2: write-var gs#14911 <= s_62_1
        fn_state.gs_14911 = s_62_1;
        // N s_62_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#14912 <= s_63_0
        fn_state.gs_14912 = s_63_0;
        // N s_63_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_64_0: read-var bottom_PAC_bit:i
        let s_64_0: i128 = fn_state.bottom_PAC_bit;
        // D s_64_1: call __id(s_64_0)
        let s_64_1: i128 = u__id(state, tracer, s_64_0);
        // C s_64_2: const #54s : i
        let s_64_2: i128 = 54;
        // D s_64_3: cmp-le s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) <= (s_64_2));
        // D s_64_4: write-var gs#14901 <= s_64_3
        fn_state.gs_14901 = s_64_3;
        // N s_64_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call HaveEnhancedPAC2(s_65_0)
        let s_65_1: bool = HaveEnhancedPAC2(state, tracer, s_65_0);
        // S s_65_2: not s_65_1
        let s_65_2: bool = !s_65_1;
        // N s_65_3: branch s_65_2 b85 b66
        if s_65_2 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_66_0: read-var ptr:u64
        let s_66_0: u64 = fn_state.ptr;
        // D s_66_1: write-var result <= s_66_0
        fn_state.result = s_66_0;
        // D s_66_2: read-var bottom_PAC_bit:i
        let s_66_2: i128 = fn_state.bottom_PAC_bit;
        // D s_66_3: call __id(s_66_2)
        let s_66_3: i128 = u__id(state, tracer, s_66_2);
        // C s_66_4: const #0s : i
        let s_66_4: i128 = 0;
        // D s_66_5: cmp-le s_66_4 s_66_3
        let s_66_5: bool = ((s_66_4) <= (s_66_3));
        // N s_66_6: branch s_66_5 b84 b67
        if s_66_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#14932 <= s_67_0
        fn_state.gs_14932 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_68_0: read-var gs#14932:u8
        let s_68_0: bool = fn_state.gs_14932;
        // N s_68_1: assert s_68_0
        let s_68_1: () = assert!(s_68_0);
        // D s_68_2: read-var result:u64
        let s_68_2: u64 = fn_state.result;
        // D s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 64u16);
        // D s_68_4: read-var PAC:u64
        let s_68_4: u64 = fn_state.PAC;
        // D s_68_5: cast zx s_68_4 -> bv
        let s_68_5: Bits = Bits::new(s_68_4 as u128, 64u16);
        // D s_68_6: xor s_68_3 s_68_5
        let s_68_6: Bits = ((s_68_3) ^ (s_68_5));
        // D s_68_7: cast reint s_68_6 -> u64
        let s_68_7: u64 = (s_68_6.value() as u64);
        // C s_68_8: const #64s : i
        let s_68_8: i128 = 64;
        // C s_68_9: const #54s : i
        let s_68_9: i128 = 54;
        // C s_68_10: const #54s : i
        let s_68_10: i128 = 54;
        // D s_68_11: read-var result:u64
        let s_68_11: u64 = fn_state.result;
        // D s_68_12: cast zx s_68_11 -> bv
        let s_68_12: Bits = Bits::new(s_68_11 as u128, 64u16);
        // D s_68_13: cast zx s_68_7 -> bv
        let s_68_13: Bits = Bits::new(s_68_7 as u128, 64u16);
        // D s_68_14: read-var bottom_PAC_bit:i
        let s_68_14: i128 = fn_state.bottom_PAC_bit;
        // D s_68_15: read-var bottom_PAC_bit:i
        let s_68_15: i128 = fn_state.bottom_PAC_bit;
        // D s_68_16: call vector_update_subrange_from_subrange(s_68_8, s_68_12, s_68_9, s_68_14, s_68_13, s_68_10, s_68_15)
        let s_68_16: Bits = vector_update_subrange_from_subrange(
            state,
            tracer,
            s_68_8,
            s_68_12,
            s_68_9,
            s_68_14,
            s_68_13,
            s_68_10,
            s_68_15,
        );
        // D s_68_17: cast reint s_68_16 -> u64
        let s_68_17: u64 = (s_68_16.value() as u64);
        // D s_68_18: write-var result <= s_68_17
        fn_state.result = s_68_17;
        // C s_68_19: const #() : ()
        let s_68_19: () = ();
        // S s_68_20: call HaveFPACCombined(s_68_19)
        let s_68_20: bool = HaveFPACCombined(state, tracer, s_68_19);
        // N s_68_21: branch s_68_20 b83 b69
        if s_68_20 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call HaveFPAC(s_69_0)
        let s_69_1: bool = HaveFPAC(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b82 b70
        if s_69_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#14936 <= s_70_0
        fn_state.gs_14936 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_71_0: read-var gs#14936:u8
        let s_71_0: bool = fn_state.gs_14936;
        // D s_71_1: write-var gs#14937 <= s_71_0
        fn_state.gs_14937 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_72_0: read-var gs#14937:u8
        let s_72_0: bool = fn_state.gs_14937;
        // N s_72_1: branch s_72_0 b75 b73
        if s_72_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_73_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_74_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_75_0: const #54s : i
        let s_75_0: i128 = 54;
        // D s_75_1: read-var result:u64
        let s_75_1: u64 = fn_state.result;
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 64u16);
        // D s_75_3: read-var bottom_PAC_bit:i
        let s_75_3: i128 = fn_state.bottom_PAC_bit;
        // C s_75_4: const #1s : i64
        let s_75_4: i64 = 1;
        // C s_75_5: cast zx s_75_4 -> i
        let s_75_5: i128 = (i128::try_from(s_75_4).unwrap());
        // D s_75_6: sub s_75_0 s_75_3
        let s_75_6: i128 = ((s_75_0) - (s_75_3));
        // D s_75_7: add s_75_6 s_75_5
        let s_75_7: i128 = (s_75_6 + s_75_5);
        // D s_75_8: bit-extract s_75_2 s_75_3 s_75_7
        let s_75_8: Bits = (Bits::new(
            ((s_75_2) >> (s_75_3)).value(),
            u16::try_from(s_75_7).unwrap(),
        ));
        // C s_75_9: const #55s : i
        let s_75_9: i128 = 55;
        // D s_75_10: read-var result:u64
        let s_75_10: u64 = fn_state.result;
        // D s_75_11: cast zx s_75_10 -> bv
        let s_75_11: Bits = Bits::new(s_75_10 as u128, 64u16);
        // C s_75_12: const #1u : u64
        let s_75_12: u64 = 1;
        // D s_75_13: bit-extract s_75_11 s_75_9 s_75_12
        let s_75_13: Bits = (Bits::new(
            ((s_75_11) >> (s_75_9)).value(),
            u16::try_from(s_75_12).unwrap(),
        ));
        // D s_75_14: cast reint s_75_13 -> u8
        let s_75_14: bool = ((s_75_13.value()) != 0);
        // C s_75_15: const #0s : i
        let s_75_15: i128 = 0;
        // C s_75_16: const #0u : u64
        let s_75_16: u64 = 0;
        // D s_75_17: cast zx s_75_14 -> u64
        let s_75_17: u64 = (s_75_14 as u64);
        // C s_75_18: const #1u : u64
        let s_75_18: u64 = 1;
        // D s_75_19: and s_75_17 s_75_18
        let s_75_19: u64 = ((s_75_17) & (s_75_18));
        // D s_75_20: cmp-eq s_75_19 s_75_18
        let s_75_20: bool = ((s_75_19) == (s_75_18));
        // D s_75_21: lsl s_75_17 s_75_15
        let s_75_21: u64 = s_75_17 << s_75_15;
        // D s_75_22: or s_75_16 s_75_21
        let s_75_22: u64 = ((s_75_16) | (s_75_21));
        // D s_75_23: cmpl s_75_21
        let s_75_23: u64 = !s_75_21;
        // D s_75_24: and s_75_16 s_75_23
        let s_75_24: u64 = ((s_75_16) & (s_75_23));
        // D s_75_25: select s_75_20 s_75_22 s_75_24
        let s_75_25: u64 = if s_75_20 { s_75_22 } else { s_75_24 };
        // D s_75_26: cast trunc s_75_25 -> u8
        let s_75_26: bool = ((s_75_25) != 0);
        // C s_75_27: const #55s : i
        let s_75_27: i128 = 55;
        // D s_75_28: read-var bottom_PAC_bit:i
        let s_75_28: i128 = fn_state.bottom_PAC_bit;
        // D s_75_29: sub s_75_27 s_75_28
        let s_75_29: i128 = ((s_75_27) - (s_75_28));
        // D s_75_30: cast reint s_75_29 -> i64
        let s_75_30: i64 = (s_75_29 as i64);
        // D s_75_31: cast zx s_75_26 -> bv
        let s_75_31: Bits = Bits::new(s_75_26 as u128, 1u16);
        // D s_75_32: cast zx s_75_30 -> i
        let s_75_32: i128 = (i128::try_from(s_75_30).unwrap());
        // D s_75_33: cast reint s_75_32 -> u64
        let s_75_33: u64 = (s_75_32 as u64);
        // D s_75_34: call replicate_bits_borealis_internal(s_75_31, s_75_33)
        let s_75_34: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_75_31,
            s_75_33,
        );
        // D s_75_35: cmp-ne s_75_8 s_75_34
        let s_75_35: bool = ((s_75_8) != (s_75_34));
        // N s_75_36: branch s_75_35 b78 b76
        if s_75_35 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_76_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_77_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_78_0: read-var data:u8
        let s_78_0: bool = fn_state.data;
        // N s_78_1: branch s_78_0 b81 b79
        if s_78_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var ga#10947 <= s_79_0
        fn_state.ga_10947 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_80_0: read-var ga#10947:u8
        let s_80_0: bool = fn_state.ga_10947;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // D s_80_2: read-var key_number:u8
        let s_80_2: bool = fn_state.key_number;
        // D s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cast reint s_80_1 -> u128
        let s_80_4: u128 = (s_80_1.value() as u128);
        // D s_80_5: size-of s_80_1
        let s_80_5: u16 = s_80_1.length();
        // D s_80_6: cast reint s_80_3 -> u128
        let s_80_6: u128 = (s_80_3.value() as u128);
        // D s_80_7: size-of s_80_3
        let s_80_7: u16 = s_80_3.length();
        // D s_80_8: lsl s_80_4 s_80_7
        let s_80_8: u128 = s_80_4 << s_80_7;
        // D s_80_9: or s_80_8 s_80_6
        let s_80_9: u128 = ((s_80_8) | (s_80_6));
        // D s_80_10: add s_80_5 s_80_7
        let s_80_10: u16 = (s_80_5 + s_80_7);
        // D s_80_11: create-bits s_80_9 s_80_10
        let s_80_11: Bits = Bits::new(s_80_9, s_80_10);
        // D s_80_12: cast reint s_80_11 -> u8
        let s_80_12: u8 = (s_80_11.value() as u8);
        // D s_80_13: call AArch64_PACFailException(s_80_12)
        let s_80_13: () = AArch64_PACFailException(state, tracer, s_80_12);
        // N s_80_14: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var ga#10947 <= s_81_0
        fn_state.ga_10947 = s_81_0;
        // N s_81_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_82_0: read-var is_combined:u8
        let s_82_0: bool = fn_state.is_combined;
        // D s_82_1: not s_82_0
        let s_82_1: bool = !s_82_0;
        // D s_82_2: write-var gs#14936 <= s_82_1
        fn_state.gs_14936 = s_82_1;
        // N s_82_3: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#14937 <= s_83_0
        fn_state.gs_14937 = s_83_0;
        // N s_83_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_84_0: read-var bottom_PAC_bit:i
        let s_84_0: i128 = fn_state.bottom_PAC_bit;
        // D s_84_1: call __id(s_84_0)
        let s_84_1: i128 = u__id(state, tracer, s_84_0);
        // C s_84_2: const #54s : i
        let s_84_2: i128 = 54;
        // D s_84_3: cmp-le s_84_1 s_84_2
        let s_84_3: bool = ((s_84_1) <= (s_84_2));
        // D s_84_4: write-var gs#14932 <= s_84_3
        fn_state.gs_14932 = s_84_3;
        // N s_84_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_85_0: read-var bottom_PAC_bit:i
        let s_85_0: i128 = fn_state.bottom_PAC_bit;
        // D s_85_1: call __id(s_85_0)
        let s_85_1: i128 = u__id(state, tracer, s_85_0);
        // C s_85_2: const #0s : i
        let s_85_2: i128 = 0;
        // D s_85_3: cmp-le s_85_2 s_85_1
        let s_85_3: bool = ((s_85_2) <= (s_85_1));
        // N s_85_4: branch s_85_3 b90 b86
        if s_85_3 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#14949 <= s_86_0
        fn_state.gs_14949 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_87_0: read-var gs#14949:u8
        let s_87_0: bool = fn_state.gs_14949;
        // N s_87_1: assert s_87_0
        let s_87_1: () = assert!(s_87_0);
        // C s_87_2: const #54s : i
        let s_87_2: i128 = 54;
        // C s_87_3: const #54s : i
        let s_87_3: i128 = 54;
        // D s_87_4: read-var PAC:u64
        let s_87_4: u64 = fn_state.PAC;
        // D s_87_5: cast zx s_87_4 -> bv
        let s_87_5: Bits = Bits::new(s_87_4 as u128, 64u16);
        // D s_87_6: read-var ptr:u64
        let s_87_6: u64 = fn_state.ptr;
        // D s_87_7: cast zx s_87_6 -> bv
        let s_87_7: Bits = Bits::new(s_87_6 as u128, 64u16);
        // D s_87_8: read-var bottom_PAC_bit:i
        let s_87_8: i128 = fn_state.bottom_PAC_bit;
        // D s_87_9: read-var bottom_PAC_bit:i
        let s_87_9: i128 = fn_state.bottom_PAC_bit;
        // D s_87_10: call subrange_subrange_eq(s_87_5, s_87_2, s_87_8, s_87_7, s_87_3, s_87_9)
        let s_87_10: bool = subrange_subrange_eq(
            state,
            tracer,
            s_87_5,
            s_87_2,
            s_87_8,
            s_87_7,
            s_87_3,
            s_87_9,
        );
        // N s_87_11: branch s_87_10 b89 b88
        if s_87_10 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_88_0: read-var key_number:u8
        let s_88_0: bool = fn_state.key_number;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // D s_88_2: not s_88_1
        let s_88_2: Bits = !s_88_1;
        // D s_88_3: cast reint s_88_2 -> u8
        let s_88_3: bool = ((s_88_2.value()) != 0);
        // D s_88_4: read-var key_number:u8
        let s_88_4: bool = fn_state.key_number;
        // D s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 1u16);
        // D s_88_6: cast zx s_88_3 -> bv
        let s_88_6: Bits = Bits::new(s_88_3 as u128, 1u16);
        // D s_88_7: cast reint s_88_5 -> u128
        let s_88_7: u128 = (s_88_5.value() as u128);
        // D s_88_8: size-of s_88_5
        let s_88_8: u16 = s_88_5.length();
        // D s_88_9: cast reint s_88_6 -> u128
        let s_88_9: u128 = (s_88_6.value() as u128);
        // D s_88_10: size-of s_88_6
        let s_88_10: u16 = s_88_6.length();
        // D s_88_11: lsl s_88_7 s_88_10
        let s_88_11: u128 = s_88_7 << s_88_10;
        // D s_88_12: or s_88_11 s_88_9
        let s_88_12: u128 = ((s_88_11) | (s_88_9));
        // D s_88_13: add s_88_8 s_88_10
        let s_88_13: u16 = (s_88_8 + s_88_10);
        // D s_88_14: create-bits s_88_12 s_88_13
        let s_88_14: Bits = Bits::new(s_88_12, s_88_13);
        // D s_88_15: cast reint s_88_14 -> u8
        let s_88_15: u8 = (s_88_14.value() as u8);
        // C s_88_16: const #55s : i
        let s_88_16: i128 = 55;
        // D s_88_17: read-var original_ptr:u64
        let s_88_17: u64 = fn_state.original_ptr;
        // D s_88_18: cast zx s_88_17 -> bv
        let s_88_18: Bits = Bits::new(s_88_17 as u128, 64u16);
        // C s_88_19: const #1s : i64
        let s_88_19: i64 = 1;
        // C s_88_20: cast zx s_88_19 -> i
        let s_88_20: i128 = (i128::try_from(s_88_19).unwrap());
        // C s_88_21: const #8s : i
        let s_88_21: i128 = 8;
        // C s_88_22: add s_88_21 s_88_20
        let s_88_22: i128 = (s_88_21 + s_88_20);
        // D s_88_23: bit-extract s_88_18 s_88_16 s_88_22
        let s_88_23: Bits = (Bits::new(
            ((s_88_18) >> (s_88_16)).value(),
            u16::try_from(s_88_22).unwrap(),
        ));
        // D s_88_24: cast reint s_88_23 -> u9
        let s_88_24: u16 = (s_88_23.value() as u16);
        // D s_88_25: cast zx s_88_24 -> bv
        let s_88_25: Bits = Bits::new(s_88_24 as u128, 9u16);
        // D s_88_26: cast zx s_88_15 -> bv
        let s_88_26: Bits = Bits::new(s_88_15 as u128, 2u16);
        // D s_88_27: cast reint s_88_25 -> u128
        let s_88_27: u128 = (s_88_25.value() as u128);
        // D s_88_28: size-of s_88_25
        let s_88_28: u16 = s_88_25.length();
        // D s_88_29: cast reint s_88_26 -> u128
        let s_88_29: u128 = (s_88_26.value() as u128);
        // D s_88_30: size-of s_88_26
        let s_88_30: u16 = s_88_26.length();
        // D s_88_31: lsl s_88_27 s_88_30
        let s_88_31: u128 = s_88_27 << s_88_30;
        // D s_88_32: or s_88_31 s_88_29
        let s_88_32: u128 = ((s_88_31) | (s_88_29));
        // D s_88_33: add s_88_28 s_88_30
        let s_88_33: u16 = (s_88_28 + s_88_30);
        // D s_88_34: create-bits s_88_32 s_88_33
        let s_88_34: Bits = Bits::new(s_88_32, s_88_33);
        // D s_88_35: cast reint s_88_34 -> u11
        let s_88_35: u16 = (s_88_34.value() as u16);
        // C s_88_36: const #0s : i
        let s_88_36: i128 = 0;
        // D s_88_37: read-var original_ptr:u64
        let s_88_37: u64 = fn_state.original_ptr;
        // D s_88_38: cast zx s_88_37 -> bv
        let s_88_38: Bits = Bits::new(s_88_37 as u128, 64u16);
        // C s_88_39: const #1s : i64
        let s_88_39: i64 = 1;
        // C s_88_40: cast zx s_88_39 -> i
        let s_88_40: i128 = (i128::try_from(s_88_39).unwrap());
        // C s_88_41: const #52s : i
        let s_88_41: i128 = 52;
        // C s_88_42: add s_88_41 s_88_40
        let s_88_42: i128 = (s_88_41 + s_88_40);
        // D s_88_43: bit-extract s_88_38 s_88_36 s_88_42
        let s_88_43: Bits = (Bits::new(
            ((s_88_38) >> (s_88_36)).value(),
            u16::try_from(s_88_42).unwrap(),
        ));
        // D s_88_44: cast reint s_88_43 -> u53
        let s_88_44: u64 = (s_88_43.value() as u64);
        // D s_88_45: cast zx s_88_35 -> bv
        let s_88_45: Bits = Bits::new(s_88_35 as u128, 11u16);
        // D s_88_46: cast zx s_88_44 -> bv
        let s_88_46: Bits = Bits::new(s_88_44 as u128, 53u16);
        // D s_88_47: cast reint s_88_45 -> u128
        let s_88_47: u128 = (s_88_45.value() as u128);
        // D s_88_48: size-of s_88_45
        let s_88_48: u16 = s_88_45.length();
        // D s_88_49: cast reint s_88_46 -> u128
        let s_88_49: u128 = (s_88_46.value() as u128);
        // D s_88_50: size-of s_88_46
        let s_88_50: u16 = s_88_46.length();
        // D s_88_51: lsl s_88_47 s_88_50
        let s_88_51: u128 = s_88_47 << s_88_50;
        // D s_88_52: or s_88_51 s_88_49
        let s_88_52: u128 = ((s_88_51) | (s_88_49));
        // D s_88_53: add s_88_48 s_88_50
        let s_88_53: u16 = (s_88_48 + s_88_50);
        // D s_88_54: create-bits s_88_52 s_88_53
        let s_88_54: Bits = Bits::new(s_88_52, s_88_53);
        // D s_88_55: cast reint s_88_54 -> u64
        let s_88_55: u64 = (s_88_54.value() as u64);
        // D s_88_56: write-var result <= s_88_55
        fn_state.result = s_88_55;
        // N s_88_57: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_89_0: read-var original_ptr:u64
        let s_89_0: u64 = fn_state.original_ptr;
        // D s_89_1: write-var result <= s_89_0
        fn_state.result = s_89_0;
        // N s_89_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_90_0: read-var bottom_PAC_bit:i
        let s_90_0: i128 = fn_state.bottom_PAC_bit;
        // D s_90_1: call __id(s_90_0)
        let s_90_1: i128 = u__id(state, tracer, s_90_0);
        // C s_90_2: const #54s : i
        let s_90_2: i128 = 54;
        // D s_90_3: cmp-le s_90_1 s_90_2
        let s_90_3: bool = ((s_90_1) <= (s_90_2));
        // D s_90_4: write-var gs#14949 <= s_90_3
        fn_state.gs_14949 = s_90_3;
        // N s_90_5: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_91_0: read-var bottom_PAC_bit:i
        let s_91_0: i128 = fn_state.bottom_PAC_bit;
        // D s_91_1: call __id(s_91_0)
        let s_91_1: i128 = u__id(state, tracer, s_91_0);
        // D s_91_2: cast reint s_91_1 -> i64
        let s_91_2: i64 = (s_91_1 as i64);
        // C s_91_3: const #1s : i
        let s_91_3: i128 = 1;
        // D s_91_4: cast zx s_91_2 -> i
        let s_91_4: i128 = (i128::try_from(s_91_2).unwrap());
        // D s_91_5: sub s_91_4 s_91_3
        let s_91_5: i128 = ((s_91_4) - (s_91_3));
        // D s_91_6: cast reint s_91_5 -> i64
        let s_91_6: i64 = (s_91_5 as i64);
        // C s_91_7: const #64s : i
        let s_91_7: i128 = 64;
        // D s_91_8: cast zx s_91_6 -> i
        let s_91_8: i128 = (i128::try_from(s_91_6).unwrap());
        // D s_91_9: cmp-lt s_91_8 s_91_7
        let s_91_9: bool = ((s_91_8) < (s_91_7));
        // D s_91_10: write-var gs#14799 <= s_91_9
        fn_state.gs_14799 = s_91_9;
        // N s_91_11: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_92_0: read-var bottom_PAC_bit:i
        let s_92_0: i128 = fn_state.bottom_PAC_bit;
        // D s_92_1: call __id(s_92_0)
        let s_92_1: i128 = u__id(state, tracer, s_92_0);
        // C s_92_2: const #64s : i
        let s_92_2: i128 = 64;
        // D s_92_3: sub s_92_2 s_92_1
        let s_92_3: i128 = ((s_92_2) - (s_92_1));
        // C s_92_4: const #1s : i
        let s_92_4: i128 = 1;
        // D s_92_5: sub s_92_3 s_92_4
        let s_92_5: i128 = ((s_92_3) - (s_92_4));
        // C s_92_6: const #64s : i
        let s_92_6: i128 = 64;
        // D s_92_7: cmp-lt s_92_5 s_92_6
        let s_92_7: bool = ((s_92_5) < (s_92_6));
        // D s_92_8: write-var gs#14794 <= s_92_7
        fn_state.gs_14794 = s_92_7;
        // N s_92_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_93_0: read-var bottom_PAC_bit:i
        let s_93_0: i128 = fn_state.bottom_PAC_bit;
        // D s_93_1: call __id(s_93_0)
        let s_93_1: i128 = u__id(state, tracer, s_93_0);
        // C s_93_2: const #56s : i
        let s_93_2: i128 = 56;
        // D s_93_3: sub s_93_2 s_93_1
        let s_93_3: i128 = ((s_93_2) - (s_93_1));
        // C s_93_4: const #1s : i
        let s_93_4: i128 = 1;
        // D s_93_5: sub s_93_3 s_93_4
        let s_93_5: i128 = ((s_93_3) - (s_93_4));
        // C s_93_6: const #0s : i
        let s_93_6: i128 = 0;
        // D s_93_7: cmp-le s_93_6 s_93_5
        let s_93_7: bool = ((s_93_6) <= (s_93_5));
        // N s_93_8: branch s_93_7 b99 b94
        if s_93_7 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#14814 <= s_94_0
        fn_state.gs_14814 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_95_0: read-var gs#14814:u8
        let s_95_0: bool = fn_state.gs_14814;
        // N s_95_1: assert s_95_0
        let s_95_1: () = assert!(s_95_0);
        // D s_95_2: read-var bottom_PAC_bit:i
        let s_95_2: i128 = fn_state.bottom_PAC_bit;
        // D s_95_3: call __id(s_95_2)
        let s_95_3: i128 = u__id(state, tracer, s_95_2);
        // D s_95_4: cast reint s_95_3 -> i64
        let s_95_4: i64 = (s_95_3 as i64);
        // C s_95_5: const #1s : i
        let s_95_5: i128 = 1;
        // D s_95_6: cast zx s_95_4 -> i
        let s_95_6: i128 = (i128::try_from(s_95_4).unwrap());
        // D s_95_7: sub s_95_6 s_95_5
        let s_95_7: i128 = ((s_95_6) - (s_95_5));
        // D s_95_8: cast reint s_95_7 -> i64
        let s_95_8: i64 = (s_95_7 as i64);
        // C s_95_9: const #0s : i
        let s_95_9: i128 = 0;
        // D s_95_10: cast zx s_95_8 -> i
        let s_95_10: i128 = (i128::try_from(s_95_8).unwrap());
        // D s_95_11: cmp-le s_95_9 s_95_10
        let s_95_11: bool = ((s_95_9) <= (s_95_10));
        // N s_95_12: branch s_95_11 b98 b96
        if s_95_11 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#14819 <= s_96_0
        fn_state.gs_14819 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_97_0: read-var gs#14819:u8
        let s_97_0: bool = fn_state.gs_14819;
        // N s_97_1: assert s_97_0
        let s_97_1: () = assert!(s_97_0);
        // C s_97_2: const #60s : i
        let s_97_2: i128 = 60;
        // D s_97_3: read-var extfield:u64
        let s_97_3: u64 = fn_state.extfield;
        // D s_97_4: cast zx s_97_3 -> bv
        let s_97_4: Bits = Bits::new(s_97_3 as u128, 64u16);
        // C s_97_5: const #1s : i64
        let s_97_5: i64 = 1;
        // C s_97_6: cast zx s_97_5 -> i
        let s_97_6: i128 = (i128::try_from(s_97_5).unwrap());
        // C s_97_7: const #3s : i
        let s_97_7: i128 = 3;
        // C s_97_8: add s_97_7 s_97_6
        let s_97_8: i128 = (s_97_7 + s_97_6);
        // D s_97_9: bit-extract s_97_4 s_97_2 s_97_8
        let s_97_9: Bits = (Bits::new(
            ((s_97_4) >> (s_97_2)).value(),
            u16::try_from(s_97_8).unwrap(),
        ));
        // D s_97_10: cast reint s_97_9 -> u8
        let s_97_10: u8 = (s_97_9.value() as u8);
        // C s_97_11: const #56s : i
        let s_97_11: i128 = 56;
        // D s_97_12: read-var ptr:u64
        let s_97_12: u64 = fn_state.ptr;
        // D s_97_13: cast zx s_97_12 -> bv
        let s_97_13: Bits = Bits::new(s_97_12 as u128, 64u16);
        // C s_97_14: const #1s : i64
        let s_97_14: i64 = 1;
        // C s_97_15: cast zx s_97_14 -> i
        let s_97_15: i128 = (i128::try_from(s_97_14).unwrap());
        // C s_97_16: const #3s : i
        let s_97_16: i128 = 3;
        // C s_97_17: add s_97_16 s_97_15
        let s_97_17: i128 = (s_97_16 + s_97_15);
        // D s_97_18: bit-extract s_97_13 s_97_11 s_97_17
        let s_97_18: Bits = (Bits::new(
            ((s_97_13) >> (s_97_11)).value(),
            u16::try_from(s_97_17).unwrap(),
        ));
        // D s_97_19: cast reint s_97_18 -> u8
        let s_97_19: u8 = (s_97_18.value() as u8);
        // D s_97_20: cast zx s_97_10 -> bv
        let s_97_20: Bits = Bits::new(s_97_10 as u128, 4u16);
        // D s_97_21: cast zx s_97_19 -> bv
        let s_97_21: Bits = Bits::new(s_97_19 as u128, 4u16);
        // D s_97_22: cast reint s_97_20 -> u128
        let s_97_22: u128 = (s_97_20.value() as u128);
        // D s_97_23: size-of s_97_20
        let s_97_23: u16 = s_97_20.length();
        // D s_97_24: cast reint s_97_21 -> u128
        let s_97_24: u128 = (s_97_21.value() as u128);
        // D s_97_25: size-of s_97_21
        let s_97_25: u16 = s_97_21.length();
        // D s_97_26: lsl s_97_22 s_97_25
        let s_97_26: u128 = s_97_22 << s_97_25;
        // D s_97_27: or s_97_26 s_97_24
        let s_97_27: u128 = ((s_97_26) | (s_97_24));
        // D s_97_28: add s_97_23 s_97_25
        let s_97_28: u16 = (s_97_23 + s_97_25);
        // D s_97_29: create-bits s_97_27 s_97_28
        let s_97_29: Bits = Bits::new(s_97_27, s_97_28);
        // D s_97_30: cast reint s_97_29 -> u8
        let s_97_30: u8 = (s_97_29.value() as u8);
        // C s_97_31: const #56s : i
        let s_97_31: i128 = 56;
        // D s_97_32: read-var bottom_PAC_bit:i
        let s_97_32: i128 = fn_state.bottom_PAC_bit;
        // D s_97_33: sub s_97_31 s_97_32
        let s_97_33: i128 = ((s_97_31) - (s_97_32));
        // D s_97_34: cast reint s_97_33 -> i64
        let s_97_34: i64 = (s_97_33 as i64);
        // C s_97_35: const #1s : i
        let s_97_35: i128 = 1;
        // D s_97_36: cast zx s_97_34 -> i
        let s_97_36: i128 = (i128::try_from(s_97_34).unwrap());
        // D s_97_37: sub s_97_36 s_97_35
        let s_97_37: i128 = ((s_97_36) - (s_97_35));
        // D s_97_38: cast reint s_97_37 -> i64
        let s_97_38: i64 = (s_97_37 as i64);
        // C s_97_39: const #1s : i
        let s_97_39: i128 = 1;
        // D s_97_40: read-var bottom_PAC_bit:i
        let s_97_40: i128 = fn_state.bottom_PAC_bit;
        // D s_97_41: sub s_97_40 s_97_39
        let s_97_41: i128 = ((s_97_40) - (s_97_39));
        // D s_97_42: cast reint s_97_41 -> i64
        let s_97_42: i64 = (s_97_41 as i64);
        // C s_97_43: const #56s : i
        let s_97_43: i128 = 56;
        // C s_97_44: const #0s : i
        let s_97_44: i128 = 0;
        // C s_97_45: const #0s : i
        let s_97_45: i128 = 0;
        // D s_97_46: read-var extfield:u64
        let s_97_46: u64 = fn_state.extfield;
        // D s_97_47: cast zx s_97_46 -> bv
        let s_97_47: Bits = Bits::new(s_97_46 as u128, 64u16);
        // D s_97_48: cast zx s_97_38 -> i
        let s_97_48: i128 = (i128::try_from(s_97_38).unwrap());
        // D s_97_49: read-var ptr:u64
        let s_97_49: u64 = fn_state.ptr;
        // D s_97_50: cast zx s_97_49 -> bv
        let s_97_50: Bits = Bits::new(s_97_49 as u128, 64u16);
        // D s_97_51: cast zx s_97_42 -> i
        let s_97_51: i128 = (i128::try_from(s_97_42).unwrap());
        // D s_97_52: call subrange_subrange_concat(s_97_43, s_97_47, s_97_48, s_97_44, s_97_50, s_97_51, s_97_45)
        let s_97_52: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_97_43,
            s_97_47,
            s_97_48,
            s_97_44,
            s_97_50,
            s_97_51,
            s_97_45,
        );
        // D s_97_53: cast reint s_97_52 -> u56
        let s_97_53: u64 = (s_97_52.value() as u64);
        // D s_97_54: cast zx s_97_30 -> bv
        let s_97_54: Bits = Bits::new(s_97_30 as u128, 8u16);
        // D s_97_55: cast zx s_97_53 -> bv
        let s_97_55: Bits = Bits::new(s_97_53 as u128, 56u16);
        // D s_97_56: cast reint s_97_54 -> u128
        let s_97_56: u128 = (s_97_54.value() as u128);
        // D s_97_57: size-of s_97_54
        let s_97_57: u16 = s_97_54.length();
        // D s_97_58: cast reint s_97_55 -> u128
        let s_97_58: u128 = (s_97_55.value() as u128);
        // D s_97_59: size-of s_97_55
        let s_97_59: u16 = s_97_55.length();
        // D s_97_60: lsl s_97_56 s_97_59
        let s_97_60: u128 = s_97_56 << s_97_59;
        // D s_97_61: or s_97_60 s_97_58
        let s_97_61: u128 = ((s_97_60) | (s_97_58));
        // D s_97_62: add s_97_57 s_97_59
        let s_97_62: u16 = (s_97_57 + s_97_59);
        // D s_97_63: create-bits s_97_61 s_97_62
        let s_97_63: Bits = Bits::new(s_97_61, s_97_62);
        // D s_97_64: cast reint s_97_63 -> u64
        let s_97_64: u64 = (s_97_63.value() as u64);
        // D s_97_65: write-var original_ptr <= s_97_64
        fn_state.original_ptr = s_97_64;
        // N s_97_66: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_98_0: read-var bottom_PAC_bit:i
        let s_98_0: i128 = fn_state.bottom_PAC_bit;
        // D s_98_1: call __id(s_98_0)
        let s_98_1: i128 = u__id(state, tracer, s_98_0);
        // D s_98_2: cast reint s_98_1 -> i64
        let s_98_2: i64 = (s_98_1 as i64);
        // C s_98_3: const #1s : i
        let s_98_3: i128 = 1;
        // D s_98_4: cast zx s_98_2 -> i
        let s_98_4: i128 = (i128::try_from(s_98_2).unwrap());
        // D s_98_5: sub s_98_4 s_98_3
        let s_98_5: i128 = ((s_98_4) - (s_98_3));
        // D s_98_6: cast reint s_98_5 -> i64
        let s_98_6: i64 = (s_98_5 as i64);
        // C s_98_7: const #64s : i
        let s_98_7: i128 = 64;
        // D s_98_8: cast zx s_98_6 -> i
        let s_98_8: i128 = (i128::try_from(s_98_6).unwrap());
        // D s_98_9: cmp-lt s_98_8 s_98_7
        let s_98_9: bool = ((s_98_8) < (s_98_7));
        // D s_98_10: write-var gs#14819 <= s_98_9
        fn_state.gs_14819 = s_98_9;
        // N s_98_11: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_99_0: read-var bottom_PAC_bit:i
        let s_99_0: i128 = fn_state.bottom_PAC_bit;
        // D s_99_1: call __id(s_99_0)
        let s_99_1: i128 = u__id(state, tracer, s_99_0);
        // C s_99_2: const #56s : i
        let s_99_2: i128 = 56;
        // D s_99_3: sub s_99_2 s_99_1
        let s_99_3: i128 = ((s_99_2) - (s_99_1));
        // C s_99_4: const #1s : i
        let s_99_4: i128 = 1;
        // D s_99_5: sub s_99_3 s_99_4
        let s_99_5: i128 = ((s_99_3) - (s_99_4));
        // C s_99_6: const #64s : i
        let s_99_6: i128 = 64;
        // D s_99_7: cmp-lt s_99_5 s_99_6
        let s_99_7: bool = ((s_99_5) < (s_99_6));
        // D s_99_8: write-var gs#14814 <= s_99_7
        fn_state.gs_14814 = s_99_7;
        // N s_99_9: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_100_0: read-var bottom_PAC_bit:i
        let s_100_0: i128 = fn_state.bottom_PAC_bit;
        // D s_100_1: call __id(s_100_0)
        let s_100_1: i128 = u__id(state, tracer, s_100_0);
        // C s_100_2: const #56s : i
        let s_100_2: i128 = 56;
        // D s_100_3: sub s_100_2 s_100_1
        let s_100_3: i128 = ((s_100_2) - (s_100_1));
        // C s_100_4: const #1s : i
        let s_100_4: i128 = 1;
        // D s_100_5: sub s_100_3 s_100_4
        let s_100_5: i128 = ((s_100_3) - (s_100_4));
        // C s_100_6: const #0s : i
        let s_100_6: i128 = 0;
        // D s_100_7: cmp-le s_100_6 s_100_5
        let s_100_7: bool = ((s_100_6) <= (s_100_5));
        // N s_100_8: branch s_100_7 b106 b101
        if s_100_7 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#14838 <= s_101_0
        fn_state.gs_14838 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_102_0: read-var gs#14838:u8
        let s_102_0: bool = fn_state.gs_14838;
        // N s_102_1: assert s_102_0
        let s_102_1: () = assert!(s_102_0);
        // D s_102_2: read-var bottom_PAC_bit:i
        let s_102_2: i128 = fn_state.bottom_PAC_bit;
        // D s_102_3: call __id(s_102_2)
        let s_102_3: i128 = u__id(state, tracer, s_102_2);
        // D s_102_4: cast reint s_102_3 -> i64
        let s_102_4: i64 = (s_102_3 as i64);
        // C s_102_5: const #1s : i
        let s_102_5: i128 = 1;
        // D s_102_6: cast zx s_102_4 -> i
        let s_102_6: i128 = (i128::try_from(s_102_4).unwrap());
        // D s_102_7: sub s_102_6 s_102_5
        let s_102_7: i128 = ((s_102_6) - (s_102_5));
        // D s_102_8: cast reint s_102_7 -> i64
        let s_102_8: i64 = (s_102_7 as i64);
        // C s_102_9: const #0s : i
        let s_102_9: i128 = 0;
        // D s_102_10: cast zx s_102_8 -> i
        let s_102_10: i128 = (i128::try_from(s_102_8).unwrap());
        // D s_102_11: cmp-le s_102_9 s_102_10
        let s_102_11: bool = ((s_102_9) <= (s_102_10));
        // N s_102_12: branch s_102_11 b105 b103
        if s_102_11 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#14843 <= s_103_0
        fn_state.gs_14843 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_104_0: read-var gs#14843:u8
        let s_104_0: bool = fn_state.gs_14843;
        // N s_104_1: assert s_104_0
        let s_104_1: () = assert!(s_104_0);
        // C s_104_2: const #56s : i
        let s_104_2: i128 = 56;
        // D s_104_3: read-var ptr:u64
        let s_104_3: u64 = fn_state.ptr;
        // D s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 64u16);
        // C s_104_5: const #1s : i64
        let s_104_5: i64 = 1;
        // C s_104_6: cast zx s_104_5 -> i
        let s_104_6: i128 = (i128::try_from(s_104_5).unwrap());
        // C s_104_7: const #7s : i
        let s_104_7: i128 = 7;
        // C s_104_8: add s_104_7 s_104_6
        let s_104_8: i128 = (s_104_7 + s_104_6);
        // D s_104_9: bit-extract s_104_4 s_104_2 s_104_8
        let s_104_9: Bits = (Bits::new(
            ((s_104_4) >> (s_104_2)).value(),
            u16::try_from(s_104_8).unwrap(),
        ));
        // D s_104_10: cast reint s_104_9 -> u8
        let s_104_10: u8 = (s_104_9.value() as u8);
        // C s_104_11: const #56s : i
        let s_104_11: i128 = 56;
        // D s_104_12: read-var bottom_PAC_bit:i
        let s_104_12: i128 = fn_state.bottom_PAC_bit;
        // D s_104_13: sub s_104_11 s_104_12
        let s_104_13: i128 = ((s_104_11) - (s_104_12));
        // D s_104_14: cast reint s_104_13 -> i64
        let s_104_14: i64 = (s_104_13 as i64);
        // C s_104_15: const #1s : i
        let s_104_15: i128 = 1;
        // D s_104_16: cast zx s_104_14 -> i
        let s_104_16: i128 = (i128::try_from(s_104_14).unwrap());
        // D s_104_17: sub s_104_16 s_104_15
        let s_104_17: i128 = ((s_104_16) - (s_104_15));
        // D s_104_18: cast reint s_104_17 -> i64
        let s_104_18: i64 = (s_104_17 as i64);
        // C s_104_19: const #1s : i
        let s_104_19: i128 = 1;
        // D s_104_20: read-var bottom_PAC_bit:i
        let s_104_20: i128 = fn_state.bottom_PAC_bit;
        // D s_104_21: sub s_104_20 s_104_19
        let s_104_21: i128 = ((s_104_20) - (s_104_19));
        // D s_104_22: cast reint s_104_21 -> i64
        let s_104_22: i64 = (s_104_21 as i64);
        // C s_104_23: const #56s : i
        let s_104_23: i128 = 56;
        // C s_104_24: const #0s : i
        let s_104_24: i128 = 0;
        // C s_104_25: const #0s : i
        let s_104_25: i128 = 0;
        // D s_104_26: read-var extfield:u64
        let s_104_26: u64 = fn_state.extfield;
        // D s_104_27: cast zx s_104_26 -> bv
        let s_104_27: Bits = Bits::new(s_104_26 as u128, 64u16);
        // D s_104_28: cast zx s_104_18 -> i
        let s_104_28: i128 = (i128::try_from(s_104_18).unwrap());
        // D s_104_29: read-var ptr:u64
        let s_104_29: u64 = fn_state.ptr;
        // D s_104_30: cast zx s_104_29 -> bv
        let s_104_30: Bits = Bits::new(s_104_29 as u128, 64u16);
        // D s_104_31: cast zx s_104_22 -> i
        let s_104_31: i128 = (i128::try_from(s_104_22).unwrap());
        // D s_104_32: call subrange_subrange_concat(s_104_23, s_104_27, s_104_28, s_104_24, s_104_30, s_104_31, s_104_25)
        let s_104_32: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_104_23,
            s_104_27,
            s_104_28,
            s_104_24,
            s_104_30,
            s_104_31,
            s_104_25,
        );
        // D s_104_33: cast reint s_104_32 -> u56
        let s_104_33: u64 = (s_104_32.value() as u64);
        // D s_104_34: cast zx s_104_10 -> bv
        let s_104_34: Bits = Bits::new(s_104_10 as u128, 8u16);
        // D s_104_35: cast zx s_104_33 -> bv
        let s_104_35: Bits = Bits::new(s_104_33 as u128, 56u16);
        // D s_104_36: cast reint s_104_34 -> u128
        let s_104_36: u128 = (s_104_34.value() as u128);
        // D s_104_37: size-of s_104_34
        let s_104_37: u16 = s_104_34.length();
        // D s_104_38: cast reint s_104_35 -> u128
        let s_104_38: u128 = (s_104_35.value() as u128);
        // D s_104_39: size-of s_104_35
        let s_104_39: u16 = s_104_35.length();
        // D s_104_40: lsl s_104_36 s_104_39
        let s_104_40: u128 = s_104_36 << s_104_39;
        // D s_104_41: or s_104_40 s_104_38
        let s_104_41: u128 = ((s_104_40) | (s_104_38));
        // D s_104_42: add s_104_37 s_104_39
        let s_104_42: u16 = (s_104_37 + s_104_39);
        // D s_104_43: create-bits s_104_41 s_104_42
        let s_104_43: Bits = Bits::new(s_104_41, s_104_42);
        // D s_104_44: cast reint s_104_43 -> u64
        let s_104_44: u64 = (s_104_43.value() as u64);
        // D s_104_45: write-var original_ptr <= s_104_44
        fn_state.original_ptr = s_104_44;
        // N s_104_46: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_105_0: read-var bottom_PAC_bit:i
        let s_105_0: i128 = fn_state.bottom_PAC_bit;
        // D s_105_1: call __id(s_105_0)
        let s_105_1: i128 = u__id(state, tracer, s_105_0);
        // D s_105_2: cast reint s_105_1 -> i64
        let s_105_2: i64 = (s_105_1 as i64);
        // C s_105_3: const #1s : i
        let s_105_3: i128 = 1;
        // D s_105_4: cast zx s_105_2 -> i
        let s_105_4: i128 = (i128::try_from(s_105_2).unwrap());
        // D s_105_5: sub s_105_4 s_105_3
        let s_105_5: i128 = ((s_105_4) - (s_105_3));
        // D s_105_6: cast reint s_105_5 -> i64
        let s_105_6: i64 = (s_105_5 as i64);
        // C s_105_7: const #64s : i
        let s_105_7: i128 = 64;
        // D s_105_8: cast zx s_105_6 -> i
        let s_105_8: i128 = (i128::try_from(s_105_6).unwrap());
        // D s_105_9: cmp-lt s_105_8 s_105_7
        let s_105_9: bool = ((s_105_8) < (s_105_7));
        // D s_105_10: write-var gs#14843 <= s_105_9
        fn_state.gs_14843 = s_105_9;
        // N s_105_11: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_106_0: read-var bottom_PAC_bit:i
        let s_106_0: i128 = fn_state.bottom_PAC_bit;
        // D s_106_1: call __id(s_106_0)
        let s_106_1: i128 = u__id(state, tracer, s_106_0);
        // C s_106_2: const #56s : i
        let s_106_2: i128 = 56;
        // D s_106_3: sub s_106_2 s_106_1
        let s_106_3: i128 = ((s_106_2) - (s_106_1));
        // C s_106_4: const #1s : i
        let s_106_4: i128 = 1;
        // D s_106_5: sub s_106_3 s_106_4
        let s_106_5: i128 = ((s_106_3) - (s_106_4));
        // C s_106_6: const #64s : i
        let s_106_6: i128 = 64;
        // D s_106_7: cmp-lt s_106_5 s_106_6
        let s_106_7: bool = ((s_106_5) < (s_106_6));
        // D s_106_8: write-var gs#14838 <= s_106_7
        fn_state.gs_14838 = s_106_7;
        // N s_106_9: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_107_0: read-var ptr:u64
        let s_107_0: u64 = fn_state.ptr;
        // D s_107_1: write-var return_value <= s_107_0
        fn_state.return_value = s_107_0;
        // N s_107_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_108_0: const #55s : i
        let s_108_0: i128 = 55;
        // D s_108_1: read-var bottom_PAC_bit:i
        let s_108_1: i128 = fn_state.bottom_PAC_bit;
        // D s_108_2: cmp-ge s_108_1 s_108_0
        let s_108_2: bool = ((s_108_1) >= (s_108_0));
        // D s_108_3: write-var gs#14786 <= s_108_2
        fn_state.gs_14786 = s_108_2;
        // N s_108_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
