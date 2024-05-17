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
use CurrentVL_read::*;
use set_subrange_zeros::*;
use ConstrainUnpredictableBool::*;
use IsSVEEnabled::*;
use common::*;
pub fn V_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    width: i64,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        gs_4160: bool,
        gs_4159: bool,
        vlen: i64,
        ga_2605: i64,
        gs_4161: bool,
        gs_4158: bool,
        gs_4152: bool,
        widthshadow_48: i64,
        n: i128,
        width: i64,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        n,
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i64
        let s_0_0: i64 = fn_state.width;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var widthshadow#48 <= s_0_2
        fn_state.widthshadow_48 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CurrentVL_read(s_0_4)
        let s_0_5: i64 = CurrentVL_read(state, tracer, s_0_4);
        // D s_0_6: write-var VL <= s_0_5
        fn_state.VL = s_0_5;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // D s_0_8: read-var n:i
        let s_0_8: i128 = fn_state.n;
        // D s_0_9: cmp-ge s_0_8 s_0_7
        let s_0_9: bool = ((s_0_8) >= (s_0_7));
        // N s_0_10: branch s_0_9 b20 b1
        if s_0_9 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#4152 <= s_1_0
        fn_state.gs_4152 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#4152:u8
        let s_2_0: bool = fn_state.gs_4152;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var widthshadow#48:i64
        let s_2_3: i64 = fn_state.widthshadow_48;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cmp-eq s_2_4 s_2_2
        let s_2_5: bool = ((s_2_4) == (s_2_2));
        // N s_2_6: branch s_2_5 b19 b3
        if s_2_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: read-var widthshadow#48:i64
        let s_3_1: i64 = fn_state.widthshadow_48;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b18 b4
        if s_3_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // D s_4_1: read-var widthshadow#48:i64
        let s_4_1: i64 = fn_state.widthshadow_48;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b17 b5
        if s_4_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i
        let s_5_0: i128 = 64;
        // D s_5_1: read-var widthshadow#48:i64
        let s_5_1: i64 = fn_state.widthshadow_48;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b16 b6
        if s_5_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i
        let s_6_0: i128 = 128;
        // D s_6_1: read-var widthshadow#48:i64
        let s_6_1: i64 = fn_state.widthshadow_48;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // D s_6_4: write-var gs#4158 <= s_6_3
        fn_state.gs_4158 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#4158:u8
        let s_7_0: bool = fn_state.gs_4158;
        // D s_7_1: write-var gs#4159 <= s_7_0
        fn_state.gs_4159 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#4159:u8
        let s_8_0: bool = fn_state.gs_4159;
        // D s_8_1: write-var gs#4160 <= s_8_0
        fn_state.gs_4160 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#4160:u8
        let s_9_0: bool = fn_state.gs_4160;
        // D s_9_1: write-var gs#4161 <= s_9_0
        fn_state.gs_4161 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#4161:u8
        let s_10_0: bool = fn_state.gs_4161;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #16975u : u32
        let s_10_2: u32 = 16975;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: call IsSVEEnabled(s_10_3)
        let s_10_4: bool = IsSVEEnabled(state, tracer, s_10_3);
        // N s_10_5: branch s_10_4 b15 b11
        if s_10_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: write-var ga#2605 <= s_11_0
        fn_state.ga_2605 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#2605:i64
        let s_12_0: i64 = fn_state.ga_2605;
        // D s_12_1: write-var vlen <= s_12_0
        fn_state.vlen = s_12_0;
        // C s_12_2: const #50u : u32
        let s_12_2: u32 = 50;
        // S s_12_3: call ConstrainUnpredictableBool(s_12_2)
        let s_12_3: bool = ConstrainUnpredictableBool(state, tracer, s_12_2);
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1800u : u32
        let s_13_0: u32 = 1800;
        // D s_13_1: read-reg s_13_0:[u2048; 32]
        let s_13_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: read-var n:i
        let s_13_2: i128 = fn_state.n;
        // D s_13_3: read-element s_13_1[s_13_2]
        let s_13_3: u64 = s_13_1[(s_13_2) as usize];
        // C s_13_4: const #1s : i
        let s_13_4: i128 = 1;
        // D s_13_5: read-var vlen:i64
        let s_13_5: i64 = fn_state.vlen;
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: sub s_13_6 s_13_4
        let s_13_7: i128 = ((s_13_6) - (s_13_4));
        // D s_13_8: cast reint s_13_7 -> i64
        let s_13_8: i64 = (s_13_7 as i64);
        // D s_13_9: read-var value_name:bv
        let s_13_9: Bits = fn_state.value_name;
        // D s_13_10: size-of s_13_9
        let s_13_10: u16 = s_13_9.length();
        // D s_13_11: cast zx s_13_10 -> i
        let s_13_11: i128 = (i128::try_from(s_13_10).unwrap());
        // D s_13_12: cast reint s_13_11 -> i64
        let s_13_12: i64 = (s_13_11 as i64);
        // C s_13_13: const #0s : i
        let s_13_13: i128 = 0;
        // D s_13_14: cast zx s_13_12 -> i
        let s_13_14: i128 = (i128::try_from(s_13_12).unwrap());
        // D s_13_15: add s_13_13 s_13_14
        let s_13_15: i128 = (s_13_13 + s_13_14);
        // D s_13_16: cast reint s_13_15 -> i64
        let s_13_16: i64 = (s_13_15 as i64);
        // C s_13_17: const #2048s : i
        let s_13_17: i128 = 2048;
        // D s_13_18: cast zx s_13_3 -> bv
        let s_13_18: Bits = Bits::new(s_13_3 as u128, 2048u16);
        // D s_13_19: cast zx s_13_8 -> i
        let s_13_19: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_20: cast zx s_13_16 -> i
        let s_13_20: i128 = (i128::try_from(s_13_16).unwrap());
        // D s_13_21: call set_subrange_zeros(s_13_17, s_13_18, s_13_19, s_13_20)
        let s_13_21: Bits = set_subrange_zeros(
            state,
            tracer,
            s_13_17,
            s_13_18,
            s_13_19,
            s_13_20,
        );
        // D s_13_22: cast reint s_13_21 -> u2048
        let s_13_22: u64 = (s_13_21.value() as u64);
        // C s_13_23: const #1800u : u32
        let s_13_23: u32 = 1800;
        // D s_13_24: read-reg s_13_23:[u2048; 32]
        let s_13_24: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_13_23 as isize);
            tracer.read_register(s_13_23 as isize, value);
            value
        };
        // D s_13_25: read-var n:i
        let s_13_25: i128 = fn_state.n;
        // D s_13_26: mutate-element s_13_24[s_13_25] <= s_13_22
        let s_13_26: [u64; 32usize] = {
            let mut local = s_13_24.clone();
            local[(s_13_25) as usize] = s_13_22;
            local
        };
        // D s_13_27: cast cvt s_13_26 -> [u2048; 0]
        let s_13_27: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_13_26);
        // D s_13_28: cast cvt s_13_27 -> [u2048; 32]
        let s_13_28: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_13_27);
            buf
        };
        // C s_13_29: const #1800u : u32
        let s_13_29: u32 = 1800;
        // N s_13_30: write-reg s_13_29 <= s_13_28
        let s_13_30: () = {
            state.write_register::<[u64; 32usize]>(s_13_29 as isize, s_13_28);
            tracer.write_register(s_13_29 as isize, s_13_28);
        };
        // C s_13_31: const #1800u : u32
        let s_13_31: u32 = 1800;
        // D s_13_32: read-reg s_13_31:[u2048; 32]
        let s_13_32: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_13_31 as isize);
            tracer.read_register(s_13_31 as isize, value);
            value
        };
        // D s_13_33: read-var n:i
        let s_13_33: i128 = fn_state.n;
        // D s_13_34: read-element s_13_32[s_13_33]
        let s_13_34: u64 = s_13_32[(s_13_33) as usize];
        // D s_13_35: read-var value_name:bv
        let s_13_35: Bits = fn_state.value_name;
        // D s_13_36: size-of s_13_35
        let s_13_36: u16 = s_13_35.length();
        // D s_13_37: cast zx s_13_36 -> i
        let s_13_37: i128 = (i128::try_from(s_13_36).unwrap());
        // D s_13_38: cast reint s_13_37 -> i64
        let s_13_38: i64 = (s_13_37 as i64);
        // C s_13_39: const #0s : i
        let s_13_39: i128 = 0;
        // D s_13_40: cast zx s_13_38 -> i
        let s_13_40: i128 = (i128::try_from(s_13_38).unwrap());
        // D s_13_41: add s_13_39 s_13_40
        let s_13_41: i128 = (s_13_39 + s_13_40);
        // D s_13_42: cast reint s_13_41 -> i64
        let s_13_42: i64 = (s_13_41 as i64);
        // C s_13_43: const #1s : i
        let s_13_43: i128 = 1;
        // D s_13_44: cast zx s_13_42 -> i
        let s_13_44: i128 = (i128::try_from(s_13_42).unwrap());
        // D s_13_45: sub s_13_44 s_13_43
        let s_13_45: i128 = ((s_13_44) - (s_13_43));
        // D s_13_46: cast reint s_13_45 -> i64
        let s_13_46: i64 = (s_13_45 as i64);
        // C s_13_47: const #0s : i
        let s_13_47: i128 = 0;
        // D s_13_48: cast zx s_13_34 -> bv
        let s_13_48: Bits = Bits::new(s_13_34 as u128, 2048u16);
        // D s_13_49: cast zx s_13_46 -> i
        let s_13_49: i128 = (i128::try_from(s_13_46).unwrap());
        // D s_13_50: read-var value_name:bv
        let s_13_50: Bits = fn_state.value_name;
        // D s_13_51: sub s_13_49 s_13_47
        let s_13_51: i128 = ((s_13_49) - (s_13_47));
        // C s_13_52: const #1u : u64
        let s_13_52: u64 = 1;
        // C s_13_53: cast zx s_13_52 -> bv
        let s_13_53: Bits = Bits::new(s_13_52 as u128, 64u16);
        // D s_13_54: lsl s_13_53 s_13_51
        let s_13_54: Bits = s_13_53 << s_13_51;
        // D s_13_55: sub s_13_54 s_13_53
        let s_13_55: Bits = ((s_13_54) - (s_13_53));
        // D s_13_56: and s_13_50 s_13_55
        let s_13_56: Bits = ((s_13_50) & (s_13_55));
        // D s_13_57: lsl s_13_56 s_13_47
        let s_13_57: Bits = s_13_56 << s_13_47;
        // D s_13_58: lsl s_13_55 s_13_47
        let s_13_58: Bits = s_13_55 << s_13_47;
        // D s_13_59: cmpl s_13_58
        let s_13_59: Bits = !s_13_58;
        // D s_13_60: and s_13_48 s_13_59
        let s_13_60: Bits = ((s_13_48) & (s_13_59));
        // D s_13_61: or s_13_60 s_13_57
        let s_13_61: Bits = ((s_13_60) | (s_13_57));
        // D s_13_62: cast reint s_13_61 -> u2048
        let s_13_62: u64 = (s_13_61.value() as u64);
        // C s_13_63: const #1800u : u32
        let s_13_63: u32 = 1800;
        // D s_13_64: read-reg s_13_63:[u2048; 32]
        let s_13_64: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_13_63 as isize);
            tracer.read_register(s_13_63 as isize, value);
            value
        };
        // D s_13_65: read-var n:i
        let s_13_65: i128 = fn_state.n;
        // D s_13_66: mutate-element s_13_64[s_13_65] <= s_13_62
        let s_13_66: [u64; 32usize] = {
            let mut local = s_13_64.clone();
            local[(s_13_65) as usize] = s_13_62;
            local
        };
        // D s_13_67: cast cvt s_13_66 -> [u2048; 0]
        let s_13_67: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_13_66);
        // D s_13_68: cast cvt s_13_67 -> [u2048; 32]
        let s_13_68: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_13_67);
            buf
        };
        // C s_13_69: const #1800u : u32
        let s_13_69: u32 = 1800;
        // N s_13_70: write-reg s_13_69 <= s_13_68
        let s_13_70: () = {
            state.write_register::<[u64; 32usize]>(s_13_69 as isize, s_13_68);
            tracer.write_register(s_13_69 as isize, s_13_68);
        };
        // N s_13_71: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #808u : u32
        let s_14_0: u32 = 808;
        // D s_14_1: read-reg s_14_0:i64
        let s_14_1: i64 = {
            let value = state.read_register::<i64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: read-var value_name:bv
        let s_14_3: Bits = fn_state.value_name;
        // D s_14_4: bits-cast zx s_14_3 -> bv length s_14_2
        let s_14_4: Bits = s_14_3.zero_extend(s_14_2);
        // D s_14_5: cast reint s_14_4 -> u2048
        let s_14_5: u64 = (s_14_4.value() as u64);
        // C s_14_6: const #1800u : u32
        let s_14_6: u32 = 1800;
        // D s_14_7: read-reg s_14_6:[u2048; 32]
        let s_14_7: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_14_6 as isize);
            tracer.read_register(s_14_6 as isize, value);
            value
        };
        // D s_14_8: read-var n:i
        let s_14_8: i128 = fn_state.n;
        // D s_14_9: mutate-element s_14_7[s_14_8] <= s_14_5
        let s_14_9: [u64; 32usize] = {
            let mut local = s_14_7.clone();
            local[(s_14_8) as usize] = s_14_5;
            local
        };
        // D s_14_10: cast cvt s_14_9 -> [u2048; 0]
        let s_14_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_14_9);
        // D s_14_11: cast cvt s_14_10 -> [u2048; 32]
        let s_14_11: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_14_10);
            buf
        };
        // C s_14_12: const #1800u : u32
        let s_14_12: u32 = 1800;
        // N s_14_13: write-reg s_14_12 <= s_14_11
        let s_14_13: () = {
            state.write_register::<[u64; 32usize]>(s_14_12 as isize, s_14_11);
            tracer.write_register(s_14_12 as isize, s_14_11);
        };
        // N s_14_14: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var VL:i64
        let s_15_0: i64 = fn_state.VL;
        // D s_15_1: write-var ga#2605 <= s_15_0
        fn_state.ga_2605 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#4158 <= s_16_0
        fn_state.gs_4158 = s_16_0;
        // N s_16_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#4159 <= s_17_0
        fn_state.gs_4159 = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#4160 <= s_18_0
        fn_state.gs_4160 = s_18_0;
        // N s_18_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#4161 <= s_19_0
        fn_state.gs_4161 = s_19_0;
        // N s_19_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #31s : i
        let s_20_0: i128 = 31;
        // D s_20_1: read-var n:i
        let s_20_1: i128 = fn_state.n;
        // D s_20_2: cmp-le s_20_1 s_20_0
        let s_20_2: bool = ((s_20_1) <= (s_20_0));
        // D s_20_3: write-var gs#4152 <= s_20_2
        fn_state.gs_4152 = s_20_2;
        // N s_20_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
