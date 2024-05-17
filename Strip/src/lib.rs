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
use subrange_subrange_concat::*;
use CalculateBottomPACBit::*;
use EffectiveTBI::*;
use u__id::*;
use EffectiveMTX::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn Strip<T: Tracer>(state: &mut State, tracer: &T, A: u64, data: bool) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        tbi: bool,
        gs_14695: bool,
        gs_14752: bool,
        gs_14728: bool,
        gs_14723: bool,
        mtx: bool,
        return_value: u64,
        extfield: u64,
        gs_14708: bool,
        gs_14703: bool,
        gs_14747: bool,
        bottom_PAC_bit: i128,
        original_ptr: u64,
        A: u64,
        data: bool,
    }
    let fn_state = FunctionState {
        A,
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
        // D s_0_4: read-var A:u64
        let s_0_4: u64 = fn_state.A;
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
        // D s_0_15: read-var A:u64
        let s_0_15: u64 = fn_state.A;
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
        // D s_0_23: read-var A:u64
        let s_0_23: u64 = fn_state.A;
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
        // D s_0_43: read-var A:u64
        let s_0_43: u64 = fn_state.A;
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
        // N s_0_66: branch s_0_65 b29 b1
        if s_0_65 {
            return block_29(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#14695 <= s_1_0
        fn_state.gs_14695 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var gs#14695:u8
        let s_2_0: bool = fn_state.gs_14695;
        // N s_2_1: branch s_2_0 b28 b3
        if s_2_0 {
            return block_28(state, tracer, fn_state);
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
    ) -> u64 {
        // D s_4_0: read-var mtx:u8
        let s_4_0: bool = fn_state.mtx;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
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
        // N s_5_8: branch s_5_7 b13 b6
        if s_5_7 {
            return block_13(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#14703 <= s_6_0
        fn_state.gs_14703 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#14703:u8
        let s_7_0: bool = fn_state.gs_14703;
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
        // N s_7_12: branch s_7_11 b12 b8
        if s_7_11 {
            return block_12(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#14708 <= s_8_0
        fn_state.gs_14708 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#14708:u8
        let s_9_0: bool = fn_state.gs_14708;
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
        // D s_9_20: read-var A:u64
        let s_9_20: u64 = fn_state.A;
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
        // D s_10_0: read-var original_ptr:u64
        let s_10_0: u64 = fn_state.original_ptr;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var return_value:u64
        let s_11_0: u64 = fn_state.return_value;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var bottom_PAC_bit:i
        let s_12_0: i128 = fn_state.bottom_PAC_bit;
        // D s_12_1: call __id(s_12_0)
        let s_12_1: i128 = u__id(state, tracer, s_12_0);
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // C s_12_3: const #1s : i
        let s_12_3: i128 = 1;
        // D s_12_4: cast zx s_12_2 -> i
        let s_12_4: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_5: sub s_12_4 s_12_3
        let s_12_5: i128 = ((s_12_4) - (s_12_3));
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // C s_12_7: const #64s : i
        let s_12_7: i128 = 64;
        // D s_12_8: cast zx s_12_6 -> i
        let s_12_8: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_9: cmp-lt s_12_8 s_12_7
        let s_12_9: bool = ((s_12_8) < (s_12_7));
        // D s_12_10: write-var gs#14708 <= s_12_9
        fn_state.gs_14708 = s_12_9;
        // N s_12_11: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var bottom_PAC_bit:i
        let s_13_0: i128 = fn_state.bottom_PAC_bit;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #64s : i
        let s_13_2: i128 = 64;
        // D s_13_3: sub s_13_2 s_13_1
        let s_13_3: i128 = ((s_13_2) - (s_13_1));
        // C s_13_4: const #1s : i
        let s_13_4: i128 = 1;
        // D s_13_5: sub s_13_3 s_13_4
        let s_13_5: i128 = ((s_13_3) - (s_13_4));
        // C s_13_6: const #64s : i
        let s_13_6: i128 = 64;
        // D s_13_7: cmp-lt s_13_5 s_13_6
        let s_13_7: bool = ((s_13_5) < (s_13_6));
        // D s_13_8: write-var gs#14703 <= s_13_7
        fn_state.gs_14703 = s_13_7;
        // N s_13_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var bottom_PAC_bit:i
        let s_14_0: i128 = fn_state.bottom_PAC_bit;
        // D s_14_1: call __id(s_14_0)
        let s_14_1: i128 = u__id(state, tracer, s_14_0);
        // C s_14_2: const #56s : i
        let s_14_2: i128 = 56;
        // D s_14_3: sub s_14_2 s_14_1
        let s_14_3: i128 = ((s_14_2) - (s_14_1));
        // C s_14_4: const #1s : i
        let s_14_4: i128 = 1;
        // D s_14_5: sub s_14_3 s_14_4
        let s_14_5: i128 = ((s_14_3) - (s_14_4));
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // D s_14_7: cmp-le s_14_6 s_14_5
        let s_14_7: bool = ((s_14_6) <= (s_14_5));
        // N s_14_8: branch s_14_7 b20 b15
        if s_14_7 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#14723 <= s_15_0
        fn_state.gs_14723 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var gs#14723:u8
        let s_16_0: bool = fn_state.gs_14723;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var bottom_PAC_bit:i
        let s_16_2: i128 = fn_state.bottom_PAC_bit;
        // D s_16_3: call __id(s_16_2)
        let s_16_3: i128 = u__id(state, tracer, s_16_2);
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // C s_16_5: const #1s : i
        let s_16_5: i128 = 1;
        // D s_16_6: cast zx s_16_4 -> i
        let s_16_6: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_7: sub s_16_6 s_16_5
        let s_16_7: i128 = ((s_16_6) - (s_16_5));
        // D s_16_8: cast reint s_16_7 -> i64
        let s_16_8: i64 = (s_16_7 as i64);
        // C s_16_9: const #0s : i
        let s_16_9: i128 = 0;
        // D s_16_10: cast zx s_16_8 -> i
        let s_16_10: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_11: cmp-le s_16_9 s_16_10
        let s_16_11: bool = ((s_16_9) <= (s_16_10));
        // N s_16_12: branch s_16_11 b19 b17
        if s_16_11 {
            return block_19(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#14728 <= s_17_0
        fn_state.gs_14728 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_18_0: read-var gs#14728:u8
        let s_18_0: bool = fn_state.gs_14728;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // C s_18_2: const #60s : i
        let s_18_2: i128 = 60;
        // D s_18_3: read-var extfield:u64
        let s_18_3: u64 = fn_state.extfield;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 64u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #3s : i
        let s_18_7: i128 = 3;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_2 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: u8 = (s_18_9.value() as u8);
        // C s_18_11: const #56s : i
        let s_18_11: i128 = 56;
        // D s_18_12: read-var A:u64
        let s_18_12: u64 = fn_state.A;
        // D s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 64u16);
        // C s_18_14: const #1s : i64
        let s_18_14: i64 = 1;
        // C s_18_15: cast zx s_18_14 -> i
        let s_18_15: i128 = (i128::try_from(s_18_14).unwrap());
        // C s_18_16: const #3s : i
        let s_18_16: i128 = 3;
        // C s_18_17: add s_18_16 s_18_15
        let s_18_17: i128 = (s_18_16 + s_18_15);
        // D s_18_18: bit-extract s_18_13 s_18_11 s_18_17
        let s_18_18: Bits = (Bits::new(
            ((s_18_13) >> (s_18_11)).value(),
            u16::try_from(s_18_17).unwrap(),
        ));
        // D s_18_19: cast reint s_18_18 -> u8
        let s_18_19: u8 = (s_18_18.value() as u8);
        // D s_18_20: cast zx s_18_10 -> bv
        let s_18_20: Bits = Bits::new(s_18_10 as u128, 4u16);
        // D s_18_21: cast zx s_18_19 -> bv
        let s_18_21: Bits = Bits::new(s_18_19 as u128, 4u16);
        // D s_18_22: cast reint s_18_20 -> u128
        let s_18_22: u128 = (s_18_20.value() as u128);
        // D s_18_23: size-of s_18_20
        let s_18_23: u16 = s_18_20.length();
        // D s_18_24: cast reint s_18_21 -> u128
        let s_18_24: u128 = (s_18_21.value() as u128);
        // D s_18_25: size-of s_18_21
        let s_18_25: u16 = s_18_21.length();
        // D s_18_26: lsl s_18_22 s_18_25
        let s_18_26: u128 = s_18_22 << s_18_25;
        // D s_18_27: or s_18_26 s_18_24
        let s_18_27: u128 = ((s_18_26) | (s_18_24));
        // D s_18_28: add s_18_23 s_18_25
        let s_18_28: u16 = (s_18_23 + s_18_25);
        // D s_18_29: create-bits s_18_27 s_18_28
        let s_18_29: Bits = Bits::new(s_18_27, s_18_28);
        // D s_18_30: cast reint s_18_29 -> u8
        let s_18_30: u8 = (s_18_29.value() as u8);
        // C s_18_31: const #56s : i
        let s_18_31: i128 = 56;
        // D s_18_32: read-var bottom_PAC_bit:i
        let s_18_32: i128 = fn_state.bottom_PAC_bit;
        // D s_18_33: sub s_18_31 s_18_32
        let s_18_33: i128 = ((s_18_31) - (s_18_32));
        // D s_18_34: cast reint s_18_33 -> i64
        let s_18_34: i64 = (s_18_33 as i64);
        // C s_18_35: const #1s : i
        let s_18_35: i128 = 1;
        // D s_18_36: cast zx s_18_34 -> i
        let s_18_36: i128 = (i128::try_from(s_18_34).unwrap());
        // D s_18_37: sub s_18_36 s_18_35
        let s_18_37: i128 = ((s_18_36) - (s_18_35));
        // D s_18_38: cast reint s_18_37 -> i64
        let s_18_38: i64 = (s_18_37 as i64);
        // C s_18_39: const #1s : i
        let s_18_39: i128 = 1;
        // D s_18_40: read-var bottom_PAC_bit:i
        let s_18_40: i128 = fn_state.bottom_PAC_bit;
        // D s_18_41: sub s_18_40 s_18_39
        let s_18_41: i128 = ((s_18_40) - (s_18_39));
        // D s_18_42: cast reint s_18_41 -> i64
        let s_18_42: i64 = (s_18_41 as i64);
        // C s_18_43: const #56s : i
        let s_18_43: i128 = 56;
        // C s_18_44: const #0s : i
        let s_18_44: i128 = 0;
        // C s_18_45: const #0s : i
        let s_18_45: i128 = 0;
        // D s_18_46: read-var extfield:u64
        let s_18_46: u64 = fn_state.extfield;
        // D s_18_47: cast zx s_18_46 -> bv
        let s_18_47: Bits = Bits::new(s_18_46 as u128, 64u16);
        // D s_18_48: cast zx s_18_38 -> i
        let s_18_48: i128 = (i128::try_from(s_18_38).unwrap());
        // D s_18_49: read-var A:u64
        let s_18_49: u64 = fn_state.A;
        // D s_18_50: cast zx s_18_49 -> bv
        let s_18_50: Bits = Bits::new(s_18_49 as u128, 64u16);
        // D s_18_51: cast zx s_18_42 -> i
        let s_18_51: i128 = (i128::try_from(s_18_42).unwrap());
        // D s_18_52: call subrange_subrange_concat(s_18_43, s_18_47, s_18_48, s_18_44, s_18_50, s_18_51, s_18_45)
        let s_18_52: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_18_43,
            s_18_47,
            s_18_48,
            s_18_44,
            s_18_50,
            s_18_51,
            s_18_45,
        );
        // D s_18_53: cast reint s_18_52 -> u56
        let s_18_53: u64 = (s_18_52.value() as u64);
        // D s_18_54: cast zx s_18_30 -> bv
        let s_18_54: Bits = Bits::new(s_18_30 as u128, 8u16);
        // D s_18_55: cast zx s_18_53 -> bv
        let s_18_55: Bits = Bits::new(s_18_53 as u128, 56u16);
        // D s_18_56: cast reint s_18_54 -> u128
        let s_18_56: u128 = (s_18_54.value() as u128);
        // D s_18_57: size-of s_18_54
        let s_18_57: u16 = s_18_54.length();
        // D s_18_58: cast reint s_18_55 -> u128
        let s_18_58: u128 = (s_18_55.value() as u128);
        // D s_18_59: size-of s_18_55
        let s_18_59: u16 = s_18_55.length();
        // D s_18_60: lsl s_18_56 s_18_59
        let s_18_60: u128 = s_18_56 << s_18_59;
        // D s_18_61: or s_18_60 s_18_58
        let s_18_61: u128 = ((s_18_60) | (s_18_58));
        // D s_18_62: add s_18_57 s_18_59
        let s_18_62: u16 = (s_18_57 + s_18_59);
        // D s_18_63: create-bits s_18_61 s_18_62
        let s_18_63: Bits = Bits::new(s_18_61, s_18_62);
        // D s_18_64: cast reint s_18_63 -> u64
        let s_18_64: u64 = (s_18_63.value() as u64);
        // D s_18_65: write-var original_ptr <= s_18_64
        fn_state.original_ptr = s_18_64;
        // N s_18_66: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var bottom_PAC_bit:i
        let s_19_0: i128 = fn_state.bottom_PAC_bit;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // C s_19_3: const #1s : i
        let s_19_3: i128 = 1;
        // D s_19_4: cast zx s_19_2 -> i
        let s_19_4: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_5: sub s_19_4 s_19_3
        let s_19_5: i128 = ((s_19_4) - (s_19_3));
        // D s_19_6: cast reint s_19_5 -> i64
        let s_19_6: i64 = (s_19_5 as i64);
        // C s_19_7: const #64s : i
        let s_19_7: i128 = 64;
        // D s_19_8: cast zx s_19_6 -> i
        let s_19_8: i128 = (i128::try_from(s_19_6).unwrap());
        // D s_19_9: cmp-lt s_19_8 s_19_7
        let s_19_9: bool = ((s_19_8) < (s_19_7));
        // D s_19_10: write-var gs#14728 <= s_19_9
        fn_state.gs_14728 = s_19_9;
        // N s_19_11: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_20_0: read-var bottom_PAC_bit:i
        let s_20_0: i128 = fn_state.bottom_PAC_bit;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // C s_20_2: const #56s : i
        let s_20_2: i128 = 56;
        // D s_20_3: sub s_20_2 s_20_1
        let s_20_3: i128 = ((s_20_2) - (s_20_1));
        // C s_20_4: const #1s : i
        let s_20_4: i128 = 1;
        // D s_20_5: sub s_20_3 s_20_4
        let s_20_5: i128 = ((s_20_3) - (s_20_4));
        // C s_20_6: const #64s : i
        let s_20_6: i128 = 64;
        // D s_20_7: cmp-lt s_20_5 s_20_6
        let s_20_7: bool = ((s_20_5) < (s_20_6));
        // D s_20_8: write-var gs#14723 <= s_20_7
        fn_state.gs_14723 = s_20_7;
        // N s_20_9: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var bottom_PAC_bit:i
        let s_21_0: i128 = fn_state.bottom_PAC_bit;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #56s : i
        let s_21_2: i128 = 56;
        // D s_21_3: sub s_21_2 s_21_1
        let s_21_3: i128 = ((s_21_2) - (s_21_1));
        // C s_21_4: const #1s : i
        let s_21_4: i128 = 1;
        // D s_21_5: sub s_21_3 s_21_4
        let s_21_5: i128 = ((s_21_3) - (s_21_4));
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // D s_21_7: cmp-le s_21_6 s_21_5
        let s_21_7: bool = ((s_21_6) <= (s_21_5));
        // N s_21_8: branch s_21_7 b27 b22
        if s_21_7 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#14747 <= s_22_0
        fn_state.gs_14747 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var gs#14747:u8
        let s_23_0: bool = fn_state.gs_14747;
        // N s_23_1: assert s_23_0
        let s_23_1: () = assert!(s_23_0);
        // D s_23_2: read-var bottom_PAC_bit:i
        let s_23_2: i128 = fn_state.bottom_PAC_bit;
        // D s_23_3: call __id(s_23_2)
        let s_23_3: i128 = u__id(state, tracer, s_23_2);
        // D s_23_4: cast reint s_23_3 -> i64
        let s_23_4: i64 = (s_23_3 as i64);
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // D s_23_6: cast zx s_23_4 -> i
        let s_23_6: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_7: sub s_23_6 s_23_5
        let s_23_7: i128 = ((s_23_6) - (s_23_5));
        // D s_23_8: cast reint s_23_7 -> i64
        let s_23_8: i64 = (s_23_7 as i64);
        // C s_23_9: const #0s : i
        let s_23_9: i128 = 0;
        // D s_23_10: cast zx s_23_8 -> i
        let s_23_10: i128 = (i128::try_from(s_23_8).unwrap());
        // D s_23_11: cmp-le s_23_9 s_23_10
        let s_23_11: bool = ((s_23_9) <= (s_23_10));
        // N s_23_12: branch s_23_11 b26 b24
        if s_23_11 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#14752 <= s_24_0
        fn_state.gs_14752 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var gs#14752:u8
        let s_25_0: bool = fn_state.gs_14752;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // C s_25_2: const #56s : i
        let s_25_2: i128 = 56;
        // D s_25_3: read-var A:u64
        let s_25_3: u64 = fn_state.A;
        // D s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 64u16);
        // C s_25_5: const #1s : i64
        let s_25_5: i64 = 1;
        // C s_25_6: cast zx s_25_5 -> i
        let s_25_6: i128 = (i128::try_from(s_25_5).unwrap());
        // C s_25_7: const #7s : i
        let s_25_7: i128 = 7;
        // C s_25_8: add s_25_7 s_25_6
        let s_25_8: i128 = (s_25_7 + s_25_6);
        // D s_25_9: bit-extract s_25_4 s_25_2 s_25_8
        let s_25_9: Bits = (Bits::new(
            ((s_25_4) >> (s_25_2)).value(),
            u16::try_from(s_25_8).unwrap(),
        ));
        // D s_25_10: cast reint s_25_9 -> u8
        let s_25_10: u8 = (s_25_9.value() as u8);
        // C s_25_11: const #56s : i
        let s_25_11: i128 = 56;
        // D s_25_12: read-var bottom_PAC_bit:i
        let s_25_12: i128 = fn_state.bottom_PAC_bit;
        // D s_25_13: sub s_25_11 s_25_12
        let s_25_13: i128 = ((s_25_11) - (s_25_12));
        // D s_25_14: cast reint s_25_13 -> i64
        let s_25_14: i64 = (s_25_13 as i64);
        // C s_25_15: const #1s : i
        let s_25_15: i128 = 1;
        // D s_25_16: cast zx s_25_14 -> i
        let s_25_16: i128 = (i128::try_from(s_25_14).unwrap());
        // D s_25_17: sub s_25_16 s_25_15
        let s_25_17: i128 = ((s_25_16) - (s_25_15));
        // D s_25_18: cast reint s_25_17 -> i64
        let s_25_18: i64 = (s_25_17 as i64);
        // C s_25_19: const #1s : i
        let s_25_19: i128 = 1;
        // D s_25_20: read-var bottom_PAC_bit:i
        let s_25_20: i128 = fn_state.bottom_PAC_bit;
        // D s_25_21: sub s_25_20 s_25_19
        let s_25_21: i128 = ((s_25_20) - (s_25_19));
        // D s_25_22: cast reint s_25_21 -> i64
        let s_25_22: i64 = (s_25_21 as i64);
        // C s_25_23: const #56s : i
        let s_25_23: i128 = 56;
        // C s_25_24: const #0s : i
        let s_25_24: i128 = 0;
        // C s_25_25: const #0s : i
        let s_25_25: i128 = 0;
        // D s_25_26: read-var extfield:u64
        let s_25_26: u64 = fn_state.extfield;
        // D s_25_27: cast zx s_25_26 -> bv
        let s_25_27: Bits = Bits::new(s_25_26 as u128, 64u16);
        // D s_25_28: cast zx s_25_18 -> i
        let s_25_28: i128 = (i128::try_from(s_25_18).unwrap());
        // D s_25_29: read-var A:u64
        let s_25_29: u64 = fn_state.A;
        // D s_25_30: cast zx s_25_29 -> bv
        let s_25_30: Bits = Bits::new(s_25_29 as u128, 64u16);
        // D s_25_31: cast zx s_25_22 -> i
        let s_25_31: i128 = (i128::try_from(s_25_22).unwrap());
        // D s_25_32: call subrange_subrange_concat(s_25_23, s_25_27, s_25_28, s_25_24, s_25_30, s_25_31, s_25_25)
        let s_25_32: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_25_23,
            s_25_27,
            s_25_28,
            s_25_24,
            s_25_30,
            s_25_31,
            s_25_25,
        );
        // D s_25_33: cast reint s_25_32 -> u56
        let s_25_33: u64 = (s_25_32.value() as u64);
        // D s_25_34: cast zx s_25_10 -> bv
        let s_25_34: Bits = Bits::new(s_25_10 as u128, 8u16);
        // D s_25_35: cast zx s_25_33 -> bv
        let s_25_35: Bits = Bits::new(s_25_33 as u128, 56u16);
        // D s_25_36: cast reint s_25_34 -> u128
        let s_25_36: u128 = (s_25_34.value() as u128);
        // D s_25_37: size-of s_25_34
        let s_25_37: u16 = s_25_34.length();
        // D s_25_38: cast reint s_25_35 -> u128
        let s_25_38: u128 = (s_25_35.value() as u128);
        // D s_25_39: size-of s_25_35
        let s_25_39: u16 = s_25_35.length();
        // D s_25_40: lsl s_25_36 s_25_39
        let s_25_40: u128 = s_25_36 << s_25_39;
        // D s_25_41: or s_25_40 s_25_38
        let s_25_41: u128 = ((s_25_40) | (s_25_38));
        // D s_25_42: add s_25_37 s_25_39
        let s_25_42: u16 = (s_25_37 + s_25_39);
        // D s_25_43: create-bits s_25_41 s_25_42
        let s_25_43: Bits = Bits::new(s_25_41, s_25_42);
        // D s_25_44: cast reint s_25_43 -> u64
        let s_25_44: u64 = (s_25_43.value() as u64);
        // D s_25_45: write-var original_ptr <= s_25_44
        fn_state.original_ptr = s_25_44;
        // N s_25_46: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_26_0: read-var bottom_PAC_bit:i
        let s_26_0: i128 = fn_state.bottom_PAC_bit;
        // D s_26_1: call __id(s_26_0)
        let s_26_1: i128 = u__id(state, tracer, s_26_0);
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // C s_26_3: const #1s : i
        let s_26_3: i128 = 1;
        // D s_26_4: cast zx s_26_2 -> i
        let s_26_4: i128 = (i128::try_from(s_26_2).unwrap());
        // D s_26_5: sub s_26_4 s_26_3
        let s_26_5: i128 = ((s_26_4) - (s_26_3));
        // D s_26_6: cast reint s_26_5 -> i64
        let s_26_6: i64 = (s_26_5 as i64);
        // C s_26_7: const #64s : i
        let s_26_7: i128 = 64;
        // D s_26_8: cast zx s_26_6 -> i
        let s_26_8: i128 = (i128::try_from(s_26_6).unwrap());
        // D s_26_9: cmp-lt s_26_8 s_26_7
        let s_26_9: bool = ((s_26_8) < (s_26_7));
        // D s_26_10: write-var gs#14752 <= s_26_9
        fn_state.gs_14752 = s_26_9;
        // N s_26_11: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var bottom_PAC_bit:i
        let s_27_0: i128 = fn_state.bottom_PAC_bit;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #56s : i
        let s_27_2: i128 = 56;
        // D s_27_3: sub s_27_2 s_27_1
        let s_27_3: i128 = ((s_27_2) - (s_27_1));
        // C s_27_4: const #1s : i
        let s_27_4: i128 = 1;
        // D s_27_5: sub s_27_3 s_27_4
        let s_27_5: i128 = ((s_27_3) - (s_27_4));
        // C s_27_6: const #64s : i
        let s_27_6: i128 = 64;
        // D s_27_7: cmp-lt s_27_5 s_27_6
        let s_27_7: bool = ((s_27_5) < (s_27_6));
        // D s_27_8: write-var gs#14747 <= s_27_7
        fn_state.gs_14747 = s_27_7;
        // N s_27_9: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_28_0: read-var A:u64
        let s_28_0: u64 = fn_state.A;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_29_0: const #55s : i
        let s_29_0: i128 = 55;
        // D s_29_1: read-var bottom_PAC_bit:i
        let s_29_1: i128 = fn_state.bottom_PAC_bit;
        // D s_29_2: cmp-ge s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) >= (s_29_0));
        // D s_29_3: write-var gs#14695 <= s_29_2
        fn_state.gs_14695 = s_29_2;
        // N s_29_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
