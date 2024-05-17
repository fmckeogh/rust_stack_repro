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
use Unreachable::*;
use Zeros::*;
use HasLargeAddress::*;
use Ones::*;
use integer_subrange::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn TLBIRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    Xt: u64,
) -> ProductType37abbcb1894e7c56 {
    #[derive(Default)]
    struct FunctionState {
        scale: i64,
        return_value: ProductType37abbcb1894e7c56,
        start_address: u64,
        num: i64,
        tg: u8,
        tg_bits: i64,
        end_address_name: u64,
        regime: u32,
        Xt: u64,
    }
    let fn_state = FunctionState {
        regime,
        Xt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u64
        let s_0_2: u64 = (s_0_1.value() as u64);
        // D s_0_3: write-var start_address <= s_0_2
        fn_state.start_address = s_0_2;
        // C s_0_4: const #64s : i
        let s_0_4: i128 = 64;
        // S s_0_5: call Zeros(s_0_4)
        let s_0_5: Bits = Zeros(state, tracer, s_0_4);
        // S s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: write-var end_address_name <= s_0_6
        fn_state.end_address_name = s_0_6;
        // C s_0_8: const #46s : i
        let s_0_8: i128 = 46;
        // D s_0_9: read-var Xt:u64
        let s_0_9: u64 = fn_state.Xt;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 64u16);
        // C s_0_11: const #1s : i64
        let s_0_11: i64 = 1;
        // C s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // C s_0_13: const #1s : i
        let s_0_13: i128 = 1;
        // C s_0_14: add s_0_13 s_0_12
        let s_0_14: i128 = (s_0_13 + s_0_12);
        // D s_0_15: bit-extract s_0_10 s_0_8 s_0_14
        let s_0_15: Bits = (Bits::new(
            ((s_0_10) >> (s_0_8)).value(),
            u16::try_from(s_0_14).unwrap(),
        ));
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // D s_0_17: write-var tg <= s_0_16
        fn_state.tg = s_0_16;
        // C s_0_18: const #44s : i
        let s_0_18: i128 = 44;
        // D s_0_19: read-var Xt:u64
        let s_0_19: u64 = fn_state.Xt;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 64u16);
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
        // D s_0_30: write-var scale <= s_0_29
        fn_state.scale = s_0_29;
        // C s_0_31: const #39s : i
        let s_0_31: i128 = 39;
        // D s_0_32: read-var Xt:u64
        let s_0_32: u64 = fn_state.Xt;
        // D s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 64u16);
        // C s_0_34: const #1s : i64
        let s_0_34: i64 = 1;
        // C s_0_35: cast zx s_0_34 -> i
        let s_0_35: i128 = (i128::try_from(s_0_34).unwrap());
        // C s_0_36: const #4s : i
        let s_0_36: i128 = 4;
        // C s_0_37: add s_0_36 s_0_35
        let s_0_37: i128 = (s_0_36 + s_0_35);
        // D s_0_38: bit-extract s_0_33 s_0_31 s_0_37
        let s_0_38: Bits = (Bits::new(
            ((s_0_33) >> (s_0_31)).value(),
            u16::try_from(s_0_37).unwrap(),
        ));
        // D s_0_39: cast reint s_0_38 -> u8
        let s_0_39: u8 = (s_0_38.value() as u8);
        // D s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 5u16);
        // D s_0_41: cast zx s_0_40 -> i
        let s_0_41: i128 = (s_0_40.value() as i128);
        // D s_0_42: cast reint s_0_41 -> i64
        let s_0_42: i64 = (s_0_41 as i64);
        // D s_0_43: write-var num <= s_0_42
        fn_state.num = s_0_42;
        // C s_0_44: const #12s : i64
        let s_0_44: i64 = 12;
        // D s_0_45: write-var tg_bits <= s_0_44
        fn_state.tg_bits = s_0_44;
        // D s_0_46: read-var tg:u8
        let s_0_46: u8 = fn_state.tg;
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // C s_0_48: const #0u : u8
        let s_0_48: u8 = 0;
        // C s_0_49: cast zx s_0_48 -> bv
        let s_0_49: Bits = Bits::new(s_0_48 as u128, 2u16);
        // D s_0_50: cmp-eq s_0_47 s_0_49
        let s_0_50: bool = ((s_0_47) == (s_0_49));
        // N s_0_51: branch s_0_50 b17 b1
        if s_0_50 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_1_0: read-var tg:u8
        let s_1_0: u8 = fn_state.tg;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #1u : u8
        let s_1_2: u8 = 1;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // D s_1_5: not s_1_4
        let s_1_5: bool = !s_1_4;
        // N s_1_6: branch s_1_5 b10 b2
        if s_1_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_2_0: const #12s : i64
        let s_2_0: i64 = 12;
        // D s_2_1: write-var tg_bits <= s_2_0
        fn_state.tg_bits = s_2_0;
        // D s_2_2: read-var regime:u32
        let s_2_2: u32 = fn_state.regime;
        // D s_2_3: call HasLargeAddress(s_2_2)
        let s_2_3: bool = HasLargeAddress(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b9 b3
        if s_2_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var Xt:u64
        let s_3_1: u64 = fn_state.Xt;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #36s : i
        let s_3_5: i128 = 36;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u37
        let s_3_8: u64 = (s_3_7.value() as u64);
        // C s_3_9: const #12s : i
        let s_3_9: i128 = 12;
        // D s_3_10: read-var start_address:u64
        let s_3_10: u64 = fn_state.start_address;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 64u16);
        // D s_3_12: cast zx s_3_8 -> bv
        let s_3_12: Bits = Bits::new(s_3_8 as u128, 37u16);
        // C s_3_13: const #36s : i
        let s_3_13: i128 = 36;
        // C s_3_14: const #1u : u64
        let s_3_14: u64 = 1;
        // C s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 64u16);
        // C s_3_16: lsl s_3_15 s_3_13
        let s_3_16: Bits = s_3_15 << s_3_13;
        // C s_3_17: sub s_3_16 s_3_15
        let s_3_17: Bits = ((s_3_16) - (s_3_15));
        // D s_3_18: and s_3_12 s_3_17
        let s_3_18: Bits = ((s_3_12) & (s_3_17));
        // D s_3_19: lsl s_3_18 s_3_9
        let s_3_19: Bits = s_3_18 << s_3_9;
        // C s_3_20: lsl s_3_17 s_3_9
        let s_3_20: Bits = s_3_17 << s_3_9;
        // C s_3_21: cmpl s_3_20
        let s_3_21: Bits = !s_3_20;
        // D s_3_22: and s_3_11 s_3_21
        let s_3_22: Bits = ((s_3_11) & (s_3_21));
        // D s_3_23: or s_3_22 s_3_19
        let s_3_23: Bits = ((s_3_22) | (s_3_19));
        // D s_3_24: cast reint s_3_23 -> u64
        let s_3_24: u64 = (s_3_23.value() as u64);
        // D s_3_25: write-var start_address <= s_3_24
        fn_state.start_address = s_3_24;
        // C s_3_26: const #36s : i
        let s_3_26: i128 = 36;
        // D s_3_27: read-var Xt:u64
        let s_3_27: u64 = fn_state.Xt;
        // D s_3_28: cast zx s_3_27 -> bv
        let s_3_28: Bits = Bits::new(s_3_27 as u128, 64u16);
        // C s_3_29: const #1u : u64
        let s_3_29: u64 = 1;
        // D s_3_30: bit-extract s_3_28 s_3_26 s_3_29
        let s_3_30: Bits = (Bits::new(
            ((s_3_28) >> (s_3_26)).value(),
            u16::try_from(s_3_29).unwrap(),
        ));
        // D s_3_31: cast reint s_3_30 -> u8
        let s_3_31: bool = ((s_3_30.value()) != 0);
        // C s_3_32: const #0s : i
        let s_3_32: i128 = 0;
        // C s_3_33: const #0u : u64
        let s_3_33: u64 = 0;
        // D s_3_34: cast zx s_3_31 -> u64
        let s_3_34: u64 = (s_3_31 as u64);
        // C s_3_35: const #1u : u64
        let s_3_35: u64 = 1;
        // D s_3_36: and s_3_34 s_3_35
        let s_3_36: u64 = ((s_3_34) & (s_3_35));
        // D s_3_37: cmp-eq s_3_36 s_3_35
        let s_3_37: bool = ((s_3_36) == (s_3_35));
        // D s_3_38: lsl s_3_34 s_3_32
        let s_3_38: u64 = s_3_34 << s_3_32;
        // D s_3_39: or s_3_33 s_3_38
        let s_3_39: u64 = ((s_3_33) | (s_3_38));
        // D s_3_40: cmpl s_3_38
        let s_3_40: u64 = !s_3_38;
        // D s_3_41: and s_3_33 s_3_40
        let s_3_41: u64 = ((s_3_33) & (s_3_40));
        // D s_3_42: select s_3_37 s_3_39 s_3_41
        let s_3_42: u64 = if s_3_37 { s_3_39 } else { s_3_41 };
        // D s_3_43: cast trunc s_3_42 -> u8
        let s_3_43: bool = ((s_3_42) != 0);
        // D s_3_44: cast zx s_3_43 -> bv
        let s_3_44: Bits = Bits::new(s_3_43 as u128, 1u16);
        // C s_3_45: const #15u : u64
        let s_3_45: u64 = 15;
        // D s_3_46: call replicate_bits_borealis_internal(s_3_44, s_3_45)
        let s_3_46: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_3_44,
            s_3_45,
        );
        // D s_3_47: cast reint s_3_46 -> u15
        let s_3_47: u16 = (s_3_46.value() as u16);
        // C s_3_48: const #49s : i
        let s_3_48: i128 = 49;
        // D s_3_49: read-var start_address:u64
        let s_3_49: u64 = fn_state.start_address;
        // D s_3_50: cast zx s_3_49 -> bv
        let s_3_50: Bits = Bits::new(s_3_49 as u128, 64u16);
        // D s_3_51: cast zx s_3_47 -> bv
        let s_3_51: Bits = Bits::new(s_3_47 as u128, 15u16);
        // C s_3_52: const #14s : i
        let s_3_52: i128 = 14;
        // C s_3_53: const #1u : u64
        let s_3_53: u64 = 1;
        // C s_3_54: cast zx s_3_53 -> bv
        let s_3_54: Bits = Bits::new(s_3_53 as u128, 64u16);
        // C s_3_55: lsl s_3_54 s_3_52
        let s_3_55: Bits = s_3_54 << s_3_52;
        // C s_3_56: sub s_3_55 s_3_54
        let s_3_56: Bits = ((s_3_55) - (s_3_54));
        // D s_3_57: and s_3_51 s_3_56
        let s_3_57: Bits = ((s_3_51) & (s_3_56));
        // D s_3_58: lsl s_3_57 s_3_48
        let s_3_58: Bits = s_3_57 << s_3_48;
        // C s_3_59: lsl s_3_56 s_3_48
        let s_3_59: Bits = s_3_56 << s_3_48;
        // C s_3_60: cmpl s_3_59
        let s_3_60: Bits = !s_3_59;
        // D s_3_61: and s_3_50 s_3_60
        let s_3_61: Bits = ((s_3_50) & (s_3_60));
        // D s_3_62: or s_3_61 s_3_58
        let s_3_62: Bits = ((s_3_61) | (s_3_58));
        // D s_3_63: cast reint s_3_62 -> u64
        let s_3_63: u64 = (s_3_62.value() as u64);
        // D s_3_64: write-var start_address <= s_3_63
        fn_state.start_address = s_3_63;
        // N s_3_65: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_4_0: read-var tg_bits:i64
        let s_4_0: i64 = fn_state.tg_bits;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // D s_4_2: read-var num:i64
        let s_4_2: i64 = fn_state.num;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: add s_4_3 s_4_1
        let s_4_4: i128 = (s_4_3 + s_4_1);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #5s : i
        let s_4_6: i128 = 5;
        // D s_4_7: read-var scale:i64
        let s_4_7: i64 = fn_state.scale;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: mul s_4_6 s_4_8
        let s_4_9: i128 = ((s_4_6) * (s_4_8));
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // C s_4_11: const #1s : i
        let s_4_11: i128 = 1;
        // D s_4_12: cast zx s_4_10 -> i
        let s_4_12: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_13: add s_4_12 s_4_11
        let s_4_13: i128 = (s_4_12 + s_4_11);
        // D s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: cast zx s_4_0 -> i
        let s_4_16: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_17: add s_4_15 s_4_16
        let s_4_17: i128 = (s_4_15 + s_4_16);
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: cast zx s_4_5 -> i
        let s_4_19: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_20: cast zx s_4_18 -> i
        let s_4_20: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_21: lsl s_4_19 s_4_20
        let s_4_21: i128 = s_4_19 << s_4_20;
        // C s_4_22: const #63s : i
        let s_4_22: i128 = 63;
        // C s_4_23: const #0s : i
        let s_4_23: i128 = 0;
        // D s_4_24: call integer_subrange(s_4_21, s_4_22, s_4_23)
        let s_4_24: Bits = integer_subrange(state, tracer, s_4_21, s_4_22, s_4_23);
        // D s_4_25: cast reint s_4_24 -> u64
        let s_4_25: u64 = (s_4_24.value() as u64);
        // D s_4_26: read-var start_address:u64
        let s_4_26: u64 = fn_state.start_address;
        // D s_4_27: cast zx s_4_26 -> bv
        let s_4_27: Bits = Bits::new(s_4_26 as u128, 64u16);
        // D s_4_28: cast zx s_4_25 -> bv
        let s_4_28: Bits = Bits::new(s_4_25 as u128, 64u16);
        // D s_4_29: add s_4_27 s_4_28
        let s_4_29: Bits = (s_4_27 + s_4_28);
        // D s_4_30: cast reint s_4_29 -> u64
        let s_4_30: u64 = (s_4_29.value() as u64);
        // D s_4_31: write-var end_address_name <= s_4_30
        fn_state.end_address_name = s_4_30;
        // C s_4_32: const #52s : i
        let s_4_32: i128 = 52;
        // D s_4_33: read-var end_address_name:u64
        let s_4_33: u64 = fn_state.end_address_name;
        // D s_4_34: cast zx s_4_33 -> bv
        let s_4_34: Bits = Bits::new(s_4_33 as u128, 64u16);
        // C s_4_35: const #1u : u64
        let s_4_35: u64 = 1;
        // D s_4_36: bit-extract s_4_34 s_4_32 s_4_35
        let s_4_36: Bits = (Bits::new(
            ((s_4_34) >> (s_4_32)).value(),
            u16::try_from(s_4_35).unwrap(),
        ));
        // D s_4_37: cast reint s_4_36 -> u8
        let s_4_37: bool = ((s_4_36.value()) != 0);
        // C s_4_38: const #0s : i
        let s_4_38: i128 = 0;
        // C s_4_39: const #0u : u64
        let s_4_39: u64 = 0;
        // D s_4_40: cast zx s_4_37 -> u64
        let s_4_40: u64 = (s_4_37 as u64);
        // C s_4_41: const #1u : u64
        let s_4_41: u64 = 1;
        // D s_4_42: and s_4_40 s_4_41
        let s_4_42: u64 = ((s_4_40) & (s_4_41));
        // D s_4_43: cmp-eq s_4_42 s_4_41
        let s_4_43: bool = ((s_4_42) == (s_4_41));
        // D s_4_44: lsl s_4_40 s_4_38
        let s_4_44: u64 = s_4_40 << s_4_38;
        // D s_4_45: or s_4_39 s_4_44
        let s_4_45: u64 = ((s_4_39) | (s_4_44));
        // D s_4_46: cmpl s_4_44
        let s_4_46: u64 = !s_4_44;
        // D s_4_47: and s_4_39 s_4_46
        let s_4_47: u64 = ((s_4_39) & (s_4_46));
        // D s_4_48: select s_4_43 s_4_45 s_4_47
        let s_4_48: u64 = if s_4_43 { s_4_45 } else { s_4_47 };
        // D s_4_49: cast trunc s_4_48 -> u8
        let s_4_49: bool = ((s_4_48) != 0);
        // C s_4_50: const #52s : i
        let s_4_50: i128 = 52;
        // D s_4_51: read-var start_address:u64
        let s_4_51: u64 = fn_state.start_address;
        // D s_4_52: cast zx s_4_51 -> bv
        let s_4_52: Bits = Bits::new(s_4_51 as u128, 64u16);
        // C s_4_53: const #1u : u64
        let s_4_53: u64 = 1;
        // D s_4_54: bit-extract s_4_52 s_4_50 s_4_53
        let s_4_54: Bits = (Bits::new(
            ((s_4_52) >> (s_4_50)).value(),
            u16::try_from(s_4_53).unwrap(),
        ));
        // D s_4_55: cast reint s_4_54 -> u8
        let s_4_55: bool = ((s_4_54.value()) != 0);
        // C s_4_56: const #0s : i
        let s_4_56: i128 = 0;
        // C s_4_57: const #0u : u64
        let s_4_57: u64 = 0;
        // D s_4_58: cast zx s_4_55 -> u64
        let s_4_58: u64 = (s_4_55 as u64);
        // C s_4_59: const #1u : u64
        let s_4_59: u64 = 1;
        // D s_4_60: and s_4_58 s_4_59
        let s_4_60: u64 = ((s_4_58) & (s_4_59));
        // D s_4_61: cmp-eq s_4_60 s_4_59
        let s_4_61: bool = ((s_4_60) == (s_4_59));
        // D s_4_62: lsl s_4_58 s_4_56
        let s_4_62: u64 = s_4_58 << s_4_56;
        // D s_4_63: or s_4_57 s_4_62
        let s_4_63: u64 = ((s_4_57) | (s_4_62));
        // D s_4_64: cmpl s_4_62
        let s_4_64: u64 = !s_4_62;
        // D s_4_65: and s_4_57 s_4_64
        let s_4_65: u64 = ((s_4_57) & (s_4_64));
        // D s_4_66: select s_4_61 s_4_63 s_4_65
        let s_4_66: u64 = if s_4_61 { s_4_63 } else { s_4_65 };
        // D s_4_67: cast trunc s_4_66 -> u8
        let s_4_67: bool = ((s_4_66) != 0);
        // D s_4_68: cast zx s_4_49 -> bv
        let s_4_68: Bits = Bits::new(s_4_49 as u128, 1u16);
        // D s_4_69: cast zx s_4_67 -> bv
        let s_4_69: Bits = Bits::new(s_4_67 as u128, 1u16);
        // D s_4_70: cmp-ne s_4_68 s_4_69
        let s_4_70: bool = ((s_4_68) != (s_4_69));
        // N s_4_71: branch s_4_70 b8 b5
        if s_4_70 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: read-var tg:u8
        let s_6_1: u8 = fn_state.tg;
        // D s_6_2: read-var start_address:u64
        let s_6_2: u64 = fn_state.start_address;
        // D s_6_3: read-var end_address_name:u64
        let s_6_3: u64 = fn_state.end_address_name;
        // D s_6_4: create-product struct = ["s_6_0", "s_6_1", "s_6_2", "s_6_3"]
        let s_6_4: ProductType37abbcb1894e7c56 = ProductType37abbcb1894e7c56 {
            _0: s_6_0,
            _1: s_6_1,
            _2: s_6_2,
            _3: s_6_3,
        };
        // D s_6_5: write-var return_value <= s_6_4
        fn_state.return_value = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_7_0: read-var return_value:struct
        let s_7_0: ProductType37abbcb1894e7c56 = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_8_0: const #52s : i
        let s_8_0: i128 = 52;
        // D s_8_1: read-var start_address:u64
        let s_8_1: u64 = fn_state.start_address;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
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
        // C s_8_18: const #12s : i64
        let s_8_18: i64 = 12;
        // D s_8_19: cast zx s_8_17 -> bv
        let s_8_19: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_20: cast zx s_8_18 -> i
        let s_8_20: i128 = (i128::try_from(s_8_18).unwrap());
        // C s_8_21: cast reint s_8_20 -> u64
        let s_8_21: u64 = (s_8_20 as u64);
        // D s_8_22: call replicate_bits_borealis_internal(s_8_19, s_8_21)
        let s_8_22: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_8_19,
            s_8_21,
        );
        // D s_8_23: cast reint s_8_22 -> u12
        let s_8_23: u16 = (s_8_22.value() as u16);
        // C s_8_24: const #52s : i
        let s_8_24: i128 = 52;
        // S s_8_25: call Ones(s_8_24)
        let s_8_25: Bits = Ones(state, tracer, s_8_24);
        // S s_8_26: cast reint s_8_25 -> u52
        let s_8_26: u64 = (s_8_25.value() as u64);
        // D s_8_27: cast zx s_8_23 -> bv
        let s_8_27: Bits = Bits::new(s_8_23 as u128, 12u16);
        // S s_8_28: cast zx s_8_26 -> bv
        let s_8_28: Bits = Bits::new(s_8_26 as u128, 52u16);
        // D s_8_29: cast reint s_8_27 -> u128
        let s_8_29: u128 = (s_8_27.value() as u128);
        // D s_8_30: size-of s_8_27
        let s_8_30: u16 = s_8_27.length();
        // S s_8_31: cast reint s_8_28 -> u128
        let s_8_31: u128 = (s_8_28.value() as u128);
        // D s_8_32: size-of s_8_28
        let s_8_32: u16 = s_8_28.length();
        // D s_8_33: lsl s_8_29 s_8_32
        let s_8_33: u128 = s_8_29 << s_8_32;
        // D s_8_34: or s_8_33 s_8_31
        let s_8_34: u128 = ((s_8_33) | (s_8_31));
        // D s_8_35: add s_8_30 s_8_32
        let s_8_35: u16 = (s_8_30 + s_8_32);
        // D s_8_36: create-bits s_8_34 s_8_35
        let s_8_36: Bits = Bits::new(s_8_34, s_8_35);
        // D s_8_37: cast reint s_8_36 -> u64
        let s_8_37: u64 = (s_8_36.value() as u64);
        // D s_8_38: write-var end_address_name <= s_8_37
        fn_state.end_address_name = s_8_37;
        // N s_8_39: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var Xt:u64
        let s_9_1: u64 = fn_state.Xt;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #36s : i
        let s_9_5: i128 = 36;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_0 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u37
        let s_9_8: u64 = (s_9_7.value() as u64);
        // C s_9_9: const #16s : i
        let s_9_9: i128 = 16;
        // D s_9_10: read-var start_address:u64
        let s_9_10: u64 = fn_state.start_address;
        // D s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 64u16);
        // D s_9_12: cast zx s_9_8 -> bv
        let s_9_12: Bits = Bits::new(s_9_8 as u128, 37u16);
        // C s_9_13: const #36s : i
        let s_9_13: i128 = 36;
        // C s_9_14: const #1u : u64
        let s_9_14: u64 = 1;
        // C s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 64u16);
        // C s_9_16: lsl s_9_15 s_9_13
        let s_9_16: Bits = s_9_15 << s_9_13;
        // C s_9_17: sub s_9_16 s_9_15
        let s_9_17: Bits = ((s_9_16) - (s_9_15));
        // D s_9_18: and s_9_12 s_9_17
        let s_9_18: Bits = ((s_9_12) & (s_9_17));
        // D s_9_19: lsl s_9_18 s_9_9
        let s_9_19: Bits = s_9_18 << s_9_9;
        // C s_9_20: lsl s_9_17 s_9_9
        let s_9_20: Bits = s_9_17 << s_9_9;
        // C s_9_21: cmpl s_9_20
        let s_9_21: Bits = !s_9_20;
        // D s_9_22: and s_9_11 s_9_21
        let s_9_22: Bits = ((s_9_11) & (s_9_21));
        // D s_9_23: or s_9_22 s_9_19
        let s_9_23: Bits = ((s_9_22) | (s_9_19));
        // D s_9_24: cast reint s_9_23 -> u64
        let s_9_24: u64 = (s_9_23.value() as u64);
        // D s_9_25: write-var start_address <= s_9_24
        fn_state.start_address = s_9_24;
        // C s_9_26: const #36s : i
        let s_9_26: i128 = 36;
        // D s_9_27: read-var Xt:u64
        let s_9_27: u64 = fn_state.Xt;
        // D s_9_28: cast zx s_9_27 -> bv
        let s_9_28: Bits = Bits::new(s_9_27 as u128, 64u16);
        // C s_9_29: const #1u : u64
        let s_9_29: u64 = 1;
        // D s_9_30: bit-extract s_9_28 s_9_26 s_9_29
        let s_9_30: Bits = (Bits::new(
            ((s_9_28) >> (s_9_26)).value(),
            u16::try_from(s_9_29).unwrap(),
        ));
        // D s_9_31: cast reint s_9_30 -> u8
        let s_9_31: bool = ((s_9_30.value()) != 0);
        // C s_9_32: const #0s : i
        let s_9_32: i128 = 0;
        // C s_9_33: const #0u : u64
        let s_9_33: u64 = 0;
        // D s_9_34: cast zx s_9_31 -> u64
        let s_9_34: u64 = (s_9_31 as u64);
        // C s_9_35: const #1u : u64
        let s_9_35: u64 = 1;
        // D s_9_36: and s_9_34 s_9_35
        let s_9_36: u64 = ((s_9_34) & (s_9_35));
        // D s_9_37: cmp-eq s_9_36 s_9_35
        let s_9_37: bool = ((s_9_36) == (s_9_35));
        // D s_9_38: lsl s_9_34 s_9_32
        let s_9_38: u64 = s_9_34 << s_9_32;
        // D s_9_39: or s_9_33 s_9_38
        let s_9_39: u64 = ((s_9_33) | (s_9_38));
        // D s_9_40: cmpl s_9_38
        let s_9_40: u64 = !s_9_38;
        // D s_9_41: and s_9_33 s_9_40
        let s_9_41: u64 = ((s_9_33) & (s_9_40));
        // D s_9_42: select s_9_37 s_9_39 s_9_41
        let s_9_42: u64 = if s_9_37 { s_9_39 } else { s_9_41 };
        // D s_9_43: cast trunc s_9_42 -> u8
        let s_9_43: bool = ((s_9_42) != 0);
        // D s_9_44: cast zx s_9_43 -> bv
        let s_9_44: Bits = Bits::new(s_9_43 as u128, 1u16);
        // C s_9_45: const #11u : u64
        let s_9_45: u64 = 11;
        // D s_9_46: call replicate_bits_borealis_internal(s_9_44, s_9_45)
        let s_9_46: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_9_44,
            s_9_45,
        );
        // D s_9_47: cast reint s_9_46 -> u11
        let s_9_47: u16 = (s_9_46.value() as u16);
        // C s_9_48: const #53s : i
        let s_9_48: i128 = 53;
        // D s_9_49: read-var start_address:u64
        let s_9_49: u64 = fn_state.start_address;
        // D s_9_50: cast zx s_9_49 -> bv
        let s_9_50: Bits = Bits::new(s_9_49 as u128, 64u16);
        // D s_9_51: cast zx s_9_47 -> bv
        let s_9_51: Bits = Bits::new(s_9_47 as u128, 11u16);
        // C s_9_52: const #10s : i
        let s_9_52: i128 = 10;
        // C s_9_53: const #1u : u64
        let s_9_53: u64 = 1;
        // C s_9_54: cast zx s_9_53 -> bv
        let s_9_54: Bits = Bits::new(s_9_53 as u128, 64u16);
        // C s_9_55: lsl s_9_54 s_9_52
        let s_9_55: Bits = s_9_54 << s_9_52;
        // C s_9_56: sub s_9_55 s_9_54
        let s_9_56: Bits = ((s_9_55) - (s_9_54));
        // D s_9_57: and s_9_51 s_9_56
        let s_9_57: Bits = ((s_9_51) & (s_9_56));
        // D s_9_58: lsl s_9_57 s_9_48
        let s_9_58: Bits = s_9_57 << s_9_48;
        // C s_9_59: lsl s_9_56 s_9_48
        let s_9_59: Bits = s_9_56 << s_9_48;
        // C s_9_60: cmpl s_9_59
        let s_9_60: Bits = !s_9_59;
        // D s_9_61: and s_9_50 s_9_60
        let s_9_61: Bits = ((s_9_50) & (s_9_60));
        // D s_9_62: or s_9_61 s_9_58
        let s_9_62: Bits = ((s_9_61) | (s_9_58));
        // D s_9_63: cast reint s_9_62 -> u64
        let s_9_63: u64 = (s_9_62.value() as u64);
        // D s_9_64: write-var start_address <= s_9_63
        fn_state.start_address = s_9_63;
        // N s_9_65: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_10_0: read-var tg:u8
        let s_10_0: u8 = fn_state.tg;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b14 b11
        if s_10_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_11_0: const #14s : i64
        let s_11_0: i64 = 14;
        // D s_11_1: write-var tg_bits <= s_11_0
        fn_state.tg_bits = s_11_0;
        // D s_11_2: read-var regime:u32
        let s_11_2: u32 = fn_state.regime;
        // D s_11_3: call HasLargeAddress(s_11_2)
        let s_11_3: bool = HasLargeAddress(state, tracer, s_11_2);
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Xt:u64
        let s_12_1: u64 = fn_state.Xt;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 64u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #36s : i
        let s_12_5: i128 = 36;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_0 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u37
        let s_12_8: u64 = (s_12_7.value() as u64);
        // C s_12_9: const #14s : i
        let s_12_9: i128 = 14;
        // D s_12_10: read-var start_address:u64
        let s_12_10: u64 = fn_state.start_address;
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 64u16);
        // D s_12_12: cast zx s_12_8 -> bv
        let s_12_12: Bits = Bits::new(s_12_8 as u128, 37u16);
        // C s_12_13: const #36s : i
        let s_12_13: i128 = 36;
        // C s_12_14: const #1u : u64
        let s_12_14: u64 = 1;
        // C s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 64u16);
        // C s_12_16: lsl s_12_15 s_12_13
        let s_12_16: Bits = s_12_15 << s_12_13;
        // C s_12_17: sub s_12_16 s_12_15
        let s_12_17: Bits = ((s_12_16) - (s_12_15));
        // D s_12_18: and s_12_12 s_12_17
        let s_12_18: Bits = ((s_12_12) & (s_12_17));
        // D s_12_19: lsl s_12_18 s_12_9
        let s_12_19: Bits = s_12_18 << s_12_9;
        // C s_12_20: lsl s_12_17 s_12_9
        let s_12_20: Bits = s_12_17 << s_12_9;
        // C s_12_21: cmpl s_12_20
        let s_12_21: Bits = !s_12_20;
        // D s_12_22: and s_12_11 s_12_21
        let s_12_22: Bits = ((s_12_11) & (s_12_21));
        // D s_12_23: or s_12_22 s_12_19
        let s_12_23: Bits = ((s_12_22) | (s_12_19));
        // D s_12_24: cast reint s_12_23 -> u64
        let s_12_24: u64 = (s_12_23.value() as u64);
        // D s_12_25: write-var start_address <= s_12_24
        fn_state.start_address = s_12_24;
        // C s_12_26: const #36s : i
        let s_12_26: i128 = 36;
        // D s_12_27: read-var Xt:u64
        let s_12_27: u64 = fn_state.Xt;
        // D s_12_28: cast zx s_12_27 -> bv
        let s_12_28: Bits = Bits::new(s_12_27 as u128, 64u16);
        // C s_12_29: const #1u : u64
        let s_12_29: u64 = 1;
        // D s_12_30: bit-extract s_12_28 s_12_26 s_12_29
        let s_12_30: Bits = (Bits::new(
            ((s_12_28) >> (s_12_26)).value(),
            u16::try_from(s_12_29).unwrap(),
        ));
        // D s_12_31: cast reint s_12_30 -> u8
        let s_12_31: bool = ((s_12_30.value()) != 0);
        // C s_12_32: const #0s : i
        let s_12_32: i128 = 0;
        // C s_12_33: const #0u : u64
        let s_12_33: u64 = 0;
        // D s_12_34: cast zx s_12_31 -> u64
        let s_12_34: u64 = (s_12_31 as u64);
        // C s_12_35: const #1u : u64
        let s_12_35: u64 = 1;
        // D s_12_36: and s_12_34 s_12_35
        let s_12_36: u64 = ((s_12_34) & (s_12_35));
        // D s_12_37: cmp-eq s_12_36 s_12_35
        let s_12_37: bool = ((s_12_36) == (s_12_35));
        // D s_12_38: lsl s_12_34 s_12_32
        let s_12_38: u64 = s_12_34 << s_12_32;
        // D s_12_39: or s_12_33 s_12_38
        let s_12_39: u64 = ((s_12_33) | (s_12_38));
        // D s_12_40: cmpl s_12_38
        let s_12_40: u64 = !s_12_38;
        // D s_12_41: and s_12_33 s_12_40
        let s_12_41: u64 = ((s_12_33) & (s_12_40));
        // D s_12_42: select s_12_37 s_12_39 s_12_41
        let s_12_42: u64 = if s_12_37 { s_12_39 } else { s_12_41 };
        // D s_12_43: cast trunc s_12_42 -> u8
        let s_12_43: bool = ((s_12_42) != 0);
        // D s_12_44: cast zx s_12_43 -> bv
        let s_12_44: Bits = Bits::new(s_12_43 as u128, 1u16);
        // C s_12_45: const #13u : u64
        let s_12_45: u64 = 13;
        // D s_12_46: call replicate_bits_borealis_internal(s_12_44, s_12_45)
        let s_12_46: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_12_44,
            s_12_45,
        );
        // D s_12_47: cast reint s_12_46 -> u13
        let s_12_47: u16 = (s_12_46.value() as u16);
        // C s_12_48: const #51s : i
        let s_12_48: i128 = 51;
        // D s_12_49: read-var start_address:u64
        let s_12_49: u64 = fn_state.start_address;
        // D s_12_50: cast zx s_12_49 -> bv
        let s_12_50: Bits = Bits::new(s_12_49 as u128, 64u16);
        // D s_12_51: cast zx s_12_47 -> bv
        let s_12_51: Bits = Bits::new(s_12_47 as u128, 13u16);
        // C s_12_52: const #12s : i
        let s_12_52: i128 = 12;
        // C s_12_53: const #1u : u64
        let s_12_53: u64 = 1;
        // C s_12_54: cast zx s_12_53 -> bv
        let s_12_54: Bits = Bits::new(s_12_53 as u128, 64u16);
        // C s_12_55: lsl s_12_54 s_12_52
        let s_12_55: Bits = s_12_54 << s_12_52;
        // C s_12_56: sub s_12_55 s_12_54
        let s_12_56: Bits = ((s_12_55) - (s_12_54));
        // D s_12_57: and s_12_51 s_12_56
        let s_12_57: Bits = ((s_12_51) & (s_12_56));
        // D s_12_58: lsl s_12_57 s_12_48
        let s_12_58: Bits = s_12_57 << s_12_48;
        // C s_12_59: lsl s_12_56 s_12_48
        let s_12_59: Bits = s_12_56 << s_12_48;
        // C s_12_60: cmpl s_12_59
        let s_12_60: Bits = !s_12_59;
        // D s_12_61: and s_12_50 s_12_60
        let s_12_61: Bits = ((s_12_50) & (s_12_60));
        // D s_12_62: or s_12_61 s_12_58
        let s_12_62: Bits = ((s_12_61) | (s_12_58));
        // D s_12_63: cast reint s_12_62 -> u64
        let s_12_63: u64 = (s_12_62.value() as u64);
        // D s_12_64: write-var start_address <= s_12_63
        fn_state.start_address = s_12_63;
        // N s_12_65: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Xt:u64
        let s_13_1: u64 = fn_state.Xt;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #36s : i
        let s_13_5: i128 = 36;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u37
        let s_13_8: u64 = (s_13_7.value() as u64);
        // C s_13_9: const #16s : i
        let s_13_9: i128 = 16;
        // D s_13_10: read-var start_address:u64
        let s_13_10: u64 = fn_state.start_address;
        // D s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 64u16);
        // D s_13_12: cast zx s_13_8 -> bv
        let s_13_12: Bits = Bits::new(s_13_8 as u128, 37u16);
        // C s_13_13: const #36s : i
        let s_13_13: i128 = 36;
        // C s_13_14: const #1u : u64
        let s_13_14: u64 = 1;
        // C s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 64u16);
        // C s_13_16: lsl s_13_15 s_13_13
        let s_13_16: Bits = s_13_15 << s_13_13;
        // C s_13_17: sub s_13_16 s_13_15
        let s_13_17: Bits = ((s_13_16) - (s_13_15));
        // D s_13_18: and s_13_12 s_13_17
        let s_13_18: Bits = ((s_13_12) & (s_13_17));
        // D s_13_19: lsl s_13_18 s_13_9
        let s_13_19: Bits = s_13_18 << s_13_9;
        // C s_13_20: lsl s_13_17 s_13_9
        let s_13_20: Bits = s_13_17 << s_13_9;
        // C s_13_21: cmpl s_13_20
        let s_13_21: Bits = !s_13_20;
        // D s_13_22: and s_13_11 s_13_21
        let s_13_22: Bits = ((s_13_11) & (s_13_21));
        // D s_13_23: or s_13_22 s_13_19
        let s_13_23: Bits = ((s_13_22) | (s_13_19));
        // D s_13_24: cast reint s_13_23 -> u64
        let s_13_24: u64 = (s_13_23.value() as u64);
        // D s_13_25: write-var start_address <= s_13_24
        fn_state.start_address = s_13_24;
        // C s_13_26: const #36s : i
        let s_13_26: i128 = 36;
        // D s_13_27: read-var Xt:u64
        let s_13_27: u64 = fn_state.Xt;
        // D s_13_28: cast zx s_13_27 -> bv
        let s_13_28: Bits = Bits::new(s_13_27 as u128, 64u16);
        // C s_13_29: const #1u : u64
        let s_13_29: u64 = 1;
        // D s_13_30: bit-extract s_13_28 s_13_26 s_13_29
        let s_13_30: Bits = (Bits::new(
            ((s_13_28) >> (s_13_26)).value(),
            u16::try_from(s_13_29).unwrap(),
        ));
        // D s_13_31: cast reint s_13_30 -> u8
        let s_13_31: bool = ((s_13_30.value()) != 0);
        // C s_13_32: const #0s : i
        let s_13_32: i128 = 0;
        // C s_13_33: const #0u : u64
        let s_13_33: u64 = 0;
        // D s_13_34: cast zx s_13_31 -> u64
        let s_13_34: u64 = (s_13_31 as u64);
        // C s_13_35: const #1u : u64
        let s_13_35: u64 = 1;
        // D s_13_36: and s_13_34 s_13_35
        let s_13_36: u64 = ((s_13_34) & (s_13_35));
        // D s_13_37: cmp-eq s_13_36 s_13_35
        let s_13_37: bool = ((s_13_36) == (s_13_35));
        // D s_13_38: lsl s_13_34 s_13_32
        let s_13_38: u64 = s_13_34 << s_13_32;
        // D s_13_39: or s_13_33 s_13_38
        let s_13_39: u64 = ((s_13_33) | (s_13_38));
        // D s_13_40: cmpl s_13_38
        let s_13_40: u64 = !s_13_38;
        // D s_13_41: and s_13_33 s_13_40
        let s_13_41: u64 = ((s_13_33) & (s_13_40));
        // D s_13_42: select s_13_37 s_13_39 s_13_41
        let s_13_42: u64 = if s_13_37 { s_13_39 } else { s_13_41 };
        // D s_13_43: cast trunc s_13_42 -> u8
        let s_13_43: bool = ((s_13_42) != 0);
        // D s_13_44: cast zx s_13_43 -> bv
        let s_13_44: Bits = Bits::new(s_13_43 as u128, 1u16);
        // C s_13_45: const #11u : u64
        let s_13_45: u64 = 11;
        // D s_13_46: call replicate_bits_borealis_internal(s_13_44, s_13_45)
        let s_13_46: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_13_44,
            s_13_45,
        );
        // D s_13_47: cast reint s_13_46 -> u11
        let s_13_47: u16 = (s_13_46.value() as u16);
        // C s_13_48: const #53s : i
        let s_13_48: i128 = 53;
        // D s_13_49: read-var start_address:u64
        let s_13_49: u64 = fn_state.start_address;
        // D s_13_50: cast zx s_13_49 -> bv
        let s_13_50: Bits = Bits::new(s_13_49 as u128, 64u16);
        // D s_13_51: cast zx s_13_47 -> bv
        let s_13_51: Bits = Bits::new(s_13_47 as u128, 11u16);
        // C s_13_52: const #10s : i
        let s_13_52: i128 = 10;
        // C s_13_53: const #1u : u64
        let s_13_53: u64 = 1;
        // C s_13_54: cast zx s_13_53 -> bv
        let s_13_54: Bits = Bits::new(s_13_53 as u128, 64u16);
        // C s_13_55: lsl s_13_54 s_13_52
        let s_13_55: Bits = s_13_54 << s_13_52;
        // C s_13_56: sub s_13_55 s_13_54
        let s_13_56: Bits = ((s_13_55) - (s_13_54));
        // D s_13_57: and s_13_51 s_13_56
        let s_13_57: Bits = ((s_13_51) & (s_13_56));
        // D s_13_58: lsl s_13_57 s_13_48
        let s_13_58: Bits = s_13_57 << s_13_48;
        // C s_13_59: lsl s_13_56 s_13_48
        let s_13_59: Bits = s_13_56 << s_13_48;
        // C s_13_60: cmpl s_13_59
        let s_13_60: Bits = !s_13_59;
        // D s_13_61: and s_13_50 s_13_60
        let s_13_61: Bits = ((s_13_50) & (s_13_60));
        // D s_13_62: or s_13_61 s_13_58
        let s_13_62: Bits = ((s_13_61) | (s_13_58));
        // D s_13_63: cast reint s_13_62 -> u64
        let s_13_63: u64 = (s_13_62.value() as u64);
        // D s_13_64: write-var start_address <= s_13_63
        fn_state.start_address = s_13_63;
        // N s_13_65: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_14_0: read-var tg:u8
        let s_14_0: u8 = fn_state.tg;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #3u : u8
        let s_14_2: u8 = 3;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // D s_15_1: write-var tg_bits <= s_15_0
        fn_state.tg_bits = s_15_0;
        // C s_15_2: const #0s : i
        let s_15_2: i128 = 0;
        // D s_15_3: read-var Xt:u64
        let s_15_3: u64 = fn_state.Xt;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 64u16);
        // C s_15_5: const #1s : i64
        let s_15_5: i64 = 1;
        // C s_15_6: cast zx s_15_5 -> i
        let s_15_6: i128 = (i128::try_from(s_15_5).unwrap());
        // C s_15_7: const #36s : i
        let s_15_7: i128 = 36;
        // C s_15_8: add s_15_7 s_15_6
        let s_15_8: i128 = (s_15_7 + s_15_6);
        // D s_15_9: bit-extract s_15_4 s_15_2 s_15_8
        let s_15_9: Bits = (Bits::new(
            ((s_15_4) >> (s_15_2)).value(),
            u16::try_from(s_15_8).unwrap(),
        ));
        // D s_15_10: cast reint s_15_9 -> u37
        let s_15_10: u64 = (s_15_9.value() as u64);
        // C s_15_11: const #16s : i
        let s_15_11: i128 = 16;
        // D s_15_12: read-var start_address:u64
        let s_15_12: u64 = fn_state.start_address;
        // D s_15_13: cast zx s_15_12 -> bv
        let s_15_13: Bits = Bits::new(s_15_12 as u128, 64u16);
        // D s_15_14: cast zx s_15_10 -> bv
        let s_15_14: Bits = Bits::new(s_15_10 as u128, 37u16);
        // C s_15_15: const #36s : i
        let s_15_15: i128 = 36;
        // C s_15_16: const #1u : u64
        let s_15_16: u64 = 1;
        // C s_15_17: cast zx s_15_16 -> bv
        let s_15_17: Bits = Bits::new(s_15_16 as u128, 64u16);
        // C s_15_18: lsl s_15_17 s_15_15
        let s_15_18: Bits = s_15_17 << s_15_15;
        // C s_15_19: sub s_15_18 s_15_17
        let s_15_19: Bits = ((s_15_18) - (s_15_17));
        // D s_15_20: and s_15_14 s_15_19
        let s_15_20: Bits = ((s_15_14) & (s_15_19));
        // D s_15_21: lsl s_15_20 s_15_11
        let s_15_21: Bits = s_15_20 << s_15_11;
        // C s_15_22: lsl s_15_19 s_15_11
        let s_15_22: Bits = s_15_19 << s_15_11;
        // C s_15_23: cmpl s_15_22
        let s_15_23: Bits = !s_15_22;
        // D s_15_24: and s_15_13 s_15_23
        let s_15_24: Bits = ((s_15_13) & (s_15_23));
        // D s_15_25: or s_15_24 s_15_21
        let s_15_25: Bits = ((s_15_24) | (s_15_21));
        // D s_15_26: cast reint s_15_25 -> u64
        let s_15_26: u64 = (s_15_25.value() as u64);
        // D s_15_27: write-var start_address <= s_15_26
        fn_state.start_address = s_15_26;
        // C s_15_28: const #36s : i
        let s_15_28: i128 = 36;
        // D s_15_29: read-var Xt:u64
        let s_15_29: u64 = fn_state.Xt;
        // D s_15_30: cast zx s_15_29 -> bv
        let s_15_30: Bits = Bits::new(s_15_29 as u128, 64u16);
        // C s_15_31: const #1u : u64
        let s_15_31: u64 = 1;
        // D s_15_32: bit-extract s_15_30 s_15_28 s_15_31
        let s_15_32: Bits = (Bits::new(
            ((s_15_30) >> (s_15_28)).value(),
            u16::try_from(s_15_31).unwrap(),
        ));
        // D s_15_33: cast reint s_15_32 -> u8
        let s_15_33: bool = ((s_15_32.value()) != 0);
        // C s_15_34: const #0s : i
        let s_15_34: i128 = 0;
        // C s_15_35: const #0u : u64
        let s_15_35: u64 = 0;
        // D s_15_36: cast zx s_15_33 -> u64
        let s_15_36: u64 = (s_15_33 as u64);
        // C s_15_37: const #1u : u64
        let s_15_37: u64 = 1;
        // D s_15_38: and s_15_36 s_15_37
        let s_15_38: u64 = ((s_15_36) & (s_15_37));
        // D s_15_39: cmp-eq s_15_38 s_15_37
        let s_15_39: bool = ((s_15_38) == (s_15_37));
        // D s_15_40: lsl s_15_36 s_15_34
        let s_15_40: u64 = s_15_36 << s_15_34;
        // D s_15_41: or s_15_35 s_15_40
        let s_15_41: u64 = ((s_15_35) | (s_15_40));
        // D s_15_42: cmpl s_15_40
        let s_15_42: u64 = !s_15_40;
        // D s_15_43: and s_15_35 s_15_42
        let s_15_43: u64 = ((s_15_35) & (s_15_42));
        // D s_15_44: select s_15_39 s_15_41 s_15_43
        let s_15_44: u64 = if s_15_39 { s_15_41 } else { s_15_43 };
        // D s_15_45: cast trunc s_15_44 -> u8
        let s_15_45: bool = ((s_15_44) != 0);
        // D s_15_46: cast zx s_15_45 -> bv
        let s_15_46: Bits = Bits::new(s_15_45 as u128, 1u16);
        // C s_15_47: const #11u : u64
        let s_15_47: u64 = 11;
        // D s_15_48: call replicate_bits_borealis_internal(s_15_46, s_15_47)
        let s_15_48: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_15_46,
            s_15_47,
        );
        // D s_15_49: cast reint s_15_48 -> u11
        let s_15_49: u16 = (s_15_48.value() as u16);
        // C s_15_50: const #53s : i
        let s_15_50: i128 = 53;
        // D s_15_51: read-var start_address:u64
        let s_15_51: u64 = fn_state.start_address;
        // D s_15_52: cast zx s_15_51 -> bv
        let s_15_52: Bits = Bits::new(s_15_51 as u128, 64u16);
        // D s_15_53: cast zx s_15_49 -> bv
        let s_15_53: Bits = Bits::new(s_15_49 as u128, 11u16);
        // C s_15_54: const #10s : i
        let s_15_54: i128 = 10;
        // C s_15_55: const #1u : u64
        let s_15_55: u64 = 1;
        // C s_15_56: cast zx s_15_55 -> bv
        let s_15_56: Bits = Bits::new(s_15_55 as u128, 64u16);
        // C s_15_57: lsl s_15_56 s_15_54
        let s_15_57: Bits = s_15_56 << s_15_54;
        // C s_15_58: sub s_15_57 s_15_56
        let s_15_58: Bits = ((s_15_57) - (s_15_56));
        // D s_15_59: and s_15_53 s_15_58
        let s_15_59: Bits = ((s_15_53) & (s_15_58));
        // D s_15_60: lsl s_15_59 s_15_50
        let s_15_60: Bits = s_15_59 << s_15_50;
        // C s_15_61: lsl s_15_58 s_15_50
        let s_15_61: Bits = s_15_58 << s_15_50;
        // C s_15_62: cmpl s_15_61
        let s_15_62: Bits = !s_15_61;
        // D s_15_63: and s_15_52 s_15_62
        let s_15_63: Bits = ((s_15_52) & (s_15_62));
        // D s_15_64: or s_15_63 s_15_60
        let s_15_64: Bits = ((s_15_63) | (s_15_60));
        // D s_15_65: cast reint s_15_64 -> u64
        let s_15_65: u64 = (s_15_64.value() as u64);
        // D s_15_66: write-var start_address <= s_15_65
        fn_state.start_address = s_15_65;
        // N s_15_67: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Unreachable(s_16_0)
        let s_16_1: () = Unreachable(state, tracer, s_16_0);
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: read-var tg:u8
        let s_17_1: u8 = fn_state.tg;
        // D s_17_2: read-var start_address:u64
        let s_17_2: u64 = fn_state.start_address;
        // D s_17_3: read-var end_address_name:u64
        let s_17_3: u64 = fn_state.end_address_name;
        // D s_17_4: create-product struct = ["s_17_0", "s_17_1", "s_17_2", "s_17_3"]
        let s_17_4: ProductType37abbcb1894e7c56 = ProductType37abbcb1894e7c56 {
            _0: s_17_0,
            _1: s_17_1,
            _2: s_17_2,
            _3: s_17_3,
        };
        // D s_17_5: write-var return_value <= s_17_4
        fn_state.return_value = s_17_4;
        // N s_17_6: jump b7
        return block_7(state, tracer, fn_state);
    }
}
