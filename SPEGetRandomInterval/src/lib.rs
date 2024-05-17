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
use PC_read__1::*;
use common::*;
pub fn SPEGetRandomInterval<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25598: (),
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        new_byte: u8,
        i: i64,
        gs_25598: (),
    }
    let fn_state = FunctionState {
        gs_25598,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #22808u : u32
        let s_0_0: u32 = 22808;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #9s : i
        let s_1_0: i128 = 9;
        // C s_1_1: const #22008u : u32
        let s_1_1: u32 = 22008;
        // D s_1_2: read-reg s_1_1:u24
        let s_1_2: u32 = {
            let value = state.read_register::<u32>(s_1_1 as isize);
            tracer.read_register(s_1_1 as isize, value);
            value
        };
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 24u16);
        // C s_1_4: const #1s : i64
        let s_1_4: i64 = 1;
        // C s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // C s_1_6: const #7s : i
        let s_1_6: i128 = 7;
        // C s_1_7: add s_1_6 s_1_5
        let s_1_7: i128 = (s_1_6 + s_1_5);
        // D s_1_8: bit-extract s_1_3 s_1_0 s_1_7
        let s_1_8: Bits = (Bits::new(
            ((s_1_3) >> (s_1_0)).value(),
            u16::try_from(s_1_7).unwrap(),
        ));
        // D s_1_9: cast reint s_1_8 -> u8
        let s_1_9: u8 = (s_1_8.value() as u8);
        // C s_1_10: const #14s : i
        let s_1_10: i128 = 14;
        // C s_1_11: const #22008u : u32
        let s_1_11: u32 = 22008;
        // D s_1_12: read-reg s_1_11:u24
        let s_1_12: u32 = {
            let value = state.read_register::<u32>(s_1_11 as isize);
            tracer.read_register(s_1_11 as isize, value);
            value
        };
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 24u16);
        // C s_1_14: const #1s : i64
        let s_1_14: i64 = 1;
        // C s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // C s_1_16: const #7s : i
        let s_1_16: i128 = 7;
        // C s_1_17: add s_1_16 s_1_15
        let s_1_17: i128 = (s_1_16 + s_1_15);
        // D s_1_18: bit-extract s_1_13 s_1_10 s_1_17
        let s_1_18: Bits = (Bits::new(
            ((s_1_13) >> (s_1_10)).value(),
            u16::try_from(s_1_17).unwrap(),
        ));
        // D s_1_19: cast reint s_1_18 -> u8
        let s_1_19: u8 = (s_1_18.value() as u8);
        // D s_1_20: cast zx s_1_9 -> bv
        let s_1_20: Bits = Bits::new(s_1_9 as u128, 8u16);
        // D s_1_21: cast zx s_1_19 -> bv
        let s_1_21: Bits = Bits::new(s_1_19 as u128, 8u16);
        // D s_1_22: xor s_1_20 s_1_21
        let s_1_22: Bits = ((s_1_20) ^ (s_1_21));
        // D s_1_23: cast reint s_1_22 -> u8
        let s_1_23: u8 = (s_1_22.value() as u8);
        // C s_1_24: const #15s : i
        let s_1_24: i128 = 15;
        // C s_1_25: const #22008u : u32
        let s_1_25: u32 = 22008;
        // D s_1_26: read-reg s_1_25:u24
        let s_1_26: u32 = {
            let value = state.read_register::<u32>(s_1_25 as isize);
            tracer.read_register(s_1_25 as isize, value);
            value
        };
        // D s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 24u16);
        // C s_1_28: const #1s : i64
        let s_1_28: i64 = 1;
        // C s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // C s_1_30: const #7s : i
        let s_1_30: i128 = 7;
        // C s_1_31: add s_1_30 s_1_29
        let s_1_31: i128 = (s_1_30 + s_1_29);
        // D s_1_32: bit-extract s_1_27 s_1_24 s_1_31
        let s_1_32: Bits = (Bits::new(
            ((s_1_27) >> (s_1_24)).value(),
            u16::try_from(s_1_31).unwrap(),
        ));
        // D s_1_33: cast reint s_1_32 -> u8
        let s_1_33: u8 = (s_1_32.value() as u8);
        // D s_1_34: cast zx s_1_23 -> bv
        let s_1_34: Bits = Bits::new(s_1_23 as u128, 8u16);
        // D s_1_35: cast zx s_1_33 -> bv
        let s_1_35: Bits = Bits::new(s_1_33 as u128, 8u16);
        // D s_1_36: xor s_1_34 s_1_35
        let s_1_36: Bits = ((s_1_34) ^ (s_1_35));
        // D s_1_37: cast reint s_1_36 -> u8
        let s_1_37: u8 = (s_1_36.value() as u8);
        // D s_1_38: write-var new_byte <= s_1_37
        fn_state.new_byte = s_1_37;
        // C s_1_39: const #16s : i
        let s_1_39: i128 = 16;
        // C s_1_40: const #22008u : u32
        let s_1_40: u32 = 22008;
        // D s_1_41: read-reg s_1_40:u24
        let s_1_41: u32 = {
            let value = state.read_register::<u32>(s_1_40 as isize);
            tracer.read_register(s_1_40 as isize, value);
            value
        };
        // D s_1_42: cast zx s_1_41 -> bv
        let s_1_42: Bits = Bits::new(s_1_41 as u128, 24u16);
        // C s_1_43: const #1s : i64
        let s_1_43: i64 = 1;
        // C s_1_44: cast zx s_1_43 -> i
        let s_1_44: i128 = (i128::try_from(s_1_43).unwrap());
        // C s_1_45: const #7s : i
        let s_1_45: i128 = 7;
        // C s_1_46: add s_1_45 s_1_44
        let s_1_46: i128 = (s_1_45 + s_1_44);
        // D s_1_47: bit-extract s_1_42 s_1_39 s_1_46
        let s_1_47: Bits = (Bits::new(
            ((s_1_42) >> (s_1_39)).value(),
            u16::try_from(s_1_46).unwrap(),
        ));
        // D s_1_48: cast reint s_1_47 -> u8
        let s_1_48: u8 = (s_1_47.value() as u8);
        // D s_1_49: read-var new_byte:u8
        let s_1_49: u8 = fn_state.new_byte;
        // D s_1_50: cast zx s_1_49 -> bv
        let s_1_50: Bits = Bits::new(s_1_49 as u128, 8u16);
        // D s_1_51: cast zx s_1_48 -> bv
        let s_1_51: Bits = Bits::new(s_1_48 as u128, 8u16);
        // D s_1_52: xor s_1_50 s_1_51
        let s_1_52: Bits = ((s_1_50) ^ (s_1_51));
        // D s_1_53: cast reint s_1_52 -> u8
        let s_1_53: u8 = (s_1_52.value() as u8);
        // C s_1_54: const #0s : i
        let s_1_54: i128 = 0;
        // C s_1_55: const #22008u : u32
        let s_1_55: u32 = 22008;
        // D s_1_56: read-reg s_1_55:u24
        let s_1_56: u32 = {
            let value = state.read_register::<u32>(s_1_55 as isize);
            tracer.read_register(s_1_55 as isize, value);
            value
        };
        // D s_1_57: cast zx s_1_56 -> bv
        let s_1_57: Bits = Bits::new(s_1_56 as u128, 24u16);
        // C s_1_58: const #1s : i64
        let s_1_58: i64 = 1;
        // C s_1_59: cast zx s_1_58 -> i
        let s_1_59: i128 = (i128::try_from(s_1_58).unwrap());
        // C s_1_60: const #15s : i
        let s_1_60: i128 = 15;
        // C s_1_61: add s_1_60 s_1_59
        let s_1_61: i128 = (s_1_60 + s_1_59);
        // D s_1_62: bit-extract s_1_57 s_1_54 s_1_61
        let s_1_62: Bits = (Bits::new(
            ((s_1_57) >> (s_1_54)).value(),
            u16::try_from(s_1_61).unwrap(),
        ));
        // D s_1_63: cast reint s_1_62 -> u16
        let s_1_63: u16 = (s_1_62.value() as u16);
        // D s_1_64: cast zx s_1_63 -> bv
        let s_1_64: Bits = Bits::new(s_1_63 as u128, 16u16);
        // D s_1_65: cast zx s_1_53 -> bv
        let s_1_65: Bits = Bits::new(s_1_53 as u128, 8u16);
        // D s_1_66: cast reint s_1_64 -> u128
        let s_1_66: u128 = (s_1_64.value() as u128);
        // D s_1_67: size-of s_1_64
        let s_1_67: u16 = s_1_64.length();
        // D s_1_68: cast reint s_1_65 -> u128
        let s_1_68: u128 = (s_1_65.value() as u128);
        // D s_1_69: size-of s_1_65
        let s_1_69: u16 = s_1_65.length();
        // D s_1_70: lsl s_1_66 s_1_69
        let s_1_70: u128 = s_1_66 << s_1_69;
        // D s_1_71: or s_1_70 s_1_68
        let s_1_71: u128 = ((s_1_70) | (s_1_68));
        // D s_1_72: add s_1_67 s_1_69
        let s_1_72: u16 = (s_1_67 + s_1_69);
        // D s_1_73: create-bits s_1_71 s_1_72
        let s_1_73: Bits = Bits::new(s_1_71, s_1_72);
        // D s_1_74: cast reint s_1_73 -> u24
        let s_1_74: u32 = (s_1_73.value() as u32);
        // C s_1_75: const #22008u : u32
        let s_1_75: u32 = 22008;
        // N s_1_76: write-reg s_1_75 <= s_1_74
        let s_1_76: () = {
            state.write_register::<u32>(s_1_75 as isize, s_1_74);
            tracer.write_register(s_1_75 as isize, s_1_74);
        };
        // N s_1_77: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #16s : i
        let s_2_0: i128 = 16;
        // C s_2_1: const #22008u : u32
        let s_2_1: u32 = 22008;
        // D s_2_2: read-reg s_2_1:u24
        let s_2_2: u32 = {
            let value = state.read_register::<u32>(s_2_1 as isize);
            tracer.read_register(s_2_1 as isize, value);
            value
        };
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 24u16);
        // C s_2_4: const #1s : i64
        let s_2_4: i64 = 1;
        // C s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // C s_2_6: const #7s : i
        let s_2_6: i128 = 7;
        // C s_2_7: add s_2_6 s_2_5
        let s_2_7: i128 = (s_2_6 + s_2_5);
        // D s_2_8: bit-extract s_2_3 s_2_0 s_2_7
        let s_2_8: Bits = (Bits::new(
            ((s_2_3) >> (s_2_0)).value(),
            u16::try_from(s_2_7).unwrap(),
        ));
        // D s_2_9: cast reint s_2_8 -> u8
        let s_2_9: u8 = (s_2_8.value() as u8);
        // N s_2_10: return s_2_9
        return s_2_9;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // C s_3_1: const #22808u : u32
        let s_3_1: u32 = 22808;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<bool>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #() : ()
        let s_3_3: () = ();
        // S s_3_4: call PC_read__1(s_3_3)
        let s_3_4: u32 = PC_read__1(state, tracer, s_3_3);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // S s_3_6: cast zx s_3_4 -> bv
        let s_3_6: Bits = Bits::new(s_3_4 as u128, 32u16);
        // C s_3_7: const #1s : i64
        let s_3_7: i64 = 1;
        // C s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // C s_3_9: const #23s : i
        let s_3_9: i128 = 23;
        // C s_3_10: add s_3_9 s_3_8
        let s_3_10: i128 = (s_3_9 + s_3_8);
        // D s_3_11: bit-extract s_3_6 s_3_5 s_3_10
        let s_3_11: Bits = (Bits::new(
            ((s_3_6) >> (s_3_5)).value(),
            u16::try_from(s_3_10).unwrap(),
        ));
        // D s_3_12: cast reint s_3_11 -> u24
        let s_3_12: u32 = (s_3_11.value() as u32);
        // C s_3_13: const #22008u : u32
        let s_3_13: u32 = 22008;
        // N s_3_14: write-reg s_3_13 <= s_3_12
        let s_3_14: () = {
            state.write_register::<u32>(s_3_13 as isize, s_3_12);
            tracer.write_register(s_3_13 as isize, s_3_12);
        };
        // C s_3_15: const #7s : i64
        let s_3_15: i64 = 7;
        // D s_3_16: write-var i <= s_3_15
        fn_state.i = s_3_15;
        // N s_3_17: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #0s : i64
        let s_4_1: i64 = 0;
        // D s_4_2: cmp-lt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) < (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #9s : i
        let s_5_0: i128 = 9;
        // C s_5_1: const #22008u : u32
        let s_5_1: u32 = 22008;
        // D s_5_2: read-reg s_5_1:u24
        let s_5_2: u32 = {
            let value = state.read_register::<u32>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 24u16);
        // C s_5_4: const #1s : i64
        let s_5_4: i64 = 1;
        // C s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #7s : i
        let s_5_6: i128 = 7;
        // C s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: bit-extract s_5_3 s_5_0 s_5_7
        let s_5_8: Bits = (Bits::new(
            ((s_5_3) >> (s_5_0)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D s_5_9: cast reint s_5_8 -> u8
        let s_5_9: u8 = (s_5_8.value() as u8);
        // C s_5_10: const #14s : i
        let s_5_10: i128 = 14;
        // C s_5_11: const #22008u : u32
        let s_5_11: u32 = 22008;
        // D s_5_12: read-reg s_5_11:u24
        let s_5_12: u32 = {
            let value = state.read_register::<u32>(s_5_11 as isize);
            tracer.read_register(s_5_11 as isize, value);
            value
        };
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 24u16);
        // C s_5_14: const #1s : i64
        let s_5_14: i64 = 1;
        // C s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // C s_5_16: const #7s : i
        let s_5_16: i128 = 7;
        // C s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: bit-extract s_5_13 s_5_10 s_5_17
        let s_5_18: Bits = (Bits::new(
            ((s_5_13) >> (s_5_10)).value(),
            u16::try_from(s_5_17).unwrap(),
        ));
        // D s_5_19: cast reint s_5_18 -> u8
        let s_5_19: u8 = (s_5_18.value() as u8);
        // D s_5_20: cast zx s_5_9 -> bv
        let s_5_20: Bits = Bits::new(s_5_9 as u128, 8u16);
        // D s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 8u16);
        // D s_5_22: xor s_5_20 s_5_21
        let s_5_22: Bits = ((s_5_20) ^ (s_5_21));
        // D s_5_23: cast reint s_5_22 -> u8
        let s_5_23: u8 = (s_5_22.value() as u8);
        // C s_5_24: const #15s : i
        let s_5_24: i128 = 15;
        // C s_5_25: const #22008u : u32
        let s_5_25: u32 = 22008;
        // D s_5_26: read-reg s_5_25:u24
        let s_5_26: u32 = {
            let value = state.read_register::<u32>(s_5_25 as isize);
            tracer.read_register(s_5_25 as isize, value);
            value
        };
        // D s_5_27: cast zx s_5_26 -> bv
        let s_5_27: Bits = Bits::new(s_5_26 as u128, 24u16);
        // C s_5_28: const #1s : i64
        let s_5_28: i64 = 1;
        // C s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // C s_5_30: const #7s : i
        let s_5_30: i128 = 7;
        // C s_5_31: add s_5_30 s_5_29
        let s_5_31: i128 = (s_5_30 + s_5_29);
        // D s_5_32: bit-extract s_5_27 s_5_24 s_5_31
        let s_5_32: Bits = (Bits::new(
            ((s_5_27) >> (s_5_24)).value(),
            u16::try_from(s_5_31).unwrap(),
        ));
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // D s_5_34: cast zx s_5_23 -> bv
        let s_5_34: Bits = Bits::new(s_5_23 as u128, 8u16);
        // D s_5_35: cast zx s_5_33 -> bv
        let s_5_35: Bits = Bits::new(s_5_33 as u128, 8u16);
        // D s_5_36: xor s_5_34 s_5_35
        let s_5_36: Bits = ((s_5_34) ^ (s_5_35));
        // D s_5_37: cast reint s_5_36 -> u8
        let s_5_37: u8 = (s_5_36.value() as u8);
        // D s_5_38: write-var new_byte <= s_5_37
        fn_state.new_byte = s_5_37;
        // C s_5_39: const #16s : i
        let s_5_39: i128 = 16;
        // C s_5_40: const #22008u : u32
        let s_5_40: u32 = 22008;
        // D s_5_41: read-reg s_5_40:u24
        let s_5_41: u32 = {
            let value = state.read_register::<u32>(s_5_40 as isize);
            tracer.read_register(s_5_40 as isize, value);
            value
        };
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 24u16);
        // C s_5_43: const #1s : i64
        let s_5_43: i64 = 1;
        // C s_5_44: cast zx s_5_43 -> i
        let s_5_44: i128 = (i128::try_from(s_5_43).unwrap());
        // C s_5_45: const #7s : i
        let s_5_45: i128 = 7;
        // C s_5_46: add s_5_45 s_5_44
        let s_5_46: i128 = (s_5_45 + s_5_44);
        // D s_5_47: bit-extract s_5_42 s_5_39 s_5_46
        let s_5_47: Bits = (Bits::new(
            ((s_5_42) >> (s_5_39)).value(),
            u16::try_from(s_5_46).unwrap(),
        ));
        // D s_5_48: cast reint s_5_47 -> u8
        let s_5_48: u8 = (s_5_47.value() as u8);
        // D s_5_49: read-var new_byte:u8
        let s_5_49: u8 = fn_state.new_byte;
        // D s_5_50: cast zx s_5_49 -> bv
        let s_5_50: Bits = Bits::new(s_5_49 as u128, 8u16);
        // D s_5_51: cast zx s_5_48 -> bv
        let s_5_51: Bits = Bits::new(s_5_48 as u128, 8u16);
        // D s_5_52: xor s_5_50 s_5_51
        let s_5_52: Bits = ((s_5_50) ^ (s_5_51));
        // D s_5_53: cast reint s_5_52 -> u8
        let s_5_53: u8 = (s_5_52.value() as u8);
        // D s_5_54: write-var new_byte <= s_5_53
        fn_state.new_byte = s_5_53;
        // C s_5_55: const #0s : i
        let s_5_55: i128 = 0;
        // C s_5_56: const #22008u : u32
        let s_5_56: u32 = 22008;
        // D s_5_57: read-reg s_5_56:u24
        let s_5_57: u32 = {
            let value = state.read_register::<u32>(s_5_56 as isize);
            tracer.read_register(s_5_56 as isize, value);
            value
        };
        // D s_5_58: cast zx s_5_57 -> bv
        let s_5_58: Bits = Bits::new(s_5_57 as u128, 24u16);
        // C s_5_59: const #1s : i64
        let s_5_59: i64 = 1;
        // C s_5_60: cast zx s_5_59 -> i
        let s_5_60: i128 = (i128::try_from(s_5_59).unwrap());
        // C s_5_61: const #15s : i
        let s_5_61: i128 = 15;
        // C s_5_62: add s_5_61 s_5_60
        let s_5_62: i128 = (s_5_61 + s_5_60);
        // D s_5_63: bit-extract s_5_58 s_5_55 s_5_62
        let s_5_63: Bits = (Bits::new(
            ((s_5_58) >> (s_5_55)).value(),
            u16::try_from(s_5_62).unwrap(),
        ));
        // D s_5_64: cast reint s_5_63 -> u16
        let s_5_64: u16 = (s_5_63.value() as u16);
        // D s_5_65: cast zx s_5_64 -> bv
        let s_5_65: Bits = Bits::new(s_5_64 as u128, 16u16);
        // D s_5_66: read-var new_byte:u8
        let s_5_66: u8 = fn_state.new_byte;
        // D s_5_67: cast zx s_5_66 -> bv
        let s_5_67: Bits = Bits::new(s_5_66 as u128, 8u16);
        // D s_5_68: cast reint s_5_65 -> u128
        let s_5_68: u128 = (s_5_65.value() as u128);
        // D s_5_69: size-of s_5_65
        let s_5_69: u16 = s_5_65.length();
        // D s_5_70: cast reint s_5_67 -> u128
        let s_5_70: u128 = (s_5_67.value() as u128);
        // D s_5_71: size-of s_5_67
        let s_5_71: u16 = s_5_67.length();
        // D s_5_72: lsl s_5_68 s_5_71
        let s_5_72: u128 = s_5_68 << s_5_71;
        // D s_5_73: or s_5_72 s_5_70
        let s_5_73: u128 = ((s_5_72) | (s_5_70));
        // D s_5_74: add s_5_69 s_5_71
        let s_5_74: u16 = (s_5_69 + s_5_71);
        // D s_5_75: create-bits s_5_73 s_5_74
        let s_5_75: Bits = Bits::new(s_5_73, s_5_74);
        // D s_5_76: cast reint s_5_75 -> u24
        let s_5_76: u32 = (s_5_75.value() as u32);
        // C s_5_77: const #22008u : u32
        let s_5_77: u32 = 22008;
        // N s_5_78: write-reg s_5_77 <= s_5_76
        let s_5_78: () = {
            state.write_register::<u32>(s_5_77 as isize, s_5_76);
            tracer.write_register(s_5_77 as isize, s_5_76);
        };
        // D s_5_79: read-var i:i64
        let s_5_79: i64 = fn_state.i;
        // C s_5_80: const #1s : i64
        let s_5_80: i64 = 1;
        // D s_5_81: sub s_5_79 s_5_80
        let s_5_81: i64 = ((s_5_79) - (s_5_80));
        // D s_5_82: write-var i <= s_5_81
        fn_state.i = s_5_81;
        // N s_5_83: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_6_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
