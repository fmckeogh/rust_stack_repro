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
use u_get_AMCGCR_Type_CG0NC::*;
use AMCGCR_read::*;
use AArch32_AutoGen_SysRegRead64::*;
use ELFromM32::*;
use integer_subrange::*;
use u_get_AMCGCR_Type_CG1NC::*;
use common::*;
pub fn AArch32_SysRegRead64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp_num: i128,
    instr: u32,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_142228: bool,
        opc1: u8,
        ga_248440: ProductTypea5cc8de4daab131c,
        b__0: u8,
        CRm: u8,
        gs_142239: bool,
        el: u8,
        gs_142236: bool,
        nshadow_1014: i64,
        gs_142233: bool,
        cp_num: i128,
        instr: u32,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        cp_num,
        instr,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16983u : u32
        let s_0_0: u32 = 16983;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELFromM32(s_0_1)
        let s_0_2: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_1);
        // D s_0_3: write-var ga#248440 <= s_0_2
        fn_state.ga_248440 = s_0_2;
        // D s_0_4: read-var ga#248440.1:struct
        let s_0_4: u8 = fn_state.ga_248440._1;
        // D s_0_5: write-var el <= s_0_4
        fn_state.el = s_0_4;
        // C s_0_6: const #4s : i
        let s_0_6: i128 = 4;
        // C s_0_7: const #4s : i
        let s_0_7: i128 = 4;
        // D s_0_8: read-var instr:u32
        let s_0_8: u32 = fn_state.instr;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_6 s_0_7
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_6)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // D s_0_12: write-var opc1 <= s_0_11
        fn_state.opc1 = s_0_11;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // C s_0_14: const #4s : i
        let s_0_14: i128 = 4;
        // D s_0_15: read-var instr:u32
        let s_0_15: u32 = fn_state.instr;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 32u16);
        // D s_0_17: bit-extract s_0_16 s_0_13 s_0_14
        let s_0_17: Bits = (Bits::new(
            ((s_0_16) >> (s_0_13)).value(),
            u16::try_from(s_0_14).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // D s_0_19: write-var CRm <= s_0_18
        fn_state.CRm = s_0_18;
        // C s_0_20: const #15s : i
        let s_0_20: i128 = 15;
        // D s_0_21: read-var cp_num:i
        let s_0_21: i128 = fn_state.cp_num;
        // D s_0_22: cmp-eq s_0_21 s_0_20
        let s_0_22: bool = ((s_0_21) == (s_0_20));
        // N s_0_23: branch s_0_22 b18 b1
        if s_0_22 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#142236 <= s_1_0
        fn_state.gs_142236 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#142236:u8
        let s_2_0: bool = fn_state.gs_142236;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#142239 <= s_3_0
        fn_state.gs_142239 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#142239:u8
        let s_4_0: bool = fn_state.gs_142239;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #3s : i
        let s_6_0: i128 = 3;
        // C s_6_1: const #0s : i
        let s_6_1: i128 = 0;
        // D s_6_2: read-var cp_num:i
        let s_6_2: i128 = fn_state.cp_num;
        // D s_6_3: call integer_subrange(s_6_2, s_6_0, s_6_1)
        let s_6_3: Bits = integer_subrange(state, tracer, s_6_2, s_6_0, s_6_1);
        // D s_6_4: cast reint s_6_3 -> u8
        let s_6_4: u8 = (s_6_3.value() as u8);
        // D s_6_5: read-var el:u8
        let s_6_5: u8 = fn_state.el;
        // D s_6_6: read-var opc1:u8
        let s_6_6: u8 = fn_state.opc1;
        // D s_6_7: read-var CRm:u8
        let s_6_7: u8 = fn_state.CRm;
        // D s_6_8: read-var t:i
        let s_6_8: i128 = fn_state.t;
        // D s_6_9: read-var t2:i
        let s_6_9: i128 = fn_state.t2;
        // D s_6_10: call AArch32_AutoGen_SysRegRead64(s_6_5, s_6_4, s_6_6, s_6_7, s_6_8, s_6_9)
        let s_6_10: () = AArch32_AutoGen_SysRegRead64(
            state,
            tracer,
            s_6_5,
            s_6_4,
            s_6_6,
            s_6_7,
            s_6_8,
            s_6_9,
        );
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var CRm:u8
        let s_7_1: u8 = fn_state.CRm;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
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
        // C s_7_18: const #0s : i
        let s_7_18: i128 = 0;
        // D s_7_19: read-var opc1:u8
        let s_7_19: u8 = fn_state.opc1;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 4u16);
        // C s_7_21: const #1s : i64
        let s_7_21: i64 = 1;
        // C s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // C s_7_23: const #2s : i
        let s_7_23: i128 = 2;
        // C s_7_24: add s_7_23 s_7_22
        let s_7_24: i128 = (s_7_23 + s_7_22);
        // D s_7_25: bit-extract s_7_20 s_7_18 s_7_24
        let s_7_25: Bits = (Bits::new(
            ((s_7_20) >> (s_7_18)).value(),
            u16::try_from(s_7_24).unwrap(),
        ));
        // D s_7_26: cast reint s_7_25 -> u8
        let s_7_26: u8 = (s_7_25.value() as u8);
        // D s_7_27: cast zx s_7_17 -> bv
        let s_7_27: Bits = Bits::new(s_7_17 as u128, 1u16);
        // D s_7_28: cast zx s_7_26 -> bv
        let s_7_28: Bits = Bits::new(s_7_26 as u128, 3u16);
        // D s_7_29: cast reint s_7_27 -> u128
        let s_7_29: u128 = (s_7_27.value() as u128);
        // D s_7_30: size-of s_7_27
        let s_7_30: u16 = s_7_27.length();
        // D s_7_31: cast reint s_7_28 -> u128
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
        // D s_7_37: cast reint s_7_36 -> u8
        let s_7_37: u8 = (s_7_36.value() as u8);
        // D s_7_38: cast zx s_7_37 -> bv
        let s_7_38: Bits = Bits::new(s_7_37 as u128, 4u16);
        // D s_7_39: cast zx s_7_38 -> i
        let s_7_39: i128 = (s_7_38.value() as i128);
        // D s_7_40: cast reint s_7_39 -> i64
        let s_7_40: i64 = (s_7_39 as i64);
        // D s_7_41: write-var nshadow#1014 <= s_7_40
        fn_state.nshadow_1014 = s_7_40;
        // C s_7_42: const #1s : i
        let s_7_42: i128 = 1;
        // D s_7_43: read-var CRm:u8
        let s_7_43: u8 = fn_state.CRm;
        // D s_7_44: cast zx s_7_43 -> bv
        let s_7_44: Bits = Bits::new(s_7_43 as u128, 4u16);
        // C s_7_45: const #1s : i64
        let s_7_45: i64 = 1;
        // C s_7_46: cast zx s_7_45 -> i
        let s_7_46: i128 = (i128::try_from(s_7_45).unwrap());
        // C s_7_47: const #2s : i
        let s_7_47: i128 = 2;
        // C s_7_48: add s_7_47 s_7_46
        let s_7_48: i128 = (s_7_47 + s_7_46);
        // D s_7_49: bit-extract s_7_44 s_7_42 s_7_48
        let s_7_49: Bits = (Bits::new(
            ((s_7_44) >> (s_7_42)).value(),
            u16::try_from(s_7_48).unwrap(),
        ));
        // D s_7_50: cast reint s_7_49 -> u8
        let s_7_50: u8 = (s_7_49.value() as u8);
        // D s_7_51: cast zx s_7_50 -> bv
        let s_7_51: Bits = Bits::new(s_7_50 as u128, 3u16);
        // C s_7_52: const #0u : u8
        let s_7_52: u8 = 0;
        // C s_7_53: cast zx s_7_52 -> bv
        let s_7_53: Bits = Bits::new(s_7_52 as u128, 3u16);
        // D s_7_54: cmp-eq s_7_51 s_7_53
        let s_7_54: bool = ((s_7_51) == (s_7_53));
        // N s_7_55: branch s_7_54 b14 b8
        if s_7_54 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var CRm:u8
        let s_8_1: u8 = fn_state.CRm;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 4u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #2s : i
        let s_8_5: i128 = 2;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 3u16);
        // C s_8_10: const #2u : u8
        let s_8_10: u8 = 2;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 3u16);
        // D s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12: bool = ((s_8_9) == (s_8_11));
        // N s_8_13: branch s_8_12 b11 b9
        if s_8_12 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call AMCGCR_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = AMCGCR_read(state, tracer, s_11_0);
        // S s_11_2: call _get_AMCGCR_Type_CG1NC(s_11_1)
        let s_11_2: u8 = u_get_AMCGCR_Type_CG1NC(state, tracer, s_11_1);
        // S s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 8u16);
        // S s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (s_11_3.value() as i128);
        // S s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: read-var nshadow#1014:i64
        let s_11_6: i64 = fn_state.nshadow_1014;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // S s_11_8: cast zx s_11_5 -> i
        let s_11_8: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_9: cmp-ge s_11_7 s_11_8
        let s_11_9: bool = ((s_11_7) >= (s_11_8));
        // N s_11_10: branch s_11_9 b13 b12
        if s_11_9 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call AMCGCR_read(s_14_0)
        let s_14_1: ProductType700c18a878c5601b = AMCGCR_read(state, tracer, s_14_0);
        // S s_14_2: call _get_AMCGCR_Type_CG0NC(s_14_1)
        let s_14_2: u8 = u_get_AMCGCR_Type_CG0NC(state, tracer, s_14_1);
        // S s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 8u16);
        // S s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (s_14_3.value() as i128);
        // S s_14_5: cast reint s_14_4 -> i64
        let s_14_5: i64 = (s_14_4 as i64);
        // D s_14_6: read-var nshadow#1014:i64
        let s_14_6: i64 = fn_state.nshadow_1014;
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // S s_14_8: cast zx s_14_5 -> i
        let s_14_8: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_9: cmp-ge s_14_7 s_14_8
        let s_14_9: bool = ((s_14_7) >= (s_14_8));
        // N s_14_10: branch s_14_9 b16 b15
        if s_14_9 {
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
        // N s_15_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #3s : i
        let s_17_0: i128 = 3;
        // D s_17_1: read-var opc1:u8
        let s_17_1: u8 = fn_state.opc1;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 4u16);
        // C s_17_3: const #1u : u64
        let s_17_3: u64 = 1;
        // D s_17_4: bit-extract s_17_2 s_17_0 s_17_3
        let s_17_4: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_3).unwrap(),
        ));
        // D s_17_5: cast reint s_17_4 -> u8
        let s_17_5: bool = ((s_17_4.value()) != 0);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // C s_17_7: const #0u : u64
        let s_17_7: u64 = 0;
        // D s_17_8: cast zx s_17_5 -> u64
        let s_17_8: u64 = (s_17_5 as u64);
        // C s_17_9: const #1u : u64
        let s_17_9: u64 = 1;
        // D s_17_10: and s_17_8 s_17_9
        let s_17_10: u64 = ((s_17_8) & (s_17_9));
        // D s_17_11: cmp-eq s_17_10 s_17_9
        let s_17_11: bool = ((s_17_10) == (s_17_9));
        // D s_17_12: lsl s_17_8 s_17_6
        let s_17_12: u64 = s_17_8 << s_17_6;
        // D s_17_13: or s_17_7 s_17_12
        let s_17_13: u64 = ((s_17_7) | (s_17_12));
        // D s_17_14: cmpl s_17_12
        let s_17_14: u64 = !s_17_12;
        // D s_17_15: and s_17_7 s_17_14
        let s_17_15: u64 = ((s_17_7) & (s_17_14));
        // D s_17_16: select s_17_11 s_17_13 s_17_15
        let s_17_16: u64 = if s_17_11 { s_17_13 } else { s_17_15 };
        // D s_17_17: cast trunc s_17_16 -> u8
        let s_17_17: bool = ((s_17_16) != 0);
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 1u16);
        // C s_17_19: const #0u : u8
        let s_17_19: bool = false;
        // C s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 1u16);
        // D s_17_21: cmp-eq s_17_18 s_17_20
        let s_17_21: bool = ((s_17_18) == (s_17_20));
        // D s_17_22: write-var gs#142239 <= s_17_21
        fn_state.gs_142239 = s_17_21;
        // N s_17_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // D s_18_1: read-var CRm:u8
        let s_18_1: u8 = fn_state.CRm;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 4u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #2s : i
        let s_18_5: i128 = 2;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_0 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // D s_18_9: write-var b__0 <= s_18_8
        fn_state.b__0 = s_18_8;
        // C s_18_10: const #2s : i
        let s_18_10: i128 = 2;
        // D s_18_11: read-var b__0:u8
        let s_18_11: u8 = fn_state.b__0;
        // D s_18_12: cast zx s_18_11 -> bv
        let s_18_12: Bits = Bits::new(s_18_11 as u128, 3u16);
        // C s_18_13: const #1s : i64
        let s_18_13: i64 = 1;
        // C s_18_14: cast zx s_18_13 -> i
        let s_18_14: i128 = (i128::try_from(s_18_13).unwrap());
        // C s_18_15: const #0s : i
        let s_18_15: i128 = 0;
        // C s_18_16: add s_18_15 s_18_14
        let s_18_16: i128 = (s_18_15 + s_18_14);
        // D s_18_17: bit-extract s_18_12 s_18_10 s_18_16
        let s_18_17: Bits = (Bits::new(
            ((s_18_12) >> (s_18_10)).value(),
            u16::try_from(s_18_16).unwrap(),
        ));
        // D s_18_18: cast reint s_18_17 -> u8
        let s_18_18: bool = ((s_18_17.value()) != 0);
        // D s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 1u16);
        // C s_18_20: const #0u : u8
        let s_18_20: bool = false;
        // C s_18_21: cast zx s_18_20 -> bv
        let s_18_21: Bits = Bits::new(s_18_20 as u128, 1u16);
        // D s_18_22: cmp-eq s_18_19 s_18_21
        let s_18_22: bool = ((s_18_19) == (s_18_21));
        // N s_18_23: branch s_18_22 b24 b19
        if s_18_22 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#142233 <= s_19_0
        fn_state.gs_142233 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#142233:u8
        let s_20_0: bool = fn_state.gs_142233;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b23 b21
        if s_20_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#142228 <= s_21_0
        fn_state.gs_142228 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#142228:u8
        let s_22_0: bool = fn_state.gs_142228;
        // D s_22_1: write-var gs#142236 <= s_22_0
        fn_state.gs_142236 = s_22_0;
        // N s_22_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#142228 <= s_23_0
        fn_state.gs_142228 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var b__0:u8
        let s_24_1: u8 = fn_state.b__0;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 3u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #0s : i
        let s_24_5: i128 = 0;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: bool = ((s_24_7.value()) != 0);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 1u16);
        // C s_24_10: const #0u : u8
        let s_24_10: bool = false;
        // C s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 1u16);
        // D s_24_12: cmp-eq s_24_9 s_24_11
        let s_24_12: bool = ((s_24_9) == (s_24_11));
        // D s_24_13: write-var gs#142233 <= s_24_12
        fn_state.gs_142233 = s_24_12;
        // N s_24_14: jump b20
        return block_20(state, tracer, fn_state);
    }
}
