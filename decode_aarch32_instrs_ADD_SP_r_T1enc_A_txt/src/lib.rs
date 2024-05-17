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
use LastInITBlock::*;
use ConditionPassed::*;
use execute_aarch32_instrs_ADD_SP_r_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_ADD_SP_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    DM: bool,
    Rdm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_295709: bool,
        shift_t: u32,
        gs_295708: bool,
        shift_nshadow_7116: i128,
        d: i64,
        DM: bool,
        Rdm: u8,
    }
    let fn_state = FunctionState {
        DM,
        Rdm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var DM:u8
        let s_2_0: bool = fn_state.DM;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var Rdm:u8
        let s_2_2: u8 = fn_state.Rdm;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var d <= s_2_15
        fn_state.d = s_2_15;
        // D s_2_17: read-var DM:u8
        let s_2_17: bool = fn_state.DM;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: read-var Rdm:u8
        let s_2_19: u8 = fn_state.Rdm;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 3u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 4u16);
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (s_2_30.value() as i128);
        // D s_2_32: cast reint s_2_31 -> i64
        let s_2_32: i64 = (s_2_31 as i64);
        // D s_2_33: write-var m <= s_2_32
        fn_state.m = s_2_32;
        // C s_2_34: const #0s : i
        let s_2_34: i128 = 0;
        // C s_2_35: const #0u : u32
        let s_2_35: u32 = 0;
        // D s_2_36: create-product struct = ["s_2_35", "s_2_34"]
        let s_2_36: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_35,
            _1: s_2_34,
        };
        // D s_2_37: extract-field s_2_36.0
        let s_2_37: u32 = s_2_36._0;
        // C s_2_38: const #0u : u32
        let s_2_38: u32 = 0;
        // D s_2_39: create-product struct = ["s_2_38", "s_2_34"]
        let s_2_39: ProductType396b95aa74979079 = ProductType396b95aa74979079 {
            _0: s_2_38,
            _1: s_2_34,
        };
        // D s_2_40: extract-field s_2_39.1
        let s_2_40: i128 = s_2_39._1;
        // D s_2_41: cast reint s_2_40 -> i64
        let s_2_41: i64 = (s_2_40 as i64);
        // D s_2_42: write-var shift_t <= s_2_37
        fn_state.shift_t = s_2_37;
        // D s_2_43: cast zx s_2_41 -> i
        let s_2_43: i128 = (i128::try_from(s_2_41).unwrap());
        // D s_2_44: write-var shift_nshadow#7116 <= s_2_43
        fn_state.shift_nshadow_7116 = s_2_43;
        // C s_2_45: const #15s : i
        let s_2_45: i128 = 15;
        // D s_2_46: read-var d:i64
        let s_2_46: i64 = fn_state.d;
        // D s_2_47: cast zx s_2_46 -> i
        let s_2_47: i128 = (i128::try_from(s_2_46).unwrap());
        // D s_2_48: cmp-eq s_2_47 s_2_45
        let s_2_48: bool = ((s_2_47) == (s_2_45));
        // N s_2_49: branch s_2_48 b10 b3
        if s_2_48 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#295708 <= s_3_0
        fn_state.gs_295708 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#295708:u8
        let s_4_0: bool = fn_state.gs_295708;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#295709 <= s_5_0
        fn_state.gs_295709 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#295709:u8
        let s_6_0: bool = fn_state.gs_295709;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var shift_t:u32
        let s_7_0: u32 = fn_state.shift_t;
        // D s_7_1: read-var shift_nshadow#7116:i
        let s_7_1: i128 = fn_state.shift_nshadow_7116;
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // D s_7_3: read-var m:i64
        let s_7_3: i64 = fn_state.m;
        // C s_7_4: const #0u : u8
        let s_7_4: bool = false;
        // D s_7_5: call execute_aarch32_instrs_ADD_SP_r_Op_A_txt(s_7_2, s_7_3, s_7_4, s_7_1, s_7_0)
        let s_7_5: () = execute_aarch32_instrs_ADD_SP_r_Op_A_txt(
            state,
            tracer,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_1,
            s_7_0,
        );
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call LastInITBlock(s_9_0)
        let s_9_1: bool = LastInITBlock(state, tracer, s_9_0);
        // S s_9_2: not s_9_1
        let s_9_2: bool = !s_9_1;
        // D s_9_3: write-var gs#295709 <= s_9_2
        fn_state.gs_295709 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call InITBlock(s_10_0)
        let s_10_1: bool = InITBlock(state, tracer, s_10_0);
        // D s_10_2: write-var gs#295708 <= s_10_1
        fn_state.gs_295708 = s_10_1;
        // N s_10_3: jump b4
        return block_4(state, tracer, fn_state);
    }
}
