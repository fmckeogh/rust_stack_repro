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
use Ones::*;
use integer_subrange::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn TLBIPRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    Xt: u128,
) -> ProductType37abbcb1894e7c56 {
    #[derive(Default)]
    struct FunctionState {
        tg: u8,
        scale: i64,
        tg_bits: i64,
        return_value: ProductType37abbcb1894e7c56,
        start_address: u64,
        num: i64,
        end_address_name: u64,
        regime: u32,
        Xt: u128,
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
        // D s_0_9: read-var Xt:u128
        let s_0_9: u128 = fn_state.Xt;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 128u16);
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
        // D s_0_19: read-var Xt:u128
        let s_0_19: u128 = fn_state.Xt;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 128u16);
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
        // D s_0_32: read-var Xt:u128
        let s_0_32: u128 = fn_state.Xt;
        // D s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 128u16);
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
        // N s_0_51: branch s_0_50 b13 b1
        if s_0_50 {
            return block_13(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b8 b2
        if s_1_5 {
            return block_8(state, tracer, fn_state);
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
        // C s_2_2: const #64s : i
        let s_2_2: i128 = 64;
        // D s_2_3: read-var Xt:u128
        let s_2_3: u128 = fn_state.Xt;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 128u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #43s : i
        let s_2_7: i128 = 43;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_2 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_2)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u44
        let s_2_10: u64 = (s_2_9.value() as u64);
        // C s_2_11: const #12s : i
        let s_2_11: i128 = 12;
        // D s_2_12: read-var start_address:u64
        let s_2_12: u64 = fn_state.start_address;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 64u16);
        // D s_2_14: cast zx s_2_10 -> bv
        let s_2_14: Bits = Bits::new(s_2_10 as u128, 44u16);
        // C s_2_15: const #43s : i
        let s_2_15: i128 = 43;
        // C s_2_16: const #1u : u64
        let s_2_16: u64 = 1;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 64u16);
        // C s_2_18: lsl s_2_17 s_2_15
        let s_2_18: Bits = s_2_17 << s_2_15;
        // C s_2_19: sub s_2_18 s_2_17
        let s_2_19: Bits = ((s_2_18) - (s_2_17));
        // D s_2_20: and s_2_14 s_2_19
        let s_2_20: Bits = ((s_2_14) & (s_2_19));
        // D s_2_21: lsl s_2_20 s_2_11
        let s_2_21: Bits = s_2_20 << s_2_11;
        // C s_2_22: lsl s_2_19 s_2_11
        let s_2_22: Bits = s_2_19 << s_2_11;
        // C s_2_23: cmpl s_2_22
        let s_2_23: Bits = !s_2_22;
        // D s_2_24: and s_2_13 s_2_23
        let s_2_24: Bits = ((s_2_13) & (s_2_23));
        // D s_2_25: or s_2_24 s_2_21
        let s_2_25: Bits = ((s_2_24) | (s_2_21));
        // D s_2_26: cast reint s_2_25 -> u64
        let s_2_26: u64 = (s_2_25.value() as u64);
        // D s_2_27: write-var start_address <= s_2_26
        fn_state.start_address = s_2_26;
        // C s_2_28: const #107s : i
        let s_2_28: i128 = 107;
        // D s_2_29: read-var Xt:u128
        let s_2_29: u128 = fn_state.Xt;
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 128u16);
        // C s_2_31: const #1u : u64
        let s_2_31: u64 = 1;
        // D s_2_32: bit-extract s_2_30 s_2_28 s_2_31
        let s_2_32: Bits = (Bits::new(
            ((s_2_30) >> (s_2_28)).value(),
            u16::try_from(s_2_31).unwrap(),
        ));
        // D s_2_33: cast reint s_2_32 -> u8
        let s_2_33: bool = ((s_2_32.value()) != 0);
        // C s_2_34: const #0s : i
        let s_2_34: i128 = 0;
        // C s_2_35: const #0u : u64
        let s_2_35: u64 = 0;
        // D s_2_36: cast zx s_2_33 -> u64
        let s_2_36: u64 = (s_2_33 as u64);
        // C s_2_37: const #1u : u64
        let s_2_37: u64 = 1;
        // D s_2_38: and s_2_36 s_2_37
        let s_2_38: u64 = ((s_2_36) & (s_2_37));
        // D s_2_39: cmp-eq s_2_38 s_2_37
        let s_2_39: bool = ((s_2_38) == (s_2_37));
        // D s_2_40: lsl s_2_36 s_2_34
        let s_2_40: u64 = s_2_36 << s_2_34;
        // D s_2_41: or s_2_35 s_2_40
        let s_2_41: u64 = ((s_2_35) | (s_2_40));
        // D s_2_42: cmpl s_2_40
        let s_2_42: u64 = !s_2_40;
        // D s_2_43: and s_2_35 s_2_42
        let s_2_43: u64 = ((s_2_35) & (s_2_42));
        // D s_2_44: select s_2_39 s_2_41 s_2_43
        let s_2_44: u64 = if s_2_39 { s_2_41 } else { s_2_43 };
        // D s_2_45: cast trunc s_2_44 -> u8
        let s_2_45: bool = ((s_2_44) != 0);
        // D s_2_46: cast zx s_2_45 -> bv
        let s_2_46: Bits = Bits::new(s_2_45 as u128, 1u16);
        // C s_2_47: const #8u : u64
        let s_2_47: u64 = 8;
        // D s_2_48: call replicate_bits_borealis_internal(s_2_46, s_2_47)
        let s_2_48: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_2_46,
            s_2_47,
        );
        // D s_2_49: cast reint s_2_48 -> u8
        let s_2_49: u8 = (s_2_48.value() as u8);
        // C s_2_50: const #56s : i
        let s_2_50: i128 = 56;
        // D s_2_51: read-var start_address:u64
        let s_2_51: u64 = fn_state.start_address;
        // D s_2_52: cast zx s_2_51 -> bv
        let s_2_52: Bits = Bits::new(s_2_51 as u128, 64u16);
        // D s_2_53: cast zx s_2_49 -> bv
        let s_2_53: Bits = Bits::new(s_2_49 as u128, 8u16);
        // C s_2_54: const #7s : i
        let s_2_54: i128 = 7;
        // C s_2_55: const #1u : u64
        let s_2_55: u64 = 1;
        // C s_2_56: cast zx s_2_55 -> bv
        let s_2_56: Bits = Bits::new(s_2_55 as u128, 64u16);
        // C s_2_57: lsl s_2_56 s_2_54
        let s_2_57: Bits = s_2_56 << s_2_54;
        // C s_2_58: sub s_2_57 s_2_56
        let s_2_58: Bits = ((s_2_57) - (s_2_56));
        // D s_2_59: and s_2_53 s_2_58
        let s_2_59: Bits = ((s_2_53) & (s_2_58));
        // D s_2_60: lsl s_2_59 s_2_50
        let s_2_60: Bits = s_2_59 << s_2_50;
        // C s_2_61: lsl s_2_58 s_2_50
        let s_2_61: Bits = s_2_58 << s_2_50;
        // C s_2_62: cmpl s_2_61
        let s_2_62: Bits = !s_2_61;
        // D s_2_63: and s_2_52 s_2_62
        let s_2_63: Bits = ((s_2_52) & (s_2_62));
        // D s_2_64: or s_2_63 s_2_60
        let s_2_64: Bits = ((s_2_63) | (s_2_60));
        // D s_2_65: cast reint s_2_64 -> u64
        let s_2_65: u64 = (s_2_64.value() as u64);
        // D s_2_66: write-var start_address <= s_2_65
        fn_state.start_address = s_2_65;
        // N s_2_67: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_3_0: read-var tg_bits:i64
        let s_3_0: i64 = fn_state.tg_bits;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var num:i64
        let s_3_2: i64 = fn_state.num;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_3 s_3_1
        let s_3_4: i128 = (s_3_3 + s_3_1);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #5s : i
        let s_3_6: i128 = 5;
        // D s_3_7: read-var scale:i64
        let s_3_7: i64 = fn_state.scale;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: mul s_3_6 s_3_8
        let s_3_9: i128 = ((s_3_6) * (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: const #1s : i
        let s_3_11: i128 = 1;
        // D s_3_12: cast zx s_3_10 -> i
        let s_3_12: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_13: add s_3_12 s_3_11
        let s_3_13: i128 = (s_3_12 + s_3_11);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast zx s_3_0 -> i
        let s_3_16: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_17: add s_3_15 s_3_16
        let s_3_17: i128 = (s_3_15 + s_3_16);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: cast zx s_3_5 -> i
        let s_3_19: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_20: cast zx s_3_18 -> i
        let s_3_20: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_21: lsl s_3_19 s_3_20
        let s_3_21: i128 = s_3_19 << s_3_20;
        // C s_3_22: const #63s : i
        let s_3_22: i128 = 63;
        // C s_3_23: const #0s : i
        let s_3_23: i128 = 0;
        // D s_3_24: call integer_subrange(s_3_21, s_3_22, s_3_23)
        let s_3_24: Bits = integer_subrange(state, tracer, s_3_21, s_3_22, s_3_23);
        // D s_3_25: cast reint s_3_24 -> u64
        let s_3_25: u64 = (s_3_24.value() as u64);
        // D s_3_26: read-var start_address:u64
        let s_3_26: u64 = fn_state.start_address;
        // D s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 64u16);
        // D s_3_28: cast zx s_3_25 -> bv
        let s_3_28: Bits = Bits::new(s_3_25 as u128, 64u16);
        // D s_3_29: add s_3_27 s_3_28
        let s_3_29: Bits = (s_3_27 + s_3_28);
        // D s_3_30: cast reint s_3_29 -> u64
        let s_3_30: u64 = (s_3_29.value() as u64);
        // D s_3_31: write-var end_address_name <= s_3_30
        fn_state.end_address_name = s_3_30;
        // C s_3_32: const #55s : i
        let s_3_32: i128 = 55;
        // D s_3_33: read-var end_address_name:u64
        let s_3_33: u64 = fn_state.end_address_name;
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 64u16);
        // C s_3_35: const #1u : u64
        let s_3_35: u64 = 1;
        // D s_3_36: bit-extract s_3_34 s_3_32 s_3_35
        let s_3_36: Bits = (Bits::new(
            ((s_3_34) >> (s_3_32)).value(),
            u16::try_from(s_3_35).unwrap(),
        ));
        // D s_3_37: cast reint s_3_36 -> u8
        let s_3_37: bool = ((s_3_36.value()) != 0);
        // C s_3_38: const #0s : i
        let s_3_38: i128 = 0;
        // C s_3_39: const #0u : u64
        let s_3_39: u64 = 0;
        // D s_3_40: cast zx s_3_37 -> u64
        let s_3_40: u64 = (s_3_37 as u64);
        // C s_3_41: const #1u : u64
        let s_3_41: u64 = 1;
        // D s_3_42: and s_3_40 s_3_41
        let s_3_42: u64 = ((s_3_40) & (s_3_41));
        // D s_3_43: cmp-eq s_3_42 s_3_41
        let s_3_43: bool = ((s_3_42) == (s_3_41));
        // D s_3_44: lsl s_3_40 s_3_38
        let s_3_44: u64 = s_3_40 << s_3_38;
        // D s_3_45: or s_3_39 s_3_44
        let s_3_45: u64 = ((s_3_39) | (s_3_44));
        // D s_3_46: cmpl s_3_44
        let s_3_46: u64 = !s_3_44;
        // D s_3_47: and s_3_39 s_3_46
        let s_3_47: u64 = ((s_3_39) & (s_3_46));
        // D s_3_48: select s_3_43 s_3_45 s_3_47
        let s_3_48: u64 = if s_3_43 { s_3_45 } else { s_3_47 };
        // D s_3_49: cast trunc s_3_48 -> u8
        let s_3_49: bool = ((s_3_48) != 0);
        // C s_3_50: const #55s : i
        let s_3_50: i128 = 55;
        // D s_3_51: read-var start_address:u64
        let s_3_51: u64 = fn_state.start_address;
        // D s_3_52: cast zx s_3_51 -> bv
        let s_3_52: Bits = Bits::new(s_3_51 as u128, 64u16);
        // C s_3_53: const #1u : u64
        let s_3_53: u64 = 1;
        // D s_3_54: bit-extract s_3_52 s_3_50 s_3_53
        let s_3_54: Bits = (Bits::new(
            ((s_3_52) >> (s_3_50)).value(),
            u16::try_from(s_3_53).unwrap(),
        ));
        // D s_3_55: cast reint s_3_54 -> u8
        let s_3_55: bool = ((s_3_54.value()) != 0);
        // C s_3_56: const #0s : i
        let s_3_56: i128 = 0;
        // C s_3_57: const #0u : u64
        let s_3_57: u64 = 0;
        // D s_3_58: cast zx s_3_55 -> u64
        let s_3_58: u64 = (s_3_55 as u64);
        // C s_3_59: const #1u : u64
        let s_3_59: u64 = 1;
        // D s_3_60: and s_3_58 s_3_59
        let s_3_60: u64 = ((s_3_58) & (s_3_59));
        // D s_3_61: cmp-eq s_3_60 s_3_59
        let s_3_61: bool = ((s_3_60) == (s_3_59));
        // D s_3_62: lsl s_3_58 s_3_56
        let s_3_62: u64 = s_3_58 << s_3_56;
        // D s_3_63: or s_3_57 s_3_62
        let s_3_63: u64 = ((s_3_57) | (s_3_62));
        // D s_3_64: cmpl s_3_62
        let s_3_64: u64 = !s_3_62;
        // D s_3_65: and s_3_57 s_3_64
        let s_3_65: u64 = ((s_3_57) & (s_3_64));
        // D s_3_66: select s_3_61 s_3_63 s_3_65
        let s_3_66: u64 = if s_3_61 { s_3_63 } else { s_3_65 };
        // D s_3_67: cast trunc s_3_66 -> u8
        let s_3_67: bool = ((s_3_66) != 0);
        // D s_3_68: cast zx s_3_49 -> bv
        let s_3_68: Bits = Bits::new(s_3_49 as u128, 1u16);
        // D s_3_69: cast zx s_3_67 -> bv
        let s_3_69: Bits = Bits::new(s_3_67 as u128, 1u16);
        // D s_3_70: cmp-ne s_3_68 s_3_69
        let s_3_70: bool = ((s_3_68) != (s_3_69));
        // N s_3_71: branch s_3_70 b7 b4
        if s_3_70 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: read-var tg:u8
        let s_5_1: u8 = fn_state.tg;
        // D s_5_2: read-var start_address:u64
        let s_5_2: u64 = fn_state.start_address;
        // D s_5_3: read-var end_address_name:u64
        let s_5_3: u64 = fn_state.end_address_name;
        // D s_5_4: create-product struct = ["s_5_0", "s_5_1", "s_5_2", "s_5_3"]
        let s_5_4: ProductType37abbcb1894e7c56 = ProductType37abbcb1894e7c56 {
            _0: s_5_0,
            _1: s_5_1,
            _2: s_5_2,
            _3: s_5_3,
        };
        // D s_5_5: write-var return_value <= s_5_4
        fn_state.return_value = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_6_0: read-var return_value:struct
        let s_6_0: ProductType37abbcb1894e7c56 = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_7_0: const #55s : i
        let s_7_0: i128 = 55;
        // D s_7_1: read-var start_address:u64
        let s_7_1: u64 = fn_state.start_address;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // C s_7_18: const #9s : i64
        let s_7_18: i64 = 9;
        // D s_7_19: cast zx s_7_17 -> bv
        let s_7_19: Bits = Bits::new(s_7_17 as u128, 1u16);
        // C s_7_20: cast zx s_7_18 -> i
        let s_7_20: i128 = (i128::try_from(s_7_18).unwrap());
        // C s_7_21: cast reint s_7_20 -> u64
        let s_7_21: u64 = (s_7_20 as u64);
        // D s_7_22: call replicate_bits_borealis_internal(s_7_19, s_7_21)
        let s_7_22: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_7_19,
            s_7_21,
        );
        // D s_7_23: cast reint s_7_22 -> u9
        let s_7_23: u16 = (s_7_22.value() as u16);
        // C s_7_24: const #55s : i
        let s_7_24: i128 = 55;
        // S s_7_25: call Ones(s_7_24)
        let s_7_25: Bits = Ones(state, tracer, s_7_24);
        // S s_7_26: cast reint s_7_25 -> u55
        let s_7_26: u64 = (s_7_25.value() as u64);
        // D s_7_27: cast zx s_7_23 -> bv
        let s_7_27: Bits = Bits::new(s_7_23 as u128, 9u16);
        // S s_7_28: cast zx s_7_26 -> bv
        let s_7_28: Bits = Bits::new(s_7_26 as u128, 55u16);
        // D s_7_29: cast reint s_7_27 -> u128
        let s_7_29: u128 = (s_7_27.value() as u128);
        // D s_7_30: size-of s_7_27
        let s_7_30: u16 = s_7_27.length();
        // S s_7_31: cast reint s_7_28 -> u128
        let s_7_31: u128 = (s_7_28.value() as u128);
        // D s_7_32: size-of s_7_28
        let s_7_32: u16 = s_7_28.length();
        // D s_7_33: lsl s_7_29 s_7_32
        let s_7_33: u128 = s_7_29 << s_7_32;
        // D s_7_34: or s_7_33 s_7_31
        let s_7_34: u128 = ((s_7_33) | (s_7_31));
        // D s_7_35: add s_7_30 s_7_32
        let s_7_35: u16 = (s_7_30 + s_7_32);
        // D s_7_36: create-bits s_7_34 s_7_35
        let s_7_36: Bits = Bits::new(s_7_34, s_7_35);
        // D s_7_37: cast reint s_7_36 -> u64
        let s_7_37: u64 = (s_7_36.value() as u64);
        // D s_7_38: write-var end_address_name <= s_7_37
        fn_state.end_address_name = s_7_37;
        // N s_7_39: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // D s_8_0: read-var tg:u8
        let s_8_0: u8 = fn_state.tg;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_9_0: const #14s : i64
        let s_9_0: i64 = 14;
        // D s_9_1: write-var tg_bits <= s_9_0
        fn_state.tg_bits = s_9_0;
        // C s_9_2: const #66s : i
        let s_9_2: i128 = 66;
        // D s_9_3: read-var Xt:u128
        let s_9_3: u128 = fn_state.Xt;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 128u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #41s : i
        let s_9_7: i128 = 41;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_2 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_2)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u42
        let s_9_10: u64 = (s_9_9.value() as u64);
        // C s_9_11: const #14s : i
        let s_9_11: i128 = 14;
        // D s_9_12: read-var start_address:u64
        let s_9_12: u64 = fn_state.start_address;
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 64u16);
        // D s_9_14: cast zx s_9_10 -> bv
        let s_9_14: Bits = Bits::new(s_9_10 as u128, 42u16);
        // C s_9_15: const #41s : i
        let s_9_15: i128 = 41;
        // C s_9_16: const #1u : u64
        let s_9_16: u64 = 1;
        // C s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 64u16);
        // C s_9_18: lsl s_9_17 s_9_15
        let s_9_18: Bits = s_9_17 << s_9_15;
        // C s_9_19: sub s_9_18 s_9_17
        let s_9_19: Bits = ((s_9_18) - (s_9_17));
        // D s_9_20: and s_9_14 s_9_19
        let s_9_20: Bits = ((s_9_14) & (s_9_19));
        // D s_9_21: lsl s_9_20 s_9_11
        let s_9_21: Bits = s_9_20 << s_9_11;
        // C s_9_22: lsl s_9_19 s_9_11
        let s_9_22: Bits = s_9_19 << s_9_11;
        // C s_9_23: cmpl s_9_22
        let s_9_23: Bits = !s_9_22;
        // D s_9_24: and s_9_13 s_9_23
        let s_9_24: Bits = ((s_9_13) & (s_9_23));
        // D s_9_25: or s_9_24 s_9_21
        let s_9_25: Bits = ((s_9_24) | (s_9_21));
        // D s_9_26: cast reint s_9_25 -> u64
        let s_9_26: u64 = (s_9_25.value() as u64);
        // D s_9_27: write-var start_address <= s_9_26
        fn_state.start_address = s_9_26;
        // C s_9_28: const #107s : i
        let s_9_28: i128 = 107;
        // D s_9_29: read-var Xt:u128
        let s_9_29: u128 = fn_state.Xt;
        // D s_9_30: cast zx s_9_29 -> bv
        let s_9_30: Bits = Bits::new(s_9_29 as u128, 128u16);
        // C s_9_31: const #1u : u64
        let s_9_31: u64 = 1;
        // D s_9_32: bit-extract s_9_30 s_9_28 s_9_31
        let s_9_32: Bits = (Bits::new(
            ((s_9_30) >> (s_9_28)).value(),
            u16::try_from(s_9_31).unwrap(),
        ));
        // D s_9_33: cast reint s_9_32 -> u8
        let s_9_33: bool = ((s_9_32.value()) != 0);
        // C s_9_34: const #0s : i
        let s_9_34: i128 = 0;
        // C s_9_35: const #0u : u64
        let s_9_35: u64 = 0;
        // D s_9_36: cast zx s_9_33 -> u64
        let s_9_36: u64 = (s_9_33 as u64);
        // C s_9_37: const #1u : u64
        let s_9_37: u64 = 1;
        // D s_9_38: and s_9_36 s_9_37
        let s_9_38: u64 = ((s_9_36) & (s_9_37));
        // D s_9_39: cmp-eq s_9_38 s_9_37
        let s_9_39: bool = ((s_9_38) == (s_9_37));
        // D s_9_40: lsl s_9_36 s_9_34
        let s_9_40: u64 = s_9_36 << s_9_34;
        // D s_9_41: or s_9_35 s_9_40
        let s_9_41: u64 = ((s_9_35) | (s_9_40));
        // D s_9_42: cmpl s_9_40
        let s_9_42: u64 = !s_9_40;
        // D s_9_43: and s_9_35 s_9_42
        let s_9_43: u64 = ((s_9_35) & (s_9_42));
        // D s_9_44: select s_9_39 s_9_41 s_9_43
        let s_9_44: u64 = if s_9_39 { s_9_41 } else { s_9_43 };
        // D s_9_45: cast trunc s_9_44 -> u8
        let s_9_45: bool = ((s_9_44) != 0);
        // D s_9_46: cast zx s_9_45 -> bv
        let s_9_46: Bits = Bits::new(s_9_45 as u128, 1u16);
        // C s_9_47: const #8u : u64
        let s_9_47: u64 = 8;
        // D s_9_48: call replicate_bits_borealis_internal(s_9_46, s_9_47)
        let s_9_48: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_9_46,
            s_9_47,
        );
        // D s_9_49: cast reint s_9_48 -> u8
        let s_9_49: u8 = (s_9_48.value() as u8);
        // C s_9_50: const #56s : i
        let s_9_50: i128 = 56;
        // D s_9_51: read-var start_address:u64
        let s_9_51: u64 = fn_state.start_address;
        // D s_9_52: cast zx s_9_51 -> bv
        let s_9_52: Bits = Bits::new(s_9_51 as u128, 64u16);
        // D s_9_53: cast zx s_9_49 -> bv
        let s_9_53: Bits = Bits::new(s_9_49 as u128, 8u16);
        // C s_9_54: const #7s : i
        let s_9_54: i128 = 7;
        // C s_9_55: const #1u : u64
        let s_9_55: u64 = 1;
        // C s_9_56: cast zx s_9_55 -> bv
        let s_9_56: Bits = Bits::new(s_9_55 as u128, 64u16);
        // C s_9_57: lsl s_9_56 s_9_54
        let s_9_57: Bits = s_9_56 << s_9_54;
        // C s_9_58: sub s_9_57 s_9_56
        let s_9_58: Bits = ((s_9_57) - (s_9_56));
        // D s_9_59: and s_9_53 s_9_58
        let s_9_59: Bits = ((s_9_53) & (s_9_58));
        // D s_9_60: lsl s_9_59 s_9_50
        let s_9_60: Bits = s_9_59 << s_9_50;
        // C s_9_61: lsl s_9_58 s_9_50
        let s_9_61: Bits = s_9_58 << s_9_50;
        // C s_9_62: cmpl s_9_61
        let s_9_62: Bits = !s_9_61;
        // D s_9_63: and s_9_52 s_9_62
        let s_9_63: Bits = ((s_9_52) & (s_9_62));
        // D s_9_64: or s_9_63 s_9_60
        let s_9_64: Bits = ((s_9_63) | (s_9_60));
        // D s_9_65: cast reint s_9_64 -> u64
        let s_9_65: u64 = (s_9_64.value() as u64);
        // D s_9_66: write-var start_address <= s_9_65
        fn_state.start_address = s_9_65;
        // N s_9_67: jump b3
        return block_3(state, tracer, fn_state);
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
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_11_0: const #16s : i64
        let s_11_0: i64 = 16;
        // D s_11_1: write-var tg_bits <= s_11_0
        fn_state.tg_bits = s_11_0;
        // C s_11_2: const #68s : i
        let s_11_2: i128 = 68;
        // D s_11_3: read-var Xt:u128
        let s_11_3: u128 = fn_state.Xt;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 128u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #39s : i
        let s_11_7: i128 = 39;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_2 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_2)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u40
        let s_11_10: u64 = (s_11_9.value() as u64);
        // C s_11_11: const #16s : i
        let s_11_11: i128 = 16;
        // D s_11_12: read-var start_address:u64
        let s_11_12: u64 = fn_state.start_address;
        // D s_11_13: cast zx s_11_12 -> bv
        let s_11_13: Bits = Bits::new(s_11_12 as u128, 64u16);
        // D s_11_14: cast zx s_11_10 -> bv
        let s_11_14: Bits = Bits::new(s_11_10 as u128, 40u16);
        // C s_11_15: const #39s : i
        let s_11_15: i128 = 39;
        // C s_11_16: const #1u : u64
        let s_11_16: u64 = 1;
        // C s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 64u16);
        // C s_11_18: lsl s_11_17 s_11_15
        let s_11_18: Bits = s_11_17 << s_11_15;
        // C s_11_19: sub s_11_18 s_11_17
        let s_11_19: Bits = ((s_11_18) - (s_11_17));
        // D s_11_20: and s_11_14 s_11_19
        let s_11_20: Bits = ((s_11_14) & (s_11_19));
        // D s_11_21: lsl s_11_20 s_11_11
        let s_11_21: Bits = s_11_20 << s_11_11;
        // C s_11_22: lsl s_11_19 s_11_11
        let s_11_22: Bits = s_11_19 << s_11_11;
        // C s_11_23: cmpl s_11_22
        let s_11_23: Bits = !s_11_22;
        // D s_11_24: and s_11_13 s_11_23
        let s_11_24: Bits = ((s_11_13) & (s_11_23));
        // D s_11_25: or s_11_24 s_11_21
        let s_11_25: Bits = ((s_11_24) | (s_11_21));
        // D s_11_26: cast reint s_11_25 -> u64
        let s_11_26: u64 = (s_11_25.value() as u64);
        // D s_11_27: write-var start_address <= s_11_26
        fn_state.start_address = s_11_26;
        // C s_11_28: const #107s : i
        let s_11_28: i128 = 107;
        // D s_11_29: read-var Xt:u128
        let s_11_29: u128 = fn_state.Xt;
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 128u16);
        // C s_11_31: const #1u : u64
        let s_11_31: u64 = 1;
        // D s_11_32: bit-extract s_11_30 s_11_28 s_11_31
        let s_11_32: Bits = (Bits::new(
            ((s_11_30) >> (s_11_28)).value(),
            u16::try_from(s_11_31).unwrap(),
        ));
        // D s_11_33: cast reint s_11_32 -> u8
        let s_11_33: bool = ((s_11_32.value()) != 0);
        // C s_11_34: const #0s : i
        let s_11_34: i128 = 0;
        // C s_11_35: const #0u : u64
        let s_11_35: u64 = 0;
        // D s_11_36: cast zx s_11_33 -> u64
        let s_11_36: u64 = (s_11_33 as u64);
        // C s_11_37: const #1u : u64
        let s_11_37: u64 = 1;
        // D s_11_38: and s_11_36 s_11_37
        let s_11_38: u64 = ((s_11_36) & (s_11_37));
        // D s_11_39: cmp-eq s_11_38 s_11_37
        let s_11_39: bool = ((s_11_38) == (s_11_37));
        // D s_11_40: lsl s_11_36 s_11_34
        let s_11_40: u64 = s_11_36 << s_11_34;
        // D s_11_41: or s_11_35 s_11_40
        let s_11_41: u64 = ((s_11_35) | (s_11_40));
        // D s_11_42: cmpl s_11_40
        let s_11_42: u64 = !s_11_40;
        // D s_11_43: and s_11_35 s_11_42
        let s_11_43: u64 = ((s_11_35) & (s_11_42));
        // D s_11_44: select s_11_39 s_11_41 s_11_43
        let s_11_44: u64 = if s_11_39 { s_11_41 } else { s_11_43 };
        // D s_11_45: cast trunc s_11_44 -> u8
        let s_11_45: bool = ((s_11_44) != 0);
        // D s_11_46: cast zx s_11_45 -> bv
        let s_11_46: Bits = Bits::new(s_11_45 as u128, 1u16);
        // C s_11_47: const #8u : u64
        let s_11_47: u64 = 8;
        // D s_11_48: call replicate_bits_borealis_internal(s_11_46, s_11_47)
        let s_11_48: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_11_46,
            s_11_47,
        );
        // D s_11_49: cast reint s_11_48 -> u8
        let s_11_49: u8 = (s_11_48.value() as u8);
        // C s_11_50: const #56s : i
        let s_11_50: i128 = 56;
        // D s_11_51: read-var start_address:u64
        let s_11_51: u64 = fn_state.start_address;
        // D s_11_52: cast zx s_11_51 -> bv
        let s_11_52: Bits = Bits::new(s_11_51 as u128, 64u16);
        // D s_11_53: cast zx s_11_49 -> bv
        let s_11_53: Bits = Bits::new(s_11_49 as u128, 8u16);
        // C s_11_54: const #7s : i
        let s_11_54: i128 = 7;
        // C s_11_55: const #1u : u64
        let s_11_55: u64 = 1;
        // C s_11_56: cast zx s_11_55 -> bv
        let s_11_56: Bits = Bits::new(s_11_55 as u128, 64u16);
        // C s_11_57: lsl s_11_56 s_11_54
        let s_11_57: Bits = s_11_56 << s_11_54;
        // C s_11_58: sub s_11_57 s_11_56
        let s_11_58: Bits = ((s_11_57) - (s_11_56));
        // D s_11_59: and s_11_53 s_11_58
        let s_11_59: Bits = ((s_11_53) & (s_11_58));
        // D s_11_60: lsl s_11_59 s_11_50
        let s_11_60: Bits = s_11_59 << s_11_50;
        // C s_11_61: lsl s_11_58 s_11_50
        let s_11_61: Bits = s_11_58 << s_11_50;
        // C s_11_62: cmpl s_11_61
        let s_11_62: Bits = !s_11_61;
        // D s_11_63: and s_11_52 s_11_62
        let s_11_63: Bits = ((s_11_52) & (s_11_62));
        // D s_11_64: or s_11_63 s_11_60
        let s_11_64: Bits = ((s_11_63) | (s_11_60));
        // D s_11_65: cast reint s_11_64 -> u64
        let s_11_65: u64 = (s_11_64.value() as u64);
        // D s_11_66: write-var start_address <= s_11_65
        fn_state.start_address = s_11_65;
        // N s_11_67: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call Unreachable(s_12_0)
        let s_12_1: () = Unreachable(state, tracer, s_12_0);
        // N s_12_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType37abbcb1894e7c56 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: read-var tg:u8
        let s_13_1: u8 = fn_state.tg;
        // D s_13_2: read-var start_address:u64
        let s_13_2: u64 = fn_state.start_address;
        // D s_13_3: read-var end_address_name:u64
        let s_13_3: u64 = fn_state.end_address_name;
        // D s_13_4: create-product struct = ["s_13_0", "s_13_1", "s_13_2", "s_13_3"]
        let s_13_4: ProductType37abbcb1894e7c56 = ProductType37abbcb1894e7c56 {
            _0: s_13_0,
            _1: s_13_1,
            _2: s_13_2,
            _3: s_13_3,
        };
        // D s_13_5: write-var return_value <= s_13_4
        fn_state.return_value = s_13_4;
        // N s_13_6: jump b6
        return block_6(state, tracer, fn_state);
    }
}
