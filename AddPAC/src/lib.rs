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
use u_get_TCR_EL2_Type_TBID1::*;
use EffectiveTBI::*;
use ConstPACField::*;
use EffectiveMTX::*;
use u_get_TCR_EL1_Type_TBID0::*;
use S1TranslationRegime__1::*;
use Zeros::*;
use HaveEnhancedPAC::*;
use ComputePAC::*;
use u_get_TCR_EL2_Type_TBI1::*;
use PtrHasUpperAndLowerAddRanges::*;
use CalculateBottomPACBit::*;
use u_get_TCR_EL2_Type_TBI0::*;
use u__id::*;
use Ones::*;
use IsZero::*;
use u_get_TCR_EL1_Type_TBI1::*;
use replicate_bits_borealis_internal::*;
use u_get_TCR_EL1_Type_TBI0::*;
use Bit::*;
use u_get_TCR_EL2_Type_TBID0::*;
use u_get_TCR_EL1_Type_TBID1::*;
use subrange_subrange_concat::*;
use HaveEnhancedPAC2::*;
use common::*;
pub fn AddPAC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ptr: u64,
    modifier: u64,
    K: u128,
    data: bool,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_14991: bool,
        tbi: bool,
        gs_14979: bool,
        gs_15005: bool,
        unusedbits_mask: u64,
        gs_15134: bool,
        PAC: u64,
        extfield: u64,
        gs_14977: bool,
        gs_14998: bool,
        gs_14980: bool,
        gs_15059: bool,
        gs_15082: bool,
        bottom_PAC_bit: i128,
        gs_15040: bool,
        ext_ptr: u64,
        gs_15015: bool,
        result: u64,
        gs_14968: bool,
        gs_15064: bool,
        top_bit: i64,
        gs_15035: bool,
        gs_14981: bool,
        mtx: bool,
        return_value: u64,
        gs_14993: bool,
        gs_15147: bool,
        ga_11022: i64,
        selbitshadow_264: bool,
        gs_15092: bool,
        gs_15122: bool,
        gs_14986: bool,
        gs_15020: bool,
        selbit: bool,
        gs_15086: bool,
        gs_14992: bool,
        gs_15106: bool,
        ptr: u64,
        modifier: u64,
        K: u128,
        data: bool,
    }
    let fn_state = FunctionState {
        ptr,
        modifier,
        K,
        data,
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
        // D s_0_22: read-var tbi:u8
        let s_0_22: bool = fn_state.tbi;
        // N s_0_23: branch s_0_22 b127 b1
        if s_0_22 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #63s : i64
        let s_1_0: i64 = 63;
        // D s_1_1: write-var ga#11022 <= s_1_0
        fn_state.ga_11022 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var ga#11022:i64
        let s_2_0: i64 = fn_state.ga_11022;
        // D s_2_1: write-var top_bit <= s_2_0
        fn_state.top_bit = s_2_0;
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call PtrHasUpperAndLowerAddRanges(s_2_2)
        let s_2_3: bool = PtrHasUpperAndLowerAddRanges(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b85 b3
        if s_2_3 {
            return block_85(state, tracer, fn_state);
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
        // N s_3_1: branch s_3_0 b84 b4
        if s_3_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_4_0: const #63s : i
        let s_4_0: i128 = 63;
        // D s_4_1: read-var ptr:u64
        let s_4_1: u64 = fn_state.ptr;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: write-var selbit <= s_4_17
        fn_state.selbit = s_4_17;
        // N s_4_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveEnhancedPAC2(s_6_0)
        let s_6_1: bool = HaveEnhancedPAC2(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b83 b7
        if s_6_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#14968 <= s_7_0
        fn_state.gs_14968 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_8_0: read-var gs#14968:u8
        let s_8_0: bool = fn_state.gs_14968;
        // N s_8_1: branch s_8_0 b82 b9
        if s_8_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var selbit:u8
        let s_10_0: bool = fn_state.selbit;
        // D s_10_1: write-var selbitshadow#264 <= s_10_0
        fn_state.selbitshadow_264 = s_10_0;
        // D s_10_2: read-var selbitshadow#264:u8
        let s_10_2: bool = fn_state.selbitshadow_264;
        // D s_10_3: call CalculateBottomPACBit(s_10_2)
        let s_10_3: i128 = CalculateBottomPACBit(state, tracer, s_10_2);
        // D s_10_4: write-var bottom_PAC_bit <= s_10_3
        fn_state.bottom_PAC_bit = s_10_3;
        // D s_10_5: read-var tbi:u8
        let s_10_5: bool = fn_state.tbi;
        // N s_10_6: branch s_10_5 b81 b11
        if s_10_5 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#15005 <= s_11_0
        fn_state.gs_15005 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var gs#15005:u8
        let s_12_0: bool = fn_state.gs_15005;
        // N s_12_1: branch s_12_0 b80 b13
        if s_12_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var selbitshadow#264:u8
        let s_13_0: bool = fn_state.selbitshadow_264;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #64u : u64
        let s_13_2: u64 = 64;
        // D s_13_3: call replicate_bits_borealis_internal(s_13_1, s_13_2)
        let s_13_3: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_13_1,
            s_13_2,
        );
        // D s_13_4: cast reint s_13_3 -> u64
        let s_13_4: u64 = (s_13_3.value() as u64);
        // D s_13_5: write-var extfield <= s_13_4
        fn_state.extfield = s_13_4;
        // D s_13_6: read-var tbi:u8
        let s_13_6: bool = fn_state.tbi;
        // N s_13_7: branch s_13_6 b73 b14
        if s_13_6 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var mtx:u8
        let s_14_0: bool = fn_state.mtx;
        // N s_14_1: branch s_14_0 b66 b15
        if s_14_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var bottom_PAC_bit:i
        let s_15_0: i128 = fn_state.bottom_PAC_bit;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #64s : i
        let s_15_2: i128 = 64;
        // D s_15_3: sub s_15_2 s_15_1
        let s_15_3: i128 = ((s_15_2) - (s_15_1));
        // C s_15_4: const #1s : i
        let s_15_4: i128 = 1;
        // D s_15_5: sub s_15_3 s_15_4
        let s_15_5: i128 = ((s_15_3) - (s_15_4));
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // D s_15_7: cmp-le s_15_6 s_15_5
        let s_15_7: bool = ((s_15_6) <= (s_15_5));
        // N s_15_8: branch s_15_7 b65 b16
        if s_15_7 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#15015 <= s_16_0
        fn_state.gs_15015 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var gs#15015:u8
        let s_17_0: bool = fn_state.gs_15015;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // D s_17_2: read-var bottom_PAC_bit:i
        let s_17_2: i128 = fn_state.bottom_PAC_bit;
        // D s_17_3: call __id(s_17_2)
        let s_17_3: i128 = u__id(state, tracer, s_17_2);
        // D s_17_4: cast reint s_17_3 -> i64
        let s_17_4: i64 = (s_17_3 as i64);
        // C s_17_5: const #1s : i
        let s_17_5: i128 = 1;
        // D s_17_6: cast zx s_17_4 -> i
        let s_17_6: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_7: sub s_17_6 s_17_5
        let s_17_7: i128 = ((s_17_6) - (s_17_5));
        // D s_17_8: cast reint s_17_7 -> i64
        let s_17_8: i64 = (s_17_7 as i64);
        // C s_17_9: const #0s : i
        let s_17_9: i128 = 0;
        // D s_17_10: cast zx s_17_8 -> i
        let s_17_10: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_11: cmp-le s_17_9 s_17_10
        let s_17_11: bool = ((s_17_9) <= (s_17_10));
        // N s_17_12: branch s_17_11 b64 b18
        if s_17_11 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#15020 <= s_18_0
        fn_state.gs_15020 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var gs#15020:u8
        let s_19_0: bool = fn_state.gs_15020;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // C s_19_2: const #64s : i
        let s_19_2: i128 = 64;
        // D s_19_3: read-var bottom_PAC_bit:i
        let s_19_3: i128 = fn_state.bottom_PAC_bit;
        // D s_19_4: sub s_19_2 s_19_3
        let s_19_4: i128 = ((s_19_2) - (s_19_3));
        // D s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // C s_19_6: const #1s : i
        let s_19_6: i128 = 1;
        // D s_19_7: cast zx s_19_5 -> i
        let s_19_7: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_8: sub s_19_7 s_19_6
        let s_19_8: i128 = ((s_19_7) - (s_19_6));
        // D s_19_9: cast reint s_19_8 -> i64
        let s_19_9: i64 = (s_19_8 as i64);
        // C s_19_10: const #1s : i
        let s_19_10: i128 = 1;
        // D s_19_11: read-var bottom_PAC_bit:i
        let s_19_11: i128 = fn_state.bottom_PAC_bit;
        // D s_19_12: sub s_19_11 s_19_10
        let s_19_12: i128 = ((s_19_11) - (s_19_10));
        // D s_19_13: cast reint s_19_12 -> i64
        let s_19_13: i64 = (s_19_12 as i64);
        // C s_19_14: const #64s : i
        let s_19_14: i128 = 64;
        // C s_19_15: const #0s : i
        let s_19_15: i128 = 0;
        // C s_19_16: const #0s : i
        let s_19_16: i128 = 0;
        // D s_19_17: read-var extfield:u64
        let s_19_17: u64 = fn_state.extfield;
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 64u16);
        // D s_19_19: cast zx s_19_9 -> i
        let s_19_19: i128 = (i128::try_from(s_19_9).unwrap());
        // D s_19_20: read-var ptr:u64
        let s_19_20: u64 = fn_state.ptr;
        // D s_19_21: cast zx s_19_20 -> bv
        let s_19_21: Bits = Bits::new(s_19_20 as u128, 64u16);
        // D s_19_22: cast zx s_19_13 -> i
        let s_19_22: i128 = (i128::try_from(s_19_13).unwrap());
        // D s_19_23: call subrange_subrange_concat(s_19_14, s_19_18, s_19_19, s_19_15, s_19_21, s_19_22, s_19_16)
        let s_19_23: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_19_14,
            s_19_18,
            s_19_19,
            s_19_15,
            s_19_21,
            s_19_22,
            s_19_16,
        );
        // D s_19_24: cast reint s_19_23 -> u64
        let s_19_24: u64 = (s_19_23.value() as u64);
        // D s_19_25: write-var ext_ptr <= s_19_24
        fn_state.ext_ptr = s_19_24;
        // N s_19_26: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_20_0: const #64s : i
        let s_20_0: i128 = 64;
        // D s_20_1: read-var K:u128
        let s_20_1: u128 = fn_state.K;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 128u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #63s : i
        let s_20_5: i128 = 63;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_0 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u64
        let s_20_8: u64 = (s_20_7.value() as u64);
        // C s_20_9: const #0s : i
        let s_20_9: i128 = 0;
        // D s_20_10: read-var K:u128
        let s_20_10: u128 = fn_state.K;
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 128u16);
        // C s_20_12: const #1s : i64
        let s_20_12: i64 = 1;
        // C s_20_13: cast zx s_20_12 -> i
        let s_20_13: i128 = (i128::try_from(s_20_12).unwrap());
        // C s_20_14: const #63s : i
        let s_20_14: i128 = 63;
        // C s_20_15: add s_20_14 s_20_13
        let s_20_15: i128 = (s_20_14 + s_20_13);
        // D s_20_16: bit-extract s_20_11 s_20_9 s_20_15
        let s_20_16: Bits = (Bits::new(
            ((s_20_11) >> (s_20_9)).value(),
            u16::try_from(s_20_15).unwrap(),
        ));
        // D s_20_17: cast reint s_20_16 -> u64
        let s_20_17: u64 = (s_20_16.value() as u64);
        // D s_20_18: read-var ext_ptr:u64
        let s_20_18: u64 = fn_state.ext_ptr;
        // D s_20_19: read-var modifier:u64
        let s_20_19: u64 = fn_state.modifier;
        // C s_20_20: const #0u : u8
        let s_20_20: bool = false;
        // D s_20_21: call ComputePAC(s_20_18, s_20_19, s_20_8, s_20_17, s_20_20)
        let s_20_21: u64 = ComputePAC(
            state,
            tracer,
            s_20_18,
            s_20_19,
            s_20_8,
            s_20_17,
            s_20_20,
        );
        // D s_20_22: write-var PAC <= s_20_21
        fn_state.PAC = s_20_21;
        // C s_20_23: const #64s : i
        let s_20_23: i128 = 64;
        // S s_20_24: call Zeros(s_20_23)
        let s_20_24: Bits = Zeros(state, tracer, s_20_23);
        // S s_20_25: cast reint s_20_24 -> u64
        let s_20_25: u64 = (s_20_24.value() as u64);
        // D s_20_26: write-var unusedbits_mask <= s_20_25
        fn_state.unusedbits_mask = s_20_25;
        // D s_20_27: read-var bottom_PAC_bit:i
        let s_20_27: i128 = fn_state.bottom_PAC_bit;
        // D s_20_28: call __id(s_20_27)
        let s_20_28: i128 = u__id(state, tracer, s_20_27);
        // C s_20_29: const #0s : i
        let s_20_29: i128 = 0;
        // D s_20_30: cmp-le s_20_29 s_20_28
        let s_20_30: bool = ((s_20_29) <= (s_20_28));
        // N s_20_31: branch s_20_30 b63 b21
        if s_20_30 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#15082 <= s_21_0
        fn_state.gs_15082 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_22_0: read-var gs#15082:u8
        let s_22_0: bool = fn_state.gs_15082;
        // N s_22_1: assert s_22_0
        let s_22_1: () = assert!(s_22_0);
        // C s_22_2: const #54s : i
        let s_22_2: i128 = 54;
        // D s_22_3: read-var bottom_PAC_bit:i
        let s_22_3: i128 = fn_state.bottom_PAC_bit;
        // D s_22_4: sub s_22_2 s_22_3
        let s_22_4: i128 = ((s_22_2) - (s_22_3));
        // D s_22_5: cast reint s_22_4 -> i64
        let s_22_5: i64 = (s_22_4 as i64);
        // C s_22_6: const #1s : i
        let s_22_6: i128 = 1;
        // D s_22_7: cast zx s_22_5 -> i
        let s_22_7: i128 = (i128::try_from(s_22_5).unwrap());
        // D s_22_8: add s_22_7 s_22_6
        let s_22_8: i128 = (s_22_7 + s_22_6);
        // D s_22_9: cast reint s_22_8 -> i64
        let s_22_9: i64 = (s_22_8 as i64);
        // D s_22_10: cast zx s_22_9 -> i
        let s_22_10: i128 = (i128::try_from(s_22_9).unwrap());
        // D s_22_11: call Ones(s_22_10)
        let s_22_11: Bits = Ones(state, tracer, s_22_10);
        // C s_22_12: const #54s : i
        let s_22_12: i128 = 54;
        // D s_22_13: read-var unusedbits_mask:u64
        let s_22_13: u64 = fn_state.unusedbits_mask;
        // D s_22_14: cast zx s_22_13 -> bv
        let s_22_14: Bits = Bits::new(s_22_13 as u128, 64u16);
        // D s_22_15: read-var bottom_PAC_bit:i
        let s_22_15: i128 = fn_state.bottom_PAC_bit;
        // D s_22_16: sub s_22_12 s_22_15
        let s_22_16: i128 = ((s_22_12) - (s_22_15));
        // C s_22_17: const #1u : u64
        let s_22_17: u64 = 1;
        // C s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 64u16);
        // D s_22_19: lsl s_22_18 s_22_16
        let s_22_19: Bits = s_22_18 << s_22_16;
        // D s_22_20: sub s_22_19 s_22_18
        let s_22_20: Bits = ((s_22_19) - (s_22_18));
        // D s_22_21: and s_22_11 s_22_20
        let s_22_21: Bits = ((s_22_11) & (s_22_20));
        // D s_22_22: lsl s_22_21 s_22_15
        let s_22_22: Bits = s_22_21 << s_22_15;
        // D s_22_23: lsl s_22_20 s_22_15
        let s_22_23: Bits = s_22_20 << s_22_15;
        // D s_22_24: cmpl s_22_23
        let s_22_24: Bits = !s_22_23;
        // D s_22_25: and s_22_14 s_22_24
        let s_22_25: Bits = ((s_22_14) & (s_22_24));
        // D s_22_26: or s_22_25 s_22_22
        let s_22_26: Bits = ((s_22_25) | (s_22_22));
        // D s_22_27: cast reint s_22_26 -> u64
        let s_22_27: u64 = (s_22_26.value() as u64);
        // D s_22_28: write-var unusedbits_mask <= s_22_27
        fn_state.unusedbits_mask = s_22_27;
        // D s_22_29: read-var tbi:u8
        let s_22_29: bool = fn_state.tbi;
        // N s_22_30: branch s_22_29 b62 b23
        if s_22_29 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var mtx:u8
        let s_23_0: bool = fn_state.mtx;
        // N s_23_1: branch s_23_0 b61 b24
        if s_23_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var ptr:u64
        let s_25_0: u64 = fn_state.ptr;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 64u16);
        // D s_25_2: read-var unusedbits_mask:u64
        let s_25_2: u64 = fn_state.unusedbits_mask;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 64u16);
        // D s_25_4: and s_25_1 s_25_3
        let s_25_4: Bits = ((s_25_1) & (s_25_3));
        // D s_25_5: cast reint s_25_4 -> u64
        let s_25_5: u64 = (s_25_4.value() as u64);
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_7: call IsZero(s_25_6)
        let s_25_7: bool = IsZero(state, tracer, s_25_6);
        // D s_25_8: not s_25_7
        let s_25_8: bool = !s_25_7;
        // N s_25_9: branch s_25_8 b60 b26
        if s_25_8 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#15086 <= s_26_0
        fn_state.gs_15086 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var gs#15086:u8
        let s_27_0: bool = fn_state.gs_15086;
        // N s_27_1: branch s_27_0 b55 b28
        if s_27_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HaveEnhancedPAC2(s_29_0)
        let s_29_1: bool = HaveEnhancedPAC2(state, tracer, s_29_0);
        // S s_29_2: not s_29_1
        let s_29_2: bool = !s_29_1;
        // N s_29_3: branch s_29_2 b46 b30
        if s_29_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_30_0: read-var tbi:u8
        let s_30_0: bool = fn_state.tbi;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_31_0: read-var mtx:u8
        let s_31_0: bool = fn_state.mtx;
        // N s_31_1: branch s_31_0 b38 b32
        if s_31_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_32_0: read-var bottom_PAC_bit:i
        let s_32_0: i128 = fn_state.bottom_PAC_bit;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // D s_32_2: cast reint s_32_1 -> i64
        let s_32_2: i64 = (s_32_1 as i64);
        // C s_32_3: const #1s : i
        let s_32_3: i128 = 1;
        // D s_32_4: cast zx s_32_2 -> i
        let s_32_4: i128 = (i128::try_from(s_32_2).unwrap());
        // D s_32_5: sub s_32_4 s_32_3
        let s_32_5: i128 = ((s_32_4) - (s_32_3));
        // D s_32_6: cast reint s_32_5 -> i64
        let s_32_6: i64 = (s_32_5 as i64);
        // C s_32_7: const #0s : i
        let s_32_7: i128 = 0;
        // D s_32_8: cast zx s_32_6 -> i
        let s_32_8: i128 = (i128::try_from(s_32_6).unwrap());
        // D s_32_9: cmp-le s_32_7 s_32_8
        let s_32_9: bool = ((s_32_7) <= (s_32_8));
        // N s_32_10: branch s_32_9 b37 b33
        if s_32_9 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#15092 <= s_33_0
        fn_state.gs_15092 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_34_0: read-var gs#15092:u8
        let s_34_0: bool = fn_state.gs_15092;
        // N s_34_1: assert s_34_0
        let s_34_1: () = assert!(s_34_0);
        // C s_34_2: const #56s : i
        let s_34_2: i128 = 56;
        // D s_34_3: read-var ptr:u64
        let s_34_3: u64 = fn_state.ptr;
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 64u16);
        // C s_34_5: const #1s : i64
        let s_34_5: i64 = 1;
        // C s_34_6: cast zx s_34_5 -> i
        let s_34_6: i128 = (i128::try_from(s_34_5).unwrap());
        // C s_34_7: const #7s : i
        let s_34_7: i128 = 7;
        // C s_34_8: add s_34_7 s_34_6
        let s_34_8: i128 = (s_34_7 + s_34_6);
        // D s_34_9: bit-extract s_34_4 s_34_2 s_34_8
        let s_34_9: Bits = (Bits::new(
            ((s_34_4) >> (s_34_2)).value(),
            u16::try_from(s_34_8).unwrap(),
        ));
        // D s_34_10: cast reint s_34_9 -> u8
        let s_34_10: u8 = (s_34_9.value() as u8);
        // C s_34_11: const #56s : i
        let s_34_11: i128 = 56;
        // D s_34_12: read-var PAC:u64
        let s_34_12: u64 = fn_state.PAC;
        // D s_34_13: cast zx s_34_12 -> bv
        let s_34_13: Bits = Bits::new(s_34_12 as u128, 64u16);
        // C s_34_14: const #1s : i64
        let s_34_14: i64 = 1;
        // C s_34_15: cast zx s_34_14 -> i
        let s_34_15: i128 = (i128::try_from(s_34_14).unwrap());
        // C s_34_16: const #7s : i
        let s_34_16: i128 = 7;
        // C s_34_17: add s_34_16 s_34_15
        let s_34_17: i128 = (s_34_16 + s_34_15);
        // D s_34_18: bit-extract s_34_13 s_34_11 s_34_17
        let s_34_18: Bits = (Bits::new(
            ((s_34_13) >> (s_34_11)).value(),
            u16::try_from(s_34_17).unwrap(),
        ));
        // D s_34_19: cast reint s_34_18 -> u8
        let s_34_19: u8 = (s_34_18.value() as u8);
        // D s_34_20: cast zx s_34_10 -> bv
        let s_34_20: Bits = Bits::new(s_34_10 as u128, 8u16);
        // D s_34_21: cast zx s_34_19 -> bv
        let s_34_21: Bits = Bits::new(s_34_19 as u128, 8u16);
        // D s_34_22: xor s_34_20 s_34_21
        let s_34_22: Bits = ((s_34_20) ^ (s_34_21));
        // D s_34_23: cast reint s_34_22 -> u8
        let s_34_23: u8 = (s_34_22.value() as u8);
        // D s_34_24: cast zx s_34_23 -> bv
        let s_34_24: Bits = Bits::new(s_34_23 as u128, 8u16);
        // D s_34_25: read-var selbitshadow#264:u8
        let s_34_25: bool = fn_state.selbitshadow_264;
        // D s_34_26: cast zx s_34_25 -> bv
        let s_34_26: Bits = Bits::new(s_34_25 as u128, 1u16);
        // D s_34_27: cast reint s_34_24 -> u128
        let s_34_27: u128 = (s_34_24.value() as u128);
        // D s_34_28: size-of s_34_24
        let s_34_28: u16 = s_34_24.length();
        // D s_34_29: cast reint s_34_26 -> u128
        let s_34_29: u128 = (s_34_26.value() as u128);
        // D s_34_30: size-of s_34_26
        let s_34_30: u16 = s_34_26.length();
        // D s_34_31: lsl s_34_27 s_34_30
        let s_34_31: u128 = s_34_27 << s_34_30;
        // D s_34_32: or s_34_31 s_34_29
        let s_34_32: u128 = ((s_34_31) | (s_34_29));
        // D s_34_33: add s_34_28 s_34_30
        let s_34_33: u16 = (s_34_28 + s_34_30);
        // D s_34_34: create-bits s_34_32 s_34_33
        let s_34_34: Bits = Bits::new(s_34_32, s_34_33);
        // D s_34_35: cast reint s_34_34 -> u9
        let s_34_35: u16 = (s_34_34.value() as u16);
        // D s_34_36: read-var ptr:u64
        let s_34_36: u64 = fn_state.ptr;
        // D s_34_37: cast zx s_34_36 -> bv
        let s_34_37: Bits = Bits::new(s_34_36 as u128, 64u16);
        // D s_34_38: read-var PAC:u64
        let s_34_38: u64 = fn_state.PAC;
        // D s_34_39: cast zx s_34_38 -> bv
        let s_34_39: Bits = Bits::new(s_34_38 as u128, 64u16);
        // D s_34_40: xor s_34_37 s_34_39
        let s_34_40: Bits = ((s_34_37) ^ (s_34_39));
        // D s_34_41: cast reint s_34_40 -> u64
        let s_34_41: u64 = (s_34_40.value() as u64);
        // C s_34_42: const #1s : i
        let s_34_42: i128 = 1;
        // D s_34_43: read-var bottom_PAC_bit:i
        let s_34_43: i128 = fn_state.bottom_PAC_bit;
        // D s_34_44: sub s_34_43 s_34_42
        let s_34_44: i128 = ((s_34_43) - (s_34_42));
        // D s_34_45: cast reint s_34_44 -> i64
        let s_34_45: i64 = (s_34_44 as i64);
        // C s_34_46: const #55s : i
        let s_34_46: i128 = 55;
        // C s_34_47: const #54s : i
        let s_34_47: i128 = 54;
        // C s_34_48: const #0s : i
        let s_34_48: i128 = 0;
        // D s_34_49: cast zx s_34_41 -> bv
        let s_34_49: Bits = Bits::new(s_34_41 as u128, 64u16);
        // D s_34_50: read-var ptr:u64
        let s_34_50: u64 = fn_state.ptr;
        // D s_34_51: cast zx s_34_50 -> bv
        let s_34_51: Bits = Bits::new(s_34_50 as u128, 64u16);
        // D s_34_52: cast zx s_34_45 -> i
        let s_34_52: i128 = (i128::try_from(s_34_45).unwrap());
        // D s_34_53: read-var bottom_PAC_bit:i
        let s_34_53: i128 = fn_state.bottom_PAC_bit;
        // D s_34_54: call subrange_subrange_concat(s_34_46, s_34_49, s_34_47, s_34_53, s_34_51, s_34_52, s_34_48)
        let s_34_54: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_34_46,
            s_34_49,
            s_34_47,
            s_34_53,
            s_34_51,
            s_34_52,
            s_34_48,
        );
        // D s_34_55: cast reint s_34_54 -> u55
        let s_34_55: u64 = (s_34_54.value() as u64);
        // D s_34_56: cast zx s_34_35 -> bv
        let s_34_56: Bits = Bits::new(s_34_35 as u128, 9u16);
        // D s_34_57: cast zx s_34_55 -> bv
        let s_34_57: Bits = Bits::new(s_34_55 as u128, 55u16);
        // D s_34_58: cast reint s_34_56 -> u128
        let s_34_58: u128 = (s_34_56.value() as u128);
        // D s_34_59: size-of s_34_56
        let s_34_59: u16 = s_34_56.length();
        // D s_34_60: cast reint s_34_57 -> u128
        let s_34_60: u128 = (s_34_57.value() as u128);
        // D s_34_61: size-of s_34_57
        let s_34_61: u16 = s_34_57.length();
        // D s_34_62: lsl s_34_58 s_34_61
        let s_34_62: u128 = s_34_58 << s_34_61;
        // D s_34_63: or s_34_62 s_34_60
        let s_34_63: u128 = ((s_34_62) | (s_34_60));
        // D s_34_64: add s_34_59 s_34_61
        let s_34_64: u16 = (s_34_59 + s_34_61);
        // D s_34_65: create-bits s_34_63 s_34_64
        let s_34_65: Bits = Bits::new(s_34_63, s_34_64);
        // D s_34_66: cast reint s_34_65 -> u64
        let s_34_66: u64 = (s_34_65.value() as u64);
        // D s_34_67: write-var result <= s_34_66
        fn_state.result = s_34_66;
        // N s_34_68: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_35_0: read-var result:u64
        let s_35_0: u64 = fn_state.result;
        // D s_35_1: write-var return_value <= s_35_0
        fn_state.return_value = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_36_0: read-var return_value:u64
        let s_36_0: u64 = fn_state.return_value;
        // N s_36_1: return s_36_0
        return s_36_0;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_37_0: read-var bottom_PAC_bit:i
        let s_37_0: i128 = fn_state.bottom_PAC_bit;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // D s_37_2: cast reint s_37_1 -> i64
        let s_37_2: i64 = (s_37_1 as i64);
        // C s_37_3: const #1s : i
        let s_37_3: i128 = 1;
        // D s_37_4: cast zx s_37_2 -> i
        let s_37_4: i128 = (i128::try_from(s_37_2).unwrap());
        // D s_37_5: sub s_37_4 s_37_3
        let s_37_5: i128 = ((s_37_4) - (s_37_3));
        // D s_37_6: cast reint s_37_5 -> i64
        let s_37_6: i64 = (s_37_5 as i64);
        // C s_37_7: const #64s : i
        let s_37_7: i128 = 64;
        // D s_37_8: cast zx s_37_6 -> i
        let s_37_8: i128 = (i128::try_from(s_37_6).unwrap());
        // D s_37_9: cmp-lt s_37_8 s_37_7
        let s_37_9: bool = ((s_37_8) < (s_37_7));
        // D s_37_10: write-var gs#15092 <= s_37_9
        fn_state.gs_15092 = s_37_9;
        // N s_37_11: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_38_0: read-var bottom_PAC_bit:i
        let s_38_0: i128 = fn_state.bottom_PAC_bit;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // D s_38_2: cast reint s_38_1 -> i64
        let s_38_2: i64 = (s_38_1 as i64);
        // C s_38_3: const #1s : i
        let s_38_3: i128 = 1;
        // D s_38_4: cast zx s_38_2 -> i
        let s_38_4: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_5: sub s_38_4 s_38_3
        let s_38_5: i128 = ((s_38_4) - (s_38_3));
        // D s_38_6: cast reint s_38_5 -> i64
        let s_38_6: i64 = (s_38_5 as i64);
        // C s_38_7: const #0s : i
        let s_38_7: i128 = 0;
        // D s_38_8: cast zx s_38_6 -> i
        let s_38_8: i128 = (i128::try_from(s_38_6).unwrap());
        // D s_38_9: cmp-le s_38_7 s_38_8
        let s_38_9: bool = ((s_38_7) <= (s_38_8));
        // N s_38_10: branch s_38_9 b41 b39
        if s_38_9 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#15106 <= s_39_0
        fn_state.gs_15106 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_40_0: read-var gs#15106:u8
        let s_40_0: bool = fn_state.gs_15106;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // C s_40_2: const #60s : i
        let s_40_2: i128 = 60;
        // D s_40_3: read-var ptr:u64
        let s_40_3: u64 = fn_state.ptr;
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 64u16);
        // C s_40_5: const #1s : i64
        let s_40_5: i64 = 1;
        // C s_40_6: cast zx s_40_5 -> i
        let s_40_6: i128 = (i128::try_from(s_40_5).unwrap());
        // C s_40_7: const #3s : i
        let s_40_7: i128 = 3;
        // C s_40_8: add s_40_7 s_40_6
        let s_40_8: i128 = (s_40_7 + s_40_6);
        // D s_40_9: bit-extract s_40_4 s_40_2 s_40_8
        let s_40_9: Bits = (Bits::new(
            ((s_40_4) >> (s_40_2)).value(),
            u16::try_from(s_40_8).unwrap(),
        ));
        // D s_40_10: cast reint s_40_9 -> u8
        let s_40_10: u8 = (s_40_9.value() as u8);
        // C s_40_11: const #60s : i
        let s_40_11: i128 = 60;
        // D s_40_12: read-var PAC:u64
        let s_40_12: u64 = fn_state.PAC;
        // D s_40_13: cast zx s_40_12 -> bv
        let s_40_13: Bits = Bits::new(s_40_12 as u128, 64u16);
        // C s_40_14: const #1s : i64
        let s_40_14: i64 = 1;
        // C s_40_15: cast zx s_40_14 -> i
        let s_40_15: i128 = (i128::try_from(s_40_14).unwrap());
        // C s_40_16: const #3s : i
        let s_40_16: i128 = 3;
        // C s_40_17: add s_40_16 s_40_15
        let s_40_17: i128 = (s_40_16 + s_40_15);
        // D s_40_18: bit-extract s_40_13 s_40_11 s_40_17
        let s_40_18: Bits = (Bits::new(
            ((s_40_13) >> (s_40_11)).value(),
            u16::try_from(s_40_17).unwrap(),
        ));
        // D s_40_19: cast reint s_40_18 -> u8
        let s_40_19: u8 = (s_40_18.value() as u8);
        // D s_40_20: cast zx s_40_10 -> bv
        let s_40_20: Bits = Bits::new(s_40_10 as u128, 4u16);
        // D s_40_21: cast zx s_40_19 -> bv
        let s_40_21: Bits = Bits::new(s_40_19 as u128, 4u16);
        // D s_40_22: xor s_40_20 s_40_21
        let s_40_22: Bits = ((s_40_20) ^ (s_40_21));
        // D s_40_23: cast reint s_40_22 -> u8
        let s_40_23: u8 = (s_40_22.value() as u8);
        // C s_40_24: const #56s : i
        let s_40_24: i128 = 56;
        // D s_40_25: read-var ptr:u64
        let s_40_25: u64 = fn_state.ptr;
        // D s_40_26: cast zx s_40_25 -> bv
        let s_40_26: Bits = Bits::new(s_40_25 as u128, 64u16);
        // C s_40_27: const #1s : i64
        let s_40_27: i64 = 1;
        // C s_40_28: cast zx s_40_27 -> i
        let s_40_28: i128 = (i128::try_from(s_40_27).unwrap());
        // C s_40_29: const #3s : i
        let s_40_29: i128 = 3;
        // C s_40_30: add s_40_29 s_40_28
        let s_40_30: i128 = (s_40_29 + s_40_28);
        // D s_40_31: bit-extract s_40_26 s_40_24 s_40_30
        let s_40_31: Bits = (Bits::new(
            ((s_40_26) >> (s_40_24)).value(),
            u16::try_from(s_40_30).unwrap(),
        ));
        // D s_40_32: cast reint s_40_31 -> u8
        let s_40_32: u8 = (s_40_31.value() as u8);
        // D s_40_33: cast zx s_40_23 -> bv
        let s_40_33: Bits = Bits::new(s_40_23 as u128, 4u16);
        // D s_40_34: cast zx s_40_32 -> bv
        let s_40_34: Bits = Bits::new(s_40_32 as u128, 4u16);
        // D s_40_35: cast reint s_40_33 -> u128
        let s_40_35: u128 = (s_40_33.value() as u128);
        // D s_40_36: size-of s_40_33
        let s_40_36: u16 = s_40_33.length();
        // D s_40_37: cast reint s_40_34 -> u128
        let s_40_37: u128 = (s_40_34.value() as u128);
        // D s_40_38: size-of s_40_34
        let s_40_38: u16 = s_40_34.length();
        // D s_40_39: lsl s_40_35 s_40_38
        let s_40_39: u128 = s_40_35 << s_40_38;
        // D s_40_40: or s_40_39 s_40_37
        let s_40_40: u128 = ((s_40_39) | (s_40_37));
        // D s_40_41: add s_40_36 s_40_38
        let s_40_41: u16 = (s_40_36 + s_40_38);
        // D s_40_42: create-bits s_40_40 s_40_41
        let s_40_42: Bits = Bits::new(s_40_40, s_40_41);
        // D s_40_43: cast reint s_40_42 -> u8
        let s_40_43: u8 = (s_40_42.value() as u8);
        // D s_40_44: cast zx s_40_43 -> bv
        let s_40_44: Bits = Bits::new(s_40_43 as u128, 8u16);
        // D s_40_45: read-var selbitshadow#264:u8
        let s_40_45: bool = fn_state.selbitshadow_264;
        // D s_40_46: cast zx s_40_45 -> bv
        let s_40_46: Bits = Bits::new(s_40_45 as u128, 1u16);
        // D s_40_47: cast reint s_40_44 -> u128
        let s_40_47: u128 = (s_40_44.value() as u128);
        // D s_40_48: size-of s_40_44
        let s_40_48: u16 = s_40_44.length();
        // D s_40_49: cast reint s_40_46 -> u128
        let s_40_49: u128 = (s_40_46.value() as u128);
        // D s_40_50: size-of s_40_46
        let s_40_50: u16 = s_40_46.length();
        // D s_40_51: lsl s_40_47 s_40_50
        let s_40_51: u128 = s_40_47 << s_40_50;
        // D s_40_52: or s_40_51 s_40_49
        let s_40_52: u128 = ((s_40_51) | (s_40_49));
        // D s_40_53: add s_40_48 s_40_50
        let s_40_53: u16 = (s_40_48 + s_40_50);
        // D s_40_54: create-bits s_40_52 s_40_53
        let s_40_54: Bits = Bits::new(s_40_52, s_40_53);
        // D s_40_55: cast reint s_40_54 -> u9
        let s_40_55: u16 = (s_40_54.value() as u16);
        // D s_40_56: read-var ptr:u64
        let s_40_56: u64 = fn_state.ptr;
        // D s_40_57: cast zx s_40_56 -> bv
        let s_40_57: Bits = Bits::new(s_40_56 as u128, 64u16);
        // D s_40_58: read-var PAC:u64
        let s_40_58: u64 = fn_state.PAC;
        // D s_40_59: cast zx s_40_58 -> bv
        let s_40_59: Bits = Bits::new(s_40_58 as u128, 64u16);
        // D s_40_60: xor s_40_57 s_40_59
        let s_40_60: Bits = ((s_40_57) ^ (s_40_59));
        // D s_40_61: cast reint s_40_60 -> u64
        let s_40_61: u64 = (s_40_60.value() as u64);
        // C s_40_62: const #1s : i
        let s_40_62: i128 = 1;
        // D s_40_63: read-var bottom_PAC_bit:i
        let s_40_63: i128 = fn_state.bottom_PAC_bit;
        // D s_40_64: sub s_40_63 s_40_62
        let s_40_64: i128 = ((s_40_63) - (s_40_62));
        // D s_40_65: cast reint s_40_64 -> i64
        let s_40_65: i64 = (s_40_64 as i64);
        // C s_40_66: const #55s : i
        let s_40_66: i128 = 55;
        // C s_40_67: const #54s : i
        let s_40_67: i128 = 54;
        // C s_40_68: const #0s : i
        let s_40_68: i128 = 0;
        // D s_40_69: cast zx s_40_61 -> bv
        let s_40_69: Bits = Bits::new(s_40_61 as u128, 64u16);
        // D s_40_70: read-var ptr:u64
        let s_40_70: u64 = fn_state.ptr;
        // D s_40_71: cast zx s_40_70 -> bv
        let s_40_71: Bits = Bits::new(s_40_70 as u128, 64u16);
        // D s_40_72: cast zx s_40_65 -> i
        let s_40_72: i128 = (i128::try_from(s_40_65).unwrap());
        // D s_40_73: read-var bottom_PAC_bit:i
        let s_40_73: i128 = fn_state.bottom_PAC_bit;
        // D s_40_74: call subrange_subrange_concat(s_40_66, s_40_69, s_40_67, s_40_73, s_40_71, s_40_72, s_40_68)
        let s_40_74: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_40_66,
            s_40_69,
            s_40_67,
            s_40_73,
            s_40_71,
            s_40_72,
            s_40_68,
        );
        // D s_40_75: cast reint s_40_74 -> u55
        let s_40_75: u64 = (s_40_74.value() as u64);
        // D s_40_76: cast zx s_40_55 -> bv
        let s_40_76: Bits = Bits::new(s_40_55 as u128, 9u16);
        // D s_40_77: cast zx s_40_75 -> bv
        let s_40_77: Bits = Bits::new(s_40_75 as u128, 55u16);
        // D s_40_78: cast reint s_40_76 -> u128
        let s_40_78: u128 = (s_40_76.value() as u128);
        // D s_40_79: size-of s_40_76
        let s_40_79: u16 = s_40_76.length();
        // D s_40_80: cast reint s_40_77 -> u128
        let s_40_80: u128 = (s_40_77.value() as u128);
        // D s_40_81: size-of s_40_77
        let s_40_81: u16 = s_40_77.length();
        // D s_40_82: lsl s_40_78 s_40_81
        let s_40_82: u128 = s_40_78 << s_40_81;
        // D s_40_83: or s_40_82 s_40_80
        let s_40_83: u128 = ((s_40_82) | (s_40_80));
        // D s_40_84: add s_40_79 s_40_81
        let s_40_84: u16 = (s_40_79 + s_40_81);
        // D s_40_85: create-bits s_40_83 s_40_84
        let s_40_85: Bits = Bits::new(s_40_83, s_40_84);
        // D s_40_86: cast reint s_40_85 -> u64
        let s_40_86: u64 = (s_40_85.value() as u64);
        // D s_40_87: write-var result <= s_40_86
        fn_state.result = s_40_86;
        // N s_40_88: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_41_0: read-var bottom_PAC_bit:i
        let s_41_0: i128 = fn_state.bottom_PAC_bit;
        // D s_41_1: call __id(s_41_0)
        let s_41_1: i128 = u__id(state, tracer, s_41_0);
        // D s_41_2: cast reint s_41_1 -> i64
        let s_41_2: i64 = (s_41_1 as i64);
        // C s_41_3: const #1s : i
        let s_41_3: i128 = 1;
        // D s_41_4: cast zx s_41_2 -> i
        let s_41_4: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_5: sub s_41_4 s_41_3
        let s_41_5: i128 = ((s_41_4) - (s_41_3));
        // D s_41_6: cast reint s_41_5 -> i64
        let s_41_6: i64 = (s_41_5 as i64);
        // C s_41_7: const #64s : i
        let s_41_7: i128 = 64;
        // D s_41_8: cast zx s_41_6 -> i
        let s_41_8: i128 = (i128::try_from(s_41_6).unwrap());
        // D s_41_9: cmp-lt s_41_8 s_41_7
        let s_41_9: bool = ((s_41_8) < (s_41_7));
        // D s_41_10: write-var gs#15106 <= s_41_9
        fn_state.gs_15106 = s_41_9;
        // N s_41_11: jump b40
        return block_40(state, tracer, fn_state);
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
        // D s_42_2: cast reint s_42_1 -> i64
        let s_42_2: i64 = (s_42_1 as i64);
        // C s_42_3: const #1s : i
        let s_42_3: i128 = 1;
        // D s_42_4: cast zx s_42_2 -> i
        let s_42_4: i128 = (i128::try_from(s_42_2).unwrap());
        // D s_42_5: sub s_42_4 s_42_3
        let s_42_5: i128 = ((s_42_4) - (s_42_3));
        // D s_42_6: cast reint s_42_5 -> i64
        let s_42_6: i64 = (s_42_5 as i64);
        // C s_42_7: const #0s : i
        let s_42_7: i128 = 0;
        // D s_42_8: cast zx s_42_6 -> i
        let s_42_8: i128 = (i128::try_from(s_42_6).unwrap());
        // D s_42_9: cmp-le s_42_7 s_42_8
        let s_42_9: bool = ((s_42_7) <= (s_42_8));
        // N s_42_10: branch s_42_9 b45 b43
        if s_42_9 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#15122 <= s_43_0
        fn_state.gs_15122 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_44_0: read-var gs#15122:u8
        let s_44_0: bool = fn_state.gs_15122;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // C s_44_2: const #56s : i
        let s_44_2: i128 = 56;
        // D s_44_3: read-var ptr:u64
        let s_44_3: u64 = fn_state.ptr;
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 64u16);
        // C s_44_5: const #1s : i64
        let s_44_5: i64 = 1;
        // C s_44_6: cast zx s_44_5 -> i
        let s_44_6: i128 = (i128::try_from(s_44_5).unwrap());
        // C s_44_7: const #7s : i
        let s_44_7: i128 = 7;
        // C s_44_8: add s_44_7 s_44_6
        let s_44_8: i128 = (s_44_7 + s_44_6);
        // D s_44_9: bit-extract s_44_4 s_44_2 s_44_8
        let s_44_9: Bits = (Bits::new(
            ((s_44_4) >> (s_44_2)).value(),
            u16::try_from(s_44_8).unwrap(),
        ));
        // D s_44_10: cast reint s_44_9 -> u8
        let s_44_10: u8 = (s_44_9.value() as u8);
        // D s_44_11: cast zx s_44_10 -> bv
        let s_44_11: Bits = Bits::new(s_44_10 as u128, 8u16);
        // D s_44_12: read-var selbitshadow#264:u8
        let s_44_12: bool = fn_state.selbitshadow_264;
        // D s_44_13: cast zx s_44_12 -> bv
        let s_44_13: Bits = Bits::new(s_44_12 as u128, 1u16);
        // D s_44_14: cast reint s_44_11 -> u128
        let s_44_14: u128 = (s_44_11.value() as u128);
        // D s_44_15: size-of s_44_11
        let s_44_15: u16 = s_44_11.length();
        // D s_44_16: cast reint s_44_13 -> u128
        let s_44_16: u128 = (s_44_13.value() as u128);
        // D s_44_17: size-of s_44_13
        let s_44_17: u16 = s_44_13.length();
        // D s_44_18: lsl s_44_14 s_44_17
        let s_44_18: u128 = s_44_14 << s_44_17;
        // D s_44_19: or s_44_18 s_44_16
        let s_44_19: u128 = ((s_44_18) | (s_44_16));
        // D s_44_20: add s_44_15 s_44_17
        let s_44_20: u16 = (s_44_15 + s_44_17);
        // D s_44_21: create-bits s_44_19 s_44_20
        let s_44_21: Bits = Bits::new(s_44_19, s_44_20);
        // D s_44_22: cast reint s_44_21 -> u9
        let s_44_22: u16 = (s_44_21.value() as u16);
        // D s_44_23: read-var ptr:u64
        let s_44_23: u64 = fn_state.ptr;
        // D s_44_24: cast zx s_44_23 -> bv
        let s_44_24: Bits = Bits::new(s_44_23 as u128, 64u16);
        // D s_44_25: read-var PAC:u64
        let s_44_25: u64 = fn_state.PAC;
        // D s_44_26: cast zx s_44_25 -> bv
        let s_44_26: Bits = Bits::new(s_44_25 as u128, 64u16);
        // D s_44_27: xor s_44_24 s_44_26
        let s_44_27: Bits = ((s_44_24) ^ (s_44_26));
        // D s_44_28: cast reint s_44_27 -> u64
        let s_44_28: u64 = (s_44_27.value() as u64);
        // C s_44_29: const #1s : i
        let s_44_29: i128 = 1;
        // D s_44_30: read-var bottom_PAC_bit:i
        let s_44_30: i128 = fn_state.bottom_PAC_bit;
        // D s_44_31: sub s_44_30 s_44_29
        let s_44_31: i128 = ((s_44_30) - (s_44_29));
        // D s_44_32: cast reint s_44_31 -> i64
        let s_44_32: i64 = (s_44_31 as i64);
        // C s_44_33: const #55s : i
        let s_44_33: i128 = 55;
        // C s_44_34: const #54s : i
        let s_44_34: i128 = 54;
        // C s_44_35: const #0s : i
        let s_44_35: i128 = 0;
        // D s_44_36: cast zx s_44_28 -> bv
        let s_44_36: Bits = Bits::new(s_44_28 as u128, 64u16);
        // D s_44_37: read-var ptr:u64
        let s_44_37: u64 = fn_state.ptr;
        // D s_44_38: cast zx s_44_37 -> bv
        let s_44_38: Bits = Bits::new(s_44_37 as u128, 64u16);
        // D s_44_39: cast zx s_44_32 -> i
        let s_44_39: i128 = (i128::try_from(s_44_32).unwrap());
        // D s_44_40: read-var bottom_PAC_bit:i
        let s_44_40: i128 = fn_state.bottom_PAC_bit;
        // D s_44_41: call subrange_subrange_concat(s_44_33, s_44_36, s_44_34, s_44_40, s_44_38, s_44_39, s_44_35)
        let s_44_41: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_44_33,
            s_44_36,
            s_44_34,
            s_44_40,
            s_44_38,
            s_44_39,
            s_44_35,
        );
        // D s_44_42: cast reint s_44_41 -> u55
        let s_44_42: u64 = (s_44_41.value() as u64);
        // D s_44_43: cast zx s_44_22 -> bv
        let s_44_43: Bits = Bits::new(s_44_22 as u128, 9u16);
        // D s_44_44: cast zx s_44_42 -> bv
        let s_44_44: Bits = Bits::new(s_44_42 as u128, 55u16);
        // D s_44_45: cast reint s_44_43 -> u128
        let s_44_45: u128 = (s_44_43.value() as u128);
        // D s_44_46: size-of s_44_43
        let s_44_46: u16 = s_44_43.length();
        // D s_44_47: cast reint s_44_44 -> u128
        let s_44_47: u128 = (s_44_44.value() as u128);
        // D s_44_48: size-of s_44_44
        let s_44_48: u16 = s_44_44.length();
        // D s_44_49: lsl s_44_45 s_44_48
        let s_44_49: u128 = s_44_45 << s_44_48;
        // D s_44_50: or s_44_49 s_44_47
        let s_44_50: u128 = ((s_44_49) | (s_44_47));
        // D s_44_51: add s_44_46 s_44_48
        let s_44_51: u16 = (s_44_46 + s_44_48);
        // D s_44_52: create-bits s_44_50 s_44_51
        let s_44_52: Bits = Bits::new(s_44_50, s_44_51);
        // D s_44_53: cast reint s_44_52 -> u64
        let s_44_53: u64 = (s_44_52.value() as u64);
        // D s_44_54: write-var result <= s_44_53
        fn_state.result = s_44_53;
        // N s_44_55: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_45_0: read-var bottom_PAC_bit:i
        let s_45_0: i128 = fn_state.bottom_PAC_bit;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // D s_45_2: cast reint s_45_1 -> i64
        let s_45_2: i64 = (s_45_1 as i64);
        // C s_45_3: const #1s : i
        let s_45_3: i128 = 1;
        // D s_45_4: cast zx s_45_2 -> i
        let s_45_4: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_5: sub s_45_4 s_45_3
        let s_45_5: i128 = ((s_45_4) - (s_45_3));
        // D s_45_6: cast reint s_45_5 -> i64
        let s_45_6: i64 = (s_45_5 as i64);
        // C s_45_7: const #64s : i
        let s_45_7: i128 = 64;
        // D s_45_8: cast zx s_45_6 -> i
        let s_45_8: i128 = (i128::try_from(s_45_6).unwrap());
        // D s_45_9: cmp-lt s_45_8 s_45_7
        let s_45_9: bool = ((s_45_8) < (s_45_7));
        // D s_45_10: write-var gs#15122 <= s_45_9
        fn_state.gs_15122 = s_45_9;
        // N s_45_11: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_46_0: read-var tbi:u8
        let s_46_0: bool = fn_state.tbi;
        // N s_46_1: branch s_46_0 b51 b47
        if s_46_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_47_0: read-var bottom_PAC_bit:i
        let s_47_0: i128 = fn_state.bottom_PAC_bit;
        // D s_47_1: call __id(s_47_0)
        let s_47_1: i128 = u__id(state, tracer, s_47_0);
        // D s_47_2: cast reint s_47_1 -> i64
        let s_47_2: i64 = (s_47_1 as i64);
        // C s_47_3: const #1s : i
        let s_47_3: i128 = 1;
        // D s_47_4: cast zx s_47_2 -> i
        let s_47_4: i128 = (i128::try_from(s_47_2).unwrap());
        // D s_47_5: sub s_47_4 s_47_3
        let s_47_5: i128 = ((s_47_4) - (s_47_3));
        // D s_47_6: cast reint s_47_5 -> i64
        let s_47_6: i64 = (s_47_5 as i64);
        // C s_47_7: const #0s : i
        let s_47_7: i128 = 0;
        // D s_47_8: cast zx s_47_6 -> i
        let s_47_8: i128 = (i128::try_from(s_47_6).unwrap());
        // D s_47_9: cmp-le s_47_7 s_47_8
        let s_47_9: bool = ((s_47_7) <= (s_47_8));
        // N s_47_10: branch s_47_9 b50 b48
        if s_47_9 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#15134 <= s_48_0
        fn_state.gs_15134 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_49_0: read-var gs#15134:u8
        let s_49_0: bool = fn_state.gs_15134;
        // N s_49_1: assert s_49_0
        let s_49_1: () = assert!(s_49_0);
        // C s_49_2: const #56s : i
        let s_49_2: i128 = 56;
        // D s_49_3: read-var PAC:u64
        let s_49_3: u64 = fn_state.PAC;
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 64u16);
        // C s_49_5: const #1s : i64
        let s_49_5: i64 = 1;
        // C s_49_6: cast zx s_49_5 -> i
        let s_49_6: i128 = (i128::try_from(s_49_5).unwrap());
        // C s_49_7: const #7s : i
        let s_49_7: i128 = 7;
        // C s_49_8: add s_49_7 s_49_6
        let s_49_8: i128 = (s_49_7 + s_49_6);
        // D s_49_9: bit-extract s_49_4 s_49_2 s_49_8
        let s_49_9: Bits = (Bits::new(
            ((s_49_4) >> (s_49_2)).value(),
            u16::try_from(s_49_8).unwrap(),
        ));
        // D s_49_10: cast reint s_49_9 -> u8
        let s_49_10: u8 = (s_49_9.value() as u8);
        // D s_49_11: cast zx s_49_10 -> bv
        let s_49_11: Bits = Bits::new(s_49_10 as u128, 8u16);
        // D s_49_12: read-var selbitshadow#264:u8
        let s_49_12: bool = fn_state.selbitshadow_264;
        // D s_49_13: cast zx s_49_12 -> bv
        let s_49_13: Bits = Bits::new(s_49_12 as u128, 1u16);
        // D s_49_14: cast reint s_49_11 -> u128
        let s_49_14: u128 = (s_49_11.value() as u128);
        // D s_49_15: size-of s_49_11
        let s_49_15: u16 = s_49_11.length();
        // D s_49_16: cast reint s_49_13 -> u128
        let s_49_16: u128 = (s_49_13.value() as u128);
        // D s_49_17: size-of s_49_13
        let s_49_17: u16 = s_49_13.length();
        // D s_49_18: lsl s_49_14 s_49_17
        let s_49_18: u128 = s_49_14 << s_49_17;
        // D s_49_19: or s_49_18 s_49_16
        let s_49_19: u128 = ((s_49_18) | (s_49_16));
        // D s_49_20: add s_49_15 s_49_17
        let s_49_20: u16 = (s_49_15 + s_49_17);
        // D s_49_21: create-bits s_49_19 s_49_20
        let s_49_21: Bits = Bits::new(s_49_19, s_49_20);
        // D s_49_22: cast reint s_49_21 -> u9
        let s_49_22: u16 = (s_49_21.value() as u16);
        // C s_49_23: const #1s : i
        let s_49_23: i128 = 1;
        // D s_49_24: read-var bottom_PAC_bit:i
        let s_49_24: i128 = fn_state.bottom_PAC_bit;
        // D s_49_25: sub s_49_24 s_49_23
        let s_49_25: i128 = ((s_49_24) - (s_49_23));
        // D s_49_26: cast reint s_49_25 -> i64
        let s_49_26: i64 = (s_49_25 as i64);
        // C s_49_27: const #55s : i
        let s_49_27: i128 = 55;
        // C s_49_28: const #54s : i
        let s_49_28: i128 = 54;
        // C s_49_29: const #0s : i
        let s_49_29: i128 = 0;
        // D s_49_30: read-var PAC:u64
        let s_49_30: u64 = fn_state.PAC;
        // D s_49_31: cast zx s_49_30 -> bv
        let s_49_31: Bits = Bits::new(s_49_30 as u128, 64u16);
        // D s_49_32: read-var ptr:u64
        let s_49_32: u64 = fn_state.ptr;
        // D s_49_33: cast zx s_49_32 -> bv
        let s_49_33: Bits = Bits::new(s_49_32 as u128, 64u16);
        // D s_49_34: cast zx s_49_26 -> i
        let s_49_34: i128 = (i128::try_from(s_49_26).unwrap());
        // D s_49_35: read-var bottom_PAC_bit:i
        let s_49_35: i128 = fn_state.bottom_PAC_bit;
        // D s_49_36: call subrange_subrange_concat(s_49_27, s_49_31, s_49_28, s_49_35, s_49_33, s_49_34, s_49_29)
        let s_49_36: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_49_27,
            s_49_31,
            s_49_28,
            s_49_35,
            s_49_33,
            s_49_34,
            s_49_29,
        );
        // D s_49_37: cast reint s_49_36 -> u55
        let s_49_37: u64 = (s_49_36.value() as u64);
        // D s_49_38: cast zx s_49_22 -> bv
        let s_49_38: Bits = Bits::new(s_49_22 as u128, 9u16);
        // D s_49_39: cast zx s_49_37 -> bv
        let s_49_39: Bits = Bits::new(s_49_37 as u128, 55u16);
        // D s_49_40: cast reint s_49_38 -> u128
        let s_49_40: u128 = (s_49_38.value() as u128);
        // D s_49_41: size-of s_49_38
        let s_49_41: u16 = s_49_38.length();
        // D s_49_42: cast reint s_49_39 -> u128
        let s_49_42: u128 = (s_49_39.value() as u128);
        // D s_49_43: size-of s_49_39
        let s_49_43: u16 = s_49_39.length();
        // D s_49_44: lsl s_49_40 s_49_43
        let s_49_44: u128 = s_49_40 << s_49_43;
        // D s_49_45: or s_49_44 s_49_42
        let s_49_45: u128 = ((s_49_44) | (s_49_42));
        // D s_49_46: add s_49_41 s_49_43
        let s_49_46: u16 = (s_49_41 + s_49_43);
        // D s_49_47: create-bits s_49_45 s_49_46
        let s_49_47: Bits = Bits::new(s_49_45, s_49_46);
        // D s_49_48: cast reint s_49_47 -> u64
        let s_49_48: u64 = (s_49_47.value() as u64);
        // D s_49_49: write-var result <= s_49_48
        fn_state.result = s_49_48;
        // D s_49_50: read-var mtx:u8
        let s_49_50: bool = fn_state.mtx;
        // D s_49_51: not s_49_50
        let s_49_51: bool = !s_49_50;
        // N s_49_52: assert s_49_51
        let s_49_52: () = assert!(s_49_51);
        // N s_49_53: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_50_0: read-var bottom_PAC_bit:i
        let s_50_0: i128 = fn_state.bottom_PAC_bit;
        // D s_50_1: call __id(s_50_0)
        let s_50_1: i128 = u__id(state, tracer, s_50_0);
        // D s_50_2: cast reint s_50_1 -> i64
        let s_50_2: i64 = (s_50_1 as i64);
        // C s_50_3: const #1s : i
        let s_50_3: i128 = 1;
        // D s_50_4: cast zx s_50_2 -> i
        let s_50_4: i128 = (i128::try_from(s_50_2).unwrap());
        // D s_50_5: sub s_50_4 s_50_3
        let s_50_5: i128 = ((s_50_4) - (s_50_3));
        // D s_50_6: cast reint s_50_5 -> i64
        let s_50_6: i64 = (s_50_5 as i64);
        // C s_50_7: const #64s : i
        let s_50_7: i128 = 64;
        // D s_50_8: cast zx s_50_6 -> i
        let s_50_8: i128 = (i128::try_from(s_50_6).unwrap());
        // D s_50_9: cmp-lt s_50_8 s_50_7
        let s_50_9: bool = ((s_50_8) < (s_50_7));
        // D s_50_10: write-var gs#15134 <= s_50_9
        fn_state.gs_15134 = s_50_9;
        // N s_50_11: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_51_0: read-var bottom_PAC_bit:i
        let s_51_0: i128 = fn_state.bottom_PAC_bit;
        // D s_51_1: call __id(s_51_0)
        let s_51_1: i128 = u__id(state, tracer, s_51_0);
        // D s_51_2: cast reint s_51_1 -> i64
        let s_51_2: i64 = (s_51_1 as i64);
        // C s_51_3: const #1s : i
        let s_51_3: i128 = 1;
        // D s_51_4: cast zx s_51_2 -> i
        let s_51_4: i128 = (i128::try_from(s_51_2).unwrap());
        // D s_51_5: sub s_51_4 s_51_3
        let s_51_5: i128 = ((s_51_4) - (s_51_3));
        // D s_51_6: cast reint s_51_5 -> i64
        let s_51_6: i64 = (s_51_5 as i64);
        // C s_51_7: const #0s : i
        let s_51_7: i128 = 0;
        // D s_51_8: cast zx s_51_6 -> i
        let s_51_8: i128 = (i128::try_from(s_51_6).unwrap());
        // D s_51_9: cmp-le s_51_7 s_51_8
        let s_51_9: bool = ((s_51_7) <= (s_51_8));
        // N s_51_10: branch s_51_9 b54 b52
        if s_51_9 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#15147 <= s_52_0
        fn_state.gs_15147 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_53_0: read-var gs#15147:u8
        let s_53_0: bool = fn_state.gs_15147;
        // N s_53_1: assert s_53_0
        let s_53_1: () = assert!(s_53_0);
        // C s_53_2: const #56s : i
        let s_53_2: i128 = 56;
        // D s_53_3: read-var ptr:u64
        let s_53_3: u64 = fn_state.ptr;
        // D s_53_4: cast zx s_53_3 -> bv
        let s_53_4: Bits = Bits::new(s_53_3 as u128, 64u16);
        // C s_53_5: const #1s : i64
        let s_53_5: i64 = 1;
        // C s_53_6: cast zx s_53_5 -> i
        let s_53_6: i128 = (i128::try_from(s_53_5).unwrap());
        // C s_53_7: const #7s : i
        let s_53_7: i128 = 7;
        // C s_53_8: add s_53_7 s_53_6
        let s_53_8: i128 = (s_53_7 + s_53_6);
        // D s_53_9: bit-extract s_53_4 s_53_2 s_53_8
        let s_53_9: Bits = (Bits::new(
            ((s_53_4) >> (s_53_2)).value(),
            u16::try_from(s_53_8).unwrap(),
        ));
        // D s_53_10: cast reint s_53_9 -> u8
        let s_53_10: u8 = (s_53_9.value() as u8);
        // D s_53_11: cast zx s_53_10 -> bv
        let s_53_11: Bits = Bits::new(s_53_10 as u128, 8u16);
        // D s_53_12: read-var selbitshadow#264:u8
        let s_53_12: bool = fn_state.selbitshadow_264;
        // D s_53_13: cast zx s_53_12 -> bv
        let s_53_13: Bits = Bits::new(s_53_12 as u128, 1u16);
        // D s_53_14: cast reint s_53_11 -> u128
        let s_53_14: u128 = (s_53_11.value() as u128);
        // D s_53_15: size-of s_53_11
        let s_53_15: u16 = s_53_11.length();
        // D s_53_16: cast reint s_53_13 -> u128
        let s_53_16: u128 = (s_53_13.value() as u128);
        // D s_53_17: size-of s_53_13
        let s_53_17: u16 = s_53_13.length();
        // D s_53_18: lsl s_53_14 s_53_17
        let s_53_18: u128 = s_53_14 << s_53_17;
        // D s_53_19: or s_53_18 s_53_16
        let s_53_19: u128 = ((s_53_18) | (s_53_16));
        // D s_53_20: add s_53_15 s_53_17
        let s_53_20: u16 = (s_53_15 + s_53_17);
        // D s_53_21: create-bits s_53_19 s_53_20
        let s_53_21: Bits = Bits::new(s_53_19, s_53_20);
        // D s_53_22: cast reint s_53_21 -> u9
        let s_53_22: u16 = (s_53_21.value() as u16);
        // C s_53_23: const #1s : i
        let s_53_23: i128 = 1;
        // D s_53_24: read-var bottom_PAC_bit:i
        let s_53_24: i128 = fn_state.bottom_PAC_bit;
        // D s_53_25: sub s_53_24 s_53_23
        let s_53_25: i128 = ((s_53_24) - (s_53_23));
        // D s_53_26: cast reint s_53_25 -> i64
        let s_53_26: i64 = (s_53_25 as i64);
        // C s_53_27: const #55s : i
        let s_53_27: i128 = 55;
        // C s_53_28: const #54s : i
        let s_53_28: i128 = 54;
        // C s_53_29: const #0s : i
        let s_53_29: i128 = 0;
        // D s_53_30: read-var PAC:u64
        let s_53_30: u64 = fn_state.PAC;
        // D s_53_31: cast zx s_53_30 -> bv
        let s_53_31: Bits = Bits::new(s_53_30 as u128, 64u16);
        // D s_53_32: read-var ptr:u64
        let s_53_32: u64 = fn_state.ptr;
        // D s_53_33: cast zx s_53_32 -> bv
        let s_53_33: Bits = Bits::new(s_53_32 as u128, 64u16);
        // D s_53_34: cast zx s_53_26 -> i
        let s_53_34: i128 = (i128::try_from(s_53_26).unwrap());
        // D s_53_35: read-var bottom_PAC_bit:i
        let s_53_35: i128 = fn_state.bottom_PAC_bit;
        // D s_53_36: call subrange_subrange_concat(s_53_27, s_53_31, s_53_28, s_53_35, s_53_33, s_53_34, s_53_29)
        let s_53_36: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_53_27,
            s_53_31,
            s_53_28,
            s_53_35,
            s_53_33,
            s_53_34,
            s_53_29,
        );
        // D s_53_37: cast reint s_53_36 -> u55
        let s_53_37: u64 = (s_53_36.value() as u64);
        // D s_53_38: cast zx s_53_22 -> bv
        let s_53_38: Bits = Bits::new(s_53_22 as u128, 9u16);
        // D s_53_39: cast zx s_53_37 -> bv
        let s_53_39: Bits = Bits::new(s_53_37 as u128, 55u16);
        // D s_53_40: cast reint s_53_38 -> u128
        let s_53_40: u128 = (s_53_38.value() as u128);
        // D s_53_41: size-of s_53_38
        let s_53_41: u16 = s_53_38.length();
        // D s_53_42: cast reint s_53_39 -> u128
        let s_53_42: u128 = (s_53_39.value() as u128);
        // D s_53_43: size-of s_53_39
        let s_53_43: u16 = s_53_39.length();
        // D s_53_44: lsl s_53_40 s_53_43
        let s_53_44: u128 = s_53_40 << s_53_43;
        // D s_53_45: or s_53_44 s_53_42
        let s_53_45: u128 = ((s_53_44) | (s_53_42));
        // D s_53_46: add s_53_41 s_53_43
        let s_53_46: u16 = (s_53_41 + s_53_43);
        // D s_53_47: create-bits s_53_45 s_53_46
        let s_53_47: Bits = Bits::new(s_53_45, s_53_46);
        // D s_53_48: cast reint s_53_47 -> u64
        let s_53_48: u64 = (s_53_47.value() as u64);
        // D s_53_49: write-var result <= s_53_48
        fn_state.result = s_53_48;
        // N s_53_50: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_54_0: read-var bottom_PAC_bit:i
        let s_54_0: i128 = fn_state.bottom_PAC_bit;
        // D s_54_1: call __id(s_54_0)
        let s_54_1: i128 = u__id(state, tracer, s_54_0);
        // D s_54_2: cast reint s_54_1 -> i64
        let s_54_2: i64 = (s_54_1 as i64);
        // C s_54_3: const #1s : i
        let s_54_3: i128 = 1;
        // D s_54_4: cast zx s_54_2 -> i
        let s_54_4: i128 = (i128::try_from(s_54_2).unwrap());
        // D s_54_5: sub s_54_4 s_54_3
        let s_54_5: i128 = ((s_54_4) - (s_54_3));
        // D s_54_6: cast reint s_54_5 -> i64
        let s_54_6: i64 = (s_54_5 as i64);
        // C s_54_7: const #64s : i
        let s_54_7: i128 = 64;
        // D s_54_8: cast zx s_54_6 -> i
        let s_54_8: i128 = (i128::try_from(s_54_6).unwrap());
        // D s_54_9: cmp-lt s_54_8 s_54_7
        let s_54_9: bool = ((s_54_8) < (s_54_7));
        // D s_54_10: write-var gs#15147 <= s_54_9
        fn_state.gs_15147 = s_54_9;
        // N s_54_11: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call HaveEnhancedPAC(s_55_0)
        let s_55_1: bool = HaveEnhancedPAC(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b59 b56
        if s_55_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call HaveEnhancedPAC2(s_56_0)
        let s_56_1: bool = HaveEnhancedPAC2(state, tracer, s_56_0);
        // S s_56_2: not s_56_1
        let s_56_2: bool = !s_56_1;
        // N s_56_3: branch s_56_2 b58 b57
        if s_56_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_57_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_58_0: const #1s : i
        let s_58_0: i128 = 1;
        // D s_58_1: read-var top_bit:i64
        let s_58_1: i64 = fn_state.top_bit;
        // D s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (i128::try_from(s_58_1).unwrap());
        // D s_58_3: sub s_58_2 s_58_0
        let s_58_3: i128 = ((s_58_2) - (s_58_0));
        // D s_58_4: cast reint s_58_3 -> i64
        let s_58_4: i64 = (s_58_3 as i64);
        // C s_58_5: const #1s : i
        let s_58_5: i128 = 1;
        // D s_58_6: read-var top_bit:i64
        let s_58_6: i64 = fn_state.top_bit;
        // D s_58_7: cast zx s_58_6 -> i
        let s_58_7: i128 = (i128::try_from(s_58_6).unwrap());
        // D s_58_8: sub s_58_7 s_58_5
        let s_58_8: i128 = ((s_58_7) - (s_58_5));
        // D s_58_9: cast reint s_58_8 -> i64
        let s_58_9: i64 = (s_58_8 as i64);
        // D s_58_10: read-var PAC:u64
        let s_58_10: u64 = fn_state.PAC;
        // D s_58_11: cast zx s_58_10 -> bv
        let s_58_11: Bits = Bits::new(s_58_10 as u128, 64u16);
        // D s_58_12: cast zx s_58_9 -> i
        let s_58_12: i128 = (i128::try_from(s_58_9).unwrap());
        // C s_58_13: const #1u : u64
        let s_58_13: u64 = 1;
        // D s_58_14: bit-extract s_58_11 s_58_12 s_58_13
        let s_58_14: Bits = (Bits::new(
            ((s_58_11) >> (s_58_12)).value(),
            u16::try_from(s_58_13).unwrap(),
        ));
        // D s_58_15: cast reint s_58_14 -> u8
        let s_58_15: bool = ((s_58_14.value()) != 0);
        // C s_58_16: const #0s : i
        let s_58_16: i128 = 0;
        // C s_58_17: const #0u : u64
        let s_58_17: u64 = 0;
        // D s_58_18: cast zx s_58_15 -> u64
        let s_58_18: u64 = (s_58_15 as u64);
        // C s_58_19: const #1u : u64
        let s_58_19: u64 = 1;
        // D s_58_20: and s_58_18 s_58_19
        let s_58_20: u64 = ((s_58_18) & (s_58_19));
        // D s_58_21: cmp-eq s_58_20 s_58_19
        let s_58_21: bool = ((s_58_20) == (s_58_19));
        // D s_58_22: lsl s_58_18 s_58_16
        let s_58_22: u64 = s_58_18 << s_58_16;
        // D s_58_23: or s_58_17 s_58_22
        let s_58_23: u64 = ((s_58_17) | (s_58_22));
        // D s_58_24: cmpl s_58_22
        let s_58_24: u64 = !s_58_22;
        // D s_58_25: and s_58_17 s_58_24
        let s_58_25: u64 = ((s_58_17) & (s_58_24));
        // D s_58_26: select s_58_21 s_58_23 s_58_25
        let s_58_26: u64 = if s_58_21 { s_58_23 } else { s_58_25 };
        // D s_58_27: cast trunc s_58_26 -> u8
        let s_58_27: bool = ((s_58_26) != 0);
        // D s_58_28: cast zx s_58_27 -> bv
        let s_58_28: Bits = Bits::new(s_58_27 as u128, 1u16);
        // D s_58_29: not s_58_28
        let s_58_29: Bits = !s_58_28;
        // D s_58_30: cast reint s_58_29 -> u8
        let s_58_30: bool = ((s_58_29.value()) != 0);
        // D s_58_31: call Bit(s_58_30)
        let s_58_31: bool = Bit(state, tracer, s_58_30);
        // D s_58_32: read-var PAC:u64
        let s_58_32: u64 = fn_state.PAC;
        // D s_58_33: cast zx s_58_32 -> bv
        let s_58_33: Bits = Bits::new(s_58_32 as u128, 64u16);
        // D s_58_34: cast zx s_58_4 -> i
        let s_58_34: i128 = (i128::try_from(s_58_4).unwrap());
        // C s_58_35: const #1u : u64
        let s_58_35: u64 = 1;
        // D s_58_36: bit-insert s_58_33 s_58_33 s_58_34 s_58_35
        let s_58_36: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_58_35 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_58_33.length(),
            );
            (s_58_33 & mask) | (s_58_33 << s_58_34)
        };
        // D s_58_37: cast reint s_58_36 -> u64
        let s_58_37: u64 = (s_58_36.value() as u64);
        // D s_58_38: write-var PAC <= s_58_37
        fn_state.PAC = s_58_37;
        // N s_58_39: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_59_0: const #0u : u64
        let s_59_0: u64 = 0;
        // D s_59_1: write-var PAC <= s_59_0
        fn_state.PAC = s_59_0;
        // N s_59_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_60_0: read-var ptr:u64
        let s_60_0: u64 = fn_state.ptr;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 64u16);
        // D s_60_2: read-var unusedbits_mask:u64
        let s_60_2: u64 = fn_state.unusedbits_mask;
        // D s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 64u16);
        // D s_60_4: and s_60_1 s_60_3
        let s_60_4: Bits = ((s_60_1) & (s_60_3));
        // D s_60_5: cast reint s_60_4 -> u64
        let s_60_5: u64 = (s_60_4.value() as u64);
        // D s_60_6: cast zx s_60_5 -> bv
        let s_60_6: Bits = Bits::new(s_60_5 as u128, 64u16);
        // D s_60_7: read-var unusedbits_mask:u64
        let s_60_7: u64 = fn_state.unusedbits_mask;
        // D s_60_8: cast zx s_60_7 -> bv
        let s_60_8: Bits = Bits::new(s_60_7 as u128, 64u16);
        // D s_60_9: cmp-ne s_60_6 s_60_8
        let s_60_9: bool = ((s_60_6) != (s_60_8));
        // D s_60_10: write-var gs#15086 <= s_60_9
        fn_state.gs_15086 = s_60_9;
        // N s_60_11: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_61_0: const #4s : i
        let s_61_0: i128 = 4;
        // S s_61_1: call Ones(s_61_0)
        let s_61_1: Bits = Ones(state, tracer, s_61_0);
        // S s_61_2: cast reint s_61_1 -> u8
        let s_61_2: u8 = (s_61_1.value() as u8);
        // C s_61_3: const #60s : i
        let s_61_3: i128 = 60;
        // D s_61_4: read-var unusedbits_mask:u64
        let s_61_4: u64 = fn_state.unusedbits_mask;
        // D s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 64u16);
        // S s_61_6: cast zx s_61_2 -> bv
        let s_61_6: Bits = Bits::new(s_61_2 as u128, 4u16);
        // C s_61_7: const #3s : i
        let s_61_7: i128 = 3;
        // C s_61_8: const #1u : u64
        let s_61_8: u64 = 1;
        // C s_61_9: cast zx s_61_8 -> bv
        let s_61_9: Bits = Bits::new(s_61_8 as u128, 64u16);
        // C s_61_10: lsl s_61_9 s_61_7
        let s_61_10: Bits = s_61_9 << s_61_7;
        // C s_61_11: sub s_61_10 s_61_9
        let s_61_11: Bits = ((s_61_10) - (s_61_9));
        // S s_61_12: and s_61_6 s_61_11
        let s_61_12: Bits = ((s_61_6) & (s_61_11));
        // S s_61_13: lsl s_61_12 s_61_3
        let s_61_13: Bits = s_61_12 << s_61_3;
        // C s_61_14: lsl s_61_11 s_61_3
        let s_61_14: Bits = s_61_11 << s_61_3;
        // C s_61_15: cmpl s_61_14
        let s_61_15: Bits = !s_61_14;
        // D s_61_16: and s_61_5 s_61_15
        let s_61_16: Bits = ((s_61_5) & (s_61_15));
        // D s_61_17: or s_61_16 s_61_13
        let s_61_17: Bits = ((s_61_16) | (s_61_13));
        // D s_61_18: cast reint s_61_17 -> u64
        let s_61_18: u64 = (s_61_17.value() as u64);
        // D s_61_19: write-var unusedbits_mask <= s_61_18
        fn_state.unusedbits_mask = s_61_18;
        // N s_61_20: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_62_0: const #8s : i
        let s_62_0: i128 = 8;
        // S s_62_1: call Ones(s_62_0)
        let s_62_1: Bits = Ones(state, tracer, s_62_0);
        // S s_62_2: cast reint s_62_1 -> u8
        let s_62_2: u8 = (s_62_1.value() as u8);
        // C s_62_3: const #56s : i
        let s_62_3: i128 = 56;
        // D s_62_4: read-var unusedbits_mask:u64
        let s_62_4: u64 = fn_state.unusedbits_mask;
        // D s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 64u16);
        // S s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 8u16);
        // C s_62_7: const #7s : i
        let s_62_7: i128 = 7;
        // C s_62_8: const #1u : u64
        let s_62_8: u64 = 1;
        // C s_62_9: cast zx s_62_8 -> bv
        let s_62_9: Bits = Bits::new(s_62_8 as u128, 64u16);
        // C s_62_10: lsl s_62_9 s_62_7
        let s_62_10: Bits = s_62_9 << s_62_7;
        // C s_62_11: sub s_62_10 s_62_9
        let s_62_11: Bits = ((s_62_10) - (s_62_9));
        // S s_62_12: and s_62_6 s_62_11
        let s_62_12: Bits = ((s_62_6) & (s_62_11));
        // S s_62_13: lsl s_62_12 s_62_3
        let s_62_13: Bits = s_62_12 << s_62_3;
        // C s_62_14: lsl s_62_11 s_62_3
        let s_62_14: Bits = s_62_11 << s_62_3;
        // C s_62_15: cmpl s_62_14
        let s_62_15: Bits = !s_62_14;
        // D s_62_16: and s_62_5 s_62_15
        let s_62_16: Bits = ((s_62_5) & (s_62_15));
        // D s_62_17: or s_62_16 s_62_13
        let s_62_17: Bits = ((s_62_16) | (s_62_13));
        // D s_62_18: cast reint s_62_17 -> u64
        let s_62_18: u64 = (s_62_17.value() as u64);
        // D s_62_19: write-var unusedbits_mask <= s_62_18
        fn_state.unusedbits_mask = s_62_18;
        // N s_62_20: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_63_0: read-var bottom_PAC_bit:i
        let s_63_0: i128 = fn_state.bottom_PAC_bit;
        // D s_63_1: call __id(s_63_0)
        let s_63_1: i128 = u__id(state, tracer, s_63_0);
        // C s_63_2: const #54s : i
        let s_63_2: i128 = 54;
        // D s_63_3: cmp-le s_63_1 s_63_2
        let s_63_3: bool = ((s_63_1) <= (s_63_2));
        // D s_63_4: write-var gs#15082 <= s_63_3
        fn_state.gs_15082 = s_63_3;
        // N s_63_5: jump b22
        return block_22(state, tracer, fn_state);
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
        // D s_64_2: cast reint s_64_1 -> i64
        let s_64_2: i64 = (s_64_1 as i64);
        // C s_64_3: const #1s : i
        let s_64_3: i128 = 1;
        // D s_64_4: cast zx s_64_2 -> i
        let s_64_4: i128 = (i128::try_from(s_64_2).unwrap());
        // D s_64_5: sub s_64_4 s_64_3
        let s_64_5: i128 = ((s_64_4) - (s_64_3));
        // D s_64_6: cast reint s_64_5 -> i64
        let s_64_6: i64 = (s_64_5 as i64);
        // C s_64_7: const #64s : i
        let s_64_7: i128 = 64;
        // D s_64_8: cast zx s_64_6 -> i
        let s_64_8: i128 = (i128::try_from(s_64_6).unwrap());
        // D s_64_9: cmp-lt s_64_8 s_64_7
        let s_64_9: bool = ((s_64_8) < (s_64_7));
        // D s_64_10: write-var gs#15020 <= s_64_9
        fn_state.gs_15020 = s_64_9;
        // N s_64_11: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_65_0: read-var bottom_PAC_bit:i
        let s_65_0: i128 = fn_state.bottom_PAC_bit;
        // D s_65_1: call __id(s_65_0)
        let s_65_1: i128 = u__id(state, tracer, s_65_0);
        // C s_65_2: const #64s : i
        let s_65_2: i128 = 64;
        // D s_65_3: sub s_65_2 s_65_1
        let s_65_3: i128 = ((s_65_2) - (s_65_1));
        // C s_65_4: const #1s : i
        let s_65_4: i128 = 1;
        // D s_65_5: sub s_65_3 s_65_4
        let s_65_5: i128 = ((s_65_3) - (s_65_4));
        // C s_65_6: const #64s : i
        let s_65_6: i128 = 64;
        // D s_65_7: cmp-lt s_65_5 s_65_6
        let s_65_7: bool = ((s_65_5) < (s_65_6));
        // D s_65_8: write-var gs#15015 <= s_65_7
        fn_state.gs_15015 = s_65_7;
        // N s_65_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_66_0: read-var bottom_PAC_bit:i
        let s_66_0: i128 = fn_state.bottom_PAC_bit;
        // D s_66_1: call __id(s_66_0)
        let s_66_1: i128 = u__id(state, tracer, s_66_0);
        // C s_66_2: const #56s : i
        let s_66_2: i128 = 56;
        // D s_66_3: sub s_66_2 s_66_1
        let s_66_3: i128 = ((s_66_2) - (s_66_1));
        // C s_66_4: const #1s : i
        let s_66_4: i128 = 1;
        // D s_66_5: sub s_66_3 s_66_4
        let s_66_5: i128 = ((s_66_3) - (s_66_4));
        // C s_66_6: const #0s : i
        let s_66_6: i128 = 0;
        // D s_66_7: cmp-le s_66_6 s_66_5
        let s_66_7: bool = ((s_66_6) <= (s_66_5));
        // N s_66_8: branch s_66_7 b72 b67
        if s_66_7 {
            return block_72(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#15035 <= s_67_0
        fn_state.gs_15035 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_68_0: read-var gs#15035:u8
        let s_68_0: bool = fn_state.gs_15035;
        // N s_68_1: assert s_68_0
        let s_68_1: () = assert!(s_68_0);
        // D s_68_2: read-var bottom_PAC_bit:i
        let s_68_2: i128 = fn_state.bottom_PAC_bit;
        // D s_68_3: call __id(s_68_2)
        let s_68_3: i128 = u__id(state, tracer, s_68_2);
        // D s_68_4: cast reint s_68_3 -> i64
        let s_68_4: i64 = (s_68_3 as i64);
        // C s_68_5: const #1s : i
        let s_68_5: i128 = 1;
        // D s_68_6: cast zx s_68_4 -> i
        let s_68_6: i128 = (i128::try_from(s_68_4).unwrap());
        // D s_68_7: sub s_68_6 s_68_5
        let s_68_7: i128 = ((s_68_6) - (s_68_5));
        // D s_68_8: cast reint s_68_7 -> i64
        let s_68_8: i64 = (s_68_7 as i64);
        // C s_68_9: const #0s : i
        let s_68_9: i128 = 0;
        // D s_68_10: cast zx s_68_8 -> i
        let s_68_10: i128 = (i128::try_from(s_68_8).unwrap());
        // D s_68_11: cmp-le s_68_9 s_68_10
        let s_68_11: bool = ((s_68_9) <= (s_68_10));
        // N s_68_12: branch s_68_11 b71 b69
        if s_68_11 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#15040 <= s_69_0
        fn_state.gs_15040 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_70_0: read-var gs#15040:u8
        let s_70_0: bool = fn_state.gs_15040;
        // N s_70_1: assert s_70_0
        let s_70_1: () = assert!(s_70_0);
        // C s_70_2: const #60s : i
        let s_70_2: i128 = 60;
        // D s_70_3: read-var extfield:u64
        let s_70_3: u64 = fn_state.extfield;
        // D s_70_4: cast zx s_70_3 -> bv
        let s_70_4: Bits = Bits::new(s_70_3 as u128, 64u16);
        // C s_70_5: const #1s : i64
        let s_70_5: i64 = 1;
        // C s_70_6: cast zx s_70_5 -> i
        let s_70_6: i128 = (i128::try_from(s_70_5).unwrap());
        // C s_70_7: const #3s : i
        let s_70_7: i128 = 3;
        // C s_70_8: add s_70_7 s_70_6
        let s_70_8: i128 = (s_70_7 + s_70_6);
        // D s_70_9: bit-extract s_70_4 s_70_2 s_70_8
        let s_70_9: Bits = (Bits::new(
            ((s_70_4) >> (s_70_2)).value(),
            u16::try_from(s_70_8).unwrap(),
        ));
        // D s_70_10: cast reint s_70_9 -> u8
        let s_70_10: u8 = (s_70_9.value() as u8);
        // C s_70_11: const #56s : i
        let s_70_11: i128 = 56;
        // D s_70_12: read-var ptr:u64
        let s_70_12: u64 = fn_state.ptr;
        // D s_70_13: cast zx s_70_12 -> bv
        let s_70_13: Bits = Bits::new(s_70_12 as u128, 64u16);
        // C s_70_14: const #1s : i64
        let s_70_14: i64 = 1;
        // C s_70_15: cast zx s_70_14 -> i
        let s_70_15: i128 = (i128::try_from(s_70_14).unwrap());
        // C s_70_16: const #3s : i
        let s_70_16: i128 = 3;
        // C s_70_17: add s_70_16 s_70_15
        let s_70_17: i128 = (s_70_16 + s_70_15);
        // D s_70_18: bit-extract s_70_13 s_70_11 s_70_17
        let s_70_18: Bits = (Bits::new(
            ((s_70_13) >> (s_70_11)).value(),
            u16::try_from(s_70_17).unwrap(),
        ));
        // D s_70_19: cast reint s_70_18 -> u8
        let s_70_19: u8 = (s_70_18.value() as u8);
        // D s_70_20: cast zx s_70_10 -> bv
        let s_70_20: Bits = Bits::new(s_70_10 as u128, 4u16);
        // D s_70_21: cast zx s_70_19 -> bv
        let s_70_21: Bits = Bits::new(s_70_19 as u128, 4u16);
        // D s_70_22: cast reint s_70_20 -> u128
        let s_70_22: u128 = (s_70_20.value() as u128);
        // D s_70_23: size-of s_70_20
        let s_70_23: u16 = s_70_20.length();
        // D s_70_24: cast reint s_70_21 -> u128
        let s_70_24: u128 = (s_70_21.value() as u128);
        // D s_70_25: size-of s_70_21
        let s_70_25: u16 = s_70_21.length();
        // D s_70_26: lsl s_70_22 s_70_25
        let s_70_26: u128 = s_70_22 << s_70_25;
        // D s_70_27: or s_70_26 s_70_24
        let s_70_27: u128 = ((s_70_26) | (s_70_24));
        // D s_70_28: add s_70_23 s_70_25
        let s_70_28: u16 = (s_70_23 + s_70_25);
        // D s_70_29: create-bits s_70_27 s_70_28
        let s_70_29: Bits = Bits::new(s_70_27, s_70_28);
        // D s_70_30: cast reint s_70_29 -> u8
        let s_70_30: u8 = (s_70_29.value() as u8);
        // C s_70_31: const #56s : i
        let s_70_31: i128 = 56;
        // D s_70_32: read-var bottom_PAC_bit:i
        let s_70_32: i128 = fn_state.bottom_PAC_bit;
        // D s_70_33: sub s_70_31 s_70_32
        let s_70_33: i128 = ((s_70_31) - (s_70_32));
        // D s_70_34: cast reint s_70_33 -> i64
        let s_70_34: i64 = (s_70_33 as i64);
        // C s_70_35: const #1s : i
        let s_70_35: i128 = 1;
        // D s_70_36: cast zx s_70_34 -> i
        let s_70_36: i128 = (i128::try_from(s_70_34).unwrap());
        // D s_70_37: sub s_70_36 s_70_35
        let s_70_37: i128 = ((s_70_36) - (s_70_35));
        // D s_70_38: cast reint s_70_37 -> i64
        let s_70_38: i64 = (s_70_37 as i64);
        // C s_70_39: const #1s : i
        let s_70_39: i128 = 1;
        // D s_70_40: read-var bottom_PAC_bit:i
        let s_70_40: i128 = fn_state.bottom_PAC_bit;
        // D s_70_41: sub s_70_40 s_70_39
        let s_70_41: i128 = ((s_70_40) - (s_70_39));
        // D s_70_42: cast reint s_70_41 -> i64
        let s_70_42: i64 = (s_70_41 as i64);
        // C s_70_43: const #56s : i
        let s_70_43: i128 = 56;
        // C s_70_44: const #0s : i
        let s_70_44: i128 = 0;
        // C s_70_45: const #0s : i
        let s_70_45: i128 = 0;
        // D s_70_46: read-var extfield:u64
        let s_70_46: u64 = fn_state.extfield;
        // D s_70_47: cast zx s_70_46 -> bv
        let s_70_47: Bits = Bits::new(s_70_46 as u128, 64u16);
        // D s_70_48: cast zx s_70_38 -> i
        let s_70_48: i128 = (i128::try_from(s_70_38).unwrap());
        // D s_70_49: read-var ptr:u64
        let s_70_49: u64 = fn_state.ptr;
        // D s_70_50: cast zx s_70_49 -> bv
        let s_70_50: Bits = Bits::new(s_70_49 as u128, 64u16);
        // D s_70_51: cast zx s_70_42 -> i
        let s_70_51: i128 = (i128::try_from(s_70_42).unwrap());
        // D s_70_52: call subrange_subrange_concat(s_70_43, s_70_47, s_70_48, s_70_44, s_70_50, s_70_51, s_70_45)
        let s_70_52: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_70_43,
            s_70_47,
            s_70_48,
            s_70_44,
            s_70_50,
            s_70_51,
            s_70_45,
        );
        // D s_70_53: cast reint s_70_52 -> u56
        let s_70_53: u64 = (s_70_52.value() as u64);
        // D s_70_54: cast zx s_70_30 -> bv
        let s_70_54: Bits = Bits::new(s_70_30 as u128, 8u16);
        // D s_70_55: cast zx s_70_53 -> bv
        let s_70_55: Bits = Bits::new(s_70_53 as u128, 56u16);
        // D s_70_56: cast reint s_70_54 -> u128
        let s_70_56: u128 = (s_70_54.value() as u128);
        // D s_70_57: size-of s_70_54
        let s_70_57: u16 = s_70_54.length();
        // D s_70_58: cast reint s_70_55 -> u128
        let s_70_58: u128 = (s_70_55.value() as u128);
        // D s_70_59: size-of s_70_55
        let s_70_59: u16 = s_70_55.length();
        // D s_70_60: lsl s_70_56 s_70_59
        let s_70_60: u128 = s_70_56 << s_70_59;
        // D s_70_61: or s_70_60 s_70_58
        let s_70_61: u128 = ((s_70_60) | (s_70_58));
        // D s_70_62: add s_70_57 s_70_59
        let s_70_62: u16 = (s_70_57 + s_70_59);
        // D s_70_63: create-bits s_70_61 s_70_62
        let s_70_63: Bits = Bits::new(s_70_61, s_70_62);
        // D s_70_64: cast reint s_70_63 -> u64
        let s_70_64: u64 = (s_70_63.value() as u64);
        // D s_70_65: write-var ext_ptr <= s_70_64
        fn_state.ext_ptr = s_70_64;
        // N s_70_66: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_71_0: read-var bottom_PAC_bit:i
        let s_71_0: i128 = fn_state.bottom_PAC_bit;
        // D s_71_1: call __id(s_71_0)
        let s_71_1: i128 = u__id(state, tracer, s_71_0);
        // D s_71_2: cast reint s_71_1 -> i64
        let s_71_2: i64 = (s_71_1 as i64);
        // C s_71_3: const #1s : i
        let s_71_3: i128 = 1;
        // D s_71_4: cast zx s_71_2 -> i
        let s_71_4: i128 = (i128::try_from(s_71_2).unwrap());
        // D s_71_5: sub s_71_4 s_71_3
        let s_71_5: i128 = ((s_71_4) - (s_71_3));
        // D s_71_6: cast reint s_71_5 -> i64
        let s_71_6: i64 = (s_71_5 as i64);
        // C s_71_7: const #64s : i
        let s_71_7: i128 = 64;
        // D s_71_8: cast zx s_71_6 -> i
        let s_71_8: i128 = (i128::try_from(s_71_6).unwrap());
        // D s_71_9: cmp-lt s_71_8 s_71_7
        let s_71_9: bool = ((s_71_8) < (s_71_7));
        // D s_71_10: write-var gs#15040 <= s_71_9
        fn_state.gs_15040 = s_71_9;
        // N s_71_11: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_72_0: read-var bottom_PAC_bit:i
        let s_72_0: i128 = fn_state.bottom_PAC_bit;
        // D s_72_1: call __id(s_72_0)
        let s_72_1: i128 = u__id(state, tracer, s_72_0);
        // C s_72_2: const #56s : i
        let s_72_2: i128 = 56;
        // D s_72_3: sub s_72_2 s_72_1
        let s_72_3: i128 = ((s_72_2) - (s_72_1));
        // C s_72_4: const #1s : i
        let s_72_4: i128 = 1;
        // D s_72_5: sub s_72_3 s_72_4
        let s_72_5: i128 = ((s_72_3) - (s_72_4));
        // C s_72_6: const #64s : i
        let s_72_6: i128 = 64;
        // D s_72_7: cmp-lt s_72_5 s_72_6
        let s_72_7: bool = ((s_72_5) < (s_72_6));
        // D s_72_8: write-var gs#15035 <= s_72_7
        fn_state.gs_15035 = s_72_7;
        // N s_72_9: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_73_0: read-var bottom_PAC_bit:i
        let s_73_0: i128 = fn_state.bottom_PAC_bit;
        // D s_73_1: call __id(s_73_0)
        let s_73_1: i128 = u__id(state, tracer, s_73_0);
        // C s_73_2: const #56s : i
        let s_73_2: i128 = 56;
        // D s_73_3: sub s_73_2 s_73_1
        let s_73_3: i128 = ((s_73_2) - (s_73_1));
        // C s_73_4: const #1s : i
        let s_73_4: i128 = 1;
        // D s_73_5: sub s_73_3 s_73_4
        let s_73_5: i128 = ((s_73_3) - (s_73_4));
        // C s_73_6: const #0s : i
        let s_73_6: i128 = 0;
        // D s_73_7: cmp-le s_73_6 s_73_5
        let s_73_7: bool = ((s_73_6) <= (s_73_5));
        // N s_73_8: branch s_73_7 b79 b74
        if s_73_7 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#15059 <= s_74_0
        fn_state.gs_15059 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_75_0: read-var gs#15059:u8
        let s_75_0: bool = fn_state.gs_15059;
        // N s_75_1: assert s_75_0
        let s_75_1: () = assert!(s_75_0);
        // D s_75_2: read-var bottom_PAC_bit:i
        let s_75_2: i128 = fn_state.bottom_PAC_bit;
        // D s_75_3: call __id(s_75_2)
        let s_75_3: i128 = u__id(state, tracer, s_75_2);
        // D s_75_4: cast reint s_75_3 -> i64
        let s_75_4: i64 = (s_75_3 as i64);
        // C s_75_5: const #1s : i
        let s_75_5: i128 = 1;
        // D s_75_6: cast zx s_75_4 -> i
        let s_75_6: i128 = (i128::try_from(s_75_4).unwrap());
        // D s_75_7: sub s_75_6 s_75_5
        let s_75_7: i128 = ((s_75_6) - (s_75_5));
        // D s_75_8: cast reint s_75_7 -> i64
        let s_75_8: i64 = (s_75_7 as i64);
        // C s_75_9: const #0s : i
        let s_75_9: i128 = 0;
        // D s_75_10: cast zx s_75_8 -> i
        let s_75_10: i128 = (i128::try_from(s_75_8).unwrap());
        // D s_75_11: cmp-le s_75_9 s_75_10
        let s_75_11: bool = ((s_75_9) <= (s_75_10));
        // N s_75_12: branch s_75_11 b78 b76
        if s_75_11 {
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
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#15064 <= s_76_0
        fn_state.gs_15064 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_77_0: read-var gs#15064:u8
        let s_77_0: bool = fn_state.gs_15064;
        // N s_77_1: assert s_77_0
        let s_77_1: () = assert!(s_77_0);
        // C s_77_2: const #56s : i
        let s_77_2: i128 = 56;
        // D s_77_3: read-var ptr:u64
        let s_77_3: u64 = fn_state.ptr;
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 64u16);
        // C s_77_5: const #1s : i64
        let s_77_5: i64 = 1;
        // C s_77_6: cast zx s_77_5 -> i
        let s_77_6: i128 = (i128::try_from(s_77_5).unwrap());
        // C s_77_7: const #7s : i
        let s_77_7: i128 = 7;
        // C s_77_8: add s_77_7 s_77_6
        let s_77_8: i128 = (s_77_7 + s_77_6);
        // D s_77_9: bit-extract s_77_4 s_77_2 s_77_8
        let s_77_9: Bits = (Bits::new(
            ((s_77_4) >> (s_77_2)).value(),
            u16::try_from(s_77_8).unwrap(),
        ));
        // D s_77_10: cast reint s_77_9 -> u8
        let s_77_10: u8 = (s_77_9.value() as u8);
        // C s_77_11: const #56s : i
        let s_77_11: i128 = 56;
        // D s_77_12: read-var bottom_PAC_bit:i
        let s_77_12: i128 = fn_state.bottom_PAC_bit;
        // D s_77_13: sub s_77_11 s_77_12
        let s_77_13: i128 = ((s_77_11) - (s_77_12));
        // D s_77_14: cast reint s_77_13 -> i64
        let s_77_14: i64 = (s_77_13 as i64);
        // C s_77_15: const #1s : i
        let s_77_15: i128 = 1;
        // D s_77_16: cast zx s_77_14 -> i
        let s_77_16: i128 = (i128::try_from(s_77_14).unwrap());
        // D s_77_17: sub s_77_16 s_77_15
        let s_77_17: i128 = ((s_77_16) - (s_77_15));
        // D s_77_18: cast reint s_77_17 -> i64
        let s_77_18: i64 = (s_77_17 as i64);
        // C s_77_19: const #1s : i
        let s_77_19: i128 = 1;
        // D s_77_20: read-var bottom_PAC_bit:i
        let s_77_20: i128 = fn_state.bottom_PAC_bit;
        // D s_77_21: sub s_77_20 s_77_19
        let s_77_21: i128 = ((s_77_20) - (s_77_19));
        // D s_77_22: cast reint s_77_21 -> i64
        let s_77_22: i64 = (s_77_21 as i64);
        // C s_77_23: const #56s : i
        let s_77_23: i128 = 56;
        // C s_77_24: const #0s : i
        let s_77_24: i128 = 0;
        // C s_77_25: const #0s : i
        let s_77_25: i128 = 0;
        // D s_77_26: read-var extfield:u64
        let s_77_26: u64 = fn_state.extfield;
        // D s_77_27: cast zx s_77_26 -> bv
        let s_77_27: Bits = Bits::new(s_77_26 as u128, 64u16);
        // D s_77_28: cast zx s_77_18 -> i
        let s_77_28: i128 = (i128::try_from(s_77_18).unwrap());
        // D s_77_29: read-var ptr:u64
        let s_77_29: u64 = fn_state.ptr;
        // D s_77_30: cast zx s_77_29 -> bv
        let s_77_30: Bits = Bits::new(s_77_29 as u128, 64u16);
        // D s_77_31: cast zx s_77_22 -> i
        let s_77_31: i128 = (i128::try_from(s_77_22).unwrap());
        // D s_77_32: call subrange_subrange_concat(s_77_23, s_77_27, s_77_28, s_77_24, s_77_30, s_77_31, s_77_25)
        let s_77_32: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_77_23,
            s_77_27,
            s_77_28,
            s_77_24,
            s_77_30,
            s_77_31,
            s_77_25,
        );
        // D s_77_33: cast reint s_77_32 -> u56
        let s_77_33: u64 = (s_77_32.value() as u64);
        // D s_77_34: cast zx s_77_10 -> bv
        let s_77_34: Bits = Bits::new(s_77_10 as u128, 8u16);
        // D s_77_35: cast zx s_77_33 -> bv
        let s_77_35: Bits = Bits::new(s_77_33 as u128, 56u16);
        // D s_77_36: cast reint s_77_34 -> u128
        let s_77_36: u128 = (s_77_34.value() as u128);
        // D s_77_37: size-of s_77_34
        let s_77_37: u16 = s_77_34.length();
        // D s_77_38: cast reint s_77_35 -> u128
        let s_77_38: u128 = (s_77_35.value() as u128);
        // D s_77_39: size-of s_77_35
        let s_77_39: u16 = s_77_35.length();
        // D s_77_40: lsl s_77_36 s_77_39
        let s_77_40: u128 = s_77_36 << s_77_39;
        // D s_77_41: or s_77_40 s_77_38
        let s_77_41: u128 = ((s_77_40) | (s_77_38));
        // D s_77_42: add s_77_37 s_77_39
        let s_77_42: u16 = (s_77_37 + s_77_39);
        // D s_77_43: create-bits s_77_41 s_77_42
        let s_77_43: Bits = Bits::new(s_77_41, s_77_42);
        // D s_77_44: cast reint s_77_43 -> u64
        let s_77_44: u64 = (s_77_43.value() as u64);
        // D s_77_45: write-var ext_ptr <= s_77_44
        fn_state.ext_ptr = s_77_44;
        // N s_77_46: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_78_0: read-var bottom_PAC_bit:i
        let s_78_0: i128 = fn_state.bottom_PAC_bit;
        // D s_78_1: call __id(s_78_0)
        let s_78_1: i128 = u__id(state, tracer, s_78_0);
        // D s_78_2: cast reint s_78_1 -> i64
        let s_78_2: i64 = (s_78_1 as i64);
        // C s_78_3: const #1s : i
        let s_78_3: i128 = 1;
        // D s_78_4: cast zx s_78_2 -> i
        let s_78_4: i128 = (i128::try_from(s_78_2).unwrap());
        // D s_78_5: sub s_78_4 s_78_3
        let s_78_5: i128 = ((s_78_4) - (s_78_3));
        // D s_78_6: cast reint s_78_5 -> i64
        let s_78_6: i64 = (s_78_5 as i64);
        // C s_78_7: const #64s : i
        let s_78_7: i128 = 64;
        // D s_78_8: cast zx s_78_6 -> i
        let s_78_8: i128 = (i128::try_from(s_78_6).unwrap());
        // D s_78_9: cmp-lt s_78_8 s_78_7
        let s_78_9: bool = ((s_78_8) < (s_78_7));
        // D s_78_10: write-var gs#15064 <= s_78_9
        fn_state.gs_15064 = s_78_9;
        // N s_78_11: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_79_0: read-var bottom_PAC_bit:i
        let s_79_0: i128 = fn_state.bottom_PAC_bit;
        // D s_79_1: call __id(s_79_0)
        let s_79_1: i128 = u__id(state, tracer, s_79_0);
        // C s_79_2: const #56s : i
        let s_79_2: i128 = 56;
        // D s_79_3: sub s_79_2 s_79_1
        let s_79_3: i128 = ((s_79_2) - (s_79_1));
        // C s_79_4: const #1s : i
        let s_79_4: i128 = 1;
        // D s_79_5: sub s_79_3 s_79_4
        let s_79_5: i128 = ((s_79_3) - (s_79_4));
        // C s_79_6: const #64s : i
        let s_79_6: i128 = 64;
        // D s_79_7: cmp-lt s_79_5 s_79_6
        let s_79_7: bool = ((s_79_5) < (s_79_6));
        // D s_79_8: write-var gs#15059 <= s_79_7
        fn_state.gs_15059 = s_79_7;
        // N s_79_9: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_80_0: read-var ptr:u64
        let s_80_0: u64 = fn_state.ptr;
        // D s_80_1: write-var return_value <= s_80_0
        fn_state.return_value = s_80_0;
        // N s_80_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_81_0: const #55s : i
        let s_81_0: i128 = 55;
        // D s_81_1: read-var bottom_PAC_bit:i
        let s_81_1: i128 = fn_state.bottom_PAC_bit;
        // D s_81_2: cmp-ge s_81_1 s_81_0
        let s_81_2: bool = ((s_81_1) >= (s_81_0));
        // D s_81_3: write-var gs#15005 <= s_81_2
        fn_state.gs_15005 = s_81_2;
        // N s_81_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_82_0: const #55s : i
        let s_82_0: i128 = 55;
        // D s_82_1: read-var ptr:u64
        let s_82_1: u64 = fn_state.ptr;
        // D s_82_2: cast zx s_82_1 -> bv
        let s_82_2: Bits = Bits::new(s_82_1 as u128, 64u16);
        // C s_82_3: const #1u : u64
        let s_82_3: u64 = 1;
        // D s_82_4: bit-extract s_82_2 s_82_0 s_82_3
        let s_82_4: Bits = (Bits::new(
            ((s_82_2) >> (s_82_0)).value(),
            u16::try_from(s_82_3).unwrap(),
        ));
        // D s_82_5: cast reint s_82_4 -> u8
        let s_82_5: bool = ((s_82_4.value()) != 0);
        // C s_82_6: const #0s : i
        let s_82_6: i128 = 0;
        // C s_82_7: const #0u : u64
        let s_82_7: u64 = 0;
        // D s_82_8: cast zx s_82_5 -> u64
        let s_82_8: u64 = (s_82_5 as u64);
        // C s_82_9: const #1u : u64
        let s_82_9: u64 = 1;
        // D s_82_10: and s_82_8 s_82_9
        let s_82_10: u64 = ((s_82_8) & (s_82_9));
        // D s_82_11: cmp-eq s_82_10 s_82_9
        let s_82_11: bool = ((s_82_10) == (s_82_9));
        // D s_82_12: lsl s_82_8 s_82_6
        let s_82_12: u64 = s_82_8 << s_82_6;
        // D s_82_13: or s_82_7 s_82_12
        let s_82_13: u64 = ((s_82_7) | (s_82_12));
        // D s_82_14: cmpl s_82_12
        let s_82_14: u64 = !s_82_12;
        // D s_82_15: and s_82_7 s_82_14
        let s_82_15: u64 = ((s_82_7) & (s_82_14));
        // D s_82_16: select s_82_11 s_82_13 s_82_15
        let s_82_16: u64 = if s_82_11 { s_82_13 } else { s_82_15 };
        // D s_82_17: cast trunc s_82_16 -> u8
        let s_82_17: bool = ((s_82_16) != 0);
        // D s_82_18: write-var selbit <= s_82_17
        fn_state.selbit = s_82_17;
        // N s_82_19: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call ConstPACField(s_83_0)
        let s_83_1: bool = ConstPACField(state, tracer, s_83_0);
        // D s_83_2: write-var gs#14968 <= s_83_1
        fn_state.gs_14968 = s_83_1;
        // N s_83_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_84_0: const #55s : i
        let s_84_0: i128 = 55;
        // D s_84_1: read-var ptr:u64
        let s_84_1: u64 = fn_state.ptr;
        // D s_84_2: cast zx s_84_1 -> bv
        let s_84_2: Bits = Bits::new(s_84_1 as u128, 64u16);
        // C s_84_3: const #1u : u64
        let s_84_3: u64 = 1;
        // D s_84_4: bit-extract s_84_2 s_84_0 s_84_3
        let s_84_4: Bits = (Bits::new(
            ((s_84_2) >> (s_84_0)).value(),
            u16::try_from(s_84_3).unwrap(),
        ));
        // D s_84_5: cast reint s_84_4 -> u8
        let s_84_5: bool = ((s_84_4.value()) != 0);
        // C s_84_6: const #0s : i
        let s_84_6: i128 = 0;
        // C s_84_7: const #0u : u64
        let s_84_7: u64 = 0;
        // D s_84_8: cast zx s_84_5 -> u64
        let s_84_8: u64 = (s_84_5 as u64);
        // C s_84_9: const #1u : u64
        let s_84_9: u64 = 1;
        // D s_84_10: and s_84_8 s_84_9
        let s_84_10: u64 = ((s_84_8) & (s_84_9));
        // D s_84_11: cmp-eq s_84_10 s_84_9
        let s_84_11: bool = ((s_84_10) == (s_84_9));
        // D s_84_12: lsl s_84_8 s_84_6
        let s_84_12: u64 = s_84_8 << s_84_6;
        // D s_84_13: or s_84_7 s_84_12
        let s_84_13: u64 = ((s_84_7) | (s_84_12));
        // D s_84_14: cmpl s_84_12
        let s_84_14: u64 = !s_84_12;
        // D s_84_15: and s_84_7 s_84_14
        let s_84_15: u64 = ((s_84_7) & (s_84_14));
        // D s_84_16: select s_84_11 s_84_13 s_84_15
        let s_84_16: u64 = if s_84_11 { s_84_13 } else { s_84_15 };
        // D s_84_17: cast trunc s_84_16 -> u8
        let s_84_17: bool = ((s_84_16) != 0);
        // D s_84_18: write-var selbit <= s_84_17
        fn_state.selbit = s_84_17;
        // N s_84_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call S1TranslationRegime__1(s_85_0)
        let s_85_1: u8 = S1TranslationRegime__1(state, tracer, s_85_0);
        // S s_85_2: cast zx s_85_1 -> bv
        let s_85_2: Bits = Bits::new(s_85_1 as u128, 2u16);
        // C s_85_3: const #440u : u32
        let s_85_3: u32 = 440;
        // D s_85_4: read-reg s_85_3:u8
        let s_85_4: u8 = {
            let value = state.read_register::<u8>(s_85_3 as isize);
            tracer.read_register(s_85_3 as isize, value);
            value
        };
        // D s_85_5: cast zx s_85_4 -> bv
        let s_85_5: Bits = Bits::new(s_85_4 as u128, 2u16);
        // D s_85_6: cmp-eq s_85_2 s_85_5
        let s_85_6: bool = ((s_85_2) == (s_85_5));
        // N s_85_7: branch s_85_6 b126 b86
        if s_85_6 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call S1TranslationRegime__1(s_86_0)
        let s_86_1: u8 = S1TranslationRegime__1(state, tracer, s_86_0);
        // S s_86_2: cast zx s_86_1 -> bv
        let s_86_2: Bits = Bits::new(s_86_1 as u128, 2u16);
        // C s_86_3: const #432u : u32
        let s_86_3: u32 = 432;
        // D s_86_4: read-reg s_86_3:u8
        let s_86_4: u8 = {
            let value = state.read_register::<u8>(s_86_3 as isize);
            tracer.read_register(s_86_3 as isize, value);
            value
        };
        // D s_86_5: cast zx s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 2u16);
        // D s_86_6: cmp-eq s_86_2 s_86_5
        let s_86_6: bool = ((s_86_2) == (s_86_5));
        // D s_86_7: write-var gs#14977 <= s_86_6
        fn_state.gs_14977 = s_86_6;
        // N s_86_8: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_87_0: read-var gs#14977:u8
        let s_87_0: bool = fn_state.gs_14977;
        // N s_87_1: assert s_87_0
        let s_87_1: () = assert!(s_87_0);
        // C s_87_2: const #() : ()
        let s_87_2: () = ();
        // S s_87_3: call S1TranslationRegime__1(s_87_2)
        let s_87_3: u8 = S1TranslationRegime__1(state, tracer, s_87_2);
        // S s_87_4: cast zx s_87_3 -> bv
        let s_87_4: Bits = Bits::new(s_87_3 as u128, 2u16);
        // C s_87_5: const #440u : u32
        let s_87_5: u32 = 440;
        // D s_87_6: read-reg s_87_5:u8
        let s_87_6: u8 = {
            let value = state.read_register::<u8>(s_87_5 as isize);
            tracer.read_register(s_87_5 as isize, value);
            value
        };
        // D s_87_7: cast zx s_87_6 -> bv
        let s_87_7: Bits = Bits::new(s_87_6 as u128, 2u16);
        // D s_87_8: cmp-eq s_87_4 s_87_7
        let s_87_8: bool = ((s_87_4) == (s_87_7));
        // N s_87_9: branch s_87_8 b107 b88
        if s_87_8 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_88_0: read-var data:u8
        let s_88_0: bool = fn_state.data;
        // N s_88_1: branch s_88_0 b101 b89
        if s_88_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_89_0: const #12816u : u32
        let s_89_0: u32 = 12816;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_TCR_EL2_Type_TBI1(s_89_1)
        let s_89_2: bool = u_get_TCR_EL2_Type_TBI1(state, tracer, s_89_1);
        // D s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #1u : u8
        let s_89_4: bool = true;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // D s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // N s_89_7: branch s_89_6 b100 b90
        if s_89_6 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#14979 <= s_90_0
        fn_state.gs_14979 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_91_0: read-var gs#14979:u8
        let s_91_0: bool = fn_state.gs_14979;
        // N s_91_1: branch s_91_0 b99 b92
        if s_91_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_92_0: const #12816u : u32
        let s_92_0: u32 = 12816;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call _get_TCR_EL2_Type_TBI0(s_92_1)
        let s_92_2: bool = u_get_TCR_EL2_Type_TBI0(state, tracer, s_92_1);
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // C s_92_4: const #1u : u8
        let s_92_4: bool = true;
        // C s_92_5: cast zx s_92_4 -> bv
        let s_92_5: Bits = Bits::new(s_92_4 as u128, 1u16);
        // D s_92_6: cmp-eq s_92_3 s_92_5
        let s_92_6: bool = ((s_92_3) == (s_92_5));
        // N s_92_7: branch s_92_6 b98 b93
        if s_92_6 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#14980 <= s_93_0
        fn_state.gs_14980 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_94_0: read-var gs#14980:u8
        let s_94_0: bool = fn_state.gs_14980;
        // D s_94_1: write-var gs#14981 <= s_94_0
        fn_state.gs_14981 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_95_0: read-var gs#14981:u8
        let s_95_0: bool = fn_state.gs_14981;
        // N s_95_1: branch s_95_0 b97 b96
        if s_95_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_96_0: const #63s : i
        let s_96_0: i128 = 63;
        // D s_96_1: read-var ptr:u64
        let s_96_1: u64 = fn_state.ptr;
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 64u16);
        // C s_96_3: const #1u : u64
        let s_96_3: u64 = 1;
        // D s_96_4: bit-extract s_96_2 s_96_0 s_96_3
        let s_96_4: Bits = (Bits::new(
            ((s_96_2) >> (s_96_0)).value(),
            u16::try_from(s_96_3).unwrap(),
        ));
        // D s_96_5: cast reint s_96_4 -> u8
        let s_96_5: bool = ((s_96_4.value()) != 0);
        // C s_96_6: const #0s : i
        let s_96_6: i128 = 0;
        // C s_96_7: const #0u : u64
        let s_96_7: u64 = 0;
        // D s_96_8: cast zx s_96_5 -> u64
        let s_96_8: u64 = (s_96_5 as u64);
        // C s_96_9: const #1u : u64
        let s_96_9: u64 = 1;
        // D s_96_10: and s_96_8 s_96_9
        let s_96_10: u64 = ((s_96_8) & (s_96_9));
        // D s_96_11: cmp-eq s_96_10 s_96_9
        let s_96_11: bool = ((s_96_10) == (s_96_9));
        // D s_96_12: lsl s_96_8 s_96_6
        let s_96_12: u64 = s_96_8 << s_96_6;
        // D s_96_13: or s_96_7 s_96_12
        let s_96_13: u64 = ((s_96_7) | (s_96_12));
        // D s_96_14: cmpl s_96_12
        let s_96_14: u64 = !s_96_12;
        // D s_96_15: and s_96_7 s_96_14
        let s_96_15: u64 = ((s_96_7) & (s_96_14));
        // D s_96_16: select s_96_11 s_96_13 s_96_15
        let s_96_16: u64 = if s_96_11 { s_96_13 } else { s_96_15 };
        // D s_96_17: cast trunc s_96_16 -> u8
        let s_96_17: bool = ((s_96_16) != 0);
        // D s_96_18: write-var selbit <= s_96_17
        fn_state.selbit = s_96_17;
        // N s_96_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_97_0: const #55s : i
        let s_97_0: i128 = 55;
        // D s_97_1: read-var ptr:u64
        let s_97_1: u64 = fn_state.ptr;
        // D s_97_2: cast zx s_97_1 -> bv
        let s_97_2: Bits = Bits::new(s_97_1 as u128, 64u16);
        // C s_97_3: const #1u : u64
        let s_97_3: u64 = 1;
        // D s_97_4: bit-extract s_97_2 s_97_0 s_97_3
        let s_97_4: Bits = (Bits::new(
            ((s_97_2) >> (s_97_0)).value(),
            u16::try_from(s_97_3).unwrap(),
        ));
        // D s_97_5: cast reint s_97_4 -> u8
        let s_97_5: bool = ((s_97_4.value()) != 0);
        // C s_97_6: const #0s : i
        let s_97_6: i128 = 0;
        // C s_97_7: const #0u : u64
        let s_97_7: u64 = 0;
        // D s_97_8: cast zx s_97_5 -> u64
        let s_97_8: u64 = (s_97_5 as u64);
        // C s_97_9: const #1u : u64
        let s_97_9: u64 = 1;
        // D s_97_10: and s_97_8 s_97_9
        let s_97_10: u64 = ((s_97_8) & (s_97_9));
        // D s_97_11: cmp-eq s_97_10 s_97_9
        let s_97_11: bool = ((s_97_10) == (s_97_9));
        // D s_97_12: lsl s_97_8 s_97_6
        let s_97_12: u64 = s_97_8 << s_97_6;
        // D s_97_13: or s_97_7 s_97_12
        let s_97_13: u64 = ((s_97_7) | (s_97_12));
        // D s_97_14: cmpl s_97_12
        let s_97_14: u64 = !s_97_12;
        // D s_97_15: and s_97_7 s_97_14
        let s_97_15: u64 = ((s_97_7) & (s_97_14));
        // D s_97_16: select s_97_11 s_97_13 s_97_15
        let s_97_16: u64 = if s_97_11 { s_97_13 } else { s_97_15 };
        // D s_97_17: cast trunc s_97_16 -> u8
        let s_97_17: bool = ((s_97_16) != 0);
        // D s_97_18: write-var selbit <= s_97_17
        fn_state.selbit = s_97_17;
        // N s_97_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_98_0: const #12816u : u32
        let s_98_0: u32 = 12816;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call _get_TCR_EL2_Type_TBID0(s_98_1)
        let s_98_2: bool = u_get_TCR_EL2_Type_TBID0(state, tracer, s_98_1);
        // D s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // C s_98_4: const #0u : u8
        let s_98_4: bool = false;
        // C s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 1u16);
        // D s_98_6: cmp-eq s_98_3 s_98_5
        let s_98_6: bool = ((s_98_3) == (s_98_5));
        // D s_98_7: write-var gs#14980 <= s_98_6
        fn_state.gs_14980 = s_98_6;
        // N s_98_8: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#14981 <= s_99_0
        fn_state.gs_14981 = s_99_0;
        // N s_99_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_100_0: const #12816u : u32
        let s_100_0: u32 = 12816;
        // D s_100_1: read-reg s_100_0:struct
        let s_100_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call _get_TCR_EL2_Type_TBID1(s_100_1)
        let s_100_2: bool = u_get_TCR_EL2_Type_TBID1(state, tracer, s_100_1);
        // D s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // C s_100_4: const #0u : u8
        let s_100_4: bool = false;
        // C s_100_5: cast zx s_100_4 -> bv
        let s_100_5: Bits = Bits::new(s_100_4 as u128, 1u16);
        // D s_100_6: cmp-eq s_100_3 s_100_5
        let s_100_6: bool = ((s_100_3) == (s_100_5));
        // D s_100_7: write-var gs#14979 <= s_100_6
        fn_state.gs_14979 = s_100_6;
        // N s_100_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_101_0: const #12816u : u32
        let s_101_0: u32 = 12816;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call _get_TCR_EL2_Type_TBI1(s_101_1)
        let s_101_2: bool = u_get_TCR_EL2_Type_TBI1(state, tracer, s_101_1);
        // D s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // C s_101_4: const #1u : u8
        let s_101_4: bool = true;
        // C s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 1u16);
        // D s_101_6: cmp-eq s_101_3 s_101_5
        let s_101_6: bool = ((s_101_3) == (s_101_5));
        // N s_101_7: branch s_101_6 b106 b102
        if s_101_6 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_102_0: const #12816u : u32
        let s_102_0: u32 = 12816;
        // D s_102_1: read-reg s_102_0:struct
        let s_102_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: call _get_TCR_EL2_Type_TBI0(s_102_1)
        let s_102_2: bool = u_get_TCR_EL2_Type_TBI0(state, tracer, s_102_1);
        // D s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // C s_102_4: const #1u : u8
        let s_102_4: bool = true;
        // C s_102_5: cast zx s_102_4 -> bv
        let s_102_5: Bits = Bits::new(s_102_4 as u128, 1u16);
        // D s_102_6: cmp-eq s_102_3 s_102_5
        let s_102_6: bool = ((s_102_3) == (s_102_5));
        // D s_102_7: write-var gs#14986 <= s_102_6
        fn_state.gs_14986 = s_102_6;
        // N s_102_8: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_103_0: read-var gs#14986:u8
        let s_103_0: bool = fn_state.gs_14986;
        // N s_103_1: branch s_103_0 b105 b104
        if s_103_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_104_0: const #63s : i
        let s_104_0: i128 = 63;
        // D s_104_1: read-var ptr:u64
        let s_104_1: u64 = fn_state.ptr;
        // D s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 64u16);
        // C s_104_3: const #1u : u64
        let s_104_3: u64 = 1;
        // D s_104_4: bit-extract s_104_2 s_104_0 s_104_3
        let s_104_4: Bits = (Bits::new(
            ((s_104_2) >> (s_104_0)).value(),
            u16::try_from(s_104_3).unwrap(),
        ));
        // D s_104_5: cast reint s_104_4 -> u8
        let s_104_5: bool = ((s_104_4.value()) != 0);
        // C s_104_6: const #0s : i
        let s_104_6: i128 = 0;
        // C s_104_7: const #0u : u64
        let s_104_7: u64 = 0;
        // D s_104_8: cast zx s_104_5 -> u64
        let s_104_8: u64 = (s_104_5 as u64);
        // C s_104_9: const #1u : u64
        let s_104_9: u64 = 1;
        // D s_104_10: and s_104_8 s_104_9
        let s_104_10: u64 = ((s_104_8) & (s_104_9));
        // D s_104_11: cmp-eq s_104_10 s_104_9
        let s_104_11: bool = ((s_104_10) == (s_104_9));
        // D s_104_12: lsl s_104_8 s_104_6
        let s_104_12: u64 = s_104_8 << s_104_6;
        // D s_104_13: or s_104_7 s_104_12
        let s_104_13: u64 = ((s_104_7) | (s_104_12));
        // D s_104_14: cmpl s_104_12
        let s_104_14: u64 = !s_104_12;
        // D s_104_15: and s_104_7 s_104_14
        let s_104_15: u64 = ((s_104_7) & (s_104_14));
        // D s_104_16: select s_104_11 s_104_13 s_104_15
        let s_104_16: u64 = if s_104_11 { s_104_13 } else { s_104_15 };
        // D s_104_17: cast trunc s_104_16 -> u8
        let s_104_17: bool = ((s_104_16) != 0);
        // D s_104_18: write-var selbit <= s_104_17
        fn_state.selbit = s_104_17;
        // N s_104_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_105_0: const #55s : i
        let s_105_0: i128 = 55;
        // D s_105_1: read-var ptr:u64
        let s_105_1: u64 = fn_state.ptr;
        // D s_105_2: cast zx s_105_1 -> bv
        let s_105_2: Bits = Bits::new(s_105_1 as u128, 64u16);
        // C s_105_3: const #1u : u64
        let s_105_3: u64 = 1;
        // D s_105_4: bit-extract s_105_2 s_105_0 s_105_3
        let s_105_4: Bits = (Bits::new(
            ((s_105_2) >> (s_105_0)).value(),
            u16::try_from(s_105_3).unwrap(),
        ));
        // D s_105_5: cast reint s_105_4 -> u8
        let s_105_5: bool = ((s_105_4.value()) != 0);
        // C s_105_6: const #0s : i
        let s_105_6: i128 = 0;
        // C s_105_7: const #0u : u64
        let s_105_7: u64 = 0;
        // D s_105_8: cast zx s_105_5 -> u64
        let s_105_8: u64 = (s_105_5 as u64);
        // C s_105_9: const #1u : u64
        let s_105_9: u64 = 1;
        // D s_105_10: and s_105_8 s_105_9
        let s_105_10: u64 = ((s_105_8) & (s_105_9));
        // D s_105_11: cmp-eq s_105_10 s_105_9
        let s_105_11: bool = ((s_105_10) == (s_105_9));
        // D s_105_12: lsl s_105_8 s_105_6
        let s_105_12: u64 = s_105_8 << s_105_6;
        // D s_105_13: or s_105_7 s_105_12
        let s_105_13: u64 = ((s_105_7) | (s_105_12));
        // D s_105_14: cmpl s_105_12
        let s_105_14: u64 = !s_105_12;
        // D s_105_15: and s_105_7 s_105_14
        let s_105_15: u64 = ((s_105_7) & (s_105_14));
        // D s_105_16: select s_105_11 s_105_13 s_105_15
        let s_105_16: u64 = if s_105_11 { s_105_13 } else { s_105_15 };
        // D s_105_17: cast trunc s_105_16 -> u8
        let s_105_17: bool = ((s_105_16) != 0);
        // D s_105_18: write-var selbit <= s_105_17
        fn_state.selbit = s_105_17;
        // N s_105_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#14986 <= s_106_0
        fn_state.gs_14986 = s_106_0;
        // N s_106_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_107_0: read-var data:u8
        let s_107_0: bool = fn_state.data;
        // N s_107_1: branch s_107_0 b120 b108
        if s_107_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_108_0: const #22392u : u32
        let s_108_0: u32 = 22392;
        // D s_108_1: read-reg s_108_0:struct
        let s_108_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call _get_TCR_EL1_Type_TBI1(s_108_1)
        let s_108_2: bool = u_get_TCR_EL1_Type_TBI1(state, tracer, s_108_1);
        // D s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // C s_108_4: const #1u : u8
        let s_108_4: bool = true;
        // C s_108_5: cast zx s_108_4 -> bv
        let s_108_5: Bits = Bits::new(s_108_4 as u128, 1u16);
        // D s_108_6: cmp-eq s_108_3 s_108_5
        let s_108_6: bool = ((s_108_3) == (s_108_5));
        // N s_108_7: branch s_108_6 b119 b109
        if s_108_6 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#14991 <= s_109_0
        fn_state.gs_14991 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_110_0: read-var gs#14991:u8
        let s_110_0: bool = fn_state.gs_14991;
        // N s_110_1: branch s_110_0 b118 b111
        if s_110_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_111_0: const #22392u : u32
        let s_111_0: u32 = 22392;
        // D s_111_1: read-reg s_111_0:struct
        let s_111_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // D s_111_2: call _get_TCR_EL1_Type_TBI0(s_111_1)
        let s_111_2: bool = u_get_TCR_EL1_Type_TBI0(state, tracer, s_111_1);
        // D s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // C s_111_4: const #1u : u8
        let s_111_4: bool = true;
        // C s_111_5: cast zx s_111_4 -> bv
        let s_111_5: Bits = Bits::new(s_111_4 as u128, 1u16);
        // D s_111_6: cmp-eq s_111_3 s_111_5
        let s_111_6: bool = ((s_111_3) == (s_111_5));
        // N s_111_7: branch s_111_6 b117 b112
        if s_111_6 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#14992 <= s_112_0
        fn_state.gs_14992 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_113_0: read-var gs#14992:u8
        let s_113_0: bool = fn_state.gs_14992;
        // D s_113_1: write-var gs#14993 <= s_113_0
        fn_state.gs_14993 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_114_0: read-var gs#14993:u8
        let s_114_0: bool = fn_state.gs_14993;
        // N s_114_1: branch s_114_0 b116 b115
        if s_114_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_115_0: const #63s : i
        let s_115_0: i128 = 63;
        // D s_115_1: read-var ptr:u64
        let s_115_1: u64 = fn_state.ptr;
        // D s_115_2: cast zx s_115_1 -> bv
        let s_115_2: Bits = Bits::new(s_115_1 as u128, 64u16);
        // C s_115_3: const #1u : u64
        let s_115_3: u64 = 1;
        // D s_115_4: bit-extract s_115_2 s_115_0 s_115_3
        let s_115_4: Bits = (Bits::new(
            ((s_115_2) >> (s_115_0)).value(),
            u16::try_from(s_115_3).unwrap(),
        ));
        // D s_115_5: cast reint s_115_4 -> u8
        let s_115_5: bool = ((s_115_4.value()) != 0);
        // C s_115_6: const #0s : i
        let s_115_6: i128 = 0;
        // C s_115_7: const #0u : u64
        let s_115_7: u64 = 0;
        // D s_115_8: cast zx s_115_5 -> u64
        let s_115_8: u64 = (s_115_5 as u64);
        // C s_115_9: const #1u : u64
        let s_115_9: u64 = 1;
        // D s_115_10: and s_115_8 s_115_9
        let s_115_10: u64 = ((s_115_8) & (s_115_9));
        // D s_115_11: cmp-eq s_115_10 s_115_9
        let s_115_11: bool = ((s_115_10) == (s_115_9));
        // D s_115_12: lsl s_115_8 s_115_6
        let s_115_12: u64 = s_115_8 << s_115_6;
        // D s_115_13: or s_115_7 s_115_12
        let s_115_13: u64 = ((s_115_7) | (s_115_12));
        // D s_115_14: cmpl s_115_12
        let s_115_14: u64 = !s_115_12;
        // D s_115_15: and s_115_7 s_115_14
        let s_115_15: u64 = ((s_115_7) & (s_115_14));
        // D s_115_16: select s_115_11 s_115_13 s_115_15
        let s_115_16: u64 = if s_115_11 { s_115_13 } else { s_115_15 };
        // D s_115_17: cast trunc s_115_16 -> u8
        let s_115_17: bool = ((s_115_16) != 0);
        // D s_115_18: write-var selbit <= s_115_17
        fn_state.selbit = s_115_17;
        // N s_115_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_116_0: const #55s : i
        let s_116_0: i128 = 55;
        // D s_116_1: read-var ptr:u64
        let s_116_1: u64 = fn_state.ptr;
        // D s_116_2: cast zx s_116_1 -> bv
        let s_116_2: Bits = Bits::new(s_116_1 as u128, 64u16);
        // C s_116_3: const #1u : u64
        let s_116_3: u64 = 1;
        // D s_116_4: bit-extract s_116_2 s_116_0 s_116_3
        let s_116_4: Bits = (Bits::new(
            ((s_116_2) >> (s_116_0)).value(),
            u16::try_from(s_116_3).unwrap(),
        ));
        // D s_116_5: cast reint s_116_4 -> u8
        let s_116_5: bool = ((s_116_4.value()) != 0);
        // C s_116_6: const #0s : i
        let s_116_6: i128 = 0;
        // C s_116_7: const #0u : u64
        let s_116_7: u64 = 0;
        // D s_116_8: cast zx s_116_5 -> u64
        let s_116_8: u64 = (s_116_5 as u64);
        // C s_116_9: const #1u : u64
        let s_116_9: u64 = 1;
        // D s_116_10: and s_116_8 s_116_9
        let s_116_10: u64 = ((s_116_8) & (s_116_9));
        // D s_116_11: cmp-eq s_116_10 s_116_9
        let s_116_11: bool = ((s_116_10) == (s_116_9));
        // D s_116_12: lsl s_116_8 s_116_6
        let s_116_12: u64 = s_116_8 << s_116_6;
        // D s_116_13: or s_116_7 s_116_12
        let s_116_13: u64 = ((s_116_7) | (s_116_12));
        // D s_116_14: cmpl s_116_12
        let s_116_14: u64 = !s_116_12;
        // D s_116_15: and s_116_7 s_116_14
        let s_116_15: u64 = ((s_116_7) & (s_116_14));
        // D s_116_16: select s_116_11 s_116_13 s_116_15
        let s_116_16: u64 = if s_116_11 { s_116_13 } else { s_116_15 };
        // D s_116_17: cast trunc s_116_16 -> u8
        let s_116_17: bool = ((s_116_16) != 0);
        // D s_116_18: write-var selbit <= s_116_17
        fn_state.selbit = s_116_17;
        // N s_116_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_117_0: const #22392u : u32
        let s_117_0: u32 = 22392;
        // D s_117_1: read-reg s_117_0:struct
        let s_117_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call _get_TCR_EL1_Type_TBID0(s_117_1)
        let s_117_2: bool = u_get_TCR_EL1_Type_TBID0(state, tracer, s_117_1);
        // D s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // C s_117_4: const #0u : u8
        let s_117_4: bool = false;
        // C s_117_5: cast zx s_117_4 -> bv
        let s_117_5: Bits = Bits::new(s_117_4 as u128, 1u16);
        // D s_117_6: cmp-eq s_117_3 s_117_5
        let s_117_6: bool = ((s_117_3) == (s_117_5));
        // D s_117_7: write-var gs#14992 <= s_117_6
        fn_state.gs_14992 = s_117_6;
        // N s_117_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_118_0: const #1u : u8
        let s_118_0: bool = true;
        // D s_118_1: write-var gs#14993 <= s_118_0
        fn_state.gs_14993 = s_118_0;
        // N s_118_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_119_0: const #22392u : u32
        let s_119_0: u32 = 22392;
        // D s_119_1: read-reg s_119_0:struct
        let s_119_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call _get_TCR_EL1_Type_TBID1(s_119_1)
        let s_119_2: bool = u_get_TCR_EL1_Type_TBID1(state, tracer, s_119_1);
        // D s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // C s_119_4: const #0u : u8
        let s_119_4: bool = false;
        // C s_119_5: cast zx s_119_4 -> bv
        let s_119_5: Bits = Bits::new(s_119_4 as u128, 1u16);
        // D s_119_6: cmp-eq s_119_3 s_119_5
        let s_119_6: bool = ((s_119_3) == (s_119_5));
        // D s_119_7: write-var gs#14991 <= s_119_6
        fn_state.gs_14991 = s_119_6;
        // N s_119_8: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_120_0: const #22392u : u32
        let s_120_0: u32 = 22392;
        // D s_120_1: read-reg s_120_0:struct
        let s_120_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call _get_TCR_EL1_Type_TBI1(s_120_1)
        let s_120_2: bool = u_get_TCR_EL1_Type_TBI1(state, tracer, s_120_1);
        // D s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // C s_120_4: const #1u : u8
        let s_120_4: bool = true;
        // C s_120_5: cast zx s_120_4 -> bv
        let s_120_5: Bits = Bits::new(s_120_4 as u128, 1u16);
        // D s_120_6: cmp-eq s_120_3 s_120_5
        let s_120_6: bool = ((s_120_3) == (s_120_5));
        // N s_120_7: branch s_120_6 b125 b121
        if s_120_6 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_121_0: const #22392u : u32
        let s_121_0: u32 = 22392;
        // D s_121_1: read-reg s_121_0:struct
        let s_121_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: call _get_TCR_EL1_Type_TBI0(s_121_1)
        let s_121_2: bool = u_get_TCR_EL1_Type_TBI0(state, tracer, s_121_1);
        // D s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // C s_121_4: const #1u : u8
        let s_121_4: bool = true;
        // C s_121_5: cast zx s_121_4 -> bv
        let s_121_5: Bits = Bits::new(s_121_4 as u128, 1u16);
        // D s_121_6: cmp-eq s_121_3 s_121_5
        let s_121_6: bool = ((s_121_3) == (s_121_5));
        // D s_121_7: write-var gs#14998 <= s_121_6
        fn_state.gs_14998 = s_121_6;
        // N s_121_8: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_122_0: read-var gs#14998:u8
        let s_122_0: bool = fn_state.gs_14998;
        // N s_122_1: branch s_122_0 b124 b123
        if s_122_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_123_0: const #63s : i
        let s_123_0: i128 = 63;
        // D s_123_1: read-var ptr:u64
        let s_123_1: u64 = fn_state.ptr;
        // D s_123_2: cast zx s_123_1 -> bv
        let s_123_2: Bits = Bits::new(s_123_1 as u128, 64u16);
        // C s_123_3: const #1u : u64
        let s_123_3: u64 = 1;
        // D s_123_4: bit-extract s_123_2 s_123_0 s_123_3
        let s_123_4: Bits = (Bits::new(
            ((s_123_2) >> (s_123_0)).value(),
            u16::try_from(s_123_3).unwrap(),
        ));
        // D s_123_5: cast reint s_123_4 -> u8
        let s_123_5: bool = ((s_123_4.value()) != 0);
        // C s_123_6: const #0s : i
        let s_123_6: i128 = 0;
        // C s_123_7: const #0u : u64
        let s_123_7: u64 = 0;
        // D s_123_8: cast zx s_123_5 -> u64
        let s_123_8: u64 = (s_123_5 as u64);
        // C s_123_9: const #1u : u64
        let s_123_9: u64 = 1;
        // D s_123_10: and s_123_8 s_123_9
        let s_123_10: u64 = ((s_123_8) & (s_123_9));
        // D s_123_11: cmp-eq s_123_10 s_123_9
        let s_123_11: bool = ((s_123_10) == (s_123_9));
        // D s_123_12: lsl s_123_8 s_123_6
        let s_123_12: u64 = s_123_8 << s_123_6;
        // D s_123_13: or s_123_7 s_123_12
        let s_123_13: u64 = ((s_123_7) | (s_123_12));
        // D s_123_14: cmpl s_123_12
        let s_123_14: u64 = !s_123_12;
        // D s_123_15: and s_123_7 s_123_14
        let s_123_15: u64 = ((s_123_7) & (s_123_14));
        // D s_123_16: select s_123_11 s_123_13 s_123_15
        let s_123_16: u64 = if s_123_11 { s_123_13 } else { s_123_15 };
        // D s_123_17: cast trunc s_123_16 -> u8
        let s_123_17: bool = ((s_123_16) != 0);
        // D s_123_18: write-var selbit <= s_123_17
        fn_state.selbit = s_123_17;
        // N s_123_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_124_0: const #55s : i
        let s_124_0: i128 = 55;
        // D s_124_1: read-var ptr:u64
        let s_124_1: u64 = fn_state.ptr;
        // D s_124_2: cast zx s_124_1 -> bv
        let s_124_2: Bits = Bits::new(s_124_1 as u128, 64u16);
        // C s_124_3: const #1u : u64
        let s_124_3: u64 = 1;
        // D s_124_4: bit-extract s_124_2 s_124_0 s_124_3
        let s_124_4: Bits = (Bits::new(
            ((s_124_2) >> (s_124_0)).value(),
            u16::try_from(s_124_3).unwrap(),
        ));
        // D s_124_5: cast reint s_124_4 -> u8
        let s_124_5: bool = ((s_124_4.value()) != 0);
        // C s_124_6: const #0s : i
        let s_124_6: i128 = 0;
        // C s_124_7: const #0u : u64
        let s_124_7: u64 = 0;
        // D s_124_8: cast zx s_124_5 -> u64
        let s_124_8: u64 = (s_124_5 as u64);
        // C s_124_9: const #1u : u64
        let s_124_9: u64 = 1;
        // D s_124_10: and s_124_8 s_124_9
        let s_124_10: u64 = ((s_124_8) & (s_124_9));
        // D s_124_11: cmp-eq s_124_10 s_124_9
        let s_124_11: bool = ((s_124_10) == (s_124_9));
        // D s_124_12: lsl s_124_8 s_124_6
        let s_124_12: u64 = s_124_8 << s_124_6;
        // D s_124_13: or s_124_7 s_124_12
        let s_124_13: u64 = ((s_124_7) | (s_124_12));
        // D s_124_14: cmpl s_124_12
        let s_124_14: u64 = !s_124_12;
        // D s_124_15: and s_124_7 s_124_14
        let s_124_15: u64 = ((s_124_7) & (s_124_14));
        // D s_124_16: select s_124_11 s_124_13 s_124_15
        let s_124_16: u64 = if s_124_11 { s_124_13 } else { s_124_15 };
        // D s_124_17: cast trunc s_124_16 -> u8
        let s_124_17: bool = ((s_124_16) != 0);
        // D s_124_18: write-var selbit <= s_124_17
        fn_state.selbit = s_124_17;
        // N s_124_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_125_0: const #1u : u8
        let s_125_0: bool = true;
        // D s_125_1: write-var gs#14998 <= s_125_0
        fn_state.gs_14998 = s_125_0;
        // N s_125_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#14977 <= s_126_0
        fn_state.gs_14977 = s_126_0;
        // N s_126_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_127_0: const #55s : i64
        let s_127_0: i64 = 55;
        // D s_127_1: write-var ga#11022 <= s_127_0
        fn_state.ga_11022 = s_127_0;
        // N s_127_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
